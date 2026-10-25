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

