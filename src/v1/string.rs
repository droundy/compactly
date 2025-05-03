use super::{bits::Bits, Encode, EncodingStrategy, Small, URange};

#[derive(Default, Debug, PartialEq, Eq)]
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

#[derive(Default)]
pub struct Lz77 {
    old: Vec<String>,
    count: <usize as Encode>::Context,
    is_lit: <bool as Encode>::Context,
    literal: <char as Encode>::Context,
    back: <usize as Encode>::Context,
    offset: <usize as Encode>::Context,
    self_offset: <usize as Encode>::Context,
    length: <usize as Encode>::Context,
}

#[derive(Debug, PartialEq, Eq)]
enum Chunk {
    Literal(char),
    Chunk {
        /// Value of 0 indicates current string, otherwise count back in old.
        back: usize,
        /// Where in the string it is located.  Counts backwards if back==0 otherwise forwards.
        offset: usize,
        /// Number of bytes in the chunk.
        length: usize,
    },
}

impl Lz77 {
    fn eager(&self, mut value: &str) -> Vec<Chunk> {
        let mut sofar = String::new();
        let mut out = Vec::new();
        while let Some(chunk) = self.eager_chunk(&mut value, &mut sofar) {
            out.push(chunk);
        }
        out
    }
    fn eager_chunk(&self, value: &mut &str, sofar: &mut String) -> Option<Chunk> {
        if value.is_empty() {
            return None;
        }
        let mut prefix = *value;
        while prefix.len() > 1 {
            let sofar_clone = sofar.clone();
            for (back, s) in std::iter::once(sofar_clone.as_str())
                .chain(self.old.iter().map(|s| s.as_str()).rev())
                .enumerate()
            {
                if let Some(mut offset) = s.find(prefix) {
                    let length = prefix.len();
                    *value = &value[length..];
                    sofar.push_str(prefix);
                    if back == 0 {
                        offset = s.len() - offset - 1;
                    }
                    return Some(Chunk::Chunk {
                        back,
                        offset,
                        length,
                    });
                }
            }

            if let Some(idx) = prefix.rfind(|_| true) {
                prefix = &prefix[..idx];
            }
        }
        // We are forced to emit a literal character
        let mut chars = value.char_indices();
        let first = chars.next()?.1;
        sofar.push(first);
        let sz = chars
            .next()
            .map(|(sz, _)| sz)
            .unwrap_or_else(|| value.len());
        *value = &value[sz..];
        Some(Chunk::Literal(first))
    }
}

#[test]
fn eager() {
    assert_eq!(Lz77::default().eager(""), Vec::new());
    assert_eq!(Lz77::default().eager("a"), vec![Chunk::Literal('a')]);
}

impl EncodingStrategy<String> for Small {
    type Context = Lz77;
    fn encode<W: std::io::Write>(
        value: &String,
        writer: &mut super::Writer<W>,
        ctx: &mut Self::Context,
    ) -> Result<(), std::io::Error> {
        let chunks = ctx.eager(value);
        chunks.len().encode(writer, &mut ctx.count)?;
        let mut first_chunk = true;
        for chunk in chunks {
            match chunk {
                Chunk::Literal(c) => {
                    if first_chunk && ctx.old.is_empty() {
                        first_chunk = false;
                    } else {
                        true.encode(writer, &mut ctx.is_lit)?;
                    }
                    c.encode(writer, &mut ctx.literal)?;
                    // println!("Encoded lit {c:?}");
                }
                Chunk::Chunk {
                    back,
                    offset,
                    length,
                } => {
                    false.encode(writer, &mut ctx.is_lit)?;
                    // println!("doing chunk {back} {offset} {length}");
                    back.encode(writer, &mut ctx.back)?;
                    (length - 2).encode(writer, &mut ctx.length)?;
                    let offset_context = if back == 0 {
                        &mut ctx.self_offset
                    } else {
                        &mut ctx.offset
                    };
                    offset.encode(writer, offset_context)?;
                    // println!("encoded chunk {back} {offset} {length}");
                }
            }
        }
        Ok(())
    }

    fn decode<R: std::io::Read>(
        reader: &mut super::Reader<R>,
        ctx: &mut Self::Context,
    ) -> Result<String, std::io::Error> {
        let n = <usize as Encode>::decode(reader, &mut ctx.count)?;
        let mut out = String::with_capacity(n);
        for _ in 0..n {
            if (ctx.old.is_empty() && out.is_empty())
                || <bool as Encode>::decode(reader, &mut ctx.is_lit)?
            {
                let c = <char as Encode>::decode(reader, &mut ctx.literal)?;
                // println!("Got a lit {c:?}");
                out.push(c);
            } else {
                // println!("Got a chunk");
                let back = <usize as Encode>::decode(reader, &mut ctx.back)?;
                // We add 2 to the length because a length of 0 or 1 makes
                // little sense.  Note also that length is a number of bytes not
                // a number of characters.
                let length = 2 + <usize as Encode>::decode(reader, &mut ctx.length)?;
                if back == 0 {
                    // We are repeating our own string.  In this case offset
                    // counts *backwards* and must be >= 1 so we shift it.
                    let offset =
                        out.len() - 1 - <usize as Encode>::decode(reader, &mut ctx.self_offset)?;
                    // println!("chunk with {offset} {length} and {}", out.len());
                    if length <= out.len() - offset {
                        // println!("We have from {offset} to {}", offset + length);
                        let x = String::from(&out[offset..offset + length]);
                        out.push_str(&x);
                    } else {
                        // println!("We are run length encoding");
                        // With extra length this means we are using run length
                        // encoding in effect, which is kind of a pain.
                        let chunk = String::from(&out[offset..]);
                        let final_length = out.len() + length;
                        while out.len() < final_length {
                            out.push_str(&chunk);
                        }
                        while out.len() > final_length {
                            out.pop();
                        }
                    }
                } else {
                    let offset = <usize as Encode>::decode(reader, &mut ctx.offset)?;
                    out.push_str(&ctx.old[ctx.old.len() - back][offset..offset + length]);
                }
            }
        }
        ctx.old.push(out.clone());
        Ok(out)
    }
}

#[test]
fn size() {
    use super::{assert_bits, Encoded};

    assert_bits!("".to_string(), 3);
    assert_bits!("a".to_string(), 11);
    assert_bits!("A".to_string(), 11);
    assert_bits!("É".to_string(), 16);
    assert_bits!("😊".to_string(), 23);
    assert_bits!("hello world".to_string(), 77);
    assert_bits!("Hello world".to_string(), 79);
    assert_bits!("hhhhhhhhhhh".to_string(), 38);

    assert_bits!(Encoded::<_, Small>::new("".to_string()), 3);
    assert_bits!('a', 8);
    assert_bits!(Encoded::<_, Small>::new("a".to_string()), 11);
    assert_bits!(Encoded::<_, Small>::new("aa".to_string()), 17);
    assert_bits!(Encoded::<_, Small>::new("aaa".to_string()), 21);
    println!("=========================================================");
    assert_bits!(Encoded::<_, Small>::new("aaaa".to_string()), 27);

    assert_bits!(Encoded::<_, Small>::new("aaaaaaaa".to_string()), 41);
    assert_bits!("aaaaaaaa".to_string(), 34);

    assert_bits!(Encoded::<_, Small>::new("hello".to_string()), 41);

    assert_bits!(
        Encoded::<_, Small>::new("hello world hello wood".to_string()),
        120
    );
    assert_bits!("hello world hello wood".to_string(), 126);
}
