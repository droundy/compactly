use std::collections::{BTreeSet, HashMap};

use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{GenericParam, TraitBound};
use synstructure::{BindingInfo, VariantInfo};

#[derive(Debug, Clone)]
struct EncodingStrategy(syn::Type);
impl EncodingStrategy {
    fn parse(binding: &BindingInfo) -> Option<EncodingStrategy> {
        crate::parse_compactly_attrs(&binding.ast().attrs)
            .single_strategy(binding)
            .map(EncodingStrategy)
    }
}

/// `#[derive(EncodeV1)]`. Like the v2 derive, the expansion names everything by
/// a path from a crate root or by a name chosen not to collide with the user's,
/// and imports nothing; see `v2::derive_compactly` for why.
pub(crate) fn derive_compactly(mut s: synstructure::Structure) -> proc_macro2::TokenStream {
    let mut bound_names = crate::RESERVED_BINDING_NAMES
        .iter()
        .map(|name| Ident::new(name, Span::call_site()))
        .collect::<BTreeSet<_>>();
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
        let type_bounds = context_type_params
            .iter()
            .map(|t| quote! { #t: compactly::v1::Encode });
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
                    #name: <#strategy as compactly::v1::EncodingStrategy<#ty>>::Context
                }
            } else {
                quote! {
                    #name: <#ty as compactly::v1::Encode>::Context
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
                <#strategy as compactly::v1::EncodingStrategy<#ty>>::encode(&#binding, writer, &mut ctx.#binding)?;
            }
        } else {
            quote! {
                <#ty as compactly::v1::Encode>::encode(#binding, writer, &mut ctx.#binding)?;
            }
        }
    });
    let num_variants = s.variants().len();
    let discriminant_type = quote! { compactly::v1::ULessThan<#num_variants> };
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
            <#discriminant_type as compactly::v1::Encode>::encode(&compactly::v1::ULessThan::<#num_variants>::new(#discriminant), writer, &mut ctx.discriminant)?;
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
                    if let Some(Some(strategy)) = binding_strategies.get(&binding.binding) {
                        let strategy = &strategy.0;
                        quote! {
                            <#strategy as compactly::v1::EncodingStrategy<#ty>>::decode(reader, &mut ctx.#binding)?
                        }
                    } else {
                        quote! {
                            <#ty as compactly::v1::Encode>::decode(reader, &mut ctx.#binding)?
                        }
                    }
                })
                .collect::<Vec<_>>();
            variant.construct(|_, i| decoding[i].clone())
        })
        .collect::<Vec<_>>();
    let discriminants = 0..s.variants().len();
    let decode = quote! {
        ::core::result::Result::Ok(match <usize as ::core::convert::From<#discriminant_type>>::from(discriminant) {
            #(#discriminants => #decode_variants,)*
            _ => return ::core::result::Result::Err(::std::io::Error::other("This discriminant should be impossible"))
        })
    };

    // The writer and reader type parameters of the generated methods.
    let sink = crate::fresh_type_param(&s.ast().generics, "W");
    let source = crate::fresh_type_param(&s.ast().generics, "R");

    s.gen_impl(quote! {
        extern crate compactly;

        pub struct DerivedContext #context_generics {
            discriminant: <#discriminant_type as compactly::v1::Encode>::Context,
            #(#context,)*
        }
        impl #context_generics ::core::default::Default for DerivedContext #context_generics_without_bound {
            fn default() -> Self {
                Self {
                    discriminant: ::core::default::Default::default(),
                    #(#bindings: ::core::default::Default::default(),)*
                }
            }
        }
        impl #context_generics ::core::clone::Clone for DerivedContext #context_generics_without_bound {
            fn clone(&self) -> Self {
                Self {
                    discriminant: ::core::clone::Clone::clone(&self.discriminant),
                    #(#bindings: ::core::clone::Clone::clone(&self.#bindings),)*
                }
            }
        }


        gen impl compactly::v1::Encode for @Self {
            #![allow(unused_variables,non_shorthand_field_patterns)]
            type Context = DerivedContext #context_generics_without_bound;
            fn encode<#sink: ::std::io::Write>(
                &self,
                writer: &mut compactly::v1::Writer<#sink>,
                ctx: &mut Self::Context,
            ) -> ::core::result::Result<(), ::std::io::Error> {
                match self { #encode_discriminant }
                match self { #encode_fields }
                ::core::result::Result::Ok(())
            }
            fn decode<#source: ::std::io::Read>(
                reader: &mut compactly::v1::Reader<#source>,
                ctx: &mut Self::Context,
            ) -> ::core::result::Result<Self, ::std::io::Error> {
                let discriminant: #discriminant_type = <#discriminant_type as compactly::v1::Encode>::decode(reader, &mut ctx.discriminant)?;
                #decode
            }
        }
    })
}
