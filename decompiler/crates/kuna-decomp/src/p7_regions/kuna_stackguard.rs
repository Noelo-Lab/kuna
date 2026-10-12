//! Port of `decompiler/cpp/kuna_stackguard.{cc,hh}` — strip the glibc
//! `-fstack-protector` canary epilogue (port of angr's StackCanarySimplifier),
//! kuna option-gated, default on since DIV-14 (per-test opt-outs keep the corpus byte-identical).
//!
//! ## What it strips
//!
//! A function compiled with `-fstack-protector` saves a canary at entry
//! (`canary = *(fs:0x28)`) and, at every exit, reloads `fs:0x28` and compares:
//!
//! ```text
//!   if (canary_slot != *(fs:0x28))
//!       __stack_chk_fail();          // no-return
//!   return v;
//! ```
//!
//! This canary check is a single SHARED return point, so a deep return cannot
//! `return` directly — Ghidra's structurer emits a `goto` to the shared tail.
//! Ghidra keeps the check verbatim; angr's StackCanarySimplifier strips it,
//! after which the tail is a bare `return v` that `ActionReturnSplit`
//! duplicates into each predecessor, eliminating the goto and the
//! `__stack_chk_fail` noise.
//!
//! The ENTRY-side canary init (`slot = *(fs:0x28)`) goes with the check
//! (GH-183): angr pops exactly that statement
//! (`first_block_copy.statements.pop(stmt_idx)`).  kuna realizes the pop as a
//! liveness release ([`collect_canary_slots`]/[`release_canary_slots`]) — the
//! init writes an addrtied stack varnode whose directwrite-preserved
//! `addrforce` is what kept `ActionDeadCode` from collecting it; clearing it
//! (plus `ScopeLocal::markNotMapped` on the slot so no adjacent local absorbs
//! the freed bytes) lets the following dead-code pass delete the store, the
//! `fs:0x28` LOAD and the TLS-base input through its ordinary consume
//! fixpoint.  A slot version still feeding a live reader (a second,
//! not-yet-stripped check) is therefore never removed.
//!
//! ## Detection is purely structural
//!
//! Detection does not depend on recovered callee names. The canary is pinned
//! by a CBRANCH whose `INT_EQUAL`/`INT_NOTEQUAL` boolean compares two
//! values that BOTH derive from a LOAD of `<base> + 0x28` (the saved canary slot
//! vs. a fresh reload of the x86-64 glibc TLS canary at `fs:0x28`).  The
//! corrupted-canary branch (which must contain the no-return handler CALL) is
//! removed with the stock in-place primitive `Funcdata::removeBranch`
//! (CBRANCH → fall-through, MULTIEQUALs patched), then
//! `removeUnreachableBlocks` collects the orphaned handler block.
//!
//! ## The option
//!
//! The decision is a P0 assertion: `option stackguard on|off` (default `on`).
//! Setting it `off` retains the upstream canary check. The live flag is
//! [`Architecture::strip_stack_guard`](crate::architecture::Architecture).
//!
//! ## The CFG surgery
//!
//! [`ActionStripStackGuard::apply`] ports the canary *detection and victim-edge
//! selection* faithfully, then performs the edit with the block-graph structuring
//! primitives `Funcdata::removeBranch` (CBRANCH → fall-through, MULTIEQUALs
//! patched) and `Funcdata::removeUnreachableBlocks` (collect the orphaned
//! `__stack_chk_fail` handler) — the same primitives `ActionDeterminedBranch`/
//! `ActionUnreachable` use.  After the strip the shared return tail is a bare
//! `return v` that `ActionReturnSplit` (the next action in the `returnsplit`
//! group) duplicates into each predecessor, eliminating the goto/label.

use std::collections::BTreeSet;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::action::{Action, ActionBase, ActionContext, ActionGroupList, ApplyResult};
use crate::block::BlockKind;
use crate::funcdata::Funcdata;
use crate::context::{BlockId, OpId, VarnodeId};

/// Does Varnode `ptr` address `<base> + 0x28` (the x86-64 TLS canary slot)?
/// (C++ `ptrIsCanarySlot`, `kuna_stackguard.cc:34`).
///
/// Peels transparent COPY/CAST off the pointer, then requires an `INT_ADD` with
/// a constant `0x28` operand.  (`fs:0x28` is the glibc x86-64 stack-guard
/// offset.)
fn ptr_is_canary_slot(ptr: VarnodeId, data: &Funcdata) -> bool {
    let mut ptr = ptr;
    for _guard in 0..8 {
        let v = match data.vbank().get(ptr) {
            Some(v) => v,
            None => return false,
        };
        if !v.is_written() {
            return false;
        }
        let def = v.get_def().expect("ptrIsCanarySlot: written vn def");
        let dop = data.obank().get(def).expect("ptrIsCanarySlot: def");
        let oc = dop.code();
        if oc == OpCode::CPUI_COPY || oc == OpCode::CPUI_CAST {
            ptr = dop.get_in(0).expect("ptrIsCanarySlot: copy/cast in0");
            continue;
        }
        if oc == OpCode::CPUI_INT_ADD {
            let a = dop.get_in(0).expect("ptrIsCanarySlot: add in0");
            let b = dop.get_in(1).expect("ptrIsCanarySlot: add in1");
            let av = data.vbank().get(a).expect("ptrIsCanarySlot: a");
            let bv = data.vbank().get(b).expect("ptrIsCanarySlot: b");
            if bv.is_constant() && bv.get_offset() == 0x28 {
                return true;
            }
            if av.is_constant() && av.get_offset() == 0x28 {
                return true;
            }
            return false;
        }
        return false;
    }
    false
}

/// Is `ptr` `FS_OFFSET + 0x28`, the canary itself?  [`ptr_is_canary_slot`]
/// accepts any `<base> + 0x28`, so a struct field at that offset passes it;
/// only a LOAD through the segment base may root [`collect_value_slots`].
fn ptr_is_fs_canary(ptr: VarnodeId, data: &Funcdata) -> bool {
    let Some(fs) = data
        .get_arch()
        .manage()
        .register_lookup()
        .and_then(|l| l.probe_register("FS_OFFSET"))
    else {
        return false;
    };
    let Some(fs_space) = fs.space else {
        return false;
    };
    let peel = |mut vn: VarnodeId| {
        for _ in 0..8 {
            let def = data.vbank().get(vn).and_then(|v| v.get_def());
            let Some(dop) = def.and_then(|d| data.obank().get(d)) else {
                break;
            };
            match (dop.code(), dop.get_in(0)) {
                (OpCode::CPUI_COPY | OpCode::CPUI_CAST, Some(in0)) => vn = in0,
                _ => break,
            }
        }
        vn
    };
    let Some(add) = data
        .vbank()
        .get(peel(ptr))
        .and_then(|v| v.get_def())
        .and_then(|d| data.obank().get(d))
        .filter(|d| d.code() == OpCode::CPUI_INT_ADD)
    else {
        return false;
    };
    let is_fs = |vn: VarnodeId| {
        data.vbank()
            .get(peel(vn))
            .is_some_and(|v| Rc::ptr_eq(v.get_space(), &fs_space) && v.get_offset() == fs.offset)
    };
    let is_slot = |vn: VarnodeId| {
        data.vbank().get(vn).is_some_and(|v| v.is_constant() && v.get_offset() == 0x28)
    };
    match (add.get_in(0), add.get_in(1)) {
        (Some(a), Some(b)) => (is_slot(b) && is_fs(a)) || (is_slot(a) && is_fs(b)),
        _ => false,
    }
}

/// Does `vn` ultimately derive from a LOAD of the canary slot?
/// (C++ `derivesFromCanaryLoad`, `kuna_stackguard.cc:60`).
///
/// Peels value-preserving ops (COPY/CAST/zext/sext/INDIRECT/zero-SUBPIECE) and
/// follows every input of a MULTIEQUAL (the saved-canary operand is a phi over
/// all paths, each tracing to the single entry load).  A visited set bounds
/// loops; `depth` bounds the walk.
fn derives_from_canary_load(
    vn: VarnodeId,
    depth: int4,
    seen: &mut BTreeSet<VarnodeId>,
    data: &Funcdata,
) -> bool {
    if depth <= 0 {
        return false;
    }
    if seen.contains(&vn) {
        return false;
    }
    seen.insert(vn);
    let v = match data.vbank().get(vn) {
        Some(v) => v,
        None => return false,
    };
    if !v.is_written() {
        return false;
    }
    let def = v.get_def().expect("derivesFromCanaryLoad: def");
    let dop = data.obank().get(def).expect("derivesFromCanaryLoad: dop");
    let oc = dop.code();
    if oc == OpCode::CPUI_LOAD {
        return ptr_is_canary_slot(dop.get_in(1).expect("derivesFromCanaryLoad: load ptr"), data);
    }
    if oc == OpCode::CPUI_COPY
        || oc == OpCode::CPUI_CAST
        || oc == OpCode::CPUI_INT_ZEXT
        || oc == OpCode::CPUI_INT_SEXT
        || oc == OpCode::CPUI_INDIRECT
    {
        let in0 = dop.get_in(0).expect("derivesFromCanaryLoad: peel in0");
        return derives_from_canary_load(in0, depth - 1, seen, data);
    }
    if oc == OpCode::CPUI_SUBPIECE {
        let in1 = dop.get_in(1).expect("derivesFromCanaryLoad: subpiece in1");
        let in1v = data.vbank().get(in1).expect("derivesFromCanaryLoad: subpiece in1 vn");
        if in1v.is_constant() && in1v.get_offset() == 0 {
            let in0 = dop.get_in(0).expect("derivesFromCanaryLoad: subpiece in0");
            return derives_from_canary_load(in0, depth - 1, seen, data);
        }
    }
    if oc == OpCode::CPUI_MULTIEQUAL {
        let n = dop.num_input();
        for i in 0..n {
            let ini = dop.get_in(i).expect("derivesFromCanaryLoad: multiequal in");
            if derives_from_canary_load(ini, depth - 1, seen, data) {
                return true;
            }
        }
        return false;
    }
    false
}

/// Collect the storage of every addrtied canary-slot version on a proven
/// canary derivation feeding the compare (the walk of
/// [`derives_from_canary_load`], with the slot addresses as output).
///
/// Mirrors the detector's peel set and bounds exactly, but walks *every*
/// MULTIEQUAL input (each proven subchain contributes) and records
/// `(address, size)` for each addrtied varnode whose value provably derives
/// from a LOAD of the canary slot — i.e. the saved-canary stack slot the pass
/// resolved — and each `FS_OFFSET + 0x28` LOAD the chains end in, so the
/// caller can also find the slot from the store side
/// ([`collect_value_slots`]).  Returns whether `vn` itself derives.
fn collect_canary_slots(
    vn: VarnodeId,
    depth: int4,
    seen: &mut BTreeSet<VarnodeId>,
    data: &Funcdata,
    slots: &mut Vec<(Address, int4)>,
    loads: &mut Vec<OpId>,
) -> bool {
    if depth <= 0 {
        return false;
    }
    if seen.contains(&vn) {
        return false;
    }
    seen.insert(vn);
    let (written, def) = match data.vbank().get(vn) {
        Some(v) => (v.is_written(), v.get_def()),
        None => return false,
    };
    if !written {
        return false;
    }
    let def = def.expect("collectCanarySlots: written vn def");
    let (oc, in0, in1, nin) = {
        let dop = data.obank().get(def).expect("collectCanarySlots: def");
        (dop.code(), dop.get_in(0), dop.get_in(1), dop.num_input())
    };
    let derived = match oc {
        OpCode::CPUI_LOAD => {
            let ptr = in1.expect("collectCanarySlots: load ptr");
            let canary = ptr_is_canary_slot(ptr, data);
            if canary && !loads.contains(&def) && ptr_is_fs_canary(ptr, data) {
                loads.push(def);
            }
            canary
        }
        OpCode::CPUI_COPY
        | OpCode::CPUI_CAST
        | OpCode::CPUI_INT_ZEXT
        | OpCode::CPUI_INT_SEXT
        | OpCode::CPUI_INDIRECT => collect_canary_slots(
            in0.expect("collectCanarySlots: peel in0"),
            depth - 1,
            seen,
            data,
            slots,
            loads,
        ),
        OpCode::CPUI_SUBPIECE => {
            let in1 = in1.expect("collectCanarySlots: subpiece in1");
            let zero = data
                .vbank()
                .get(in1)
                .map(|v| v.is_constant() && v.get_offset() == 0)
                .unwrap_or(false);
            zero && collect_canary_slots(
                in0.expect("collectCanarySlots: subpiece in0"),
                depth - 1,
                seen,
                data,
                slots,
                loads,
            )
        }
        OpCode::CPUI_MULTIEQUAL => {
            let mut any = false;
            for i in 0..nin {
                let ini = data
                    .obank()
                    .get(def)
                    .expect("collectCanarySlots: multiequal")
                    .get_in(i)
                    .expect("collectCanarySlots: multiequal in");
                if collect_canary_slots(ini, depth - 1, seen, data, slots, loads) {
                    any = true;
                }
            }
            any
        }
        _ => false,
    };
    if derived {
        let v = data.vbank().get(vn).expect("collectCanarySlots: vn");
        if v.is_addr_tied() {
            let key = (v.get_addr().clone(), v.get_size());
            if !slots.contains(&key) {
                slots.push(key);
            }
        }
    }
    derived
}

/// The addrtied storage a protector value was written to.
///
/// A forward fixpoint from each init op's output over the value-preserving
/// readers — `COPY`/`CAST` (the store into the frame slot), `INDIRECT` (the
/// slot carried across a call or an aliasing store), and a `MULTIEQUAL` only
/// when EVERY input is already known to hold the value.  Every addrtied member
/// of the resulting set holds the protector value, so the liveness release
/// cannot reach an unrelated local; only stack-space members are recorded, so a
/// copy of the value into a global keeps its store.  Shared with the MSVC `/GS`
/// sibling, which roots it at the entry-side cookie scramble.
pub(crate) fn collect_value_slots(
    inits: &[OpId],
    data: &Funcdata,
    slots: &mut Vec<(Address, int4)>,
) {
    let mut set: BTreeSet<VarnodeId> = BTreeSet::new();
    let mut work: Vec<VarnodeId> = Vec::new();
    for &op in inits {
        if let Some(out) = data.obank().get(op).and_then(|o| o.get_out()) {
            if set.insert(out) {
                work.push(out);
            }
        }
    }
    while let Some(vn) = work.pop() {
        let readers: Vec<OpId> = match data.vbank().get(vn) {
            Some(v) => v.descend_iter().collect(),
            None => continue,
        };
        for r in readers {
            let Some(rop) = data.obank().get(r) else { continue };
            let joins = match rop.code() {
                OpCode::CPUI_COPY | OpCode::CPUI_CAST => true,
                OpCode::CPUI_INDIRECT => rop.get_in(0) == Some(vn),
                OpCode::CPUI_MULTIEQUAL => {
                    (0..rop.num_input()).all(|i| rop.get_in(i).is_some_and(|x| set.contains(&x)))
                }
                _ => false,
            };
            if !joins {
                continue;
            }
            let Some(out) = rop.get_out() else { continue };
            if set.insert(out) {
                work.push(out);
            }
        }
    }
    let Some(stack) = data.get_arch().manage().get_stack_space().map(Rc::clone) else {
        return;
    };
    for vn in set {
        let Some(v) = data.vbank().get(vn) else { continue };
        let on_stack = v.get_addr().get_space().is_some_and(|s| Rc::ptr_eq(s, &stack));
        if !v.is_addr_tied() || !on_stack {
            continue;
        }
        let key = (v.get_addr().clone(), v.get_size());
        if !slots.contains(&key) {
            slots.push(key);
        }
    }
}

/// Release the address-forced liveness on every version of the resolved
/// canary slot(s) — the kuna realization of angr `StackCanarySimplifier`'s
/// `first_block_copy.statements.pop(stmt_idx)`, which pops the entry-side
/// `canary_slot = *(fs:0x28)` init store (GH-183).
///
/// Shared with the MSVC `/GS` sibling
/// [`kuna_msvcstackguard`](crate::kuna_msvcstackguard), which resolves its own
/// cookie slot and releases it here: the step is about the slot's liveness, not
/// about which protector wrote it.
///
/// The init store writes an addrtied stack varnode; because it is a real
/// (non-marker) write — or an INDIRECT carrying the value across a call under
/// a different input storage — `ActionDirectWrite` marks it direct-write, so
/// `ActionDeadCode` preserves its `addrforce` and the store survives the strip
/// as a dead first statement (`v = *(fs+0x28)`), pinning the `fs:0x28` LOAD
/// and the TLS-base input with it.  After a successful strip this clears
/// `addrforce` on each addrtied varnode at the resolved slot storage.  It is
/// purely a liveness release: the following `ActionDeadCode` still keeps any
/// version with a live reader (e.g. a second, not-yet-stripped check), so a
/// store that cannot be tied to a stripped check is never removed; once the
/// last check is stripped the init store, the LOAD and the TLS-base residue
/// all die through the stock consume fixpoint.
pub(crate) fn release_canary_slots(data: &mut Funcdata, slots: &[(Address, int4)]) {
    for (addr, size) in slots {
        let ids: Vec<VarnodeId> = data.vbank().iter_loc_size_addr(*size, addr).collect();
        for vn in ids {
            let v = data.vbank_mut().get_mut(vn).expect("releaseCanarySlots: stale vn");
            if v.is_addr_tied() && v.is_addr_force() {
                v.clear_addr_force();
            }
        }
        // Excise the slot from the local scope (`ScopeLocal::markNotMapped`,
        // the `checkUnaliasedReturn` idiom for a stack slot that must not
        // become a local): the slot's auto symbol goes away with its ops, and
        // the range boundary keeps `restructureVarnode` from growing an
        // adjacent open local (a char buf[], a piece-written struct) over the
        // freed bytes.
        if let Some(space) = addr.get_space().map(Rc::clone) {
            data.scope_local_mark_not_mapped(&space, addr.get_offset(), *size, false);
        }
    }
}

/// Is the boolean feeding `cbranch` a stack-canary compare?
/// (C++ `isCanaryCompare`, `kuna_stackguard.cc:88`).
///
/// True when it is an `INT_EQUAL`/`INT_NOTEQUAL` whose BOTH operands derive from
/// a LOAD of the canary slot (saved canary vs. fresh reload).  Returns
/// `Some(is_not_equal)` so the caller can pick the corrupted-canary (fail)
/// successor; `None` when it is not a canary compare.
fn is_canary_compare(cbranch: OpId, data: &Funcdata) -> Option<bool> {
    let cb = data.obank().get(cbranch).expect("isCanaryCompare: cbranch");
    let boolvn = cb.get_in(1).expect("isCanaryCompare: cbranch in1");
    let bv = data.vbank().get(boolvn).expect("isCanaryCompare: boolvn");
    if !bv.is_written() {
        return None;
    }
    let cmp = bv.get_def().expect("isCanaryCompare: cmp def");
    let cmpop = data.obank().get(cmp).expect("isCanaryCompare: cmp");
    let oc = cmpop.code();
    if oc != OpCode::CPUI_INT_EQUAL && oc != OpCode::CPUI_INT_NOTEQUAL {
        return None;
    }
    let in0 = cmpop.get_in(0).expect("isCanaryCompare: cmp in0");
    let in1 = cmpop.get_in(1).expect("isCanaryCompare: cmp in1");
    let mut s0: BTreeSet<VarnodeId> = BTreeSet::new();
    let mut s1: BTreeSet<VarnodeId> = BTreeSet::new();
    if !derives_from_canary_load(in0, 32, &mut s0, data) {
        return None;
    }
    if !derives_from_canary_load(in1, 32, &mut s1, data) {
        return None;
    }
    Some(oc == OpCode::CPUI_INT_NOTEQUAL)
}

/// Does basic block `bb` contain a CALL (the canary-fail handler)?
/// (C++ `blockHasCall`, `kuna_stackguard.cc:104`).
fn block_has_call(bb: BlockId, data: &Funcdata) -> bool {
    for op in data.bb_ops(bb) {
        let oc = data.obank().get(op).expect("blockHasCall: op").code();
        if oc == OpCode::CPUI_CALL || oc == OpCode::CPUI_CALLIND || oc == OpCode::CPUI_CALLOTHER {
            return true;
        }
    }
    false
}

/// (kuna) Strip a glibc `-fstack-protector` canary epilogue
/// (C++ `ActionStripStackGuard`, `kuna_stackguard.hh:59`).
///
/// When `option stackguard on`, finds a `__stack_chk_fail` call guarded by a
/// canary compare and removes the failure branch + the now-orphaned handler
/// block.  Inert (returns 0) when the option is off or no canary is present.
pub struct ActionStripStackGuard {
    base: ActionBase,
    /// Explicit enable override for callers without a configured architecture.
    enabled: bool,
}

impl ActionStripStackGuard {
    /// The scheduler passes false; `apply` also reads the live architecture gate.
    pub fn new(enabled: bool, g: impl Into<String>) -> ActionStripStackGuard {
        ActionStripStackGuard { base: ActionBase::new(0, "stripstackguard", g), enabled }
    }
}

impl Action for ActionStripStackGuard {
    fn base(&self) -> &ActionBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut ActionBase {
        &mut self.base
    }

    /// C++ `clone`: filtered by group membership.
    fn clone_filtered(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Action>> {
        if !grouplist.contains(self.get_group()) {
            return None;
        }
        Some(Box::new(ActionStripStackGuard { base: self.base.clone(), enabled: self.enabled }))
    }

    /// C++ `ActionStripStackGuard::apply`, `kuna_stackguard.cc:115`.
    ///
    /// Remove a structurally proven canary failure branch and its unreachable handler.
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++ `if (!data.getArch()->strip_stack_guard) return 0;` — the live gate
        // is carried on the boundary Architecture (`build_arch_handle`); `enabled`
        // stays as the unit-test OR-override (action registered false).
        if !self.enabled && !data.get_arch().strip_stack_guard {
            return 0; // P0 assertion not set
        }

        let size = data.bblocks_get_size();
        for i in 0..size {
            let h = data.bblocks_get_block(i);
            // dynamic_cast<BlockBasic *>(...) — only basic blocks qualify.
            if !matches!(data.bblocks_ref().block(h).kind(), BlockKind::Basic(_)) {
                continue;
            }
            if data.bblocks_ref().block(h).size_out() != 2 {
                continue;
            }
            let cb = match data.bb_op_tail(h) {
                Some(op) => op,
                None => continue, // lastOp() == (PcodeOp *)0
            };
            if data.obank().get(cb).expect("stackguard: cb").code() != OpCode::CPUI_CBRANCH {
                continue;
            }
            let is_not_equal = match is_canary_compare(cb, data) {
                Some(b) => b,
                None => continue,
            };

            // The successor taken when the boolean condition is TRUE.
            let flip = data.obank().get(cb).expect("stackguard: cb flip").is_boolean_flip();
            let hblk = data.bblocks_ref().block(h);
            let cond_true: BlockId = if flip { hblk.get_false_out() } else { hblk.get_true_out() };
            let cond_false: BlockId = if flip { hblk.get_true_out() } else { hblk.get_false_out() };
            // Canary corrupted => the no-return handler.  For `slot != reload`
            // the true edge is the failure; for `slot == reload` the false edge.
            let fail_out: BlockId = if is_not_equal { cond_true } else { cond_false };
            // dynamic_cast<BlockBasic *>(failOut) — must be basic.
            if !matches!(data.bblocks_ref().block(fail_out).kind(), BlockKind::Basic(_)) {
                continue;
            }
            // Safety: the corrupted branch must actually call the handler.
            if !block_has_call(fail_out, data) {
                continue;
            }

            // Find the edge h -> failBlk.
            let mut idx: int4 = -1;
            let hblk = data.bblocks_ref().block(h);
            for j in 0..hblk.size_out() {
                if hblk.get_out(j) == fail_out {
                    idx = j;
                    break;
                }
            }
            if idx < 0 {
                continue;
            }
            // Capture the compare operands before the CBRANCH dies: they root
            // the entry-side canary-init release below (GH-183).
            let (cmp_in0, cmp_in1) = {
                let cbop = data.obank().get(cb).expect("stackguard: cb ins");
                let boolvn = cbop.get_in(1).expect("stackguard: cb in1");
                let cmp = data
                    .vbank()
                    .get(boolvn)
                    .expect("stackguard: boolvn")
                    .get_def()
                    .expect("stackguard: cmp def");
                let cmpop = data.obank().get(cmp).expect("stackguard: cmp");
                (
                    cmpop.get_in(0).expect("stackguard: cmp in0"),
                    cmpop.get_in(1).expect("stackguard: cmp in1"),
                )
            };
            // The in-place CFG surgery (the W4/W8 funcdata_block primitives are
            // now in the merged tree).
            // `removeBranch` severs the corrupted-canary edge (dropping the
            // CBRANCH and patching the saved-canary MULTIEQUAL phis), leaving the
            // `__stack_chk_fail` handler block unreachable; `removeUnreachableBlocks`
            // then collects it.  The shared bare-return tail that survives is
            // duplicated into each predecessor by the immediately-following
            // `ActionReturnSplit`, eliminating the goto/label and inlining the deep
            // match path as a direct `return 1`.
            // Resolve the saved-canary slot storage from the compare's own
            // derivation chains before the CBRANCH dies (GH-183), and from the
            // store side of the canary LOADs they end in: a check copy
            // propagation folded onto the LOAD itself reads no slot (GH-866).
            let mut slots: Vec<(Address, int4)> = Vec::new();
            let mut loads: Vec<OpId> = Vec::new();
            let mut seen: BTreeSet<VarnodeId> = BTreeSet::new();
            collect_canary_slots(cmp_in0, 32, &mut seen, data, &mut slots, &mut loads);
            let mut seen: BTreeSet<VarnodeId> = BTreeSet::new();
            collect_canary_slots(cmp_in1, 32, &mut seen, data, &mut slots, &mut loads);
            collect_value_slots(&loads, data, &mut slots);
            data.remove_branch(h, idx).expect("ActionStripStackGuard: removeBranch");
            data.remove_unreachable_blocks(false, true)
                .expect("ActionStripStackGuard: removeUnreachableBlocks");
            // The check is stripped; release the entry-side canary init (the
            // angr `statements.pop(stmt_idx)` step) so the following
            // ActionDeadCode collects it with the compare/reload residue.
            release_canary_slots(data, &slots);
            self.base.count += 1;
            // One canary per apply; the fullloop re-invokes and self-gates (the
            // compare/handler are gone on the next pass).
            return 1;
        }
        0
    }
}

/// (kuna) Toggle glibc stack-protector epilogue stripping
/// (C++ `OptionStackGuard`, `kuna_stackguard.hh:74`).
///
/// `off` retains the upstream rendering (the Action is inert). `on`
/// strips the `-fstack-protector` canary check + `__stack_chk_fail` call, like
/// angr's StackCanarySimplifier. This standalone value is separate from the
/// live [`Architecture::strip_stack_guard`](crate::architecture::Architecture)
/// flag used by the option dispatcher.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StackGuardOption {
    /// True when the canary epilogue is stripped
    /// (C++ `Architecture::strip_stack_guard`).
    pub enabled: bool,
}

impl Default for StackGuardOption {
    /// Standalone option state starts disabled; Architecture's live default is on.
    fn default() -> Self {
        StackGuardOption { enabled: false }
    }
}

impl StackGuardOption {
    /// Apply the option (C++ `OptionStackGuard::apply`).
    pub fn apply(&mut self, val: bool) -> String {
        self.enabled = val;
        let prop = if val { "on" } else { "off" };
        format!("Stack-protector canary epilogue stripping turned {prop}")
    }

    /// Read the gate (C++ `glb->strip_stack_guard`).
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

#[cfg(test)]
#[path = "kuna_stackguard/tests.rs"]
mod tests;
