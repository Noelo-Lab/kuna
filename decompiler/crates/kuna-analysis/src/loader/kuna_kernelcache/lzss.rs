//! The `complzss` container: a 0x180-byte big-endian header (magic, Adler-32
//! of the decoded bytes, decoded size, compressed size) followed by an
//! Okumura LZSS stream (4 KiB window, 18-byte maximum match).

pub(super) const MAGIC: &[u8; 8] = b"complzss";
const HEADER_LEN: usize = 0x180;
const N: usize = 4096;
const F: usize = 18;
const THRESHOLD: usize = 2;

pub(super) fn is_complzss(bytes: &[u8]) -> bool {
    bytes.starts_with(MAGIC)
}

/// Decode a `complzss` container, checking its declared size and Adler-32.
pub(super) fn decode(bytes: &[u8], limit: usize) -> Result<Vec<u8>, String> {
    let be = |off: usize| {
        bytes.get(off..off + 4).map(|s| u32::from_be_bytes(s.try_into().unwrap()))
    };
    let (Some(adler), Some(size), Some(csize)) = (be(8), be(12), be(16)) else {
        return Err("truncated complzss header".into());
    };
    let (size, csize) = (size as usize, csize as usize);
    if size > limit {
        return Err(format!("complzss declares {size} decoded bytes, over the {limit}-byte limit"));
    }
    let src = bytes
        .get(HEADER_LEN..HEADER_LEN + csize)
        .ok_or("complzss compressed size runs past the end of the payload")?;
    let out = decode_stream(src, size);
    if out.len() != size {
        return Err(format!("complzss decoded {} bytes, header declares {size}", out.len()));
    }
    if adler32(&out) != adler {
        return Err("complzss Adler-32 mismatch".into());
    }
    Ok(out)
}

pub(super) fn decode_stream(src: &[u8], size: usize) -> Vec<u8> {
    let mut ring = [0u8; N];
    ring[..N - F].fill(b' ');
    let mut r = N - F;
    let mut out = Vec::with_capacity(size);
    let mut input = src.iter().copied();
    let mut flags = 0u32;
    while out.len() < size {
        flags >>= 1;
        if flags & 0x100 == 0 {
            let Some(c) = input.next() else { break };
            flags = c as u32 | 0xff00;
        }
        if flags & 1 != 0 {
            let Some(c) = input.next() else { break };
            out.push(c);
            ring[r] = c;
            r = (r + 1) & (N - 1);
            continue;
        }
        let (Some(lo), Some(hi)) = (input.next(), input.next()) else { break };
        let at = lo as usize | ((hi as usize & 0xf0) << 4);
        let len = (hi as usize & 0x0f) + THRESHOLD;
        for k in 0..=len {
            let c = ring[(at + k) & (N - 1)];
            out.push(c);
            ring[r] = c;
            r = (r + 1) & (N - 1);
        }
    }
    out.truncate(size);
    out
}

pub(super) fn adler32(data: &[u8]) -> u32 {
    const MOD: u32 = 65521;
    let (mut a, mut b) = (1u32, 0u32);
    for chunk in data.chunks(5552) {
        for &x in chunk {
            a += x as u32;
            b += a;
        }
        a %= MOD;
        b %= MOD;
    }
    (b << 16) | a
}
