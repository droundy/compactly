use super::{Encode, Strategy};
use crate::Normal;

impl<T: Encode, const N: usize> Encode for [T; N] {
    type Context = T::Context;
    #[inline]
    fn encode<E: super::EntropyCoder>(value: &Self, writer: &mut E, ctx: &mut Self::Context) {
        for v in value {
            Normal::encode(v, writer, ctx);
        }
    }
    #[inline]
    fn decode<D: super::EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        let mut x = Vec::with_capacity(N);
        for _ in 0..N {
            x.push(T::decode(reader, ctx)?);
        }
        x.try_into()
            .map_err(|_| std::io::Error::other("impossible: x should have N values"))
    }

    /// `N` elements and no length — `N` is known at compile time.
    const MAX_BYTES: usize = <T as Encode>::MAX_BYTES.saturating_mul(N);

    #[inline]
    async fn decode_awaiting<D: super::AsyncEntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<[T; N], std::io::Error> {
        let mut x = Vec::with_capacity(N);
        for _ in 0..N {
            x.push(<T as Encode>::decode_async(reader, ctx).await?);
        }
        x.try_into()
            .map_err(|_| std::io::Error::other("impossible: x should have N values"))
    }
}

/// R9: the async per-element loop above had no chunked round-trip.
///
/// `[T; N]` carries no length, so an off-by-one against `N` or a mis-threaded
/// `ctx` desyncs the coder rather than erroring, and shows up as a wrong value
/// *after* the array — hence the trailing sentinel below. One byte per chunk
/// forces a suspension between elements, which is the case the sync fast path
/// would otherwise hide.
#[cfg(all(test, feature = "stream"))]
mod async_tests {
    use crate::v2::stream::tests::Chunks;
    use futures_executor::block_on;

    #[test]
    fn arrays_round_trip_from_a_stream() {
        // The sentinel catches a desync that leaves the array itself correct.
        type Item = ([u64; 5], [bool; 3], [[u8; 2]; 2], u64);
        let value: Item = (
            [0, 1, u64::MAX, 12345, 7],
            [true, false, true],
            [[1, 2], [3, 4]],
            0xdead_beef,
        );

        for chunk_size in [1, 2, 7, 64] {
            let range = crate::v2::Range::encode(&value);
            let decoded: Item = block_on(crate::v2::Range::decode_stream(Chunks::new(
                &range, chunk_size,
            )))
            .unwrap();
            assert_eq!(decoded, value, "Range at chunk_size={chunk_size}");

            let ans = crate::v2::Ans::encode(&value);
            let decoded: Item =
                block_on(crate::v2::Ans::decode_stream(Chunks::new(&ans, chunk_size))).unwrap();
            assert_eq!(decoded, value, "Ans at chunk_size={chunk_size}");
        }
    }

    /// A zero-length array must consume nothing at all — the `N = 0` edge of the
    /// loop, where a stray read would show up in the sentinel.
    #[test]
    fn empty_arrays_consume_nothing() {
        type Item = ([u64; 0], u32);
        let value: Item = ([], 0x1234_5678);
        let encoded = crate::v2::Ans::encode(&value);
        let decoded: Item =
            block_on(crate::v2::Ans::decode_stream(Chunks::new(&encoded, 1))).unwrap();
        assert_eq!(decoded, value);
    }
}
