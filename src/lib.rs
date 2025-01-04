use std::io::{Read, Write};

use cabac::{
    traits::{CabacReader, CabacWriter},
    vp8::{VP8Context, VP8Reader, VP8Writer},
};

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

impl Encode for bool {
    type Context = VP8Context;
    fn encode<W: Write>(
        &self,
        writer: &mut cabac::vp8::VP8Writer<W>,
        ctx: &mut Self::Context,
    ) -> Result<(), std::io::Error> {
        writer.put(*self, ctx)
    }
    fn decode<R: Read>(
        reader: &mut cabac::vp8::VP8Reader<R>,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        reader.get(ctx)
    }
}

impl<T: Encode, const N: usize> Encode for [T; N] {
    type Context = T::Context;
    fn encode<W: Write>(
        &self,
        writer: &mut cabac::vp8::VP8Writer<W>,
        ctx: &mut Self::Context,
    ) -> Result<(), std::io::Error> {
        for v in self {
            v.encode(writer, ctx)?;
        }
        Ok(())
    }
    fn decode<R: Read>(
        reader: &mut cabac::vp8::VP8Reader<R>,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        let mut x = Vec::with_capacity(N);
        for _ in 0..N {
            x.push(T::decode(reader, ctx)?);
        }
        x.try_into()
            .map_err(|_| std::io::Error::other("impossible: x should have N values"))
    }
}

#[cfg(test)]
macro_rules! assert_size {
    ($v:expr, $size:literal) => {
        let v = $v;
        let bytes = crate::encode(&v);
        println!("bytes are {bytes:?}");
        let decoded = crate::decode(&bytes);
        assert_eq!(decoded, Some(v));
        assert_eq!(bytes.len(), $size);
    };
}

#[test]
fn bools_size() {
    assert_size!(true, 1);
    assert_size!(false, 0);
    assert_size!([false; 128], 0);
    assert_size!([true; 2], 2);
    assert_size!([true; 7], 2);
    assert_size!([true; 16], 2);
    assert_size!([true; 64], 2);
    assert_size!([false, true], 2);
}
