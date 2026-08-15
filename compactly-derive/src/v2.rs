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
        crate::parse_compactly_attrs(attrs)
            .strategies
            .into_iter()
            .map(EncodingStrategy)
            .collect()
    }
    fn parse(binding: &BindingInfo) -> Option<EncodingStrategy> {
        crate::parse_compactly_attrs(&binding.ast().attrs)
            .single_strategy(binding)
            .map(EncodingStrategy)
    }
}

/// The strategy a field is encoded with: whatever `#[compactly(...)]` named, or
/// `Normal` when the field carries no attribute. Since `Normal` is just the
/// default of `Encode<S>`, both cases generate the same `<Ty as Encode<S>>::…`
/// call and neither the context nor the encode/decode bodies need a branch.
fn strategy_or_normal(strategy: Option<&EncodingStrategy>) -> proc_macro2::TokenStream {
    match strategy {
        Some(EncodingStrategy(ty)) => quote! { #ty },
        None => quote! { Normal },
    }
}

/// `#[derive(EncodeV2)]`: the full `Encode` impl — `encode`/`decode` plus the
/// async-decode members (`MAX_BYTES`, `decode_awaiting`) that `decode_stream`
/// needs.
///
/// There is only one derive. `Encode<S>` is a single trait with no opt-out, so
/// every field's strategy must support decoding from a stream too — which
/// every strategy in this crate does, and a hand-written `Encode` impl must
/// too, now that `MAX_BYTES`/`decode_awaiting` are required members rather
/// than a separate trait a type could decline.
pub(crate) fn derive_compactly(mut s: synstructure::Structure) -> proc_macro2::TokenStream {
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
    // a cheap refcount bump and uses less memory after deserialization. A field
    // can opt out with `#[compactly(LowCardinality, allow_string)]`.
    let antipattern_warnings = s
        .variants()
        .iter()
        .flat_map(|variant| variant.bindings().iter())
        .filter_map(|binding| {
            let strategy = binding_strategies.get(&binding.binding)?.as_ref()?;
            let ty = &binding.ast().ty;
            if is_low_cardinality(&strategy.0)
                && type_mentions_string(ty)
                && !crate::parse_compactly_attrs(&binding.ast().attrs).allow_string
            {
                Some(ty.span())
            } else {
                None
            }
        })
        .enumerate()
        .map(|(i, span)| {
            Warning::new_deprecated("LowCardinalityString")
                .old("encode a String field with `#[compactly(LowCardinality)]`, which clones (reallocates) the String on every repeated value")
                .new("use `Arc<str>` (i.e. `#[compactly(LowCardinality)] field: Arc<str>`), so a cache hit is a cheap refcount bump and deserialization shares buffers; or, if you have weighed the tradeoff and want a `String` anyway, silence this warning for the field with `#[compactly(LowCardinality, allow_string)]`")
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
            let strategy = strategy_or_normal(strategy.as_ref());
            quote! {
                #name: <#ty as Encode<#strategy>>::Context
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
        let strategy = strategy_or_normal(binding_strategies.get(binding).and_then(|s| s.as_ref()));
        quote! {
            <#ty as Encode<#strategy>>::encode(&#binding, writer, &mut ctx.#binding);
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
            Normal::encode(&compactly::v2::AtMost::<#max_discriminant>::new(#discriminant), writer, &mut ctx.discriminant);
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
                    let ty = &binding.ast().ty;
                    let strategy = strategy_or_normal(
                        binding_strategies
                            .get(&binding.binding)
                            .and_then(|s| s.as_ref()),
                    );
                    let name = &binding.binding;
                    quote! {
                        <#ty as Encode<#strategy>>::decode(reader, &mut ctx.#name)?
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
                let strategy = strategy_or_normal(
                    binding_strategies
                        .get(&binding.binding)
                        .and_then(|s| s.as_ref()),
                );
                quote! { <#ty as Encode<#strategy>>::MAX_BYTES }
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
                    let strategy = strategy_or_normal(
                        binding_strategies
                            .get(&binding.binding)
                            .and_then(|s| s.as_ref()),
                    );
                    quote! {
                        <#ty as Encode<#strategy>>::decode_async(reader, &mut ctx.#binding).await?
                    }
                })
                .collect::<Vec<_>>();
            variant.construct(|_, i| decoding[i].clone())
        })
        .collect::<Vec<_>>();
    let discriminants_async = 0..s.variants().len();

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
            let decoded = s.variants()[0].construct(|_, _| quote! { <#ty as Encode<#strategy>>::decode(reader, ctx)? });
            let decoded_async = s.variants()[0].construct(|_, _| quote! { <#ty as Encode<#strategy>>::decode_async(reader, ctx).await? });
            quote! {
                impl Encode<#strategy> for #typename {
                    type Context = <#ty as Encode<#strategy>>::Context;
                    fn encode<E: EntropyCoder>(value: &#typename, writer: &mut E, ctx: &mut Self::Context) {
                        <#ty as Encode<#strategy>>::encode(&value.#field_name, writer, ctx)
                    }
                    fn decode<D: EntropyDecoder>(reader: &mut D, ctx: &mut Self::Context) -> Result<#typename, std::io::Error> {
                        Ok(#decoded)
                    }

                    const MAX_BYTES: usize = <#ty as Encode<#strategy>>::MAX_BYTES;

                    fn decode_awaiting<D: compactly::v2::AsyncEntropyDecoder>(
                        reader: &mut D,
                        ctx: &mut Self::Context,
                    ) -> impl ::core::future::Future<Output = Result<#typename, std::io::Error>> {
                        async move { Ok(#decoded_async) }
                    }
                }
            }
        })
        .collect::<Vec<_>>()
    };

    s.gen_impl(quote! {
        extern crate compactly;
        use compactly::v2::{Encode, EntropyCoder, EntropyDecoder, Strategy as _};
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

        gen impl Encode<Normal> for @Self {
            #![allow(unused_variables,non_shorthand_field_patterns)]
            type Context = DerivedContext #context_generics_without_bound;
            fn encode<E: EntropyCoder>(value: &Self, writer: &mut E, ctx: &mut Self::Context) {
                match value { #encode_discriminant }
                match value { #encode_fields }
            }
            fn decode<D: EntropyDecoder>(
                reader: &mut D,
                ctx: &mut Self::Context,
            ) -> Result<Self, std::io::Error> {
                let discriminant: #discriminant_type = <#discriminant_type as Encode>::decode(reader, &mut ctx.discriminant)?;
                #decode
            }

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
                <#discriminant_type as Encode>::MAX_BYTES.saturating_add(worst)
            };

            fn decode_awaiting<D: compactly::v2::AsyncEntropyDecoder>(
                reader: &mut D,
                ctx: &mut Self::Context,
            ) -> impl ::core::future::Future<Output = Result<Self, std::io::Error>> {
                // Not boxed. A recursive user type would make this future
                // infinitely sized, but no such type exists: a context holds
                // one field per field, so `struct Tree { kids: Vec<Tree> }`
                // already fails to compile on the *sync* path with a context
                // layout cycle — through `Box` and `Option` just as much as
                // through `Vec`, since neither adds indirection to the context.
                // Boxing here would buy nothing and cost an allocation.
                async move {
                    let discriminant: #discriminant_type =
                        <#discriminant_type as Encode>::decode_async(
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
        output.contains("<[u8; N] as Encode<Normal>>::Context"),
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
        output.contains("discriminant_0: <u32 as Encode<Normal>>::Context,"),
        "expected user field renamed to discriminant_0:\n{output}"
    );
    assert!(
        !output.contains(
            "discriminant: <compactly::v2::AtMost<0usize> as Encode>::Context,\n        discriminant: <u32 as Encode<Normal>>::Context,"
        ),
        "must not have duplicate discriminant fields:\n{output}"
    );
}

#[test]
fn low_cardinality_string_warns() {
    // A `LowCardinality<String>` field should expand to a deprecation warning
    // steering the user toward `Arc<str>`; non-String LowCardinality fields,
    // `Arc<str>` fields, and fields carrying the `allow_string` opt-out should not.
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
            #[compactly(LowCardinality, allow_string)]
            silenced: String,
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
#[should_panic(expected = "unknown compactly flag `alow_string`")]
fn misspelled_flag_is_rejected() {
    // A snake_case bare ident that isn't a known flag (here `alow_string`, a typo
    // of `allow_string`) must be rejected with a clear message rather than being
    // treated as a bogus encoding strategy and panicking later with a raw
    // BindingInfo debug dump.
    let di: syn::DeriveInput = syn::parse_quote! {
        pub struct Record {
            #[compactly(LowCardinality, alow_string)]
            recclass: String,
        }
    };
    let s = synstructure::Structure::new(&di);
    let _ = derive_compactly(s);
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
            use compactly::v2::{Encode, EntropyCoder, EntropyDecoder, Strategy as _};
            use compactly::{
                Small, LowCardinality, Decimal, Compressible, Incompressible, Mapping, Normal,
                Sorted, Values,
            };
            pub struct DerivedContext {
                discriminant: <compactly::v2::AtMost<0usize> as Encode>::Context,
                __binding_0: <u32 as Encode<Normal>>::Context,
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
            impl Encode<Small> for NewType {
                type Context = <u32 as Encode<Small>>::Context;
                fn encode<E: EntropyCoder>(
                    value: &NewType,
                    writer: &mut E,
                    ctx: &mut Self::Context,
                ) {
                    <u32 as Encode<Small>>::encode(&value.0, writer, ctx)
                }
                fn decode<D: EntropyDecoder>(
                    reader: &mut D,
                    ctx: &mut Self::Context,
                ) -> Result<NewType, std::io::Error> {
                    Ok(NewType(<u32 as Encode<Small>>::decode(reader, ctx)?))
                }
                const MAX_BYTES: usize = <u32 as Encode<Small>>::MAX_BYTES;
                fn decode_awaiting<D: compactly::v2::AsyncEntropyDecoder>(
                    reader: &mut D,
                    ctx: &mut Self::Context,
                ) -> impl ::core::future::Future<Output = Result<NewType, std::io::Error>> {
                    async move {
                        Ok(NewType(<u32 as Encode<Small>>::decode_async(reader, ctx).await?))
                    }
                }
            }
            impl Encode<Sorted> for NewType {
                type Context = <u32 as Encode<Sorted>>::Context;
                fn encode<E: EntropyCoder>(
                    value: &NewType,
                    writer: &mut E,
                    ctx: &mut Self::Context,
                ) {
                    <u32 as Encode<Sorted>>::encode(&value.0, writer, ctx)
                }
                fn decode<D: EntropyDecoder>(
                    reader: &mut D,
                    ctx: &mut Self::Context,
                ) -> Result<NewType, std::io::Error> {
                    Ok(NewType(<u32 as Encode<Sorted>>::decode(reader, ctx)?))
                }
                const MAX_BYTES: usize = <u32 as Encode<Sorted>>::MAX_BYTES;
                fn decode_awaiting<D: compactly::v2::AsyncEntropyDecoder>(
                    reader: &mut D,
                    ctx: &mut Self::Context,
                ) -> impl ::core::future::Future<Output = Result<NewType, std::io::Error>> {
                    async move {
                        Ok(NewType(<u32 as Encode<Sorted>>::decode_async(reader, ctx).await?))
                    }
                }
            }
            impl Encode<Normal> for NewType {
                #![allow(unused_variables, non_shorthand_field_patterns)]
                type Context = DerivedContext;
                fn encode<E: EntropyCoder>(
                    value: &Self,
                    writer: &mut E,
                    ctx: &mut Self::Context,
                ) {
                    match value {
                        NewType(ref __binding_0) => {
                            Normal::encode(
                                &compactly::v2::AtMost::<0usize>::new(0usize),
                                writer,
                                &mut ctx.discriminant,
                            );
                        }
                    }
                    match value {
                        NewType(ref __binding_0) => {
                            <u32 as Encode<
                                Normal,
                            >>::encode(&__binding_0, writer, &mut ctx.__binding_0);
                        }
                    }
                }
                fn decode<D: EntropyDecoder>(
                    reader: &mut D,
                    ctx: &mut Self::Context,
                ) -> Result<Self, std::io::Error> {
                    let discriminant: compactly::v2::AtMost<0usize> = <compactly::v2::AtMost<
                        0usize,
                    > as Encode>::decode(reader, &mut ctx.discriminant)?;
                    Ok(
                        match usize::from(discriminant) {
                            0usize => {
                                NewType(
                                    <u32 as Encode<
                                        Normal,
                                    >>::decode(reader, &mut ctx.__binding_0)?,
                                )
                            }
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
                /// The discriminant, plus the worst variant: within a variant the
                /// fields are coded in sequence so their bounds **sum**, and across
                /// variants only one is ever coded so they **max**. A field whose
                /// strategy declares `usize::MAX` saturates the whole type to
                /// unbounded, which is the safe direction — an unbounded type simply
                /// never takes the async decoder's sync fast path.
                const MAX_BYTES: usize = {
                    let variants: [usize; 1usize] = [
                        0usize.saturating_add(<u32 as Encode<Normal>>::MAX_BYTES),
                    ];
                    let mut worst = 0usize;
                    let mut i = 0;
                    while i < 1usize {
                        if variants[i] > worst {
                            worst = variants[i];
                        }
                        i += 1;
                    }
                    <compactly::v2::AtMost<0usize> as Encode>::MAX_BYTES.saturating_add(worst)
                };
                fn decode_awaiting<D: compactly::v2::AsyncEntropyDecoder>(
                    reader: &mut D,
                    ctx: &mut Self::Context,
                ) -> impl ::core::future::Future<Output = Result<Self, std::io::Error>> {
                    async move {
                        let discriminant: compactly::v2::AtMost<0usize> = <compactly::v2::AtMost<
                            0usize,
                        > as Encode>::decode_async(reader, &mut ctx.discriminant)
                            .await?;
                        Ok(
                            match usize::from(discriminant) {
                                0usize => {
                                    NewType(
                                        <u32 as Encode<
                                            Normal,
                                        >>::decode_async(reader, &mut ctx.__binding_0)
                                            .await?,
                                    )
                                }
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
            use compactly::v2::{Encode, EntropyCoder, EntropyDecoder, Strategy as _};
            use compactly::{
                Small, LowCardinality, Decimal, Compressible, Incompressible, Mapping, Normal,
                Sorted, Values,
            };
            pub struct DerivedContext {
                discriminant: <compactly::v2::AtMost<0usize> as Encode>::Context,
                __binding_0: <u32 as Encode<Normal>>::Context,
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
            impl Encode<Sorted> for NewType {
                type Context = <u32 as Encode<Sorted>>::Context;
                fn encode<E: EntropyCoder>(
                    value: &NewType,
                    writer: &mut E,
                    ctx: &mut Self::Context,
                ) {
                    <u32 as Encode<Sorted>>::encode(&value.0, writer, ctx)
                }
                fn decode<D: EntropyDecoder>(
                    reader: &mut D,
                    ctx: &mut Self::Context,
                ) -> Result<NewType, std::io::Error> {
                    Ok(NewType(<u32 as Encode<Sorted>>::decode(reader, ctx)?))
                }
                const MAX_BYTES: usize = <u32 as Encode<Sorted>>::MAX_BYTES;
                fn decode_awaiting<D: compactly::v2::AsyncEntropyDecoder>(
                    reader: &mut D,
                    ctx: &mut Self::Context,
                ) -> impl ::core::future::Future<Output = Result<NewType, std::io::Error>> {
                    async move {
                        Ok(NewType(<u32 as Encode<Sorted>>::decode_async(reader, ctx).await?))
                    }
                }
            }
            impl Encode<Normal> for NewType {
                #![allow(unused_variables, non_shorthand_field_patterns)]
                type Context = DerivedContext;
                fn encode<E: EntropyCoder>(
                    value: &Self,
                    writer: &mut E,
                    ctx: &mut Self::Context,
                ) {
                    match value {
                        NewType(ref __binding_0) => {
                            Normal::encode(
                                &compactly::v2::AtMost::<0usize>::new(0usize),
                                writer,
                                &mut ctx.discriminant,
                            );
                        }
                    }
                    match value {
                        NewType(ref __binding_0) => {
                            <u32 as Encode<
                                Normal,
                            >>::encode(&__binding_0, writer, &mut ctx.__binding_0);
                        }
                    }
                }
                fn decode<D: EntropyDecoder>(
                    reader: &mut D,
                    ctx: &mut Self::Context,
                ) -> Result<Self, std::io::Error> {
                    let discriminant: compactly::v2::AtMost<0usize> = <compactly::v2::AtMost<
                        0usize,
                    > as Encode>::decode(reader, &mut ctx.discriminant)?;
                    Ok(
                        match usize::from(discriminant) {
                            0usize => {
                                NewType(
                                    <u32 as Encode<
                                        Normal,
                                    >>::decode(reader, &mut ctx.__binding_0)?,
                                )
                            }
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
                /// The discriminant, plus the worst variant: within a variant the
                /// fields are coded in sequence so their bounds **sum**, and across
                /// variants only one is ever coded so they **max**. A field whose
                /// strategy declares `usize::MAX` saturates the whole type to
                /// unbounded, which is the safe direction — an unbounded type simply
                /// never takes the async decoder's sync fast path.
                const MAX_BYTES: usize = {
                    let variants: [usize; 1usize] = [
                        0usize.saturating_add(<u32 as Encode<Normal>>::MAX_BYTES),
                    ];
                    let mut worst = 0usize;
                    let mut i = 0;
                    while i < 1usize {
                        if variants[i] > worst {
                            worst = variants[i];
                        }
                        i += 1;
                    }
                    <compactly::v2::AtMost<0usize> as Encode>::MAX_BYTES.saturating_add(worst)
                };
                fn decode_awaiting<D: compactly::v2::AsyncEntropyDecoder>(
                    reader: &mut D,
                    ctx: &mut Self::Context,
                ) -> impl ::core::future::Future<Output = Result<Self, std::io::Error>> {
                    async move {
                        let discriminant: compactly::v2::AtMost<0usize> = <compactly::v2::AtMost<
                            0usize,
                        > as Encode>::decode_async(reader, &mut ctx.discriminant)
                            .await?;
                        Ok(
                            match usize::from(discriminant) {
                                0usize => {
                                    NewType(
                                        <u32 as Encode<
                                            Normal,
                                        >>::decode_async(reader, &mut ctx.__binding_0)
                                            .await?,
                                    )
                                }
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
            }
        };
    "#]]
    .assert_eq(&pretty(derive_compactly(s)));
}

/// There is only one derive, and it always emits the async-decode members
/// alongside the sync ones, in the same `impl Encode<Normal>` block — no
/// separate twin impl and no opt-out.
#[test]
fn derive_always_includes_async_decode_members() {
    let di: syn::DeriveInput = syn::parse_quote! {
        pub struct NewType(u32);
    };
    let s = synstructure::Structure::new(&di);
    let output = pretty(derive_compactly(s));

    for needle in [
        "pub struct DerivedContext",
        "impl Encode<Normal> for NewType",
        "fn encode<E: EntropyCoder>",
        "fn decode<D: EntropyDecoder>",
        "const MAX_BYTES: usize",
        "fn decode_awaiting<D: compactly::v2::AsyncEntropyDecoder>",
    ] {
        assert!(
            output.contains(needle),
            "derive output is missing {needle:?}:\n{output}"
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
            use compactly::v2::{Encode, EntropyCoder, EntropyDecoder, Strategy as _};
            use compactly::{
                Small, LowCardinality, Decimal, Compressible, Incompressible, Mapping, Normal,
                Sorted, Values,
            };
            pub struct DerivedContext {
                discriminant: <compactly::v2::AtMost<0usize> as Encode>::Context,
                __binding_0: <u32 as Encode<Normal>>::Context,
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
            impl Encode<Normal> for NewType {
                #![allow(unused_variables, non_shorthand_field_patterns)]
                type Context = DerivedContext;
                fn encode<E: EntropyCoder>(
                    value: &Self,
                    writer: &mut E,
                    ctx: &mut Self::Context,
                ) {
                    match value {
                        NewType(ref __binding_0) => {
                            Normal::encode(
                                &compactly::v2::AtMost::<0usize>::new(0usize),
                                writer,
                                &mut ctx.discriminant,
                            );
                        }
                    }
                    match value {
                        NewType(ref __binding_0) => {
                            <u32 as Encode<
                                Normal,
                            >>::encode(&__binding_0, writer, &mut ctx.__binding_0);
                        }
                    }
                }
                fn decode<D: EntropyDecoder>(
                    reader: &mut D,
                    ctx: &mut Self::Context,
                ) -> Result<Self, std::io::Error> {
                    let discriminant: compactly::v2::AtMost<0usize> = <compactly::v2::AtMost<
                        0usize,
                    > as Encode>::decode(reader, &mut ctx.discriminant)?;
                    Ok(
                        match usize::from(discriminant) {
                            0usize => {
                                NewType(
                                    <u32 as Encode<
                                        Normal,
                                    >>::decode(reader, &mut ctx.__binding_0)?,
                                )
                            }
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
                /// The discriminant, plus the worst variant: within a variant the
                /// fields are coded in sequence so their bounds **sum**, and across
                /// variants only one is ever coded so they **max**. A field whose
                /// strategy declares `usize::MAX` saturates the whole type to
                /// unbounded, which is the safe direction — an unbounded type simply
                /// never takes the async decoder's sync fast path.
                const MAX_BYTES: usize = {
                    let variants: [usize; 1usize] = [
                        0usize.saturating_add(<u32 as Encode<Normal>>::MAX_BYTES),
                    ];
                    let mut worst = 0usize;
                    let mut i = 0;
                    while i < 1usize {
                        if variants[i] > worst {
                            worst = variants[i];
                        }
                        i += 1;
                    }
                    <compactly::v2::AtMost<0usize> as Encode>::MAX_BYTES.saturating_add(worst)
                };
                fn decode_awaiting<D: compactly::v2::AsyncEntropyDecoder>(
                    reader: &mut D,
                    ctx: &mut Self::Context,
                ) -> impl ::core::future::Future<Output = Result<Self, std::io::Error>> {
                    async move {
                        let discriminant: compactly::v2::AtMost<0usize> = <compactly::v2::AtMost<
                            0usize,
                        > as Encode>::decode_async(reader, &mut ctx.discriminant)
                            .await?;
                        Ok(
                            match usize::from(discriminant) {
                                0usize => {
                                    NewType(
                                        <u32 as Encode<
                                            Normal,
                                        >>::decode_async(reader, &mut ctx.__binding_0)
                                            .await?,
                                    )
                                }
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
            }
        };
    "#]]
    .assert_eq(&pretty(derive_compactly(s)));
}
