//! (kuna) `pcform` — the address an instruction computes from its own address,
//! which is an address whatever its size.
//!
//! # The gap
//!
//! [`super::xrefs`] files a constant as a reference only at 4096 or above, the
//! `ScalarOperandAnalyzer.checkOperands` floor: a smaller immediate "could be a
//! number, even if it is in the address space". That is the right call for an
//! immediate and the wrong one for an address formed from the program counter.
//! A zero-based image — an arm64 Mach-O dylib, an ELF shared object or PIE, a
//! firmware blob linked at 0 — maps code and data in its first page, and there
//! the floor drops every address the program forms into it:
//!
//! ```text
//! 0x2ec  adrp x0, 0x0            ; x0 = the page of _audit_target
//! 0x2f0  add  x0, x0, #0x2d0     ; x0 = 0x2d0, _audit_target
//!
//! 0x54d  lea  rdi,[rip+0xe6]     ; rdi = 0x63a, main (_start's argument)
//! ```
//!
//! Lifting the floor does not answer it: `mov w0,#0x2d0` and `add x0,x0,#0x2d0`
//! carry the same constant, and in a zero-based image every such number below
//! the first section's end lands in a mapped byte.
//!
//! # The rule
//!
//! A value an instruction computes from its own address is an address, so the
//! floor does not apply to it; it still has to land in a mapped section. SLEIGH
//! folds these operands at decode time (`adrp`'s `inst_start & ~0xfff`, x86-64's
//! `inst_next + disp32`, A32 and Thumb `adr`'s aligned `pc`), so their p-code is
//! a plain constant, exactly what a `movz` or `mov eax,imm32` lifts to. The
//! instruction's own encoding tells them apart: [`PcForm::formed`] recognises the
//! PC-relative forms and recomputes their value from the bytes, and only a
//! constant equal to that value is exempt. The ISAs whose PC-relative address
//! formation SLEIGH folds:
//!
//!  * **AArch64** `adr`/`adrp`: `(w & 0x1f000000) == 0x10000000`;
//!  * **x86-64** RIP-relative `lea`: the value is the fall-through plus the
//!    instruction's last four bytes, the `disp32` that ends every such `lea`.
//!    Any other instruction ending in four bytes ends in an immediate, and a
//!    value equal to `next + imm32` is not one it carries;
//!  * **A32** `adr` (`add`/`sub rd,pc,#imm`) and **Thumb** `adr` (T1, and the
//!    32-bit T2/T3). A four-byte ARM instruction is read both ways, which is
//!    safe because the value must match.
//!
//! RISC-V `auipc` and A32 register reads of `pc` keep the PC visible in the
//! p-code (`inst_start + imm`), and [`super::kuna_regcarry`] reads those
//! directly; [`super::kuna_picbase`] and [`super::kuna_picpool`] compose their
//! values from a proven PC base.
//!
//! Only an image that maps something below the floor builds a [`PcForm`]; every
//! other image cannot hold an address below it, and its walk is unchanged.

use object::Object;

use super::kuna_poolref::PoolImage;

/// The `checkOperands` floor the exemption is about: an image that maps
/// nothing below it is never affected.
const FLOOR: u64 = 4096;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Isa {
    Aarch64,
    X86_64,
    Arm,
}

/// The PC-relative values one instruction formed: none, or up to two (an
/// ARM four-byte word read as A32 and as Thumb-2).
#[derive(Debug, Clone, Copy, Default)]
pub(super) struct Formed([Option<u64>; 2]);

impl Formed {
    /// Did the instruction compute `v` from its own address?
    pub(super) fn contains(&self, v: u64) -> bool {
        self.0.contains(&Some(v))
    }

    #[cfg(test)]
    pub(super) fn one(v: u64) -> Self {
        Formed([Some(v), None])
    }
}

/// The recogniser for one image.
pub(super) struct PcForm<'a> {
    isa: Isa,
    code: &'a PoolImage<'a>,
}

impl<'a> PcForm<'a> {
    /// The recogniser for `file`, or `None` when nothing it could find would
    /// change an answer: the image maps nothing below the floor, or its ISA
    /// keeps the PC visible in the p-code.
    pub(super) fn new(file: &object::File, code: Option<&'a PoolImage<'a>>, mapped: &[(u64, u64)]) -> Option<Self> {
        if mapped.first().is_none_or(|&(lo, _)| lo >= FLOOR) {
            return None;
        }
        let isa = match file.architecture() {
            object::Architecture::Aarch64 => Isa::Aarch64,
            object::Architecture::X86_64 => Isa::X86_64,
            object::Architecture::Arm if file.is_little_endian() => Isa::Arm,
            _ => return None,
        };
        Some(PcForm { isa, code: code? })
    }

    /// The values the `len`-byte instruction at `vma` computes from its own
    /// address, if it is one of the PC-relative forms.
    pub(super) fn formed(&self, vma: u64, len: u32) -> Formed {
        let le16 = |at: u64| self.code.bytes_at(at, 2).map(|b| u16::from_le_bytes([b[0], b[1]]));
        let le32 = |at: u64| self.code.bytes_at(at, 4).map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]));
        match (self.isa, len) {
            (Isa::Aarch64, 4) => Formed([le32(vma).and_then(|w| aarch64(vma, w)), None]),
            (Isa::X86_64, 6..) => {
                let next = vma.wrapping_add(u64::from(len));
                let disp = le32(next.wrapping_sub(4)).map(|d| i64::from(d as i32) as u64);
                Formed([disp.map(|d| next.wrapping_add(d)), None])
            }
            (Isa::Arm, 2) => Formed([le16(vma).and_then(|h| thumb16(vma, h)), None]),
            (Isa::Arm, 4) => {
                let a32 = le32(vma).and_then(|w| a32(vma, w));
                let t32 = le16(vma).zip(le16(vma + 2)).and_then(|(h1, h2)| thumb32(vma, h1, h2));
                Formed([a32, t32])
            }
            _ => Formed::default(),
        }
    }
}

/// AArch64 `adr`/`adrp`: `immlo` in bits 29-30, the signed `immhi` in 5-23.
fn aarch64(vma: u64, w: u32) -> Option<u64> {
    if w & 0x1f00_0000 != 0x1000_0000 {
        return None;
    }
    let imm = ((((w >> 5) & 0x7ffff) << 2 | (w >> 29) & 3) << 11) as i32 >> 11;
    Some(if w >> 31 == 1 {
        (vma & !0xfff).wrapping_add((i64::from(imm) << 12) as u64)
    } else {
        vma.wrapping_add(i64::from(imm) as u64)
    })
}

/// A32 `adr`: `add rd,pc,#imm` or `sub rd,pc,#imm`, the PC read as `vma + 8`.
fn a32(vma: u64, w: u32) -> Option<u64> {
    if w >> 28 == 0xf {
        return None;
    }
    let imm = u64::from((w & 0xff).rotate_right(2 * ((w >> 8) & 0xf)));
    let pc = vma.wrapping_add(8) & 0xffff_ffff;
    match (w >> 16) & 0xfff {
        0x28f => Some(pc.wrapping_add(imm) & 0xffff_ffff),
        0x24f => Some(pc.wrapping_sub(imm) & 0xffff_ffff),
        _ => None,
    }
}

/// The word-aligned PC a Thumb `adr` reads.
fn thumb_pc(vma: u64) -> u64 {
    vma.wrapping_add(4) & !3 & 0xffff_ffff
}

/// Thumb `adr` T1: `1010 0 Rd imm8`, a word offset.
fn thumb16(vma: u64, h: u16) -> Option<u64> {
    (h & 0xf800 == 0xa000).then(|| thumb_pc(vma) + 4 * u64::from(h & 0xff))
}

/// Thumb-2 `adr` T2 (`sub`) and T3 (`add`): `11110 i 10 1010 1111` /
/// `11110 i 10 0000 1111`, then `0 imm3 Rd imm8`.
fn thumb32(vma: u64, h1: u16, h2: u16) -> Option<u64> {
    if h2 & 0x8000 != 0 {
        return None;
    }
    let imm = u64::from(h1 >> 10 & 1) << 11 | u64::from(h2 >> 12 & 7) << 8 | u64::from(h2 & 0xff);
    match h1 & 0xfbff {
        0xf2af => Some(thumb_pc(vma).wrapping_sub(imm) & 0xffff_ffff),
        0xf20f => Some(thumb_pc(vma) + imm),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Encodings as `llvm-objdump` prints them in the images they came from.
    #[test]
    fn aarch64_adrp_and_adr_are_recomputed_from_the_encoding() {
        // The issue's dylib: `adrp x0, 0x0` at 0x2ec.
        assert_eq!(aarch64(0x2ec, 0x9000_0000), Some(0));
        // ld-linux-aarch64.so.1
        assert_eq!(aarch64(0xae4, 0xf000_0150), Some(0x2b000));
        assert_eq!(aarch64(0x737c, 0xb000_012b), Some(0x2c000));
        assert_eq!(aarch64(0xfbcc, 0xd000_0043), Some(0x19000));
        // `adr x0, #8`, `adr x0, #-8`, `adr x3, #65519`
        assert_eq!(aarch64(0x400, 0x1000_0040), Some(0x408));
        assert_eq!(aarch64(0x404, 0x10ff_ffc0), Some(0x3fc));
        assert_eq!(aarch64(0x410, 0x7007_ff63), Some(0x103ff));
        // `mov x0, #0x2d0` and `add x0, x0, #0x2d0` carry the same constant.
        assert_eq!(aarch64(0x2f0, 0xd280_5a00), None);
        assert_eq!(aarch64(0x2f0, 0x910b_4000), None);
    }

    #[test]
    fn arm_adr_spellings_are_recomputed_from_the_encoding() {
        // A32 `add r12, pc, #0` (a PLT entry's `adr`) and `sub r2, pc, #52`.
        assert_eq!(a32(0x7a0, 0xe28f_c000), Some(0x7a8));
        assert_eq!(a32(0xc70, 0xe24f_2034), Some(0xc44));
        // `add r0, r4, #1328` reads r4.
        assert_eq!(a32(0xc74, 0xe284_0e53), None);
        // Thumb `adr r0, #8`, `adr.w r0, #512`, `adr.w r1, #-24`.
        assert_eq!(thumb16(0xd02, 0xa002), Some(0xd0c));
        assert_eq!(thumb32(0xe00, 0xf20f, 0x2000), Some(0x1004));
        assert_eq!(thumb32(0xe04, 0xf2af, 0x0118), Some(0xdf0));
        // `add.w r0, r4, #16`
        assert_eq!(thumb32(0xe08, 0xf104, 0x0010), None);
    }
}
