#![recursion_limit = "128"]

use std::collections::BTreeSet;

use proc_macro2::{Ident, Span};
use quote::quote;
use synstructure::{decl_derive, VariantInfo};

decl_derive!([Encode, attributes(compactly_hash)] => derive_compactly);

fn derive_compactly(mut s: synstructure::Structure) -> proc_macro2::TokenStream {
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
            let ident = Ident::new(&format!("__binding_{i}"), Span::call_site());
            assert!(!bound_names.contains(&ident));
            bound_names.insert(ident.clone());
            ident
        }
    });
    let context = s
        .variants()
        .iter()
        .flat_map(|variant| {
            variant.bindings().iter().map(|binding| {
                let ty = &binding.ast().ty;
                let name = &binding.binding;
                quote! {
                    #name: <#ty as Encode>::Context
                }
            })
        })
        .collect::<Vec<_>>();
    let bindings = s
        .variants()
        .iter()
        .flat_map(|variant| variant.bindings().iter().map(|binding| &binding.binding))
        .collect::<Vec<_>>();

    let encode_fields = s.each(|binding| {
        let binding = &binding.binding;
        quote! {
            #binding.encode(writer, &mut ctx.#binding)?;
        }
    });
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
            #discriminant.encode(writer, &mut ctx.discriminant)?;
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
                    quote! {
                        Encode::decode(reader, &mut ctx.#binding)?
                    }
                })
                .collect::<Vec<_>>();
            variant.construct(|_, i| decoding[i].clone())
        })
        .collect::<Vec<_>>();
    let discriminants = 0..s.variants().len();
    let decode = quote! {
        Ok(match discriminant {
            #(#discriminants => #decode_variants,)*
            _ => return Err(std::io::Error::other("This discriminant should be impossible"))
        })
    };

    s.gen_impl(quote! {
        extern crate compactly;
        use compactly::Encode;

        pub struct DerivedContext {
            discriminant: <usize as Encode>::Context,
            #(#context,)*
        }
        impl Default for DerivedContext {
            fn default() -> Self {
                Self {
                    discriminant: Default::default(),
                    #(#bindings: Default::default(),)*
                }
            }
        }


        gen impl Encode for @Self {
            #![allow(unused_variables,non_shorthand_field_patterns)]
            type Context = DerivedContext;
            fn encode<W: std::io::Write>(
                &self,
                writer: &mut cabac::vp8::VP8Writer<W>,
                ctx: &mut Self::Context,
            ) -> Result<(), std::io::Error> {
                match self { #encode_discriminant }
                match self { #encode_fields }
                Ok(())
            }
            fn decode<R: std::io::Read>(
                reader: &mut cabac::vp8::VP8Reader<R>,
                ctx: &mut Self::Context,
            ) -> Result<Self, std::io::Error> {
                let discriminant = Encode::decode(reader, &mut ctx.discriminant)?;
                #decode
            }
        }
    })
}

#[test]
fn zero_size() {
    synstructure::test_derive! {
        derive_compactly {
            struct A;
        }
        expands to {
            const _: () = {
                extern crate compactly;
                use compactly::Encode;

                pub struct DerivedContext {
                    discriminant : <usize as Encode>::Context,
                }
                impl Default for DerivedContext {
                    fn default() -> Self {
                        Self {
                            discriminant: Default::default(),
                        }
                    }
                }

                impl Encode for A {
                    #![allow(unused_variables,non_shorthand_field_patterns)]
                    type Context = DerivedContext;
                    fn encode<W: std::io::Write>(
                            &self,
                            writer: &mut cabac::vp8::VP8Writer<W>,
                            ctx: &mut Self::Context,
                        ) -> Result<(), std::io::Error> {
                            match self {
                                A => {
                                    0usize.encode(writer, &mut ctx.discriminant)?;
                                }
                            }
                            match self {
                                A => {}
                            }
                        Ok(())
                    }
                    fn decode<R: std::io::Read>(
                            reader: &mut cabac::vp8::VP8Reader<R>,
                            ctx: &mut Self::Context,
                        ) -> Result<Self, std::io::Error> {
                        let discriminant = Encode::decode(reader, &mut ctx.discriminant)?;
                        Ok(match discriminant {
                            0usize => A,
                            _ => return Err(std::io::Error::other("This discriminant should be impossible"))
                        })
                    }
                }
            };
        }
    }
}

#[test]
fn tuple_struct() {
    synstructure::test_derive! {
        derive_compactly {
            struct A(usize);
        }
        expands to {
            const _: () = {
                extern crate compactly;
                use compactly::Encode;

                pub struct DerivedContext {
                    discriminant : <usize as Encode>::Context,
                    __binding_0 : <usize as Encode>::Context,
                }
                impl Default for DerivedContext {
                    fn default() -> Self {
                        Self {
                            discriminant: Default::default(),
                            __binding_0: Default::default(),
                        }
                    }
                }

                impl Encode for A {
                    #![allow(unused_variables,non_shorthand_field_patterns)]
                    type Context = DerivedContext;
                    fn encode<W: std::io::Write>(
                            &self,
                            writer: &mut cabac::vp8::VP8Writer<W>,
                            ctx: &mut Self::Context,
                        ) -> Result<(), std::io::Error> {
                        match self {
                            A (ref __binding_0, ) => {
                                0usize.encode(writer, &mut ctx.discriminant)?;
                            }
                        }
                        match self {
                            A (ref __binding_0, ) => {
                                {
                                    __binding_0.encode(writer, &mut ctx.__binding_0)?;
                                }
                            }
                        }
                        Ok(())
                    }
                    fn decode<R: std::io::Read>(
                            reader: &mut cabac::vp8::VP8Reader<R>,
                            ctx: &mut Self::Context,
                        ) -> Result<Self, std::io::Error> {
                        let discriminant = Encode::decode(reader, &mut ctx.discriminant)?;
                        Ok (match discriminant {
                            0usize => A (Encode::decode(reader, &mut ctx.__binding_0)?,),
                            _ => return Err(std::io::Error::other("This discriminant should be impossible"))
                        })
                    }
                }
            };
        }
    }
}

#[test]
fn normal_struct() {
    synstructure::test_derive! {
        derive_compactly {
            struct A {
                age: usize,
                dead: bool,
            }
        }
        expands to {
            const _: () = {
                extern crate compactly;
                use compactly::Encode;

                pub struct DerivedContext {
                    discriminant : <usize as Encode>::Context,
                    age: <usize as Encode>::Context,
                    dead: <bool as Encode>::Context,
                }
                impl Default for DerivedContext {
                    fn default() -> Self {
                        Self {
                            discriminant: Default::default(),
                            age: Default::default(),
                            dead: Default::default(),
                        }
                    }
                }

                impl Encode for A {
                    #![allow(unused_variables,non_shorthand_field_patterns)]
                    type Context = DerivedContext;
                    fn encode<W: std::io::Write>(
                            &self,
                            writer: &mut cabac::vp8::VP8Writer<W>,
                            ctx: &mut Self::Context,
                        ) -> Result<(), std::io::Error> {
                        match self {
                            A {
                                age: ref age, dead: ref dead,
                            } => {
                                0usize.encode(writer, &mut ctx.discriminant)?;
                            }
                        }
                        match self {
                            A {age: ref age, dead: ref dead,} => {
                                {
                                    age.encode(writer, &mut ctx.age)?;
                                }
                                {
                                    dead.encode(writer, &mut ctx.dead)?;
                                }
                            }
                        }
                        Ok(())
                    }
                    fn decode<R: std::io::Read>(
                            reader: &mut cabac::vp8::VP8Reader<R>,
                            ctx: &mut Self::Context,
                        ) -> Result<Self, std::io::Error> {
                        let discriminant = Encode::decode(reader, &mut ctx.discriminant)?;
                        Ok (match discriminant {
                            0usize => A {
                                age: Encode::decode(reader, &mut ctx.age)?,
                                dead: Encode::decode(reader, &mut ctx.dead)?,
                            },
                            _ => return Err(std::io::Error::other("This discriminant should be impossible"))
                        })
                    }
                }
            };
        }
    }
}

#[test]
fn an_enum() {
    synstructure::test_derive! {
        derive_compactly {
            enum A {
                A { age: usize },
                B { big: bool },
            }
        }
        expands to {
            const _: () = {
            extern crate compactly;
            use compactly::Encode;
            pub struct DerivedContext {
                discriminant: <usize as Encode>::Context,
                age: <usize as Encode>::Context,
                big: <bool as Encode>::Context,
            }
            impl Default for DerivedContext {
                fn default() -> Self {
                    Self {
                        discriminant: Default::default(),
                        age: Default::default(),
                        big: Default::default(),
                    }
                }
            }
            impl Encode for A {
                #![allow(unused_variables,non_shorthand_field_patterns)]
                type Context = DerivedContext;
                fn encode<W : std::io::Write> (
                    & self, writer: &mut cabac::vp8::VP8Writer<W>, ctx: &mut Self::Context,)
                -> Result<(), std::io::Error> {
                    match self {
                        A::A { age: ref age, } => {
                            0usize.encode(writer, &mut ctx.discriminant)?;
                        }
                        A::B { big: ref big, } => {
                            1usize.encode(writer, &mut ctx.discriminant)?;
                        }
                    }
                    match self {
                        A::A { age: ref age, } => {
                            {
                                age.encode(writer, &mut ctx.age)?;
                            }
                        }
                        A::B { big: ref big, } => {
                            {
                                big.encode(writer, &mut ctx.big)?;
                            }
                        }
                    }
                    Ok (())
                }
                fn decode<R: std::io::Read> (
                    reader: &mut cabac::vp8::VP8Reader<R>,
                    ctx: &mut Self::Context,
                ) -> Result<Self, std::io::Error> {
                    let discriminant = Encode::decode(reader, &mut ctx.discriminant)?;
                    Ok (
                        match discriminant {
                            0usize => A::A {
                                age: Encode::decode(reader, &mut ctx.age)?,
                            },
                            1usize => A::B {
                                big: Encode::decode(reader, &mut ctx.big)?,
                            },
                            _ => return Err (
                                std::io::Error::other ("This discriminant should be impossible"))
                            }
                        )
                    }
                }
            };
        }
    }
}
#[test]
fn generics() {
    synstructure::test_derive! {
        derive_compactly {
            struct A<T> {
                value: T,
            }
        }
        expands to {
            const _: () = {
                extern crate compactly;
                use compactly::Encode;

                pub struct DerivedContext<T: Encode> {
                    discriminant : <usize as Encode>::Context,
                    value: <T as Encode>::Context,
                }
                impl<T: Encode> Default for DerivedContext<T> {
                    fn default() -> Self {
                        Self {
                            discriminant: Default::default(),
                            value: Default::default(),
                        }
                    }
                }

                impl<T: Encode> Encode for A<T> {
                    #![allow(unused_variables,non_shorthand_field_patterns)]
                    type Context = DerivedContext<T>;
                    fn encode<W: std::io::Write>(
                            &self,
                            writer: &mut cabac::vp8::VP8Writer<W>,
                            ctx: &mut Self::Context,
                        ) -> Result<(), std::io::Error> {
                        match self {
                            A {
                                value: ref value,
                            } => {
                                0usize.encode(writer, &mut ctx.discriminant)?;
                            }
                        }
                        match self {
                            A {value: ref value,} => {
                                {
                                    value.encode(writer, &mut ctx.value)?;
                                }
                            }
                        }
                        Ok(())
                    }
                    fn decode<R: std::io::Read>(
                            reader: &mut cabac::vp8::VP8Reader<R>,
                            ctx: &mut Self::Context,
                        ) -> Result<Self, std::io::Error> {
                        let discriminant = Encode::decode(reader, &mut ctx.discriminant)?;
                        Ok (match discriminant {
                            0usize => A {
                                value: Encode::decode(reader, &mut ctx.value)?,
                            },
                            _ => return Err(std::io::Error::other("This discriminant should be impossible"))
                        })
                    }
                }
            };
        }
    }
}
