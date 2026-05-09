use std::{
    io::{Read, Write},
    marker::{PhantomData, PhantomPinned},
};

use super::Encode;

impl<T> Encode for PhantomData<T> {
    type Context = ();

    #[inline]
    fn encode<W: Write>(
        &self,
        _writer: &mut super::Writer<W>,
        _ctx: &mut Self::Context,
    ) -> Result<(), std::io::Error> {
        Ok(())
    }

    #[inline]
    fn decode<R: Read>(
        _reader: &mut super::Reader<R>,
        _ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        Ok(PhantomData)
    }

    #[inline]
    fn millibits(&self, _ctx: &mut Self::Context) -> Option<usize> {
        Some(0)
    }
}

impl Encode for PhantomPinned {
    type Context = ();

    #[inline]
    fn encode<W: Write>(
        &self,
        _writer: &mut super::Writer<W>,
        _ctx: &mut Self::Context,
    ) -> Result<(), std::io::Error> {
        Ok(())
    }

    #[inline]
    fn decode<R: Read>(
        _reader: &mut super::Reader<R>,
        _ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        Ok(PhantomPinned)
    }

    #[inline]
    fn millibits(&self, _ctx: &mut Self::Context) -> Option<usize> {
        Some(0)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn phantom_data_encoding() {
        assert_bits!(PhantomData::<u32>, 0);
        assert_bits!(PhantomData::<String>, 0);
        assert_bits!(PhantomData::<Vec<i32>>, 0);

        let phantom_vec: Vec<PhantomData<bool>> = vec![PhantomData; 1000];
        assert_bits!(phantom_vec, 3); // Only the length encoding, no data
    }

    #[test]
    fn phantom_pinned_encoding() {
        assert_bits!(PhantomPinned, 0);

        let encoded = super::super::encode(&PhantomPinned);
        let decoded: Option<PhantomPinned> = super::super::decode(&encoded);
        assert_eq!(decoded, Some(PhantomPinned));

        // Arrays of PhantomPinned only encode length
        assert_bits!([PhantomPinned; 256], 8);
    }
}
