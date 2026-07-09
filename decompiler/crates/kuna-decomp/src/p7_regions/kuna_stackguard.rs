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
//! ## Detection is purely structural
//!
//! kuna's BFD console loader does not resolve a PLT stub to its imported name,
//! so the callee name is NOT a usable signal.  The canary is pinned by its
//! compare: a CBRANCH whose `INT_EQUAL`/`INT_NOTEQUAL` boolean compares two
//! values that BOTH derive from a LOAD of `<base> + 0x28` (the saved canary slot
//! vs. a fresh reload of the x86-64 glibc TLS canary at `fs:0x28`).  The
//! corrupted-canary branch (which must contain the no-return handler CALL) is
//! removed with the stock in-place primitive `Funcdata::removeBranch`
//! (CBRANCH → fall-through, MULTIEQUALs patched), then
//! `removeUnreachableBlocks` collects the orphaned handler block.
//!
//! ## The option
//!
//! The decision is a P0 assertion: `option stackguard on|off` (default `off` =
//! upstream byte-identical; the Action is inert when off).  The live flag is
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
    /// Resolved `glb->strip_stack_guard` gate (STUB(W4); see module docs).  The
    /// C++ reads `data.getArch()->strip_stack_guard` inline, but the seam
    /// [`Funcdata::get_arch`] does not carry kuna analysis flags, so — as in the
    /// sibling kuna Actions/Rules — the gate is resolved at construction.
    enabled: bool,
}

impl ActionStripStackGuard {
    /// Constructor (C++ `ActionStripStackGuard(const string &g)` — flags 0,
    /// name "stripstackguard").  `enabled` is the resolved `strip_stack_guard`
    /// gate (default on per DIV-14; `option stackguard off` restores the upstream rendering).
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
    /// The detection — option gate, per-block CBRANCH-on-canary-compare scan,
    /// fail-successor selection, handler-call safety, and victim-edge index —
    /// is realized faithfully.  The terminal `removeBranch`/
    /// `removeUnreachableBlocks` pair is the W4/W8 funcdata_block seam (see the
    /// module docs); the scan still runs and `idx` is computed so the seam is
    /// a single drop-in point.
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++ `if (!data.getArch()->strip_stack_guard) return 0;` — the live gate
        // is carried on the seam Architecture (`build_arch_handle`); `enabled`
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
            // The in-place CFG surgery (the W4/W8 funcdata_block primitives are
            // now in the merged tree).  The C++ here is:
            //   data.removeBranch(h,idx);                  // CBRANCH -> fall-through, MULTIEQUALs patched
            //   data.removeUnreachableBlocks(false,true);  // collect the orphaned handler block
            //   return 1; // one canary per apply; the fullloop re-invokes and self-gates
            // `removeBranch` severs the corrupted-canary edge (dropping the
            // CBRANCH and patching the saved-canary MULTIEQUAL phis), leaving the
            // `__stack_chk_fail` handler block unreachable; `removeUnreachableBlocks`
            // then collects it.  The shared bare-return tail that survives is
            // duplicated into each predecessor by the immediately-following
            // `ActionReturnSplit`, eliminating the goto/label and inlining the deep
            // match path as a direct `return 1`.
            data.remove_branch(h, idx).expect("ActionStripStackGuard: removeBranch");
            data.remove_unreachable_blocks(false, true)
                .expect("ActionStripStackGuard: removeUnreachableBlocks");
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
/// `off` (default) is upstream byte-identical (the Action is inert).  `on`
/// strips the `-fstack-protector` canary check + `__stack_chk_fail` call, like
/// angr's StackCanarySimplifier.  The resolved value is written into
/// [`Architecture::strip_stack_guard`](crate::architecture::Architecture).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StackGuardOption {
    /// True when the canary epilogue is stripped
    /// (C++ `Architecture::strip_stack_guard`).
    pub enabled: bool,
}

impl Default for StackGuardOption {
    /// Shipped default: `option stackguard off` (upstream byte-identical;
    /// `architecture.rs` sets `strip_stack_guard = false`).
    fn default() -> Self {
        StackGuardOption { enabled: false }
    }
}

impl StackGuardOption {
    /// Apply the option (C++ `OptionStackGuard::apply`):
    /// ```text
    ///   bool val = onOrOff(p1);
    ///   glb->strip_stack_guard = val;
    ///   string prop = val ? "on" : "off";
    ///   return "Stack-protector canary epilogue stripping turned "+prop;
    /// ```
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
