//! (kuna) Read an x86-32 callee's stack purge off its own returns.
//!
//! Under a model whose extrapop is `unknown` (`x86win.cspec`'s `__stdcall`,
//! `__thiscall`, `__fastcall`), `StackSolver` has only a guess for a call that
//! no later merge pins, and the guess for an internal callee is "pops nothing".
//! A `__stdcall` or `__thiscall` callee pops its arguments, so every stack
//! offset after such a call is displaced by their size: stack arguments of
//! later calls are dropped and the caller's locals alias outgoing-argument
//! slots. Ghidra's analysis records each function's purge from its `RET n`;
//! this does the same for a direct call's target, at flow time, but only where
//! the callee's own code proves it.
//!
//! The callee's p-code is walked from its entry along every path (fall-through
//! and direct branches, so an incremental-link `jmp` thunk reaches the body),
//! tracking the stack pointer and the general registers as entry-relative
//! stack addresses. A return states an extrapop only when the stack pointer is
//! exactly back at its entry value as the `RET` starts; the extrapop is then
//! the constant the `RET` adds (`RET 0x14` is `ESP += 4; ESP += 0x14`). A
//! direct call inside the callee moves the stack pointer by that callee's own
//! proven extrapop, read the same way a bounded number of levels down; any
//! other call leaves it unknown, so the callee's returns must restore it from a
//! frame pointer the callee itself set. Every reachable return has to agree.
//!
//! The CRT helpers that move their caller's stack (`_chkstk`/`_alloca_probe`,
//! `__SEH_prolog4`/`__SEH_epilog4`, `__EH_prolog3`/`__EH_epilog3`) return with
//! the stack pointer somewhere else, by their own arithmetic or by loading it
//! from the caller's frame pointer, and are classified as such so their callers
//! do not trust the registers they rewrite. An indirect jump, an undecodable
//! instruction, a return reached with the stack pointer unknown, an import's
//! address-table slot, or a budget overrun states nothing.
//!
//! A proven pop is applied only where it leaves the caller's stack model with
//! nothing to guess: every call in the function has a pop its prototype states
//! or its callee proves ([`every_pop_settled`]). The solver's guesses for the
//! calls it cannot read are coherent with each other, not with the truth (an
//! import's guess counts the pushes in front of it, which in
//! `f(GetModuleHandleA(0), ...)` include the outer call's), so one proven pop
//! among them moves the guessed calls' arguments instead of fixing them.

use crate::kuna_fasthash::FxHashMap as HashMap;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{spacetype, AddrSpace};
use kuna_base::types::{int4, uintb};
use kuna_num::opcodes::OpCode;
use kuna_num::pcoderaw::VarnodeData;
use kuna_sleigh::translate::PcodeEmit;

use crate::architecture::Architecture;
use crate::fspec::EXTRAPOP_UNKNOWN;

/// Distinct instructions decoded per callee before the walk gives up.
const BUDGET: usize = 2048;

/// Instruction visits per callee, re-visits after a widening included.
const STEPS: usize = 8192;

/// Levels of nested direct calls whose own returns are read.
const DEPTH: u8 = 3;

/// Instructions read after a call for the caller's cleanup of its arguments.
const CLEANUP_SCAN: usize = 32;

/// The largest extrapop accepted.
const MAX_EXTRAPOP: int4 = 0x404;

/// An unbounded side of a stack range.
const INF: i64 = 1 << 40;

/// The general registers, in register-space order.
const GPRS: [&str; 8] = ["EAX", "ECX", "EDX", "EBX", "ESP", "EBP", "ESI", "EDI"];
const SP: usize = 4;
const BP: usize = 5;
const CALLER_SAVED: [usize; 3] = [0, 1, 2];
const CALLEE_SAVED: [usize; 4] = [3, 5, 6, 7];

/// The extrapop (`4 + purge`) of the direct call target `entry`, when the
/// language is x86-32 under a default model that leaves extrapop unknown and
/// the callee's own returns prove it.
pub fn callee_extra_pop(arch: &Architecture, entry: &Address) -> Option<int4> {
    if !applies(arch) || entry.is_invalid() {
        return None;
    }
    match summary(arch, entry, DEPTH).effect {
        Effect::Balanced(ep) => Some(ep),
        _ => None,
    }
}

/// Does every call in `data` have a pop its prototype states or its callee's
/// returns prove?
pub fn every_pop_settled(data: &crate::funcdata::Funcdata) -> bool {
    (0..data.num_calls()).all(|i| {
        let fc = data.get_call_specs(i);
        fc.get_extra_pop() != EXTRAPOP_UNKNOWN || fc.callee_purge().is_some()
    })
}

/// (`calleeargs`) The stack bytes the callee of the direct call at `call_at`
/// touches and can take as arguments, when the walk places every access it
/// makes. A parameter the callee never reads is still pushed and removed for
/// it, so what its returns pop and what the caller removes right after the call
/// widen the bound.
pub fn callee_arg_bytes(arch: &Architecture, entry: &Address, call_at: &Address) -> Option<StackArgs> {
    if !applies(arch) || entry.is_invalid() {
        return None;
    }
    let s = summary(arch, entry, DEPTH);
    let read = s.arg_bytes?;
    let popped = s.ret_pops.map_or(0, |ep| (ep - 4).max(0) as u32);
    Some(StackArgs { read, bound: read.max(popped).max(caller_cleanup(arch, call_at)) })
}

/// (`calleeargs`) For a direct callee whose extent is not stated, the stack
/// bytes its reached returns pop or the caller removes right after the call at
/// `call_at`, when either is seen. A return reached by decoding past a call not
/// known to end the path can make it wrong, so it only ever keeps a trial.
pub fn callee_window(arch: &Architecture, entry: &Address, call_at: &Address) -> Option<u32> {
    if !applies(arch) || entry.is_invalid() {
        return None;
    }
    let s = summary(arch, entry, DEPTH);
    if s.effect == Effect::Unbalanced {
        return None;
    }
    let cleanup = caller_cleanup(arch, call_at);
    match s.ret_pops {
        Some(ep) => Some(((ep - 4).max(0) as u32).max(cleanup)),
        None => (cleanup > 0).then_some(cleanup),
    }
}

/// Does the direct call target `entry` return with its caller's stack pointer
/// moved by its own code (a stack-moving helper)?
pub fn moves_stack(arch: &Architecture, entry: &Address) -> bool {
    applies(arch) && !entry.is_invalid() && summary(arch, entry, DEPTH).effect == Effect::Unbalanced
}

/// (`calleeargs`) What a direct call's callee does with the stack above its
/// return address, in bytes from the first argument slot.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StackArgs {
    /// The bytes its code touches.
    pub read: u32,
    /// The bytes that can be its arguments at this call: `read`, widened to
    /// what its returns pop and what the caller removes right after the call.
    pub bound: u32,
}

/// The bytes the caller takes off the stack after the call at `call_at`, read
/// along the straight-line code that follows it: how far above where the call
/// returns (taking only its return address) a cleanup lifts the stack pointer
/// (`pop ecx`, `add esp,8`, or one MSVC defers past later calls) before a
/// branch or a return. A later call is taken to pop what its callee's returns
/// prove, or else only its return address, so a cleanup is never credited
/// with more than the caller pushed. A pop into a callee-saved register is the
/// epilogue restoring it and ends the scan: the frame teardown after it is not
/// a cleanup, however far a later call's unproven pop has left the scan's
/// stack pointer from the real one. One right after a push is a move
/// (`push 3; pop ebx`), which the scan passes over.
pub(crate) fn caller_cleanup(arch: &Architecture, call_at: &Address) -> u32 {
    let (Some(space), Some(regs)) = (call_at.get_space().cloned(), registers(arch)) else { return 0 };
    let Some(call) = decode(arch, &space, call_at.get_offset()) else { return 0 };
    let mut at = call_at.get_offset().wrapping_add(call.1);
    let (mut sp, mut high, mut pushed) = (0i64, 0i64, false);
    for _ in 0..CLEANUP_SCAN {
        let Some(insn) = decode(arch, &space, at) else { break };
        let (ops, len) = (&insn.0, insn.1);
        let mut state = State::at_entry();
        state.0[SP] = Some(Val::exact(sp));
        let s = step(ops, &regs, state);
        if s.ret.is_some() || !s.targets.is_empty() || !s.falls_through {
            break;
        }
        let Some(after) = s.state.0[SP].and_then(Val::as_exact) else { break };
        let pops = match s.call {
            None => 0,
            Some(Call::Direct(t)) => match summary(arch, &Address::new(space.clone(), t), DEPTH).effect {
                Effect::Balanced(ep) => ep as i64,
                Effect::Unknown => 4,
                _ => break,
            },
            Some(Call::Indirect(_)) => 4,
        };
        if s.call.is_none() && after > sp {
            if s.writes == 0 {
                high = high.max(after);
            } else if !pushed {
                break;
            }
        }
        pushed = s.call.is_none() && after == sp - 4;
        sp = after + pops;
        at = at.wrapping_add(len);
    }
    high.min(MAX_EXTRAPOP as i64) as u32
}

/// x86-32 under a default model whose extrapop is unknown.
pub(crate) fn applies(arch: &Architecture) -> bool {
    arch.archid.starts_with("x86:LE:32:")
        && arch.default_fp().map(|m| m.get_extra_pop()) == Some(EXTRAPOP_UNKNOWN)
}

/// What a callee does to its caller's stack pointer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Effect {
    /// Every reachable return leaves the stack pointer this far above where
    /// the call left it.
    Balanced(int4),
    /// No path returns.
    NoReturn,
    /// A return is reached with the stack pointer moved by the callee's own
    /// code: a stack-moving helper.
    Unbalanced,
    /// The walk cannot tell.
    Unknown,
}

/// The cached result of walking one callee.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Summary {
    pub effect: Effect,
    /// The callee-saved registers ([`CALLEE_SAVED`] bits) the callee's own
    /// code, or a helper it calls, writes.
    pub writes: u8,
    /// (`calleeargs`) The stack-argument bytes the callee touches, rounded up
    /// to whole slots, when every access on every path is placed.
    pub arg_bytes: Option<u32>,
    /// The most any reached return adds to the stack pointer, whether or not
    /// the stack pointer was back at its entry value there.
    pub ret_pops: Option<int4>,
    /// (`calleeargs`) A return leaves a stack address in a caller-saved
    /// register.
    pub returns_stack: bool,
}

const UNKNOWN: Summary =
    Summary { effect: Effect::Unknown, writes: 0, arg_bytes: None, ret_pops: None, returns_stack: false };

/// One instruction's p-code and length.
type Decoded = Rc<(Vec<(OpCode, Option<VarnodeData>, Vec<VarnodeData>)>, uintb)>;

/// Decoded instructions kept for the walks, at most.
const DECODED: usize = 1 << 12;

/// The walks already taken, on the architecture.
#[derive(Default)]
pub struct Cache {
    /// Whether the walks kept were taken with `calleeargs` on, which follows
    /// the stack addresses a walk loses.
    tracked: bool,
    /// Instructions the walks decoded, emptied when full.
    decoded: HashMap<uintb, Decoded>,
    /// Walks whose nested calls all fit within the depth asked, with the
    /// depth they needed: the answer for that depth and every deeper one.
    whole: HashMap<uintb, (Summary, u8)>,
    /// Walks the depth limit cut short, by the depth asked; at depth [`CUT`],
    /// what a call through a pointer slot does.
    cut: HashMap<(uintb, u8), Summary>,
}

/// A depth no walk reaches: the walk was cut short.
const CUT: u8 = u8::MAX;

/// The summary of the function at `entry`, reading nested direct calls
/// `depth` levels down.
pub(crate) fn summary(arch: &Architecture, entry: &Address, depth: u8) -> Summary {
    summary_at(arch, entry, depth).0
}

/// [`summary`] and the depth it needed. A walk is reused at any depth it fits
/// within and otherwise cached per depth, so the answer never depends on which
/// function asked first; the walks are dropped when `calleeargs` changes.
fn summary_at(arch: &Architecture, entry: &Address, depth: u8) -> (Summary, u8) {
    let off = entry.get_offset();
    if arch.kuna_callee_purge.borrow().tracked != arch.kuna_calleeargs {
        let mut cache = arch.kuna_callee_purge.borrow_mut();
        cache.tracked = arch.kuna_calleeargs;
        cache.whole.clear();
        cache.cut.clear();
    }
    {
        let cache = arch.kuna_callee_purge.borrow();
        if let Some(&(hit, need)) = cache.whole.get(&off).filter(|(_, need)| *need <= depth) {
            return (hit, need);
        }
        if let Some(&hit) = cache.cut.get(&(off, depth)) {
            return (hit, CUT);
        }
    }
    let found = if crate::p4_calls::kuna_calleenoreturn::is_no_return(arch, entry) {
        (Summary { effect: Effect::NoReturn, ..UNKNOWN }, 0)
    } else if is_import(arch, entry) {
        (UNKNOWN, 0)
    } else {
        walk(arch, entry, depth).unwrap_or((UNKNOWN, 0))
    };
    let mut cache = arch.kuna_callee_purge.borrow_mut();
    if found.1 <= depth {
        cache.whole.insert(off, found);
    } else {
        cache.cut.insert((off, depth), found.0);
    }
    found
}

/// What a call through the pointer at `slot` (an import's address-table entry)
/// does: it does not return when the slot names a function that does not, and
/// is otherwise unknown. Cached with the walks.
fn slot_effect(arch: &Architecture, slot: &Address) -> Summary {
    let key = (slot.get_offset(), CUT);
    if let Some(&hit) = arch.kuna_callee_purge.borrow().cut.get(&key) {
        return hit;
    }
    let found = if crate::p4_calls::kuna_calleenoreturn::is_no_return(arch, slot) {
        Summary { effect: Effect::NoReturn, ..UNKNOWN }
    } else {
        UNKNOWN
    };
    arch.kuna_callee_purge.borrow_mut().cut.insert(key, found);
    found
}

/// The instruction at `at`, decoded once for every walk that reaches it.
fn decode(arch: &Architecture, space: &Rc<AddrSpace>, at: uintb) -> Option<Decoded> {
    if let Some(hit) = arch.kuna_callee_purge.borrow().decoded.get(&at) {
        return Some(hit.clone());
    }
    let mut collect = Collect::default();
    let len = arch.translate().one_instruction(&mut collect, &Address::new(space.clone(), at)).ok()?;
    let found: Decoded = Rc::new((collect.0, len.max(1) as uintb));
    let mut cache = arch.kuna_callee_purge.borrow_mut();
    if cache.decoded.len() >= DECODED {
        cache.decoded.clear();
    }
    cache.decoded.insert(at, found.clone());
    Some(found)
}

/// An import's address-table slot, which a resolved `call [slot]` targets:
/// data, not code to walk.
fn is_import(arch: &Architecture, entry: &Address) -> bool {
    let externref = crate::varnode::varnode_flags::externref;
    if arch.symboltab.get_property(entry) & externref != 0 {
        return true;
    }
    let Some(global) = arch.symboltab.get_global_scope() else { return false };
    arch.symboltab.query_properties(global, entry, 4, entry).1 & externref != 0
}

#[derive(Default)]
struct Collect(Vec<(OpCode, Option<VarnodeData>, Vec<VarnodeData>)>);

impl PcodeEmit for Collect {
    fn dump(&mut self, _at: &Address, opcode: OpCode, out: Option<&VarnodeData>, inputs: &[VarnodeData]) {
        self.0.push((opcode, out.cloned(), inputs.to_vec()));
    }
}

/// An entry-relative stack address, as far as the walk can place it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Val {
    /// A stack address between `lo` and `hi` bytes above the entry stack
    /// pointer; `own` when the imprecision comes from the callee's own code
    /// rather than from a call it makes.
    Stack { lo: i64, hi: i64, own: bool },
    /// The frame pointer the caller left in `EBP`.
    CallerFrame,
    /// Possibly a stack address the walk cannot place.
    Opaque { own: bool },
}

fn bound(b: i64, k: i64) -> i64 {
    if b <= -INF {
        -INF
    } else if b >= INF {
        INF
    } else {
        (b + k).clamp(-INF, INF)
    }
}

impl Val {
    pub(crate) fn exact(o: i64) -> Val {
        Val::Stack { lo: o, hi: o, own: false }
    }

    pub(crate) fn as_exact(self) -> Option<i64> {
        match self {
            Val::Stack { lo, hi, .. } if lo == hi => Some(lo),
            _ => None,
        }
    }

    fn own(self) -> bool {
        match self {
            Val::Stack { own, .. } | Val::Opaque { own } => own,
            Val::CallerFrame => true,
        }
    }

    fn shift(self, k: i64) -> Val {
        match self {
            Val::Stack { lo, hi, own } => Val::Stack { lo: bound(lo, k), hi: bound(hi, k), own },
            Val::CallerFrame => Val::Opaque { own: true },
            v => v,
        }
    }

    /// The value plus or minus a register: a pointer into a caller's local
    /// object stays among the locals; anything else is unplaced.
    fn index(self, plus: bool, is_sp: bool) -> Val {
        match self {
            Val::Stack { lo, hi, .. } if is_sp => {
                if plus {
                    Val::Stack { lo, hi: INF, own: true }
                } else {
                    Val::Stack { lo: -INF, hi, own: true }
                }
            }
            Val::Stack { hi, .. } if hi < 0 => Val::Stack { lo: -INF, hi: -1, own: true },
            Val::Stack { .. } => Val::Stack { lo: -INF, hi: INF, own: true },
            _ => Val::Opaque { own: true },
        }
    }
}

/// The least value both paths agree on; two different exact stack addresses
/// meeting is the callee's own doing, and an imprecision a call left stays the
/// call's.
fn join(a: Option<Val>, b: Option<Val>) -> Option<Val> {
    match (a, b) {
        (None, x) | (x, None) => x,
        (Some(x), Some(y)) if x == y => Some(x),
        (Some(Val::Stack { lo: l1, hi: h1, own: o1 }), Some(Val::Stack { lo: l2, hi: h2, own: o2 })) => {
            let differ = l1 == h1 && l2 == h2;
            Some(Val::Stack { lo: l1.min(l2), hi: h1.max(h2), own: o1 || o2 || differ })
        }
        (Some(x), Some(y)) => Some(Val::Opaque { own: x.own() || y.own() }),
    }
}

/// `joined` pushed to a fixed point against `old`: a bound that moved goes
/// unbounded, except that a pointer walking a local object stays below the
/// return address.
fn widen(old: Option<Val>, joined: Option<Val>, is_sp: bool) -> Option<Val> {
    match (old, joined) {
        (Some(Val::Stack { lo: l0, hi: h0, .. }), Some(Val::Stack { lo, hi, own })) if (lo, hi) != (l0, h0) => {
            let lo = if lo < l0 { -INF } else { lo };
            let hi = if hi > h0 {
                if !is_sp && h0 < 0 {
                    -1
                } else {
                    INF
                }
            } else {
                hi
            };
            Some(Val::Stack { lo, hi, own })
        }
        (_, j) => j,
    }
}

/// The walk's state at an instruction boundary: each general register's value
/// (`None`, not a stack address), and (`calleeargs`) the general registers
/// ([`State::lost`]) and the stack slots below the return address
/// ([`slot_bit`]) that may hold a stack address the walk no longer places.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct State(pub [Option<Val>; 8], pub u8, pub u64);

impl State {
    pub(crate) fn at_entry() -> State {
        let mut r = [None; 8];
        r[SP] = Some(Val::exact(0));
        r[BP] = Some(Val::CallerFrame);
        State(r, 0, 0)
    }

    fn join(&self, other: &State) -> State {
        let mut r = self.0;
        for (i, v) in r.iter_mut().enumerate() {
            *v = widen(self.0[i], join(self.0[i], other.0[i]), i == SP);
        }
        State(r, self.1 | other.1, self.2 | other.2)
    }

    /// Does general register `r` hold a stack address the walk lost?
    pub(crate) fn lost(&self, r: usize) -> bool {
        self.1 & (1 << r) != 0
    }
}

/// The bit of the 4-byte stack slot at entry-relative `o`, for the 64 slots
/// below the return address.
pub(crate) fn slot_bit(o: i64) -> Option<u64> {
    ((-256..0).contains(&o) && o % 4 == 0).then(|| 1u64 << (-o / 4 - 1))
}

/// The bits of the slots an access of `size` bytes at `at` may overlap.
fn slots_in(at: Option<Val>, size: u32) -> u64 {
    let Some(Val::Stack { lo, hi, .. }) = at else { return 0 };
    let end = bound(hi, size as i64).min(0);
    let mut o = lo.max(-256).div_euclid(4) * 4;
    let mut bits = 0;
    while o < end {
        bits |= slot_bit(o).unwrap_or(0);
        o += 4;
    }
    bits
}

/// The general registers as the walk recognizes them.
pub(crate) struct Regs {
    pub space: i32,
    pub offs: [u64; 8],
}

impl Regs {
    fn space_of(v: &VarnodeData) -> Option<i32> {
        v.space.as_ref().map(|s| s.get_index())
    }

    /// The general register `v` is exactly.
    fn exact(&self, v: &VarnodeData) -> Option<usize> {
        if v.size != 4 || Self::space_of(v) != Some(self.space) {
            return None;
        }
        self.offs.iter().position(|&o| o == v.offset)
    }

    /// The general registers `v` overlaps without being one of them.
    fn partial(&self, v: &VarnodeData) -> impl Iterator<Item = usize> + '_ {
        let hit = Self::space_of(v) == Some(self.space) && self.exact(v).is_none();
        let (lo, hi) = (v.offset, v.offset + v.size as u64);
        self.offs.iter().enumerate().filter(move |(_, &o)| hit && lo < o + 4 && o < hi).map(|(i, _)| i)
    }
}

fn kind(v: &VarnodeData) -> Option<spacetype> {
    v.space.as_ref().map(|s| s.get_type())
}

fn konst(v: &VarnodeData) -> Option<i64> {
    (kind(v) == Some(spacetype::IPTR_CONSTANT)).then(|| v.offset as u32 as i32 as i64)
}

/// The call an instruction ends in.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Call {
    Direct(uintb),
    /// Through a register or memory; the slot when the target was loaded from
    /// a constant address (an import's IAT entry).
    Indirect(Option<uintb>),
}

/// What one instruction does to the walk.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Step {
    pub state: State,
    pub targets: Vec<uintb>,
    pub falls_through: bool,
    /// A return: the stack pointer as the instruction starts and at its
    /// `RETURN`.
    pub ret: Option<(Val, Val)>,
    /// What the instruction's own constant adjustments add to the stack pointer
    /// before its `RETURN` (`RET 0x14`: 0x18).
    pub ret_pop: Option<i64>,
    pub call: Option<Call>,
    /// An indirect jump: the walk cannot follow it.
    pub unknown: bool,
    /// The callee-saved registers ([`CALLEE_SAVED`] bits) the instruction writes.
    pub writes: u8,
    /// The end (entry-relative) of the highest argument byte touched, 4 if none.
    pub arg_end: i64,
    /// An access the walk cannot place, or an argument's address handed on.
    pub unplaced: bool,
}

/// Note an access of `size` bytes at `at`: a stack address wholly below the
/// first argument is a local; an exact one above it is an argument byte.
pub(crate) fn touch(at: Option<Val>, size: u32, arg_end: &mut i64, unplaced: &mut bool) {
    match at {
        None => {}
        Some(Val::Stack { lo, hi, .. }) => {
            if bound(hi, size as i64) <= 4 {
            } else if lo == hi {
                *arg_end = (*arg_end).max(lo + size as i64);
            } else {
                *unplaced = true;
            }
        }
        Some(_) => *unplaced = true,
    }
}

/// Note `v` leaving the walk's sight (stored, or live in a register at a call
/// or return): an address at or above the return address hands the arguments
/// on; the caller's frame pointer and a local's address do not.
pub(crate) fn escape(v: Option<Val>, unplaced: &mut bool) {
    match v {
        Some(Val::Stack { hi, .. }) if hi >= 0 => *unplaced = true,
        Some(Val::Opaque { .. }) => *unplaced = true,
        _ => {}
    }
}

fn saved_bit(r: usize) -> u8 {
    CALLEE_SAVED.iter().position(|&s| s == r).map_or(0, |i| 1 << i)
}

/// Summarize one instruction's p-code from `state`.
pub(crate) fn step(ops: &[(OpCode, Option<VarnodeData>, Vec<VarnodeData>)], regs: &Regs, state: State) -> Step {
    step_with(ops, regs, state, false)
}

/// [`step`], also following (`track`, for `calleeargs`) the stack addresses
/// the walk loses: one parked in a stack slot and loaded back, or returned by a
/// call. A load or store through one leaves the extent unstated, and so does a
/// stack address written outside the general registers or into part of one,
/// including one stored through a pointer the walk cannot place (the caller's
/// frame pointer excepted, which the CRT's `_NLG_Notify` records).
pub(crate) fn step_with(
    ops: &[(OpCode, Option<VarnodeData>, Vec<VarnodeData>)],
    regs: &Regs,
    state: State,
    track: bool,
) -> Step {
    let mut s = Step {
        state,
        targets: Vec::new(),
        falls_through: true,
        ret: None,
        ret_pop: None,
        call: None,
        unknown: false,
        writes: 0,
        arg_end: 4,
        unplaced: false,
    };
    let start_sp = state.0[SP].unwrap_or(Val::Opaque { own: true });
    let mut temps: Vec<(u64, Option<Val>)> = Vec::new();
    let mut loaded_from: Vec<(u64, uintb)> = Vec::new();
    let mut trapped: Vec<u64> = Vec::new();
    let mut lost_temps: Vec<u64> = Vec::new();
    let mut sp_adds: i64 = 0;
    let unique = |v: &VarnodeData| kind(v) == Some(spacetype::IPTR_INTERNAL);
    for (opc, out, ins) in ops {
        let lost_of = |v: &VarnodeData, s: &Step| -> bool {
            if let Some(r) = regs.exact(v) {
                return s.state.lost(r);
            }
            if unique(v) {
                return lost_temps.contains(&v.offset);
            }
            regs.partial(v).any(|r| s.state.lost(r))
        };
        let lost: Vec<bool> = if track { ins.iter().map(|v| lost_of(v, &s)).collect() } else { Vec::new() };
        let lost_at = |i: usize| lost.get(i).copied().unwrap_or(false);
        let mut out_lost = lost.iter().any(|&b| b);
        let eval = |v: &VarnodeData, s: &Step| -> Option<Val> {
            if let Some(r) = regs.exact(v) {
                return s.state.0[r];
            }
            if unique(v) {
                return temps.iter().rev().find(|(o, _)| *o == v.offset).and_then(|(_, x)| *x);
            }
            None
        };
        let vals: Vec<Option<Val>> = ins.iter().map(|v| eval(v, &s)).collect();
        let any_stack = vals.iter().any(|v| v.is_some());
        let result: Option<Val> = match opc {
            OpCode::CPUI_COPY => vals.first().copied().flatten(),
            OpCode::CPUI_INT_ADD | OpCode::CPUI_INT_SUB if ins.len() == 2 => {
                let plus = *opc == OpCode::CPUI_INT_ADD;
                let to_sp = out.as_ref().and_then(|o| regs.exact(o)) == Some(SP);
                if let (true, Some(SP), Some(k)) = (to_sp, regs.exact(&ins[0]), konst(&ins[1])) {
                    sp_adds += if plus { k } else { -k };
                }
                match (vals[0], vals[1], konst(&ins[0]), konst(&ins[1])) {
                    (Some(v), _, _, Some(k)) => Some(v.shift(if plus { k } else { -k })),
                    (_, Some(v), Some(k), _) if plus => Some(v.shift(k)),
                    (Some(_), Some(_), _, _) if !plus => None,
                    (Some(_), Some(_), _, _) => Some(Val::Opaque { own: true }),
                    (Some(v), None, _, None) => Some(v.index(plus, to_sp)),
                    (None, Some(v), None, _) if plus => Some(v.index(true, to_sp)),
                    (None, Some(_), _, _) => Some(Val::Opaque { own: true }),
                    _ => None,
                }
            }
            OpCode::CPUI_INT_AND if ins.len() == 2 => {
                let aligned = |v: Val, m: i64| match v {
                    Val::Stack { lo, hi, .. } if (-4096..0).contains(&m) && (-m).count_ones() == 1 => {
                        Val::Stack { lo: bound(lo, m + 1), hi, own: true }
                    }
                    _ => Val::Opaque { own: true },
                };
                match (vals[0], vals[1], konst(&ins[0]), konst(&ins[1])) {
                    (Some(v), _, _, Some(m)) | (_, Some(v), Some(m), _) => Some(aligned(v, m)),
                    _ if any_stack => Some(Val::Opaque { own: true }),
                    _ => None,
                }
            }
            OpCode::CPUI_LOAD => {
                if let (Some(o), Some(a)) = (out.as_ref(), ins.get(1)) {
                    if unique(o) && kind(a) == Some(spacetype::IPTR_CONSTANT) {
                        loaded_from.push((o.offset, a.offset));
                    }
                }
                let size = out.as_ref().map_or(4, |o| o.size);
                let at = vals.get(1).copied().flatten();
                touch(at, size, &mut s.arg_end, &mut s.unplaced);
                s.unplaced |= lost_at(1);
                out_lost = track && s.state.2 & slots_in(at, size) != 0;
                None
            }
            OpCode::CPUI_STORE => {
                let size = ins.get(2).map_or(4, |v| v.size);
                let at = vals.get(1).copied().flatten();
                let value = vals.get(2).copied().flatten();
                touch(at, size, &mut s.arg_end, &mut s.unplaced);
                escape(value, &mut s.unplaced);
                if track {
                    s.unplaced |= lost_at(1);
                    let carries = value.is_some() || lost_at(2);
                    match (at, at.and_then(Val::as_exact).and_then(slot_bit).filter(|_| size == 4)) {
                        (_, Some(b)) if carries => s.state.2 |= b,
                        (_, Some(b)) => s.state.2 &= !b,
                        (Some(Val::Stack { .. }), None) => s.unplaced |= carries,
                        _ => s.unplaced |= matches!(value, Some(Val::Stack { .. })) || lost_at(2),
                    }
                }
                None
            }
            OpCode::CPUI_CALLOTHER => {
                if let Some(o) = out.as_ref().filter(|o| unique(o)) {
                    trapped.push(o.offset);
                }
                None
            }
            OpCode::CPUI_RETURN => {
                if ins.first().is_some_and(|d| kind(d) != Some(spacetype::IPTR_CONSTANT)) {
                    let at = s.state.0[SP].unwrap_or(Val::Opaque { own: true });
                    s.ret = Some((start_sp, at));
                    s.ret_pop = Some(sp_adds);
                    s.falls_through = false;
                }
                None
            }
            OpCode::CPUI_BRANCH if ins.first().is_some_and(|d| kind(d) != Some(spacetype::IPTR_CONSTANT)) => {
                s.targets.push(ins[0].offset);
                s.falls_through = false;
                None
            }
            OpCode::CPUI_CBRANCH if ins.first().is_some_and(|d| kind(d) != Some(spacetype::IPTR_CONSTANT)) => {
                s.targets.push(ins[0].offset);
                None
            }
            OpCode::CPUI_BRANCHIND => {
                let trap = ins.first().is_some_and(|d| unique(d) && trapped.contains(&d.offset));
                s.unknown |= !trap;
                s.falls_through = false;
                None
            }
            OpCode::CPUI_CALL => {
                if let Some(d) = ins.first().filter(|d| kind(d) != Some(spacetype::IPTR_CONSTANT)) {
                    s.call = Some(Call::Direct(d.offset));
                }
                None
            }
            OpCode::CPUI_CALLIND => {
                let target = ins.first();
                let interrupt = target.is_some_and(|d| unique(d) && trapped.contains(&d.offset));
                if !interrupt {
                    let slot = target
                        .filter(|d| unique(d))
                        .and_then(|d| loaded_from.iter().rev().find(|(o, _)| *o == d.offset).map(|(_, a)| *a));
                    s.call = Some(Call::Indirect(slot));
                }
                None
            }
            _ if any_stack => Some(Val::Opaque { own: true }),
            _ => None,
        };
        let Some(o) = out else { continue };
        if let Some(r) = regs.exact(o) {
            s.writes |= saved_bit(r);
            s.state.0[r] = if r == SP { Some(result.unwrap_or(Val::Opaque { own: true })) } else { result };
            if out_lost && r != SP {
                s.state.1 |= 1 << r;
            } else {
                s.state.1 &= !(1 << r);
            }
        } else if unique(o) {
            lost_temps.retain(|&t| t != o.offset);
            if out_lost {
                lost_temps.push(o.offset);
            }
            temps.push((o.offset, result));
        } else {
            let partial: Vec<usize> = regs.partial(o).collect();
            if track && (result.is_some() || out_lost) && (o.size >= 4 || !partial.is_empty()) {
                s.unplaced = true;
            }
            for r in partial {
                s.writes |= saved_bit(r);
                s.state.0[r] = if s.state.0[r].is_some() || r == SP { Some(Val::Opaque { own: true }) } else { None };
                if out_lost && r != SP {
                    s.state.1 |= 1 << r;
                }
            }
        }
    }
    s
}

fn registers(arch: &Architecture) -> Option<Regs> {
    let t = arch.translate();
    let mut offs = [0u64; 8];
    let mut space = None;
    for (i, name) in GPRS.iter().enumerate() {
        let v = t.get_register_data(name).ok()?;
        let idx = v.space.as_ref()?.get_index();
        if v.size != 4 || space.is_some_and(|s| s != idx) {
            return None;
        }
        space = Some(idx);
        offs[i] = v.offset;
    }
    Some(Regs { space: space?, offs })
}

/// The stack pointer after a call the walk cannot read: above it by at least
/// nothing (a stack-moving helper may have lowered it) and at most back at the
/// return address, unless the caller had already climbed past it.
fn after_unread_call(sp: Val) -> Val {
    match sp {
        Val::Stack { hi, own, .. } => Val::Stack { lo: -INF, hi: bound(hi, 4).max(0), own },
        Val::CallerFrame => Val::Opaque { own: true },
        v => v,
    }
}

fn clobber(state: &mut State) {
    for r in CALLER_SAVED {
        state.0[r] = None;
        state.1 &= !(1 << r);
    }
}

/// Walk the function at `entry`, with the depth its nested calls needed;
/// `None` when it cannot be decoded at all.
fn walk(arch: &Architecture, entry: &Address, depth: u8) -> Option<(Summary, u8)> {
    let space: Rc<AddrSpace> = entry.get_space()?.clone();
    let regs = registers(arch)?;
    let mut seen: HashMap<uintb, State> = HashMap::default();
    let mut work: Vec<(uintb, State)> = vec![(entry.get_offset(), State::at_entry())];
    let (mut unknown, mut unbalanced, mut incomplete, mut unplaced) = (false, false, false, false);
    let mut arg_end: i64 = 4;
    let mut ret_pops: Option<int4> = None;
    let mut need = 0u8;
    let mut writes = 0u8;
    let mut extrapop: Option<int4> = None;
    let mut returns = false;
    let mut returns_stack = false;
    let track = arch.kuna_calleeargs;
    let mut steps = 0usize;
    while let Some((at, incoming)) = work.pop() {
        let state = match seen.get(&at) {
            Some(old) => {
                let joined = old.join(&incoming);
                if joined == *old {
                    continue;
                }
                joined
            }
            None => incoming,
        };
        seen.insert(at, state);
        steps += 1;
        if seen.len() > BUDGET || steps > STEPS {
            incomplete = true;
            break;
        }
        let Some(insn) = decode(arch, &space, at) else {
            incomplete = true;
            continue;
        };
        let (ops, len) = (&insn.0, &insn.1);
        let mut s = step_with(ops, &regs, state, track);
        writes |= s.writes;
        incomplete |= s.unknown;
        unplaced |= s.unplaced;
        arg_end = arg_end.max(s.arg_end);
        if s.ret.is_some() || s.call.is_some() {
            for (r, v) in s.state.0.iter().enumerate() {
                if r != SP {
                    escape(*v, &mut unplaced);
                }
            }
        }
        if let Some(p) = s.ret_pop.filter(|p| (4..=MAX_EXTRAPOP as i64).contains(p)) {
            ret_pops = Some(ret_pops.map_or(p as int4, |q| q.max(p as int4)));
        }
        if s.ret.is_none() && s.state.0[SP].and_then(Val::as_exact).is_some_and(|sp| sp > 0) {
            unbalanced = true;
        }
        if let Some((pre, post)) = s.ret {
            returns = true;
            returns_stack |= CALLER_SAVED.iter().any(|&r| s.state.0[r].is_some() || s.state.lost(r));
            match (pre.as_exact(), post.as_exact()) {
                (Some(0), Some(ep)) => {
                    let ep = ep as int4;
                    if !(4..=MAX_EXTRAPOP).contains(&ep) || ep % 4 != 0 {
                        unbalanced = true;
                    } else if extrapop.is_some_and(|q| q != ep) {
                        unknown = true;
                    } else {
                        extrapop = Some(ep);
                    }
                }
                (Some(_), _) => unbalanced = true,
                _ if pre.own() => unbalanced = true,
                _ => unknown = true,
            }
        }
        if let Some(call) = s.call {
            let effect = match call {
                Call::Direct(t) if depth == 0 => {
                    let target = Address::new(space.clone(), t);
                    if crate::p4_calls::kuna_calleenoreturn::is_no_return(arch, &target) {
                        Summary { effect: Effect::NoReturn, ..UNKNOWN }
                    } else {
                        need = CUT;
                        UNKNOWN
                    }
                }
                Call::Direct(t) => {
                    let (nested, nested_need) = summary_at(arch, &Address::new(space.clone(), t), depth - 1);
                    need = need.max(nested_need.saturating_add(1));
                    nested
                }
                Call::Indirect(Some(slot)) => slot_effect(arch, &Address::new(space.clone(), slot)),
                Call::Indirect(None) => UNKNOWN,
            };
            let sp = s.state.0[SP].unwrap_or(Val::Opaque { own: true });
            match effect.arg_bytes {
                Some(0) => {}
                Some(n) => touch(Some(sp.shift(4)), n, &mut arg_end, &mut unplaced),
                None => unplaced |= effect.effect == Effect::Unbalanced,
            }
            clobber(&mut s.state);
            if track && effect.returns_stack {
                s.state.1 |= CALLER_SAVED.iter().fold(0, |m, &r| m | 1 << r);
            }
            match effect.effect {
                Effect::Balanced(ep) => s.state.0[SP] = Some(sp.shift(ep as i64)),
                Effect::NoReturn => s.falls_through = false,
                Effect::Unknown => s.state.0[SP] = Some(after_unread_call(sp)),
                Effect::Unbalanced => {
                    s.state.0[SP] = Some(Val::Opaque { own: false });
                    writes |= effect.writes;
                    for (i, &r) in CALLEE_SAVED.iter().enumerate() {
                        if effect.writes & (1 << i) != 0 {
                            s.state.0[r] = Some(Val::Opaque { own: false });
                        }
                    }
                }
            }
        }
        if s.falls_through {
            work.push((at.wrapping_add(*len), s.state));
        }
        work.extend(s.targets.iter().map(|&t| (t, s.state)));
    }
    let arg_bytes = (!incomplete && !unplaced).then(|| (((arg_end + 3) & !3) - 4) as u32);
    unknown |= incomplete;
    let effect = if unbalanced {
        Effect::Unbalanced
    } else if unknown {
        Effect::Unknown
    } else if let Some(ep) = extrapop {
        Effect::Balanced(ep)
    } else if returns {
        Effect::Unknown
    } else {
        Effect::NoReturn
    };
    Some((Summary { effect, writes, arg_bytes, ret_pops, returns_stack }, need))
}

#[cfg(test)]
mod tests;
