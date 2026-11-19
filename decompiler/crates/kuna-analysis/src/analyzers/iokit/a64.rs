//! A forward AArch64 register tracker: just enough of the instruction set to
//! follow how XNU code builds constant addresses (`ADRP`/`ADD`/`ADR`, `MOV`,
//! `MOVZ`, `LDR` from a constant slot) into the argument registers of a call.
//! Any other instruction that writes a general register makes it unknown.

use crate::objc::sections::MachoImage;

/// A direct call (`BL`, or `B` as a tail call) with the first four argument
/// registers as far as they are known.
pub(super) struct Call {
    pub target: u64,
    pub args: [Option<u64>; 4],
}

/// A 64-bit store of a known value to a known address.
pub(super) struct Store {
    pub addr: u64,
    pub value: u64,
}

pub(super) enum Event {
    Call(Call),
    Store(Store),
}

#[derive(Clone, Default)]
struct Regs([Option<u64>; 32]);

impl Regs {
    fn get(&self, r: u32) -> Option<u64> {
        if r == 31 { None } else { self.0[r as usize] }
    }

    fn set(&mut self, r: u32, v: Option<u64>) {
        if r != 31 {
            self.0[r as usize] = v;
        }
    }

    fn clobber_call(&mut self) {
        for r in 0..=18 {
            self.0[r] = None;
        }
        self.0[30] = None;
    }
}

fn sext(v: u64, bits: u32) -> i64 {
    ((v << (64 - bits)) as i64) >> (64 - bits)
}

fn rd(ins: u32) -> u32 {
    ins & 31
}

fn rn(ins: u32) -> u32 {
    (ins >> 5) & 31
}

fn adr_imm(ins: u32) -> i64 {
    sext((((ins >> 29) & 3) | (((ins >> 5) & 0x7ffff) << 2)) as u64, 21)
}

enum Step {
    Next,
    Stop,
}

/// Execute one instruction at `pc` on `regs`, reporting calls and stores.
fn step(img: &MachoImage, pc: u64, ins: u32, regs: &mut Regs, events: &mut Vec<Event>, ctor: Option<u64>) -> Step {
    let branch = |ins: u32| pc.wrapping_add((sext((ins & 0x03ff_ffff) as u64, 26) * 4) as u64);
    let args = |regs: &Regs| [regs.get(0), regs.get(1), regs.get(2), regs.get(3)];
    match ins {
        _ if ins & 0x9f00_0000 == 0x9000_0000 => {
            regs.set(rd(ins), Some((pc & !0xfff).wrapping_add((adr_imm(ins) << 12) as u64)));
        }
        _ if ins & 0x9f00_0000 == 0x1000_0000 => regs.set(rd(ins), Some(pc.wrapping_add(adr_imm(ins) as u64))),
        _ if ins & 0xff80_0000 == 0x9100_0000 => {
            let imm = ((ins >> 10) & 0xfff) as u64;
            let imm = if ins & (1 << 22) != 0 { imm << 12 } else { imm };
            let v = regs.get(rn(ins)).map(|b| b.wrapping_add(imm));
            regs.set(rd(ins), v);
        }
        _ if ins & 0x7f80_0000 == 0x5280_0000 => {
            regs.set(rd(ins), Some((((ins >> 5) & 0xffff) as u64) << (16 * ((ins >> 21) & 3))));
        }
        _ if ins & 0x7fe0_ffe0 == 0x2a00_03e0 => {
            let rm = (ins >> 16) & 31;
            let v = if rm == 31 { Some(0) } else { regs.get(rm) };
            let v = if ins & (1 << 31) == 0 { v.map(|v| v & 0xffff_ffff) } else { v };
            regs.set(rd(ins), v);
        }
        _ if ins & 0xffc0_0000 == 0xf940_0000 => {
            let addr = regs.get(rn(ins)).map(|b| b.wrapping_add((((ins >> 10) & 0xfff) as u64) * 8));
            regs.set(rd(ins), addr.and_then(|a| img.read_ptr(a)));
        }
        _ if ins & 0xff00_0000 == 0x5800_0000 => {
            let addr = pc.wrapping_add((sext(((ins >> 5) & 0x7ffff) as u64, 19) * 4) as u64);
            regs.set(rd(ins), img.read_ptr(addr));
        }
        _ if ins & 0xffc0_0000 == 0xf900_0000 => {
            let addr = regs.get(rn(ins)).map(|b| b.wrapping_add((((ins >> 10) & 0xfff) as u64) * 8));
            if let (Some(addr), Some(value)) = (addr, regs.get(rd(ins))) {
                events.push(Event::Store(Store { addr, value }));
            }
        }
        // PAC / AUT / XPAC sign or strip a pointer in place; statically the value is unchanged.
        _ if ins & 0xffff_c000 == 0xdac1_0000 => {}
        _ if ins & 0xfc00_0000 == 0x9400_0000 => {
            let (target, args) = (branch(ins), args(regs));
            regs.clobber_call();
            if Some(target) == ctor {
                regs.set(0, args[0]);
            }
            events.push(Event::Call(Call { target, args }));
        }
        _ if ins & 0xfc00_0000 == 0x1400_0000 => {
            events.push(Event::Call(Call { target: branch(ins), args: args(regs) }));
            return Step::Stop;
        }
        // BLR, BLRAA/BLRAB/BLRAAZ/BLRABZ: an indirect call.
        _ if ins & 0xfefff800 == 0xd63f0800 || ins & 0xfffffc1f == 0xd63f0000 => regs.clobber_call(),
        // RET/RETAA/RETAB, BR and its authenticated forms, ERET.
        _ if ins & 0xfe9f_0000 == 0xd61f_0000 => return Step::Stop,
        _ => clobber_generic(ins, regs),
    }
    Step::Next
}

/// Forget every general register `ins` may write: `Rd` of a data-processing
/// instruction, `Rt`/`Rt2` (and a written-back base) of a load.
fn clobber_generic(ins: u32, regs: &mut Regs) {
    let op0 = (ins >> 25) & 0xf;
    if matches!(op0, 0x8 | 0x9 | 0x5 | 0xd) {
        regs.set(rd(ins), None);
        return;
    }
    if op0 & 0x5 != 0x4 {
        return;
    }
    let load = match (ins >> 27) & 7 {
        0b101 => ins & (1 << 22) != 0,
        0b011 => true,
        _ => (ins >> 22) & 3 != 0,
    };
    if load {
        regs.set(rd(ins), None);
        if (ins >> 27) & 7 == 0b101 {
            regs.set((ins >> 10) & 31, None);
        }
    }
    if (ins >> 27) & 7 == 0b111 && (ins >> 24) & 3 == 0 && (ins >> 10) & 1 == 1 {
        regs.set(rn(ins), None);
    }
}

/// Follow straight-line code from `entry` for at most `budget` instructions,
/// stopping at a return, an indirect or tail branch, or an undecodable address.
/// Conditional branches fall through. A call to `ctor` (a C++ constructor)
/// returns its `this` in `x0`, as Apple's arm64 ABI specifies.
pub(super) fn trace(img: &MachoImage, entry: u64, budget: usize, ctor: Option<u64>) -> Vec<Event> {
    let mut regs = Regs::default();
    let mut events = Vec::new();
    let mut pc = entry;
    for _ in 0..budget {
        let Some(ins) = img.read_u32(pc) else { break };
        if let Step::Stop = step(img, pc, ins, &mut regs, &mut events, ctor) {
            break;
        }
        pc = pc.wrapping_add(4);
    }
    events
}

/// The constant a short leaf function returns in `x0`, if it computes one and
/// returns within `limit` instructions (`getMetaClass() const { return &gMetaClass; }`).
pub(super) fn returned_constant(img: &MachoImage, entry: u64, limit: usize) -> Option<u64> {
    let mut regs = Regs::default();
    let mut events = Vec::new();
    let mut pc = entry;
    for _ in 0..limit {
        let ins = img.read_u32(pc)?;
        if ins == 0xd65f_03c0 || ins & 0xffff_fbff == 0xd65f_0bff {
            return regs.get(0);
        }
        if let Step::Stop = step(img, pc, ins, &mut regs, &mut events, None) {
            return None;
        }
        if !events.is_empty() {
            return None;
        }
        pc = pc.wrapping_add(4);
    }
    None
}

/// Every address `ADRP`/`ADD`/`ADR` compute in straight-line code from `entry`.
pub(super) fn constants(img: &MachoImage, entry: u64, budget: usize) -> Vec<u64> {
    let mut regs = Regs::default();
    let mut events = Vec::new();
    let mut out = Vec::new();
    let mut pc = entry;
    for _ in 0..budget {
        let Some(ins) = img.read_u32(pc) else { break };
        let stop = matches!(step(img, pc, ins, &mut regs, &mut events, None), Step::Stop);
        if ins & 0x1f00_0000 == 0x1000_0000 || ins & 0xff80_0000 == 0x9100_0000 {
            out.extend(regs.get(rd(ins)));
        }
        if stop {
            break;
        }
        pc = pc.wrapping_add(4);
    }
    out
}
