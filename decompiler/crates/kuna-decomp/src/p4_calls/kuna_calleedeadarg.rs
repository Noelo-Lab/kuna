//! (kuna) `calleedeadarg` — drop a recovered call argument the callee's own body
//! proves it never reads.
//!
//! # The symptom
//!
//! One Mach-O arm64 crackme, decompiled by kuna, contradicts itself inside a
//! single file:
//!
//! ```text
//! int _secret_function(void);          // the header kuna itself emits
//! ...
//!   v3 = scanf("%d",&v2);
//!   if (v2 == 0x539)
//!     _secret_function(v3);            // called with an argument anyway
//! ```
//!
//! That is not C: the declaration and the call cannot both be right, so the
//! export does not recompile and an agent reading the output cannot tell whether
//! the callee consumes the passed value. The disassembly settles it — the callee
//! is `stp x29,x30,[sp,#-0x10]!; mov x29,sp; adrp x0,…; add x0,x0,#0xeec; bl
//! printf; …`, which OVERWRITES `x0` before it ever reads it.
//!
//! # Why the call site invents the argument
//!
//! `ActionActiveParam` recovers an unlocked callee's argument list from the
//! CALLER's data flow alone (`FuncCallSpecs::checkInputTrialUse`, `fspec.cc:5592`):
//! a trial is *active* when the ABI's argument register holds a value the caller
//! wrote and does not otherwise use. Here `w0` holds `scanf`'s return value,
//! nothing else consumes it, and `AncestorRealistic` returns `pop_solid` for a
//! Varnode defined by a CALL — so the trial is admitted. Every ingredient of that
//! decision is on the caller's side of the call, and on the caller's side the
//! evidence really is ambiguous: a live argument register at a call to an
//! unprototyped callee is exactly what a real argument looks like. The
//! `entrymainproto` record already names this as kuna's standing behaviour
//! ("at any unprototyped callee reached with a live argument register").
//!
//! `calleearity`/`calleearityfwd` cannot help: they reconcile a call against a
//! SIBLING call to the same callee in the same function, they are only ever
//! additive, and here there is exactly one call site.
//!
//! # The evidence this pass adds
//!
//! The callee's own body, read directly. [`probe_callee_entry_dead`] decodes the
//! callee starting at its entry and answers one question per register range:
//! *does every path from the entry WRITE these bytes before reading them?* If it
//! does, the register is dead on entry and cannot be carrying a parameter.
//!
//! That evidence alone is not enough to act on, and the second half is what keeps
//! this pass off real argument lists. The veto also requires the value in the
//! register to be an earlier call's **leftover result** — not something the
//! caller loaded, computed, or received as its own parameter
//! ([`is_leftover_call_result`]). Two reasons. It is the shape the symptom is
//! actually made of, since only a value the caller never placed can be an
//! argument by coincidence. And a caller that DID place a value there is passing
//! an argument the callee happens to ignore (`int dm_init(bool of_live)` built
//! with `of_live` compiled out is a real one): dropping it would be a judgement
//! about the source, and — worse — dropping a LEADING one punches a hole in the
//! register argument list that `ParamListStandard::fillinMap`'s positional rules
//! read as the end of the list, taking every later argument with it. Measured on
//! u-boot ARM, requiring only the callee evidence emptied the argument lists of
//! `do_bootm(ctx->cmdtp,0,v2,bootm_argv)` and
//! `ubifs_scan_a_node(a0,v9,v11,a1,v1,1)` exactly that way.
//!
//! With both conditions met the trial is scored `no-use` like any other
//! definitely-unused trial — the CALL input is freed and the argument
//! disappears.
//!
//! This is the same shape of evidence as the callee-body probe `rustabi` takes
//! for the call OUTPUT seam ([`crate::kuna_rustabi::probe_callee_return_writes`]),
//! and it is taken the same way: from the driver, right after the flow build,
//! because the per-function `ArchContext` the pipeline runs against carries the
//! load image but no translator. Results are cached per callee entry on the
//! `Architecture`, so each distinct body is decoded once per run.
//!
//! # What the walk can prove, and what it refuses to
//!
//! The claim is one-sided: `dead` means *no counter-example was found on a walk
//! that covered every path*, and everything the walk cannot see makes it decline.
//!
//! * Each path carries the set of register bytes already written on it. A read of
//!   a byte not in that set is a **read-before-write** and vetoes those bytes for
//!   the whole callee.
//! * Every path ENDS somewhere — at a `RETURN`, at a nested
//!   `CALL`/`CALLIND`/`CALLOTHER`, at an unresolved `BRANCHIND`, at a `LOAD`/
//!   `STORE` naming the register space (an indexed register file), or at an
//!   undecodable instruction — and the register must already be WRITTEN when it
//!   does. Past that point the walk is not reading the code that runs, so only
//!   bytes already written stay provable. That is what lets a body which
//!   overwrites `x0` and then calls `printf` still prove `x0` dead, while a body
//!   whose first act is a call proves nothing.
//! * Requiring the write is not the same as requiring the absence of a read, and
//!   the difference is load-bearing. A callee whose entire body is `ret` reads
//!   nothing at all, so a "never read" rule would declare EVERY register dead
//!   there and delete the arguments of every stub, thunk and placeholder in the
//!   image — which is exactly what the `stackreturn` datatest (three callees
//!   that are one `c3` byte each) catches. The claim this pass makes is the
//!   positive one: the callee demonstrably CLOBBERS the register, so the value
//!   the caller left there cannot be reaching it.
//! * An instruction whose p-code contains an internal (constant-space) branch is
//!   scored against the set it was ENTERED with and credits none of its writes:
//!   a conditionally-executed write must not hide a later read.
//! * A walk that records NO terminator at all proves nothing either, and this
//!   one is easy to read the wrong way round: the "written before every
//!   terminator" test is a conjunction, so over an empty list it holds for every
//!   register at once. Every path closing back onto an already-visited address
//!   is what a body that is one endless loop does — and what a PE import's IAT
//!   slot does when its pointer bytes are decoded as instructions, which is
//!   where the argument lists of `CloseHandle` and `Process32NextW` went.
//! * The instruction budget, a too-large written set or a too-large cut list
//!   abandon the whole summary, which then proves nothing at all.
//!
//! Only the `register` space is answered. A `ram`-space (global) trial would need
//! the walk to model memory, which it does not, so those keep today's answer.
//!
//! Default-**on**: this only ever REMOVES an argument, and only against a decoded
//! body that contradicts it. Flip `off` to restore the pre-option rendering.

use std::collections::{BTreeSet, HashMap};
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::error::KunaResult;
use kuna_base::marshal::ElementId;
use kuna_base::space::spacetype;
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;
use kuna_num::pcoderaw::VarnodeData;

use crate::context::VarnodeId;
use crate::funcdata::Funcdata;
use crate::p0_knowledge::options::on_or_off;

/// Marshaling element `<calleedeadarg>` (kuna 4000+ range; 4139 was the previous
/// high-water mark).
pub const ELEM_CALLEEDEADARG: ElementId = ElementId::new("calleedeadarg", 4140);

/// (kuna) Drop a provably-unread call argument: `calleedeadarg on|off`.
pub struct OptionCalleeDeadArg;

impl OptionCalleeDeadArg {
    /// The option name.
    pub const NAME: &'static str = "calleedeadarg";

    /// Resolve the flag and its confirmation message; the caller writes it into
    /// `Architecture::callee_dead_arg`.
    pub fn apply(&self, p1: &str) -> KunaResult<(bool, String)> {
        let val = on_or_off(p1)?;
        let prop = if val { "on" } else { "off" };
        Ok((val, format!("Callee-body dead-argument veto turned {prop}")))
    }
}

/// How many machine instructions one callee probe decodes before abandoning the
/// summary. Every path normally ends at the callee's first call or return, so
/// this bound is reached only by a branchy argument-free prologue — a body whose
/// answer would be "may read" almost surely.
const MAX_PROBE_INSTRUCTIONS: u32 = 192;

/// How many distinct register bytes one path may accumulate before the summary
/// declares itself incomplete.
const MAX_WRITTEN_BYTES: usize = 4096;

/// How many cuts one summary keeps before declaring itself incomplete.
const MAX_CUTS: usize = 64;

/// A set of individual register-space bytes, keyed by `(space index, offset)`.
type ByteSet = BTreeSet<(int4, u64)>;

/// What a bounded decode of a callee body proves about its entry liveness.
///
/// Consumed through [`Self::proves_dead`], which answers `false` for everything
/// the walk could not cover, so an incomplete summary never vetoes an argument.
#[derive(Clone, Debug, Default)]
pub struct CalleeEntryDead {
    /// The `register` space's manager index — the only space this summary
    /// answers for. `-1` when the image has no register space.
    reg_idx: int4,
    /// Register byte ranges some path READS before writing — `(space, offset, size)`.
    reads: Vec<(int4, u64, int4)>,
    /// For every path terminator — a `RETURN` as much as a nested call or an
    /// unresolved branch — the register bytes already written on the way to it.
    /// A range is only dead if it is fully written before EVERY one of them, and
    /// an EMPTY list is no evidence at all rather than every range dead (see
    /// [`Self::proves_dead`]).
    cuts: Vec<ByteSet>,
    /// Did the walk cover every path with nothing abandoned?
    complete: bool,
}

impl CalleeEntryDead {
    /// Does this summary *prove* the callee never reads any byte of
    /// `[addr, addr+size)` before writing it?
    pub fn proves_dead(&self, addr: &Address, size: int4) -> bool {
        if !self.complete || size <= 0 {
            return false;
        }
        // No path terminator was recorded, so no path was seen ENDING with the
        // register written and the conjunction over `cuts` below would hold
        // vacuously — for every register at once.  A walk reaches here when the
        // revisit rule closes every path back onto an already-visited address,
        // which is what a body that is one endless loop does, and what data
        // decoded as instructions usually does.
        if self.cuts.is_empty() {
            return false;
        }
        let Some(sp) = addr.get_space() else { return false };
        if self.reg_idx < 0 || sp.get_index() != self.reg_idx {
            return false;
        }
        let (idx, off) = (self.reg_idx, addr.get_offset());
        let end = off.wrapping_add(size as u64);
        if end < off {
            return false;
        }
        if self
            .reads
            .iter()
            .any(|&(ridx, roff, rsz)| ridx == idx && roff < end && off < roff + rsz as u64)
        {
            return false;
        }
        self.cuts.iter().all(|c| (off..end).all(|b| c.contains(&(idx, b))))
    }

    /// Did the walk complete? (Diagnostics and tests.)
    pub fn is_complete(&self) -> bool {
        self.complete
    }
}

/// One decoded p-code op, kept in emission order.
struct RawOp {
    opc: OpCode,
    out: Option<VarnodeData>,
    ins: Vec<VarnodeData>,
}

/// Recording sink for [`probe_callee_entry_dead`]: the raw p-code of ONE machine
/// instruction, in order, plus whether that instruction branches inside itself.
#[derive(Default)]
struct EntryEmit {
    ops: Vec<RawOp>,
    internal_flow: bool,
}

impl kuna_sleigh::translate::PcodeEmit for EntryEmit {
    fn dump(
        &mut self,
        _addr: &Address,
        opc: OpCode,
        outvar: Option<&VarnodeData>,
        vars: &[VarnodeData],
    ) {
        if matches!(opc, OpCode::CPUI_BRANCH | OpCode::CPUI_CBRANCH) {
            if let Some(sp) = vars.first().and_then(|v| v.space.as_ref()) {
                if sp.get_type() == spacetype::IPTR_CONSTANT {
                    self.internal_flow = true;
                }
            }
        }
        self.ops.push(RawOp { opc, out: outvar.cloned(), ins: vars.to_vec() });
    }
}

/// A pending path: where to resume and what is already written on the way there.
struct Frame {
    at: Address,
    written: ByteSet,
}

/// Decode a callee body and report which register bytes it is *proven* never to
/// read before writing (C++ has no analogue).
///
/// The walk starts at `entry`, follows fall-through and resolved machine branch
/// targets, and ends a path at a `RETURN`, a nested call, an unresolved indirect
/// branch, a register-file `LOAD`/`STORE`, or an undecodable instruction —
/// recording, at each of those, the bytes already written on the way there. Only
/// bytes written before EVERY such terminator stay provable. It abandons the
/// whole summary — proving nothing — on the instruction budget or a runaway
/// written/cut set.
pub fn probe_callee_entry_dead<T: kuna_sleigh::translate::Translate + ?Sized>(
    tr: &T,
    entry: &Address,
    reg_idx: int4,
) -> CalleeEntryDead {
    let mut res =
        CalleeEntryDead { reg_idx, reads: Vec::new(), cuts: Vec::new(), complete: true };
    let Some(entry_space) = entry.get_space() else {
        res.complete = false;
        return res;
    };
    if entry_space.get_type() != spacetype::IPTR_PROCESSOR || reg_idx < 0 {
        res.complete = false;
        return res;
    }
    let mut visited: HashMap<(int4, u64), ByteSet> = HashMap::new();
    let mut todo: Vec<Frame> = vec![Frame { at: entry.clone(), written: ByteSet::new() }];
    let mut budget = MAX_PROBE_INSTRUCTIONS;
    while let Some(frame) = todo.pop() {
        let Some(sp) = frame.at.get_space() else {
            res.complete = false;
            break;
        };
        let key = (sp.get_index(), frame.at.get_offset());
        // A revisit whose incoming written set is a SUPERSET of one already
        // processed adds nothing: the earlier, more conservative pass recorded
        // at least every read-before-write this one would.
        if let Some(prev) = visited.get(&key) {
            if prev.is_subset(&frame.written) {
                continue;
            }
        }
        visited.insert(key, frame.written.clone());
        if budget == 0 || res.cuts.len() > MAX_CUTS || frame.written.len() > MAX_WRITTEN_BYTES {
            res.complete = false;
            break;
        }
        budget -= 1;
        let mut emit = EntryEmit::default();
        let len = match tr.one_instruction(&mut emit, &frame.at) {
            Ok(n) if n > 0 => n,
            // Undecodable: the path ends where the walk cannot see.
            _ => {
                res.cuts.push(frame.written);
                continue;
            }
        };
        match step_instruction(&mut res, &emit, frame.written, &frame.at, len) {
            Some(next) => todo.extend(next),
            None => break,
        }
    }
    if !res.complete {
        res.reads.clear();
        res.cuts.clear();
    }
    res
}

/// Run one decoded instruction's p-code against the incoming written set.
///
/// Records every read-before-write into `res`, and returns the frames the walk
/// should continue with (empty when the path ended). `None` means the summary
/// was abandoned.
fn step_instruction(
    res: &mut CalleeEntryDead,
    emit: &EntryEmit,
    written: ByteSet,
    at: &Address,
    len: i32,
) -> Option<Vec<Frame>> {
    let incoming = written.clone();
    let mut cur = written;
    let mut targets: Vec<Address> = Vec::new();
    let mut ends_flow = false;
    for op in &emit.ops {
        // Reads. An instruction that branches inside itself is scored against
        // the set it was entered with, since a conditionally-executed write
        // earlier in the same instruction may not have run.
        let base = if emit.internal_flow { &incoming } else { &cur };
        for (i, v) in op.ins.iter().enumerate() {
            if skip_input(op.opc, i) {
                continue;
            }
            let Some(sp) = v.space.as_ref() else { continue };
            if sp.get_index() != res.reg_idx {
                continue;
            }
            let (idx, off, sz) = (res.reg_idx, v.offset, v.size as int4);
            if (off..off + v.size as u64).any(|b| !base.contains(&(idx, b))) {
                res.reads.push((idx, off, sz));
            }
        }
        match op.opc {
            // Control transfer into code this walk is not reading.
            OpCode::CPUI_CALL | OpCode::CPUI_CALLIND | OpCode::CPUI_CALLOTHER
            | OpCode::CPUI_BRANCHIND => {
                res.cuts.push(cur);
                return Some(Vec::new());
            }
            // A RETURN is a path terminator like any other: the register has to
            // be written BEFORE it, or the body has shown nothing.  A callee
            // whose whole body is `ret` reads nothing, and treating that as
            // proof would delete the arguments of every stub and thunk.
            OpCode::CPUI_RETURN => {
                res.cuts.push(cur);
                return Some(Vec::new());
            }
            // The `<spaceid>` operand's offset IS the space-manager index. An
            // access to the register space through LOAD/STORE is an indexed
            // register file: the walk cannot say which register it names.
            OpCode::CPUI_LOAD | OpCode::CPUI_STORE => {
                if op.ins.first().map(|v| v.offset as int4) == Some(res.reg_idx) {
                    res.cuts.push(cur);
                    return Some(Vec::new());
                }
            }
            OpCode::CPUI_BRANCH | OpCode::CPUI_CBRANCH => {
                match op.ins.first().and_then(|v| v.space.as_ref()) {
                    // p-code-relative: stays inside this instruction, and the
                    // whole instruction is being read anyway.
                    Some(sp) if sp.get_type() == spacetype::IPTR_CONSTANT => {}
                    Some(sp) => {
                        targets.push(Address::new(Rc::clone(sp), op.ins[0].offset));
                        if op.opc == OpCode::CPUI_BRANCH {
                            ends_flow = true;
                        }
                    }
                    None => {
                        res.cuts.push(cur);
                        return Some(Vec::new());
                    }
                }
            }
            _ => {}
        }
        // Writes, credited only for an instruction with no internal branching.
        if !emit.internal_flow {
            if let Some(o) = &op.out {
                if let Some(sp) = o.space.as_ref() {
                    if sp.get_index() == res.reg_idx {
                        let idx = res.reg_idx;
                        for b in o.offset..o.offset + o.size as u64 {
                            cur.insert((idx, b));
                        }
                        if cur.len() > MAX_WRITTEN_BYTES {
                            res.complete = false;
                            return None;
                        }
                    }
                }
            }
        }
    }
    let mut next: Vec<Frame> =
        targets.into_iter().map(|t| Frame { at: t, written: cur.clone() }).collect();
    if !ends_flow {
        next.push(Frame { at: at + len as i64, written: cur });
    }
    Some(next)
}

/// Is input `i` of `opc` an address/annotation operand rather than a data read?
fn skip_input(opc: OpCode, i: usize) -> bool {
    match opc {
        // Slot 0 is the branch/call destination.
        OpCode::CPUI_BRANCH | OpCode::CPUI_CBRANCH | OpCode::CPUI_BRANCHIND
        | OpCode::CPUI_CALL | OpCode::CPUI_CALLIND | OpCode::CPUI_RETURN => i == 0,
        // Slot 0 is the userop id; slot 0 of LOAD/STORE is the space id.
        OpCode::CPUI_CALLOTHER | OpCode::CPUI_LOAD | OpCode::CPUI_STORE => i == 0,
        _ => false,
    }
}

/// Take the callee-body entry-liveness probe for every direct call in `data`,
/// before the action pipeline reaches the trial-scoring seam that needs it.
///
/// Called from the driver for the same reason
/// [`crate::kuna_rustabi::seed_callee_return_writes`] is: the per-function
/// `ArchContext` the pipeline runs against carries the load image but no
/// translator, so the probe cannot be taken lazily at the seam. Results are
/// cached on the `Architecture`, so each distinct callee body is decoded once per
/// run. A no-op with the option off.
pub fn seed_callee_entry_dead(
    arch: &mut crate::architecture::Architecture,
    data: &mut Funcdata,
) {
    if !arch.callee_dead_arg {
        return;
    }
    // The veto needs an EARLIER call to have left the value in the register, so
    // a function with fewer than two calls can never produce one — and probing
    // its callees would be pure cost.  This matters most in ghidra mode, where a
    // decode is a round trip to the host.
    if data.num_calls() < 2 {
        return;
    }
    let reg_idx =
        arch.manage().get_space_by_name("register").map(|s| s.get_index()).unwrap_or(-1);
    if reg_idx < 0 {
        return;
    }
    let mut entries: Vec<Address> = Vec::new();
    for i in 0..data.num_calls() {
        let e = data.get_call_specs(i).get_entry_address().clone();
        if e.is_invalid() {
            continue;
        }
        entries.push(e);
    }
    for e in entries {
        let Some(sp) = e.get_space() else { continue };
        let key = (sp.get_index(), e.get_offset());
        if !arch.kuna_callee_dead_cache.contains_key(&key) {
            let probed = probe_callee_entry_dead(arch.translate(), &e, reg_idx);
            arch.kuna_callee_dead_cache.insert(key, Rc::new(probed));
        }
        if let Some(d) = arch.kuna_callee_dead_cache.get(&key) {
            data.kuna_set_callee_entry_dead(&e, Rc::clone(d));
        }
    }
}

/// How deep the leftover-return-value test walks through value-preserving ops
/// before giving up. The chain between the two calls is short but not direct:
/// on the AArch64 witness it is `INDIRECT -> INDIRECT -> INT_ZEXT -> SUBPIECE ->
/// PIECE -> INDIRECT(creation)`, six links of register-width bookkeeping around
/// the call that produced the value.
const MAX_LEFTOVER_DEPTH: u32 = 12;

/// Is `vn` a value the caller never computed *for this call* — the leftover
/// output of an EARLIER call still sitting in the register?
///
/// True only when EVERY chain back from `vn` bottoms out at a call: a `CALL`/
/// `CALLIND` output, or the INDIRECT creation `guard_calls` plants for each
/// register a call may clobber. Value-preserving and width-adjusting links
/// (`COPY`, `SUBPIECE`, `PIECE`, `INT_ZEXT`, `INT_SEXT`, a non-creation
/// `INDIRECT` standing for "survived across a call") are followed through, and
/// a `PIECE` or `MULTIEQUAL` has to have all of its inputs qualify. Anything
/// else — a constant, a `LOAD`, arithmetic, or a Varnode that is a function
/// input — answers `false`, because the caller put it there on purpose.
fn is_leftover_call_result(data: &Funcdata, vn: VarnodeId, depth: u32) -> bool {
    if depth >= MAX_LEFTOVER_DEPTH {
        return false;
    }
    let Some(v) = data.vbank().get(vn) else { return false };
    if !v.is_written() {
        return false;
    }
    let Some(def) = v.get_def() else { return false };
    let Some(op) = data.obank().get(def) else { return false };
    let all_inputs = |n: int4| -> bool {
        (0..n).all(|i| match data.obank().get(def).and_then(|o| o.get_in(i)) {
            Some(x) => is_leftover_call_result(data, x, depth + 1),
            None => false,
        })
    };
    match op.code() {
        OpCode::CPUI_CALL | OpCode::CPUI_CALLIND => true,
        OpCode::CPUI_INDIRECT if op.is_indirect_creation() => true,
        OpCode::CPUI_COPY
        | OpCode::CPUI_SUBPIECE
        | OpCode::CPUI_INT_ZEXT
        | OpCode::CPUI_INT_SEXT
        | OpCode::CPUI_INDIRECT => match op.get_in(0) {
            Some(next) => is_leftover_call_result(data, next, depth + 1),
            None => false,
        },
        OpCode::CPUI_PIECE => all_inputs(2),
        OpCode::CPUI_MULTIEQUAL => all_inputs(op.num_input()),
        _ => false,
    }
}

/// Does the callee's own body prove this register trial is not an argument?
///
/// The `checkInputTrialUse` register arm asks this before scoring a trial. Two
/// independent things have to hold, and the pass is only as safe as their
/// conjunction:
///
/// 1. the value in the register is an earlier call's leftover result, not
///    something the caller placed there ([`is_leftover_call_result`]); and
/// 2. the callee overwrites that register on every path from its entry, before
///    ever reading it ([`CalleeEntryDead::proves_dead`]).
///
/// Answers `false` with the option off, for a non-register trial, for an
/// indirect call, and for every callee the probe could not fully cover.
pub fn trial_is_dead_in_callee(
    data: &Funcdata,
    call_idx: int4,
    slot: int4,
    trial_addr: &Address,
    trial_size: int4,
) -> bool {
    if !data.get_arch().callee_dead_arg {
        return false;
    }
    let entry = data.get_call_specs(call_idx).get_entry_address();
    if entry.is_invalid() {
        return false;
    }
    let dead = match data.kuna_callee_entry_dead(entry) {
        Some(d) => d.proves_dead(trial_addr, trial_size),
        None => false,
    };
    if !dead {
        return false;
    }
    let op = data.get_call_specs(call_idx).get_op();
    match data.obank().get(op).and_then(|o| o.get_in(slot)) {
        Some(vn) => is_leftover_call_result(data, vn, 0),
        None => false,
    }
}

#[cfg(test)]
#[path = "kuna_calleedeadarg/tests.rs"]
mod tests;
