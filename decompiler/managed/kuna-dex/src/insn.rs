//! Dalvik instruction decoding: every format of the bytecode specification,
//! and the packed-switch, sparse-switch and fill-array-data payloads.

use crate::{err, Result};

/// Instruction formats (the spec's format IDs).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Fmt {
    Bad,
    F10x,
    F12x,
    F11n,
    F11x,
    F10t,
    F20t,
    F22x,
    F21t,
    F21s,
    F21h,
    F21c,
    F23x,
    F22b,
    F22t,
    F22s,
    F22c,
    F30t,
    F32x,
    F31i,
    F31t,
    F31c,
    F35c,
    F3rc,
    F45cc,
    F4rcc,
    F51l,
}

fn format(op: u8) -> Fmt {
    use Fmt::*;
    match op {
        0x00 | 0x0e => F10x,
        0x01 | 0x04 | 0x07 | 0x21 => F12x,
        0x02 | 0x05 | 0x08 => F22x,
        0x03 | 0x06 | 0x09 => F32x,
        0x0a..=0x0d | 0x0f..=0x11 | 0x1d | 0x1e | 0x27 => F11x,
        0x12 => F11n,
        0x13 | 0x16 => F21s,
        0x14 | 0x17 => F31i,
        0x15 | 0x19 => F21h,
        0x18 => F51l,
        0x1a | 0x1c | 0x1f | 0x22 | 0x60..=0x6d | 0xfe | 0xff => F21c,
        0x1b => F31c,
        0x20 | 0x23 | 0x52..=0x5f => F22c,
        0x24 | 0x6e..=0x72 | 0xfc => F35c,
        0x25 | 0x74..=0x78 | 0xfd => F3rc,
        0x26 | 0x2b | 0x2c => F31t,
        0x28 => F10t,
        0x29 => F20t,
        0x2a => F30t,
        0x2d..=0x31 | 0x44..=0x51 | 0x90..=0xaf => F23x,
        0x32..=0x37 => F22t,
        0x38..=0x3d => F21t,
        0x7b..=0x8f | 0xb0..=0xcf => F12x,
        0xd0..=0xd7 => F22s,
        0xd8..=0xe2 => F22b,
        0xfa => F45cc,
        0xfb => F4rcc,
        _ => Bad,
    }
}

fn units(f: Fmt) -> usize {
    use Fmt::*;
    match f {
        Bad => 1,
        F10x | F12x | F11n | F11x | F10t => 1,
        F20t | F22x | F21t | F21s | F21h | F21c | F23x | F22b | F22t | F22s | F22c => 2,
        F30t | F32x | F31i | F31t | F31c | F35c | F3rc => 3,
        F45cc | F4rcc => 4,
        F51l => 5,
    }
}

/// One decoded instruction. Register operands are `a`, `b`, `c` in the
/// spec's naming (vA, vB, vC); `lit` is the literal, the branch offset or the
/// payload offset (in code units, relative to `addr`); `idx` the pool index;
/// `proto` the proto index of `invoke-polymorphic`. Invoke-style argument
/// lists are `args` (or `first`..`first + count` for the range forms).
#[derive(Clone, Debug, Default)]
pub struct Insn {
    pub addr: u32,
    pub size: u32,
    pub op: u8,
    pub a: u32,
    pub b: u32,
    pub c: u32,
    pub lit: i64,
    pub idx: u32,
    pub proto: u32,
    pub range: bool,
    pub count: u32,
    pub list: [u16; 5],
}

impl Insn {
    /// The argument registers of an invoke or `filled-new-array`.
    pub fn args(&self) -> Vec<u32> {
        if self.range {
            (self.c..self.c + self.count).collect()
        } else {
            self.list[..self.count as usize]
                .iter()
                .map(|&r| u32::from(r))
                .collect()
        }
    }

    pub fn next(&self) -> u32 {
        self.addr + self.size
    }

    /// The absolute address of a branch or payload operand.
    pub fn target(&self) -> i64 {
        i64::from(self.addr) + self.lit
    }
}

pub(crate) fn unit(code: &[u8], i: usize) -> Result<u16> {
    match code.get(2 * i..2 * i + 2) {
        Some(b) => Ok(u16::from_le_bytes([b[0], b[1]])),
        None => Err(err(format!("instruction runs past the end of the code at {i:#x}"))),
    }
}

/// The size in code units of the payload starting at `i`, if one does.
fn payload_units(code: &[u8], i: usize) -> Result<Option<usize>> {
    let ident = unit(code, i)?;
    Ok(match ident {
        0x0100 => Some(4 + 2 * usize::from(unit(code, i + 1)?)),
        0x0200 => Some(2 + 4 * usize::from(unit(code, i + 1)?)),
        0x0300 => {
            let width = usize::from(unit(code, i + 1)?);
            let size = (u32::from(unit(code, i + 2)?) | (u32::from(unit(code, i + 3)?) << 16)) as usize;
            Some(4 + (size.saturating_mul(width)).div_ceil(2))
        }
        _ => None,
    })
}

/// Decodes the instruction array linearly, skipping payloads.
pub fn decode(code: &[u8]) -> Result<Vec<Insn>> {
    let total = code.len() / 2;
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < total {
        if let Some(n) = payload_units(code, i)? {
            if i + n > total {
                return Err(err(format!("payload at {i:#x} runs past the end of the code")));
            }
            i += n;
            continue;
        }
        let insn = decode_one(code, i)?;
        i += insn.size as usize;
        out.push(insn);
    }
    Ok(out)
}

fn decode_one(code: &[u8], i: usize) -> Result<Insn> {
    let u0 = unit(code, i)?;
    let op = (u0 & 0xff) as u8;
    let f = format(op);
    if f == Fmt::Bad {
        return Err(err(format!("unknown opcode {op:#04x} at {i:#x}")));
    }
    let n = units(f);
    let mut u = [0u16; 5];
    for (k, slot) in u.iter_mut().enumerate().take(n) {
        *slot = unit(code, i + k)?;
    }
    let aa = u32::from(u0 >> 8);
    let a4 = u32::from((u0 >> 8) & 0xf);
    let b4 = u32::from(u0 >> 12);
    let w32 = |lo: u16, hi: u16| u32::from(lo) | (u32::from(hi) << 16);
    let mut x = Insn {
        addr: i as u32,
        size: n as u32,
        op,
        ..Insn::default()
    };
    use Fmt::*;
    match f {
        Bad | F10x => {}
        F12x => {
            x.a = a4;
            x.b = b4;
        }
        F11n => {
            x.a = a4;
            x.lit = i64::from(((u0 >> 12) as i8) << 4 >> 4);
        }
        F11x => x.a = aa,
        F10t => x.lit = i64::from((u0 >> 8) as u8 as i8),
        F20t => x.lit = i64::from(u[1] as i16),
        F22x => {
            x.a = aa;
            x.b = u32::from(u[1]);
        }
        F21t | F21s => {
            x.a = aa;
            x.lit = i64::from(u[1] as i16);
        }
        F21h => {
            x.a = aa;
            x.lit = if op == 0x15 {
                i64::from((i32::from(u[1])) << 16)
            } else {
                (i64::from(u[1] as i16)) << 48
            };
        }
        F21c => {
            x.a = aa;
            x.idx = u32::from(u[1]);
        }
        F23x => {
            x.a = aa;
            x.b = u32::from(u[1] & 0xff);
            x.c = u32::from(u[1] >> 8);
        }
        F22b => {
            x.a = aa;
            x.b = u32::from(u[1] & 0xff);
            x.lit = i64::from((u[1] >> 8) as u8 as i8);
        }
        F22t | F22s => {
            x.a = a4;
            x.b = b4;
            x.lit = i64::from(u[1] as i16);
        }
        F22c => {
            x.a = a4;
            x.b = b4;
            x.idx = u32::from(u[1]);
        }
        F30t => x.lit = i64::from(w32(u[1], u[2]) as i32),
        F32x => {
            x.a = u32::from(u[1]);
            x.b = u32::from(u[2]);
        }
        F31i | F31t => {
            x.a = aa;
            x.lit = i64::from(w32(u[1], u[2]) as i32);
        }
        F31c => {
            x.a = aa;
            x.idx = w32(u[1], u[2]);
        }
        F35c | F45cc => {
            x.count = b4;
            if x.count > 5 {
                return Err(err(format!("argument count {} > 5 at {i:#x}", x.count)));
            }
            x.idx = u32::from(u[1]);
            x.list = [
                u[2] & 0xf,
                (u[2] >> 4) & 0xf,
                (u[2] >> 8) & 0xf,
                u[2] >> 12,
                a4 as u16,
            ];
            if f == F45cc {
                x.proto = u32::from(u[3]);
            }
        }
        F3rc | F4rcc => {
            x.range = true;
            x.count = aa;
            x.idx = u32::from(u[1]);
            x.c = u32::from(u[2]);
            if f == F4rcc {
                x.proto = u32::from(u[3]);
            }
        }
        F51l => {
            x.a = aa;
            x.lit = (u64::from(u[1])
                | (u64::from(u[2]) << 16)
                | (u64::from(u[3]) << 32)
                | (u64::from(u[4]) << 48)) as i64;
        }
    }
    Ok(x)
}

fn payload_at(code: &[u8], addr: i64, ident: u16) -> Result<usize> {
    let at = usize::try_from(addr).map_err(|_| err(format!("payload offset {addr} is negative")))?;
    if unit(code, at)? != ident {
        return Err(err(format!("no payload {ident:#06x} at {at:#x}")));
    }
    Ok(at)
}

/// A packed-switch payload: `(first key, relative targets)`.
pub fn packed_switch(code: &[u8], addr: i64) -> Result<(i32, Vec<i32>)> {
    let at = payload_at(code, addr, 0x0100)?;
    let size = usize::from(unit(code, at + 1)?);
    let w = |k: usize| -> Result<i32> {
        Ok((u32::from(unit(code, k)?) | (u32::from(unit(code, k + 1)?) << 16)) as i32)
    };
    let first = w(at + 2)?;
    let targets = (0..size).map(|k| w(at + 4 + 2 * k)).collect::<Result<_>>()?;
    Ok((first, targets))
}

/// A sparse-switch payload: `(key, relative target)` pairs.
pub fn sparse_switch(code: &[u8], addr: i64) -> Result<Vec<(i32, i32)>> {
    let at = payload_at(code, addr, 0x0200)?;
    let size = usize::from(unit(code, at + 1)?);
    let w = |k: usize| -> Result<i32> {
        Ok((u32::from(unit(code, k)?) | (u32::from(unit(code, k + 1)?) << 16)) as i32)
    };
    (0..size)
        .map(|k| Ok((w(at + 2 + 2 * k)?, w(at + 2 + 2 * size + 2 * k)?)))
        .collect()
}

/// A fill-array-data payload: element width and the elements, zero-extended.
pub fn fill_array(code: &[u8], addr: i64) -> Result<(u16, Vec<u64>)> {
    let at = payload_at(code, addr, 0x0300)?;
    let width = unit(code, at + 1)?;
    let size = (u32::from(unit(code, at + 2)?) | (u32::from(unit(code, at + 3)?) << 16)) as usize;
    if !matches!(width, 1 | 2 | 4 | 8) {
        return Err(err(format!("fill-array-data element width {width}")));
    }
    let w = usize::from(width);
    let start = 2 * (at + 4);
    let bytes = size
        .checked_mul(w)
        .and_then(|n| code.get(start..start.checked_add(n)?))
        .ok_or_else(|| err(format!("fill-array-data payload at {at:#x} is out of bounds")))?;
    let elements = bytes
        .chunks_exact(w)
        .map(|c| c.iter().rev().fold(0u64, |v, &b| (v << 8) | u64::from(b)))
        .collect();
    Ok((width, elements))
}
