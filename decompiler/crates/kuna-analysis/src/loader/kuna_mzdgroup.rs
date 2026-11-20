//! (kuna `mzdgroup`) The data segment an MS-DOS program's startup code loads
//! into DS.
//!
//! DOS starts a program with DS holding its PSP. The startup code of a compiled
//! program loads the segment its near data pointers address (the linker's
//! DGROUP) into DS before it calls into the program, and the program keeps it
//! there: a function that changes DS restores it. [`startup_ds`] reads that
//! segment off the entry code.
//!
//! The walk decodes forward from the entry `CS:IP` and tracks the 16-bit
//! registers that hold a segment of the module: an immediate whose word the
//! relocation table names (`mov dx, DGROUP`), CS at the entry, and a relocated
//! word read through `CS:` (`mov ds, cs:[6]`). Register moves, `mov sreg, reg`,
//! `mov reg, sreg` and a `push` the next instruction pops carry them. Any other
//! write drops a register (CS only changes by a far transfer, which ends the
//! walk, though a `CS:` override's p-code restates it), and so does an
//! interrupt for each register its service can return in: INT 21h by the
//! function in AH (DS too for the services that return a pointer in it, and
//! for an unknown function), any other interrupt every register.
//!
//! The walk decodes straight through. After an instruction that does not fall
//! through it resumes at the next forward branch target, and at a target a
//! register keeps its segment only when every path into the target brings the
//! same one. A call or DOS exit that a pending forward branch can skip ends its
//! path the same way, and DS there is a vote. DGROUP is the segment DS holds at
//! the first call or DOS exit no pending branch can skip, when every vote
//! agrees with it (MSC's DOS 1 error exit, which runs before DS is set, votes
//! for nothing); a walk that ends before such a call, at a return, an indirect
//! or far jump or [`MAX_INSNS`] instructions, takes the segment every vote
//! names (Lattice C's startup calls into the program with an error branch
//! still pending). An undecodable byte or a branch into an instruction states
//! none, and DGROUP must start inside the module.
//!
//! The segment at the first call is only DGROUP when the program keeps it, so
//! the walk reads on past that call through the rest of the startup, and
//! decodes each direct callee the startup enters with a known DS up to its
//! return. A callee that returns with another known segment in DS (an `init`
//! that loads the data segment after a banner printed with DS = CS), or a path
//! of the startup that ends with another known segment still in DS (a first
//! call made with a scratch segment, then `main` with the real one), states
//! nothing. A routine the startup calls with another known segment in DS and
//! then restores DGROUP (Borland's StartExit, which walks the `_INIT_` table
//! through DS) is one of the [`Startup::others`] that keep DS unknown.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::AddrSpace;
use kuna_decomp::architecture::Architecture;
use kuna_num::opcodes::OpCode;
use kuna_num::pcoderaw::VarnodeData;
use kuna_sleigh::translate::{PcodeEmit, Translate};

use crate::listing::classify::classify;
use crate::listing::model::RawOp;
use crate::loadimage_mz::MzImage;

/// The instructions the walk decodes before it gives up.
pub const MAX_INSNS: usize = 512;

const AX: usize = 0;
const CX: usize = 1;
const DX: usize = 2;
const BX: usize = 3;
const BP: usize = 5;
const SI: usize = 6;
const DI: usize = 7;
const ES: usize = 8;
const CS: usize = 9;
const DS: usize = 11;
const REGS: usize = 12;
/// AH has a write bit of its own, past the tracked registers, so a write to AL
/// keeps the DOS function number.
const AH: usize = 12;

/// The tracked registers: the 16-bit general registers in ModR/M order, then
/// the segment registers in `sreg` order, then AH.
const NAMES: [&[u8]; REGS + 1] =
    [b"AX", b"CX", b"DX", b"BX", b"SP", b"BP", b"SI", b"DI", b"ES", b"CS", b"SS", b"DS", b"AH"];

const fn bits(regs: &[usize]) -> u16 {
    let mut mask = 0u16;
    let mut i = 0;
    while i < regs.len() {
        mask |= 1 << regs[i];
        i += 1;
    }
    mask
}

const SERVICE_ANY: u16 = bits(&[AX, AH, BX, CX, DX, SI, DI, BP, ES]);
const INTERRUPT_ANY: u16 = SERVICE_ANY | bits(&[DS]);

/// One decoded instruction as the walk reads it.
pub struct Step {
    /// Byte length.
    pub len: u32,
    /// Bit `r` is set when the instruction writes register `r` or part of it.
    pub writes: u16,
    /// A `CALL`, direct or indirect.
    pub call: bool,
    /// Whether the next instruction can run after this one.
    pub falls_through: bool,
    /// The direct branch and call targets.
    pub targets: Vec<u64>,
}

/// The registers an `INT n` writes, with `ah` the function number when the
/// walk knows it, or `None` when the interrupt ends the program.
fn interrupt_writes(n: u8, ah: Option<u8>) -> Option<u16> {
    match (n, ah) {
        (0x20 | 0x27, _) | (0x21, Some(0x00 | 0x31 | 0x4c)) => None,
        (0x21, Some(0x1a | 0x25)) => Some(0),
        (0x21, Some(0x02 | 0x06 | 0x09)) => Some(bits(&[AX])),
        (0x21, Some(0x30)) => Some(bits(&[AX, AH, BX, CX])),
        (0x21, Some(0x2f | 0x35)) => Some(bits(&[BX, ES])),
        (0x21, Some(0x44)) => Some(bits(&[AX, AH, DX])),
        (0x21, Some(0x48 | 0x4a)) => Some(bits(&[AX, AH, BX])),
        (0x21, Some(0x51 | 0x62)) => Some(bits(&[BX])),
        (0x21, Some(0x1b | 0x1c | 0x1f | 0x32 | 0x4b | 0x5d | 0x63) | None) => Some(INTERRUPT_ANY),
        (0x21, _) => Some(SERVICE_ANY),
        _ => Some(INTERRUPT_ANY),
    }
}

/// The bytes of an instruction past its prefixes, and whether a `CS:` override
/// or an operand- or address-size prefix is among them.
fn strip_prefixes(bytes: &[u8]) -> (&[u8], bool, bool) {
    let mut cs = false;
    let mut wide = false;
    let mut at = 0;
    while let Some(&byte) = bytes.get(at) {
        match byte {
            0x2e => cs = true,
            0x66 | 0x67 => wide = true,
            0x26 | 0x36 | 0x3e | 0x64 | 0x65 | 0xf0 | 0xf2 | 0xf3 => {}
            _ => break,
        }
        at += 1;
    }
    (&bytes[at..], cs, wide)
}

/// What the walk knows at one point: each register's segment, and AH.
#[derive(Clone, Copy, PartialEq, Eq)]
struct State {
    vals: [Option<u16>; REGS],
    ah: Option<u8>,
}

impl State {
    /// A routine's state on entry: CS, DS when known, nothing else.
    fn entered(cs: u16, ds: Option<u16>) -> State {
        let mut state = State { vals: [None; REGS], ah: None };
        state.vals[CS] = Some(cs);
        state.vals[DS] = ds;
        state
    }

    /// The state after a call that returns, which keeps the segment registers
    /// other than ES.
    fn after_call(mut self) -> State {
        self.kill(SERVICE_ANY);
        self
    }

    fn kill(&mut self, mask: u16) {
        for (reg, val) in self.vals.iter_mut().enumerate() {
            if mask & (1 << reg) != 0 {
                *val = None;
            }
        }
        if mask & (1 << AH) != 0 {
            self.ah = None;
        }
    }

    /// The facts both `self` and `other` hold.
    fn meet(mut self, other: &State) -> State {
        for (val, theirs) in self.vals.iter_mut().zip(other.vals) {
            if *val != theirs {
                *val = None;
            }
        }
        if self.ah != other.ah {
            self.ah = None;
        }
        self
    }

    /// Record the value the instruction `op` gives a register, returning the
    /// register it assigned.
    fn assign(
        &mut self,
        op: &[u8],
        cs_override: bool,
        imm_at: u64,
        image: &MzImage,
        popped: Option<Option<u16>>,
    ) -> Option<usize> {
        let modrm = op.get(1).copied().unwrap_or(0);
        let direct = modrm >> 6 == 3;
        let reg = usize::from((modrm >> 3) & 7);
        let rm = usize::from(modrm & 7);
        let (dst, val) = match op {
            [opc @ 0xb8..=0xbf, lo, hi, ..] => {
                let dst = usize::from(opc - 0xb8);
                if dst == AX {
                    self.ah = Some(*hi);
                }
                let relocated = image.relocated_words.binary_search(&imm_at).is_ok();
                (dst, relocated.then(|| u16::from_le_bytes([*lo, *hi])))
            }
            [0xb4, imm, ..] => {
                self.ah = Some(*imm);
                return None;
            }
            [0x8e, ..] if matches!(reg, 0 | 2 | 3) && direct => (ES + reg, self.vals[rm]),
            [0x8e, rest @ ..] if matches!(reg, 0 | 2 | 3) && modrm & 0xc7 == 0x06 && cs_override => {
                let (Some(&lo), Some(&hi), Some(cs)) = (rest.get(1), rest.get(2), self.vals[CS]) else {
                    return None;
                };
                let word = u64::from(cs) * 16 + u64::from(u16::from_le_bytes([lo, hi]));
                let relocated = image.relocated_words.binary_search(&word).is_ok();
                (ES + reg, relocated.then(|| image_word(image, word)).flatten())
            }
            [0x8c, ..] if direct && reg < 4 => (rm, self.vals[ES + reg]),
            [0x8b, ..] if direct => (reg, self.vals[rm]),
            [0x89, ..] if direct => (rm, self.vals[reg]),
            [opc @ (0x58..=0x5f | 0x07 | 0x17 | 0x1f), ..] => {
                let dst = match opc {
                    0x58..=0x5f => usize::from(opc - 0x58),
                    _ => ES + usize::from(opc >> 3),
                };
                (dst, popped?)
            }
            _ => return None,
        };
        self.vals[dst] = val;
        Some(dst)
    }

    /// The value a `push` instruction `op` puts on the stack.
    fn pushed(&self, op: &[u8]) -> Option<Option<u16>> {
        let src = match op.first()? {
            opc @ 0x50..=0x57 => usize::from(opc - 0x50),
            opc @ (0x06 | 0x0e | 0x16 | 0x1e) => ES + usize::from(opc >> 3),
            _ => return None,
        };
        Some(self.vals[src])
    }
}

fn image_word(image: &MzImage, linear: u64) -> Option<u16> {
    let at = usize::try_from(linear.checked_sub(image.base)?).ok()?;
    let bytes = image.module.get(at..at.checked_add(2)?)?;
    Some(u16::from_le_bytes([bytes[0], bytes[1]]))
}

/// The meet of `state` (the fall-through's, when it reaches `at`) and every
/// state a pending forward branch carries to `at`, which leaves `pending`.
fn arrive(state: Option<State>, pending: &mut Vec<(u64, State)>, at: u64) -> Option<State> {
    let mut joined = state;
    pending.retain(|(target, carried)| {
        if *target != at {
            return true;
        }
        joined = Some(joined.map_or(*carried, |state| state.meet(carried)));
        false
    });
    joined
}

/// The instructions `keeps_ds` decodes in one callee before it gives up.
const CALLEE_INSNS: usize = 128;

/// How an instruction leaves the walk's path.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Flow {
    Next,
    /// A `CALL`, with its direct callee and the code segment that runs it.
    Call(Option<(u64, u16)>),
    Exit,
    Return,
    /// Any other instruction that does not fall through.
    Stop,
}

/// The direct callee of the call `op` decodes to, and its code segment.
fn callee(op: &[u8], step: &Step, cs: Option<u16>) -> Option<(u64, u16)> {
    let target = *step.targets.first()?;
    let cs = match op {
        [0x9a, _, _, lo, hi, ..] => u16::from_le_bytes([*lo, *hi]),
        [0xe8, ..] => cs?,
        _ => return None,
    };
    Some((target, cs))
}

/// One forward pass over a routine: the state reaching the next instruction
/// and the states the pending forward branches carry.
///
/// A backward branch is not followed; the registers assigned between its
/// target and itself are dropped, since another trip through that code can
/// assign them again from values the trip changed. A branch back to code
/// before the routine's start is not followed either.
struct Tracer<'a> {
    image: &'a MzImage,
    start: u64,
    vma: u64,
    reaching: Option<State>,
    pending: Vec<(u64, State)>,
    assigned: Vec<(u64, usize)>,
    popped: Option<Option<u16>>,
}

impl<'a> Tracer<'a> {
    fn new(image: &'a MzImage, start: u64, state: State) -> Self {
        let reaching = Some(state);
        Tracer { image, start, vma: start, reaching, pending: Vec::new(), assigned: Vec::new(), popped: None }
    }

    /// Decode the instruction at the cursor and apply it: the state after it,
    /// how it leaves the path, and the address after it. `None` when no path
    /// reaches the cursor, the bytes do not decode or a pending branch lands
    /// inside the instruction.
    fn step(&mut self, decode: &mut impl FnMut(u64) -> Option<Step>) -> Option<(State, Flow, u64)> {
        let vma = self.vma;
        if self.pending.iter().any(|&(target, _)| target == vma) {
            self.popped = None;
        }
        let mut state = arrive(self.reaching.take(), &mut self.pending, vma)?;
        let step = decode(vma)?;
        let end = vma + u64::from(step.len);
        if step.len == 0 || self.pending.iter().any(|&(target, _)| target > vma && target < end) {
            return None;
        }
        let image = self.image;
        let at = usize::try_from(vma.checked_sub(image.base)?).ok()?;
        let bytes = image.module.get(at..at.checked_add(step.len as usize)?)?;
        let (op, cs_override, wide) = strip_prefixes(bytes);
        let imm_at = vma + (bytes.len() - op.len()) as u64 + 1;
        let push = state.pushed(op);
        let flow = match op {
            [0xcd, n, ..] => match interrupt_writes(*n, state.ah) {
                Some(writes) => {
                    state.kill(writes);
                    Flow::Next
                }
                None => Flow::Exit,
            },
            [0xcc | 0xce, ..] => {
                state.kill(INTERRUPT_ANY);
                Flow::Next
            }
            _ if step.call => Flow::Call(callee(op, &step, state.vals[CS])),
            _ => {
                state.kill(step.writes & !(1 << CS));
                if !wide {
                    if let Some(reg) = state.assign(op, cs_override, imm_at, image, self.popped) {
                        self.assigned.push((vma, reg));
                    }
                }
                for &target in step.targets.iter().filter(|&&target| target <= vma && target >= self.start) {
                    for &(_, reg) in self.assigned.iter().filter(|&&(site, _)| site >= target) {
                        state.vals[reg] = None;
                    }
                }
                self.pending.extend(step.targets.iter().filter(|&&target| target > vma).map(|&target| (target, state)));
                match op.first() {
                    _ if step.falls_through => Flow::Next,
                    Some(0xc2 | 0xc3 | 0xca | 0xcb) => Flow::Return,
                    _ => Flow::Stop,
                }
            }
        };
        self.popped = push;
        Some((state, flow, end))
    }

    fn fall(&mut self, state: State, end: u64) {
        self.reaching = Some(state);
        self.vma = end;
    }

    /// Leave the path for the nearest pending branch target; false when none
    /// is left.
    fn leave(&mut self) -> bool {
        self.reaching = None;
        match self.pending.iter().map(|&(target, _)| target).min() {
            Some(next) => {
                self.vma = next;
                true
            }
            None => false,
        }
    }

    fn skippable(&self) -> bool {
        !self.pending.is_empty()
    }
}

/// Whether the routine at `at`, entered with code segment `cs` and data
/// segment `ds`, keeps `ds`: false when the walk sees it return with another
/// known segment in DS. A direct call it makes with another known segment in
/// DS adds its callee to `others`.
fn keeps_ds(
    image: &MzImage,
    at: u64,
    cs: u16,
    ds: u16,
    others: &mut Vec<u64>,
    decode: &mut impl FnMut(u64) -> Option<Step>,
) -> bool {
    if at < image.base || at >= image.base + image.module.len() as u64 {
        return true;
    }
    let mut tracer = Tracer::new(image, at, State::entered(cs, Some(ds)));
    for _ in 0..CALLEE_INSNS {
        let Some((state, flow, end)) = tracer.step(decode) else { return true };
        let other = state.vals[DS].is_some_and(|held| held != ds);
        match flow {
            Flow::Next => {
                tracer.fall(state, end);
                continue;
            }
            Flow::Call(target) => {
                if other {
                    others.extend(target.map(|(callee, _)| callee));
                }
                tracer.fall(state.after_call(), end);
                continue;
            }
            Flow::Return if other => return false,
            Flow::Exit | Flow::Return | Flow::Stop => {}
        }
        if !tracer.leave() {
            return true;
        }
    }
    true
}

/// The data segment the startup code loads into DS, and the routines it calls
/// with another known segment in DS.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Startup {
    /// The segment value, DGROUP.
    pub ds: u16,
    /// The linear entry addresses of the routines entered with another DS,
    /// sorted.
    pub others: Vec<u64>,
}

/// The segment DS holds when the startup code that starts at `image.entry`
/// with code segment `entry_cs` makes the first call every path reaches,
/// decoding each instruction with `decode`.
///
/// Past that call the walk reads on through the rest of the startup, taking
/// each call to return with the segment registers other than ES kept. A
/// direct callee the startup enters with another known segment in DS is one
/// of [`Startup::others`]; a path of the startup that ends with another known
/// segment still in DS states nothing.
/// Each direct callee entered with a known DS is decoded too (`keeps_ds`), and
/// one seen to return with another segment in DS states nothing either.
pub fn walk(image: &MzImage, entry_cs: u16, mut decode: impl FnMut(u64) -> Option<Step>) -> Option<Startup> {
    let mut tracer = Tracer::new(image, image.entry, State::entered(entry_cs, None));
    let mut votes = Votes::default();
    let mut others: Vec<u64> = Vec::new();
    let mut dgroup: Option<u16> = None;
    let finish = |ds: Option<u16>, mut others: Vec<u64>| {
        others.sort_unstable();
        others.dedup();
        Some(Startup { ds: ds?, others })
    };
    for _ in 0..MAX_INSNS {
        let Some((state, flow, end)) = tracer.step(&mut decode) else { return finish(dgroup, others) };
        let ds = state.vals[DS];
        let other = dgroup.is_some_and(|dgroup| ds.is_some_and(|ds| ds != dgroup));
        match flow {
            Flow::Next => {
                tracer.fall(state, end);
                continue;
            }
            Flow::Call(target) => {
                match dgroup {
                    Some(_) if other => others.extend(target.map(|(at, _)| at)),
                    Some(_) => {}
                    None if tracer.skippable() => votes.add(ds),
                    None => dgroup = Some(votes.verdict(ds)?),
                }
                if let (Some(ds), Some((at, cs))) = (ds, target) {
                    if !keeps_ds(image, at, cs, ds, &mut others, &mut decode) {
                        return None;
                    }
                }
                if dgroup.is_some() {
                    tracer.fall(state.after_call(), end);
                    continue;
                }
            }
            Flow::Exit if dgroup.is_none() && !tracer.skippable() => return finish(votes.verdict(ds), others),
            Flow::Exit if dgroup.is_none() => votes.add(ds),
            _ if other => return None,
            Flow::Exit | Flow::Return | Flow::Stop => {}
        }
        if !tracer.leave() {
            return finish(dgroup.or_else(|| votes.unanimous()), others);
        }
    }
    finish(dgroup.or_else(|| votes.unanimous()), others)
}

/// The segments DS holds at the calls and exits the walk leaves because a
/// pending forward branch can skip them.
#[derive(Default)]
struct Votes {
    seen: Option<u16>,
    split: bool,
}

impl Votes {
    fn add(&mut self, ds: Option<u16>) {
        let Some(ds) = ds else { return };
        match self.seen {
            None => self.seen = Some(ds),
            Some(seen) if seen != ds => self.split = true,
            Some(_) => {}
        }
    }

    /// The segment every vote names, when they agree.
    fn unanimous(&self) -> Option<u16> {
        self.seen.filter(|_| !self.split)
    }

    /// `ds`, the segment at a call or exit every path reaches, when every vote
    /// agrees with it.
    fn verdict(&self, ds: Option<u16>) -> Option<u16> {
        let ds = ds?;
        (!self.split && self.seen.is_none_or(|seen| seen == ds)).then_some(ds)
    }
}

#[derive(Default)]
struct Capture {
    ops: Vec<(OpCode, Option<VarnodeData>, Option<VarnodeData>)>,
}

impl PcodeEmit for Capture {
    fn dump(&mut self, _addr: &Address, opc: OpCode, outvar: Option<&VarnodeData>, vars: &[VarnodeData]) {
        self.ops.push((opc, outvar.cloned(), vars.first().cloned()));
    }
}

fn overlaps(a: &VarnodeData, b: &VarnodeData) -> bool {
    let same_space = match (&a.space, &b.space) {
        (Some(x), Some(y)) => Rc::ptr_eq(x, y),
        _ => false,
    };
    same_space && a.offset < b.offset + u64::from(b.size) && b.offset < a.offset + u64::from(a.size)
}

fn sleigh_step(translate: &dyn Translate, code_space: &Rc<AddrSpace>, regs: &[VarnodeData], vma: u64) -> Option<Step> {
    let addr = Address::new(Rc::clone(code_space), vma);
    let mut cap = Capture::default();
    let decoded = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| translate.one_instruction(&mut cap, &addr)));
    let len = match decoded {
        Ok(Ok(len)) if len > 0 => len as u32,
        _ => return None,
    };
    let raw: Vec<RawOp> = cap.ops.iter().map(|(opcode, _, in0)| RawOp { opcode: *opcode, in0: in0.clone() }).collect();
    let classified = classify(&raw, vma, len);
    let mut writes = 0u16;
    for out in cap.ops.iter().filter_map(|(_, out, _)| out.as_ref()) {
        for (reg, loc) in regs.iter().enumerate() {
            if overlaps(out, loc) {
                writes |= 1 << reg;
            }
        }
    }
    let flow = &classified.flow;
    Some(Step {
        len,
        writes,
        call: flow.is_call,
        falls_through: classified.fall_through.is_some() && !flow.is_computed,
        targets: classified.flows,
    })
}

/// The data segment an MS-DOS program's startup code loads into DS, and the
/// routines it calls with another, when the walk above can tell and DGROUP
/// starts inside the load module.
pub fn startup_ds(
    arch: &Architecture,
    translate: &dyn Translate,
    code_space: &Rc<AddrSpace>,
    image: &MzImage,
) -> Option<Startup> {
    let regs: Vec<VarnodeData> =
        NAMES.iter().map(|name| arch.get_register_varnode(name).ok()).collect::<Option<_>>()?;
    let entry_cs = image.code_segments.iter().find(|&&(linear, _)| linear == image.entry).map(|&(_, cs)| cs)?;
    let startup = crate::loader::kuna_realmodecs::without_commits(arch, translate, || {
        walk(image, entry_cs, |vma| sleigh_step(translate, code_space, &regs, vma))
    })?;
    let start = u64::from(startup.ds) * 16;
    (start >= image.base && start < image.base + image.module.len() as u64).then_some(startup)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A decoder for the handful of 8086 forms the tests use: register writes
    /// by opcode, `CALL`, `RET`, `JMP`/`Jcc` rel8 and `INT`.
    fn step(image: &MzImage, vma: u64) -> Option<Step> {
        let b = image.module.get((vma - image.base) as usize..).filter(|b| b.len() >= 3)?;
        let gpr = |r: u8| if r == 0 { 1 | 1 << AH } else { 1u16 << r };
        let rel8 = |len: u64| (vma + len).wrapping_add(i64::from(b[1] as i8) as u64);
        let rel16 = || (vma + 3).wrapping_add(i64::from(i16::from_le_bytes([b[1], b[2]])) as u64);
        let (len, writes, call, falls_through, targets) = match b[0] {
            0xb8..=0xbf => (3, gpr(b[0] - 0xb8), false, true, vec![]),
            0xb4 => (2, gpr(0), false, true, vec![]),
            0xb0 => (2, 1, false, true, vec![]),
            0x8e => (2, 1 << (8 + ((b[1] >> 3) & 7)), false, true, vec![]),
            0x8b | 0x8c => (
                2,
                gpr((b[1] >> 3) & 7) * u16::from(b[0] == 0x8b) | gpr(b[1] & 7) * u16::from(b[0] == 0x8c),
                false,
                true,
                vec![],
            ),
            0x50..=0x57 | 0x06 | 0x0e | 0x16 | 0x1e => (1, 1 << 4, false, true, vec![]),
            0x58..=0x5f => (1, 1 << 4 | gpr(b[0] - 0x58), false, true, vec![]),
            0x07 | 0x17 | 0x1f => (1, 1 << 4 | 1 << (8 + (b[0] >> 3)), false, true, vec![]),
            0xcd => (2, 1 << 4, true, true, vec![]),
            0xe8 => (3, 1 << 4, true, true, vec![rel16()]),
            0xc3 => (1, 1 << 4, false, false, vec![]),
            0xeb => (2, 0, false, false, vec![rel8(2)]),
            0x70..=0x7f => (2, 0, false, true, vec![rel8(2)]),
            0x90 => (1, 0, false, true, vec![]),
            _ => return None,
        };
        Some(Step { len, writes, call, falls_through, targets })
    }

    fn image(code: &[u8], relocated: &[usize]) -> MzImage {
        let mut module = vec![0x90u8; 0x200];
        module[..code.len()].copy_from_slice(code);
        MzImage {
            module,
            base: 0x10000,
            entry: 0x10000,
            stack: 0,
            relocations: relocated.len(),
            relocated_words: relocated.iter().map(|&at| 0x10000 + at as u64).collect(),
            skipped_relocations: 0,
            packer: None,
            code_segments: vec![(0x10000, 0x1000)],
        }
    }

    fn startup(code: &[u8], relocated: &[usize]) -> Option<Startup> {
        let image = image(code, relocated);
        walk(&image, 0x1000, |vma| step(&image, vma))
    }

    fn ds(code: &[u8], relocated: &[usize]) -> Option<u16> {
        startup(code, relocated).map(|startup| startup.ds)
    }

    #[test]
    fn a_relocated_segment_moved_into_ds_before_the_first_call_is_dgroup() {
        let code = [0xb8, 0x10, 0x10, 0x8e, 0xd8, 0xe8, 0x00, 0x00];
        assert_eq!(ds(&code, &[1]), Some(0x1010));
        assert_eq!(ds(&code, &[]), None, "an unrelocated immediate is no segment of the module");
    }

    #[test]
    fn dos_version_keeps_dx_and_the_date_overwrites_it() {
        let turbo_c = [0xba, 0x10, 0x10, 0xb4, 0x30, 0xcd, 0x21, 0x8e, 0xda, 0xe8, 0x00, 0x00];
        assert_eq!(ds(&turbo_c, &[1]), Some(0x1010));
        let mut date = turbo_c;
        date[4] = 0x2a;
        assert_eq!(ds(&date, &[1]), None);
        let switch_char = [0x0e, 0x1f, 0xb4, 0x37, 0xb0, 0x00, 0xcd, 0x21, 0xe8, 0x00, 0x00];
        assert_eq!(ds(&switch_char, &[]), Some(0x1000), "a write to AL keeps the function number in AH");
    }

    #[test]
    fn a_segment_set_on_one_path_only_is_dropped_at_the_join() {
        let code = [0x73, 0x02, 0x0e, 0x1f, 0xe8, 0x00, 0x00];
        assert_eq!(ds(&code, &[]), None);
        let both = [0x0e, 0x1f, 0x73, 0x01, 0x90, 0xe8, 0x00, 0x00];
        assert_eq!(ds(&both, &[]), Some(0x1000), "push cs; pop ds before the branch holds on both paths");
    }

    #[test]
    fn a_dead_fall_through_resumes_at_the_branch_target() {
        let code = [0xb8, 0x10, 0x10, 0x73, 0x03, 0xb4, 0x4c, 0xc3, 0x50, 0x1f, 0xe8, 0x00, 0x00];
        assert_eq!(ds(&code, &[1]), Some(0x1010), "the branch carries AX past the fall-through's write");
        let exit = [0xb8, 0x10, 0x10, 0x8e, 0xd8, 0xb4, 0x4c, 0xcd, 0x21];
        assert_eq!(ds(&exit, &[1]), Some(0x1010), "a DOS exit with no branch target ahead decides");
        assert_eq!(ds(&[0x0e, 0x1f, 0xc3], &[]), None, "a return decides nothing");
    }

    #[test]
    fn a_call_a_branch_can_skip_is_a_side_path() {
        let msc = [0x73, 0x04, 0x50, 0xe8, 0x00, 0x00, 0xb8, 0x10, 0x10, 0x8e, 0xd8, 0xe8, 0x00, 0x00];
        assert_eq!(ds(&msc, &[7]), Some(0x1010), "the error call before DS is set decides nothing");
        let psp = [0x73, 0x05, 0x0e, 0x1f, 0xe8, 0x00, 0x00, 0xe8, 0x00, 0x00];
        assert_eq!(ds(&psp, &[]), None, "a DS the skipped path loads is not the joined path's");
    }

    #[test]
    fn a_call_with_an_error_branch_pending_votes() {
        let lattice = [0xba, 0x10, 0x10, 0x72, 0x08, 0x8e, 0xda, 0xe8, 0x00, 0x00, 0x90, 0x90, 0x90, 0xeb, 0xf1];
        assert_eq!(ds(&lattice, &[1]), Some(0x1010), "the startup's only call names DGROUP");
        let split = [0x0e, 0x1f, 0x73, 0x03, 0xe8, 0x00, 0x00, 0xb8, 0x10, 0x10, 0x8e, 0xd8, 0xe8, 0x00, 0x00];
        assert_eq!(ds(&split, &[8]), None, "a side path's DS that the joined path replaces is a conflict");
    }

    #[test]
    fn a_callee_that_returns_with_another_segment_in_ds_states_nothing() {
        let mut code = [0x90u8; 0x22];
        code[..0x0c].copy_from_slice(&[0x0e, 0x1f, 0xe8, 0x0b, 0x00, 0xe8, 0x18, 0x00, 0xb4, 0x4c, 0xcd, 0x21]);
        code[0x10..0x16].copy_from_slice(&[0xb8, 0x10, 0x10, 0x8e, 0xd8, 0xc3]);
        code[0x20] = 0xc3;
        assert_eq!(ds(&code, &[0x11]), None, "the banner's DS = CS is not the segment init leaves in DS");
        code[0x10..0x18].copy_from_slice(&[0x1e, 0xb8, 0x10, 0x10, 0x8e, 0xd8, 0x1f, 0xc3]);
        assert_eq!(ds(&code, &[0x12]), Some(0x1000), "a callee that restores DS keeps it");
    }

    #[test]
    fn a_later_call_with_another_segment_in_ds_states_nothing() {
        let mut code = [0x90u8; 0x31];
        code[..0x14].copy_from_slice(&[
            0xb8, 0x20, 0x10, 0x8e, 0xd8, 0xe8, 0x18, 0x00, 0xb8, 0x10, 0x10, 0x8e, 0xd8, 0xe8, 0x20, 0x00, 0xb4, 0x4c,
            0xcd, 0x21,
        ]);
        code[0x20] = 0xc3;
        code[0x30] = 0xc3;
        assert_eq!(ds(&code, &[1, 9]), None, "the BSS segment the first call runs with is not main's");
        code[9] = 0x20;
        assert_eq!(ds(&code, &[1, 9]), Some(0x1020));
    }

    #[test]
    fn a_callee_the_startup_enters_with_another_ds_is_left_out() {
        let mut code = [0x90u8; 0x41];
        code[..0x19].copy_from_slice(&[
            0xb8, 0x10, 0x10, 0x8e, 0xd8, 0xe8, 0x18, 0x00, 0x1e, 0xb8, 0x0e, 0x10, 0x8e, 0xd8, 0xe8, 0x1f, 0x00, 0x1f,
            0xe8, 0x2b, 0x00, 0xb4, 0x4c, 0xcd, 0x21,
        ]);
        code[0x20] = 0xc3;
        code[0x30] = 0xc3;
        code[0x40] = 0xc3;
        let borland = startup(&code, &[1, 0x0a]);
        assert_eq!(borland, Some(Startup { ds: 0x1010, others: vec![0x10030] }), "StartExit runs with _INIT_ in DS");

        let mut msc = [0x90u8; 0x31];
        msc[..0x12].copy_from_slice(&[
            0x73, 0x04, 0x50, 0xe8, 0x1a, 0x00, 0xb8, 0x10, 0x10, 0x8e, 0xd8, 0xe8, 0x22, 0x00, 0xb4, 0x4c, 0xcd, 0x21,
        ]);
        msc[0x20] = 0xc3;
        msc[0x30] = 0xc3;
        let entered_with_psp = startup(&msc, &[7]);
        assert_eq!(
            entered_with_psp,
            Some(Startup { ds: 0x1010, others: vec![] }),
            "the DOS 1 error call's runtime routine is one the program calls with DGROUP too"
        );
    }

    #[test]
    fn an_interrupt_other_than_dos_drops_ds() {
        let code = [0x0e, 0x1f, 0xcd, 0x2f, 0xe8, 0x00, 0x00];
        assert_eq!(ds(&code, &[]), None);
    }

    #[test]
    fn interrupt_writes_follow_the_dos_function() {
        assert_eq!(interrupt_writes(0x21, Some(0x4c)), None);
        assert_eq!(interrupt_writes(0x20, None), None);
        assert_eq!(interrupt_writes(0x21, Some(0x30)), Some(bits(&[AX, AH, BX, CX])));
        assert_eq!(interrupt_writes(0x21, Some(0x3d)), Some(SERVICE_ANY));
        assert_eq!(interrupt_writes(0x21, Some(0x1f)), Some(INTERRUPT_ANY), "the drive parameter block is DS:BX");
        assert_eq!(interrupt_writes(0x21, None), Some(INTERRUPT_ANY));
        assert_eq!(interrupt_writes(0x10, Some(0x30)), Some(INTERRUPT_ANY));
    }
}
