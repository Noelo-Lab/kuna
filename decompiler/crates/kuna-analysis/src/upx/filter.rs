//! UPX's per-block *filters* -- the reversible branch-target transforms it
//! applies to executable segments before compressing them, so that the
//! otherwise-unique relative displacements of nearby calls become identical
//! byte patterns the LZ back-end can match.
//!
//! Ported from UPX's `src/filter/{cto,ctoj,ctok,ct}.h` (GPL-2.0-or-later).
//! Only the *unfilter* direction exists here; a static unpacker never filters.
//!
//! Coverage is deliberately partial and *loud*: [`unfilter`] refuses an id it
//! does not implement rather than passing the block through untouched. An
//! unimplemented filter left unapplied is invisible -- block sizes still add up,
//! the end marker still lands, the ELF still parses -- and would hand back an
//! image whose call targets are all wrong. That is the one failure this module
//! exists to prevent.
//!
//! Implemented: the x86 "calltrick with offset" family (`0x24`/`0x25`/`0x26`
//! plain, `0x36`/`0x46` with `jmp`, `0x49` with `jmp` and `jcc` -- the one every
//! Linux x86/x86-64 ELF and every win64/pe uses), the plain x86 calltricks
//! `0x11`..`0x16` a win32/pe brute-force pack may pick, and the naive
//! ARM/AArch64 branch tricks (`0x50`/`0x51`/`0x52`).

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnsupportedFilter {
    pub id: u8,
}

impl fmt::Display for UnsupportedFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let what = match self.id {
            0x80..=0x87 => " (ctojr32: x86 call/jmp/jcc with relative renumbering)",
            0x55 | 0x56 => " (RISC-V auipc)",
            0x90..=0x93 | 0xa0..=0xa3 | 0xb0..=0xb3 => " (delta/sub)",
            0xd0 => " (PowerPC branch trick)",
            _ => "",
        };
        write!(f, "unimplemented UPX filter 0x{:02x}{what}", self.id)
    }
}

/// Reverse the filter `id` over `buf` in place. `cto` is the block's `b_cto8`
/// (the byte UPX chose as the "this displacement was rewritten" tag). UPX's ELF
/// unpack path always initializes the filter with `addvalue == 0`.
pub fn unfilter(id: u8, cto: u8, buf: &mut [u8]) -> Result<(), UnsupportedFilter> {
    unfilter_at(id, cto, 0, buf)
}

/// [`unfilter`] for a buffer that starts `addvalue` bytes into the image the
/// packer filtered -- the PE arm, which filters only the code and biases every
/// rewritten target by the code's distance from the image's first section. The
/// ARM tricks are only ever applied with no bias, so a biased one is refused.
pub fn unfilter_at(
    id: u8,
    cto: u8,
    addvalue: u32,
    buf: &mut [u8],
) -> Result<(), UnsupportedFilter> {
    match id {
        0x00 => {}
        0x11 => ct32(buf, addvalue, X86Cond::E8, false),
        0x12 => ct32(buf, addvalue, X86Cond::E9, false),
        0x13 => ct32(buf, addvalue, X86Cond::E8E9, false),
        0x14 => ct32(buf, addvalue, X86Cond::E8, true),
        0x15 => ct32(buf, addvalue, X86Cond::E9, true),
        0x16 => ct32(buf, addvalue, X86Cond::E8E9, true),
        0x24 => cto32(buf, cto, addvalue, X86Cond::E8),
        0x25 => cto32(buf, cto, addvalue, X86Cond::E9),
        0x26 | 0x36 | 0x46 => cto32(buf, cto, addvalue, X86Cond::E8E9),
        0x49 => cto32(buf, cto, addvalue, X86Cond::E8E9Jcc),
        0x50 if addvalue == 0 => ct24arm_le(buf),
        0x51 if addvalue == 0 => ct24arm_be(buf),
        0x52 if addvalue == 0 => ct26arm_le(buf),
        _ => return Err(UnsupportedFilter { id }),
    }
    Ok(())
}

/// Whether [`unfilter`] can reverse this id -- lets a caller refuse a whole
/// image before writing any of it.
pub fn is_supported(id: u8) -> bool {
    matches!(id, 0x00 | 0x11..=0x16 | 0x24 | 0x25 | 0x26 | 0x36 | 0x46 | 0x49 | 0x50 | 0x51 | 0x52)
}

/// Which opcodes a `cto` filter treats as a rewritten branch. `0x36`/`0x46`
/// (`ctoj`) and `0x26` (`cto`) share `E8E9` because their unfilter conditions
/// coincide; only `0x49` (`ctok`) also claims two-byte `jcc`.
#[derive(Clone, Copy)]
enum X86Cond {
    E8,
    E9,
    E8E9,
    E8E9Jcc,
}

impl X86Cond {
    fn matches(self, buf: &[u8], ic: usize, lastcall: usize) -> bool {
        let b = buf[ic];
        match self {
            X86Cond::E8 => b == 0xe8,
            X86Cond::E9 => b == 0xe9,
            X86Cond::E8E9 => b == 0xe8 || b == 0xe9,
            X86Cond::E8E9Jcc => {
                b == 0xe8
                    || b == 0xe9
                    // A `0f 8x` jcc, but never the byte right after a rewritten
                    // call -- that one is displacement, not opcode. Index 0 has
                    // no preceding byte, and UPX's own read there is out of
                    // bounds, so it cannot be a meaningful match.
                    || (ic >= 1 && lastcall != ic && buf[ic - 1] == 0x0f && (0x80..=0x8f).contains(&b))
            }
        }
    }
}

/// `u_cto32_*` / `u_ctoj32_*` / `u_ctok32_*`: walk the block, and wherever a
/// branch opcode is followed by the `cto` tag byte, turn the stored big-endian
/// absolute target back into the original little-endian relative displacement.
fn cto32(buf: &mut [u8], cto: u8, addvalue: u32, cond: X86Cond) {
    if buf.len() < 6 {
        return;
    }
    let size5 = buf.len() - 5;
    let cto32 = u32::from(cto) << 24;
    let mut lastcall = 0usize;
    let mut ic = 0usize;
    while ic < size5 {
        if cond.matches(buf, ic, lastcall) {
            let jc = u32::from_be_bytes([buf[ic + 1], buf[ic + 2], buf[ic + 3], buf[ic + 4]]);
            if buf[ic + 1] == cto {
                let rel = jc
                    .wrapping_sub(ic as u32)
                    .wrapping_sub(1)
                    .wrapping_sub(addvalue)
                    .wrapping_sub(cto32);
                buf[ic + 1..ic + 5].copy_from_slice(&rel.to_le_bytes());
                ic += 4;
                lastcall = ic + 1;
            }
        }
        ic += 1;
    }
}

/// `u_ct32_*`: the untagged calltrick. Every branch opcode's displacement was
/// made absolute (and, for `0x14`..`0x16`, stored big-endian); subtract the
/// position back out. Nothing marks a rewritten displacement, so every opcode
/// byte the walk lands on is one.
fn ct32(buf: &mut [u8], addvalue: u32, cond: X86Cond, bswap: bool) {
    if buf.len() < 6 {
        return;
    }
    let end = buf.len() - 5;
    let mut b = 0usize;
    while b < end {
        if cond.matches(buf, b, 0) {
            let a = b + 1;
            let w = [buf[a], buf[a + 1], buf[a + 2], buf[a + 3]];
            let v = if bswap { u32::from_be_bytes(w) } else { u32::from_le_bytes(w) };
            let v = v.wrapping_sub(a as u32).wrapping_sub(addvalue);
            buf[a..a + 4].copy_from_slice(&v.to_le_bytes());
            b += 4;
        }
        b += 1;
    }
}

/// `u_ct24arm_le`: every word whose condition/opcode nibble says `BL`/`B` gets
/// its 24-bit word displacement de-biased by the instruction's own word index.
fn ct24arm_le(buf: &mut [u8]) {
    arm_words(buf, |b, a| {
        if b[3] & 0x0f == 0x0b {
            let v = u32::from(b[0]) | u32::from(b[1]) << 8 | u32::from(b[2]) << 16;
            let v = v.wrapping_sub(a / 4) & 0x00ff_ffff;
            b[0] = v as u8;
            b[1] = (v >> 8) as u8;
            b[2] = (v >> 16) as u8;
        }
    });
}

/// `u_ct24arm_be`: the same trick on a big-endian ARM word, where the opcode
/// nibble is in byte 0 and the displacement occupies bytes 1..4.
fn ct24arm_be(buf: &mut [u8]) {
    arm_words(buf, |b, a| {
        if b[0] & 0x0f == 0x0b {
            let v = u32::from(b[1]) << 16 | u32::from(b[2]) << 8 | u32::from(b[3]);
            let v = v.wrapping_sub(a / 4) & 0x00ff_ffff;
            b[1] = (v >> 16) as u8;
            b[2] = (v >> 8) as u8;
            b[3] = v as u8;
        }
    });
}

/// `u_ct26arm_le`: AArch64 `B`/`BL`, whose 26-bit immediate is the low bits of
/// the word -- the top six bits are the opcode and must survive.
fn ct26arm_le(buf: &mut [u8]) {
    arm_words(buf, |b, a| {
        if b[3] & 0x7c == 0x14 {
            let w = u32::from_le_bytes([b[0], b[1], b[2], b[3]]);
            let v = (w & 0x03ff_ffff).wrapping_sub(a / 4);
            let w = (w & 0xfc00_0000) | (v & 0x03ff_ffff);
            b.copy_from_slice(&w.to_le_bytes());
        }
    });
}

/// The shared word walk of UPX's `CT{24,26}ARM_*` macros: word 0 always, then
/// every fourth byte strictly below `len - 4`.
fn arm_words(buf: &mut [u8], mut f: impl FnMut(&mut [u8], u32)) {
    if buf.len() < 8 {
        return;
    }
    let end = buf.len() - 4;
    let mut a = 0usize;
    loop {
        f(&mut buf[a..a + 4], a as u32);
        a += 4;
        if a >= end {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unimplemented_ids_are_refused_not_ignored() {
        let mut buf = [0u8; 32];
        assert!(unfilter(0x80, 0, &mut buf).is_err());
        assert!(unfilter(0xd0, 0, &mut buf).is_err());
        assert!(unfilter(0x55, 0, &mut buf).is_err());
        assert!(!is_supported(0x80));
    }

    /// The `0x49` unfilter is the inverse of what UPX's filter wrote: a `call`
    /// whose stored big-endian target carries the `cto` tag byte becomes the
    /// original little-endian `rel32` again.
    #[test]
    fn cto32_restores_a_tagged_call() {
        // `e8` at index 3; original rel32 = 0x11223344, so the filtered form is
        // big-endian (rel + ic + 1 + cto<<24) with cto = 0x00.
        let ic = 3u32;
        let rel: u32 = 0x0011_2233;
        let stored = rel.wrapping_add(ic).wrapping_add(1);
        let mut buf = vec![0x90u8; 16];
        buf[3] = 0xe8;
        buf[4..8].copy_from_slice(&stored.to_be_bytes());
        unfilter(0x49, 0x00, &mut buf).unwrap();
        assert_eq!(u32::from_le_bytes([buf[4], buf[5], buf[6], buf[7]]), rel);
    }

    /// The PE arm unfilters only the code, which starts `addvalue` bytes into
    /// the image the packer filtered; the stored target carries that bias.
    #[test]
    fn cto32_folds_out_the_code_bias() {
        let (ic, rel, addvalue) = (3u32, 0x0011_2233u32, 0x1000u32);
        let mut buf = vec![0x90u8; 16];
        buf[3] = 0xe8;
        buf[4..8].copy_from_slice(&rel.wrapping_add(ic + 1).wrapping_add(addvalue).to_be_bytes());
        unfilter_at(0x49, 0x00, addvalue, &mut buf).unwrap();
        assert_eq!(u32::from_le_bytes([buf[4], buf[5], buf[6], buf[7]]), rel);
    }

    /// `ct32` (`0x11`..`0x16`): every `e8`/`e9` the walk lands on had its
    /// displacement made absolute -- stored big-endian for the `bswap` ids --
    /// and comes back relative.
    #[test]
    fn ct32_restores_every_call_it_walks_onto() {
        let calls = [(2usize, 0x10u32), (9, 0xffff_ff00)];
        for (id, bswap) in [(0x11u8, false), (0x14, true)] {
            let mut buf = vec![0x90u8; 20];
            for (at, rel) in calls {
                buf[at] = 0xe8;
                let abs = rel.wrapping_add(at as u32 + 1).wrapping_add(0x400);
                let w = if bswap { abs.to_be_bytes() } else { abs.to_le_bytes() };
                buf[at + 1..at + 5].copy_from_slice(&w);
            }
            unfilter_at(id, 0, 0x400, &mut buf).unwrap();
            for (at, rel) in calls {
                assert_eq!(
                    u32::from_le_bytes(buf[at + 1..at + 5].try_into().unwrap()),
                    rel,
                    "filter {id:#x}"
                );
            }
        }
        assert!(
            unfilter_at(0x52, 0, 0x400, &mut [0u8; 16]).is_err(),
            "a biased ARM trick is refused"
        );
    }

    #[test]
    fn cto32_leaves_untagged_branches_alone() {
        let mut buf = vec![0x90u8; 16];
        buf[3] = 0xe8;
        buf[4..8].copy_from_slice(&[0x77, 0x66, 0x55, 0x44]);
        let before = buf.clone();
        unfilter(0x49, 0x00, &mut buf).unwrap();
        assert_eq!(buf, before);
    }
}
