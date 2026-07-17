use super::atmost::geometric::{bl_offset_seeded, blbl_tree_seeded, geometric_seeded};
use super::atmost::{AtMost, AtMostContext};
use super::bit_context::BitContext;
use super::{Encode, EncodingStrategy, EntropyCoder, EntropyDecoder, Small};
use crate::{Incompressible, Sorted};

#[cfg(test)]
use expect_test::expect;

macro_rules! impl_uint {
    ($t:ident, $mod:ident, $bits:literal) => {
        mod $mod {
            use super::*;

            #[derive(Default, Clone)]
            pub struct SortedContext {
                previous: Option<$t>,
                not_sorted: <bool as Encode>::Context,
                value: <Small as EncodingStrategy<$t>>::Context,
                difference: <Small as EncodingStrategy<$t>>::Context,
            }

            impl EncodingStrategy<$t> for Sorted {
                type Context = SortedContext;
                fn encode<E: EntropyCoder>(value: &$t, writer: &mut E, ctx: &mut Self::Context) {
                    if let Some(previous) = ctx.previous.take() {
                        let not_sorted = *value < previous;
                        not_sorted.encode(writer, &mut ctx.not_sorted);
                        if not_sorted {
                            Small::encode(value, writer, &mut ctx.value);
                        } else {
                            Small::encode(&(*value - previous), writer, &mut ctx.difference);
                        }
                    } else {
                        Small::encode(value, writer, &mut ctx.value);
                    }
                    ctx.previous = Some(*value);
                }
                fn decode<D: EntropyDecoder>(
                    reader: &mut D,
                    ctx: &mut Self::Context,
                ) -> Result<$t, std::io::Error> {
                    let out = if let Some(previous) = ctx.previous.take() {
                        let not_sorted = bool::decode(reader, &mut ctx.not_sorted)?;
                        if not_sorted {
                            Small::decode(reader, &mut ctx.value)?
                        } else {
                            previous
                                + <Small as EncodingStrategy<$t>>::decode(
                                    reader,
                                    &mut ctx.difference,
                                )?
                        }
                    } else {
                        Small::decode(reader, &mut ctx.value)?
                    };
                    ctx.previous = Some(out);
                    Ok(out)
                }
            }

            impl EncodingStrategy<$t> for Incompressible {
                type Context = ();
                fn encode<E: EntropyCoder>(value: &$t, writer: &mut E, _ctx: &mut Self::Context) {
                    writer.encode_incompressible_bytes(&value.to_le_bytes())
                }
                fn decode<D: EntropyDecoder>(
                    reader: &mut D,
                    _ctx: &mut Self::Context,
                ) -> Result<$t, std::io::Error> {
                    let mut b = [0; std::mem::size_of::<$t>()];
                    reader.decode_incompressible_bytes(&mut b)?;
                    Ok($t::from_le_bytes(b))
                }
            }
        }
    };
}
impl_uint!(u128, u128_mod, 128);
impl_uint!(u64, u64_mod, 64);
impl_uint!(u32, u32_mod, 32);
impl_uint!(u16, u16_mod, 16);

// The default integer `Encode` is variable-length (`Small`'s hierarchical
// bit-length algorithm under a uniform-value prior), so a fresh context
// costs ~`bits` bits for an arbitrary/incompressible value but far fewer
// for small values — unlike the old fixed per-bit-position encoding, where
// every value in a range cost the same. These tests therefore probe
// individual representative values rather than asserting a whole range
// shares one size.

#[test]
fn size_u64() {
    use super::estimated_bits;
    expect!["6"].assert_eq(&estimated_bits!(0_u64));
    expect!["6"].assert_eq(&estimated_bits!(1_u64));
    expect!["19"].assert_eq(&estimated_bits!(255_u64));
    expect!["19"].assert_eq(&estimated_bits!(256_u64));
    expect!["29"].assert_eq(&estimated_bits!(1_000_000_u64));
    expect!["65"].assert_eq(&estimated_bits!(u64::MAX));
    expect!["59"].assert_eq(&estimated_bits!([0_u64; 128]));
    expect!["10"].assert_eq(&estimated_bits!([1_u64; 2]));
    expect!["35"].assert_eq(&estimated_bits!([1_u64; 19]));
    expect!["44"].assert_eq(&estimated_bits!([
        0_u64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1
    ]));
}

#[test]
fn size_u32() {
    use super::estimated_bits;
    expect!["4"].assert_eq(&estimated_bits!(0_u32));
    expect!["4"].assert_eq(&estimated_bits!(1_u32));
    expect!["17"].assert_eq(&estimated_bits!(255_u32));
    expect!["17"].assert_eq(&estimated_bits!(256_u32));
    expect!["28"].assert_eq(&estimated_bits!(1_000_000_u32));
    expect!["33"].assert_eq(&estimated_bits!(u32::MAX));
    expect!["33"].assert_eq(&estimated_bits!([0_u32; 128]));
    expect!["3139"].assert_eq(&estimated_bits!([u32::MAX; 128]));
    expect!["6"].assert_eq(&estimated_bits!([1_u32; 2]));
    expect!["20"].assert_eq(&estimated_bits!([1_u32; 19]));
    expect!["27"].assert_eq(&estimated_bits!([
        0_u32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1
    ]));
}

#[test]
fn size_u16() {
    use super::estimated_bits;
    expect!["8"].assert_eq(&estimated_bits!(0_u16));
    expect!["8"].assert_eq(&estimated_bits!(1_u16));
    expect!["14"].assert_eq(&estimated_bits!(255_u16));
    expect!["14"].assert_eq(&estimated_bits!(256_u16));
    expect!["17"].assert_eq(&estimated_bits!(u16::MAX));
    expect!["77"].assert_eq(&estimated_bits!([0_u16; 128]));
    expect!["1095"].assert_eq(&estimated_bits!([u16::MAX; 128]));
    expect!["14"].assert_eq(&estimated_bits!([1_u16; 2]));
    expect!["46"].assert_eq(&estimated_bits!([1_u16; 19]));
    expect!["56"].assert_eq(&estimated_bits!([
        0_u16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1
    ]));
}

#[test]
fn size_u128() {
    use super::estimated_bits;
    expect!["6"].assert_eq(&estimated_bits!(0_u128));
    expect!["6"].assert_eq(&estimated_bits!(1_u128));
    expect!["19"].assert_eq(&estimated_bits!(255_u128));
    expect!["129"].assert_eq(&estimated_bits!(u128::MAX));
    expect!["35"].assert_eq(&estimated_bits!([1_u128; 19]));
}

#[test]
fn default_encoding_roundtrips_every_leading_zero_depth() {
    // `compact_u32`'s dense `0..4096` loop only compares two encode paths
    // (it never decodes) and only touches leading-zero counts near the
    // top of the range. Round-trip representative values per possible
    // `leading_zeros()` count instead, through the actual default `Encode`
    // (`super::encode`/`super::decode`, not `Encoded<_, Small>`) — this
    // touches every bit length, and through it every `blbl` bucket and
    // every `bl`-mantissa width the hierarchical encoding produces, so a
    // bug isolated to one bucket can't hide behind the point-probe tests.
    // At each depth, check all-zero, all-one, and alternating mantissas
    // (not just the all-zero minimum of that depth's range) so a bug
    // isolated to one mantissa bit position — e.g. an off-by-one in the
    // `partial[lz][i]` loop or a byte-boundary mistake in
    // `value_bytes[full_bytes]` — can't hide behind an all-zero payload.
    macro_rules! check {
        ($t:ty, $bits:literal) => {
            for lz in 0..=$bits {
                if lz == $bits {
                    assert_eq!(super::decode::<$t>(&super::encode(&(0 as $t))), Some(0));
                    continue;
                }
                let msb_pos = $bits - 1 - lz;
                let leading: $t = 1 << msb_pos;
                let mantissa_mask: $t = if msb_pos == 0 { 0 } else { (1 << msb_pos) - 1 };
                for mantissa in [
                    0,
                    mantissa_mask,
                    0xAAAA_AAAA_AAAA_AAAA_AAAA_AAAA_AAAA_AAAA_u128 as $t & mantissa_mask,
                ] {
                    let v = leading | mantissa;
                    assert_eq!(
                        super::decode::<$t>(&super::encode(&v)),
                        Some(v),
                        "{}::leading_zeros() == {lz}, mantissa = {mantissa:#x}",
                        stringify!($t),
                    );
                }
            }
        };
    }
    check!(u16, 16);
    check!(u32, 32);
    check!(u64, 64);
    check!(u128, 128);
}

// The shared variable-length integer encoding, Elias-delta style: code the
// value's *bit length* `bl` (in `0..=$bits`) hierarchically, then the
// value's mantissa below its implicit top bit.
//
// - `blbl = bit_length_of(bl)` goes through one `AtMost<$blbl_max>` tree
//   symbol (`$blbl_max + 1` codes). `blbl <= 1` pins `bl = blbl` (so the
//   values 0 and 1 finish after this single shallow symbol — the common
//   case for `usize` lengths/counts), and the top code pins `bl = $bits`
//   exactly ($bits is a power of two, the only valid bit length that
//   wide), so neither end needs an offset.
// - Otherwise `bl`'s offset within its bucket (`bl - 2^(blbl-1)`, one of
//   `2^(blbl-1)` values) goes through that bucket's own complete
//   `AtMost` tree as a second symbol — one coder step, not per-bit ops:
//   the per-bit variant measured 20-40% slower encode on mid/large
//   values (5 extra buffered ops per value on the Ans coder).
// - Then the value's `bl - 1` mantissa bits: full bytes as incompressible
//   bytes, the partial top byte as adaptive bits with per-`(lz, position)`
//   contexts.
//
// Versus coding the leading-zero count directly through one deep
// `AtMost<$bits - 1>` tree (the previous scheme), the path for tiny values
// shrinks from `log2($bits)` adaptive decisions to `log2($blbl_max + 1)`
// (6 → 3 for u64) — which halves both the fully-adapted floor (~11.3
// millibits per decision, `BitContext`'s 254/256 probability cap) and the
// fresh-context minimum (~0.26 bits per decision, `seed_context`'s
// 4-observation cap) on the values that dominate real `usize` data.
// Mid-size values instead pay one extra (shallow) coder step.
//
// `u16` deliberately does NOT use this macro: its old single-tree scheme
// (below the invocations) is a *complete* power-of-two `AtMost<15>` that
// takes the fast speculating walk in one coder step, and the hierarchy's
// two-symbol split measured 9-20% slower decode there while the shallow
// tree left little floor to win back. The wider types' trees are deep
// enough (and, for the uneven u128 count, slow enough) that the hierarchy
// wins.
macro_rules! impl_compact {
    ($t:ident, $context:ident, $default_context:ident, $bits:literal, $blbl_max:literal) => {
        // $blbl_max must be the bit length of $bits itself, i.e. the
        // largest possible `blbl` code.
        const _: () = assert!((1usize << ($blbl_max - 1)) == $bits);

        #[derive(Clone)]
        pub struct $context {
            /// One tree symbol for `blbl` in `0..=$blbl_max`.
            blbl: <AtMost<$blbl_max> as Encode>::Context,
            /// Per-bucket offset trees: bucket `c` holds bit lengths
            /// `2^(c-1)..2^c`, and `o<c>` codes `bl - 2^(c-1)`. Buckets
            /// past this type's width exist as fields (the macro emits one
            /// fixed shape) but are never touched by `encode`/`decode`.
            o2: <AtMost<1> as Encode>::Context,
            o3: <AtMost<3> as Encode>::Context,
            o4: <AtMost<7> as Encode>::Context,
            o5: <AtMost<15> as Encode>::Context,
            o6: <AtMost<31> as Encode>::Context,
            o7: <AtMost<63> as Encode>::Context,
            /// `partial[lz][i]`: context for bit `i` of the value's partial
            /// top byte, given `lz = $bits - bl` leading zeros. Only
            /// `(bl - 1) % 8` bits are used per row.
            partial: [[<bool as Encode>::Context; 8]; $bits],
        }
        impl Default for $context {
            fn default() -> Self {
                Self {
                    blbl: Default::default(),
                    o2: Default::default(),
                    o3: Default::default(),
                    o4: Default::default(),
                    o5: Default::default(),
                    o6: Default::default(),
                    o7: Default::default(),
                    partial: [[Default::default(); 8]; $bits],
                }
            }
        }

        impl $context {
            /// A context seeded from one of the two priors in
            /// `atmost::geometric`: `mirror = false` matches a
            /// uniformly-random `$t` (large magnitudes dominant — the
            /// default `Encode`'s prior), `mirror = true` reverses it so
            /// tiny magnitudes dominate (`usize`'s prior; see
            /// `usizes.rs`). `Small` itself uses the flat `Default`
            /// instead. Impossible buckets seed flat (see
            /// `bl_offset_seeded`), so one fixed shape serves every width.
            pub(crate) const fn seeded(mirror: bool) -> Self {
                Self::seeded_capped(mirror, $bits)
            }

            /// [`Self::seeded`] with the prior capped at bit length
            /// `max_bl`: bit lengths past it get zero prior weight. The
            /// signed default `Encode` (`impl_signed!` below) codes its
            /// magnitude — a uniform `($bits - 1)`-bit value — through
            /// this very context type, so its prior caps one below the
            /// width.
            pub(crate) const fn seeded_capped(mirror: bool, max_bl: usize) -> Self {
                Self {
                    blbl: AtMostContext {
                        bits: blbl_tree_seeded::<$blbl_max>(mirror, max_bl),
                    },
                    o2: AtMostContext {
                        bits: bl_offset_seeded::<1>(mirror, max_bl),
                    },
                    o3: AtMostContext {
                        bits: bl_offset_seeded::<3>(mirror, max_bl),
                    },
                    o4: AtMostContext {
                        bits: bl_offset_seeded::<7>(mirror, max_bl),
                    },
                    o5: AtMostContext {
                        bits: bl_offset_seeded::<15>(mirror, max_bl),
                    },
                    o6: AtMostContext {
                        bits: bl_offset_seeded::<31>(mirror, max_bl),
                    },
                    o7: AtMostContext {
                        bits: bl_offset_seeded::<63>(mirror, max_bl),
                    },
                    partial: [[BitContext::True0False0; 8]; $bits],
                }
            }
        }

        impl EncodingStrategy<$t> for Small {
            type Context = $context;
            #[inline]
            fn encode<E: EntropyCoder>(value: &$t, writer: &mut E, ctx: &mut Self::Context) {
                let bl = ($bits - value.leading_zeros()) as usize;
                let blbl = if bl == 0 { 0 } else { bl.ilog2() as usize + 1 };
                AtMost::<$blbl_max>::new(blbl).encode(writer, &mut ctx.blbl);
                if (2..$blbl_max).contains(&blbl) {
                    let offset = bl - (1 << (blbl - 1));
                    match blbl {
                        2 => AtMost::<1>::new(offset).encode(writer, &mut ctx.o2),
                        3 => AtMost::<3>::new(offset).encode(writer, &mut ctx.o3),
                        4 => AtMost::<7>::new(offset).encode(writer, &mut ctx.o4),
                        5 => AtMost::<15>::new(offset).encode(writer, &mut ctx.o5),
                        6 => AtMost::<31>::new(offset).encode(writer, &mut ctx.o6),
                        _ => AtMost::<63>::new(offset).encode(writer, &mut ctx.o7),
                    }
                }
                if bl < 2 {
                    return;
                }
                let lz = $bits - bl;
                let sig_bits = bl - 1;
                let full_bytes = sig_bits / 8;
                let partial_bits = sig_bits % 8;
                let value_bytes = value.to_le_bytes();
                if full_bytes > 0 {
                    writer.encode_incompressible_bytes(&value_bytes[..full_bytes]);
                }
                for i in 0..partial_bits {
                    let bit = (value_bytes[full_bytes] >> i) & 1 == 1;
                    bit.encode(writer, &mut ctx.partial[lz][i]);
                }
            }
            #[inline]
            fn decode<D: EntropyDecoder>(
                reader: &mut D,
                ctx: &mut Self::Context,
            ) -> Result<$t, std::io::Error> {
                let blbl = usize::from(AtMost::<$blbl_max>::decode(reader, &mut ctx.blbl)?);
                let bl: usize = if blbl <= 1 {
                    blbl
                } else if blbl == $blbl_max {
                    $bits
                } else {
                    let offset = match blbl {
                        2 => usize::from(AtMost::<1>::decode(reader, &mut ctx.o2)?),
                        3 => usize::from(AtMost::<3>::decode(reader, &mut ctx.o3)?),
                        4 => usize::from(AtMost::<7>::decode(reader, &mut ctx.o4)?),
                        5 => usize::from(AtMost::<15>::decode(reader, &mut ctx.o5)?),
                        6 => usize::from(AtMost::<31>::decode(reader, &mut ctx.o6)?),
                        _ => usize::from(AtMost::<63>::decode(reader, &mut ctx.o7)?),
                    };
                    (1usize << (blbl - 1)) + offset
                };
                if bl < 2 {
                    return Ok(bl as $t);
                }
                let lz = $bits - bl;
                let sig_bits = bl - 1;
                let full_bytes = sig_bits / 8;
                let partial_bits = sig_bits % 8;
                let mut value_bytes = [0u8; std::mem::size_of::<$t>()];
                if full_bytes > 0 {
                    reader.decode_incompressible_bytes(&mut value_bytes[..full_bytes])?;
                }
                for i in 0..partial_bits {
                    if bool::decode(reader, &mut ctx.partial[lz][i])? {
                        value_bytes[full_bytes] |= 1 << i;
                    }
                }
                // Restore the implicit leading 1.
                value_bytes[full_bytes] |= 1 << partial_bits;
                Ok($t::from_le_bytes(value_bytes.try_into().unwrap()))
            }
        }

        // The default `Encode` for `$t` reuses `Small`'s exact encode/decode
        // logic (there is exactly one compiled copy, shared by both) via a
        // context that wraps `Small`'s own context type and only overrides
        // its `Default` with the uniform-value prior's seeds, under which a
        // fresh context costs close to `$bits` bits for an
        // arbitrary/incompressible value (as a plain literal encoding
        // would), while `Small`'s flat seed pays a few extra fresh bits.
        // Both share one adaptive context layout, so this default still
        // learns skewed distributions quickly.
        #[derive(Clone)]
        pub struct $default_context($context);

        impl Default for $default_context {
            fn default() -> Self {
                Self(const { $context::seeded(false) })
            }
        }

        impl Encode for $t {
            type Context = $default_context;
            #[inline]
            fn encode<E: EntropyCoder>(&self, writer: &mut E, ctx: &mut Self::Context) {
                Small::encode(self, writer, &mut ctx.0)
            }
            #[inline]
            fn decode<D: EntropyDecoder>(
                reader: &mut D,
                ctx: &mut Self::Context,
            ) -> Result<$t, std::io::Error> {
                Small::decode(reader, &mut ctx.0)
            }
        }
    };
}

impl_compact!(u128, U128Compact, U128Default, 128, 8);
impl_compact!(u64, U64Compact, U64Default, 64, 7);
impl_compact!(u32, U32Compact, U32Default, 32, 6);

// `u16`'s legacy single-tree scheme (see the note above `impl_compact!`):
// the leading-zero count through one complete `AtMost<15>` symbol —
// `code = lz.saturating_sub(1)`, so lz=0 and lz=1 share code 0
// (disambiguated by `lz_is_one`) while lz=16 maps to code 15 with no extra
// bool — then the value mantissa exactly as the hierarchical scheme's.
#[derive(Clone, Default)]
pub struct U16Compact {
    leading_zeros: <AtMost<15> as Encode>::Context,
    lz_is_one: <bool as Encode>::Context,
    /// `partial[lz][i]`: context for bit `i` of the partial top byte, given
    /// `lz` leading zeros. Only `(15 - lz) % 8` bits are used per row.
    partial: [[<bool as Encode>::Context; 8]; 16],
}

impl EncodingStrategy<u16> for Small {
    type Context = U16Compact;
    #[inline]
    fn encode<E: EntropyCoder>(value: &u16, writer: &mut E, ctx: &mut Self::Context) {
        let lz = value.leading_zeros() as usize;
        let afewbits_val = lz.saturating_sub(1);
        AtMost::<15>::new(afewbits_val).encode(writer, &mut ctx.leading_zeros);
        if afewbits_val == 0 {
            (lz == 1).encode(writer, &mut ctx.lz_is_one);
        }
        if lz >= 15 {
            return;
        }
        let sig_bits = 15 - lz;
        let full_bytes = sig_bits / 8;
        let partial_bits = sig_bits % 8;
        let value_bytes = value.to_le_bytes();
        if full_bytes > 0 {
            writer.encode_incompressible_bytes(&value_bytes[..full_bytes]);
        }
        for i in 0..partial_bits {
            let bit = (value_bytes[full_bytes] >> i) & 1 == 1;
            bit.encode(writer, &mut ctx.partial[lz][i]);
        }
    }
    #[inline]
    fn decode<D: EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<u16, std::io::Error> {
        let afewbits_val = usize::from(AtMost::<15>::decode(reader, &mut ctx.leading_zeros)?);
        let lz = if afewbits_val == 0 {
            if bool::decode(reader, &mut ctx.lz_is_one)? {
                1
            } else {
                0
            }
        } else {
            afewbits_val + 1
        };
        if lz >= 15 {
            return if lz == 16 { Ok(0) } else { Ok(1) };
        }
        let sig_bits = 15 - lz;
        let full_bytes = sig_bits / 8;
        let partial_bits = sig_bits % 8;
        let mut value_bytes = [0u8; 2];
        if full_bytes > 0 {
            reader.decode_incompressible_bytes(&mut value_bytes[..full_bytes])?;
        }
        for i in 0..partial_bits {
            if bool::decode(reader, &mut ctx.partial[lz][i])? {
                value_bytes[full_bytes] |= 1 << i;
            }
        }
        // Restore the implicit leading 1.
        value_bytes[full_bytes] |= 1 << partial_bits;
        Ok(u16::from_le_bytes(value_bytes))
    }
}

/// The default `Encode` for `u16` reuses `Small`'s exact encode/decode
/// logic via a context wrapper that only overrides `Default`: the
/// leading-zero-count tree is seeded with the geometric prior
/// (`geometric_seeded`), so a fresh context costs close to 16 bits for an
/// arbitrary value while `Small`'s flat seed pays ~4 extra fresh bits.
#[derive(Clone)]
pub struct U16Default(U16Compact);

impl Default for U16Default {
    fn default() -> Self {
        Self(U16Compact {
            leading_zeros: AtMostContext {
                bits: const { geometric_seeded::<15>() },
            },
            ..U16Compact::default()
        })
    }
}

impl Encode for u16 {
    type Context = U16Default;
    #[inline]
    fn encode<E: EntropyCoder>(&self, writer: &mut E, ctx: &mut Self::Context) {
        Small::encode(self, writer, &mut ctx.0)
    }
    #[inline]
    fn decode<D: EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<u16, std::io::Error> {
        Small::decode(reader, &mut ctx.0)
    }
}

#[test]
fn compact_u16() {
    use super::estimated_bits;
    use crate::{Encoded, Small};
    expect!["4"].assert_eq(&estimated_bits!(Encoded::<_, Small>::new(0_u16)));
    expect!["4"].assert_eq(&estimated_bits!(Encoded::<_, Small>::new(1_u16)));
    expect!["5"].assert_eq(&estimated_bits!(Encoded::<_, Small>::new(2_u16)));
    expect!["5"].assert_eq(&estimated_bits!(Encoded::<_, Small>::new(3_u16)));
    expect!["6"].assert_eq(&estimated_bits!(Encoded::<_, Small>::new(4_u16)));
    expect!["6"].assert_eq(&estimated_bits!(Encoded::<_, Small>::new(5_u16)));
    expect!["6"].assert_eq(&estimated_bits!(Encoded::<_, Small>::new(6_u16)));
    expect!["6"].assert_eq(&estimated_bits!(Encoded::<_, Small>::new(7_u16)));
    expect!["7"].assert_eq(&estimated_bits!(Encoded::<_, Small>::new(8_u16)));
    expect!["20"].assert_eq(&estimated_bits!(Encoded::<_, Small>::new(u16::MAX)));
    expect!["27"].assert_eq(&estimated_bits!([Encoded::<_, Small>::new(0_u16); 128]));
    expect!["1104"].assert_eq(&estimated_bits!([Encoded::<_, Small>::new(u16::MAX); 128]));
    expect!["6"].assert_eq(&estimated_bits!([Encoded::<_, Small>::new(1_u16); 2]));
    expect!["17"].assert_eq(&estimated_bits!([Encoded::<_, Small>::new(1_u16); 19]));
    expect!["24"].assert_eq(&estimated_bits!([
        0_u16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1
    ]
    .map(Encoded::<_, Small>::new)));
}

#[test]
fn compact_u32() {
    use super::estimated_bits;
    use crate::{Encoded, Small};
    expect!["3"].assert_eq(&estimated_bits!(Encoded::<_, Small>::new(0_u32)));
    expect!["3"].assert_eq(&estimated_bits!(Encoded::<_, Small>::new(1_u32)));
    expect!["5"].assert_eq(&estimated_bits!(Encoded::<_, Small>::new(2_u32)));
    expect!["5"].assert_eq(&estimated_bits!(Encoded::<_, Small>::new(3_u32)));
    expect!["6"].assert_eq(&estimated_bits!(Encoded::<_, Small>::new(4_u32)));
    expect!["6"].assert_eq(&estimated_bits!(Encoded::<_, Small>::new(5_u32)));
    expect!["6"].assert_eq(&estimated_bits!(Encoded::<_, Small>::new(6_u32)));
    expect!["6"].assert_eq(&estimated_bits!(Encoded::<_, Small>::new(7_u32)));
    expect!["8"].assert_eq(&estimated_bits!(Encoded::<_, Small>::new(8_u32)));
    expect!["34"].assert_eq(&estimated_bits!(Encoded::<_, Small>::new(u32::MAX)));
    expect!["25"].assert_eq(&estimated_bits!([Encoded::<_, Small>::new(0_u32); 128]));
    expect!["3146"].assert_eq(&estimated_bits!([Encoded::<_, Small>::new(u32::MAX); 128]));
    expect!["5"].assert_eq(&estimated_bits!([Encoded::<_, Small>::new(1_u32); 2]));
    expect!["15"].assert_eq(&estimated_bits!([Encoded::<_, Small>::new(1_u32); 19]));
    expect!["21"].assert_eq(&estimated_bits!([
        0_u32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1
    ]
    .map(Encoded::<_, Small>::new)));

    for i in 0_u32..4096 {
        assert_eq!(
            super::encode(&Encoded::<_, Small>::new(i)),
            super::encode_with(Small, &i)
        );
    }
}

#[test]
fn normal_u32() {
    // The goal of the "normal" encoding for integers is to always encode with
    // the same total number of bits.  Adaption can then shift things based on
    // what is actually seen.
    expect!["3608 mb"].assert_eq(&0_u32.millibits().to_string());
    expect!["3608 mb"].assert_eq(&1_u32.millibits().to_string());
    expect!["8065 mb"].assert_eq(&2_u32.millibits().to_string());
    expect!["8065 mb"].assert_eq(&3_u32.millibits().to_string());
    expect!["8056 mb"].assert_eq(&4_u32.millibits().to_string());
    expect!["10043 mb"].assert_eq(&8_u32.millibits().to_string());
    expect!["10035 mb"].assert_eq(&16_u32.millibits().to_string());
    expect!["10036 mb"].assert_eq(&(1u32 << 5).millibits().to_string());
    expect!["16651 mb"].assert_eq(&(1u32 << 7).millibits().to_string());
    expect!["16644 mb"].assert_eq(&(1u32 << 9).millibits().to_string());
    expect!["18302 mb"].assert_eq(&(1u32 << 11).millibits().to_string());
    expect!["25893 mb"].assert_eq(&(1u32 << 16).millibits().to_string());
    expect!["31552 mb"].assert_eq(&(1u32 << 24).millibits().to_string());
    expect!["33203 mb"].assert_eq(&(1u32 << 28).millibits().to_string());
    expect!["32776 mb"].assert_eq(&(1u32 << 31).millibits().to_string());
    expect!["32776 mb"].assert_eq(&u32::MAX.millibits().to_string());

    // Non-power-of-two, mixed-bit-pattern values, spread across magnitudes.
    expect!["8056 mb"].assert_eq(&5_u32.millibits().to_string());
    expect!["10028 mb"].assert_eq(&100_u32.millibits().to_string());
    expect!["18295 mb"].assert_eq(&12345_u32.millibits().to_string());
    expect!["27559 mb"].assert_eq(&1_000_000_u32.millibits().to_string());
    expect!["33196 mb"].assert_eq(&0x5A5A5A5A_u32.millibits().to_string());
    expect!["32776 mb"].assert_eq(&3_000_000_000_u32.millibits().to_string());
}

#[test]
fn normal_u16() {
    // The goal of the "normal" encoding for integers is to always encode with
    // the same total number of bits.  Adaption can then shift things based on
    // what is actually seen.
    expect!["8206 mb"].assert_eq(&0_u16.millibits().to_string());
    expect!["8212 mb"].assert_eq(&1_u16.millibits().to_string());
    expect!["8215 mb"].assert_eq(&2_u16.millibits().to_string());
    expect!["8215 mb"].assert_eq(&3_u16.millibits().to_string());
    expect!["8212 mb"].assert_eq(&4_u16.millibits().to_string());
    expect!["9781 mb"].assert_eq(&8_u16.millibits().to_string());
    expect!["9775 mb"].assert_eq(&16_u16.millibits().to_string());
    expect!["9777 mb"].assert_eq(&(1u16 << 5).millibits().to_string());
    expect!["13781 mb"].assert_eq(&(1u16 << 7).millibits().to_string());
    expect!["13777 mb"].assert_eq(&(1u16 << 9).millibits().to_string());
    expect!["15715 mb"].assert_eq(&(1u16 << 11).millibits().to_string());
    expect!["16383 mb"].assert_eq(&(1u16 << 13).millibits().to_string());
    expect!["17034 mb"].assert_eq(&(1u16 << 15).millibits().to_string());
    expect!["17034 mb"].assert_eq(&u16::MAX.millibits().to_string());

    // Non-power-of-two, mixed-bit-pattern values, spread across magnitudes.
    expect!["8212 mb"].assert_eq(&5_u16.millibits().to_string());
    expect!["9769 mb"].assert_eq(&100_u16.millibits().to_string());
    expect!["16383 mb"].assert_eq(&12345_u16.millibits().to_string());
    expect!["16034 mb"].assert_eq(&0x5A5A_u16.millibits().to_string());
    expect!["17034 mb"].assert_eq(&54321_u16.millibits().to_string());
}

#[test]
fn normal_u64() {
    // The goal of the "normal" encoding for integers is to always encode with
    // the same total number of bits.  Adaption can then shift things based on
    // what is actually seen.
    expect!["6215 mb"].assert_eq(&0_u64.millibits().to_string());
    expect!["6214 mb"].assert_eq(&1_u64.millibits().to_string());
    expect!["8065 mb"].assert_eq(&2_u64.millibits().to_string());
    expect!["10036 mb"].assert_eq(&(1u64 << 5).millibits().to_string());
    expect!["19000 mb"].assert_eq(&(1u64 << 7).millibits().to_string());
    expect!["18993 mb"].assert_eq(&(1u64 << 9).millibits().to_string());
    expect!["20651 mb"].assert_eq(&(1u64 << 11).millibits().to_string());
    expect!["27242 mb"].assert_eq(&(1u64 << 16).millibits().to_string());
    expect!["32901 mb"].assert_eq(&(1u64 << 24).millibits().to_string());
    expect!["34552 mb"].assert_eq(&(1u64 << 28).millibits().to_string());
    expect!["44269 mb"].assert_eq(&(1u64 << 31).millibits().to_string());
    expect!["51553 mb"].assert_eq(&(1u64 << 45).millibits().to_string());
    expect!["63553 mb"].assert_eq(&(1u64 << 57).millibits().to_string());
    expect!["64517 mb"].assert_eq(&u64::MAX.millibits().to_string());

    // Non-power-of-two, mixed-bit-pattern values, spread across magnitudes.
    expect!["8056 mb"].assert_eq(&5_u64.millibits().to_string());
    expect!["20644 mb"].assert_eq(&12345_u64.millibits().to_string());
    expect!["34553 mb"].assert_eq(&1_000_000_000_u64.millibits().to_string());
    expect!["65196 mb"].assert_eq(&0x5A5A5A5A5A5A5A5A_u64.millibits().to_string());
    expect!["64517 mb"].assert_eq(&18_000_000_000_000_000_000_u64.millibits().to_string());
}

#[test]
fn normal_u128() {
    // The goal of the "normal" encoding for integers is to always encode with
    // the same total number of bits.  Adaption can then shift things based on
    // what is actually seen. This is the Millibits-based coverage requested
    // in place of more `estimated_bits!` point probes on `size_u128`.
    expect!["6215 mb"].assert_eq(&0_u128.millibits().to_string());
    expect!["6214 mb"].assert_eq(&1_u128.millibits().to_string());
    expect!["8065 mb"].assert_eq(&2_u128.millibits().to_string());
    expect!["10036 mb"].assert_eq(&(1u128 << 5).millibits().to_string());
    expect!["19000 mb"].assert_eq(&(1u128 << 7).millibits().to_string());
    expect!["18993 mb"].assert_eq(&(1u128 << 9).millibits().to_string());
    expect!["20651 mb"].assert_eq(&(1u128 << 11).millibits().to_string());
    expect!["27242 mb"].assert_eq(&(1u128 << 16).millibits().to_string());
    expect!["32901 mb"].assert_eq(&(1u128 << 24).millibits().to_string());
    expect!["45877 mb"].assert_eq(&(1u128 << 31).millibits().to_string());
    expect!["53161 mb"].assert_eq(&(1u128 << 45).millibits().to_string());
    expect!["65161 mb"].assert_eq(&(1u128 << 57).millibits().to_string());
    expect!["78776 mb"].assert_eq(&(1u128 << 64).millibits().to_string());
    expect!["98410 mb"].assert_eq(&(1u128 << 90).millibits().to_string());
    expect!["116063 mb"].assert_eq(&(1u128 << 110).millibits().to_string());
    expect!["128776 mb"].assert_eq(&(1u128 << 127).millibits().to_string());
    expect!["128776 mb"].assert_eq(&u128::MAX.millibits().to_string());

    // Non-power-of-two, mixed-bit-pattern values, spread across magnitudes.
    expect!["8056 mb"].assert_eq(&5_u128.millibits().to_string());
    expect!["20644 mb"].assert_eq(&12345_u128.millibits().to_string());
    expect!["34553 mb"].assert_eq(&1_000_000_000_u128.millibits().to_string());
    expect!["129714 mb"].assert_eq(
        &0x5A5A5A5A5A5A5A5A5A5A5A5A5A5A5A5A_u128
            .millibits()
            .to_string(),
    );
    expect!["129714 mb"].assert_eq(&(u128::MAX / 3).millibits().to_string());
}

// The default-`Encode` half of `impl_signed!`, for the wide types: a sign
// bit plus the magnitude (`abs_diff` from `0`/`-1`, a bijection onto
// uniform `($bits - 1)`-bit values) through the same hierarchical encoding
// as the unsigned default — with the prior capped one bit length below the
// width, since the magnitude never reaches it. Same shape as `isize`
// (`usizes.rs`), different prior.
macro_rules! impl_signed_default_hierarchical {
    ($signed:ident, $unsigned:ident, $bits:literal) => {
        /// The unsigned hierarchical context the magnitude codes
        /// through — the very type `Small<$unsigned>` uses.
        type MagnitudeContext = <Small as EncodingStrategy<$unsigned>>::Context;

        #[derive(Clone)]
        pub struct NormalContext {
            is_negative: <bool as Encode>::Context,
            magnitude: MagnitudeContext,
        }
        impl Default for NormalContext {
            fn default() -> Self {
                Self {
                    is_negative: Default::default(),
                    magnitude: const { MagnitudeContext::seeded_capped(false, $bits - 1) },
                }
            }
        }

        impl Encode for $signed {
            type Context = NormalContext;
            #[inline]
            fn encode<E: EntropyCoder>(&self, writer: &mut E, ctx: &mut Self::Context) {
                let is_neg = *self < 0;
                is_neg.encode(writer, &mut ctx.is_negative);
                let mag: $unsigned = if is_neg {
                    self.abs_diff(-1)
                } else {
                    self.abs_diff(0)
                };
                Small::encode(&mag, writer, &mut ctx.magnitude);
            }
            #[inline]
            fn decode<D: EntropyDecoder>(
                reader: &mut D,
                ctx: &mut Self::Context,
            ) -> Result<Self, std::io::Error> {
                let is_neg = bool::decode(reader, &mut ctx.is_negative)?;
                let mag: $unsigned = Small::decode(reader, &mut ctx.magnitude)?;
                if is_neg {
                    Ok(-1 - (mag as $signed))
                } else {
                    Ok(mag as $signed)
                }
            }
        }
    };
}

// The legacy default-`Encode` half of `impl_signed!`, kept for `i16` (see
// the note above `impl_compact!` — the 16-bit types measured fastest on
// their old paths): a fixed-width per-bit encoding — sign, then unary
// adaptive leading-zero bits over the magnitude, with magnitudes below 256
// finishing in the heavily-optimized `u8` byte tree.
macro_rules! impl_signed_default_legacy {
    ($signed:ident, $unsigned:ident, $bits:literal) => {
        #[derive(Clone)]
        pub struct NormalContext {
            is_negative: <bool as Encode>::Context,
            // Magnitude is in [0, 2^($bits-1)-1]: effectively a ($bits-1)-bit unsigned.
            // The MSB of $unsigned is always 0, so we use $bits-1 leading_zero slots.
            leading_zero: [<bool as Encode>::Context; $bits - 1],
            u8_ctx: <u8 as Encode>::Context,
            partial: [[<bool as Encode>::Context; 8]; $bits - 1],
        }
        impl Default for NormalContext {
            fn default() -> Self {
                Self {
                    is_negative: Default::default(),
                    leading_zero: [Default::default(); $bits - 1],
                    u8_ctx: Default::default(),
                    partial: [[Default::default(); 8]; $bits - 1],
                }
            }
        }

        impl Encode for $signed {
            type Context = NormalContext;
            #[inline]
            fn encode<E: EntropyCoder>(&self, writer: &mut E, ctx: &mut Self::Context) {
                let is_neg = *self < 0;
                is_neg.encode(writer, &mut ctx.is_negative);
                let mag: $unsigned = if is_neg {
                    self.abs_diff(-1)
                } else {
                    self.abs_diff(0)
                };
                // Encode magnitude as a ($bits-1)-bit value.
                // mag < 2^($bits-1) so mag.leading_zeros() >= 1; adjusted_lz = leading_zeros - 1.
                const MBITS: usize = $bits - 1;
                let lz = mag.leading_zeros() as usize - 1;
                if lz >= MBITS - 8 {
                    for i in (8..MBITS).rev() {
                        false.encode(writer, &mut ctx.leading_zero[i]);
                    }
                    (mag as u8).encode(writer, &mut ctx.u8_ctx);
                    return;
                }
                for i in (MBITS - lz..MBITS).rev() {
                    false.encode(writer, &mut ctx.leading_zero[i]);
                }
                true.encode(writer, &mut ctx.leading_zero[MBITS - 1 - lz]);
                let sig_bits = MBITS - 1 - lz;
                let full_bytes = sig_bits / 8;
                let partial_bits = sig_bits % 8;
                let value_bytes = mag.to_le_bytes();
                if full_bytes > 0 {
                    writer.encode_incompressible_bytes(&value_bytes[..full_bytes]);
                }
                for i in 0..partial_bits {
                    let bit = (value_bytes[full_bytes] >> i) & 1 == 1;
                    bit.encode(writer, &mut ctx.partial[lz][i]);
                }
            }
            #[inline]
            fn decode<D: EntropyDecoder>(
                reader: &mut D,
                ctx: &mut Self::Context,
            ) -> Result<Self, std::io::Error> {
                let is_neg = bool::decode(reader, &mut ctx.is_negative)?;
                const MBITS: usize = $bits - 1;
                let mut lz = 0usize;
                let mag: $unsigned = loop {
                    if lz >= MBITS - 8 {
                        let v = u8::decode(reader, &mut ctx.u8_ctx)?;
                        break v as $unsigned;
                    }
                    if bool::decode(reader, &mut ctx.leading_zero[MBITS - 1 - lz])? {
                        let sig_bits = MBITS - 1 - lz;
                        let full_bytes = sig_bits / 8;
                        let partial_bits = sig_bits % 8;
                        let mut value_bytes = [0u8; std::mem::size_of::<$unsigned>()];
                        if full_bytes > 0 {
                            reader.decode_incompressible_bytes(&mut value_bytes[..full_bytes])?;
                        }
                        for i in 0..partial_bits {
                            if bool::decode(reader, &mut ctx.partial[lz][i])? {
                                value_bytes[full_bytes] |= 1 << i;
                            }
                        }
                        value_bytes[full_bytes] |= 1 << partial_bits;
                        break $unsigned::from_le_bytes(value_bytes.try_into().unwrap());
                    }
                    lz += 1;
                };
                if is_neg {
                    Ok(-1 - (mag as $signed))
                } else {
                    Ok(mag as $signed)
                }
            }
        }
    };
}

macro_rules! impl_signed {
    ($signed:ident, $unsigned:ident, $bits:literal, $mod:ident, $default:ident) => {
        mod $mod {
            use super::*;

            $default!($signed, $unsigned, $bits);

            #[derive(Clone)]
            pub struct Context {
                is_negative: <bool as Encode>::Context,
                positive: <Small as EncodingStrategy<$unsigned>>::Context,
                negative: <Small as EncodingStrategy<$unsigned>>::Context,
            }
            impl Default for Context {
                #[inline]
                fn default() -> Self {
                    Self {
                        is_negative: Default::default(),
                        positive: Default::default(),
                        negative: Default::default(),
                    }
                }
            }

            impl EncodingStrategy<$signed> for Small {
                type Context = Context;
                #[inline]
                fn encode<E: EntropyCoder>(
                    value: &$signed,
                    writer: &mut E,
                    ctx: &mut Self::Context,
                ) {
                    (*value < 0).encode(writer, &mut ctx.is_negative);
                    if *value < 0 {
                        Small::encode(&value.abs_diff(-1), writer, &mut ctx.negative)
                    } else {
                        Small::encode(&value.abs_diff(0), writer, &mut ctx.positive)
                    }
                }
                #[inline]
                fn decode<D: EntropyDecoder>(
                    reader: &mut D,
                    ctx: &mut Self::Context,
                ) -> Result<$signed, std::io::Error> {
                    if bool::decode(reader, &mut ctx.is_negative)? {
                        let p = <Small as EncodingStrategy<$unsigned>>::decode(
                            reader,
                            &mut ctx.negative,
                        )?;
                        Ok(-1 - (p as $signed))
                    } else {
                        let p = <Small as EncodingStrategy<$unsigned>>::decode(
                            reader,
                            &mut ctx.positive,
                        )?;
                        Ok(p as $signed)
                    }
                }
            }
            #[derive(Default, Clone)]
            pub struct SortedContext {
                previous: Option<$signed>,
                not_sorted: <bool as Encode>::Context,
                value: <Small as EncodingStrategy<$signed>>::Context,
                difference: <Small as EncodingStrategy<$unsigned>>::Context,
            }

            impl EncodingStrategy<$signed> for Sorted {
                type Context = SortedContext;
                fn encode<E: EntropyCoder>(
                    value: &$signed,
                    writer: &mut E,
                    ctx: &mut Self::Context,
                ) {
                    if let Some(previous) = ctx.previous.take() {
                        let not_sorted = *value < previous;
                        not_sorted.encode(writer, &mut ctx.not_sorted);
                        if not_sorted {
                            Small::encode(value, writer, &mut ctx.value);
                        } else {
                            Small::encode(&(value.abs_diff(previous)), writer, &mut ctx.difference);
                        }
                    } else {
                        Small::encode(value, writer, &mut ctx.value);
                    }
                    ctx.previous = Some(*value);
                }
                fn decode<E: EntropyDecoder>(
                    reader: &mut E,
                    ctx: &mut Self::Context,
                ) -> Result<$signed, std::io::Error> {
                    let out = if let Some(previous) = ctx.previous.take() {
                        let not_sorted = bool::decode(reader, &mut ctx.not_sorted)?;
                        if not_sorted {
                            Small::decode(reader, &mut ctx.value)?
                        } else {
                            previous
                                .checked_add_unsigned(
                                    <Small as EncodingStrategy<$unsigned>>::decode(
                                        reader,
                                        &mut ctx.difference,
                                    )?,
                                )
                                .ok_or_else(|| std::io::Error::other("invalid addition"))?
                        }
                    } else {
                        Small::decode(reader, &mut ctx.value)?
                    };
                    ctx.previous = Some(out);
                    Ok(out)
                }
            }
        }
    };
}

impl_signed!(i128, u128, 128, mod_i128, impl_signed_default_hierarchical);
impl_signed!(i16, u16, 16, mod_i16, impl_signed_default_legacy);
impl_signed!(i32, u32, 32, mod_i32, impl_signed_default_hierarchical);
impl_signed!(i64, u64, 64, mod_i64, impl_signed_default_hierarchical);

#[test]
fn signed() {
    use super::{assert_bits_all, assert_millibits, estimated_bits};
    use crate::{Encoded, Small};
    use std::collections::BTreeSet;

    expect!["4"].assert_eq(&estimated_bits!(Encoded::<_, Small>::new(0_i32)));
    expect!["4"].assert_eq(&estimated_bits!(Encoded::<_, Small>::new(1_i32)));
    expect!["4"].assert_eq(&estimated_bits!(Encoded::<_, Small>::new(-1_i32)));
    expect!["38"].assert_eq(&estimated_bits!(Encoded::<_, Small>::new(i32::MAX)));
    expect!["38"].assert_eq(&estimated_bits!(Encoded::<_, Small>::new(i32::MIN)));
    // The default signed `Encode` is now variable-length like the unsigned
    // one (sign bit + hierarchical magnitude), so representative values
    // are probed individually rather than asserting a whole range shares
    // one size.
    expect!["5"].assert_eq(&estimated_bits!(0_i32));
    expect!["5"].assert_eq(&estimated_bits!(1_i32));
    expect!["5"].assert_eq(&estimated_bits!(-1_i32));
    expect!["18"].assert_eq(&estimated_bits!(137_i32));
    expect!["33"].assert_eq(&estimated_bits!(i32::MIN));
    expect!["33"].assert_eq(&estimated_bits!(i32::MAX));

    expect!["5"].assert_eq(&estimated_bits!(Encoded::<_, Small>::new(0_i16)));
    expect!["5"].assert_eq(&estimated_bits!(Encoded::<_, Small>::new(1_i16)));
    expect!["5"].assert_eq(&estimated_bits!(Encoded::<_, Small>::new(-1_i16)));
    expect!["20"].assert_eq(&estimated_bits!(Encoded::<_, Small>::new(i16::MAX)));
    expect!["20"].assert_eq(&estimated_bits!(Encoded::<_, Small>::new(i16::MIN)));
    // i16 keeps the legacy fixed-width default (see `impl_compact!`'s
    // 16-bit note), so every value costs exactly 16 bits fresh.
    assert_bits_all!([i16::MAX, 0, 1, 7, 137, i16::MAX - 1], expect!["16"]);
    expect!["16"].assert_eq(&estimated_bits!(i16::MIN));

    expect!["4"].assert_eq(&estimated_bits!(Encoded::<_, Small>::new(0_i64)));
    expect!["4"].assert_eq(&estimated_bits!(Encoded::<_, Small>::new(1_i64)));
    expect!["4"].assert_eq(&estimated_bits!(Encoded::<_, Small>::new(-1_i64)));
    expect!["71"].assert_eq(&estimated_bits!(Encoded::<_, Small>::new(i64::MAX)));
    expect!["71"].assert_eq(&estimated_bits!(Encoded::<_, Small>::new(i64::MIN)));
    expect!["7"].assert_eq(&estimated_bits!(0_i64));
    expect!["7"].assert_eq(&estimated_bits!(-1_i64));
    expect!["20"].assert_eq(&estimated_bits!(137_i64));
    expect!["65"].assert_eq(&estimated_bits!(i64::MIN));
    expect!["65"].assert_eq(&estimated_bits!(i64::MAX));

    expect!["7"].assert_eq(&estimated_bits!(0_i128));
    expect!["7"].assert_eq(&estimated_bits!(-1_i128));
    expect!["130"].assert_eq(&estimated_bits!(i128::MIN));
    expect!["130"].assert_eq(&estimated_bits!(i128::MAX));

    assert_millibits!(
        BTreeSet::from([-1i16, 0, 1, 2]),
        expect!["Millibits(21251)"]
    );
    assert_millibits!(
        BTreeSet::from([-1i64, 0, 1, 2]),
        expect!["Millibits(18254)"]
    );
    assert_millibits!(
        BTreeSet::from([i16::MIN, i16::MAX]),
        expect!["Millibits(45256)"]
    );
    assert_millibits!(
        BTreeSet::from([i64::MIN, i64::MAX]),
        expect!["Millibits(142256)"]
    );
}

#[test]
fn signed_default_roundtrips_every_magnitude_depth() {
    // The signed analogue of
    // `default_encoding_roundtrips_every_leading_zero_depth`: the default
    // signed `Encode` maps a value to sign + magnitude (`abs_diff` from
    // `0`/`-1`), so round-trip both signs of representative magnitudes at
    // every possible magnitude bit length — all-zero, all-one, and
    // alternating mantissas — touching every `blbl` bucket and offset
    // width of the capped-prior context, including the extremes
    // `MIN`/`MAX` (magnitude all-ones at the top depth).
    macro_rules! check {
        ($t:ty, $bits:literal) => {
            for bl in 0..$bits {
                let mags: [$t; 3] = if bl == 0 {
                    [0, 0, 0]
                } else {
                    let top: $t = 1 << (bl - 1);
                    let mask: $t = top - 1;
                    [
                        top,
                        top | mask,
                        top | (0xAAAA_AAAA_AAAA_AAAA_AAAA_AAAA_AAAA_AAAA_u128 as $t & mask),
                    ]
                };
                for mag in mags {
                    for v in [mag, -1 - mag] {
                        assert_eq!(
                            super::decode::<$t>(&super::encode(&v)),
                            Some(v),
                            "{} with magnitude bit length {bl}",
                            stringify!($t),
                        );
                    }
                }
            }
        };
    }
    check!(i16, 16);
    check!(i32, 32);
    check!(i64, 64);
    check!(i128, 128);
}
