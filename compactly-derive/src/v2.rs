use std::collections::{BTreeSet, HashMap};

use proc_macro2::{Ident, Span};
use quote::{quote, ToTokens};
use syn::{Attribute, GenericParam, TraitBound};
use synstructure::{BindingInfo, VariantInfo};

#[derive(Debug, Clone)]
struct EncodingStrategy(syn::Type);
impl EncodingStrategy {
    fn parse_attrs(attrs: &[Attribute]) -> Vec<EncodingStrategy> {
        let mut strategies: Vec<EncodingStrategy> = Vec::new();
        for a in attrs {
            if a.path().is_ident("compactly") {
                a.parse_nested_meta(|meta| {
                    strategies.push(EncodingStrategy(syn::Type::Path(
                        syn::TypePath {
                            qself: None,
                            path: meta.path,
                        },
                    )));
                    Ok(())
                })
                .expect("compactly wants a list of strategies: {a}");
            }
        }
        strategies
    }
    fn parse(binding: &BindingInfo) -> Option<EncodingStrategy> {
        match Self::parse_attrs(&binding.ast().attrs).as_slice() {
            [] => None,
            [s] => Some(s.clone()),
            _ => panic!(
                "Cannot support multiple encoding strategies: {binding:?}"
            ),
        }
    }
}

pub(crate) fn derive_compactly(
    mut s: synstructure::Structure,
) -> proc_macro2::TokenStream {
    let mut bound_names = BTreeSet::new();
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
    let (_impl_generics, _ty_generics, where_clause) =
        s.ast().generics.split_for_impl();
    let mut where_clause = where_clause.cloned();
    s.add_trait_bounds(
        &encode_trait,
        &mut where_clause,
        synstructure::AddBounds::Generics,
    );

    let context_types = s
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
    let context_generics = if context_types.is_empty() {
        quote! {}
    } else {
        quote! { <#(#context_types: Encode),*> }
    };
    let context_generics_without_bound = if context_types.is_empty() {
        quote! {}
    } else {
        quote! { <#(#context_types),*> }
    };
    let mut binding_strategies: HashMap<Ident, Option<EncodingStrategy>> =
        HashMap::new();
    let mut strategies = Vec::new();
    for binding in
        s.variants().iter().flat_map(|variant| variant.bindings().iter())
    {
        let strategy = EncodingStrategy::parse(binding);
        strategies.push(strategy.clone());
        binding_strategies.insert(binding.binding.clone(), strategy);
    }
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
        .flat_map(|variant| {
            variant.bindings().iter().map(|binding| &binding.binding)
        })
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
    let discriminant_type = quote! { compactly::v2::ULessThan<#num_variants> };
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
            compactly::v2::ULessThan::<#num_variants>::new(#discriminant).encode(writer, &mut ctx.discriminant);
        }
    });

    let decode_variants = s
        .variants()
        .iter()
        .enumerate()
        .map(|(_, variant)| {
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

    s.gen_impl(quote! {
        extern crate compactly;
        use compactly::v2::{Encode, EncodingStrategy, EntropyCoder, EntropyDecoder};
        use compactly::{Small, LowCardinality, Decimal, Compressible, Incompressible, Mapping, Normal, Sorted, Values};

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
            use compactly::v2::{Encode, EncodingStrategy, EntropyCoder, EntropyDecoder};
            use compactly::{
                Small, LowCardinality, Decimal, Compressible, Incompressible, Mapping, Normal,
                Sorted, Values,
            };
            pub struct DerivedContext {
                discriminant: <compactly::v2::ULessThan<1usize> as Encode>::Context,
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
                            compactly::v2::ULessThan::<1usize>::new(0usize)
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
                    let discriminant: compactly::v2::ULessThan<1usize> = Encode::decode(
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
            use compactly::v2::{Encode, EncodingStrategy, EntropyCoder, EntropyDecoder};
            use compactly::{
                Small, LowCardinality, Decimal, Compressible, Incompressible, Mapping, Normal,
                Sorted, Values,
            };
            pub struct DerivedContext {
                discriminant: <compactly::v2::ULessThan<1usize> as Encode>::Context,
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
                            compactly::v2::ULessThan::<1usize>::new(0usize)
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
                    let discriminant: compactly::v2::ULessThan<1usize> = Encode::decode(
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
fn impl_newtype() {
    let di: syn::DeriveInput = syn::parse_quote! {
        pub struct NewType(u32);
    };
    let s = synstructure::Structure::new(&di);

    expect_test::expect![[r#"
        const _: () = {
            extern crate compactly;
            use compactly::v2::{Encode, EncodingStrategy, EntropyCoder, EntropyDecoder};
            use compactly::{
                Small, LowCardinality, Decimal, Compressible, Incompressible, Mapping, Normal,
                Sorted, Values,
            };
            pub struct DerivedContext {
                discriminant: <compactly::v2::ULessThan<1usize> as Encode>::Context,
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
                            compactly::v2::ULessThan::<1usize>::new(0usize)
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
                    let discriminant: compactly::v2::ULessThan<1usize> = Encode::decode(
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
