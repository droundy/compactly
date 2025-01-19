use cabac::traits::CabacWriter;
use cabac::vp8::{VP8Reader, VP8Writer};
use std::io::{Read, Write};

pub use compactly_derive::Encode;

mod array;
mod bools;
mod byte;
mod ints;
mod maps;
mod sets;
mod tuples;
mod urange;
mod usizes;
mod vecs;

pub use urange::URange;

pub trait Encode: Sized {
    type Context: Default;

    fn encode<W: Write>(
        &self,
        writer: &mut cabac::vp8::VP8Writer<W>,
        ctx: &mut Self::Context,
    ) -> Result<(), std::io::Error>;

    fn decode<R: Read>(
        reader: &mut cabac::vp8::VP8Reader<R>,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error>;
}

pub fn encode<T: Encode>(value: &T) -> Vec<u8> {
    let mut out = Vec::with_capacity(8);
    let mut writer = VP8Writer::new(&mut out).unwrap();
    value
        .encode(&mut writer, &mut T::Context::default())
        .unwrap();
    writer.finish().unwrap();
    out
}

pub fn decode<T: Encode>(mut bytes: &[u8]) -> Option<T> {
    let mut reader = VP8Reader::new(&mut bytes).unwrap();
    T::decode(&mut reader, &mut T::Context::default()).ok()
}

#[cfg(test)]
macro_rules! assert_size {
    ($v:expr, $size:expr) => {
        let v = $v;
        let bytes = crate::encode(&v);
        let decoded = crate::decode(&bytes);
        assert_eq!(decoded, Some(v), "decoded value is incorrect");
        assert_eq!(bytes.len(), $size, "unexpected size");
    };
}
#[cfg(test)]
pub(crate) use assert_size;

#[cfg(test)]
macro_rules! assert_bits {
    ($v:expr, $size:expr) => {
        let v = (
            ($v, $v, $v, $v, $v, $v, $v, $v),
            ($v, $v, $v, $v, $v, $v, $v, $v),
            ($v, $v, $v, $v, $v, $v, $v, $v),
            ($v, $v, $v, $v, $v, $v, $v, $v),
            ($v, $v, $v, $v, $v, $v, $v, $v),
            ($v, $v, $v, $v, $v, $v, $v, $v),
            ($v, $v, $v, $v, $v, $v, $v, $v),
            ($v, $v, $v, $v, $v, $v, $v, $v),
        );
        let bytes = crate::encode(&v);
        let decoded = crate::decode(&bytes);
        assert_eq!(decoded, Some(v), "decoded value is incorrect");
        assert_eq!((bytes.len() + 4) / 8, $size, "unexpected number of bits");
    };
}
#[cfg(test)]
pub(crate) use assert_bits;
