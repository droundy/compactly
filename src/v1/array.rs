use super::Encode;
use std::io::{Read, Write};

impl<T: Encode, const N: usize> Encode for [T; N] {
    type Context = T::Context;
    fn encode<W: Write>(
        &self,
        writer: &mut super::Writer<W>,
        ctx: &mut Self::Context,
    ) -> Result<(), std::io::Error> {
        for v in self {
            v.encode(writer, ctx)?;
        }
        Ok(())
    }
    fn decode<R: Read>(
        reader: &mut super::Reader<R>,
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
