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
use super::assert_size;

#[test]
fn phantom_data_encoding() {
    assert_size!(PhantomData::<u32>, @"1");
    assert_size!(PhantomData::<String>, @"1");
    assert_size!(PhantomData::<Vec<i32>>, @"1");

    let phantom_vec: Vec<PhantomData<bool>> = vec![PhantomData; 1000];
    assert_size!(phantom_vec, @"3"); // Only the length encoding, no data
}

#[test]
fn phantom_pinned_encoding() {
    assert_size!(PhantomPinned, @"1");

    let encoded = super::super::encode(&PhantomPinned);
    let decoded: Option<PhantomPinned> = super::super::decode(&encoded);
    assert_eq!(decoded, Some(PhantomPinned));
    assert_size!([PhantomPinned; 256], @"1");
}
