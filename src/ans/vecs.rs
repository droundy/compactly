use super::{Encode, EncodingStrategy};
use crate::{Normal, Small, Sorted};

impl<T: Encode> Encode for Vec<T> {
    type Context = Context<T, Normal>;
    #[inline]
    fn encode<E: super::EntropyCoder>(&self, writer: &mut E, ctx: &mut Self::Context) {
        crate::Values::<Normal>::encode(self, writer, ctx)
    }
    #[inline]
    fn decode<D: super::EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        crate::Values::<Normal>::decode(reader, ctx)
    }
}
#[test]
fn size() {
    use super::assert_bits;
    assert_bits!(Vec::<usize>::new(), 3);
    for value in 0_usize..4 {
        assert_bits!(vec![dbg!(value)], 6);
    }
    assert_bits!(dbg!((0_usize..1).collect::<Vec<_>>()), 6);
    assert_bits!(dbg!((0_usize..2).collect::<Vec<_>>()), 10);
    assert_bits!(dbg!((0_usize..10).collect::<Vec<_>>()), 61);
}

pub struct Context<T, S: EncodingStrategy<T>> {
    len: <Small as EncodingStrategy<usize>>::Context,
    values: S::Context,
}
impl<T, S: EncodingStrategy<T>> Default for Context<T, S> {
    fn default() -> Self {
        Self {
            len: Default::default(),
            values: Default::default(),
        }
    }
}
impl<T, S: EncodingStrategy<T>> Clone for Context<T, S> {
    fn clone(&self) -> Self {
        Self {
            len: self.len.clone(),
            values: self.values.clone(),
        }
    }
}

impl<T, S: EncodingStrategy<T>> EncodingStrategy<Vec<T>> for crate::Values<S> {
    type Context = Context<T, S>;
    fn decode<D: super::EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<Vec<T>, std::io::Error> {
        let n = Small::decode(reader, &mut ctx.len)?;
        let mut x = Vec::with_capacity(n);
        for _ in 0..n {
            x.push(S::decode(reader, &mut ctx.values)?);
        }
        Ok(x)
    }
    fn encode<E: super::EntropyCoder>(value: &Vec<T>, writer: &mut E, ctx: &mut Self::Context) {
        Small::encode(&value.len(), writer, &mut ctx.len);
        for v in value {
            S::encode(v, writer, &mut ctx.values);
        }
    }
}

#[derive(Clone)]
pub struct SortedContext<T: Encode> {
    previous: Vec<T>,
    shared_prefix: <Small as EncodingStrategy<usize>>::Context,
    len: <Small as EncodingStrategy<usize>>::Context,
    value: <T as Encode>::Context,
}
impl<T: Encode> Default for SortedContext<T> {
    fn default() -> Self {
        Self {
            previous: Vec::new(),
            shared_prefix: Default::default(),
            len: Default::default(),
            value: Default::default(),
        }
    }
}

impl<T: Encode + Clone + Eq> EncodingStrategy<Vec<T>> for Sorted {
    type Context = SortedContext<T>;
    fn decode<D: super::EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<Vec<T>, std::io::Error> {
        let len: usize = Small::decode(reader, &mut ctx.len)?;
        let mut out = Vec::new();
        if ctx.previous.is_empty() {
            out.reserve_exact(len);
        } else {
            let shared_prefix: usize = Small::decode(reader, &mut ctx.shared_prefix)?;
            out.reserve_exact(shared_prefix + len);
            debug_assert!(shared_prefix <= ctx.previous.len());
            out.extend_from_slice(&ctx.previous[..shared_prefix]);
        }
        for _ in 0..len {
            out.push(T::decode(reader, &mut ctx.value)?);
        }
        ctx.previous = out.clone();
        Ok(out)
    }
    fn encode<E: super::EntropyCoder>(value: &Vec<T>, writer: &mut E, ctx: &mut Self::Context) {
        if ctx.previous.is_empty() {
            let len = value.len();
            Small::encode(&len, writer, &mut ctx.len);
            for b in value {
                b.encode(writer, &mut ctx.value);
            }
        } else {
            let shared_prefix = value
                .iter()
                .zip(ctx.previous.iter())
                .take_while(|(a, b)| a == b)
                .count();
            let len = value.len() - shared_prefix;
            Small::encode(&len, writer, &mut ctx.len);
            Small::encode(&shared_prefix, writer, &mut ctx.shared_prefix);
            for b in &value[shared_prefix..] {
                b.encode(writer, &mut ctx.value);
            }
        }
        ctx.previous = value.clone();
    }
}
