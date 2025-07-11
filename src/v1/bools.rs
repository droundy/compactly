use crate::Sorted;

use super::Encode;
use super::{bit_context::BitContext, EncodingStrategy};
use std::io::{Read, Write};

impl Encode for bool {
    type Context = BitContext;
    #[inline]
    fn encode<W: Write>(
        &self,
        writer: &mut super::Writer<W>,
        ctx: &mut Self::Context,
    ) -> Result<(), std::io::Error> {
        // println!("Encoding {self:?}");
        writer.encode(*self, ctx)
    }
    #[inline]
    fn decode<R: Read>(
        reader: &mut super::Reader<R>,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        let b = reader.decode(ctx)?;
        // println!("Decoding {b:?}");
        Ok(b)
    }
    #[inline]
    fn millibits(&self, ctx: &mut Self::Context) -> Option<usize> {
        Some(ctx.millibits_required(*self) as usize)
    }
}

impl EncodingStrategy<bool> for Sorted {
    type Context = BitContext;
    fn decode<R: Read>(
        reader: &mut super::Reader<R>,
        ctx: &mut Self::Context,
    ) -> Result<bool, std::io::Error> {
        bool::decode(reader, ctx)
    }
    fn encode<W: Write>(
        value: &bool,
        writer: &mut super::Writer<W>,
        ctx: &mut Self::Context,
    ) -> Result<(), std::io::Error> {
        value.encode(writer, ctx)
    }
    fn millibits(value: &bool, ctx: &mut Self::Context) -> Option<usize> {
        value.millibits(ctx)
    }
}

#[test]
fn size() {
    use super::assert_bits;
    assert_bits!(true, 1);
    assert_bits!(false, 1);
    assert_bits!([false; 128], 7);
    assert_bits!([true; 2], 1);
    assert_bits!([true; 3], 1);
    assert_bits!([true; 16], 3);
    assert_bits!([true; 64], 5);
    assert_bits!([false, true], 3);
}

#[test]
fn millibits_required() {
    let mut bc = BitContext::default();
    assert_eq!(bc.probability().as_f64(), 0.5);

    assert_eq!(bc.millibits_required(true), 1000);

    assert_eq!(bc, BitContext::True1False0);
    assert!(bc.probability().as_f64() < 0.5);

    assert_eq!(BitContext::True1False0.millibits_required(true), 582);

    assert_eq!(bc.millibits_required(true), 582);
    assert_eq!(bc.millibits_required(true), 415);
    assert_eq!(bc.millibits_required(false), 2327);
    assert_eq!(bc.millibits_required(false), 1590);
}
