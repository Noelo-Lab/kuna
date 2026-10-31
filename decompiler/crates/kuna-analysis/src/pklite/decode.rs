//! The PKLITE compressed stream: LZ77 over an 8 KiB window, with fixed Huffman
//! codes for the match length and for the high bits of the match distance.
//!
//! Control bits and data bytes share one byte stream. Bits come from 16-bit
//! little-endian control words, least significant first, and the next word is
//! read the moment the last bit of the current one is consumed -- before any
//! data byte that follows that bit. A `0` bit is a literal byte; a `1` bit is a
//! length code, then (unless the length says otherwise) a distance code, then the
//! distance's low byte. One length code escapes to a byte that is a longer
//! length, or `0xff` (stop), `0xfe` (no-op) or `0xfd` (an uncompressed area).

use super::PkliteError;

/// Literal-byte transform the "extra compression" (`-e`) option applies.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Extra {
    None,
    /// XOR with the number of control bits left in the current word.
    BitCount,
    /// XOR with `0xff`: a customized v1.23 seen in RemoteAccess 1.11.
    Complement,
}

#[derive(Debug, Clone, Copy)]
pub(super) struct Params {
    pub large: bool,
    pub v120: bool,
    pub extra: Extra,
    pub offset_xor: u8,
    /// v1.00 (including the July 1990 beta) may copy from one byte before the
    /// output starts, which reads as `0`.
    pub allow_prehistory: bool,
}

pub(super) struct Decoded {
    pub bytes: Vec<u8>,
    /// File offset one past the last byte the stream consumed.
    pub end: usize,
    pub uncompressed_area: bool,
}

/// No real-mode load module comes near this; past it the stream is not one.
const MAX_OUTPUT: usize = 1 << 20;
const WINDOW: usize = 8192;

/// `(code length << 12) | code`, indexed by symbol, read high bit first.
const LENGTHS_LARGE: [u16; 24] = [
    0x2003, 0x3000, 0x4002, 0x4003, 0x4004, 0x500a, 0x500b, 0x500c, 0x601a, 0x601b, 0x703a, 0x703b,
    0x703c, 0x807a, 0x807b, 0x807c, 0x90fa, 0x90fb, 0x90fc, 0x90fd, 0x90fe, 0x90ff, 0x601c, 0x2002,
];
const LENGTHS_SMALL: [u16; 9] =
    [0x2000, 0x3004, 0x3005, 0x400c, 0x400d, 0x400e, 0x400f, 0x3003, 0x3002];
const LENGTHS_V120_LARGE: [u16; 21] = [
    0x2003, 0x3000, 0x4005, 0x4006, 0x5006, 0x5007, 0x6008, 0x6009, 0x7020, 0x7021, 0x7022, 0x7023,
    0x8048, 0x8049, 0x804a, 0x9096, 0x9097, 0x6013, 0x2002, 0x4007, 0x5005,
];
const LENGTHS_V120_SMALL: [u16; 11] =
    [0x2003, 0x3000, 0x4004, 0x4005, 0x500e, 0x601e, 0x601f, 0x4006, 0x2002, 0x4003, 0x4002];
const OFFSETS: [u16; 32] = [
    0x1001, 0x4000, 0x4001, 0x5004, 0x5005, 0x5006, 0x5007, 0x6010, 0x6011, 0x6012, 0x6013, 0x6014,
    0x6015, 0x6016, 0x702e, 0x702f, 0x7030, 0x7031, 0x7032, 0x7033, 0x7034, 0x7035, 0x7036, 0x7037,
    0x7038, 0x7039, 0x703a, 0x703b, 0x703c, 0x703d, 0x703e, 0x703f,
];
const OFFSETS_V120: [u16; 32] = [
    0x1001, 0x3000, 0x5004, 0x5005, 0x5006, 0x5007, 0x6010, 0x6011, 0x6012, 0x6013, 0x6014, 0x6015,
    0x702c, 0x702d, 0x702e, 0x702f, 0x7030, 0x7031, 0x7032, 0x7033, 0x7034, 0x7035, 0x7036, 0x7037,
    0x7038, 0x7039, 0x703a, 0x703b, 0x703c, 0x703d, 0x703e, 0x703f,
];

/// PKZIP 2.04c/2.04e (registered) were compressed by an internal PKLITE whose
/// uncompressed area leaves nine bytes of stale memory in the history that later
/// matches copy from. Those bytes sit at a fixed offset from the start of code.
const HISTORY_HACKS: [(usize, [u8; 9]); 2] = [
    (19652, [0xb8, 0x1d, 0xf9, 0x7f, 0x17, 0x14, 0xd3, 0xb5, 0xbe]),
    (20084, [0xaa, 0x8a, 0x07, 0xa4, 0x89, 0xc9, 0x97, 0x21, 0x58]),
];
const UNCOMPRESSED_SIG: [u8; 8] = *b"PKLITE\x26\xa3";

/// The symbol layout of one of the four length alphabets.
struct Lengths {
    long: u16,
    two: u16,
    two_high: Option<u16>,
    zero_literal: Option<u16>,
    long_bias: usize,
}

impl Lengths {
    fn of(p: &Params) -> (Lengths, &'static [u16]) {
        match (p.large, p.v120) {
            (true, false) => (
                Lengths { long: 22, two: 23, two_high: None, zero_literal: None, long_bias: 25 },
                &LENGTHS_LARGE,
            ),
            (true, true) => (
                Lengths {
                    long: 17,
                    two: 18,
                    two_high: Some(19),
                    zero_literal: Some(20),
                    long_bias: 20,
                },
                &LENGTHS_V120_LARGE,
            ),
            (false, false) => (
                Lengths { long: 7, two: 8, two_high: None, zero_literal: None, long_bias: 10 },
                &LENGTHS_SMALL,
            ),
            (false, true) => (
                Lengths {
                    long: 7,
                    two: 8,
                    two_high: Some(9),
                    zero_literal: Some(10),
                    long_bias: 10,
                },
                &LENGTHS_V120_SMALL,
            ),
        }
    }
}

/// A canonical-free prefix code as a binary tree; leaves hold `!symbol`.
struct Tree {
    nodes: Vec<[i32; 2]>,
}

impl Tree {
    fn new(table: &[u16]) -> Tree {
        let mut nodes = vec![[0i32; 2]];
        for (sym, &entry) in table.iter().enumerate() {
            let len = u32::from(entry >> 12);
            let code = u32::from(entry & 0x0fff);
            let mut at = 0usize;
            for k in (0..len).rev() {
                let bit = ((code >> k) & 1) as usize;
                if k == 0 {
                    nodes[at][bit] = !(sym as i32);
                } else {
                    if nodes[at][bit] == 0 {
                        nodes.push([0, 0]);
                        nodes[at][bit] = (nodes.len() - 1) as i32;
                    }
                    at = nodes[at][bit] as usize;
                }
            }
        }
        Tree { nodes }
    }

    fn read(&self, r: &mut Reader<'_>) -> Result<u16, PkliteError> {
        let mut at = 0usize;
        loop {
            let next = self.nodes[at][r.bit()? as usize];
            if next < 0 {
                return Ok(!next as u16);
            }
            if next == 0 {
                return Err(corrupt("a code that no PKLITE table contains"));
            }
            at = next as usize;
        }
    }
}

struct Reader<'a> {
    data: &'a [u8],
    pos: usize,
    word: u16,
    left: u32,
}

impl<'a> Reader<'a> {
    fn new(data: &'a [u8], pos: usize) -> Result<Self, PkliteError> {
        let mut r = Reader { data, pos, word: 0, left: 0 };
        r.fill()?;
        Ok(r)
    }

    fn fill(&mut self) -> Result<(), PkliteError> {
        let w = self
            .data
            .get(self.pos..self.pos + 2)
            .ok_or_else(|| corrupt("the compressed stream runs past the end of the file"))?;
        self.word = u16::from_le_bytes([w[0], w[1]]);
        self.left = 16;
        self.pos += 2;
        Ok(())
    }

    fn bit(&mut self) -> Result<u32, PkliteError> {
        let v = u32::from(self.word & 1);
        self.word >>= 1;
        self.left -= 1;
        if self.left == 0 {
            self.fill()?;
        }
        Ok(v)
    }

    fn byte(&mut self) -> Result<u8, PkliteError> {
        let b = *self
            .data
            .get(self.pos)
            .ok_or_else(|| corrupt("the compressed stream runs past the end of the file"))?;
        self.pos += 1;
        Ok(b)
    }
}

/// The output plus the history window matches copy from. The two diverge only
/// inside an uncompressed area, whose bytes enter the output but not (except
/// under [`HISTORY_HACKS`]) the history.
struct Window {
    out: Vec<u8>,
    ring: Vec<u8>,
    cur: usize,
}

impl Window {
    fn history(&mut self, b: u8) {
        self.ring[self.cur] = b;
        self.cur = (self.cur + 1) & (WINDOW - 1);
    }

    fn emit(&mut self, b: u8) -> Result<(), PkliteError> {
        if self.out.len() >= MAX_OUTPUT {
            return Err(corrupt(
                "the stream decodes past 1 MiB, more than a DOS load module holds",
            ));
        }
        self.out.push(b);
        Ok(())
    }

    fn literal(&mut self, b: u8) -> Result<(), PkliteError> {
        self.emit(b)?;
        self.history(b);
        Ok(())
    }

    fn copy(&mut self, dist: usize, len: usize) -> Result<(), PkliteError> {
        let mut from = (self.cur + WINDOW - dist) & (WINDOW - 1);
        for _ in 0..len {
            let b = self.ring[from];
            self.literal(b)?;
            from = (from + 1) & (WINDOW - 1);
        }
        Ok(())
    }
}

/// Decode the stream at `pos` of `file` until its stop code. `code_start` is
/// the host's start of code, which locates [`HISTORY_HACKS`].
pub(super) fn decompress(
    file: &[u8],
    pos: usize,
    code_start: usize,
    p: &Params,
) -> Result<Decoded, PkliteError> {
    let (sym, table) = Lengths::of(p);
    let lengths = Tree::new(table);
    let offsets = Tree::new(if p.v120 { &OFFSETS_V120 } else { &OFFSETS });
    let mut r = Reader::new(file, pos)?;
    let mut w = Window { out: Vec::new(), ring: vec![0u8; WINDOW], cur: 0 };
    let mut unc_area: Option<(usize, bool)> = None;

    loop {
        if r.bit()? == 0 {
            let mut b = r.byte()?;
            match p.extra {
                Extra::None => {}
                Extra::BitCount => b ^= r.left as u8,
                Extra::Complement => b ^= 0xff,
            }
            w.literal(b)?;
            continue;
        }

        let code = lengths.read(&mut r)?;
        let (len, high) = if code < sym.long {
            (usize::from(code) + 3, None)
        } else if code == sym.two {
            (2, Some(0))
        } else if code == sym.long {
            let b = r.byte()?;
            match b {
                0xff => break,
                0xfe if p.large => continue,
                0xfd if p.large => {
                    unc_area = Some(uncompressed_area(file, code_start, &mut r, &mut w)?);
                    continue;
                }
                0xfd | 0xfe => {
                    return Err(corrupt(&format!(
                        "escape code {b:#04x} in a small-compression stream"
                    )))
                }
                _ => (usize::from(b) + sym.long_bias, None),
            }
        } else if Some(code) == sym.zero_literal {
            w.literal(0)?;
            continue;
        } else if Some(code) == sym.two_high {
            (2, Some(1))
        } else {
            return Err(corrupt("a length symbol outside the alphabet"));
        };

        let high = match high {
            Some(h) => h,
            None => usize::from(offsets.read(&mut r)?),
        };
        let low = r.byte()? ^ p.offset_xor;
        let dist = (high << 8) | usize::from(low);
        let written = w.out.len();

        if let Some((area_end, in_history)) = unc_area {
            if !in_history && dist > written - area_end {
                return Err(PkliteError::Unsupported(
                    "a match reaches back into an uncompressed area, whose original bytes the \
                     file does not keep"
                        .into(),
                ));
            }
        }
        let prehistory = p.allow_prehistory && dist == written + 1;
        if !prehistory && (dist == 0 || dist > written) {
            return Err(corrupt(&format!(
                "match distance {dist} with only {written} bytes decoded"
            )));
        }
        w.copy(dist, len)?;
    }

    Ok(Decoded { bytes: w.out, end: r.pos, uncompressed_area: unc_area.is_some() })
}

/// An uncompressed area: a length byte and that many stored bytes. The original
/// file had PKLITE's eight-byte signature plus the length byte in front of them,
/// so those nine bytes are written back; what the history holds for them is
/// zero, except in the two files [`HISTORY_HACKS`] names. Returns the output
/// length at the end of the area and whether its bytes entered the history.
fn uncompressed_area(
    file: &[u8],
    code_start: usize,
    r: &mut Reader<'_>,
    w: &mut Window,
) -> Result<(usize, bool), PkliteError> {
    let len = r.byte()?;
    let hack = HISTORY_HACKS
        .iter()
        .find(|(at, bytes)| file.get(code_start + at..code_start + at + 9) == Some(&bytes[..]));
    for i in 0..9 {
        w.history(hack.map_or(0, |(_, bytes)| bytes[i]));
    }
    for b in UNCOMPRESSED_SIG {
        w.emit(b)?;
    }
    w.emit(len)?;
    for _ in 0..len {
        let b = r.byte()?;
        w.history(if hack.is_some() { b } else { 0 });
        w.emit(b)?;
    }
    Ok((w.out.len(), hack.is_some()))
}

fn corrupt(what: &str) -> PkliteError {
    PkliteError::Corrupt(what.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_code_table_is_a_complete_prefix_code() {
        for table in [
            &LENGTHS_LARGE[..],
            &LENGTHS_SMALL[..],
            &LENGTHS_V120_LARGE[..],
            &LENGTHS_V120_SMALL[..],
            &OFFSETS[..],
            &OFFSETS_V120[..],
        ] {
            let kraft: u64 = table.iter().map(|e| 1u64 << (12 - (e >> 12))).sum();
            assert_eq!(kraft, 1 << 12, "{table:x?}");
            let tree = Tree::new(table);
            assert!(tree.nodes.iter().all(|n| n[0] != 0 && n[1] != 0), "{table:x?}");
        }
    }
}
