//! (kuna `fatstr`) The instruction-level `&str` length finder.
//!
//! A Rust string literal that only code references (`lea rdi, [s]; mov esi, 15;
//! call f`) has no static `(ptr, len)` record, so the record pass cannot bound
//! it and the NUL-run scan glues it to the next literal. The length is in the
//! code: `&str` travels as two consecutive scalar arguments, so the register
//! after the pointer's in the calling convention holds it. This scans the
//! executable sections for an address of read-only text loaded into an
//! argument register and a small constant loaded into the next argument
//! register within a few instructions:
//!
//! - x86-64 (SysV): `lea r64, [rip+d]` (or, in a non-PIE image, `mov r32,
//!   imm32`) and `mov r32, imm32` (`B8+r`, with REX.B for r8-r15) into the next
//!   argument register (`rdx` beside a returned `rax`), within 32 bytes either
//!   side;
//! - AArch64: `adrp xN` + `add xN, xN, #lo` and `movz w(N+1), #imm`, within
//!   eight instructions either side.
//!
//! The pair must then reach a call, a tail jump or (for `rax:rdx` and `x0:x1`)
//! a return, as a `&str` argument or result does.
//!
//! The text must be valid UTF-8 of `2..=4096` bytes in read-only data. Each hit
//! is a bounded string fact, which ends any glued NUL-scan row at that start.

use object::read::{Object, ObjectSection};
use object::SectionKind;

use super::kuna_strrecords::{is_text, ro_regions, RoRegion};
use crate::pass::{AnalysisCtx, AnalysisOutput, AnalysisPass, Phase, StringFact};

/// SysV integer argument registers by x86 register number.
const SYSV_ARGS: [u8; 6] = [7, 6, 2, 1, 8, 9];
const WINDOW: usize = 32;

/// The register that carries the length beside a pointer in `reg`: the next
/// argument register, or `rdx` beside a returned `rax`.
fn next_arg(reg: u8) -> Option<u8> {
    if reg == 0 {
        return Some(2);
    }
    let i = SYSV_ARGS.iter().position(|&r| r == reg)?;
    SYSV_ARGS.get(i + 1).copied()
}

/// `mov r32, imm32` loading `reg` at `code[i..]`: the immediate.
fn mov_imm(code: &[u8], i: usize, reg: u8) -> Option<u32> {
    let (rex_b, at) = match code.get(i)? {
        0x41 => (8, i + 1),
        _ => (0, i),
    };
    let op = *code.get(at)?;
    if !(0xb8..=0xbf).contains(&op) || (op - 0xb8) + rex_b != reg {
        return None;
    }
    Some(u32::from_le_bytes(code.get(at + 1..at + 5)?.try_into().ok()?))
}

/// The `mov len_reg, imm32` nearest the pointer load at `code[i..i + width]`,
/// after it first: the load of the length that goes with this pointer, not of
/// a neighbouring call's.
/// The pair must then reach its consumer: a call or tail jump for argument
/// registers, a `ret` for the returned `rax:rdx`.
fn nearest_mov(code: &[u8], i: usize, width: usize, reg: u8, len_reg: u8) -> Option<u32> {
    let (len, end) = (1..=WINDOW).find_map(|d| {
        let after = i + width + d - 1;
        mov_imm(code, after, len_reg)
            .map(|v| (v, after + 5 + usize::from(code[after] == 0x41)))
            .or_else(|| i.checked_sub(d).and_then(|j| mov_imm(code, j, len_reg)).map(|v| (v, i + width)))
    })?;
    let tail = code.get(end..(end + WINDOW).min(code.len()))?;
    let consumed = if reg == 0 {
        tail.iter().take(16).any(|&b| b == 0xc3)
    } else {
        tail.windows(2).any(|w| w[0] == 0xe8 || w[0] == 0xe9 || (w[0] == 0xff && matches!((w[1] >> 3) & 7, 2 | 4)))
    };
    consumed.then_some(len)
}

fn x86_hits(vma: u64, code: &[u8], regions: &[RoRegion], out: &mut Vec<StringFact>) {
    let mut i = 0;
    while i + 7 <= code.len() {
        let rex = code[i];
        if (rex == 0x48 || rex == 0x4c) && code[i + 1] == 0x8d && code[i + 2] & 0xc7 == 0x05 {
            let reg = ((code[i + 2] >> 3) & 7) + if rex == 0x4c { 8 } else { 0 };
            let disp = i32::from_le_bytes(code[i + 3..i + 7].try_into().expect("4 bytes"));
            let ptr = (vma + i as u64 + 7).wrapping_add(disp as i64 as u64);
            if let Some(len_reg) = next_arg(reg) {
                if let Some(len) = nearest_mov(code, i, 7, reg, len_reg) {
                    push(regions, ptr, u64::from(len), out);
                }
            }
            i += 7;
            continue;
        }
        if let Some((reg, ptr)) = (0..16).find_map(|reg| mov_imm(code, i, reg).map(|v| (reg, u64::from(v)))) {
            let width = if code[i] == 0x41 { 6 } else { 5 };
            if let Some(len_reg) = next_arg(reg).filter(|_| regions.iter().any(|r| r.contains(ptr))) {
                if let Some(len) = nearest_mov(code, i, width, reg, len_reg) {
                    push(regions, ptr, u64::from(len), out);
                }
            }
            i += width;
            continue;
        }
        i += 1;
    }
}

fn a64_hits(vma: u64, code: &[u8], regions: &[RoRegion], out: &mut Vec<StringFact>) {
    let words: Vec<u32> = code.chunks_exact(4).map(|w| u32::from_le_bytes(w.try_into().expect("4 bytes"))).collect();
    for (k, &w) in words.iter().enumerate() {
        if w & 0x9f00_0000 != 0x9000_0000 {
            continue;
        }
        let rd = w & 31;
        let imm = (((w >> 29) & 3) | (((w >> 5) & 0x7ffff) << 2)) as i64;
        let imm = (imm << 43) >> 43;
        let pc = vma + 4 * k as u64;
        let page = (pc & !0xfff).wrapping_add((imm << 12) as u64);
        let Some(&add) = words.get(k + 1) else { continue };
        if add & 0xffc0_0000 != 0x9100_0000 || add & 31 != rd || (add >> 5) & 31 != rd {
            continue;
        }
        let ptr = page + u64::from((add >> 10) & 0xfff);
        let len_reg = rd + 1;
        if rd >= 7 {
            continue;
        }
        let movz = |j: usize| {
            let m = *words.get(j)?;
            (m & 0xffe0_0000 == 0x5280_0000 && m & 31 == len_reg).then(|| (m >> 5) & 0xffff)
        };
        let len = (1..=8).find_map(|d| movz(k + 1 + d).or_else(|| k.checked_sub(d).and_then(movz)));
        let consumed = words.iter().skip(k + 2).take(10).any(|&m| {
            m & 0x7c00_0000 == 0x1400_0000 || m & 0xffff_fc1f == 0xd63f_0000 || m == 0xd65f_03c0
        });
        if let Some(len) = len.filter(|_| consumed) {
            push(regions, ptr, u64::from(len), out);
        }
    }
}

fn push(regions: &[RoRegion], ptr: u64, len: u64, out: &mut Vec<StringFact>) {
    if !(2..=4096).contains(&len) {
        return;
    }
    let Some(bytes) = regions.iter().find(|r| r.contains(ptr)).and_then(|r| r.bytes(ptr, len)) else { return };
    if is_text(bytes) {
        out.push(StringFact { addr: ptr, len: len as u32 });
    }
}

/// Every code-side `(ptr, len)` literal of the image.
pub fn scan(file: &object::File) -> Vec<StringFact> {
    let regions = ro_regions(file);
    let mut out = Vec::new();
    let arch = file.architecture();
    for sec in file.sections().filter(|s| s.kind() == SectionKind::Text) {
        let Ok(code) = sec.data() else { continue };
        match arch {
            object::Architecture::X86_64 => x86_hits(sec.address(), code, &regions, &mut out),
            object::Architecture::Aarch64 => a64_hits(sec.address(), code, &regions, &mut out),
            _ => {}
        }
    }
    out.sort_by_key(|f| (f.addr, f.len));
    out.dedup_by_key(|f| f.addr);
    // A literal referenced inside another ends that one, as the NUL-scan rows
    // are split at every bounded start (a `fmt::Arguments` template spans
    // several pieces).
    let starts: Vec<u64> = out.iter().map(|f| f.addr).collect();
    for (f, next) in out.iter_mut().zip(starts.iter().skip(1)) {
        if *next < f.addr + u64::from(f.len) {
            f.len = (*next - f.addr) as u32;
        }
    }
    out.retain(|f| f.len >= 2);
    out
}

/// (kuna `fatstr`) The code-side length pass, registered beside the record pass
/// for rustc images and committed under the same gate.
pub struct StrCodeLenPass;

impl AnalysisPass for StrCodeLenPass {
    fn phase(&self) -> Phase {
        Phase::P1
    }
    fn id(&self) -> &'static str {
        "strcodelen"
    }
    fn run(&self, ctx: &AnalysisCtx) -> AnalysisOutput {
        AnalysisOutput { bounded_strings: scan(ctx.file), ..AnalysisOutput::default() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn x86_lea_then_mov_esi() {
        let text = b"hello world!\0";
        let regions = vec![RoRegion { vma: 0x2000, data: text }];
        // lea rdi,[rip+X] at 0x1000 (len 7) -> 0x2000 ; mov esi, 5
        let disp = (0x2000i64 - 0x1007) as i32;
        let mut code = vec![0x48, 0x8d, 0x3d];
        code.extend_from_slice(&disp.to_le_bytes());
        code.extend_from_slice(&[0xbe, 5, 0, 0, 0]);
        let mut out = Vec::new();
        x86_hits(0x1000, &code, &regions, &mut out);
        assert!(out.is_empty(), "no call consumes the pair");
        code.extend_from_slice(&[0xe8, 0, 0, 0, 0]);
        x86_hits(0x1000, &code, &regions, &mut out);
        assert_eq!(out, vec![StringFact { addr: 0x2000, len: 5 }]);
    }

    #[test]
    fn a64_adrp_add_movz() {
        let text = b"hello world!\0";
        let regions = vec![RoRegion { vma: 0x2010, data: text }];
        // adrp x0, #0x1000 (pc 0x1000 -> page 0x2000); add x0, x0, #0x10; mov w1, #5
        let adrp = 0x9000_0000u32 | (1 << 29);
        let add = 0x9100_0000u32 | (0x10 << 10);
        let mov = 0x5280_0000u32 | (5 << 5) | 1;
        let bl = 0x9400_0000u32;
        let code: Vec<u8> = [adrp, add, mov, bl].iter().flat_map(|w| w.to_le_bytes()).collect();
        let mut out = Vec::new();
        a64_hits(0x1000, &code, &regions, &mut out);
        assert_eq!(out, vec![StringFact { addr: 0x2010, len: 5 }]);
    }
}
