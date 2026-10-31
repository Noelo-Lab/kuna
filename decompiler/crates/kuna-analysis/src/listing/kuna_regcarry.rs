//! (kuna) `regcarry` — carry a register's constant to the instructions that
//! fall after it, so an address a fixed-width ISA builds in two instructions is
//! a reference.
//!
//! # The gap
//!
//! [`super::xrefs`] reads a reference out of one instruction's p-code. That is
//! the whole answer where one instruction can hold a whole address — x86-64's
//! `lea rdi,[rip+0x36a]` lifts to `RDI = COPY 0x13c9:8` — and no answer where
//! it cannot:
//!
//! ```text
//! 0x100000418  adrp x0, 0x100000000    ; x0 = the page
//! 0x10000041c  add  x0, x0, #0x488     ; x0 = 0x100000488, "AUDIT_STATIC_LABEL"
//!
//! 0x111a2      lui  a1, 0x12           ; a1 = 0x12000
//! 0x111a6      lw   a1, 0x1d0(a1)      ; reads 0x121d0, audit_global
//! ```
//!
//! Neither instruction names the datum: the first holds a page (or the high
//! twenty bits), the second a register plus a twelve-bit offset. AArch64
//! `adrp`, RISC-V `lui`/`auipc`, MIPS `lui`, PowerPC `lis` and ARM `add rX,pc`
//! all spell an address that way, so every string, global and function pointer
//! a program formed with them was referenced by nothing.
//!
//! # The rule
//!
//! A register an instruction leaves holding a constant that could be an address
//! or its high part — the `checkOperands` floor: at least 4096 and not a byte
//! mask — is carried to the instruction's fall-through, and each instruction
//! that receives it evaluates its own p-code with the carried values seeded. A
//! carried value lives for at most [`MAX_HOPS`] instructions, is dropped when its
//! register is written with anything the evaluation cannot follow, and survives
//! a call only in a register the calling convention preserves and the call
//! instruction itself did not write (a link register). Three shapes are read, as
//! [`super::kuna_picbase`] reads them:
//!
//!  * the address a `LOAD` reads, a [`XrefKind::Read`];
//!  * the address a `STORE` writes, a [`XrefKind::Write`];
//!  * an address an `INT_ADD`/`INT_SUB` composes into a whole register —
//!    `add x0,x0,#0x488` — a [`XrefKind::Data`]: forming an address is not
//!    reading it.
//!
//! Only a value a carried register contributed to is reported; a single
//! instruction's own constants are [`super::xrefs`]' to report. The `Data`
//! shape is narrower than the other two because nothing dereferences the value
//! to vouch for it, and the same pair builds integers:
//!
//!  * a copy of a carried value (`mv a1,a0`, which RISC-V spells `addi a1,a0,0`)
//!    is the reference its source already was;
//!  * bitwise composition (`movk`, `ori`, `movt`) and an add narrower than the
//!    register it lands in (RV64's `addiw`, a `w` register) are how a program
//!    builds a number;
//!  * a value only formed to form another — the middle `add` of an A32 PLT entry,
//!    whose register the very next instruction consumes as a base and overwrites
//!    — is a step, not an address;
//!  * an access that writes its own address back (`ldr pc,[ip,#0x2f0]!`, `lwzu`)
//!    is one reference, the access.
//!
//! An ELF `ET_DYN` image is relocated as a whole, so an absolute immediate in it
//! is a number, never an address: `lui s2,0x3; addi s2,s2,-5` in a RISC-V shared
//! object is 12283. There a constant starts a carry only when it was computed
//! from the instruction's own address (`auipc`'s `inst_start + imm`, A32's
//! `pc` read as `inst_start + 8`). x86-64 and AArch64 are exempt: SLEIGH folds
//! their PC-relative operands (`lea rip`, `adrp`) into a plain constant, so the
//! PC is not visible, and neither ISA builds an integer from an add of two
//! immediates.
//!
//! # Where a carried value holds
//!
//! Only along a run that nothing else enters. The walk is breadth-first and
//! decodes an instruction before it has seen every edge into it, so a value is
//! carried on the strength of the edge it arrived by, and each reference it
//! produces records the instructions it travelled through. Once the whole walk
//! has drained, a reference is filed only if none of those instructions is a
//! branch target, a switch case or a function entry — a join, where another
//! path may hand the register a different value. Nothing is carried along a
//! branch edge, and an instruction whose own p-code branches (ARM predication,
//! x86 `rep`) passes on no register it writes.

use std::collections::{BTreeSet, HashMap, HashSet};
use std::rc::Rc;

use kuna_base::space::{spacetype, AddrSpace};
use kuna_decomp::kuna_fasthash::FxBuildHasher;
use kuna_num::opcodes::OpCode;
use kuna_num::pcoderaw::VarnodeData;
use smallvec::SmallVec;

use super::kuna_poolref::PoolImage;
use super::xrefs::{in_range, looks_like_address, FullOp, XrefKind};

/// How many instructions a carried value may reach past the one that formed
/// it. Compilers schedule the two halves of a pair a few instructions apart
/// (and across a call when the register is callee-saved); every hop is one
/// more instruction over which the register is believed.
const MAX_HOPS: usize = 8;

/// How many registers one instruction may carry on.
const MAX_TRACKED: usize = 8;

/// How far past an instruction's own address a constant may lie and still be
/// its materialized PC: A32 reads `pc` as `inst_start + 8`, Thumb as `+ 4`.
const PC_WINDOW: u64 = 16;

/// One register the walk carries.
#[derive(Debug, Clone, Copy)]
struct Held {
    off: u64,
    size: u32,
    v: u64,
    /// How many instructions it has reached since it was formed.
    age: u8,
    pc: bool,
    /// The [`Found`] `Data` row that formed it, if one did.
    formed: Option<usize>,
}

/// What one instruction hands its fall-through: the registers, and the
/// instructions the oldest of them has reached so far, most recent last.
#[derive(Debug, Clone, Default)]
struct Carry {
    held: SmallVec<[Held; MAX_TRACKED]>,
    trail: Trail,
}

/// The instructions a carried value has reached.
type Trail = SmallVec<[u64; MAX_HOPS]>;

/// The fixed facts the evaluation reads.
pub(super) struct Machine<'a> {
    reg: Rc<AddrSpace>,
    data_index: Option<u64>,
    big_endian: bool,
    /// An `ET_DYN` image on an ISA whose PC-relative operands keep the PC
    /// visible: only a PC-derived constant may start a carry.
    pc_only: bool,
    preserved: &'a [(u64, u64)],
    mapped: &'a [(u64, u64)],
    pool: Option<&'a PoolImage<'a>>,
}

impl<'a> Machine<'a> {
    pub(super) fn new(
        file: &object::File,
        reg: Option<&Rc<AddrSpace>>,
        data_space: Option<&Rc<AddrSpace>>,
        preserved: &'a [(u64, u64)],
        mapped: &'a [(u64, u64)],
        pool: Option<&'a PoolImage<'a>>,
    ) -> Option<Self> {
        use object::Object;
        let reg = Rc::clone(reg?);
        if mapped.is_empty() {
            return None;
        }
        let pc_only = file.format() == object::BinaryFormat::Elf
            && file.kind() == object::ObjectKind::Dynamic
            && !matches!(file.architecture(), object::Architecture::X86_64 | object::Architecture::Aarch64);
        Some(Machine {
            big_endian: reg.is_big_endian(),
            reg,
            data_index: data_space.and_then(|d| u64::try_from(d.get_index()).ok()),
            pc_only,
            preserved,
            mapped,
            pool,
        })
    }

    fn is_register(&self, vn: &VarnodeData) -> bool {
        vn.space.as_ref().is_some_and(|s| Rc::ptr_eq(s, &self.reg))
    }

    fn admits(&self, v: u64) -> bool {
        looks_like_address(v) && in_range(self.mapped, v)
    }
}

/// The walk-wide record: what is in flight, every branch target, and the
/// references waiting for [`Tracker::finish`].
#[derive(Default)]
pub(super) struct Tracker {
    pending: HashMap<u64, Carry, FxBuildHasher>,
    targets: HashSet<u64, FxBuildHasher>,
    found: Vec<Found>,
    /// Reused across evaluations, so that none allocates.
    scratch: Frame,
    hits: Vec<Hit>,
}

/// A reference that holds if none of `trail` is a join.
struct Found {
    from: u64,
    to: u64,
    kind: XrefKind,
    trail: Trail,
    /// The same instruction accesses `to`: this is its writeback.
    writeback: bool,
    /// The trails of later references that only used this one as a step.
    stepped: Vec<Trail>,
}

/// One instruction, as [`Tracker::step`] reads it.
pub(super) struct Insn<'a> {
    pub vma: u64,
    pub ops: &'a [FullOp],
    pub fall: Option<u64>,
    /// The static branch targets; none for a call.
    pub branches: &'a [u64],
}

/// A value inside one instruction's evaluation.
#[derive(Debug, Clone, Copy)]
struct Val {
    v: u64,
    /// A carried register contributed to it.
    carried: bool,
    /// The width of the `INT_ADD`/`INT_SUB` of this instruction that combined a
    /// carried value with something else, if one did and nothing since changed
    /// its width.
    composed: Option<u32>,
    /// How many instructions the oldest carried input has reached.
    age: u8,
    /// Computed from the instruction's own address.
    pc: bool,
    /// The [`Found`] `Data` row that formed exactly this value.
    formed: Option<usize>,
    /// The formed value this one was computed from.
    base: Option<usize>,
}

fn mask(size: u32) -> u64 {
    if size == 0 || size >= 8 {
        u64::MAX
    } else {
        (1u64 << (8 * size)) - 1
    }
}

fn sign_extend(v: u64, size: u32) -> u64 {
    if size == 0 || size >= 8 {
        return v;
    }
    let sign = 1u64 << (8 * size - 1);
    let v = v & mask(size);
    if v & sign != 0 { v | !mask(size) } else { v }
}

fn is_constant(vn: &VarnodeData) -> bool {
    vn.space.as_ref().is_some_and(|s| s.get_type() == spacetype::IPTR_CONSTANT)
}

fn overlaps(a: (u64, u32), b: (u64, u32)) -> bool {
    a.0 < b.0.saturating_add(u64::from(b.1)) && b.0 < a.0.saturating_add(u64::from(a.1))
}

/// The ops [`fold`] models: the only ones that can produce a carried value.
fn folds(code: OpCode) -> bool {
    matches!(
        code,
        OpCode::CPUI_COPY
            | OpCode::CPUI_INT_ZEXT
            | OpCode::CPUI_INT_SEXT
            | OpCode::CPUI_CAST
            | OpCode::CPUI_SUBPIECE
            | OpCode::CPUI_INT_ADD
            | OpCode::CPUI_INT_SUB
            | OpCode::CPUI_INT_OR
            | OpCode::CPUI_INT_AND
            | OpCode::CPUI_INT_XOR
            | OpCode::CPUI_INT_LEFT
            | OpCode::CPUI_INT_RIGHT
            | OpCode::CPUI_PIECE
    )
}

/// Could this instruction, with nothing carried into it, leave a register
/// holding a constant worth carrying? The cheap gate that keeps almost every
/// instruction out of the evaluation: some op must compute an address-like
/// value from constants alone (`adrp`'s page, RISC-V `lui`'s `0x15 << 0xc`) or
/// take a positive address-like constant (`auipc`'s `inst_start`) — not the
/// 12-bit immediate RISC-V copies into a temporary ahead of almost every
/// instruction — and it must not be a call, which nothing it forms survives.
fn has_seed(ops: &[FullOp]) -> bool {
    let konst = |vn: &VarnodeData| {
        is_constant(vn).then(|| Val {
            v: vn.offset & mask(vn.size),
            carried: false,
            composed: None,
            age: 0,
            pc: false,
            formed: None,
            base: None,
        })
    };
    let big = |vn: &VarnodeData| {
        let v = vn.offset & mask(vn.size);
        looks_like_address(v) && is_constant(vn) && sign_extend(v, vn.size) as i64 > 0
    };
    let mut seed = false;
    for op in ops {
        match op.opcode {
            OpCode::CPUI_CALL | OpCode::CPUI_CALLIND => return false,
            code if !seed && folds(code) && op.out.is_some() => {
                seed = op.ins.iter().any(big)
                    || op.ins.iter().all(is_constant)
                        && fold(op, op.ins.first().and_then(konst), op.ins.get(1).and_then(konst))
                            .is_some_and(|v| looks_like_address(v.v));
            }
            _ => {}
        }
    }
    seed
}

/// Does `op` branch inside its own instruction — a p-code-relative target, or
/// the instruction's own fall-through? Then what the instruction writes is
/// conditional.
fn branches_within(op: &FullOp, fall: u64) -> bool {
    matches!(op.opcode, OpCode::CPUI_BRANCH | OpCode::CPUI_CBRANCH)
        && op.ins.first().is_some_and(|vn| is_constant(vn) || vn.offset == fall)
}

/// One instruction's registers and temporaries while it is evaluated.
#[derive(Default)]
struct Frame {
    vma: u64,
    regs: Vec<(u64, u32, Val)>,
    temps: Vec<(u64, u32, Val)>,
    written: Vec<(u64, u32)>,
}

impl Frame {
    fn read(&self, m: &Machine, vn: &VarnodeData) -> Option<Val> {
        let space = vn.space.as_ref()?;
        match space.get_type() {
            spacetype::IPTR_CONSTANT => {
                let v = vn.offset & mask(vn.size);
                Some(Val {
                    v,
                    carried: false,
                    composed: None,
                    age: 0,
                    pc: v.wrapping_sub(self.vma) < PC_WINDOW,
                    formed: None,
                    base: None,
                })
            }
            spacetype::IPTR_INTERNAL => self
                .temps
                .iter()
                .find(|&&(o, s, _)| o == vn.offset && s == vn.size)
                .map(|&(_, _, v)| v),
            _ if m.is_register(vn) => {
                let (lo, hi) = (vn.offset, vn.offset.saturating_add(u64::from(vn.size)));
                self.regs.iter().find_map(|&(o, s, val)| {
                    let end = o.saturating_add(u64::from(s));
                    if lo < o || hi > end {
                        return None;
                    }
                    if (lo, hi) == (o, end) {
                        return Some(val);
                    }
                    let shift = if m.big_endian { end - hi } else { lo - o };
                    let v = val.v.checked_shr((8 * shift) as u32).unwrap_or(0) & mask(vn.size);
                    Some(Val { v, composed: None, ..val })
                })
            }
            _ => None,
        }
    }

    fn write(&mut self, m: &Machine, vn: &VarnodeData, val: Option<Val>) {
        let Some(space) = vn.space.as_ref() else { return };
        let val = val.map(|v| Val { v: v.v & mask(vn.size), ..v });
        let span = (vn.offset, vn.size);
        if space.get_type() == spacetype::IPTR_INTERNAL {
            self.temps.retain(|&(o, s, _)| !overlaps((o, s), span));
            if let Some(v) = val {
                self.temps.push((vn.offset, vn.size, v));
            }
        } else if m.is_register(vn) {
            self.regs.retain(|&(o, s, _)| !overlaps((o, s), span));
            self.written.push(span);
            if let Some(v) = val {
                self.regs.push((vn.offset, vn.size, v));
            }
        }
    }
}

/// The value `op` produces from `a` (and `b`), where the evaluation models it.
fn fold(op: &FullOp, a: Option<Val>, b: Option<Val>) -> Option<Val> {
    let out = op.out.as_ref()?;
    let a = a?;
    let resized = out.size != op.ins.first()?.size;
    let unary = |v: u64| {
        Some(Val { v: v & mask(out.size), composed: if resized { None } else { a.composed }, ..a })
    };
    match op.opcode {
        OpCode::CPUI_COPY | OpCode::CPUI_INT_ZEXT | OpCode::CPUI_CAST => unary(a.v),
        OpCode::CPUI_INT_SEXT => unary(sign_extend(a.v, op.ins.first()?.size)),
        OpCode::CPUI_SUBPIECE => {
            let by = b?.v;
            unary(a.v.checked_shr(u32::try_from(by.checked_mul(8)?).ok()?).unwrap_or(0))
        }
        OpCode::CPUI_INT_ADD
        | OpCode::CPUI_INT_SUB
        | OpCode::CPUI_INT_OR
        | OpCode::CPUI_INT_AND
        | OpCode::CPUI_INT_XOR
        | OpCode::CPUI_INT_LEFT
        | OpCode::CPUI_INT_RIGHT
        | OpCode::CPUI_PIECE => {
            let b = b?;
            let shift = |n: u64| u32::try_from(n).ok().filter(|&n| n < 64);
            let v = match op.opcode {
                OpCode::CPUI_INT_ADD => a.v.wrapping_add(b.v),
                OpCode::CPUI_INT_SUB => a.v.wrapping_sub(b.v),
                OpCode::CPUI_INT_OR => a.v | b.v,
                OpCode::CPUI_INT_AND => a.v & b.v,
                OpCode::CPUI_INT_XOR => a.v ^ b.v,
                OpCode::CPUI_INT_LEFT => shift(b.v).map_or(0, |n| a.v << n),
                OpCode::CPUI_INT_RIGHT => shift(b.v).map_or(0, |n| a.v >> n),
                _ => {
                    let low = op.ins.get(1)?.size;
                    shift(u64::from(low) * 8).map_or(0, |n| a.v << n) | b.v
                }
            } & mask(out.size);
            let moved = (a.carried && v == a.v) || (b.carried && v == b.v);
            let carried = a.carried || b.carried;
            let composed = match op.opcode {
                OpCode::CPUI_INT_ADD | OpCode::CPUI_INT_SUB if carried && !moved => Some(out.size),
                OpCode::CPUI_INT_ADD | OpCode::CPUI_INT_SUB => a.composed.or(b.composed),
                _ => None,
            };
            let input = if a.carried { a } else { b };
            let (formed, base) = if moved { (input.formed, input.base) } else { (None, input.formed.or(input.base)) };
            Some(Val { v, carried, composed, age: a.age.max(b.age), pc: a.pc || b.pc, formed, base })
        }
        _ => None,
    }
}

/// A reference one instruction produces, before it joins [`Tracker::found`].
struct Hit {
    to: u64,
    kind: XrefKind,
    age: u8,
    /// The earlier `Data` row the address was computed from, when it differs.
    base: Option<usize>,
}

impl Tracker {
    /// Forget what is in flight: a new function body starts with nothing.
    pub(super) fn begin_body(&mut self) {
        self.pending.clear();
    }

    /// Record a branch edge into `to` that the walk did not take through
    /// [`Tracker::step`] (a switch case).
    pub(super) fn enter(&mut self, to: u64) {
        self.targets.insert(to);
    }

    /// Drop what instructions the walk has since discarded contributed.
    pub(super) fn discard(&mut self, removed: &HashSet<u64>) {
        self.pending.clear();
        self.found.retain(|f| !removed.contains(&f.from));
        for f in &mut self.found {
            f.stepped.retain(|t| t.last().is_none_or(|at| !removed.contains(at)));
        }
    }

    /// Consume one decoded instruction: evaluate it with what reached it, keep
    /// the references only the carried registers make, and hand what it leaves
    /// on to its fall-through.
    pub(super) fn step(&mut self, m: &Machine, insn: Insn) {
        self.targets.extend(insn.branches.iter().copied());
        let carry = if self.pending.is_empty() { None } else { self.pending.remove(&insn.vma) };
        if carry.is_none() && !has_seed(insn.ops) {
            return;
        }
        let mut frame = std::mem::take(&mut self.scratch);
        let mut hits = std::mem::take(&mut self.hits);
        self.evaluate(m, &insn, carry.unwrap_or_default(), &mut frame, &mut hits);
        self.scratch = frame;
        self.hits = hits;
    }

    fn evaluate(&mut self, m: &Machine, insn: &Insn, carry: Carry, frame: &mut Frame, hits: &mut Vec<Hit>) {
        let &Insn { vma, ops, fall, .. } = insn;
        frame.vma = vma;
        frame.regs.clear();
        frame.temps.clear();
        frame.written.clear();
        hits.clear();
        frame.regs.extend(carry.held.iter().map(|h| {
            let val = Val {
                v: h.v,
                carried: true,
                composed: None,
                age: h.age.saturating_add(1),
                pc: h.pc,
                formed: h.formed,
                base: None,
            };
            (h.off, h.size, val)
        }));
        let first = self.found.len();
        let base_of = |found: &[Found], val: &Val, to: u64| {
            val.formed.or(val.base).filter(|&i| i < first && found[i].to != to)
        };
        let fall_vma = fall.unwrap_or(u64::MAX);
        let mut conditional = false;
        for op in ops {
            let from_data = || op.ins.first().is_some_and(|vn| Some(vn.offset) == m.data_index);
            match op.opcode {
                OpCode::CPUI_LOAD | OpCode::CPUI_STORE => {
                    let load = op.opcode == OpCode::CPUI_LOAD;
                    if let Some(a) = op.ins.get(1).and_then(|vn| frame.read(m, vn)) {
                        if a.carried && from_data() && m.admits(a.v) {
                            let kind = if load { XrefKind::Read } else { XrefKind::Write };
                            let base = base_of(&self.found, &a, a.v);
                            hits.push(Hit { to: a.v, kind, age: a.age, base });
                            let width = op.out.as_ref().map(|o| o.size).filter(|_| load);
                            if let Some(t) = width.and_then(|w| m.pool?.follow(a.v, w, m.mapped)) {
                                hits.push(Hit { to: t, kind: XrefKind::Data, age: a.age, base: None });
                            }
                        }
                    }
                    if let Some(out) = op.out.as_ref().filter(|_| load) {
                        frame.write(m, out, None);
                    }
                }
                OpCode::CPUI_CALL | OpCode::CPUI_CALLIND | OpCode::CPUI_CALLOTHER => {
                    if let Some(out) = &op.out {
                        frame.write(m, out, None);
                    }
                    let (preserved, written) = (m.preserved, &frame.written);
                    frame.regs.retain(|&(o, s, _)| {
                        let end = o.saturating_add(u64::from(s));
                        preserved.iter().any(|&(lo, hi)| o >= lo && end <= hi)
                            && !written.iter().any(|&w| overlaps(w, (o, s)))
                    });
                }
                OpCode::CPUI_BRANCH | OpCode::CPUI_CBRANCH => {
                    conditional |= branches_within(op, fall_vma);
                }
                _ => {
                    let Some(out) = &op.out else { continue };
                    let a = op.ins.first().and_then(|vn| frame.read(m, vn));
                    let b = op.ins.get(1).and_then(|vn| frame.read(m, vn));
                    let mut val = fold(op, a, b);
                    if let Some(v) = val.as_mut() {
                        if v.carried
                            && v.composed == Some(out.size)
                            && m.is_register(out)
                            && v.v != fall_vma
                            && m.admits(v.v)
                        {
                            let base = base_of(&self.found, v, v.v);
                            hits.push(Hit { to: v.v, kind: XrefKind::Data, age: v.age, base });
                            v.formed = Some(first + hits.len() - 1);
                        }
                    }
                    frame.write(m, out, val);
                }
            }
        }

        for (k, h) in hits.iter().enumerate() {
            let reach = usize::from(h.age).saturating_sub(1).min(carry.trail.len());
            let mut trail: Trail = carry.trail[carry.trail.len() - reach..].iter().copied().collect();
            trail.push(vma);
            if let Some(i) = h.base {
                if !frame.regs.iter().any(|&(_, _, v)| v.formed == Some(i)) {
                    self.found[i].stepped.push(trail.clone());
                }
            }
            let writeback = h.kind == XrefKind::Data
                && hits.iter().enumerate().any(|(j, o)| j != k && o.to == h.to && o.kind != XrefKind::Data);
            self.found.push(Found { from: vma, to: h.to, kind: h.kind, trail, writeback, stepped: Vec::new() });
        }

        let Some(next) = fall else { return };
        let mut held: SmallVec<[Held; MAX_TRACKED]> = SmallVec::new();
        for &(off, size, v) in &frame.regs {
            let keep = usize::from(v.age) < MAX_HOPS
                && looks_like_address(v.v)
                && (v.carried || v.pc || !m.pc_only)
                && !(conditional && frame.written.iter().any(|&w| overlaps(w, (off, size))));
            if keep {
                let age = if v.carried { v.age } else { 0 };
                held.push(Held { off, size, v: v.v, age, pc: v.pc, formed: v.formed });
            }
        }
        if held.is_empty() {
            return;
        }
        held.sort_by_key(|h| h.age);
        held.truncate(MAX_TRACKED);
        let oldest = held.iter().map(|h| usize::from(h.age)).max().unwrap_or(0);
        let mut trail = carry.trail;
        trail.push(vma);
        let keep = oldest.min(trail.len());
        trail.drain(..trail.len() - keep);
        self.pending.entry(next).or_insert(Carry { held, trail });
    }

    /// The references whose every carrying instruction nothing else enters, as
    /// `(from, to, kind)`.
    pub(super) fn finish(&self, funcs: &BTreeSet<u64>) -> Vec<(u64, u64, XrefKind)> {
        let holds = |trail: &[u64]| trail.iter().all(|at| !self.targets.contains(at) && !funcs.contains(at));
        self.found
            .iter()
            .filter(|f| !f.writeback && holds(&f.trail) && !f.stepped.iter().any(|t| holds(t)))
            .map(|f| (f.from, f.to, f.kind))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `(constant, register, unique)`, as [`super::super::kuna_picbase`]'s tests
    /// build them; a `LOAD`/`STORE` names the data space by index 2.
    fn spaces() -> (Rc<AddrSpace>, Rc<AddrSpace>, Rc<AddrSpace>) {
        (
            Rc::new(AddrSpace::new_for_decode(spacetype::IPTR_CONSTANT)),
            Rc::new(AddrSpace::new_for_decode(spacetype::IPTR_PROCESSOR)),
            Rc::new(AddrSpace::new_for_decode(spacetype::IPTR_INTERNAL)),
        )
    }

    fn vn(space: &Rc<AddrSpace>, offset: u64, size: u32) -> VarnodeData {
        VarnodeData { space: Some(Rc::clone(space)), offset, size }
    }

    fn op(opcode: OpCode, out: Option<VarnodeData>, ins: Vec<VarnodeData>) -> FullOp {
        FullOp { opcode, out, ins }
    }

    const MAPPED: [(u64, u64); 1] = [(0x10000, 0x40000)];
    /// The callee-saved span (`x19`).
    const PRESERVED: [(u64, u64); 1] = [(0x98, 0xa0)];

    struct Isa {
        c: Rc<AddrSpace>,
        r: Rc<AddrSpace>,
        u: Rc<AddrSpace>,
    }

    impl Isa {
        fn new() -> Self {
            let (c, r, u) = spaces();
            Isa { c, r, u }
        }
        fn machine(&self, pc_only: bool) -> Machine<'static> {
            Machine {
                reg: Rc::clone(&self.r),
                data_index: Some(2),
                big_endian: false,
                pc_only,
                preserved: &PRESERVED,
                mapped: &MAPPED,
                pool: None,
            }
        }
        fn k(&self, v: u64) -> VarnodeData {
            vn(&self.c, v, 8)
        }
        fn x(&self, n: u64) -> VarnodeData {
            vn(&self.r, n * 8, 8)
        }
        fn t(&self, n: u64) -> VarnodeData {
            vn(&self.u, 0x100 + n * 8, 8)
        }
        /// `xD = page` (`adrp`, `lui` folded).
        fn set(&self, d: u64, v: u64) -> Vec<FullOp> {
            vec![op(OpCode::CPUI_COPY, Some(self.x(d)), vec![self.k(v)])]
        }
        /// `xD = xN + imm`.
        fn add(&self, d: u64, n: u64, imm: u64) -> Vec<FullOp> {
            vec![op(OpCode::CPUI_INT_ADD, Some(self.x(d)), vec![self.x(n), self.k(imm)])]
        }
        /// `xD = *(xN + imm)`.
        fn load(&self, d: u64, n: u64, imm: u64) -> Vec<FullOp> {
            vec![
                op(OpCode::CPUI_INT_ADD, Some(self.t(0)), vec![self.x(n), self.k(imm)]),
                op(OpCode::CPUI_LOAD, Some(self.x(d)), vec![self.k(2), self.t(0)]),
            ]
        }
        /// `*(xN + imm) = xS`.
        fn store(&self, s: u64, n: u64, imm: u64) -> Vec<FullOp> {
            vec![
                op(OpCode::CPUI_INT_ADD, Some(self.t(0)), vec![self.x(n), self.k(imm)]),
                op(OpCode::CPUI_STORE, None, vec![self.k(2), self.t(0), self.x(s)]),
            ]
        }
        /// `bl target`, writing the link register `x30` first.
        fn call(&self, at: u64) -> Vec<FullOp> {
            vec![
                op(OpCode::CPUI_INT_ADD, Some(self.x(30)), vec![self.k(at), self.k(4)]),
                op(OpCode::CPUI_CALL, None, vec![self.k(0x30000)]),
            ]
        }
    }

    /// Walk `insns` as one straight-line run starting at 0x20000, four bytes
    /// apart, with `targets` entered from elsewhere.
    fn walk(m: &Machine, insns: &[Vec<FullOp>], targets: &[u64]) -> Vec<(u64, u64, XrefKind)> {
        let mut t = Tracker::default();
        for &at in targets {
            t.enter(at);
        }
        for (i, ops) in insns.iter().enumerate() {
            let vma = 0x20000 + 4 * i as u64;
            t.step(m, Insn { vma, ops, fall: Some(vma + 4), branches: &[] });
        }
        let mut out = t.finish(&BTreeSet::from([0x20000]));
        out.sort_unstable();
        out
    }

    #[test]
    fn a_page_and_its_offset_form_the_address_at_the_second_instruction() {
        let isa = Isa::new();
        let m = isa.machine(false);
        let got = walk(&m, &[isa.set(0, 0x21000), isa.add(0, 0, 0x488)], &[]);
        assert_eq!(got, vec![(0x20004, 0x21488, XrefKind::Data)]);
    }

    #[test]
    fn a_load_and_a_store_through_a_carried_base_are_a_read_and_a_write() {
        let isa = Isa::new();
        let m = isa.machine(false);
        let got = walk(&m, &[isa.set(1, 0x21000), isa.load(0, 1, 0x1d0), isa.store(0, 1, 0x1d8)], &[]);
        assert_eq!(got, vec![(0x20004, 0x211d0, XrefKind::Read), (0x20008, 0x211d8, XrefKind::Write)]);
    }

    #[test]
    fn a_join_between_the_halves_drops_the_reference() {
        let isa = Isa::new();
        let m = isa.machine(false);
        let insns = [isa.set(0, 0x21000), isa.set(1, 5), isa.add(0, 0, 0x488)];
        assert_eq!(walk(&m, &insns, &[0x20008]).len(), 0, "the add is a branch target");
        assert_eq!(walk(&m, &insns, &[0x20004]).len(), 0, "so is the instruction it passed through");
        assert_eq!(walk(&m, &insns, &[0x20000]).len(), 1, "the page's own instruction may be a join");
    }

    #[test]
    fn a_call_keeps_a_preserved_register_and_drops_the_rest() {
        let isa = Isa::new();
        let m = isa.machine(false);
        let saved = walk(&m, &[isa.set(19, 0x21000), isa.call(0x20004), isa.add(0, 19, 0x25)], &[]);
        assert_eq!(saved, vec![(0x20008, 0x21025, XrefKind::Data)]);
        let scratch = walk(&m, &[isa.set(8, 0x21000), isa.call(0x20004), isa.add(0, 8, 0x25)], &[]);
        assert!(scratch.is_empty(), "{scratch:?}");
    }

    #[test]
    fn a_register_the_call_itself_writes_does_not_survive_it() {
        let isa = Isa::new();
        let preserved = [(0x98, 0xa0), (30 * 8, 31 * 8)];
        let m = Machine { preserved: &preserved, ..isa.machine(false) };
        let got = walk(&m, &[isa.call(0x20000), isa.add(0, 30, 0x10)], &[]);
        assert!(got.is_empty(), "a link register the cspec calls unaffected: {got:?}");
    }

    #[test]
    fn a_copy_bitwise_or_narrowed_composition_is_not_an_address() {
        let isa = Isa::new();
        let m = isa.machine(false);
        let mv = walk(&m, &[isa.set(0, 0x21000), isa.add(1, 0, 0)], &[]);
        assert!(mv.is_empty(), "`addi a1,a0,0` is a move: {mv:?}");
        let ori = vec![op(OpCode::CPUI_INT_OR, Some(isa.x(0)), vec![isa.x(0), isa.k(0xff)])];
        assert!(walk(&m, &[isa.set(0, 0x21000), ori], &[]).is_empty(), "`ori` builds a number");
        let w = vn(&isa.u, 0x200, 4);
        let addiw = vec![
            op(OpCode::CPUI_INT_ADD, Some(isa.t(0)), vec![isa.x(0), isa.k(0x4c0)]),
            op(OpCode::CPUI_SUBPIECE, Some(w.clone()), vec![isa.t(0), vn(&isa.c, 0, 4)]),
            op(OpCode::CPUI_INT_SEXT, Some(isa.x(0)), vec![w]),
        ];
        assert!(walk(&m, &[isa.set(0, 0x21000), addiw], &[]).is_empty(), "`addiw` is 32-bit arithmetic");
    }

    #[test]
    fn a_position_independent_image_carries_only_a_pc_derived_value() {
        let isa = Isa::new();
        let m = isa.machine(true);
        let lui = walk(&m, &[isa.set(0, 0x21000), isa.add(0, 0, 0x488)], &[]);
        assert!(lui.is_empty(), "an absolute immediate is a number in an ET_DYN image: {lui:?}");
        let auipc = vec![op(OpCode::CPUI_INT_ADD, Some(isa.x(0)), vec![isa.k(0x1000), isa.k(0x20000)])];
        let got = walk(&m, &[auipc, isa.add(0, 0, 0x488)], &[]);
        assert_eq!(got, vec![(0x20004, 0x21488, XrefKind::Data)]);
    }

    #[test]
    fn a_value_formed_only_to_form_the_next_is_a_step() {
        let isa = Isa::new();
        let m = isa.machine(false);
        let writeback = vec![
            op(OpCode::CPUI_INT_ADD, Some(isa.x(12)), vec![isa.x(12), isa.k(0x2f0)]),
            op(OpCode::CPUI_LOAD, Some(isa.t(1)), vec![isa.k(2), isa.x(12)]),
        ];
        let got = walk(&m, &[isa.set(12, 0x20008), isa.add(12, 12, 0x1000), writeback], &[]);
        assert_eq!(got, vec![(0x20008, 0x212f8, XrefKind::Read)], "an A32 PLT entry reads its slot");
        let kept = walk(&m, &[isa.set(8, 0x21000), isa.add(8, 8, 0x40), isa.load(0, 8, 8)], &[]);
        assert_eq!(kept, vec![(0x20004, 0x21040, XrefKind::Data), (0x20008, 0x21048, XrefKind::Read)]);
    }

    #[test]
    fn a_predicated_write_is_not_carried_and_the_hop_bound_ends_a_carry() {
        let isa = Isa::new();
        let m = isa.machine(false);
        let mut predicated = isa.set(0, 0x22000);
        predicated.insert(0, op(OpCode::CPUI_CBRANCH, None, vec![isa.k(0x20008), isa.x(5)]));
        let got = walk(&m, &[isa.set(0, 0x21000), predicated, isa.add(0, 0, 0x10)], &[]);
        assert!(got.is_empty(), "x0 is 0x21000 or 0x22000 after a conditional write: {got:?}");
        let mut far = vec![isa.set(0, 0x21000)];
        far.extend((0..MAX_HOPS).map(|_| isa.set(1, 5)));
        far.push(isa.add(0, 0, 0x10));
        assert!(walk(&m, &far, &[]).is_empty());
    }
}
