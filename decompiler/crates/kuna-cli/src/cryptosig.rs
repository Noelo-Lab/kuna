//! The signature set and byte scan behind `kuna crypto` — findcrypt's question
//! ("which well-known cipher and hash constants does this image carry, and
//! where") answered over the image's own mapped bytes.
//!
//! Three shapes of evidence are recognized:
//!
//! * **tables** — S-boxes, round-constant arrays, initial hash values, CRC
//!   tables. Each is matched at every byte offset of every loaded, initialized
//!   section, in every layout an implementation plausibly stores it in: bytes,
//!   bytes widened to 32-bit words, and 32/64-bit words in either byte order
//!   (plus the hi/lo 32-bit pair layout a 32-bit SHA-512 uses);
//! * **immediates** — lone 32/64-bit values that only ever appear as an
//!   instruction operand or a literal-pool word (the TEA delta, the CRC
//!   polynomial, SHA-1's round constants), matched in executable sections only
//!   and in the image's own byte order;
//! * **alphabets** — a printable run of exactly 64 pairwise-distinct characters
//!   (optionally followed by `=`), which is how a Base64 alphabet, standard or
//!   custom, sits in `.rodata`.
//!
//! A match wholly inside a longer one is dropped, so the SHA-1 IV is not also
//! reported as the MD5 state it begins with and an immediate that is really the
//! first word of a table in `.text` stays a table row.
//!
//! The SHA-2 and MD5 tables and the Blowfish digits of pi were generated with
//! exact integer arithmetic (cube/square roots of the primes, Machin's formula);
//! the AES, DES and CRC tables are derived here from their definitions, and the
//! unit tests pin each against its published first entries.

use std::collections::HashMap;

/// What a signature matched.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Kind {
    Table,
    Immediate,
    Alphabet,
}

impl Kind {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Kind::Table => "table",
            Kind::Immediate => "immediate",
            Kind::Alphabet => "alphabet",
        }
    }
}

/// One mapped, initialized range of the image.
pub(crate) struct Region<'d> {
    pub name: Option<String>,
    pub vma: u64,
    pub data: &'d [u8],
    pub executable: bool,
}

/// A recognized constant.
#[derive(Clone, Debug)]
pub(crate) struct Hit {
    pub addr: u64,
    pub byte_len: usize,
    pub algorithm: &'static str,
    pub name: String,
    pub kind: Kind,
    /// How the values are laid out: `u8`, `u8->u32le`, `u32le`, `u64be`, `u32x2le`, `text`.
    pub layout: &'static str,
    pub section: Option<String>,
    /// The alphabet itself, for [`Kind::Alphabet`].
    pub text: Option<String>,
}

/// A concrete byte pattern and what it means.
struct Pattern {
    bytes: Vec<u8>,
    algorithm: &'static str,
    name: String,
    kind: Kind,
    layout: &'static str,
}

enum Values<'a> {
    Bytes(&'a [u8]),
    U32(&'a [u32]),
    U64(&'a [u64]),
}

fn words32(vals: impl Iterator<Item = u32>, be: bool) -> Vec<u8> {
    vals.flat_map(|v| if be { v.to_be_bytes() } else { v.to_le_bytes() }).collect()
}

fn words64(vals: &[u64], be: bool) -> Vec<u8> {
    vals.iter().flat_map(|v| if be { v.to_be_bytes() } else { v.to_le_bytes() }).collect()
}

/// Every stored layout of a table.
fn table(out: &mut Vec<Pattern>, algorithm: &'static str, name: &str, values: Values) {
    let mut push = |bytes: Vec<u8>, layout: &'static str| {
        if !out.iter().any(|p| p.bytes == bytes) {
            out.push(Pattern { bytes, algorithm, name: name.to_string(), kind: Kind::Table, layout });
        }
    };
    match values {
        Values::Bytes(b) => {
            push(b.to_vec(), "u8");
            push(words32(b.iter().map(|&x| u32::from(x)), false), "u8->u32le");
            push(words32(b.iter().map(|&x| u32::from(x)), true), "u8->u32be");
        }
        Values::U32(w) => {
            push(words32(w.iter().copied(), false), "u32le");
            push(words32(w.iter().copied(), true), "u32be");
        }
        Values::U64(w) => {
            push(words64(w, false), "u64le");
            push(words64(w, true), "u64be");
            let pairs = || w.iter().flat_map(|v| [(v >> 32) as u32, *v as u32]);
            push(words32(pairs(), false), "u32x2le");
            push(words32(pairs(), true), "u32x2be");
        }
    }
}

fn immediate(out: &mut Vec<Pattern>, algorithm: &'static str, name: &str, value: u64, width: usize, be: bool) {
    let bytes = if width == 8 {
        if be { value.to_be_bytes().to_vec() } else { value.to_le_bytes().to_vec() }
    } else if be {
        (value as u32).to_be_bytes().to_vec()
    } else {
        (value as u32).to_le_bytes().to_vec()
    };
    let layout = match (width, be) {
        (8, false) => "u64le",
        (8, true) => "u64be",
        (_, false) => "u32le",
        (_, true) => "u32be",
    };
    out.push(Pattern { bytes, algorithm, name: name.to_string(), kind: Kind::Immediate, layout });
}

fn xtime(x: u8) -> u8 {
    (x << 1) ^ if x & 0x80 != 0 { 0x1b } else { 0 }
}

fn gmul(mut a: u8, mut b: u8) -> u8 {
    let mut p = 0u8;
    while b != 0 {
        if b & 1 != 0 {
            p ^= a;
        }
        a = xtime(a);
        b >>= 1;
    }
    p
}

/// The AES forward S-box, from its definition (multiplicative inverse in
/// GF(2^8) followed by the affine map).
pub(crate) fn aes_sbox() -> [u8; 256] {
    let mut s = [0u8; 256];
    for (x, slot) in s.iter_mut().enumerate() {
        let x = x as u8;
        let inv = if x == 0 { 0 } else { (1..=255u8).find(|&y| gmul(x, y) == 1).unwrap_or(0) };
        *slot = inv ^ inv.rotate_left(1) ^ inv.rotate_left(2) ^ inv.rotate_left(3) ^ inv.rotate_left(4) ^ 0x63;
    }
    s
}

pub(crate) fn aes_inv_sbox(s: &[u8; 256]) -> [u8; 256] {
    let mut inv = [0u8; 256];
    for (i, &v) in s.iter().enumerate() {
        inv[v as usize] = i as u8;
    }
    inv
}

/// `Te0`/`Td0` in the common big-endian-word convention (`Te0[0] == 0xc66363a5`);
/// `Te1..Te3` are its byte rotations.
pub(crate) fn aes_t0(s: &[u8; 256], coef: [u8; 4]) -> [u32; 256] {
    let mut t = [0u32; 256];
    for (i, slot) in t.iter_mut().enumerate() {
        let b = coef.map(|c| gmul(s[i], c));
        *slot = u32::from_be_bytes(b);
    }
    t
}

/// A reflected (LSB-first) CRC-32 table.
pub(crate) fn crc32_reflected(poly: u32) -> [u32; 256] {
    let mut t = [0u32; 256];
    for (i, slot) in t.iter_mut().enumerate() {
        let mut c = i as u32;
        for _ in 0..8 {
            c = if c & 1 != 0 { (c >> 1) ^ poly } else { c >> 1 };
        }
        *slot = c;
    }
    t
}

/// A normal (MSB-first) CRC-32 table.
pub(crate) fn crc32_normal(poly: u32) -> [u32; 256] {
    let mut t = [0u32; 256];
    for (i, slot) in t.iter_mut().enumerate() {
        let mut c = (i as u32) << 24;
        for _ in 0..8 {
            c = if c & 0x8000_0000 != 0 { (c << 1) ^ poly } else { c << 1 };
        }
        *slot = c;
    }
    t
}

fn des_ip() -> Vec<u8> {
    [58u8, 60, 62, 64, 57, 59, 61, 63].iter().flat_map(|&s| (0..8).map(move |j| s - 8 * j)).collect()
}

fn patterns(little_endian: bool) -> Vec<Pattern> {
    let mut out = Vec::new();
    let s = aes_sbox();
    let inv = aes_inv_sbox(&s);
    table(&mut out, "AES", "S-box", Values::Bytes(&s));
    table(&mut out, "AES", "inverse S-box", Values::Bytes(&inv));
    let te0 = aes_t0(&s, [2, 1, 1, 3]);
    let td0 = aes_t0(&inv, [0x0e, 0x09, 0x0d, 0x0b]);
    for r in 0..4u32 {
        let te: Vec<u32> = te0.iter().map(|v| v.rotate_right(8 * r)).collect();
        let td: Vec<u32> = td0.iter().map(|v| v.rotate_right(8 * r)).collect();
        table(&mut out, "AES", &format!("Te{r} T-table"), Values::U32(&te));
        table(&mut out, "AES", &format!("Td{r} T-table"), Values::U32(&td));
    }
    let rcon = [0x01u8, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x1b, 0x36];
    table(&mut out, "AES", "rcon", Values::Bytes(&rcon));
    let rcon_words: Vec<u32> = rcon.iter().map(|&b| u32::from(b) << 24).collect();
    table(&mut out, "AES", "rcon", Values::U32(&rcon_words));

    table(&mut out, "DES", "S-box S1", Values::Bytes(&DES_S1));
    table(&mut out, "DES", "initial permutation IP", Values::Bytes(&des_ip()));
    table(&mut out, "DES", "permuted choice PC-1", Values::Bytes(&DES_PC1));

    table(&mut out, "MD5", "T sine table", Values::U32(&MD5_T));
    table(&mut out, "SHA-1", "initial hash values H0..H4", Values::U32(&SHA1_H0));
    table(&mut out, "MD5/SHA-1", "initial state A..D", Values::U32(&SHA1_H0[..4]));
    table(&mut out, "SHA-256", "round constants K", Values::U32(&SHA256_K));
    table(&mut out, "SHA-256/BLAKE2s", "initial hash values H0 (= BLAKE2s IV)", Values::U32(&SHA256_H0));
    table(&mut out, "SHA-224", "initial hash values H0", Values::U32(&SHA224_H0));
    table(&mut out, "SHA-512", "round constants K", Values::U64(&SHA512_K));
    table(&mut out, "SHA-512/BLAKE2b", "initial hash values H0 (= BLAKE2b IV)", Values::U64(&SHA512_H0));
    table(&mut out, "SHA-384", "initial hash values H0", Values::U64(&SHA384_H0));

    table(&mut out, "CRC-32", "table (reflected, poly 0xEDB88320)", Values::U32(&crc32_reflected(0xEDB8_8320)));
    table(&mut out, "CRC-32", "table (normal, poly 0x04C11DB7)", Values::U32(&crc32_normal(0x04C1_1DB7)));
    table(&mut out, "CRC-32C", "table (reflected, poly 0x82F63B78)", Values::U32(&crc32_reflected(0x82F6_3B78)));

    table(&mut out, "Blowfish", "P-array", Values::U32(&BLOWFISH_P));
    for (i, head) in [BLOWFISH_S0_HEAD, BLOWFISH_S1_HEAD, BLOWFISH_S2_HEAD, BLOWFISH_S3_HEAD].iter().enumerate() {
        table(&mut out, "Blowfish", &format!("S-box S{i} (first 16 words)"), Values::U32(head));
    }

    for (text, name) in [(&b"expand 32-byte k"[..], "sigma \"expand 32-byte k\""), (&b"expand 16-byte k"[..], "tau \"expand 16-byte k\"")] {
        out.push(Pattern { bytes: text.to_vec(), algorithm: "ChaCha/Salsa20", name: name.into(), kind: Kind::Table, layout: "text" });
    }

    let be = !little_endian;
    for (algorithm, name, value) in IMMEDIATES32 {
        immediate(&mut out, algorithm, name, *value, 4, be);
    }
    for (algorithm, name, value) in IMMEDIATES64 {
        immediate(&mut out, algorithm, name, *value, 8, be);
    }
    out
}

const IMMEDIATES32: &[(&str, &str, u64)] = &[
    ("TEA/XTEA", "delta 0x9E3779B9 (golden ratio; also RC5/RC6 Q32)", 0x9E37_79B9),
    ("TEA/XTEA", "negated delta 0x61C88647", 0x61C8_8647),
    ("TEA", "decrypt sum 0xC6EF3720 (32 * delta)", 0xC6EF_3720),
    ("RC5/RC6", "P32 0xB7E15163", 0xB7E1_5163),
    ("CRC-32", "polynomial 0xEDB88320 (reflected)", 0xEDB8_8320),
    ("CRC-32", "polynomial 0x04C11DB7", 0x04C1_1DB7),
    ("CRC-32C", "polynomial 0x82F63B78 (reflected)", 0x82F6_3B78),
    ("SHA-1", "round constant K0 0x5A827999 (also MD4)", 0x5A82_7999),
    ("SHA-1", "round constant K1 0x6ED9EBA1 (also MD4)", 0x6ED9_EBA1),
    ("SHA-1", "round constant K2 0x8F1BBCDC", 0x8F1B_BCDC),
    ("SHA-1", "round constant K3 0xCA62C1D6", 0xCA62_C1D6),
    ("MD5/SHA-1", "initial state A 0x67452301", 0x6745_2301),
    ("SHA-1", "initial state E 0xC3D2E1F0", 0xC3D2_E1F0),
    ("MD5", "T[0] 0xD76AA478", 0xD76A_A478),
    ("SHA-256/BLAKE2s", "H0[0] 0x6A09E667", 0x6A09_E667),
    ("SHA-256", "K[0] 0x428A2F98", 0x428A_2F98),
    ("SHA-224", "H0[0] 0xC1059ED8", 0xC105_9ED8),
    ("ChaCha/Salsa20", "sigma word \"expa\" 0x61707865", 0x6170_7865),
];

const IMMEDIATES64: &[(&str, &str, u64)] = &[
    ("SHA-512/BLAKE2b", "H0[0] 0x6A09E667F3BCC908", 0x6A09_E667_F3BC_C908),
    ("SHA-512", "K[0] 0x428A2F98D728AE22", 0x428A_2F98_D728_AE22),
    ("SHA-384", "H0[0] 0xCBBB9D5DC1059ED8", 0xCBBB_9D5D_C105_9ED8),
];

/// Base64 alphabets: a printable run of exactly 64 distinct characters, or 65
/// ending in `=`, at least 48 of them alphanumeric.
fn alphabets(region: &Region, out: &mut Vec<Hit>) {
    const STANDARD: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    const URLSAFE: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
    let data = region.data;
    let mut i = 0;
    while i < data.len() {
        if !(0x21..=0x7e).contains(&data[i]) {
            i += 1;
            continue;
        }
        let start = i;
        while i < data.len() && (0x21..=0x7e).contains(&data[i]) {
            i += 1;
        }
        let run = &data[start..i];
        let body = match run.len() {
            64 => run,
            65 if run[64] == b'=' => &run[..64],
            _ => continue,
        };
        let mut seen = [false; 256];
        if body.iter().any(|&b| std::mem::replace(&mut seen[b as usize], true)) || seen[b'=' as usize] && run.len() == 65 {
            continue;
        }
        if body.iter().filter(|b| b.is_ascii_alphanumeric()).count() < 48 {
            continue;
        }
        let name = if body == STANDARD {
            "alphabet (standard)"
        } else if body == URLSAFE {
            "alphabet (URL-safe)"
        } else {
            "alphabet (custom)"
        };
        out.push(Hit {
            addr: region.vma + start as u64,
            byte_len: run.len(),
            algorithm: "Base64",
            name: name.into(),
            kind: Kind::Alphabet,
            layout: "text",
            section: region.name.clone(),
            text: Some(String::from_utf8_lossy(run).into_owned()),
        });
    }
}

/// Scan `regions` for every signature. Rows come back address-ordered, with any
/// match wholly inside a longer one dropped.
pub(crate) fn scan(regions: &[Region], little_endian: bool) -> Vec<Hit> {
    let pats = patterns(little_endian);
    let mut by_key: HashMap<[u8; 4], Vec<usize>> = HashMap::new();
    for (i, p) in pats.iter().enumerate() {
        let key = [p.bytes[0], p.bytes[1], p.bytes[2], p.bytes[3]];
        by_key.entry(key).or_default().push(i);
    }
    let mut hits = Vec::new();
    for region in regions {
        let data = region.data;
        for off in 0..data.len().saturating_sub(3) {
            let key = [data[off], data[off + 1], data[off + 2], data[off + 3]];
            let Some(ids) = by_key.get(&key) else { continue };
            for &id in ids {
                let p = &pats[id];
                if p.kind == Kind::Immediate && !region.executable {
                    continue;
                }
                if data[off..].starts_with(&p.bytes) {
                    hits.push(Hit {
                        addr: region.vma + off as u64,
                        byte_len: p.bytes.len(),
                        algorithm: p.algorithm,
                        name: p.name.clone(),
                        kind: p.kind,
                        layout: p.layout,
                        section: region.name.clone(),
                        text: None,
                    });
                }
            }
        }
        alphabets(region, &mut hits);
    }
    hits.sort_by(|a, b| (a.addr, std::cmp::Reverse(a.byte_len)).cmp(&(b.addr, std::cmp::Reverse(b.byte_len))));
    let mut kept: Vec<Hit> = Vec::new();
    let mut reach = 0u64;
    for h in hits {
        let end = h.addr + h.byte_len as u64;
        if !kept.is_empty() && end <= reach {
            continue;
        }
        reach = reach.max(end);
        kept.push(h);
    }
    kept
}

const DES_S1: [u8; 64] = [
    14, 4, 13, 1, 2, 15, 11, 8, 3, 10, 6, 12, 5, 9, 0, 7,
    0, 15, 7, 4, 14, 2, 13, 1, 10, 6, 12, 11, 9, 5, 3, 8,
    4, 1, 14, 8, 13, 6, 2, 11, 15, 12, 9, 7, 3, 10, 5, 0,
    15, 12, 8, 2, 4, 9, 1, 7, 5, 11, 3, 14, 10, 0, 6, 13,
];

const DES_PC1: [u8; 56] = [
    57, 49, 41, 33, 25, 17, 9, 1, 58, 50, 42, 34, 26, 18,
    10, 2, 59, 51, 43, 35, 27, 19, 11, 3, 60, 52, 44, 36,
    63, 55, 47, 39, 31, 23, 15, 7, 62, 54, 46, 38, 30, 22,
    14, 6, 61, 53, 45, 37, 29, 21, 13, 5, 28, 20, 12, 4,
];

const SHA1_H0: [u32; 5] = [0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476, 0xc3d2e1f0];

const SHA256_K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];
const SHA256_H0: [u32; 8] = [
    0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
];
const SHA224_H0: [u32; 8] = [
    0xc1059ed8, 0x367cd507, 0x3070dd17, 0xf70e5939, 0xffc00b31, 0x68581511, 0x64f98fa7, 0xbefa4fa4,
];
const SHA512_K: [u64; 80] = [
    0x428a2f98d728ae22, 0x7137449123ef65cd, 0xb5c0fbcfec4d3b2f, 0xe9b5dba58189dbbc,
    0x3956c25bf348b538, 0x59f111f1b605d019, 0x923f82a4af194f9b, 0xab1c5ed5da6d8118,
    0xd807aa98a3030242, 0x12835b0145706fbe, 0x243185be4ee4b28c, 0x550c7dc3d5ffb4e2,
    0x72be5d74f27b896f, 0x80deb1fe3b1696b1, 0x9bdc06a725c71235, 0xc19bf174cf692694,
    0xe49b69c19ef14ad2, 0xefbe4786384f25e3, 0x0fc19dc68b8cd5b5, 0x240ca1cc77ac9c65,
    0x2de92c6f592b0275, 0x4a7484aa6ea6e483, 0x5cb0a9dcbd41fbd4, 0x76f988da831153b5,
    0x983e5152ee66dfab, 0xa831c66d2db43210, 0xb00327c898fb213f, 0xbf597fc7beef0ee4,
    0xc6e00bf33da88fc2, 0xd5a79147930aa725, 0x06ca6351e003826f, 0x142929670a0e6e70,
    0x27b70a8546d22ffc, 0x2e1b21385c26c926, 0x4d2c6dfc5ac42aed, 0x53380d139d95b3df,
    0x650a73548baf63de, 0x766a0abb3c77b2a8, 0x81c2c92e47edaee6, 0x92722c851482353b,
    0xa2bfe8a14cf10364, 0xa81a664bbc423001, 0xc24b8b70d0f89791, 0xc76c51a30654be30,
    0xd192e819d6ef5218, 0xd69906245565a910, 0xf40e35855771202a, 0x106aa07032bbd1b8,
    0x19a4c116b8d2d0c8, 0x1e376c085141ab53, 0x2748774cdf8eeb99, 0x34b0bcb5e19b48a8,
    0x391c0cb3c5c95a63, 0x4ed8aa4ae3418acb, 0x5b9cca4f7763e373, 0x682e6ff3d6b2b8a3,
    0x748f82ee5defb2fc, 0x78a5636f43172f60, 0x84c87814a1f0ab72, 0x8cc702081a6439ec,
    0x90befffa23631e28, 0xa4506cebde82bde9, 0xbef9a3f7b2c67915, 0xc67178f2e372532b,
    0xca273eceea26619c, 0xd186b8c721c0c207, 0xeada7dd6cde0eb1e, 0xf57d4f7fee6ed178,
    0x06f067aa72176fba, 0x0a637dc5a2c898a6, 0x113f9804bef90dae, 0x1b710b35131c471b,
    0x28db77f523047d84, 0x32caab7b40c72493, 0x3c9ebe0a15c9bebc, 0x431d67c49c100d4c,
    0x4cc5d4becb3e42b6, 0x597f299cfc657e2a, 0x5fcb6fab3ad6faec, 0x6c44198c4a475817,
];
const SHA512_H0: [u64; 8] = [
    0x6a09e667f3bcc908, 0xbb67ae8584caa73b, 0x3c6ef372fe94f82b, 0xa54ff53a5f1d36f1,
    0x510e527fade682d1, 0x9b05688c2b3e6c1f, 0x1f83d9abfb41bd6b, 0x5be0cd19137e2179,
];
const SHA384_H0: [u64; 8] = [
    0xcbbb9d5dc1059ed8, 0x629a292a367cd507, 0x9159015a3070dd17, 0x152fecd8f70e5939,
    0x67332667ffc00b31, 0x8eb44a8768581511, 0xdb0c2e0d64f98fa7, 0x47b5481dbefa4fa4,
];
const MD5_T: [u32; 64] = [
    0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee, 0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501,
    0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be, 0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821,
    0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa, 0xd62f105d, 0x02441453, 0xd8a1e681, 0xe7d3fbc8,
    0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed, 0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a,
    0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c, 0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70,
    0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05, 0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665,
    0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039, 0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
    0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1, 0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391,
];
const BLOWFISH_P: [u32; 18] = [
    0x243f6a88, 0x85a308d3, 0x13198a2e, 0x03707344, 0xa4093822, 0x299f31d0, 0x082efa98, 0xec4e6c89,
    0x452821e6, 0x38d01377, 0xbe5466cf, 0x34e90c6c, 0xc0ac29b7, 0xc97c50dd, 0x3f84d5b5, 0xb5470917,
    0x9216d5d9, 0x8979fb1b,
];
const BLOWFISH_S0_HEAD: [u32; 16] = [
    0xd1310ba6, 0x98dfb5ac, 0x2ffd72db, 0xd01adfb7, 0xb8e1afed, 0x6a267e96, 0xba7c9045, 0xf12c7f99,
    0x24a19947, 0xb3916cf7, 0x0801f2e2, 0x858efc16, 0x636920d8, 0x71574e69, 0xa458fea3, 0xf4933d7e,
];
const BLOWFISH_S1_HEAD: [u32; 16] = [
    0x4b7a70e9, 0xb5b32944, 0xdb75092e, 0xc4192623, 0xad6ea6b0, 0x49a7df7d, 0x9cee60b8, 0x8fedb266,
    0xecaa8c71, 0x699a17ff, 0x5664526c, 0xc2b19ee1, 0x193602a5, 0x75094c29, 0xa0591340, 0xe4183a3e,
];
const BLOWFISH_S2_HEAD: [u32; 16] = [
    0xe93d5a68, 0x948140f7, 0xf64c261c, 0x94692934, 0x411520f7, 0x7602d4f7, 0xbcf46b2e, 0xd4a20068,
    0xd4082471, 0x3320f46a, 0x43b7d4b7, 0x500061af, 0x1e39f62e, 0x97244546, 0x14214f74, 0xbf8b8840,
];
const BLOWFISH_S3_HEAD: [u32; 16] = [
    0x3a39ce37, 0xd3faf5cf, 0xabc27737, 0x5ac52d1b, 0x5cb0679e, 0x4fa33742, 0xd3822740, 0x99bc9bbe,
    0xd5118e9d, 0xbf0f7315, 0xd62d1c7e, 0xc700c47b, 0xb78c1b6b, 0x21a19045, 0xb26eb1be, 0x6a366eb4,
];

#[cfg(test)]
mod tests {
    use super::*;

    fn region(data: &[u8], executable: bool) -> Region<'_> {
        Region { name: Some(".x".into()), vma: 0x1000, data, executable }
    }

    #[test]
    fn derived_tables_match_their_published_entries() {
        let s = aes_sbox();
        assert_eq!((s[0], s[1], s[0x53], s[255]), (0x63, 0x7c, 0xed, 0x16));
        let inv = aes_inv_sbox(&s);
        assert_eq!((inv[0], inv[255]), (0x52, 0x7d));
        assert_eq!(aes_t0(&s, [2, 1, 1, 3])[0], 0xc66363a5);
        assert_eq!(aes_t0(&inv, [0x0e, 0x09, 0x0d, 0x0b])[0], 0x51f4a750);
        let crc = crc32_reflected(0xEDB8_8320);
        assert_eq!((crc[1], crc[255]), (0x77073096, 0x2d02ef8d));
        assert_eq!(crc32_normal(0x04C1_1DB7)[1], 0x04c11db7);
        assert_eq!(&des_ip()[..4], &[58, 50, 42, 34]);
        assert_eq!(des_ip()[63], 7);
    }

    #[test]
    fn generated_tables_match_their_published_entries() {
        assert_eq!((SHA256_K[0], SHA256_K[63]), (0x428a2f98, 0xc67178f2));
        assert_eq!((SHA512_K[0], SHA512_K[79]), (0x428a2f98d728ae22, 0x6c44198c4a475817));
        assert_eq!(SHA256_H0[7], 0x5be0cd19);
        assert_eq!(SHA512_H0[0], 0x6a09e667f3bcc908);
        assert_eq!(SHA384_H0[0], 0xcbbb9d5dc1059ed8);
        assert_eq!(SHA224_H0[0], 0xc1059ed8);
        assert_eq!((MD5_T[0], MD5_T[63]), (0xd76aa478, 0xeb86d391));
        assert_eq!((BLOWFISH_P[0], BLOWFISH_P[17]), (0x243f6a88, 0x8979fb1b));
        assert_eq!(
            (BLOWFISH_S0_HEAD[0], BLOWFISH_S1_HEAD[0], BLOWFISH_S2_HEAD[0], BLOWFISH_S3_HEAD[0]),
            (0xd1310ba6, 0x4b7a70e9, 0xe93d5a68, 0x3a39ce37)
        );
    }

    #[test]
    fn a_table_is_found_at_any_offset_in_any_layout() {
        let mut data = vec![0xccu8; 3];
        data.extend_from_slice(&aes_sbox());
        data.extend(SHA256_K.iter().flat_map(|v| v.to_be_bytes()));
        let hits = scan(&[region(&data, false)], true);
        assert_eq!(hits.len(), 2, "{hits:?}");
        assert_eq!((hits[0].addr, hits[0].algorithm, hits[0].layout, hits[0].byte_len), (0x1003, "AES", "u8", 256));
        assert_eq!((hits[1].addr, hits[1].name.as_str(), hits[1].layout), (0x1103, "round constants K", "u32be"));
    }

    #[test]
    fn a_contained_match_is_reported_once_as_the_longer_one() {
        let data: Vec<u8> = SHA1_H0.iter().flat_map(|v| v.to_le_bytes()).collect();
        let hits = scan(&[region(&data, true)], true);
        assert_eq!(hits.len(), 1, "{hits:?}");
        assert_eq!(hits[0].algorithm, "SHA-1");
    }

    #[test]
    fn immediates_are_read_in_code_only() {
        let code = [0x05u8, 0xb9, 0x79, 0x37, 0x9e, 0xc3];
        let hits = scan(&[region(&code, true)], true);
        assert_eq!(hits.len(), 1);
        assert_eq!((hits[0].addr, hits[0].kind, hits[0].algorithm), (0x1001, Kind::Immediate, "TEA/XTEA"));
        assert!(scan(&[region(&code, false)], true).is_empty());
        assert!(scan(&[region(&code, true)], false).is_empty(), "a big-endian image reads the word the other way");
    }

    #[test]
    fn base64_alphabets_standard_and_custom() {
        let mut data = b"\0ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/\0".to_vec();
        data.extend_from_slice(b"ZYXWVUTSRQPONMLKJIHGFEDCBAzyxwvutsrqponmlkjihgfedcba9876543210+/=\0");
        data.extend_from_slice(b"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA\0");
        let hits = scan(&[region(&data, false)], true);
        assert_eq!(hits.len(), 2, "{hits:?}");
        assert_eq!((hits[0].addr, hits[0].name.as_str()), (0x1001, "alphabet (standard)"));
        assert_eq!((hits[1].name.as_str(), hits[1].byte_len), ("alphabet (custom)", 65));
    }
}
