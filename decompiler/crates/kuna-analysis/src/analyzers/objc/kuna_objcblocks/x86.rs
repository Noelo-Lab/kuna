//! (kuna) Block recovery on x86-64 Mach-O: the creator loads the isa from its
//! GOT slot (`mov r, [rip + slot]`), takes the invoke and descriptor addresses
//! with `lea r, [rip + addr]`, and stores them to its frame through `rbp`,
//! `rsp` or a register holding a frame address. [`sites`] decodes only the
//! functions that reference an isa slot or a global block literal, with a
//! length decoder for the whole instruction set and a register state for the
//! instructions a block is built from. Frame offsets are relative to `rsp` at
//! the function's entry.

use std::collections::HashMap;

use super::{ByrefCell, Found, Held, Scan, Site};
use crate::analyzers::objc::sections::MachoImage;

const RAX: usize = 0;
const RCX: usize = 1;
const RDX: usize = 2;
const RBX: usize = 3;
const RSP: usize = 4;
const RBP: usize = 5;
const RSI: usize = 6;
const RDI: usize = 7;
/// The registers a call does not preserve (System V).
const CALLER_SAVED: [usize; 9] = [RAX, RCX, RDX, RSI, RDI, 8, 9, 10, 11];

#[derive(Clone, Copy, PartialEq, Debug)]
enum Base {
    Rip,
    Reg(usize),
    None,
}

/// A memory operand; `index` when it has an index register (or 32-bit
/// addressing), which no frame access of a block literal uses.
#[derive(Clone, Copy, PartialEq, Debug)]
struct Mem {
    base: Base,
    index: bool,
    disp: i64,
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Rm {
    Reg(usize),
    Mem(Mem),
}

/// One decoded instruction: its length, opcode map (0 one-byte, 1 `0F`,
/// 2 `0F 38`, 3 `0F 3A`) and opcode, the mandatory prefix (`pp`: 0 none,
/// 1 `66`, 2 `F3`, 3 `F2`), the ModRM operands and the immediate.
#[derive(Clone, Copy, PartialEq, Debug)]
pub(super) struct Insn {
    pub(super) len: usize,
    map: u8,
    op: u8,
    pp: u8,
    wide: bool,
    opsize16: bool,
    vex_l: bool,
    reg: usize,
    rm: Option<Rm>,
    imm: i64,
}

/// Decode the instruction at the start of `code` (64-bit mode). `None` for
/// bytes that are not one: an opcode invalid in 64-bit mode, EVEX, a
/// truncated or over-long encoding.
pub(super) fn decode(code: &[u8]) -> Option<Insn> {
    let code = &code[..code.len().min(15)];
    let mut i = 0;
    let (mut pp, mut opsize16, mut addr32) = (0u8, false, false);
    loop {
        match *code.get(i)? {
            0x66 => {
                opsize16 = true;
                if pp == 0 {
                    pp = 1;
                }
            }
            0xF3 => pp = 2,
            0xF2 => pp = 3,
            0x67 => addr32 = true,
            0x2E | 0x36 | 0x3E | 0x26 | 0x64 | 0x65 | 0xF0 => {}
            _ => break,
        }
        i += 1;
    }
    let mut rex = 0u8;
    if code[i] & 0xF0 == 0x40 {
        rex = code[i];
        i += 1;
    }
    let next = |i: &mut usize| -> Option<u8> {
        let b = *code.get(*i)?;
        *i += 1;
        Some(b)
    };
    let mut b = next(&mut i)?;
    let (mut map, mut vex_l) = (0u8, false);
    if b == 0xC5 || b == 0xC4 {
        if rex != 0 {
            return None;
        }
        let p = next(&mut i)?;
        rex = 0x40 | ((!p >> 5) & 4);
        map = 1;
        let mut last = p;
        if b == 0xC4 {
            last = next(&mut i)?;
            rex = 0x40 | ((!p >> 5) & 7) | ((last >> 4) & 8);
            map = p & 0x1F;
            if !(1..=3).contains(&map) {
                return None;
            }
        }
        vex_l = last & 4 != 0;
        pp = last & 3;
        opsize16 = false;
        b = next(&mut i)?;
    } else if b == 0x0F {
        map = 1;
        b = next(&mut i)?;
        if b == 0x38 || b == 0x3A {
            map = if b == 0x38 { 2 } else { 3 };
            b = next(&mut i)?;
        }
    }
    let wide = rex & 8 != 0;
    let immz = if opsize16 && !wide { 2 } else { 4 };
    // Whether a ModRM byte follows, and the immediate's size; `F6`/`F7` have
    // one only for `test` (`/0`, `/1`), added once the ModRM byte is read.
    let (modrm, mut immn): (bool, usize) = match map {
        0 => match b {
            0x00..=0x3F => match b & 7 {
                0..=3 => (true, 0),
                4 => (false, 1),
                5 => (false, immz),
                _ => return None,
            },
            0x50..=0x5F | 0x6C..=0x6F | 0x90..=0x99 | 0x9B..=0x9F | 0xA4..=0xA7 | 0xAA..=0xAF => (false, 0),
            0xC3 | 0xC9 | 0xCB | 0xCC | 0xCF | 0xD7 | 0xEC..=0xEF | 0xF1 | 0xF4 | 0xF5 | 0xF8..=0xFD => (false, 0),
            0x63 | 0x84..=0x8F | 0xD0..=0xD3 | 0xD8..=0xDF | 0xF6 | 0xF7 | 0xFE | 0xFF => (true, 0),
            0x68 | 0xA9 => (false, immz),
            0x69 | 0x81 | 0xC7 => (true, immz),
            0x6A | 0x70..=0x7F | 0xA8 | 0xB0..=0xB7 | 0xCD | 0xE0..=0xE7 | 0xEB => (false, 1),
            0x6B | 0x80 | 0x83 | 0xC0 | 0xC1 | 0xC6 => (true, 1),
            0xA0..=0xA3 => (false, if addr32 { 4 } else { 8 }),
            0xB8..=0xBF => (false, if wide { 8 } else { immz }),
            0xC2 | 0xCA => (false, 2),
            0xC8 => (false, 3),
            0xE8 | 0xE9 => (false, 4),
            _ => return None,
        },
        1 => match b {
            0x05..=0x09 | 0x0B | 0x0E | 0x30..=0x35 | 0x77 | 0xA0..=0xA2 | 0xA8..=0xAA | 0xC8..=0xCF => (false, 0),
            0x80..=0x8F => (false, 4),
            0x70..=0x73 | 0xA4 | 0xAC | 0xBA | 0xC2 | 0xC4..=0xC6 => (true, 1),
            0x04 | 0x0A | 0x0C | 0x0F | 0x24..=0x27 | 0x36..=0x3F | 0x7A | 0x7B | 0xA6 | 0xA7 => return None,
            _ => (true, 0),
        },
        2 => (true, 0),
        _ => (true, 1),
    };
    let mut reg = if map == 0 && !modrm { (b & 7) as usize | ((rex & 1) as usize) << 3 } else { 0 };
    let mut rm = None;
    if modrm {
        let m = next(&mut i)?;
        let (md, low) = (m >> 6, (m & 7) as usize);
        let ext = ((rex & 1) as usize) << 3;
        reg = ((m >> 3) & 7) as usize | ((rex & 4) as usize) << 1;
        if map == 0 && (b == 0xF6 || b == 0xF7) && reg & 6 == 0 {
            immn = if b == 0xF6 { 1 } else { immz };
        }
        rm = Some(if md == 3 {
            Rm::Reg(low | ext)
        } else {
            let mut base = Base::Reg(low | ext);
            let mut index = addr32;
            let mut dispn = [0usize, 1, 4][md as usize];
            if low == 4 {
                let sib = next(&mut i)?;
                index |= ((sib >> 3) & 7) as usize | ((rex & 2) as usize) << 2 != 4;
                base = Base::Reg((sib & 7) as usize | ext);
                if sib & 7 == 5 && md == 0 {
                    base = Base::None;
                    dispn = 4;
                }
            } else if low == 5 && md == 0 {
                base = Base::Rip;
                dispn = 4;
            }
            let disp = signed(code.get(i..i.checked_add(dispn)?)?);
            i += dispn;
            Rm::Mem(Mem { base, index, disp })
        });
    }
    let imm = signed(code.get(i..i.checked_add(immn)?)?);
    Some(Insn { len: i + immn, map, op: b, pp, wide, opsize16, vex_l, reg, rm, imm })
}

/// A little-endian signed integer of up to eight bytes.
fn signed(bytes: &[u8]) -> i64 {
    let mut raw = [0u8; 8];
    raw[..bytes.len()].copy_from_slice(bytes);
    let shift = 64 - 8 * bytes.len() as u32;
    if shift == 64 { 0 } else { (i64::from_le_bytes(raw) << shift) >> shift }
}

impl Insn {
    /// `ret`, or a jump that never comes back to the next instruction.
    fn ends_path(&self) -> bool {
        match (self.map, self.op) {
            (0, 0xC2 | 0xC3 | 0xE9 | 0xEB) => true,
            (0, 0xFF) => self.reg & 6 == 4,
            _ => false,
        }
    }

    fn is_call(&self) -> bool {
        self.map == 0 && (self.op == 0xE8 || self.op == 0xFF && self.reg & 6 == 2)
    }

    /// `ret`, or a jump out of the text or through a slot: a function ends.
    fn leaves(&self, pc: u64, is_code: &dyn Fn(u64) -> bool) -> bool {
        match (self.map, self.op, self.transfer(pc)) {
            (0, 0xC2 | 0xC3, _) | (0, 0xFF, Some(Transfer::Slot(_))) => self.reg & 7 != 2,
            (0, 0xE9, Some(Transfer::Direct(target))) => !is_code(target),
            _ => false,
        }
    }

    /// The address a `rip`-relative operand names.
    fn rip_target(&self, pc: u64) -> Option<u64> {
        match self.rm {
            Some(Rm::Mem(Mem { base: Base::Rip, disp, .. })) => Some(pc.wrapping_add(self.len as u64).wrapping_add(disp as u64)),
            _ => None,
        }
    }

    /// Where a direct or `rip`-indirect call or jump goes: the target, or the
    /// slot holding it.
    fn transfer(&self, pc: u64) -> Option<Transfer> {
        match (self.map, self.op) {
            (0, 0xE8 | 0xE9) => Some(Transfer::Direct(pc.wrapping_add(self.len as u64).wrapping_add(self.imm as u64))),
            (0, 0xFF) if matches!(self.reg & 7, 2 | 4) => self.rip_target(pc).map(Transfer::Slot),
            _ => None,
        }
    }

    /// The width of the operand a general-register instruction moves.
    fn width(&self) -> u8 {
        if self.wide { 8 } else if self.opsize16 { 2 } else { 4 }
    }

    /// The width an SSE/AVX store writes, when this is one.
    fn vector_store(&self) -> Option<u8> {
        let full = if self.vex_l { 32 } else { 16 };
        match (self.map, self.op, self.pp) {
            (1, 0x11, 2) => Some(4),
            (1, 0x11, 3) | (1, 0x13 | 0x17, _) | (1, 0xD6, 1) => Some(8),
            (1, 0x11 | 0x29 | 0x2B, _) | (1, 0x7F | 0xE7, 1) | (1, 0x7F, 2) => Some(full),
            (1, 0x7E, 1) => Some(if self.wide { 8 } else { 4 }),
            _ => None,
        }
    }

    /// The general registers an instruction the scan does not model may
    /// write, as a bit per register: more than it does write is safe, fewer
    /// is not.
    fn clobbers(&self) -> u16 {
        let bit = |r: usize| 1u16 << (r & 15);
        let set = |rs: &[usize]| rs.iter().fold(0, |m, &r| m | bit(r));
        let rm = match self.rm {
            Some(Rm::Reg(r)) => bit(r),
            _ => 0,
        };
        let both = bit(self.reg) | rm;
        match (self.map, self.op) {
            (0, 0x38..=0x3B | 0x84 | 0x85) => 0,
            (0, 0x80..=0x83) if self.reg & 7 == 7 => 0,
            (0, 0xF6 | 0xF7) => (if self.reg & 6 != 0 { rm } else { 0 }) | (if self.reg & 4 != 0 { set(&[RAX, RDX]) } else { 0 }),
            (0, 0x80..=0x83 | 0x8F | 0xC0 | 0xC1 | 0xC6 | 0xC7 | 0xD0..=0xD3 | 0xFE | 0xFF) => rm,
            (0, 0xD8..=0xDF) => bit(RAX),
            (0, _) if self.rm.is_some() => both,
            (0, 0x91..=0x97 | 0xB0..=0xBF) => bit(RAX) | bit(self.reg),
            (0, 0x50..=0x57 | 0x68 | 0x6A | 0x70..=0x7F | 0x90 | 0xA8 | 0xA9 | 0xE3 | 0xEB | 0xE9 | 0x3C | 0x3D) => 0,
            (0, 0x58..=0x5F) => bit(self.reg),
            (0, 0x9C | 0x9D | 0xC8) => bit(RSP),
            (0, _) => set(&[RAX, RCX, RDX, RSI, RDI]),
            (1, 0x10..=0x17 | 0x18..=0x1F | 0x28..=0x2B | 0x2E | 0x2F | 0x51..=0x7D | 0x7F | 0x80..=0x8F | 0xAE | 0xC2 | 0xC4 | 0xC6) => 0,
            (1, 0xD0..=0xFF) if self.op != 0xD7 => 0,
            (1, 0x00 | 0x01 | 0x90..=0x9F | 0xBA) => rm,
            (1, 0xC8..=0xCF) => bit((self.op & 7) as usize | (self.reg & 8)),
            (1, 0x05 | 0x31..=0x33 | 0xA2 | 0xB0 | 0xB1 | 0xC7) => set(&[RAX, RCX, RDX, RBX, 11]) | both,
            (1, _) if self.rm.is_some() => both,
            (1, _) => set(&[RAX, RCX, RDX]),
            (2, 0xF0..=0xFF) | (3, 0x14..=0x17 | 0xF0) => both,
            _ => 0,
        }
    }
}

enum Transfer {
    Direct(u64),
    Slot(u64),
}

/// The frame state of one function: `rsp` relative to its entry value, and
/// the `rsp`/`rbp` an epilogue is about to undo, restored for the code after
/// the `ret` (another path of the same function).
struct Frame {
    sp: Option<i64>,
    body: Option<(Option<i64>, Option<Held>)>,
}

impl Frame {
    fn held(&self, scan: &Scan, r: usize) -> Option<Held> {
        if r == RSP { self.sp.map(|s| Held::Stack(s as u64)) } else { scan.regs[r] }
    }

    fn set(&mut self, scan: &mut Scan, r: usize, held: Option<Held>) {
        if r == RSP {
            self.sp = match held {
                Some(Held::Stack(off)) => Some(off as i64),
                _ => None,
            };
        } else {
            scan.regs[r] = held;
        }
    }

    /// The frame offset a memory operand names.
    fn slot(&self, scan: &Scan, m: Mem) -> Option<u64> {
        match (m.index, m.base) {
            (false, Base::Reg(r)) => match self.held(scan, r) {
                Some(Held::Stack(b)) => Some(b.wrapping_add(m.disp as u64)),
                _ => None,
            },
            _ => None,
        }
    }

    fn step(&mut self, scan: &mut Scan, pc: u64, insn: &Insn) {
        let epilogue = self.body;
        self.body = None;
        let narrow = |held: Option<Held>, width: u8| match held {
            Some(Held::Imm(v)) if width == 4 => Some(Held::Imm(v & 0xFFFF_FFFF)),
            held if width == 8 => held,
            _ => None,
        };
        let width = insn.width();
        if let (Some(w), Some(Rm::Mem(m))) = (insn.vector_store(), insn.rm) {
            if let Some(off) = self.slot(scan, m) {
                scan.store_held(pc, off, w, true, None, None);
            }
            return;
        }
        match (insn.map, insn.op, insn.rm) {
            (0, 0x8B, Some(Rm::Reg(src))) | (0, 0x89, Some(Rm::Reg(src))) => {
                let (dst, src) = if insn.op == 0x8B { (insn.reg, src) } else { (src, insn.reg) };
                let held = narrow(self.held(scan, src), width);
                if dst == RSP {
                    self.body = epilogue.or(Some((self.sp, scan.regs[RBP])));
                }
                self.set(scan, dst, held);
            }
            (0, 0x8B, Some(Rm::Mem(_))) => {
                let isa = insn.wide && insn.rip_target(pc).is_some_and(|slot| scan.isa_slots.contains(&slot));
                self.set(scan, insn.reg, isa.then_some(Held::Isa));
            }
            (0, 0x89, Some(Rm::Mem(m))) => {
                if let Some(off) = self.slot(scan, m) {
                    let held = narrow(self.held(scan, insn.reg), width);
                    scan.store_held(pc, off, width, false, held, None);
                }
            }
            (0, 0x88 | 0xC6, Some(Rm::Mem(m))) => {
                if let Some(off) = self.slot(scan, m) {
                    scan.store_held(pc, off, 1, false, None, None);
                }
            }
            (0, 0xC7, Some(rm)) if insn.reg & 7 == 0 => {
                let held = narrow(Some(Held::Imm(insn.imm as u64)), width);
                match rm {
                    Rm::Reg(r) => self.set(scan, r, held),
                    Rm::Mem(m) => {
                        if let Some(off) = self.slot(scan, m) {
                            scan.store_held(pc, off, width, false, held, None);
                        }
                    }
                }
            }
            (0, 0xB8..=0xBF, None) => {
                let held = narrow(Some(Held::Imm(insn.imm as u64)), width);
                self.set(scan, insn.reg, held);
            }
            (0, 0x8D, Some(Rm::Mem(m))) => {
                let held = match insn.rip_target(pc) {
                    Some(a) => {
                        scan.global_ref(pc, a);
                        Some(Held::Addr(a))
                    }
                    None => self.slot(scan, m).map(Held::Stack),
                };
                if insn.reg == RSP {
                    self.body = epilogue.or(Some((self.sp, scan.regs[RBP])));
                }
                self.set(scan, insn.reg, if insn.wide { held } else { None });
            }
            (0, 0x31 | 0x33, Some(Rm::Reg(r))) if r == insn.reg => self.set(scan, r, Some(Held::Imm(0))),
            (0, 0x50..=0x57 | 0x68 | 0x6A, None) => self.sp = self.sp.map(|s| s.wrapping_sub(8)),
            (0, 0xFF, Some(_)) if insn.reg & 7 == 6 => self.sp = self.sp.map(|s| s.wrapping_sub(8)),
            (0, 0x58..=0x5F, None) => {
                self.body = epilogue.or(Some((self.sp, scan.regs[RBP])));
                let sp = self.sp.map(|s| s.wrapping_add(8));
                self.set(scan, insn.reg, None);
                self.sp = sp;
            }
            (0, 0xC9, None) => {
                self.body = epilogue.or(Some((self.sp, scan.regs[RBP])));
                self.sp = match scan.regs[RBP] {
                    Some(Held::Stack(b)) => Some((b as i64).wrapping_add(8)),
                    _ => None,
                };
                scan.regs[RBP] = None;
            }
            (0, 0x81 | 0x83, Some(Rm::Reg(r))) if matches!(insn.reg & 7, 0 | 5) => {
                let by = if insn.reg & 7 == 0 { insn.imm } else { insn.imm.wrapping_neg() };
                if r == RSP && by > 0 {
                    self.body = epilogue.or(Some((self.sp, scan.regs[RBP])));
                }
                let held = match self.held(scan, r) {
                    Some(Held::Stack(b)) if insn.wide => Some(Held::Stack(b.wrapping_add(by as u64))),
                    _ => None,
                };
                self.set(scan, r, held);
            }
            _ if insn.is_call() => {
                for r in CALLER_SAVED {
                    scan.regs[r] = None;
                }
            }
            _ if insn.ends_path() => {
                if let Some((sp, rbp)) = epilogue {
                    self.sp = sp;
                    scan.regs[RBP] = rbp;
                }
                for r in 0..16 {
                    if !matches!(scan.regs[r], Some(Held::Stack(_))) {
                        scan.regs[r] = None;
                    }
                }
            }
            _ => {
                let written = insn.clobbers();
                for r in (0..16).filter(|r| written & 1 << r != 0) {
                    self.set(scan, r, None);
                }
            }
        }
    }
}

/// The `(section, start, end)` of each function of `text` holding four bytes
/// that, as a `rip`-relative displacement, land on one of `targets` (sorted):
/// the functions worth decoding.
fn referencing_functions(text: &[(u64, &[u8])], targets: &[u64], starts: &[u64]) -> Vec<(usize, u64, u64)> {
    let (Some(&lo), Some(&hi)) = (targets.first(), targets.last()) else { return Vec::new() };
    let mut out: Vec<(usize, u64, u64)> = Vec::new();
    for (k, &(base, data)) in text.iter().enumerate() {
        let end = base.saturating_add(data.len() as u64);
        let mut skip_to = 0u64;
        for (i, w) in data.windows(4).enumerate() {
            let at = base.wrapping_add(i as u64);
            let target = at.wrapping_add(4).wrapping_add(i32::from_le_bytes([w[0], w[1], w[2], w[3]]) as i64 as u64);
            if target < lo || target > hi || at < skip_to || targets.binary_search(&target).is_err() {
                continue;
            }
            let next = starts.partition_point(|&s| s <= at);
            let Some(&start) = next.checked_sub(1).and_then(|n| starts.get(n)) else { continue };
            if start < base {
                continue;
            }
            let stop = starts.get(next).copied().unwrap_or(end).min(end);
            out.push((k, start, stop));
            skip_to = stop;
        }
    }
    out
}

/// The stack blocks, `__block` cells and global-block references of every
/// x86-64 function that names an isa slot or a global literal. `starts`
/// (sorted) bounds each function when the image lists them all (`listed`);
/// otherwise the code after a `ret` may be another function's, and a literal
/// built there has no known creator. Bytes that do not decode are stepped
/// over with the state cleared.
pub(super) fn sites(
    read_ptr: &dyn Fn(u64) -> Option<u64>,
    is_code: &dyn Fn(u64) -> bool,
    text: &[(u64, &[u8])],
    isa_slots: &[u64],
    globals: &[u64],
    starts: &[u64],
    listed: bool,
) -> Found {
    let mut targets: Vec<u64> = isa_slots.iter().chain(globals).copied().collect();
    targets.sort_unstable();
    targets.dedup();
    let (mut sites, mut cells): (Vec<Site>, Vec<ByrefCell>) = (Vec::new(), Vec::new());
    let mut scan = Scan::new(read_ptr, is_code, isa_slots, globals);
    for (k, start, stop) in referencing_functions(text, &targets, starts) {
        let (base, data) = text[k];
        let mut frame = Frame { sp: Some(0), body: None };
        scan.start = Some(start);
        scan.last_ret = 0;
        let mut pc = start;
        while pc < stop {
            let Some(insn) = data.get((pc - base) as usize..(stop - base) as usize).and_then(decode) else {
                scan.flush(&mut sites, &mut cells);
                frame = Frame { sp: None, body: None };
                pc += 1;
                continue;
            };
            frame.step(&mut scan, pc, &insn);
            if !listed && insn.leaves(pc, is_code) {
                scan.last_ret = pc;
                scan.start = None;
            }
            pc += insn.len as u64;
        }
        scan.flush(&mut sites, &mut cells);
    }
    Found { sites, cells, global_refs: scan.global_refs }
}

/// What an x86-64 copy helper says each capture is: `(offset, kind)` as
/// [`super::copy_helper_kinds`] reads them from arm64 code. The capture is
/// the last load into `rdi` (`objc_retain`, `objc_retainBlock`) or `rsi`
/// (`_Block_object_assign`, its flags in `edx`).
pub(super) fn copy_helper_kinds(img: &MachoImage, helper: u64, runtime: &HashMap<u64, String>) -> Vec<(u64, u32)> {
    let mut out = Vec::new();
    let mut loaded: [Option<u64>; 16] = [None; 16];
    let mut imm: [Option<u32>; 16] = [None; 16];
    let mut pc = helper;
    for _ in 0..96 {
        let Some(insn) = (1..=15).rev().find_map(|n| img.read(pc, n)).and_then(decode) else { break };
        match (insn.map, insn.op, insn.rm) {
            (0, 0x8B, Some(Rm::Mem(m))) => {
                loaded[insn.reg] = (insn.wide && !m.index && m.base != Base::Rip).then_some(m.disp as u64);
                imm[insn.reg] = None;
            }
            (0, 0x8B, Some(Rm::Reg(src))) => (loaded[insn.reg], imm[insn.reg]) = (loaded[src], imm[src]),
            (0, 0x89, Some(Rm::Reg(dst))) => (loaded[dst], imm[dst]) = (loaded[insn.reg], imm[insn.reg]),
            (0, 0xB8..=0xBF, None) => (loaded[insn.reg], imm[insn.reg]) = (None, Some(insn.imm as u32)),
            _ => {
                let name = match insn.transfer(pc) {
                    Some(Transfer::Direct(target)) => runtime.get(&target).map(String::as_str),
                    Some(Transfer::Slot(slot)) => img.bind_name(slot).map(|n| n.strip_prefix('_').unwrap_or(n)),
                    None => None,
                };
                match name {
                    Some("objc_retain") => out.extend(loaded[RDI].map(|o| (o, 3))),
                    Some("objc_retainBlock") => out.extend(loaded[RDI].map(|o| (o, 7))),
                    Some("_Block_object_assign") => out.extend(loaded[RSI].zip(imm[RDX])),
                    _ => {}
                }
                if insn.is_call() {
                    for r in CALLER_SAVED {
                        (loaded[r], imm[r]) = (None, None);
                    }
                } else if insn.ends_path() {
                    break;
                } else {
                    let written = insn.clobbers();
                    for r in (0..16).filter(|r| written & 1 << r != 0) {
                        (loaded[r], imm[r]) = (None, None);
                    }
                }
            }
        }
        pc = pc.wrapping_add(insn.len as u64);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bytes(hex: &str) -> Vec<u8> {
        let hex: String = hex.split_whitespace().collect();
        (0..hex.len() / 2).map(|i| u8::from_str_radix(&hex[2 * i..2 * i + 2], 16).unwrap()).collect()
    }

    /// One encoding of each operand shape, padded so the length is the
    /// decoder's and not the slice's.
    #[test]
    fn lengths_follow_the_encoding() {
        for hex in [
            "55",
            "4889e5",
            "4883ec30",
            "488b0d13170000",
            "48894dd8",
            "b9000000c0",
            "48b90000002020000000",
            "48c745d800000000",
            "c7459000000042",
            "66c745900100",
            "f20f1145c8",
            "0f2945b0",
            "c5fb1145c8",
            "c4e37d0645b001",
            "ff15d3160000",
            "e8e8000000",
            "0f8412010000",
            "f7c100010000",
            "f7d9",
            "f60701",
            "a1954 4ef2322653 83b",
            "6666662e0f1f840000000000",
            "48890424",
            "4c8b7c2408",
            "8b04c5 10000000",
            "f3c3",
            "c20800",
        ] {
            let insn = bytes(hex);
            let mut padded = insn.clone();
            padded.extend([0xCC; 16]);
            assert_eq!(decode(&padded).map(|i| i.len), Some(insn.len()), "{hex}");
        }
        for hex in ["06", "62f1fd08", "0f0f008e", "d5", "4889", "e8000000", "48"] {
            assert_eq!(decode(&bytes(hex)), None, "{hex}");
        }
        assert_eq!(decode(&[]), None);
    }

    /// `-[Runner maker:]` of `macho_objc_blocks_x86_64` at 0x1000008e0: the isa
    /// from its GOT slot stored at `[rbp - 0x28]`, the flags from `ecx`, the
    /// invoke and descriptor from `lea`, the capture at `[rbp - 8]`.
    #[test]
    fn rbp_frame_stores_build_the_literal() {
        let code = bytes(
            "55 4889e5 4883ec30 488b07 4801d0 488b0d13170000 48894dd8 b9000000c0 48894de0
             488d0d37000000 48894de8 488d0d1c170000 48894df0 488945f8 488b35f5260000
             488d4dd8 ff15d3160000 4883c430 5d c3",
        );
        let is_code = |a: u64| (0x1_0000_0000..0x1_0000_1000).contains(&a);
        let found = sites(&|_| None, &is_code, &[(0x1_0000_08e0, &code)], &[0x1_0000_2008], &[], &[0x1_0000_08e0], true);
        assert_eq!(found.sites.len(), 1);
        let s = &found.sites[0];
        assert_eq!((s.pc, s.start, s.depth, s.frame as i64), (0x1_0000_08f5, Some(0x1_0000_08e0), 0, -0x30));
        assert_eq!((s.invoke, s.descriptor), (0x1_0000_0940, Some(0x1_0000_2030)));
        let caps: Vec<(u64, u8)> = s.captures.iter().map(|c| (c.off.wrapping_sub(s.frame), c.width)).collect();
        assert_eq!(caps, [(32, 8)]);
        let none = sites(&|_| None, &is_code, &[(0x1_0000_08e0, &code)], &[0x1_0000_2008], &[], &[], true);
        assert!(none.sites.is_empty(), "no function start bounds the decode");
        let other = sites(&|_| None, &is_code, &[(0x1_0000_08e0, &code)], &[0x1_0000_2010], &[], &[0x1_0000_08e0], true);
        assert!(other.sites.is_empty(), "another slot is not the isa");
    }

    /// A frame without `rbp`: `push rbx; sub rsp, 0x30` puts `[rsp + 8]` 0x30
    /// below the entry `rsp`, and after the epilogue's `ret` the next path has
    /// the body's frame again. A global literal's `lea` is a reference to it.
    /// In an image that does not list its function starts the second literal
    /// may be another function's.
    #[test]
    fn rsp_frame_and_global_reference() {
        let code = bytes(
            "53 4883ec30 4c8b25fc0f0000 4c89642408 488d05eb000000 4889442418 488d0ddc1f0000
             48894c2420 e800000000 4883c430 5b c3
             4c8b25cd0f0000 4c89642408 488d05bd000000 4889442418 488d3ded1f0000 4883c430 5b c3",
        );
        let is_code = |a: u64| (0x1000..0x2000).contains(&a);
        let found = sites(&|_| None, &is_code, &[(0x1000, &code)], &[0x2008], &[0x3040], &[0x1000], true);
        let got: Vec<(u64, i64, u64, Option<u64>)> = found.sites.iter().map(|s| (s.pc, s.frame as i64, s.invoke, s.descriptor)).collect();
        assert_eq!(got, [(0x100c, -0x30, 0x1103, Some(0x3000)), (0x103b, -0x30, 0x1104, None)]);
        assert_eq!(found.global_refs, [(0x3040, 0x104c, 0, Some(0x1000))]);
        let unlisted = sites(&|_| None, &is_code, &[(0x1000, &code)], &[0x2008], &[0x3040], &[0x1000], false);
        let creators: Vec<(u64, Option<u64>)> = unlisted.sites.iter().map(|s| (s.last_ret, s.start)).collect();
        assert_eq!(creators, [(0, Some(0x1000)), (0x1033, None)], "past a ret the creator is unknown");
    }

    /// What the scan does not model clears what it may write and nothing else.
    #[test]
    fn unmodelled_writes_clear_their_registers() {
        let written = |hex: &str| {
            let mask = decode(&bytes(hex)).unwrap().clobbers();
            (0..16).filter(|r| mask & 1 << r != 0).collect::<Vec<usize>>()
        };
        assert_eq!(written("4801d0"), [RAX, RDX], "add rax, rdx");
        assert_eq!(written("4839d0"), [] as [usize; 0], "cmp rax, rdx");
        assert_eq!(written("4883e4f0"), [RSP], "and rsp, -16");
        assert_eq!(written("4883fd10"), [] as [usize; 0], "cmp rbp, 16");
        assert_eq!(written("48f7f1"), [RAX, RCX, RDX], "div rcx");
        assert_eq!(written("660f6fe5"), [] as [usize; 0], "movdqa xmm4, xmm5");
        assert_eq!(written("5b"), [RBX], "pop rbx");
        assert_eq!(written("480fafc6"), [RAX, RSI], "imul rax, rsi");
    }
}
