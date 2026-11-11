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
use crate::context::{BlockId, OpId, VarnodeId};
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

/// A matched barrier arm whose values escape only through join phis: the
/// post-SSA form of the guarded cold call ([`GuardedColdCall`] is the pre-SSA
/// one-hop form). The arm may be shared by several guards and its call result
/// may feed MULTIEQUALs at the join — exactly what the one-hop matcher
/// refuses — so this match carries EVERY guard edge and is only sound to act
/// on once SSA has made every read explicit.
pub struct GuardedColdCallPhi {
    /// Every `(guard block, out-edge index)` entering the cold arm. Each
    /// guard ends in a CBRANCH whose condition reads a global flag.
    pub guard_edges: Vec<(BlockId, int4)>,
    /// The cold arm `A`: one block, one runtime CALL, bookkeeping only.
    pub cold_block: BlockId,
    /// The runtime CALL inside `A` that identified the shape.
    pub call_op: OpId,
}

/// Opcodes a GC-bookkeeping arm may contain besides its one CALL: the buffer
/// stores, the loads of old values, address arithmetic, value plumbing, the
/// call's own INDIRECT effects, shared-arm phis, and the closing branch.
/// Anything else is program logic and refuses the match.
fn wbarm_op_allowed(code: OpCode) -> bool {
    matches!(
        code,
        OpCode::CPUI_COPY
            | OpCode::CPUI_LOAD
            | OpCode::CPUI_STORE
            | OpCode::CPUI_INT_ADD
            | OpCode::CPUI_INT_SUB
            | OpCode::CPUI_INT_MULT
            | OpCode::CPUI_INT_LEFT
            | OpCode::CPUI_INT_RIGHT
            | OpCode::CPUI_INT_AND
            | OpCode::CPUI_INT_OR
            | OpCode::CPUI_INT_XOR
            | OpCode::CPUI_INT_ZEXT
            | OpCode::CPUI_INT_SEXT
            | OpCode::CPUI_SUBPIECE
            | OpCode::CPUI_PIECE
            | OpCode::CPUI_PTRADD
            | OpCode::CPUI_PTRSUB
            | OpCode::CPUI_CAST
            | OpCode::CPUI_MULTIEQUAL
            | OpCode::CPUI_INDIRECT
            | OpCode::CPUI_BRANCH
    )
}

/// `true` if any op in `arm` reads one piece of a *split* hot value — a
/// `SUBPIECE` of a value `V` that is defined outside `arm` and whose other
/// pieces are read outside `arm`.
///
/// This is the return-staging hazard the fold must refuse: a 16-byte call
/// result (a Go `(ptr,len)` return staged in a scratch slot, e.g. a
/// `runtime.newobject`) whose low half threads the join while its high half is
/// consumed only by the barrier arm. Collapsing the arm leaves `V` read
/// through one piece where it was read through two, and the downstream partial
/// -value narrowing drops the preserved piece's definition — the orphaned
/// `newobject` binding and lost return the adversarial review found. Pure
/// buffer bookkeeping never splits a hot value's pieces across the arm
/// boundary, so this leaves the legitimate folds untouched; the check is a
/// refusal, never an edit, so a missed case is a kept barrier, never wrong C.
fn barrier_threads_return(data: &Funcdata, call_op: OpId, arm: BlockId) -> bool {
    // The barrier call's output is the GC buffer pointer. In a plain
    // buffer-append arm that pointer dies in the arm's stores. But when the
    // function stages a Go `(ptr,len)` return across the barrier — the common
    // `x = new(T); *x = …; return x` shape, where the write barrier guards the
    // pointer store and the object pointer is also the return value — the
    // barrier call's output threads through the join MULTIEQUAL into the
    // function's RETURN. Folding that arm collapses the phi onto the hot value
    // and the post-fold partial-value narrowing drops the return-staging
    // definition (the orphaned `runtime.newobject` and lost return the
    // adversarial review found).
    //
    // So: refuse any barrier whose output reaches a RETURN. A forward taint
    // through value plumbing and the join phis, bounded. This is a refusal,
    // never an edit — a kept barrier is correct C, only less folded.
    // Only a value-returning function can stage a return across the barrier.
    // A void function's barrier may preserve a caller register (an INDIRECT)
    // that is later a store address — legitimate, and foldable; gating here
    // keeps the pure buffer-append fold while refusing the return-staging one.
    let returns_value = data
        .obank()
        .iter_code(OpCode::CPUI_RETURN)
        .filter_map(|rid| data.obank().get(rid))
        .any(|o| o.num_input() > 1);
    if !returns_value {
        return false;
    }
    // Seed from the barrier's formal output AND its INDIRECT effects in the
    // arm (the buffer pointer usually comes back as an INDIRECT, not the
    // formal output), so the taint follows whichever varnode the join phi
    // actually threads.
    let mut seen = std::collections::HashSet::new();
    let mut stack: Vec<(VarnodeId, usize)> = Vec::new();
    if let Some(out) = data.obank().get(call_op).and_then(|o| o.get_out()) {
        stack.push((out, 0));
    }
    for aop in data.bb_ops(arm) {
        let Some(o) = data.obank().get(aop) else { continue };
        if o.code() != OpCode::CPUI_INDIRECT {
            continue;
        }
        let affects_call = o
            .get_in(1)
            .and_then(|iop| data.vbank().get(iop))
            .map(|iv| {
                crate::funcdata_varnode::op_iop_decode(iv.get_addr().get_offset()) == call_op
            })
            .unwrap_or(false);
        if affects_call {
            if let Some(out) = o.get_out() {
                stack.push((out, 0));
            }
        }
    }
    if stack.is_empty() {
        return false;
    }
    while let Some((vn, depth)) = stack.pop() {
        if depth > 24 || !seen.insert(vn) {
            continue;
        }
        let Some(v) = data.vbank().get(vn) else { continue };
        for reader in v.descend_iter() {
            let Some(r) = data.obank().get(reader) else { continue };
            if r.code() == OpCode::CPUI_RETURN {
                return true;
            }
            // The barrier-updated pointer used as a STORE *address* outside the
            // arm is the program storing through the object pointer (field
            // initialization of a staged return), not buffer bookkeeping. That
            // pointer and its stores are the return staging that folding
            // breaks; refuse. (A STORE inside the arm through this pointer is
            // the GC buffer append — allowed.)
            if r.code() == OpCode::CPUI_STORE
                && r.get_parent() != Some(arm)
                && r.get_in(1) == Some(vn)
            {
                return true;
            }
            // Follow value plumbing and phis forward to any produced value.
            match r.code() {
                OpCode::CPUI_COPY
                | OpCode::CPUI_CAST
                | OpCode::CPUI_MULTIEQUAL
                | OpCode::CPUI_INDIRECT
                | OpCode::CPUI_SUBPIECE
                | OpCode::CPUI_PIECE
                | OpCode::CPUI_INT_ZEXT
                | OpCode::CPUI_INT_SEXT
                | OpCode::CPUI_PTRADD
                | OpCode::CPUI_PTRSUB
                | OpCode::CPUI_INT_ADD => {
                    if let Some(o) = r.get_out() {
                        stack.push((o, depth + 1));
                    }
                }
                // A STORE through the buffer pointer is bookkeeping, not a
                // return path: do not follow it.
                _ => {}
            }
        }
    }
    false
}

/// Walk `vn`'s definition chain toward a pointer root: follows the base
/// operand of address arithmetic and value plumbing, bounded. `true` iff the
/// root is `root_op`'s output — the wb buffer pointer the arm's stores must
/// write through.
fn vn_roots_at_call(data: &Funcdata, mut vn: VarnodeId, root_op: OpId, arm: BlockId) -> bool {
    for _ in 0..16 {
        let Some(v) = data.vbank().get(vn) else { return false };
        let Some(def) = v.get_def() else { return false };
        let Some(op) = data.obank().get(def) else { return false };
        if def == root_op {
            return true;
        }
        // The buffer register usually comes back as the call's INDIRECT
        // effect rather than its formal output: accept an INDIRECT in the arm
        // whose affecting op (the iop input) IS the matched call.
        if op.code() == OpCode::CPUI_INDIRECT && op.get_parent() == Some(arm) {
            if let Some(iop_vn) = op.get_in(1) {
                if let Some(iv) = data.vbank().get(iop_vn) {
                    let aff = crate::funcdata_varnode::op_iop_decode(iv.get_addr().get_offset());
                    if aff == root_op {
                        return true;
                    }
                }
            }
            return false;
        }
        // Stay inside the arm: a base defined elsewhere is not the buffer.
        if op.get_parent() != Some(arm) {
            return false;
        }
        vn = match op.code() {
            OpCode::CPUI_COPY
            | OpCode::CPUI_CAST
            | OpCode::CPUI_INT_ADD
            | OpCode::CPUI_INT_SUB
            | OpCode::CPUI_PTRADD
            | OpCode::CPUI_PTRSUB => match op.get_in(0) {
                Some(base) => base,
                None => return false,
            },
            _ => return false,
        };
    }
    false
}

/// `true` if the CBRANCH condition `cond` derives (bounded) from a global
/// flag: a varnode whose storage is addressable memory (a direct global
/// read), or a LOAD through a constant pointer. A stack or register
/// condition is a program decision, never the write-barrier flag.
fn cond_reads_global_flag(data: &Funcdata, cond: VarnodeId) -> bool {
    let mut stack = vec![(cond, 0usize)];
    while let Some((vn, depth)) = stack.pop() {
        if depth > 6 {
            continue;
        }
        let Some(v) = data.vbank().get(vn) else { continue };
        if v.get_space().get_name() == "ram" {
            return true;
        }
        let Some(def) = v.get_def() else { continue };
        let Some(op) = data.obank().get(def) else { continue };
        match op.code() {
            OpCode::CPUI_LOAD => {
                if let Some(ptr) = op.get_in(1) {
                    if data.vbank().get(ptr).map(|p| p.is_constant()).unwrap_or(false) {
                        return true;
                    }
                    stack.push((ptr, depth + 1));
                }
            }
            OpCode::CPUI_COPY
            | OpCode::CPUI_CAST
            | OpCode::CPUI_BOOL_NEGATE
            | OpCode::CPUI_INT_EQUAL
            | OpCode::CPUI_INT_NOTEQUAL
            | OpCode::CPUI_INT_LESS
            | OpCode::CPUI_INT_SLESS
            | OpCode::CPUI_INT_AND
            | OpCode::CPUI_SUBPIECE
            | OpCode::CPUI_INT_ZEXT => {
                for i in 0..op.num_input() {
                    if let Some(inp) = op.get_in(i) {
                        stack.push((inp, depth + 1));
                    }
                }
            }
            _ => {}
        }
    }
    false
}

/// Match one post-SSA barrier arm whose cold-arm callee satisfies `callee`.
///
/// The invariants, each one a refusal:
/// - the CALL's block `A` is Basic with exactly ONE out-edge (the join) and
///   no other CALL of any kind;
/// - EVERY predecessor of `A` is a Basic block ending in a CBRANCH (so every
///   way in is a guard), and every guard's condition derives from a global
///   flag ([`cond_reads_global_flag`]);
/// - every op in `A` is on the bookkeeping allowlist
///   ([`wbarm_op_allowed`]), and every STORE in `A` writes through a pointer
///   rooted at the matched call's output ([`vn_roots_at_call`]) — a store to
///   program memory refuses the whole match;
/// - every value DEFINED in `A` (call output, INDIRECT effects, arm phis,
///   arithmetic) is read only inside `A` or by a MULTIEQUAL in `A`'s join —
///   so severing the arm can strand no read anywhere else.
///
/// Acting on a match = cutting the guard edges (the caller cuts one per
/// apply and re-matches): `branch_remove_internal` drops the corresponding
/// MULTIEQUAL inputs, and once the last edge goes the unreachable sweep
/// removes `A`, its join phi inputs with it.
pub fn match_guarded_cold_call_phi(
    data: &Funcdata,
    callee: &dyn Fn(&str) -> bool,
) -> Option<GuardedColdCallPhi> {
    'calls: for i in 0..data.num_calls() {
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
        if blocks.block(cold).size_out() != 1 || blocks.block(cold).size_in() < 1 {
            continue;
        }
        let join = blocks.block(cold).get_out(0);
        if join == cold {
            continue;
        }
        if block_has_other_call(data, cold, call_op) {
            continue;
        }
        // Refuse an arm that consumes one piece of a split hot value (a
        // return-staged call result): collapsing it drops the preserved
        // piece's definition (the adversarial-review miscompile).
        if barrier_threads_return(data, call_op, cold) {
            continue;
        }
        // Every way in is a global-flag guard.
        let n_in = blocks.block(cold).size_in();
        let mut guard_edges: Vec<(BlockId, int4)> = Vec::new();
        for j in 0..n_in {
            let guard = blocks.block(cold).get_in(j);
            if guard == cold || guard == join {
                continue 'calls;
            }
            if !matches!(blocks.block(guard).kind(), BlockKind::Basic(_)) {
                continue 'calls;
            }
            if blocks.block(guard).size_out() != 2 {
                continue 'calls;
            }
            let Some(tail) = data.bb_op_tail(guard) else { continue 'calls };
            let Some(tail_op) = data.obank().get(tail) else { continue 'calls };
            if tail_op.code() != OpCode::CPUI_CBRANCH {
                continue 'calls;
            }
            let Some(cond) = tail_op.get_in(1) else { continue 'calls };
            if !cond_reads_global_flag(data, cond) {
                continue 'calls;
            }
            let Some(edge) = (0..2).find(|&e| blocks.block(guard).get_out(e) == cold) else {
                continue 'calls;
            };
            guard_edges.push((guard, edge));
        }
        if guard_edges.is_empty() {
            continue;
        }
        // Bookkeeping ops only, buffer-rooted stores only, no escaping value.
        for aop in data.bb_ops(cold) {
            let Some(o) = data.obank().get(aop) else { continue 'calls };
            if aop != call_op && !wbarm_op_allowed(o.code()) {
                continue 'calls;
            }
            if o.code() == OpCode::CPUI_STORE {
                let Some(ptr) = o.get_in(1) else { continue 'calls };
                if !vn_roots_at_call(data, ptr, call_op, cold) {
                    continue 'calls;
                }
            }
            if let Some(out) = o.get_out() {
                let Some(v) = data.vbank().get(out) else { continue 'calls };
                for reader in v.descend_iter() {
                    let Some(r) = data.obank().get(reader) else { continue 'calls };
                    match r.get_parent() {
                        Some(p) if p == cold => {}
                        Some(p) if p == join && r.code() == OpCode::CPUI_MULTIEQUAL => {}
                        _ => continue 'calls,
                    }
                }
            }
        }
        return Some(GuardedColdCallPhi { guard_edges, cold_block: cold, call_op });
    }
    None
}
