//! S8 **ITE-region → `?:` ternary recovery** (a port of angr's
//! [`ITERegionConverter`] — `angr/analyses/decompiler/optimization_passes/
//! ite_region_converter.py`, and its `_find_ite_assignment_regions`).
//!
//! # The structural gap
//!
//! At `-O0` a source ternary such as `flags ? "%s," : "%s"` compiles to a two-arm
//! register-assignment **diamond**: one arm does `lea rax, "%s,"`, the other
//! `lea rax, "%s"`, and both converge on the shared continuation (the use of the
//! merged value).  kuna inherits Ghidra's collapse-based structurer, which renders
//! each such diamond as a literal `if (c) { v = A } else { v = B }` statement pair
//! — **doubling** the branch count of any function full of them.  On iproute2's
//! `print_link_flags` this pays kuna 41 `if`s / 0 ternaries (232 LOC) where the
//! source and angr emit 21 `if`s / 20 ternaries (107 LOC), GED 140.
//!
//! angr's `ITERegionConverter` matches exactly this schema — a conditional-jump
//! region with two single-assignment children converging on one tail — and
//! rewrites it to `x = c ? a : b`.
//!
//! # Why this is a **runtime choice** (default-OFF, agent-flippable)
//!
//! The diamond → ternary rewrite reproduces the source *only when the source used
//! a ternary*.  When the source instead wrote an explicit `if/else`, the same
//! object code is produced, and rewriting it to `?:` **diverges** from the source.
//! There is no way to tell the two apart from the binary, so this ships default-OFF
//! and per-function agent-flippable (`kuna catalog` → `--option iteregion on`):
//! turn it on for format/print/flag-heavy code (where the ternary form dominates),
//! leave it off where explicit `if/else` is the likely source.  With the option off
//! [`ActionIteRegion::apply`] early-returns, nothing is marked, and output is
//! byte-identical to upstream.
//!
//! # What this pass does
//!
//! Running once after [`ActionFinalStructure`](crate::blockaction::ActionFinalStructure)
//! (the structured tree is final and goto targets are labelled) — alongside
//! [`ActionDedupIteTail`](crate::p8_structure::kuna_dedupitetail::ActionDedupIteTail)
//! and the other post-structuring rewrites — [`ActionIteRegion`] finds every
//! narrow, unambiguous assignment diamond and marks it for ternary rendering:
//!
//! * a 3-component [`BlockIf`](crate::block::BlockKind::If) (`cond` / true-clause /
//!   else-clause) with **no own goto**,
//! * whose condition block is a **single leaf** whose only printed op is the
//!   `CBRANCH` (no side-effecting statement rides on the condition),
//! * whose true- and else-clauses are each a **single leaf** whose only printed op
//!   is a `COPY` writing to the **same variable** (same storage) — the post-SSA
//!   instances of the diamond's merge.
//!
//! The match is **print-only**: the pass mutates *no* p-code.  It sets the
//! [`kuna_iteregion`](crate::op::pcodeop_addlflags::kuna_iteregion) addl-flag on
//! the matched `CBRANCH`; the S9 C printer
//! ([`PrintC::emit_block_if`](crate::printc)) sees that flag, re-derives the same
//! match via [`match_ite_assignment`], and emits the single statement
//! `dest = ( cond ) ? A : B;` in place of the `if/else` — the *only* new
//! infrastructure this feature needs (Ghidra's `printc` has no ternary operator).
//! The two arm COPYs and the condition are still live in the IR, so the merged
//! variable's later uses render unchanged; only the layout of the `if` differs.
//!
//! The transform is bounded and conservative: it declines on anything but the exact
//! single-COPY / single-CBRANCH / same-destination shape (never a nested if/loop, a
//! multi-statement arm, a labelled/goto leaf, or two arms writing different
//! variables), so a follow-up fold of the single-def/single-use temp into its use
//! site is left for a later increment.  It is gated by `option iteregion on|off`
//! (`iteregion`, default-OFF); each function with matches is logged via
//! [`Funcdata::warning_header`].

use kuna_base::error::KunaResult;
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::action::{Action, ActionBase, ActionContext, ActionGroupList, ApplyResult};
use crate::block::BlockType;
use crate::funcdata::Funcdata;
use crate::op::pcodeop_addlflags;
use crate::options::on_or_off;
use crate::seams::{BlockId, OpId, VarnodeId};

use kuna_base::marshal::ElementId;

/// Marshaling element `<iteregion>` (kuna).  ElementIds live in the 4000+ range
/// (4100 gotoreduce … 4109 dedupitetail are taken; this is 4110).
pub const ELEM_ITEREGION: ElementId = ElementId::new("iteregion", 4110);

/// A matched two-arm assignment diamond that renders as `dest = cond ? A : B`.
/// Derived purely from the structured [`BlockIf`]; carries the op handles the S9
/// printer needs so the ternary emit reuses the normal condition/RHS rendering.
#[derive(Clone, Copy)]
pub struct IteAssignMatch {
    /// The condition leaf (structured block 0 of the `if`); rendered with
    /// `ONLY_BRANCH` gives `( cond )`.
    pub cond_block: BlockId,
    /// The `CBRANCH` op in `cond_block` (carries the
    /// [`kuna_iteregion`](pcodeop_addlflags::kuna_iteregion) mark).
    pub cbranch: OpId,
    /// The single `COPY` op of the **true** clause (taken when the rendered
    /// condition is true).  Its RHS renders as the `?` value.
    pub true_op: OpId,
    /// The single `COPY` op of the **else** clause.  Its RHS renders as the `:`
    /// value.
    pub else_op: OpId,
    /// The assignment target (`true_op`'s output; storage-equal to `else_op`'s).
    pub dest: VarnodeId,
}

/// \brief Rewrite a two-arm assignment diamond to a `?:` ternary (a port of angr's
/// `ITERegionConverter`).  Print-only + option-gated (`iteregion`), a deliberate
/// runtime choice: it matches the source only when the source used a ternary.
pub struct ActionIteRegion {
    base: ActionBase,
}

impl ActionIteRegion {
    /// Construct in group `g`.
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionIteRegion { base: ActionBase::new(0, "iteregion", g) })
    }
}

impl Action for ActionIteRegion {
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
        Some(Box::new(ActionIteRegion { base: self.base.clone() }))
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // P0 assertion `option iteregion on|off`, carried on the seam Architecture.
        // Default-OFF (a runtime choice) ⇒ nothing marked ⇒ byte-identical output.
        if !data.get_arch().iteregion {
            return 0;
        }
        if data.sblocks_get_size() == 0 {
            return 0; // not structured
        }
        let root = data.sblocks_root();

        // Snapshot the matched CBRANCH of every eligible diamond, then mark them
        // (a pure print-time hint the S9 printer consumes; no p-code is touched).
        let mut marks: Vec<OpId> = Vec::new();
        for n in collect_nodes(data, root) {
            if let Some(m) = match_ite_assignment(data, n) {
                marks.push(m.cbranch);
            }
        }
        if marks.is_empty() {
            return 0;
        }

        let count = marks.len() as int4;
        for cbranch in marks {
            if let Some(o) = data.obank_mut().get_mut(cbranch) {
                o.set_additional_flag(pcodeop_addlflags::kuna_iteregion);
            }
        }

        let name = data.get_name().to_string();
        data.warning_header(&format!(
            "iteregion: rewrote {count} if/else assignment diamond(s) to ?: ternary in {name}"
        ));
        self.base_mut().count += count;
        0
    }
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

/// If the structured node `n` is a narrow two-arm assignment diamond — a
/// 3-component [`BlockIf`](crate::block::BlockKind::If) with no own goto, a
/// single-`CBRANCH` condition leaf, and both clauses a single `COPY` to the same
/// variable — return the [`IteAssignMatch`] the S9 printer needs.  Otherwise
/// `None`.
///
/// Shared by [`ActionIteRegion::apply`] (which marks the `CBRANCH`) and the printer
/// (which re-derives the same match to emit the ternary), so the schema is defined
/// once.
pub fn match_ite_assignment(data: &Funcdata, n: BlockId) -> Option<IteAssignMatch> {
    let g = data.sblocks_ref();
    let blk = g.block(n);
    // 3-component if/else (cond / true / else) with no own unstructured goto.
    if blk.get_type() != BlockType::If || blk.get_size() != 3 {
        return None;
    }
    if blk.get_if_goto_target().is_some() {
        return None;
    }
    let cond_block = blk.get_block(0);
    let true_clause = blk.get_block(1);
    let else_clause = blk.get_block(2);

    // Condition leaf: a leaf block whose terminator (last printed op) is the
    // CBRANCH.  Leading statements are allowed (e.g. the `flags &= ~IFF_x` that
    // precedes each `_PF` diamond) — the printer emits them normally *before* the
    // ternary, so nothing is lost.
    let cbranch = cond_cbranch(data, cond_block)?;

    // Each arm: a single leaf whose only printed op is a COPY to a variable.
    let (true_op, dest_true) = single_copy_assign(data, true_clause)?;
    let (else_op, dest_else) = single_copy_assign(data, else_clause)?;

    // Both arms must write the SAME variable (the post-SSA merge target), so the
    // ternary has one well-defined destination.
    if !same_storage(data, dest_true, dest_else) {
        return None;
    }

    Some(IteAssignMatch { cond_block, cbranch, true_op, else_op, dest: dest_true })
}

/// The single basic block underlying a structured leaf `id`, descending through a
/// one-component `Ls`/`Graph` wrapper (a structured arm is usually a list of one).
/// `None` if `id` (or its sole child) is not a clean, unlabelled `Copy`/`Basic`
/// leaf.
fn leaf_bblock(data: &Funcdata, id: BlockId) -> Option<BlockId> {
    let g = data.sblocks_ref();
    let blk = g.block(id);
    match blk.get_type() {
        BlockType::Basic => Some(id),
        BlockType::Copy => {
            // A leaf that is itself an unstructured-goto target (labelled) must not
            // be collapsed into a ternary — its label is referenced elsewhere.
            if (blk.get_flags() & crate::block::block_flags::f_unstructured_targ) != 0 {
                return None;
            }
            blk.get_copy()
        }
        BlockType::Ls | BlockType::Graph => {
            if blk.get_size() != 1 {
                return None;
            }
            leaf_bblock(data, blk.get_block(0))
        }
        _ => None,
    }
}

/// If the structured leaf `id` is a condition block whose **last** printed op is a
/// `CBRANCH`, return that op.  Any leading statements (the `flags &= ~IFF_x` in
/// front of each `_PF` diamond) are permitted — the printer re-emits them normally
/// before the ternary, so nothing is dropped.  Declines a block that does not end
/// in a `CBRANCH`.
fn cond_cbranch(data: &Funcdata, id: BlockId) -> Option<OpId> {
    let bb = leaf_bblock(data, id)?;
    let ops = printed_ops(data, bb);
    let op = *ops.last()?;
    if data.obank().get(op)?.code() != OpCode::CPUI_CBRANCH {
        return None;
    }
    Some(op)
}

/// If the structured leaf `id` is an arm whose single printed op is a `COPY`
/// writing to a variable, return `(copy_op, out_vn)`.  This is the "single
/// assignment" arm angr's `_find_ite_assignment_regions` requires (each child is
/// exactly one assignment statement).
fn single_copy_assign(data: &Funcdata, id: BlockId) -> Option<(OpId, VarnodeId)> {
    let bb = leaf_bblock(data, id)?;
    let ops = printed_ops(data, bb);
    if ops.len() != 1 {
        return None;
    }
    let op = ops[0];
    let o = data.obank().get(op)?;
    if o.code() != OpCode::CPUI_COPY {
        return None;
    }
    let out = o.get_out()?;
    // The COPY must have a readable source operand to render as the ternary value.
    o.get_in(0)?;
    Some((op, out))
}

/// The **statement-bearing** ops of bblock `bb`, in order — exactly what the C
/// printer would emit as separate statements ([`crate::printc`]'s
/// `emit_basic_block_ops`): skip non-printing SSA-bookkeeping ops, skip the
/// unconditional [`CPUI_BRANCH`] terminator (no C), and skip any op whose output is
/// an **implied** varnode (inlined at its single use site, so it emits no statement
/// of its own — e.g. the `INT_NOTEQUAL` feeding a `CBRANCH`, or a folded RHS).  A
/// [`CPUI_CBRANCH`] (no output) *is* kept.  Skipping implied ops is what makes a
/// bare `if (a != 0)` condition block count as one statement (the CBRANCH), not two.
fn printed_ops(data: &Funcdata, bb: BlockId) -> Vec<OpId> {
    let mut out = Vec::new();
    let mut op = data.bb_op_head(bb);
    while let Some(o) = op {
        op = data.bb_op_next(o);
        if let Some(opref) = data.obank().get(o) {
            if opref.not_printed() || opref.code() == OpCode::CPUI_BRANCH {
                continue;
            }
            // An op whose output is implied is inlined at its use — no statement.
            if let Some(outvn) = opref.get_out() {
                if data.vbank().get(outvn).map(|v| v.is_implied()).unwrap_or(false) {
                    continue;
                }
            }
            out.push(o);
        }
    }
    out
}

/// Do two varnodes render as the **same variable** — same storage space, offset,
/// and size?  (The two post-SSA COPY outputs of a merged diamond share storage.)
fn same_storage(data: &Funcdata, a: VarnodeId, b: VarnodeId) -> bool {
    let (ra, rb) = match (data.vbank().get(a), data.vbank().get(b)) {
        (Some(x), Some(y)) => (x, y),
        _ => return false,
    };
    let aa = ra.get_addr();
    let ab = rb.get_addr();
    let a_space = aa.get_space().map(|s| s.get_name().to_string());
    let b_space = ab.get_space().map(|s| s.get_name().to_string());
    a_space == b_space && aa.get_offset() == ab.get_offset() && ra.get_size() == rb.get_size()
}

//===========================================================================
// Option parse (`iteregion on|off`).
//===========================================================================

/// The `iteregion on|off` ArchOption (angr `ITERegionConverter`).
pub struct OptionIteRegion;

impl OptionIteRegion {
    /// The option name.
    pub const NAME: &'static str = "iteregion";

    /// Parse `on`/`off`, returning the bool value + a confirmation message.
    pub fn apply(&self, p1: &str) -> KunaResult<(bool, String)> {
        let val = on_or_off(p1)?;
        let prop = if val { "on" } else { "off" };
        Ok((val, format!("ITE-region assignment-diamond -> ?: ternary recovery turned {prop}")))
    }
}
