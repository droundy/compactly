#![recursion_limit = "128"]

use quote::quote;
use synstructure::decl_derive;

decl_derive!([Encode, attributes(compactly_hash)] => derive_compactly);

fn derive_compactly(mut s: synstructure::Structure) -> proc_macro2::TokenStream {
    let context = quote! {
        ()
    };

    let encode = quote! {};

    s.bind_with(|_| synstructure::BindStyle::RefMut);

    let decode = quote! { Ok(Self) };

    s.gen_impl(quote! {
        extern crate compactly;
        gen impl compactly::Encode for @Self {
            type Context = #context;
            fn encode<W: std::io::Write>(
                &self,
                writer: &mut cabac::vp8::VP8Writer<W>,
                ctx: &mut Self::Context,
            ) -> Result<(), std::io::Error> {
                #encode
                Ok(())
            }
            fn decode<R: std::io::Read>(
                reader: &mut cabac::vp8::VP8Reader<R>,
                ctx: &mut Self::Context,
            ) -> Result<Self, std::io::Error> {
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
                impl compactly::Encode for A {
                    type Context = ();
                    fn encode<W: std::io::Write>(
                            &self,
                            writer: &mut cabac::vp8::VP8Writer<W>,
                            ctx: &mut Self::Context,
                        ) -> Result<(), std::io::Error> {
                        Ok(())
                    }
                    fn decode<R: std::io::Read>(
                            reader: &mut cabac::vp8::VP8Reader<R>,
                            ctx: &mut Self::Context,
                        ) -> Result<Self, std::io::Error> {
                        Ok(Self)
                    }
                }
            };
        }
    }
}
