use super::{Encode, EntropyCoder, EntropyDecoder};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

impl Encode for Ipv4Addr {
    // Octet 0 has ~5 bits of real entropy (structured prefix); octets 1–3 are
    // near-uniform (7.7–7.8 bits) with essentially no zero bytes in real-world
    // data, so storing them incompressibly is 5× faster with negligible size cost.
    type Context = <u8 as Encode>::Context;
    #[inline]
    fn encode<E: EntropyCoder>(&self, writer: &mut E, ctx: &mut Self::Context) {
        let o = self.octets();
        o[0].encode(writer, ctx);
        writer.encode_incompressible_bytes(&o[1..]);
    }
    #[inline]
    fn decode<D: EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        let mut octets = [0u8; 4];
        octets[0] = u8::decode(reader, ctx)?;
        reader.decode_incompressible_bytes(&mut octets[1..])?;
        Ok(Ipv4Addr::from(octets))
    }
}

// Encoding order for Ipv6Addr (batched):
//   1. Zero flags for octets 1–14 (14 adaptive bits), all at once.
//   2. Octet 0 adaptive byte (never zero in practice).
//   3. Non-zero adaptive bytes for octets 1–6 and 11–12.
//   4. Non-zero incompressible bytes for octets 7–10 and 13–14, plus octet 15
//      always, in one batch call.
//
// Context layout:
//   zero[i]  ↔ octet i+1   (i in 0..14)
//   nz[0]    ↔ octet 0
//   nz[1+i]  ↔ octet 1+i   (i in 0..6, OctetCtx non-zero half)
//   nz[7+i]  ↔ octet 11+i  (i in 0..2, OctetCtx non-zero half)
//
// Octets 7–10 (zero[6..10]) and 13–14 (zero[12..14]) use zero-skip with
// incompressible non-zero values because their non-zero entropy is 7.4–7.9 bits.
// Octet 15 (1.4% zeros, 7.4 bits) is always incompressible.
#[derive(Default, Clone)]
pub struct Ipv6Context {
    zero: [<bool as Encode>::Context; 14],
    nz: [<u8 as Encode>::Context; 9],
}

impl Encode for Ipv6Addr {
    type Context = Ipv6Context;
    #[inline]
    fn encode<E: EntropyCoder>(&self, writer: &mut E, ctx: &mut Self::Context) {
        let o = self.octets();
        // Phase 1: all zero flags for octets 1–14
        let z: [bool; 14] = std::array::from_fn(|i| o[i + 1] == 0);
        for (zf, c) in z.iter().zip(ctx.zero.iter_mut()) {
            zf.encode(writer, c);
        }
        // Phase 2: adaptive bytes
        o[0].encode(writer, &mut ctx.nz[0]);
        for i in 0..6 {
            if !z[i] {
                o[1 + i].encode(writer, &mut ctx.nz[1 + i]);
            }
        }
        for i in 0..2 {
            if !z[10 + i] {
                o[11 + i].encode(writer, &mut ctx.nz[7 + i]);
            }
        }
        // Phase 3: incompressible bytes in one batch
        let mut buf = [0u8; 7];
        let mut n = 0;
        for i in 0..4 {
            if !z[6 + i] {
                buf[n] = o[7 + i];
                n += 1;
            }
        }
        for i in 0..2 {
            if !z[12 + i] {
                buf[n] = o[13 + i];
                n += 1;
            }
        }
        buf[n] = o[15];
        n += 1;
        writer.encode_incompressible_bytes(&buf[..n]);
    }
    #[inline]
    fn decode<D: EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        // Phase 1: all zero flags for octets 1–14
        let mut z = [false; 14];
        for (zf, c) in z.iter_mut().zip(ctx.zero.iter_mut()) {
            *zf = bool::decode(reader, c)?;
        }
        // Phase 2: adaptive bytes
        let mut o = [0u8; 16];
        o[0] = u8::decode(reader, &mut ctx.nz[0])?;
        for i in 0..6 {
            if !z[i] {
                o[1 + i] = u8::decode(reader, &mut ctx.nz[1 + i])?;
            }
        }
        for i in 0..2 {
            if !z[10 + i] {
                o[11 + i] = u8::decode(reader, &mut ctx.nz[7 + i])?;
            }
        }
        // Phase 3: batch incompressible read
        let n = z[6..10].iter().filter(|&&z| !z).count()
            + z[12..14].iter().filter(|&&z| !z).count()
            + 1;
        let mut buf = [0u8; 7];
        reader.decode_incompressible_bytes(&mut buf[..n])?;
        let mut idx = 0;
        for i in 0..4 {
            if !z[6 + i] {
                o[7 + i] = buf[idx];
                idx += 1;
            }
        }
        for i in 0..2 {
            if !z[12 + i] {
                o[13 + i] = buf[idx];
                idx += 1;
            }
        }
        o[15] = buf[idx];
        Ok(Ipv6Addr::from(o))
    }
}

impl Encode for IpAddr {
    type Context = (
        <bool as Encode>::Context,
        <Ipv4Addr as Encode>::Context,
        <Ipv6Addr as Encode>::Context,
    );
    #[inline]
    fn encode<E: EntropyCoder>(&self, writer: &mut E, ctx: &mut Self::Context) {
        match self {
            IpAddr::V4(addr) => {
                true.encode(writer, &mut ctx.0);
                addr.encode(writer, &mut ctx.1);
            }
            IpAddr::V6(addr) => {
                false.encode(writer, &mut ctx.0);
                addr.encode(writer, &mut ctx.2);
            }
        }
    }
    #[inline]
    fn decode<D: EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        if bool::decode(reader, &mut ctx.0)? {
            Ipv4Addr::decode(reader, &mut ctx.1).map(IpAddr::V4)
        } else {
            Ipv6Addr::decode(reader, &mut ctx.2).map(IpAddr::V6)
        }
    }
}

impl Encode for SocketAddrV4 {
    type Context = (<Ipv4Addr as Encode>::Context, <u16 as Encode>::Context);
    #[inline]
    fn encode<E: EntropyCoder>(&self, writer: &mut E, ctx: &mut Self::Context) {
        self.ip().encode(writer, &mut ctx.0);
        self.port().encode(writer, &mut ctx.1);
    }
    #[inline]
    fn decode<D: EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        let ip = Ipv4Addr::decode(reader, &mut ctx.0)?;
        let port = u16::decode(reader, &mut ctx.1)?;
        Ok(SocketAddrV4::new(ip, port))
    }
}

impl Encode for SocketAddrV6 {
    type Context = (
        <Ipv6Addr as Encode>::Context,
        <u16 as Encode>::Context,
        <u32 as Encode>::Context,
        <u32 as Encode>::Context,
    );
    #[inline]
    fn encode<E: EntropyCoder>(&self, writer: &mut E, ctx: &mut Self::Context) {
        self.ip().encode(writer, &mut ctx.0);
        self.port().encode(writer, &mut ctx.1);
        self.flowinfo().encode(writer, &mut ctx.2);
        self.scope_id().encode(writer, &mut ctx.3);
    }
    #[inline]
    fn decode<D: EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        let ip = Ipv6Addr::decode(reader, &mut ctx.0)?;
        let port = u16::decode(reader, &mut ctx.1)?;
        let flowinfo = u32::decode(reader, &mut ctx.2)?;
        let scope_id = u32::decode(reader, &mut ctx.3)?;
        Ok(SocketAddrV6::new(ip, port, flowinfo, scope_id))
    }
}

impl Encode for SocketAddr {
    type Context = (
        <bool as Encode>::Context,
        <SocketAddrV4 as Encode>::Context,
        <SocketAddrV6 as Encode>::Context,
    );
    #[inline]
    fn encode<E: EntropyCoder>(&self, writer: &mut E, ctx: &mut Self::Context) {
        match self {
            SocketAddr::V4(addr) => {
                true.encode(writer, &mut ctx.0);
                addr.encode(writer, &mut ctx.1);
            }
            SocketAddr::V6(addr) => {
                false.encode(writer, &mut ctx.0);
                addr.encode(writer, &mut ctx.2);
            }
        }
    }
    #[inline]
    fn decode<D: EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        if bool::decode(reader, &mut ctx.0)? {
            SocketAddrV4::decode(reader, &mut ctx.1).map(SocketAddr::V4)
        } else {
            SocketAddrV6::decode(reader, &mut ctx.2).map(SocketAddr::V6)
        }
    }
}

#[test]
fn net_roundtrip() {
    use super::{decode, encode};

    let v4 = Ipv4Addr::new(192, 168, 1, 1);
    assert_eq!(decode::<Ipv4Addr>(&encode(&v4)), Some(v4));

    let v6 = Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1);
    assert_eq!(decode::<Ipv6Addr>(&encode(&v6)), Some(v6));

    let ip4: IpAddr = IpAddr::V4(v4);
    let ip6: IpAddr = IpAddr::V6(v6);
    assert_eq!(decode::<IpAddr>(&encode(&ip4)), Some(ip4));
    assert_eq!(decode::<IpAddr>(&encode(&ip6)), Some(ip6));

    let sa4 = SocketAddrV4::new(v4, 8080);
    assert_eq!(decode::<SocketAddrV4>(&encode(&sa4)), Some(sa4));

    let sa6 = SocketAddrV6::new(v6, 443, 0, 0);
    assert_eq!(decode::<SocketAddrV6>(&encode(&sa6)), Some(sa6));

    let sock4: SocketAddr = SocketAddr::V4(sa4);
    let sock6: SocketAddr = SocketAddr::V6(sa6);
    assert_eq!(decode::<SocketAddr>(&encode(&sock4)), Some(sock4));
    assert_eq!(decode::<SocketAddr>(&encode(&sock6)), Some(sock6));
}
