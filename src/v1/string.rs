use super::{bits::Bits, Encode, URange};

#[derive(Default)]
pub struct CharContext {
    is_ascii: <bool as Encode>::Context,
    ascii: <Bits<128> as Encode>::Context,
    n_chunks: <URange<3> as Encode>::Context,
    chunk1: <Bits<32> as Encode>::Context,
    chunks: [<Bits<64> as Encode>::Context; 3],
}

impl Encode for char {
    type Context = CharContext;
    #[inline]
    fn encode<W: std::io::Write>(
        &self,
        writer: &mut super::Writer<W>,
        ctx: &mut Self::Context,
    ) -> Result<(), std::io::Error> {
        let mut x = u32::from(*self);
        let is_ascii = x < 128;
        is_ascii.encode(writer, &mut ctx.is_ascii)?;
        if is_ascii {
            Bits::<128>::take_from(&mut x).encode(writer, &mut ctx.ascii)
        } else {
            let n_chunks = if x < 32 * 64 {
                0
            } else if x < 32 * 64 * 64 {
                1
            } else {
                2
            };
            let n_chunks = URange::<3>::try_from(n_chunks).unwrap();
            n_chunks.encode(writer, &mut ctx.n_chunks)?;
            Bits::<32>::take_from(&mut x).encode(writer, &mut ctx.chunk1)?;
            for i in 0_usize..1 + usize::from(n_chunks) {
                Bits::<64>::take_from(&mut x).encode(writer, &mut ctx.chunks[i])?;
            }
            Ok(())
        }
    }
    #[inline]
    fn decode<R: std::io::Read>(
        reader: &mut super::Reader<R>,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        if bool::decode(reader, &mut ctx.is_ascii)? {
            let v: u8 = Bits::<128>::decode(reader, &mut ctx.ascii)?.into();
            Ok(char::from(v))
        } else {
            let n_chunks = URange::<3>::decode(reader, &mut ctx.n_chunks)?;
            let mut out: u32 = u8::from(Bits::<32>::decode(reader, &mut ctx.chunk1)?) as u32;
            for i in 0_usize..1 + usize::from(n_chunks) {
                let chunk = u8::from(Bits::<64>::decode(reader, &mut ctx.chunks[i])?) as u32;
                out |= chunk << (5 + 6 * i);
            }
            char::from_u32(out).ok_or_else(|| std::io::Error::other("invalid char value"))
        }
    }
}

// impl super::EncodeCorrelated for char {
//     fn correlated_encode<W: std::io::Write>(
//         &self,
//         writer: &mut super::Writer<W>,
//         base_ctx: &mut <Self as Encode>::Context,
//         correlated_ctx: &mut <Self as Encode>::Context,
//     ) -> Result<(), std::io::Error> {
//         let mut x = u32::from(*self);
//         let is_ascii = x < 128;
//         is_ascii.correlated_encode(writer, &mut base_ctx.is_ascii, &mut correlated_ctx.is_ascii)?;
//         if is_ascii {
//             Bits::<128>::take_from(&mut x).encode(writer, &mut ctx.ascii)
//         } else {
//             let n_chunks = if x < 32 * 64 {
//                 0
//             } else if x < 32 * 64 * 64 {
//                 1
//             } else {
//                 2
//             };
//             let n_chunks = URange::<3>::try_from(n_chunks).unwrap();
//             n_chunks.encode(writer, &mut ctx.n_chunks)?;
//             Bits::<32>::take_from(&mut x).encode(writer, &mut ctx.chunk1)?;
//             for i in 0_usize..1 + usize::from(n_chunks) {
//                 Bits::<64>::take_from(&mut x).encode(writer, &mut ctx.chunks[i])?;
//             }
//             Ok(())
//         }
//     }
//     fn correlated_decode<R: std::io::Read>(
//         reader: &mut super::Reader<R>,
//         base_ctx: &mut <Self as Encode>::Context,
//         correlated_ctx: &mut <Self as Encode>::Context,
//     ) -> Result<Self, std::io::Error> {
//     }
// }

#[derive(Default)]
pub struct Context {
    len: <usize as Encode>::Context,
    chars: <char as Encode>::Context,
}

impl Encode for String {
    type Context = Context;
    #[inline]
    fn encode<W: std::io::Write>(
        &self,
        writer: &mut super::Writer<W>,
        ctx: &mut Self::Context,
    ) -> Result<(), std::io::Error> {
        self.chars().count().encode(writer, &mut ctx.len)?;
        for b in self.chars() {
            b.encode(writer, &mut ctx.chars)?;
        }
        Ok(())
    }
    #[inline]
    fn decode<R: std::io::Read>(
        reader: &mut super::Reader<R>,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        let len = usize::decode(reader, &mut ctx.len)?;
        let mut out = String::with_capacity(len);
        for _ in 0..len {
            out.push(char::decode(reader, &mut ctx.chars)?);
        }
        Ok(out)
    }
}

#[test]
fn size() {
    use super::assert_bits;

    assert_bits!("".to_string(), 3);
    assert_bits!("a".to_string(), 11);
    assert_bits!("A".to_string(), 11);
    assert_bits!("É".to_string(), 16);
    assert_bits!("😊".to_string(), 23);
    assert_bits!("hello world".to_string(), 77);
    assert_bits!("Hello world".to_string(), 79);
    assert_bits!("hhhhhhhhhhh".to_string(), 38);
}
