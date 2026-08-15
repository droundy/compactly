use super::sentinel::Sentinel;
use super::{Encode, EncodeExt, EntropyCoder, EntropyDecoder, Strategy};
use crate::{Incompressible, Normal, Small, Sorted};
use std::collections::VecDeque;

#[cfg(test)]
use expect_test::expect;

impl<T: Encode> Encode for Vec<T> {
    type Context = Context<T, Normal>;
    #[inline]
    fn encode<E: super::EntropyCoder>(value: &Self, writer: &mut E, ctx: &mut Self::Context) {
        crate::Values::<Normal>::encode(value, writer, ctx)
    }
    #[inline]
    fn decode<D: super::EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        crate::Values::<Normal>::decode(reader, ctx)
    }
}

impl<T: Encode> Encode for Box<[T]> {
    type Context = Context<T, Normal>;
    #[inline]
    fn encode<E: super::EntropyCoder>(value: &Self, writer: &mut E, ctx: &mut Self::Context) {
        crate::Values::<Normal>::encode(value, writer, ctx)
    }
    #[inline]
    fn decode<D: super::EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        crate::Values::<Normal>::decode(reader, ctx)
    }
}

impl<T: Encode<S>, S> Encode<crate::Values<S>> for VecDeque<T> {
    type Context = Context<T, S>;
    fn encode<E: EntropyCoder>(value: &VecDeque<T>, writer: &mut E, ctx: &mut Self::Context) {
        Small::encode(&value.len(), writer, &mut ctx.len);
        let mut sentinel = Sentinel::new();
        for v in value {
            sentinel.encode(writer);
            <T as Encode<S>>::encode(v, writer, &mut ctx.values);
        }
    }
    fn decode<D: EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<VecDeque<T>, std::io::Error> {
        let n = Small::decode(reader, &mut ctx.len)?;
        let mut out = VecDeque::with_capacity(super::capacity_for::<T>(n));
        let mut sentinel = Sentinel::new();
        for _ in 0..n {
            sentinel.decode(reader)?;
            out.push_back(<T as Encode<S>>::decode(reader, &mut ctx.values)?);
        }
        Ok(out)
    }
}

impl<T: Encode> Encode for VecDeque<T> {
    type Context = Context<T, Normal>;
    #[inline]
    fn encode<E: EntropyCoder>(value: &Self, writer: &mut E, ctx: &mut Self::Context) {
        crate::Values::<Normal>::encode(value, writer, ctx)
    }
    #[inline]
    fn decode<D: EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        crate::Values::<Normal>::decode(reader, ctx)
    }
}

/// `Box<T>` is transparent to the strategy: the box itself costs nothing on the
/// wire, so whatever strategy is asked for applies to the value inside. Covers
/// `Box<T>` under every strategy `T` supports, including ones from other crates.
///
/// (`Arc<T>`/`Rc<T>` are deliberately *not* transparent — their default encoding
/// keeps a dictionary of repeated values, see [`arc`](super::arc).)
#[diagnostic::do_not_recommend]
impl<T: Encode<S>, S> Encode<S> for Box<T> {
    type Context = <T as Encode<S>>::Context;
    #[inline]
    fn encode<E: EntropyCoder>(value: &Self, writer: &mut E, ctx: &mut Self::Context) {
        <T as Encode<S>>::encode(value, writer, ctx)
    }
    #[inline]
    fn decode<D: EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        <T as Encode<S>>::decode(reader, ctx).map(Box::new)
    }
}
#[test]
fn size() {
    use super::estimated_bits;
    expect!["3"].assert_eq(&estimated_bits!(Vec::<usize>::new()));
    // Unlike most fresh-context codes, `0..4` no longer costs the same
    // number of bits for every value: `usize`'s default `Encode` is now
    // deliberately skewed toward small magnitudes (`UsizeContext` in
    // `usizes.rs` seeds with `SeededDistribution::TinyNumbers`, from
    // `atmost::geometric`), so `vec![0]` should already read cheaper than
    // `vec![3]` even before any adaptation.
    expect!["4"].assert_eq(&estimated_bits!(vec![0_usize]));
    expect!["5"].assert_eq(&estimated_bits!(vec![1_usize]));
    expect!["7"].assert_eq(&estimated_bits!(vec![2_usize]));
    expect!["7"].assert_eq(&estimated_bits!(vec![3_usize]));
    expect!["4"].assert_eq(&estimated_bits!(dbg!((0_usize..1).collect::<Vec<_>>())));
    expect!["8"].assert_eq(&estimated_bits!(dbg!((0_usize..2).collect::<Vec<_>>())));
    expect!["55"].assert_eq(&estimated_bits!(dbg!((0_usize..10).collect::<Vec<_>>())));
}

pub struct Context<T: Encode<S>, S> {
    len: <usize as Encode<Small>>::Context,
    values: <T as Encode<S>>::Context,
}
impl<T: Encode<S>, S> Default for Context<T, S> {
    fn default() -> Self {
        Self {
            len: Default::default(),
            values: Default::default(),
        }
    }
}
impl<T: Encode<S>, S> Clone for Context<T, S> {
    fn clone(&self) -> Self {
        Self {
            len: self.len.clone(),
            values: self.values.clone(),
        }
    }
}

impl<T: Encode<S>, S> Encode<crate::Values<S>> for Vec<T> {
    type Context = Context<T, S>;
    fn decode<D: super::EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<Vec<T>, std::io::Error> {
        let n = Small::decode(reader, &mut ctx.len)?;
        let mut x = Vec::with_capacity(super::capacity_for::<T>(n));
        let mut sentinel = Sentinel::new();
        for _ in 0..n {
            sentinel.decode(reader)?;
            x.push(<T as Encode<S>>::decode(reader, &mut ctx.values)?);
        }
        Ok(x)
    }
    fn encode<E: super::EntropyCoder>(value: &Vec<T>, writer: &mut E, ctx: &mut Self::Context) {
        Small::encode(&value.len(), writer, &mut ctx.len);
        let mut sentinel = Sentinel::new();
        for v in value {
            sentinel.encode(writer);
            <T as Encode<S>>::encode(v, writer, &mut ctx.values);
        }
    }
}

impl<T: Encode<S>, S> Encode<crate::Values<S>> for Box<[T]> {
    type Context = Context<T, S>;
    fn decode<D: super::EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<Box<[T]>, std::io::Error> {
        let n = Small::decode(reader, &mut ctx.len)?;
        let mut x = Vec::with_capacity(super::capacity_for::<T>(n));
        let mut sentinel = Sentinel::new();
        for _ in 0..n {
            sentinel.decode(reader)?;
            x.push(<T as Encode<S>>::decode(reader, &mut ctx.values)?);
        }
        Ok(x.into_boxed_slice())
    }
    fn encode<E: super::EntropyCoder>(value: &Box<[T]>, writer: &mut E, ctx: &mut Self::Context) {
        Small::encode(&value.len(), writer, &mut ctx.len);
        let mut sentinel = Sentinel::new();
        for v in value {
            sentinel.encode(writer);
            <T as Encode<S>>::encode(v, writer, &mut ctx.values);
        }
    }
}

#[derive(Clone)]
pub struct SortedContext<T: Encode> {
    previous: Vec<T>,
    shared_prefix: <usize as Encode<Small>>::Context,
    len: <usize as Encode<Small>>::Context,
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

impl<T: Encode + Clone + Eq> Encode<Sorted> for Vec<T> {
    type Context = SortedContext<T>;
    fn decode<D: super::EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<Vec<T>, std::io::Error> {
        let len: usize = Small::decode(reader, &mut ctx.len)?;
        // Build in place in `ctx.previous` (its buffer is reused across the
        // collection) and return one exact-size clone, instead of copying
        // the shared prefix out and cloning the result back — the same fix
        // that won `Sorted<String>` decode 6-8% (see OPTIMIZING.md).
        if !ctx.previous.is_empty() {
            let shared_prefix: usize = Small::decode(reader, &mut ctx.shared_prefix)?;
            debug_assert!(shared_prefix <= ctx.previous.len());
            ctx.previous.truncate(shared_prefix);
        }
        ctx.previous.reserve(super::capacity_for::<T>(len));
        let mut sentinel = Sentinel::new();
        for _ in 0..len {
            sentinel.decode(reader)?;
            ctx.previous.push(T::decode(reader, &mut ctx.value)?);
        }
        Ok(ctx.previous.clone())
    }
    fn encode<E: super::EntropyCoder>(value: &Vec<T>, writer: &mut E, ctx: &mut Self::Context) {
        if ctx.previous.is_empty() {
            let len = value.len();
            Small::encode(&len, writer, &mut ctx.len);
            let mut sentinel = Sentinel::new();
            for b in value {
                sentinel.encode(writer);
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
            let mut sentinel = Sentinel::new();
            for b in &value[shared_prefix..] {
                sentinel.encode(writer);
                b.encode(writer, &mut ctx.value);
            }
        }
        ctx.previous.clone_from(value);
    }
}

/// Largest incompressible run coded in one call, so a corrupt length cannot
/// force one huge allocation on decode.
const INCOMPRESSIBLE_PIECE: usize = 1 << 16;

/// Split a run of `len` incompressible bytes into the pieces that get coded one
/// per `encode_incompressible_bytes` / `decode_incompressible_bytes` call.
///
/// **Encode and decode must both drive their loop from this**, because coders
/// are free to attach per-call bookkeeping: `Ans` pushes one op per call and
/// consumes one per call, so a side that makes a different number of calls
/// desynchronizes the op stream. That is why an empty run still yields exactly
/// one (zero-length) piece rather than no pieces at all.
///
/// Splitting is invisible to the encoded bytes: `Range` appends each piece to
/// the same withheld slot (no entropy is written between pieces, so the slot
/// cannot advance), and `Ans` concatenates them into the same region.
fn incompressible_pieces(len: usize) -> impl Iterator<Item = usize> {
    let mut remaining = len;
    let mut done = false;
    std::iter::from_fn(move || {
        if done {
            return None;
        }
        let piece = remaining.min(INCOMPRESSIBLE_PIECE);
        remaining -= piece;
        done = remaining == 0;
        Some(piece)
    })
}

impl Encode<Incompressible> for Vec<u8> {
    type Context = <usize as Encode<Small>>::Context;
    fn encode<E: super::EntropyCoder>(value: &Vec<u8>, writer: &mut E, ctx: &mut Self::Context) {
        Small::encode(&value.len(), writer, ctx);
        let mut start = 0;
        for piece in incompressible_pieces(value.len()) {
            writer.encode_incompressible_bytes(&value[start..start + piece]);
            start += piece;
        }
    }
    fn decode<D: super::EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<Vec<u8>, std::io::Error> {
        // Grow in bounded pieces rather than one `vec![0; len]`: a corrupt or
        // truncated stream can decode an absurd `len`, and allocating it whole
        // would panic (capacity overflow) or speculatively allocate gigabytes
        // before `decode_incompressible_bytes` can report the stream is short.
        let len: usize = Small::decode(reader, ctx)?;
        let mut out = Vec::new();
        for piece in incompressible_pieces(len) {
            let start = out.len();
            out.resize(start + piece, 0);
            reader.decode_incompressible_bytes(&mut out[start..])?;
        }
        Ok(out)
    }
}

/// An empty run must still be one piece, and the pieces must sum to `len` — the
/// encode and decode loops are only in lockstep if both hold.
#[test]
fn incompressible_pieces_are_symmetric() {
    for len in [0, 1, 100, INCOMPRESSIBLE_PIECE - 1, INCOMPRESSIBLE_PIECE] {
        let pieces: Vec<usize> = incompressible_pieces(len).collect();
        assert_eq!(pieces.len(), 1, "len {len} should be one piece");
        assert_eq!(pieces[0], len);
    }
    for len in [INCOMPRESSIBLE_PIECE + 1, 3 * INCOMPRESSIBLE_PIECE + 7] {
        let pieces: Vec<usize> = incompressible_pieces(len).collect();
        assert_eq!(pieces.len(), len.div_ceil(INCOMPRESSIBLE_PIECE));
        assert_eq!(pieces.iter().sum::<usize>(), len);
        assert!(pieces.iter().all(|&p| p <= INCOMPRESSIBLE_PIECE));
    }
}

/// `incompressible_pieces`'s doc claims splitting a run is invisible to
/// `Range`'s output, which is what lets the encode side chunk without changing
/// the format. Pin it: coding one run as several calls must be byte-identical to
/// coding it as one.
#[test]
fn splitting_a_range_run_is_byte_identical() {
    use super::{EntropyCoder, Range};
    let bytes: Vec<u8> = (0..3 * INCOMPRESSIBLE_PIECE + 13)
        .map(|i| (i * 31 + i / 7) as u8)
        .collect();

    let mut whole = Range::default();
    whole.encode_bit(&mut Default::default(), true);
    whole.encode_incompressible_bytes(&bytes);
    whole.encode_bit(&mut Default::default(), false);

    let mut split = Range::default();
    split.encode_bit(&mut Default::default(), true);
    let mut start = 0;
    for piece in incompressible_pieces(bytes.len()) {
        split.encode_incompressible_bytes(&bytes[start..start + piece]);
        start += piece;
    }
    split.encode_bit(&mut Default::default(), false);

    assert_eq!(whole.into_vec(), split.into_vec());
}

/// A run spanning several pieces must round-trip through both coders, and the
/// bytes must not depend on how the run was split.
#[test]
fn multi_piece_incompressible_round_trips() {
    use crate::Encoded;
    for len in [
        0usize,
        1,
        INCOMPRESSIBLE_PIECE,
        2 * INCOMPRESSIBLE_PIECE + 13,
    ] {
        let bytes: Vec<u8> = (0..len).map(|i| (i * 7 + i / 251) as u8).collect();
        let v = Encoded::<Vec<u8>, Incompressible>::new(bytes);
        let encoded = super::encode(&v);
        assert_eq!(super::decode(&encoded).as_ref(), Some(&v), "len {len}");
    }
}
