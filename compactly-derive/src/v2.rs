use std::collections::{BTreeSet, HashMap};

use proc_macro2::{Ident, Span};
use proc_macro_warning::Warning;
use quote::{quote, ToTokens};
use syn::{spanned::Spanned, Attribute, GenericParam, TraitBound};
use synstructure::{BindingInfo, VariantInfo};

/// Does `ty` name `LowCardinality` (ignoring any leading path qualifier)?
fn is_low_cardinality(ty: &syn::Type) -> bool {
    matches!(ty, syn::Type::Path(p)
        if p.path.segments.last().is_some_and(|s| s.ident == "LowCardinality"))
}

/// Does `ty` mention `String` anywhere (e.g. `String`, `Option<String>`,
/// `Vec<String>`, `Option<Vec<String>>`)? Used to flag the
/// `LowCardinality<String>` antipattern, which clones the String on every
/// repeated value; users should prefer `LowCardinality<Arc<str>>` instead.
fn type_mentions_string(ty: &syn::Type) -> bool {
    match ty {
        syn::Type::Path(p) => p.path.segments.iter().any(|seg| {
            seg.ident == "String"
                || match &seg.arguments {
                    syn::PathArguments::AngleBracketed(args) => args.args.iter().any(
                        |a| matches!(a, syn::GenericArgument::Type(t) if type_mentions_string(t)),
                    ),
                    _ => false,
                }
        }),
        _ => false,
    }
}

#[derive(Debug, Clone)]
struct EncodingStrategy(syn::Type);
impl EncodingStrategy {
    fn parse_attrs(attrs: &[Attribute]) -> Vec<EncodingStrategy> {
        attrs
            .iter()
            .filter_map(|a| {
                if a.path().is_ident("compactly") {
                    let strategy: syn::Type = a.parse_args().expect("Unrecognize strategy");
                    Some(EncodingStrategy(strategy))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    }
    fn parse(binding: &BindingInfo) -> Option<EncodingStrategy> {
        match Self::parse_attrs(&binding.ast().attrs).as_slice() {
            [] => None,
            [s] => Some(s.clone()),
            _ => panic!("Cannot support multiple encoding strategies: {binding:?}"),
        }
    }
}

/// `#[derive(EncodeV2)]`: the sync `Encode` impl and nothing else.
///
/// Deliberately emits no [`DecodeAsync`] impl. Doing so would put
/// `Normal: DecodeAsync<FieldTy>` on every field of every derived type — a
/// requirement a hand-written `Encode` impl cannot satisfy, and one that would
/// bite users who never enable the `stream` feature. Async is opt-in via
/// [`derive_compactly_async`].
pub(crate) fn derive_compactly(s: synstructure::Structure) -> proc_macro2::TokenStream {
    derive(s, false)
}

/// `#[derive(EncodeV2Async)]`: everything [`derive_compactly`] emits, **plus**
/// the `DecodeAsync` twin.
///
/// A superset rather than a companion derive, because the generated
/// `DerivedContext` lives inside synstructure's anonymous `const _` block: a
/// second macro could not name it, and would have to reach its fields by
/// guessing the binding names this one chose. Emitting both from one expansion
/// keeps that coupling internal.
///
/// `DecodeAsync` is required transitively — every field type needs one too — so
/// this is the derive to reach for on a whole type graph you intend to stream.
pub(crate) fn derive_compactly_async(s: synstructure::Structure) -> proc_macro2::TokenStream {
    derive(s, true)
}

fn derive(mut s: synstructure::Structure, emit_async: bool) -> proc_macro2::TokenStream {
    let mut bound_names = BTreeSet::new();
    bound_names.insert(Ident::new("discriminant", Span::call_site()));
    s.binding_name(|field, i| {
        if let Some(name) = &field.ident {
            if bound_names.contains(name) {
                for i in 0..10_000 {
                    let ident = Ident::new(&format!("{name}_{i}"), Span::call_site());
                    if !bound_names.contains(&ident) {
                        bound_names.insert(ident.clone());
                        return ident;
                    }
                }
                panic!("compactly does not currently support types with more than 10k identical field names");
            } else {
                bound_names.insert(name.clone());
                name.clone()
            }
        } else {
            let ident = {
                let ident = Ident::new(&format!("__binding_{i}"), Span::call_site());
            if bound_names.contains(&ident){
                crate::get_unique_name(&bound_names, "__binding_", 10000)
            }
            else {
                ident
            }
        };
            assert!(!bound_names.contains(&ident));
            bound_names.insert(ident.clone());
            ident
        }
    });

    let encode_trait = syn::parse_str::<TraitBound>("Encode").unwrap();
    let (_impl_generics, _ty_generics, where_clause) = s.ast().generics.split_for_impl();
    let mut where_clause = where_clause.cloned();
    s.add_trait_bounds(
        &encode_trait,
        &mut where_clause,
        synstructure::AddBounds::Generics,
    );

    let context_type_params = s
        .ast()
        .generics
        .params
        .iter()
        .filter_map(|param| {
            if let GenericParam::Type(ty) = param {
                Some(ty.ident.clone())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    let context_const_params = s
        .ast()
        .generics
        .params
        .iter()
        .filter_map(|param| {
            if let GenericParam::Const(c) = param {
                Some((c.ident.clone(), c.ty.clone()))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    let context_generics = {
        let type_bounds = context_type_params.iter().map(|t| quote! { #t: Encode });
        let const_defs = context_const_params
            .iter()
            .map(|(name, ty)| quote! { const #name: #ty });
        let items = type_bounds.chain(const_defs).collect::<Vec<_>>();
        if items.is_empty() {
            quote! {}
        } else {
            quote! { <#(#items),*> }
        }
    };
    let context_generics_without_bound = {
        let type_names = context_type_params.iter().map(|t| quote! { #t });
        let const_names = context_const_params
            .iter()
            .map(|(name, _)| quote! { #name });
        let items = type_names.chain(const_names).collect::<Vec<_>>();
        if items.is_empty() {
            quote! {}
        } else {
            quote! { <#(#items),*> }
        }
    };
    let mut binding_strategies: HashMap<Ident, Option<EncodingStrategy>> = HashMap::new();
    let mut strategies = Vec::new();
    for binding in s
        .variants()
        .iter()
        .flat_map(|variant| variant.bindings().iter())
    {
        let strategy = EncodingStrategy::parse(binding);
        strategies.push(strategy.clone());
        binding_strategies.insert(binding.binding.clone(), strategy);
    }

    // Emit a deprecation-style compiler warning for the `LowCardinality<String>`
    // antipattern: on a repeated value, the String variant clones (reallocates)
    // the cached String, whereas `LowCardinality<Arc<str>>` turns a cache hit into
    // a cheap refcount bump and uses less memory after deserialization.
    let antipattern_warnings = s
        .variants()
        .iter()
        .flat_map(|variant| variant.bindings().iter())
        .filter_map(|binding| {
            let strategy = binding_strategies.get(&binding.binding)?.as_ref()?;
            let ty = &binding.ast().ty;
            if is_low_cardinality(&strategy.0) && type_mentions_string(ty) {
                Some(ty.span())
            } else {
                None
            }
        })
        .enumerate()
        .map(|(i, span)| {
            Warning::new_deprecated("LowCardinalityString")
                .old("encode a String field with `#[compactly(LowCardinality)]`, which clones (reallocates) the String on every repeated value")
                .new("use `Arc<str>` (i.e. `#[compactly(LowCardinality)] field: Arc<str>`), so a cache hit is a cheap refcount bump and deserialization shares buffers")
                .index(i)
                .span(span)
                .build_or_panic()
        })
        .collect::<Vec<_>>();
    let context = s
        .variants()
        .iter()
        .flat_map(|variant| variant.bindings().iter())
        .zip(strategies.iter().cloned())
        .map(|(binding, strategy)| {
            let ty = &binding.ast().ty;
            let name = &binding.binding;
            if let Some(strategy) = strategy {
                let strategy = strategy.0;
                quote! {
                    #name: <#strategy as EncodingStrategy<#ty>>::Context
                }
            } else {
                quote! {
                    #name: <#ty as Encode>::Context
                }
            }
        })
        .collect::<Vec<_>>();
    let bindings = s
        .variants()
        .iter()
        .flat_map(|variant| variant.bindings().iter().map(|binding| &binding.binding))
        .collect::<Vec<_>>();

    let encode_fields = s.each(|binding| {
        let ty = &binding.ast().ty;
        let binding = &binding.binding;
        if let Some(Some(strategy)) = binding_strategies.get(binding) {
            let strategy = &strategy.0;
            quote! {
                <#strategy as EncodingStrategy<#ty>>::encode(&#binding, writer, &mut ctx.#binding);
            }
        } else {
            quote! {
                #binding.encode(writer, &mut ctx.#binding);
            }
        }
    });
    let num_variants = s.variants().len();
    let max_discriminant = num_variants - 1;
    let discriminant_type = quote! { compactly::v2::AtMost<#max_discriminant> };
    let get_discriminant = |variant: &VariantInfo| -> usize {
        s.variants()
            .iter()
            .enumerate()
            .find(|(_, v)| v.ast().ident == variant.ast().ident)
            .map(|x| x.0)
            .expect("bug: invalid variant")
    };
    let encode_discriminant = s.each_variant(|variant| {
        let discriminant = get_discriminant(variant);
        quote! {
            compactly::v2::AtMost::<#max_discriminant>::new(#discriminant).encode(writer, &mut ctx.discriminant);
        }
    });

    let decode_variants = s
        .variants()
        .iter()
        .map(|variant| {
            let decoding = variant
                .bindings()
                .iter()
                .map(|binding| {
                    if let Some(Some(strategy)) = binding_strategies.get(&binding.binding) {
                        let strategy = &strategy.0;
                        let ty = &binding.ast().ty;
                        quote! {
                            <#strategy as EncodingStrategy<#ty>>::decode(reader, &mut ctx.#binding)?
                        }
                    } else {
                        quote! {
                            Encode::decode(reader, &mut ctx.#binding)?
                        }
                    }
                })
                .collect::<Vec<_>>();
            variant.construct(|_, i| decoding[i].clone())
        })
        .collect::<Vec<_>>();
    let discriminants = 0..s.variants().len();
    let decode = quote! {
        Ok(match usize::from(discriminant) {
            #(#discriminants => #decode_variants,)*
            _ => return Err(std::io::Error::other("This discriminant should be impossible"))
        })
    };

    // `MAX_BYTES` for the derived type: the discriminant, plus the worst variant.
    // Within a variant the fields are coded in sequence, so their bounds **sum**;
    // across variants only one is ever coded, so they **max**. Each field
    // contributes whatever its own strategy declares, so a field that has not
    // opted in (`usize::MAX`) saturates the whole type to unbounded — which is
    // the safe direction, since an unbounded type simply never takes the async
    // decoder's sync fast path.
    let variant_bounds = s
        .variants()
        .iter()
        .map(|variant| {
            let field_bounds = variant.bindings().iter().map(|binding| {
                let ty = &binding.ast().ty;
                if let Some(Some(strategy)) = binding_strategies.get(&binding.binding) {
                    let strategy = &strategy.0;
                    quote! { <#strategy as DecodeAsync<#ty>>::MAX_BYTES }
                } else {
                    quote! { <Normal as DecodeAsync<#ty>>::MAX_BYTES }
                }
            });
            quote! { 0usize #(.saturating_add(#field_bounds))* }
        })
        .collect::<Vec<_>>();
    let num_variant_bounds = variant_bounds.len();

    // The async twin's decode arms, mirroring `decode_variants` with `.await`.
    let decode_variants_async = s
        .variants()
        .iter()
        .map(|variant| {
            let decoding = variant
                .bindings()
                .iter()
                .map(|binding| {
                    let ty = &binding.ast().ty;
                    if let Some(Some(strategy)) = binding_strategies.get(&binding.binding) {
                        let strategy = &strategy.0;
                        quote! {
                            <#strategy as DecodeAsync<#ty>>::decode_async(reader, &mut ctx.#binding).await?
                        }
                    } else {
                        quote! {
                            <Normal as DecodeAsync<#ty>>::decode_async(reader, &mut ctx.#binding).await?
                        }
                    }
                })
                .collect::<Vec<_>>();
            variant.construct(|_, i| decoding[i].clone())
        })
        .collect::<Vec<_>>();
    let discriminants_async = 0..s.variants().len();

    // Bounds for the async impl: one predicate per generic *type parameter*,
    // never per field type. Bounding field types instead looks more precise and
    // is what a first cut reaches for, but it makes a recursive type require
    // itself — `struct Tree { kids: Vec<Tree> }` would ask the solver to prove
    // `Normal: DecodeAsync<Tree>` from `Normal: DecodeAsync<Vec<Tree>>` from
    // `Normal: DecodeAsync<Tree>`, with no base case, and it overflows. Concrete
    // field types need no predicate anyway: the compiler discharges them
    // straight from the impls, which is exactly how the sync derive gets away
    // with `AddBounds::Generics`.
    let async_param_bounds = context_type_params
        .iter()
        .map(|t| {
            // The context equality restates what the blanket `EncodingStrategy
            // for Normal` impl already says; with `Normal: DecodeAsync<#t>` in
            // the param env the compiler stops normalizing through that impl.
            quote! {
                Normal: DecodeAsync<#t> + EncodingStrategy<#t, Context = <#t as Encode>::Context>
            }
        })
        .collect::<Vec<_>>();
    // A field written `#[compactly(Small)] x: T` codes through `Small`, and no
    // bound on `Normal` implies `Small` has an async twin for `T`. Only fields
    // whose type *is* a parameter need this — a composite like `Vec<T>` resolves
    // through the impls given the parameter bounds above.
    let async_strategy_bounds = {
        let mut seen = std::collections::HashSet::new();
        s.variants()
            .iter()
            .flat_map(|variant| variant.bindings().iter())
            .filter_map(|binding| {
                let strategy = &binding_strategies.get(&binding.binding)?.as_ref()?.0;
                let ty = &binding.ast().ty;
                let is_param = matches!(ty, syn::Type::Path(p)
                    if p.qself.is_none()
                        && p.path.get_ident().is_some_and(|i| context_type_params.contains(i)));
                if !is_param {
                    return None;
                }
                let bound = quote! { #strategy: DecodeAsync<#ty> };
                seen.insert(bound.to_string()).then_some(bound)
            })
            .collect::<Vec<_>>()
    };
    let async_type_bounds = context_type_params.iter().map(|t| quote! { #t: Encode });
    let async_impl_generics = {
        let type_params = context_type_params.iter().map(|t| quote! { #t });
        let const_defs = context_const_params
            .iter()
            .map(|(name, ty)| quote! { const #name: #ty });
        let items = type_params.chain(const_defs).collect::<Vec<_>>();
        if items.is_empty() {
            quote! {}
        } else {
            quote! { <#(#items),*> }
        }
    };
    let self_ty = {
        let name = &s.ast().ident;
        quote! { #name #context_generics_without_bound }
    };

    let strategies_to_impl = EncodingStrategy::parse_attrs(&s.ast().attrs);
    let impl_strategies = if strategies_to_impl.is_empty() {
        Vec::new()
    } else {
        let typename = s.ast().ident.clone();
        assert_eq!(num_variants, 1, "Cannot derive strategy for an enum");
        let bindings = s.variants()[0].bindings();
        assert_eq!(
            bindings.len(),
            1,
            "Can only derive strategy for newtype structs"
        );
        let binding = &bindings[0];
        strategies_to_impl
        .into_iter()
        .map(|EncodingStrategy(strategy)| {
            let ty = binding.ast().ty.clone();
            let field_name = binding.ast().ident.as_ref().map(|i| i.to_token_stream()).unwrap_or(quote! {0});
            let decoded = s.variants()[0].construct(|_, _| quote! { <#strategy as EncodingStrategy<#ty>>::decode(reader, ctx)? });
            quote! {
                impl EncodingStrategy<#typename> for #strategy {
                    type Context = <#strategy as EncodingStrategy<#ty>>::Context;
                    fn encode<E: EntropyCoder>(value: &#typename, writer: &mut E, ctx: &mut Self::Context) {
                        <#strategy as EncodingStrategy<#ty>>::encode(&value.#field_name, writer, ctx)
                    }
                    fn decode<D: EntropyDecoder>(reader: &mut D, ctx: &mut Self::Context) -> Result<#typename, std::io::Error> {
                        Ok(#decoded)
                    }
                }
            }
        })
        .collect::<Vec<_>>()
    };

    // Only `EncodeV2Async` emits this. Emitting it from the plain derive would
    // impose `Normal: DecodeAsync<FieldTy>` on every field of every derived
    // type — unsatisfiable for a hand-written `Encode` impl, and imposed even
    // on users who never enable the `stream` feature.
    let async_impl = if emit_async {
        quote! {
            impl #async_impl_generics DecodeAsync<#self_ty> for Normal
            where
                #(#async_type_bounds,)*
                #(#async_param_bounds,)*
                #(#async_strategy_bounds,)*
            {
                /// The discriminant, plus the worst variant: within a variant the
                /// fields are coded in sequence so their bounds **sum**, and across
                /// variants only one is ever coded so they **max**. A field whose
                /// strategy declares `usize::MAX` saturates the whole type to
                /// unbounded, which is the safe direction — an unbounded type simply
                /// never takes the async decoder's sync fast path.
                const MAX_BYTES: usize = {
                    let variants: [usize; #num_variant_bounds] = [#(#variant_bounds),*];
                    let mut worst = 0usize;
                    let mut i = 0;
                    while i < #num_variant_bounds {
                        if variants[i] > worst {
                            worst = variants[i];
                        }
                        i += 1;
                    }
                    <Normal as DecodeAsync<#discriminant_type>>::MAX_BYTES.saturating_add(worst)
                };

                fn decode_awaiting<D: compactly::v2::AsyncEntropyDecoder>(
                    reader: &mut D,
                    ctx: &mut Self::Context,
                ) -> impl ::core::future::Future<Output = Result<#self_ty, std::io::Error>> {
                    #![allow(unused_variables, non_shorthand_field_patterns)]
                    // Not boxed. A recursive user type would make this future
                    // infinitely sized, but no such type exists: a context holds
                    // one field per field, so `struct Tree { kids: Vec<Tree> }`
                    // already fails to compile on the *sync* path with a context
                    // layout cycle — through `Box` and `Option` just as much as
                    // through `Vec`, since neither adds indirection to the context.
                    // Boxing here would buy nothing and cost an allocation.
                    async move {
                        let discriminant: #discriminant_type =
                            <Normal as DecodeAsync<#discriminant_type>>::decode_async(
                                reader,
                                &mut ctx.discriminant,
                            )
                            .await?;
                        Ok(match usize::from(discriminant) {
                            #(#discriminants_async => #decode_variants_async,)*
                            _ => return Err(std::io::Error::other(
                                "This discriminant should be impossible",
                            )),
                        })
                    }
                }
            }
        }
    } else {
        quote! {}
    };

    s.gen_impl(quote! {
        extern crate compactly;
        use compactly::v2::{DecodeAsync, Encode, EncodingStrategy, EntropyCoder, EntropyDecoder};
        use compactly::{Small, LowCardinality, Decimal, Compressible, Incompressible, Mapping, Normal, Sorted, Values};

        #(#antipattern_warnings)*

        pub struct DerivedContext #context_generics {
            discriminant: <#discriminant_type as Encode>::Context,
            #(#context,)*
        }
        impl #context_generics Default for DerivedContext #context_generics_without_bound {
            fn default() -> Self {
                Self {
                    discriminant: Default::default(),
                    #(#bindings: Default::default(),)*
                }
            }
        }
        impl #context_generics Clone for DerivedContext #context_generics_without_bound {
            fn clone(&self) -> Self {
                Self {
                    discriminant: self.discriminant.clone(),
                    #(#bindings: self.#bindings.clone(),)*
                }
            }
        }

        #(#impl_strategies)*

        #async_impl

        gen impl Encode for @Self {
            #![allow(unused_variables,non_shorthand_field_patterns)]
            type Context = DerivedContext #context_generics_without_bound;
            fn encode<E: EntropyCoder>(&self, writer: &mut E, ctx: &mut Self::Context) {
                match self { #encode_discriminant }
                match self { #encode_fields }
            }
            fn decode<D: EntropyDecoder>(
                reader: &mut D,
                ctx: &mut Self::Context,
            ) -> Result<Self, std::io::Error> {
                let discriminant: #discriminant_type = Encode::decode(reader, &mut ctx.discriminant)?;
                #decode
            }
        }
    })
}

#[cfg(test)]
fn pretty(tokens: proc_macro2::TokenStream) -> String {
    if let Ok(syntax_tree) = syn::parse2::<syn::File>(tokens.clone()) {
        prettyplease::unparse(&syntax_tree)
    } else {
        tokens.to_string()
    }
}

#[test]
fn const_generic_in_field_type_forwarded_to_context() {
    // Const generic params that appear in field types must be forwarded to
    // DerivedContext so that references like `<[u8; N] as Encode>::Context`
    // are valid inside the struct body.
    let di: syn::DeriveInput = syn::parse_quote! {
        pub struct Buffer<const N: usize> {
            data: [u8; N],
        }
    };
    let s = synstructure::Structure::new(&di);
    let output = pretty(derive_compactly(s));
    assert!(
        output.contains("pub struct DerivedContext<const N: usize> {"),
        "expected DerivedContext to carry `const N: usize`:\n{output}"
    );
    assert!(
        output.contains("<[u8; N] as Encode>::Context"),
        "expected field to reference N:\n{output}"
    );
}

#[test]
fn field_named_discriminant_is_renamed() {
    // A user field named `discriminant` used to collide with the hardcoded
    // `discriminant` field in DerivedContext. After pre-seeding bound_names with
    // "discriminant", the user field is automatically renamed to `discriminant_0`.
    let di: syn::DeriveInput = syn::parse_quote! {
        pub struct HasDiscriminant {
            discriminant: u32,
            value: bool,
        }
    };
    let s = synstructure::Structure::new(&di);
    let output = pretty(derive_compactly(s));
    assert!(
        output.contains("discriminant: <compactly::v2::AtMost<0usize> as Encode>::Context,"),
        "expected hardcoded discriminant field:\n{output}"
    );
    assert!(
        output.contains("discriminant_0: <u32 as Encode>::Context,"),
        "expected user field renamed to discriminant_0:\n{output}"
    );
    assert!(
        !output.contains(
            "discriminant: <compactly::v2::AtMost<0usize> as Encode>::Context,\n        discriminant: <u32 as Encode>::Context,"
        ),
        "must not have duplicate discriminant fields:\n{output}"
    );
}

#[test]
fn low_cardinality_string_warns() {
    // A `LowCardinality<String>` field should expand to a deprecation warning
    // steering the user toward `Arc<str>`; non-String LowCardinality fields and
    // `Arc<str>` fields should not.
    let di: syn::DeriveInput = syn::parse_quote! {
        pub struct Record {
            #[compactly(LowCardinality)]
            recclass: String,
            #[compactly(LowCardinality)]
            tags: Option<Vec<String>>,
            #[compactly(LowCardinality)]
            shared: std::sync::Arc<str>,
            #[compactly(LowCardinality)]
            count: u32,
        }
    };
    let s = synstructure::Structure::new(&di);
    let output = pretty(derive_compactly(s));
    // Two String-bearing fields → two warnings.
    assert_eq!(
        output.matches("#[deprecated").count(),
        2,
        "expected exactly two deprecation warnings (String + Option<Vec<String>>):\n{output}"
    );
    assert!(
        output.contains("fn LowCardinalityString_0()")
            && output.contains("fn LowCardinalityString_1()"),
        "expected indexed warning fns:\n{output}"
    );
}

#[test]
fn impl_two_strategies() {
    let di: syn::DeriveInput = syn::parse_quote! {
        #[compactly(Small)]
        #[compactly(Sorted)]
        pub struct NewType(u32);
    };
    let s = synstructure::Structure::new(&di);

    expect_test::expect![[r#"
        const _: () = {
            extern crate compactly;
            use compactly::v2::{
                DecodeAsync, Encode, EncodingStrategy, EntropyCoder, EntropyDecoder,
            };
            use compactly::{
                Small, LowCardinality, Decimal, Compressible, Incompressible, Mapping, Normal,
                Sorted, Values,
            };
            pub struct DerivedContext {
                discriminant: <compactly::v2::AtMost<0usize> as Encode>::Context,
                __binding_0: <u32 as Encode>::Context,
            }
            impl Default for DerivedContext {
                fn default() -> Self {
                    Self {
                        discriminant: Default::default(),
                        __binding_0: Default::default(),
                    }
                }
            }
            impl Clone for DerivedContext {
                fn clone(&self) -> Self {
                    Self {
                        discriminant: self.discriminant.clone(),
                        __binding_0: self.__binding_0.clone(),
                    }
                }
            }
            impl EncodingStrategy<NewType> for Small {
                type Context = <Small as EncodingStrategy<u32>>::Context;
                fn encode<E: EntropyCoder>(
                    value: &NewType,
                    writer: &mut E,
                    ctx: &mut Self::Context,
                ) {
                    <Small as EncodingStrategy<u32>>::encode(&value.0, writer, ctx)
                }
                fn decode<D: EntropyDecoder>(
                    reader: &mut D,
                    ctx: &mut Self::Context,
                ) -> Result<NewType, std::io::Error> {
                    Ok(NewType(<Small as EncodingStrategy<u32>>::decode(reader, ctx)?))
                }
            }
            impl EncodingStrategy<NewType> for Sorted {
                type Context = <Sorted as EncodingStrategy<u32>>::Context;
                fn encode<E: EntropyCoder>(
                    value: &NewType,
                    writer: &mut E,
                    ctx: &mut Self::Context,
                ) {
                    <Sorted as EncodingStrategy<u32>>::encode(&value.0, writer, ctx)
                }
                fn decode<D: EntropyDecoder>(
                    reader: &mut D,
                    ctx: &mut Self::Context,
                ) -> Result<NewType, std::io::Error> {
                    Ok(NewType(<Sorted as EncodingStrategy<u32>>::decode(reader, ctx)?))
                }
            }
            impl Encode for NewType {
                #![allow(unused_variables, non_shorthand_field_patterns)]
                type Context = DerivedContext;
                fn encode<E: EntropyCoder>(&self, writer: &mut E, ctx: &mut Self::Context) {
                    match self {
                        NewType(ref __binding_0) => {
                            compactly::v2::AtMost::<0usize>::new(0usize)
                                .encode(writer, &mut ctx.discriminant);
                        }
                    }
                    match self {
                        NewType(ref __binding_0) => {
                            __binding_0.encode(writer, &mut ctx.__binding_0);
                        }
                    }
                }
                fn decode<D: EntropyDecoder>(
                    reader: &mut D,
                    ctx: &mut Self::Context,
                ) -> Result<Self, std::io::Error> {
                    let discriminant: compactly::v2::AtMost<0usize> = Encode::decode(
                        reader,
                        &mut ctx.discriminant,
                    )?;
                    Ok(
                        match usize::from(discriminant) {
                            0usize => NewType(Encode::decode(reader, &mut ctx.__binding_0)?),
                            _ => {
                                return Err(
                                    std::io::Error::other(
                                        "This discriminant should be impossible",
                                    ),
                                );
                            }
                        },
                    )
                }
            }
        };
    "#]]
    .assert_eq(&pretty(derive_compactly(s)));
}

#[test]
fn impl_strategies() {
    let di: syn::DeriveInput = syn::parse_quote! {
        #[compactly(Sorted)]
        pub struct NewType(u32);
    };
    let s = synstructure::Structure::new(&di);

    expect_test::expect![[r#"
        const _: () = {
            extern crate compactly;
            use compactly::v2::{
                DecodeAsync, Encode, EncodingStrategy, EntropyCoder, EntropyDecoder,
            };
            use compactly::{
                Small, LowCardinality, Decimal, Compressible, Incompressible, Mapping, Normal,
                Sorted, Values,
            };
            pub struct DerivedContext {
                discriminant: <compactly::v2::AtMost<0usize> as Encode>::Context,
                __binding_0: <u32 as Encode>::Context,
            }
            impl Default for DerivedContext {
                fn default() -> Self {
                    Self {
                        discriminant: Default::default(),
                        __binding_0: Default::default(),
                    }
                }
            }
            impl Clone for DerivedContext {
                fn clone(&self) -> Self {
                    Self {
                        discriminant: self.discriminant.clone(),
                        __binding_0: self.__binding_0.clone(),
                    }
                }
            }
            impl EncodingStrategy<NewType> for Sorted {
                type Context = <Sorted as EncodingStrategy<u32>>::Context;
                fn encode<E: EntropyCoder>(
                    value: &NewType,
                    writer: &mut E,
                    ctx: &mut Self::Context,
                ) {
                    <Sorted as EncodingStrategy<u32>>::encode(&value.0, writer, ctx)
                }
                fn decode<D: EntropyDecoder>(
                    reader: &mut D,
                    ctx: &mut Self::Context,
                ) -> Result<NewType, std::io::Error> {
                    Ok(NewType(<Sorted as EncodingStrategy<u32>>::decode(reader, ctx)?))
                }
            }
            impl Encode for NewType {
                #![allow(unused_variables, non_shorthand_field_patterns)]
                type Context = DerivedContext;
                fn encode<E: EntropyCoder>(&self, writer: &mut E, ctx: &mut Self::Context) {
                    match self {
                        NewType(ref __binding_0) => {
                            compactly::v2::AtMost::<0usize>::new(0usize)
                                .encode(writer, &mut ctx.discriminant);
                        }
                    }
                    match self {
                        NewType(ref __binding_0) => {
                            __binding_0.encode(writer, &mut ctx.__binding_0);
                        }
                    }
                }
                fn decode<D: EntropyDecoder>(
                    reader: &mut D,
                    ctx: &mut Self::Context,
                ) -> Result<Self, std::io::Error> {
                    let discriminant: compactly::v2::AtMost<0usize> = Encode::decode(
                        reader,
                        &mut ctx.discriminant,
                    )?;
                    Ok(
                        match usize::from(discriminant) {
                            0usize => NewType(Encode::decode(reader, &mut ctx.__binding_0)?),
                            _ => {
                                return Err(
                                    std::io::Error::other(
                                        "This discriminant should be impossible",
                                    ),
                                );
                            }
                        },
                    )
                }
            }
        };
    "#]]
    .assert_eq(&pretty(derive_compactly(s)));
}

/// The async derive is a strict superset of the sync one: the same `Encode`
/// impl and context, plus the `DecodeAsync` twin. Snapshotted separately so the
/// twin's output is covered — the other snapshots here all take the sync derive,
/// which deliberately emits none.
#[test]
fn impl_newtype_async_adds_the_twin() {
    let di: syn::DeriveInput = syn::parse_quote! {
        pub struct NewType(u32);
    };
    let s = synstructure::Structure::new(&di);
    let sync = pretty(derive_compactly(synstructure::Structure::new(&di)));
    let with_async = pretty(derive_compactly_async(s));

    assert!(
        !sync.contains("impl DecodeAsync"),
        "the sync derive must not emit a DecodeAsync impl:\n{sync}"
    );
    assert!(
        with_async.contains("impl DecodeAsync<NewType> for Normal"),
        "the async derive must emit the twin:\n{with_async}"
    );
    // Everything the sync derive emits is still there: same context, same
    // `Encode` impl, so switching derives never loses the sync half.
    for needle in [
        "pub struct DerivedContext",
        "impl Encode for NewType",
        "fn encode<E: EntropyCoder>",
        "fn decode<D: EntropyDecoder>",
    ] {
        assert!(
            with_async.contains(needle),
            "async derive dropped {needle:?}:\n{with_async}"
        );
    }
}

#[test]
fn impl_newtype() {
    let di: syn::DeriveInput = syn::parse_quote! {
        pub struct NewType(u32);
    };
    let s = synstructure::Structure::new(&di);

    expect_test::expect![[r#"
        const _: () = {
            extern crate compactly;
            use compactly::v2::{
                DecodeAsync, Encode, EncodingStrategy, EntropyCoder, EntropyDecoder,
            };
            use compactly::{
                Small, LowCardinality, Decimal, Compressible, Incompressible, Mapping, Normal,
                Sorted, Values,
            };
            pub struct DerivedContext {
                discriminant: <compactly::v2::AtMost<0usize> as Encode>::Context,
                __binding_0: <u32 as Encode>::Context,
            }
            impl Default for DerivedContext {
                fn default() -> Self {
                    Self {
                        discriminant: Default::default(),
                        __binding_0: Default::default(),
                    }
                }
            }
            impl Clone for DerivedContext {
                fn clone(&self) -> Self {
                    Self {
                        discriminant: self.discriminant.clone(),
                        __binding_0: self.__binding_0.clone(),
                    }
                }
            }
            impl Encode for NewType {
                #![allow(unused_variables, non_shorthand_field_patterns)]
                type Context = DerivedContext;
                fn encode<E: EntropyCoder>(&self, writer: &mut E, ctx: &mut Self::Context) {
                    match self {
                        NewType(ref __binding_0) => {
                            compactly::v2::AtMost::<0usize>::new(0usize)
                                .encode(writer, &mut ctx.discriminant);
                        }
                    }
                    match self {
                        NewType(ref __binding_0) => {
                            __binding_0.encode(writer, &mut ctx.__binding_0);
                        }
                    }
                }
                fn decode<D: EntropyDecoder>(
                    reader: &mut D,
                    ctx: &mut Self::Context,
                ) -> Result<Self, std::io::Error> {
                    let discriminant: compactly::v2::AtMost<0usize> = Encode::decode(
                        reader,
                        &mut ctx.discriminant,
                    )?;
                    Ok(
                        match usize::from(discriminant) {
                            0usize => NewType(Encode::decode(reader, &mut ctx.__binding_0)?),
                            _ => {
                                return Err(
                                    std::io::Error::other(
                                        "This discriminant should be impossible",
                                    ),
                                );
                            }
                        },
                    )
                }
            }
        };
    "#]]
    .assert_eq(&pretty(derive_compactly(s)));
}
