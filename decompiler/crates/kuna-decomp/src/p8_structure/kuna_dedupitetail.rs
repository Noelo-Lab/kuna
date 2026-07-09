//! S8 condition-aware **if/else tail deduplication** (the *inverse* of the SAILR
//! duplication passes; a port of angr's structurer ITE region-deduplication —
//! `angr/analyses/decompiler/structuring/structurer_base.py`'s
//! `_merge_nodes` / the `RegionSimplifier` condition-processing that converges two
//! predecessors onto one labelled block instead of cloning a shared continuation).
//!
//! # The structural gap
//!
//! kuna inherits Ghidra's collapse-based structurer
//! ([`crate::blockaction`]), which, when a single basic block is reachable from
//! *both* arms of an `if/else`, **clones** it into each arm rather than converging
//! the two predecessors onto one copy.  On `true_ubuntu1804::usage` (x86-64
//! `0x401610`) the trailing `if (v4 == 0) { A } else { B }` emits the
//! `__printf_chk("...online help...")` / `setlocale(5,0)` / `strncmp(...,"en_",3)`
//! prefix **twice** — once at the head of each arm — because the compiler emitted
//! two physically distinct copies of that prefix and the structurer kept both.
//! angr emits each shared continuation **once**.  This pays kuna an extra `if`,
//! extra lines, and a duplicated call per cloned block.
//!
//! This is the exact **inverse** of the three SAILR goto-reduction passes
//! ([`crate::p8_structure::kuna_gotoreduce`] / `kuna_crossjumpreverter` /
//! `kuna_taildup`), which *duplicate* a shared tail to remove a `goto`.  Here kuna
//! has *over-duplicated* a shared if/else head, and the readability win is to
//! **merge** the identical copies back into one.
//!
//! # What this pass does
//!
//! Running once after [`ActionFinalStructure`](crate::blockaction::ActionFinalStructure)
//! (the structured tree is final and goto targets are labelled), and after the
//! goto-reduction passes, [`ActionDedupIteTail`] finds every 3-component
//! [`BlockIf`](crate::block::BlockKind::If) (`cond` / true-clause / else-clause)
//! whose two clauses share a maximal common **prefix** of *leaf* blocks
//! ([`BlockCopy`](crate::block::BlockKind::Copy)/[`BlockBasic`](crate::block::BlockKind::Basic))
//! that emit **identical C** (the same printed-op sequence, matching constants by
//! value, named storage by location, and call targets by resolved callee — see
//! [`leaves_emit_same_c`]).  It then **hoists** that shared prefix out of both arms
//! into the parent's component list immediately *before* the `if`, deleting the
//! redundant copy from the second arm.
//!
//! Hoisting a shared prefix is **unconditionally sound**: both arms execute the
//! prefix first regardless of the condition, so moving the single retained copy
//! ahead of the `if` (and dropping the duplicate) changes nothing about which code
//! runs or in what order — it is purely a print-tree edit (the structured
//! `list`/`parent`), exactly like
//! [`kuna_flatten_ifelse`](crate::block::BlockGraph::kuna_flatten_ifelse): no p-code
//! is cloned and SSA/def-use is untouched, because the retained leaf still points at
//! its original bblock (the printer re-emits its ops).
//!
//! The transform is bounded and conservative — only flat *leaf* components are
//! considered (never a nested `if`/loop, whose hoisting would change control flow),
//! the leaf C-equivalence is exact and declines on any doubt (differing op shape,
//! constant, storage, callee, or a non-block-local temporary), and a leaf carrying
//! its own label/goto annotation is never hoisted.  Each merge is logged via
//! [`Funcdata::warning_header`].  It is gated by `option dedupitetail on|off`
//! (`dedup_ite_tail`, default-ON per DIV-14) — when off, [`ActionDedupIteTail::apply`]
//! early-returns and output is byte-identical.

use kuna_base::error::KunaResult;
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::action::{Action, ActionBase, ActionContext, ActionGroupList, ApplyResult};
use crate::block::{block_flags, BlockType};
use crate::funcdata::Funcdata;
use crate::options::on_or_off;
use crate::context::{BlockId, VarnodeId};

use kuna_base::marshal::ElementId;

/// Marshaling element `<dedupitetail>` (kuna).  ElementIds live in the 4000+ range
/// (4100 gotoreduce, 4101 tailcalljump, 4102 regionstructure, 4103 noreturn_extern,
/// 4104 noreturn_externmatch, 4105 crossjumprevert, 4106 switchsharedcase,
/// 4107 ifelseflatten, 4108 taildup are taken; this is 4109).
pub const ELEM_DEDUPITETAIL: ElementId = ElementId::new("dedupitetail", 4109);

/// Maximum number of leaf blocks hoisted out of one `if/else` (cap the work and the
/// surgery so the pass stays cheap and the merge stays auditable).
pub const MAX_HOISTED_LEAVES: usize = 8;
/// Maximum number of printed p-code ops compared per leaf-equivalence check.
pub const MAX_LEAF_OPS: usize = 64;

/// \brief Deduplicate a cloned `if/else` head: hoist the maximal common leaf
/// *prefix* shared by both arms out of the `if`, emitting one copy instead of two
/// (the inverse of the SAILR duplication passes).  Option-gated (`dedup_ite_tail`).
pub struct ActionDedupIteTail {
    base: ActionBase,
}

impl ActionDedupIteTail {
    /// Construct in group `g`.
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionDedupIteTail { base: ActionBase::new(0, "dedupitetail", g) })
    }
}

impl Action for ActionDedupIteTail {
    fn base(&self) -> &ActionBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut ActionBase {
        &mut self.base
    }
    fn clone_filtered(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Action>> {
        if !grouplist.contains(self.get_group()) {
            return None;
        }
        Some(Box::new(ActionDedupIteTail { base: self.base.clone() }))
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // P0 assertion `option dedupitetail on|off`, carried on the Architecture boundary.
        if !data.get_arch().dedup_ite_tail {
            return 0;
        }
        if data.sblocks_get_size() == 0 {
            return 0; // not structured
        }
        let root = data.sblocks_root();

        // Snapshot every candidate if/else and the matched prefix/suffix runs, then
        // apply each (re-verifying as we go, since a prior splice can shift the tree).
        // (if_id, kind, [retained-leaf-ids], [duplicate-leaf-ids]) where kind is
        // Prefix (hoist before the if) or Suffix (hoist after the if).
        let mut plans: Vec<(BlockId, HoistKind, Vec<BlockId>, Vec<BlockId>)> = Vec::new();
        for n in collect_nodes(data, root) {
            if let Some((id, r, d)) = plan_prefix_merge(data, n) {
                plans.push((id, HoistKind::Prefix, r, d));
            }
            if let Some((id, r, d)) = plan_suffix_merge(data, n) {
                plans.push((id, HoistKind::Suffix, r, d));
            }
        }
        if plans.is_empty() {
            return 0;
        }

        let mut count: int4 = 0;
        for (if_id, kind, retained, duplicate) in &plans {
            let ok = match kind {
                HoistKind::Prefix => {
                    data.sblocks_mut().kuna_hoist_ite_prefix(*if_id, retained, duplicate)
                }
                HoistKind::Suffix => {
                    data.sblocks_mut().kuna_hoist_ite_suffix(*if_id, retained, duplicate)
                }
            };
            if ok {
                count += 1;
            }
        }

        if count > 0 {
            let name = data.get_name().to_string();
            data.warning_header(&format!(
                "dedupitetail: merged {count} duplicated if/else tail(s) in {name}"
            ));
        }
        self.base_mut().count += count;
        0
    }
}

/// Which end of the two arms a shared run is hoisted from.
#[derive(Clone, Copy, PartialEq, Eq)]
enum HoistKind {
    /// Shared leading run — hoisted *before* the `if` (unconditionally sound).
    Prefix,
    /// Shared trailing run — hoisted *after* the `if` (sound only when neither arm's
    /// divergent middle can skip it).
    Suffix,
}

/// Depth-first snapshot of every structured-block node reachable from `root`.
fn collect_nodes(data: &Funcdata, root: BlockId) -> Vec<BlockId> {
    let mut out = Vec::new();
    let mut stack = vec![root];
    while let Some(id) = stack.pop() {
        out.push(id);
        let g = data.sblocks_ref();
        let n = g.block(id).get_size();
        for i in 0..n {
            stack.push(g.block(id).get_block(i));
        }
    }
    out
}

/// Validate that `n` is a hoist-eligible 3-component `BlockIf` and return its two
/// clause component lists `(true_comps, else_comps)`.  `None` if `n` is not a
/// 3-component `BlockIf` with no own goto, a list/graph parent, and both clauses
/// being plain `Ls`/`Graph` lists.
fn ite_clause_comps(data: &Funcdata, n: BlockId) -> Option<(Vec<BlockId>, Vec<BlockId>)> {
    let g = data.sblocks_ref();
    let blk = g.block(n);
    if blk.get_type() != BlockType::If || blk.get_size() != 3 {
        return None;
    }
    // The if must not carry its own unstructured goto (an if-goto target).
    if blk.get_if_goto_target().is_some() {
        return None;
    }
    // Parent must be a list/graph able to host hoisted leaves as the if's siblings.
    let parent = blk.get_parent()?;
    match g.block(parent).get_type() {
        BlockType::Ls | BlockType::Graph => {}
        _ => return None,
    }
    let true_clause = blk.get_block(1);
    let else_clause = blk.get_block(2);
    // Both clauses must be plain lists (so we can splice off leading/trailing comps).
    match (g.block(true_clause).get_type(), g.block(else_clause).get_type()) {
        (BlockType::Ls, BlockType::Ls)
        | (BlockType::Ls, BlockType::Graph)
        | (BlockType::Graph, BlockType::Ls)
        | (BlockType::Graph, BlockType::Graph) => {}
        _ => return None,
    }
    let tcomps: Vec<BlockId> =
        (0..g.block(true_clause).get_size()).map(|i| g.block(true_clause).get_block(i)).collect();
    let ecomps: Vec<BlockId> =
        (0..g.block(else_clause).get_size()).map(|i| g.block(else_clause).get_block(i)).collect();
    Some((tcomps, ecomps))
}

/// If `n` is a hoist-eligible `BlockIf` whose two clauses share a maximal common
/// **prefix** of hoistable, C-equivalent *leaf* components, return
/// `(if_id, retained_prefix, duplicate_prefix)` (true-arm retained, else-arm dropped).
/// `None` if the shared prefix is empty.
fn plan_prefix_merge(
    data: &Funcdata,
    n: BlockId,
) -> Option<(BlockId, Vec<BlockId>, Vec<BlockId>)> {
    let (tcomps, ecomps) = ite_clause_comps(data, n)?;
    // Both lists must keep at least one component *after* the shared prefix — never
    // empty an arm (that is the ifelseflatten / full-arm-collapse shape, not a tail
    // dedup, and emptying a clause would change the if's structure).
    let maxrun = tcomps.len().min(ecomps.len()).saturating_sub(1).min(MAX_HOISTED_LEAVES);
    let mut retained = Vec::new();
    let mut duplicate = Vec::new();
    for i in 0..maxrun {
        if !equiv_leaf_pair(data, tcomps[i], ecomps[i]) {
            break;
        }
        retained.push(tcomps[i]);
        duplicate.push(ecomps[i]);
    }
    if retained.is_empty() {
        return None;
    }
    Some((n, retained, duplicate))
}

/// If `n` is a hoist-eligible `BlockIf` whose two clauses share a maximal common
/// **suffix** of hoistable, C-equivalent *leaf* components — and whose divergent
/// middles always fall through to that suffix (so hoisting it after the `if` is
/// sound) — return `(if_id, retained_suffix, duplicate_suffix)` (true-arm retained,
/// else-arm dropped, in source order).  `None` if the shared suffix is empty or the
/// fall-through soundness check fails.
fn plan_suffix_merge(
    data: &Funcdata,
    n: BlockId,
) -> Option<(BlockId, Vec<BlockId>, Vec<BlockId>)> {
    let (tcomps, ecomps) = ite_clause_comps(data, n)?;
    let tn = tcomps.len();
    let en = ecomps.len();
    // Keep at least one divergent component before the shared suffix in each arm.
    let maxrun = tn.min(en).saturating_sub(1).min(MAX_HOISTED_LEAVES);
    let mut retained = Vec::new();
    let mut duplicate = Vec::new();
    for k in 0..maxrun {
        let ta = tcomps[tn - 1 - k];
        let eb = ecomps[en - 1 - k];
        if !equiv_leaf_pair(data, ta, eb) {
            break;
        }
        retained.push(ta);
        duplicate.push(eb);
    }
    if retained.is_empty() {
        return None;
    }
    let run = retained.len();
    // Soundness: the divergent middle that precedes the shared suffix in *each* arm
    // must fall through to it — i.e. the component immediately before the suffix run
    // must not statement-terminate (a `return`/no-return) or transfer control away
    // (a `goto`/`break`/`continue`), which could skip the suffix.  If either arm's
    // pre-suffix tail can bypass the suffix, declining keeps output correct.
    let t_before = tcomps[tn - 1 - run];
    let e_before = ecomps[en - 1 - run];
    if !falls_through(data, t_before) || !falls_through(data, e_before) {
        return None;
    }
    // Return the runs in source (forward) order for the surgery helper.
    retained.reverse();
    duplicate.reverse();
    Some((n, retained, duplicate))
}

/// Are `ta` (true-arm) and `eb` (else-arm) both clean C-equivalent leaf components?
fn equiv_leaf_pair(data: &Funcdata, ta: BlockId, eb: BlockId) -> bool {
    if !is_clean_leaf(data, ta) || !is_clean_leaf(data, eb) {
        return false;
    }
    match (leaf_bb(data, ta), leaf_bb(data, eb)) {
        (Some(tbb), Some(ebb)) => leaves_emit_same_c(data, tbb, ebb),
        _ => false,
    }
}

/// Does the structured component `id` always *fall through* to its successor — i.e.
/// it neither statement-terminates (`return`/no-return) nor ends in an explicit
/// control transfer (`goto`/`break`/`continue`/`MultiGoto`) that could skip a
/// following suffix?  Conservative: anything not provably fall-through returns
/// `false` (declines the suffix hoist).
fn falls_through(data: &Funcdata, id: BlockId) -> bool {
    let g = data.sblocks_ref();
    match g.block(id).get_type() {
        // A control transfer can bypass a following suffix.
        BlockType::Goto | BlockType::MultiGoto => false,
        // A leaf falls through unless it ends in a return / no-return call.
        BlockType::Copy | BlockType::Basic => !leaf_terminates(data, id),
        // A list/graph falls through iff its last component does.
        BlockType::Ls | BlockType::Graph => {
            let nsz = g.block(id).get_size();
            if nsz == 0 {
                return true; // empty list: pure fall-through
            }
            falls_through(data, g.block(id).get_block(nsz - 1))
        }
        // An if/then or if/else falls through when *both* its reachable arms do; an
        // if carrying its own goto can transfer away — decline.  Conservatively treat
        // a structured conditional as fall-through only when it has no own goto and
        // neither arm terminates/transfers.  (A 2-component if/then always has an
        // implicit fall-through path when the condition is false, so it falls through.)
        BlockType::If => {
            if g.block(id).get_if_goto_target().is_some() {
                return false;
            }
            // if/then (2 comps): the false path falls through ⇒ fall-through.
            // if/else (3 comps): both arms must fall through.
            if g.block(id).get_size() == 3 {
                falls_through(data, g.block(id).get_block(1))
                    && falls_through(data, g.block(id).get_block(2))
            } else {
                true
            }
        }
        // Loops / switches / conditions: conservatively non-fall-through (a loop may
        // contain a break to elsewhere, a switch may have terminating cases).
        _ => false,
    }
}

/// Does the basic/copy leaf `id` end in a `return` (or a no-return tail call), so
/// control never falls out of it?
fn leaf_terminates(data: &Funcdata, id: BlockId) -> bool {
    let bb = match leaf_bb(data, id) {
        Some(b) => b,
        None => return false,
    };
    let tail = match data.bb_op_tail(bb) {
        Some(t) => t,
        None => return false,
    };
    let op = match data.obank().get(tail) {
        Some(o) => o,
        None => return false,
    };
    match op.code() {
        OpCode::CPUI_RETURN => true,
        OpCode::CPUI_CALL | OpCode::CPUI_CALLIND | OpCode::CPUI_CALLOTHER => {
            data.bblocks_ref().block(bb).size_out() == 0
        }
        _ => false,
    }
}

/// Is `id` a `Copy`/`Basic` leaf carrying no unstructured label/goto annotation
/// (so re-parenting it is a pure print-tree move)?
fn is_clean_leaf(data: &Funcdata, id: BlockId) -> bool {
    let g = data.sblocks_ref();
    let blk = g.block(id);
    match blk.get_type() {
        BlockType::Copy | BlockType::Basic => {}
        _ => return false,
    }
    // A leaf that is itself an unstructured goto target (labelled) must not be moved
    // — its label is referenced from elsewhere.
    let f = blk.get_flags();
    (f & block_flags::f_unstructured_targ) == 0
}

/// The underlying bblock of a `Copy`/`Basic` leaf.
fn leaf_bb(data: &Funcdata, id: BlockId) -> Option<BlockId> {
    let g = data.sblocks_ref();
    match g.block(id).get_type() {
        BlockType::Copy => g.block(id).get_copy(),
        BlockType::Basic => Some(id),
        _ => None,
    }
}

/// Decide whether two leaf bblocks `a` and `b` emit **identical C**: the same printed
/// p-code op sequence where each op has the same opcode and input count, and every
/// corresponding varnode pair is render-equivalent (see [`vn_equiv`]).  Conservative —
/// declines (returns `false`) on the first mismatch, on an unresolved op, on a `STORE`
/// pair that cannot be proven identical, or when either block exceeds [`MAX_LEAF_OPS`]
/// printed ops.
///
/// This intentionally over-approximates "would print the same statements": two ops
/// that print identically have the same opcode + inputs by the same C names/values.
fn leaves_emit_same_c(data: &Funcdata, a: BlockId, b: BlockId) -> bool {
    // Gather the printed ops of each block (skip SSA-bookkeeping non-printing ops:
    // MULTIEQUAL/INDIRECT/markers emit no C statement and are re-resolved by the
    // printer per copy).
    let aops = printed_ops(data, a);
    let bops = printed_ops(data, b);
    if aops.len() != bops.len() || aops.is_empty() {
        return false;
    }
    if aops.len() > MAX_LEAF_OPS {
        return false;
    }
    // Positional unique-temp correspondence: a block-local temporary in A maps to the
    // temporary at the same producing position in B.  Built up as we walk; any
    // mismatch in the correspondence (a temp used before defined, or mapped two ways)
    // declines.
    let mut tmap: std::collections::HashMap<VarnodeId, VarnodeId> = std::collections::HashMap::new();
    for (&oa, &ob) in aops.iter().zip(bops.iter()) {
        let (opa, opb) = match (data.obank().get(oa), data.obank().get(ob)) {
            (Some(x), Some(y)) => (x, y),
            _ => return false,
        };
        let code = opa.code();
        if code != opb.code() || opa.num_input() != opb.num_input() {
            return false;
        }
        // A direct CALL's fspec input (slot 0) is resolved op-wise to the callee entry
        // address (the fspec varnode offset is a per-call-site `qlst` index that always
        // differs); two CALLs match iff they resolve to the same callee.
        let is_call = matches!(
            code,
            OpCode::CPUI_CALL | OpCode::CPUI_CALLIND | OpCode::CPUI_CALLOTHER
        );
        if is_call && !callee_op_equiv(data, oa, ob) {
            return false;
        }
        // Compare each input (skipping the CALL fspec slot 0, handled above), then out.
        for s in 0..opa.num_input() {
            if is_call && s == 0 {
                continue;
            }
            if !vn_equiv(data, opa.get_in(s), opb.get_in(s), code, s, &mut tmap) {
                return false;
            }
        }
        if !vn_equiv(data, opa.get_out(), opb.get_out(), code, -1, &mut tmap) {
            return false;
        }
    }
    true
}

/// The printed (statement-bearing) ops of bblock `bb`, in order.  Skips
/// non-printing SSA-bookkeeping ops *and* the unconditional terminator
/// [`CPUI_BRANCH`](OpCode::CPUI_BRANCH): it emits no C statement (the structurer
/// renders the block's out-edge as fall-through / `goto`), and whether a block
/// carries an explicit branch to its successor versus falling through is a structural
/// artifact, not a difference in printed code.  A `CPUI_CBRANCH` (a conditional, the
/// terminator of an `if`-condition block) *is* kept — it distinguishes condition
/// blocks, which must match exactly.
fn printed_ops(data: &Funcdata, bb: BlockId) -> Vec<crate::context::OpId> {
    let mut out = Vec::new();
    let mut op = data.bb_op_head(bb);
    while let Some(o) = op {
        op = data.bb_op_next(o);
        if let Some(opref) = data.obank().get(o) {
            if opref.not_printed() || opref.code() == OpCode::CPUI_BRANCH {
                continue;
            }
            out.push(o);
        }
    }
    out
}

/// Are two varnodes `a`/`b` (operands of corresponding ops at input slot `slot`,
/// `-1` for the output, with op `code`) **render-equivalent** — i.e. would the C
/// printer emit the same token for both?
///
/// Render-equivalence holds when:
/// * both `None` (no operand), or
/// * both constants of equal value+size, or
/// * the `fspec` (call-spec) operands of a CALL resolve to the same callee, or
/// * both non-constant varnodes at the same storage [`Address`] (so the same C
///   variable / fixed expression), or
/// * both are block-local temporaries already paired (or newly paired here) by the
///   positional correspondence `tmap` — the SSA temps that the printer inlines, whose
///   raw offsets differ between the two physical copies but render the same.
///
/// Declines on anything else (an `iop`/marker pair that is not a resolvable callee, a
/// storage-location mismatch, or a temp paired inconsistently).
fn vn_equiv(
    data: &Funcdata,
    a: Option<VarnodeId>,
    b: Option<VarnodeId>,
    code: OpCode,
    slot: int4,
    tmap: &mut std::collections::HashMap<VarnodeId, VarnodeId>,
) -> bool {
    let (va, vb) = match (a, b) {
        (None, None) => return true,
        (Some(x), Some(y)) => (x, y),
        _ => return false,
    };
    let (ra, rb) = match (data.vbank().get(va), data.vbank().get(vb)) {
        (Some(x), Some(y)) => (x, y),
        _ => return false,
    };
    let aa = ra.get_addr();
    let ab = rb.get_addr();
    let aspace = aa.get_space();
    let bspace = ab.get_space();
    let a_space_name = aspace.map(|s| s.get_name().to_string());
    let b_space_name = bspace.map(|s| s.get_name().to_string());
    // A CALL's `fspec` operand (slot 0) lives in the `fspec` space; its offset is a
    // per-call-site call-spec index.  That slot is resolved op-wise to the callee
    // *before* this function is called (see `callee_op_equiv`) and skipped here.  An
    // fspec varnode reaching `vn_equiv` (any other slot) cannot be compared without its
    // op — conservatively decline.
    let _ = (code, slot);
    if a_space_name.as_deref() == Some("fspec") || b_space_name.as_deref() == Some("fspec") {
        return false;
    }
    // Constants render by value: equal value + size.
    if ra.is_constant() || rb.is_constant() {
        if !(ra.is_constant() && rb.is_constant()) {
            return false;
        }
        return ra.get_offset() == rb.get_offset() && ra.get_size() == rb.get_size();
    }
    // A block-local SSA temporary (unique space) renders inline; its raw offset
    // differs between the two physical copies.  Pair them positionally via `tmap`.
    let a_is_unique = a_space_name.as_deref() == Some("unique");
    let b_is_unique = b_space_name.as_deref() == Some("unique");
    if a_is_unique || b_is_unique {
        if !(a_is_unique && b_is_unique) {
            return false;
        }
        if ra.get_size() != rb.get_size() {
            return false;
        }
        return match tmap.get(&va) {
            Some(&mapped) => mapped == vb,
            None => {
                // First sighting: pair them (one-to-one; reject if vb already taken).
                if tmap.values().any(|&m| m == vb) {
                    return false;
                }
                tmap.insert(va, vb);
                true
            }
        };
    }
    // Any other non-constant operand (register / stack / ram global) renders by its
    // storage location — require the same space + offset + size so the printer emits
    // the same C name / fixed reference.
    a_space_name == b_space_name && aa.get_offset() == ab.get_offset() && ra.get_size() == rb.get_size()
}

/// Do two CALL ops `oa`/`ob` resolve to the same callee?  Each op's call-spec index
/// comes from [`Funcdata::get_call_specs_index`] (matched by the op handle, not the
/// fspec varnode offset, which is a distinct per-call-site index); two direct calls
/// match iff their [`FuncCallSpecs`](crate::fspec::FuncCallSpecs) entry addresses are
/// equal and valid.  An indirect call (invalid/empty entry address) conservatively
/// declines — it would need a same-function-pointer-storage proof we don't attempt.
fn callee_op_equiv(data: &Funcdata, oa: crate::context::OpId, ob: crate::context::OpId) -> bool {
    let ia = match data.get_call_specs_index(oa) {
        Some(i) => i,
        None => return false,
    };
    let ib = match data.get_call_specs_index(ob) {
        Some(i) => i,
        None => return false,
    };
    let ea = data.get_call_specs(ia).get_entry_address();
    let eb = data.get_call_specs(ib).get_entry_address();
    if ea.is_invalid() || eb.is_invalid() {
        return false;
    }
    ea == eb
}

//===========================================================================
// Option parse (`dedupitetail on|off`).
//===========================================================================

/// The `dedupitetail on|off` ArchOption (angr structurer ITE region-dedup).
pub struct OptionDedupIteTail;

impl OptionDedupIteTail {
    /// The option name.
    pub const NAME: &'static str = "dedupitetail";

    /// Parse `on`/`off`, returning the bool value + a confirmation message.
    pub fn apply(&self, p1: &str) -> KunaResult<(bool, String)> {
        let val = on_or_off(p1)?;
        let prop = if val { "on" } else { "off" };
        Ok((val, format!("If/else tail deduplication turned {prop}")))
    }
}
