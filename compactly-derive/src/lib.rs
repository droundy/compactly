#![recursion_limit = "128"]

use proc_macro2::{Ident, Span};
use quote::quote;
use synstructure::{decl_derive, VariantInfo};

decl_derive!([Encode, attributes(compactly_hash)] => derive_compactly);

fn derive_compactly(mut s: synstructure::Structure) -> proc_macro2::TokenStream {
    fn context_type(variant: &VariantInfo) -> Ident {
        proc_macro2::Ident::new(
            &format!("{}Context", variant.ast().ident),
            Span::call_site(),
        )
    }
    fn context_field(variant: &VariantInfo) -> Ident {
        proc_macro2::Ident::new(
            &format!("{}_context", variant.ast().ident),
            Span::call_site(),
        )
    }
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
        .map(|(i, variant)| {
            let decoding = variant
                .bindings()
                .iter()
                .map(|binding| {
                    let ty = &binding.ast().ty;
                    quote! {
                        <#ty as Encode>::decode(reader, &mut ctx.#binding)?
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

        #[derive(Default)]
        struct DerivedContext {
            discriminant: <usize as Encode>::Context,
            #(#context,)*
        }


        gen impl Encode for @Self {
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
                let discriminant = <usize as Encode>::decode(reader, &mut ctx.discriminant)?;
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

                #[derive(Default)]
                struct DerivedContext {
                    discriminant : <usize as Encode>::Context,
                }

                impl Encode for A {
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
                        let discriminant = <usize as Encode>::decode(reader, &mut ctx.discriminant)?;
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

                #[derive(Default)]
                struct DerivedContext {
                    discriminant : <usize as Encode>::Context,
                    __binding_0 : <usize as Encode>::Context,
                }

                impl Encode for A {
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
                        let discriminant = <usize as Encode>::decode(reader, &mut ctx.discriminant)?;
                        Ok (match discriminant {
                            0usize => A (<usize as Encode>::decode(reader, &mut ctx.__binding_0)?,),
                            _ => return Err(std::io::Error::other("This discriminant should be impossible"))
                        })
                    }
                }
            };
        }
    }
}
