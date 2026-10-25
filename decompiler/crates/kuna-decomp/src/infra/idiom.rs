//! (kuna) Shared CFG idiom shapes for language-runtime reductions
//! (`docs/multilang.md` W2).
//!
//! A compiled language runtime stamps the same few control-flow shapes into
//! every function it manages — a guard branch to a cold runtime call, a
//! stack-growth loop around the entry — and each language pack wants to match
//! one and collapse it. The *matchers* live here, shared and separately
//! testable; each reduction stays its own `kuna_<slug>.rs` action with its own
//! option, per the one-feature-one-module rule. The matchers only read the
//! `bblocks` CFG; what to do with a match (which edge to cut, which op to
//! lower) is the caller's.
//!
//! The first shape, [`match_entry_guard_loop`], is the Go stack-growth
//! prologue (`p2_lift/kuna_gomorestack.rs`); the guarded-cold-call diamond the
//! existing strippers hand-match is the next candidate to migrate when a new
//! consumer lands.

use kuna_num::opcodes::OpCode;

use kuna_base::types::int4;

use crate::block::BlockKind;
use crate::context::{BlockId, OpId};
use crate::funcdata::Funcdata;

/// A matched entry-guard loop: a conditional at the function entry whose cold
/// arm calls a runtime routine and loops back to re-run the entry.
///
/// ```text
///        E (size_in 0)
///        |
///        v
///   +--> S:  cmp ...; CBRANCH ----+   guard_block = S, guard_edge = S->G
///   |    |                        |
///   |    v (hot)                  v (cold)
///   |  body...               G:  CALL runtime-routine
///   +----------------------------+   grow_block = G (sole out-edge back to S)
/// ```
///
/// Cutting `guard_edge` makes `G` unreachable and the loop disappears; the
/// entry sees straight-line flow, so no entry phi forms for any live register.
pub struct EntryGuardLoop {
    /// The guard block `S` holding the terminal CBRANCH.
    pub guard_block: BlockId,
    /// The out-edge index of `S` that enters the cold arm `G`.
    pub guard_edge: int4,
    /// The cold arm `G`: calls the runtime routine, loops back to `S`.
    pub grow_block: BlockId,
    /// The runtime CALL inside `G` that identified the shape.
    pub call_op: OpId,
}

/// Match one entry-guard loop whose cold-arm callee satisfies `callee`.
///
/// The shape is held tight so a name alone can never cut a real edge:
/// - the CALL's block `G` has exactly one in-edge and one out-edge, both `S`
///   (the tight two-block cycle a re-run-the-entry loop compiles to);
/// - `S` ends in a CBRANCH with two out-edges (one of them `G`);
/// - `S` is the function entry: its only other in-edge comes from a block with
///   no in-edges of its own (the empty entry block `FlowInfo::generate_blocks`
///   inserts whenever the start address has an incoming edge — which the loop
///   back-edge guarantees here).
///
/// Returns the first match; a function carries at most one entry guard, and a
/// caller that edits the CFG re-matches rather than holding stale ids.
pub fn match_entry_guard_loop(
    data: &Funcdata,
    callee: &dyn Fn(&str) -> bool,
) -> Option<EntryGuardLoop> {
    for i in 0..data.num_calls() {
        let fc = data.get_call_specs(i);
        if !callee(fc.get_name()) {
            continue;
        }
        let call_op = fc.get_op();
        let Some(op) = data.obank().get(call_op) else { continue };
        let Some(grow) = op.get_parent() else { continue };
        let blocks = data.bblocks_ref();
        if !matches!(blocks.block(grow).kind(), BlockKind::Basic(_)) {
            continue;
        }
        // The tight cycle: G's one successor is also its one predecessor.
        if blocks.block(grow).size_out() != 1 || blocks.block(grow).size_in() != 1 {
            continue;
        }
        let guard = blocks.block(grow).get_out(0);
        if guard != blocks.block(grow).get_in(0) {
            continue;
        }
        if !matches!(blocks.block(guard).kind(), BlockKind::Basic(_)) {
            continue;
        }
        if blocks.block(guard).size_out() != 2 || blocks.block(guard).size_in() != 2 {
            continue;
        }
        // The guard's terminal op is the conditional the runtime planted.
        let Some(tail) = data.bb_op_tail(guard) else { continue };
        let Some(tail_op) = data.obank().get(tail) else { continue };
        if tail_op.code() != OpCode::CPUI_CBRANCH {
            continue;
        }
        // The guard's other predecessor is the inserted entry block: a block
        // nothing enters. Anywhere else in the CFG, a two-block cycle hangs
        // off a reachable predecessor and this test refuses it.
        let other = (0..2)
            .map(|j| blocks.block(guard).get_in(j))
            .find(|&p| p != grow);
        let Some(entry) = other else { continue };
        if blocks.block(entry).size_in() != 0 {
            continue;
        }
        let guard_edge = (0..2)
            .find(|&j| blocks.block(guard).get_out(j) == grow)
            .expect("entry-guard loop: G is one of S's two out-edges");
        return Some(EntryGuardLoop { guard_block: guard, guard_edge, grow_block: grow, call_op });
    }
    None
}

/// Match the TRUNCATED form of the entry-guard loop: the same guard and cold
/// arm, but the runtime call was marked no-return (an unstripped image's
/// discovered no-return propagation reaches `runtime.morestack*` through its
/// `newstack` tail), so flow halts at the call, the loop back-edge was never
/// lifted, and no entry block was inserted. The shape is held as tight as the
/// loop form:
/// - the cold arm `G` is a basic block with NO out-edges (the halt), exactly
///   one in-edge, and the matched call is the ONLY call it contains;
/// - its one predecessor `S` ends in a CBRANCH with two out-edges;
/// - `S` is the function start: a block nothing enters (without the back-edge
///   there is no inserted entry block, so the guard itself has no
///   predecessors). Anywhere else, a guarded diverging call hangs off a
///   reachable predecessor and this refuses it.
///
/// Cutting `guard_edge` is the same assertion as the loop form's: the body
/// runs on a stack that holds its frame.
pub fn match_entry_guard_halt(
    data: &Funcdata,
    callee: &dyn Fn(&str) -> bool,
) -> Option<EntryGuardLoop> {
    for i in 0..data.num_calls() {
        let fc = data.get_call_specs(i);
        if !callee(fc.get_name()) {
            continue;
        }
        let call_op = fc.get_op();
        let Some(op) = data.obank().get(call_op) else { continue };
        let Some(grow) = op.get_parent() else { continue };
        let blocks = data.bblocks_ref();
        if !matches!(blocks.block(grow).kind(), BlockKind::Basic(_)) {
            continue;
        }
        // Diverging one-entry cold arm.
        if blocks.block(grow).size_out() != 0 || blocks.block(grow).size_in() != 1 {
            continue;
        }
        // The matched call is the only call in the arm: a diverging block that
        // does anything else is a real panic/abort path, never the prologue.
        let other_call = (0..data.num_calls()).any(|j| {
            if j == i {
                return false;
            }
            let oc = data.get_call_specs(j).get_op();
            data.obank().get(oc).and_then(|o| o.get_parent()) == Some(grow)
        });
        if other_call {
            continue;
        }
        let guard = blocks.block(grow).get_in(0);
        if !matches!(blocks.block(guard).kind(), BlockKind::Basic(_)) {
            continue;
        }
        if blocks.block(guard).size_out() != 2 || blocks.block(guard).size_in() != 0 {
            continue;
        }
        let Some(tail) = data.bb_op_tail(guard) else { continue };
        let Some(tail_op) = data.obank().get(tail) else { continue };
        if tail_op.code() != OpCode::CPUI_CBRANCH {
            continue;
        }
        let guard_edge = (0..2)
            .find(|&j| blocks.block(guard).get_out(j) == grow)
            .expect("entry-guard halt: G is one of S's two out-edges");
        return Some(EntryGuardLoop { guard_block: guard, guard_edge, grow_block: grow, call_op });
    }
    None
}

/// A matched guarded cold call: a conditional whose one arm is a single cold
/// block that calls a runtime routine and rejoins the other arm immediately.
///
/// ```text
///   S:  ...; CBRANCH ----------+      guard_block = S, guard_edge = S->A
///   |                          |
///   v (hot)                    v (cold)
///   J:  ...                A:  CALL runtime-routine; ...
///   ^                          |
///   +--------------------------+      cold_block = A (sole out-edge to J)
/// ```
///
/// Cutting `guard_edge` makes `A` unreachable; the hot path through `J` — the
/// program's own semantics — is untouched, and whatever fed only the cold
/// arm's call dies to ordinary dead-code elimination.
pub struct GuardedColdCall {
    /// The guard block `S` holding the terminal CBRANCH.
    pub guard_block: BlockId,
    /// The out-edge index of `S` that enters the cold arm `A`.
    pub guard_edge: int4,
    /// The cold arm `A`: one block, one runtime CALL, rejoins at `J`.
    pub cold_block: BlockId,
    /// The runtime CALL inside `A` that identified the shape.
    pub call_op: OpId,
}

/// Match one guarded-cold-call diamond whose cold-arm callee satisfies
/// `callee`.
///
/// Held as tight as [`match_entry_guard_loop`], and by the same principle —
/// the name is only consulted inside the shape:
/// - the CALL's block `A` has exactly one in-edge (the guard `S`) and one
///   out-edge (the join `J`), and carries no OTHER call of any kind;
/// - `S` ends in a CBRANCH with two out-edges, one of them `A`;
/// - `S`'s other out-edge goes DIRECTLY to `J` — the diamond closes
///   immediately, which is what a compiler-planted flag test looks like and a
///   meaningful `if (...) { helper(); }` usually does not (its arms carry the
///   program's own statements and rarely re-join in one hop with a lone call).
///
/// Either polarity is accepted: the cold arm is identified by what it
/// contains, not by the branch sense (Go emits the barrier arm as the
/// fall-through). Returns the first match; a caller that edits the CFG
/// re-matches rather than holding stale ids.
pub fn match_guarded_cold_call(
    data: &Funcdata,
    callee: &dyn Fn(&str) -> bool,
) -> Option<GuardedColdCall> {
    for i in 0..data.num_calls() {
        let fc = data.get_call_specs(i);
        if !callee(fc.get_name()) {
            continue;
        }
        let call_op = fc.get_op();
        let Some(op) = data.obank().get(call_op) else { continue };
        let Some(cold) = op.get_parent() else { continue };
        let blocks = data.bblocks_ref();
        if !matches!(blocks.block(cold).kind(), BlockKind::Basic(_)) {
            continue;
        }
        if blocks.block(cold).size_in() != 1 || blocks.block(cold).size_out() != 1 {
            continue;
        }
        let guard = blocks.block(cold).get_in(0);
        let join = blocks.block(cold).get_out(0);
        if guard == cold || join == cold || join == guard {
            continue;
        }
        if !matches!(blocks.block(guard).kind(), BlockKind::Basic(_)) {
            continue;
        }
        if blocks.block(guard).size_out() != 2 {
            continue;
        }
        let Some(tail) = data.bb_op_tail(guard) else { continue };
        let Some(tail_op) = data.obank().get(tail) else { continue };
        if tail_op.code() != OpCode::CPUI_CBRANCH {
            continue;
        }
        // The hot arm must close the diamond in one hop.
        let guard_edge = (0..2).find(|&j| blocks.block(guard).get_out(j) == cold);
        let Some(guard_edge) = guard_edge else { continue };
        if blocks.block(guard).get_out(1 - guard_edge) != join {
            continue;
        }
        // The cold arm's only transfer is the matched runtime call.
        if block_has_other_call(data, cold, call_op) {
            continue;
        }
        return Some(GuardedColdCall { guard_block: guard, guard_edge, cold_block: cold, call_op });
    }
    None
}

/// `true` if block `bl` holds any CALL/CALLIND besides `keep` — a cold arm
/// with a second transfer is a real `if` body, not a runtime check.
fn block_has_other_call(data: &Funcdata, bl: BlockId, keep: OpId) -> bool {
    for i in 0..data.num_calls() {
        let fc = data.get_call_specs(i);
        let op = fc.get_op();
        if op == keep {
            continue;
        }
        if let Some(o) = data.obank().get(op) {
            if o.get_parent() == Some(bl) {
                return true;
            }
        }
    }
    false
}
