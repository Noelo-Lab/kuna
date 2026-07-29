//! S8 **condition-based join restructuring** (`condjoin`) — let the short-circuit
//! condition fold absorb a *multi-statement* clause block, so a guard cascade whose
//! arms reconverge on a shared body structures as one compound condition instead of
//! a forward `goto` into the sibling arm.
//!
//! # The structural gap
//!
//! lighttpd `server.c::main` (angr corpus `newbury`, `0x40f696`) opens with the
//! classic SUID guard:
//!
//! ```c
//!   if (getuid() == 0 || (geteuid() == getuid() && getegid() == getgid())) {
//!       /* BODY: setlocale(); tzset(); the server loop */
//!   } else {
//!       fputs("Are you nuts ? ...", stderr); return -1;
//!   }
//! ```
//!
//! The three guards form a `ruleBlockOr` cascade over four basic blocks:
//!
//! ```text
//!   B0: getuid();            if (== 0)          -> BODY   else B1
//!   B1: v = geteuid();       if (v == getuid()) -> B2     else B3
//!   B2: v = getegid();       if (v != getgid()) -> B3     else BODY
//!   B3: v = -1; fputs(...);  -> RET       (in-degree 2: from B1 and B2)
//!   BODY: ... server loop    -> RET       (in-degree 2: from B0 and B2)
//! ```
//!
//! Every gate of [`CollapseStructure::rule_block_or`](crate::blockaction) holds for
//! `(B1,B2)` and then for `(B0, B1||B2)` **except one**: `is_complex(orblock)`.
//! [`Funcdata::bb_is_complex`](crate::funcdata::Funcdata::bb_is_complex) (the port of
//! Ghidra `BlockBasic::isComplex`, `block.cc:2403`) calls a block complex once its
//! statement count exceeds 2, and B1/B2 each hold two calls plus the branch — three.
//! So the fold is declined, the cascade is left to the if/sequence schemas, and one
//! of the two edges into `BODY` has to be virtualized into
//! `goto label_f6ec;` + a `label_f6ec:` inside the sibling arm.  kuna emits 1 goto /
//! 1 label where angr emits 0 / 0.
//!
//! # What this option changes
//!
//! Ghidra's `isComplex` is a **readability budget, not a correctness gate**.  The
//! printer already renders a folded `BlockCondition`'s second operand under
//! [`modifiers::COMMA_SEPARATE`](crate::printc) (`PrintC::emit_block_condition` sets
//! it on `b1`), and `emit_basic_block_ops` under that modifier walks *every* op in the
//! block and emits them comma-separated with the semicolon suppressed.  Nothing is
//! dropped; a 2-statement or-block simply renders `(v = getegid(), v == getgid())`.
//! Ghidra itself exercises that path today for the 1-extra-statement case (`statement
//! > 2` admits one).
//!
//! `condjoin` therefore does not add a transform — it **relaxes the budget**, under
//! strict preconditions, exactly where relaxing it removes a goto:
//!
//! ```c
//!   if ((getuid() != 0) && ((v = geteuid(), v != getuid()) ||
//!                           (v = getegid(), v != getgid()))) {
//!       v = -1; fputs("Are you nuts ? ...", stderr);
//!   } else {
//!       setlocale(2,"C"); tzset(); do { ... } while (...);
//!   }
//!   return v;
//! ```
//!
//! 0 gotos, 0 labels.  This is the same escape hatch angr's Phoenix structurer has:
//! when `_is_single_statement_block(left)` is false,
//! `_match_acyclic_short_circuit_conditions_type_{a,b,c,d}` do not decline — they wrap
//! the operand in a `MultiStatementExpression` (angr's comma expression), gated by
//! `_should_use_multistmtexprs` / `MultiStmtExprMode` and
//! `multistmtexpr_stmt_threshold = 5`.  kuna's `COMMA_SEPARATE` *is*
//! `MultiStatementExpression`; kuna's `is_complex` *is* `_is_single_statement_block`.
//!
//! # Why this cannot reorder anything (the soundness theorem)
//!
//! The proposal (#56) framed the fix as "invert the guard chain by de Morgan and hoist
//! the join", and warned that a wrong inversion emits *wrong* C because the predicates
//! call functions (`getuid`/`geteuid`/`getegid`/`getgid`) that cannot be reordered past
//! one another.  **No such reordering is possible here, and no purity analysis is
//! needed.**  The proof is structural:
//!
//! * [`BlockGraph::new_block_condition`](crate::block) only *relabels* two blocks that
//!   are already sequenced `bl` → `orblock` in the CFG as the left and right operands
//!   of a `&&`/`||`.  It never swaps them, duplicates them, or moves an op.
//! * C's `&&`/`||` short-circuit strictly left-to-right, and the comma operator
//!   sequences strictly left-to-right.  So the emitted expression evaluates `bl`'s
//!   statements and condition, and *then*, only if the short circuit does not cut,
//!   `orblock`'s statements and condition — byte-for-byte the CFG's own order and
//!   conditionality.
//! * The only synthesized operation is `negate_condition_rec`, a boolean sense flip on
//!   an already-computed condition.  It is order-neutral.
//!
//! What this pass *does* have to guarantee is that `orblock` **renders as an
//! expression at all** — that is what the preconditions below are for.
//!
//! # Preconditions (all must hold; declining is always correct)
//!
//! In addition to every existing `rule_block_or` gate — `orblock` is single-in, 2-out,
//! non-switch, not an interior goto target, and not reached by a back edge, which
//! together already give S6 (no label can be emitted inside the expression) —
//! [`compute_condfold_blocks`] admits a `BlockBasic` only when:
//!
//! * **S2 — expression-shaped.**  Every op is a marker (skipped by the printer), an op
//!   with an output varnode, a void `CALL`/`CALLIND`, a `STORE`, or the single terminal
//!   `CBRANCH`.  Anything else — `RETURN`, a second branch, `BRANCHIND`, `CALLOTHER`
//!   (userop rendering can be multi-line/special), or any op flagged no-return —
//!   declines the block.  The last op must be that `CBRANCH`.
//! * **S3 — statement budget.**  At most [`MAX_CONDJOIN_STMTS`] *scored* statements and
//!   at most [`MAX_CONDJOIN_CALLS`] calls in the block.  A statement is scored with the
//!   *same* `Varnode::calc_explicit` approximation `bb_is_complex` uses (addr-tied /
//!   no-descend / used-outside-block / over `max_implied_ref` refs ⇒ printed), never by
//!   reading `Varnode::is_explicit()`: structuring runs before `ActionMarkExplicit`, so
//!   the flags are not yet meaningful.  This is Ghidra's own approximation and it is
//!   *not* exact in either direction — a varnode the final `ActionMarkExplicit` makes
//!   explicit can score as implied here, so a block admitted at the cap can render one
//!   statement wider than the nominal budget (bash `get_random` is the observed case:
//!   scored 2, renders 3).  That is a readability slack, never a correctness one —
//!   `COMMA_SEPARATE` emits every op either way.
//! * **S5 — no comment loss.**  `emit_basic_block_ops` skips `emit_comment_group`
//!   entirely under `COMMA_SEPARATE`, so a block carrying a comment (e.g. a
//!   "WARNING: Subroutine does not return" annotation) is declined.  Ghidra already
//!   accepts that loss for the 1-statement case; `condjoin` does not widen it.
//!
//! and the fold site additionally enforces:
//!
//! * **S4 — expression-size caps.**  The resulting `BlockCondition` may hold at most
//!   [`MAX_CONDJOIN_LEAVES`] condition leaves and at most
//!   [`MAX_CONDJOIN_TOTAL_STMTS`] conservatively-scored statements across all of them.
//!   `collapse_conditions` is a fixpoint loop with no natural bound, so without these
//!   a long cascade would fold into an unreadable monster.
//! * every leaf of a compound operand must itself be admissible (the recursion in
//!   `is_condfoldable`), because a `BlockCondition` nested as the right operand emits
//!   *both* of its sub-blocks under `COMMA_SEPARATE`.
//!
//! # Gating
//!
//! `option condjoin on|off` (`cond_join`, **default-OFF**).  When off,
//! [`compute_condfold_blocks`] is never called, the admissible-set is empty, and every
//! gate falls back to the exact upstream `is_complex` verdict — output is byte-identical
//! and the datatest corpus is untouched.

use std::collections::BTreeMap;

use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::context::{BlockId, OpId};
use crate::funcdata::Funcdata;

/// Marshaling element `<condjoin>` (kuna).  ElementIds live in the 4000+ range
/// (…4111 earlyreturn, 4112 switchreturn taken on `main`; 4113 is taken by
/// `condfold` (PR #193), developed in parallel with this one — so this is 4114).
pub const ELEM_CONDJOIN: kuna_base::marshal::ElementId =
    kuna_base::marshal::ElementId::new("condjoin", 4114);

/// Maximum conservatively-scored printed statements in a single relaxed or-block.
/// Ghidra's own budget admits 1 (its `statement > 2` counts the branch); angr's
/// `multistmtexpr_stmt_threshold` is 5.  2 is the tightened middle that admits the
/// `v = geteuid(); v == getuid()` shape and little else.
pub const MAX_CONDJOIN_STMTS: int4 = 2;

/// Maximum calls in a single relaxed or-block.  The target's clause blocks hold two
/// (`geteuid`/`getuid`, `getegid`/`getgid`), one of which inlines into the compare.
pub const MAX_CONDJOIN_CALLS: int4 = 2;

/// Maximum condition leaves in the `BlockCondition` a relaxed fold may build.  The
/// target needs 3 (`getuid` test, `geteuid` test, `getegid` test).
pub const MAX_CONDJOIN_LEAVES: int4 = 4;

/// Maximum scored statements summed over every leaf of the folded `BlockCondition`.
/// The `newbury::main` target scores 0 (both of its relaxed clause blocks reduce to
/// two calls whose outputs inline), so the cap binds only on wider folds.
pub const MAX_CONDJOIN_TOTAL_STMTS: int4 = 4;

/// Score a `bblocks` `BlockBasic` for condjoin admissibility.
///
/// Returns `Some(statement_count)` when the block satisfies S2/S3/S5 (see the module
/// docs) and may therefore be absorbed into a short-circuit condition as a
/// comma-separated multi-statement operand; `None` when any precondition fails.
///
/// The statement count uses the same `Varnode::calc_explicit` approximation as
/// [`Funcdata::bb_is_complex`] — it deliberately does **not** read
/// `Varnode::is_explicit()`/`is_implied()`, which are not yet computed when structuring
/// runs.  Like Ghidra's own use of it, the approximation is inexact in both directions;
/// see S3 in the module docs.
fn score_condfold_block(data: &Funcdata, bb: BlockId) -> Option<int4> {
    // Decline with a reason, traced to stderr under `KUNA_CONDJOIN_DEBUG=1` (run
    // `decomp_dbg` directly to see it; the `kuna` CLI drops the child's stderr).
    // Declining is always correct: the fold simply does not happen and today's
    // goto+label output stands.
    macro_rules! decline {
        ($reason:literal) => {{
            trace(bb, $reason);
            return None;
        }};
    }
    // A short-circuit clause block is a binary condition block by construction.
    if data.bblocks_ref().block(bb).size_out() != 2 {
        decline!("not a 2-out condition block");
    }
    let ops = data.bb_ops(bb);
    let tail = *ops.last()?;

    let maxref = data.get_arch().max_implied_ref;
    let mut statements: int4 = 0;
    let mut calls: int4 = 0;

    for inst in ops.iter().copied() {
        let instobj = data.obank().get(inst)?;
        // Markers (MULTIEQUAL/INDIRECT) are never printed.
        if instobj.is_marker() {
            continue;
        }
        let code = instobj.code();
        if inst == tail {
            // S2: the block must end in exactly one CBRANCH (the condition the fold
            // turns into an operand).  `size_out() == 2` makes anything else
            // impossible, but check it so this module is safe by construction rather
            // than safe by luck.
            if code != OpCode::CPUI_CBRANCH {
                decline!("tail op is not a CBRANCH");
            }
            continue;
        }
        // S2: no second control-flow op anywhere before the tail.
        if instobj.is_flow_break() {
            decline!("interior flow-break op (RETURN / second branch)");
        }
        // S2: userop rendering can be multi-line / special-cased; never absorb one.
        if code == OpCode::CPUI_CALLOTHER {
            decline!("CALLOTHER (userop rendering is not expression-safe)");
        }
        // S2: a call that does not return terminates the block; it can never be a
        // sub-expression of a condition.
        if instobj.is_call() {
            if (instobj.get_flags() & crate::op::pcodeop_flags::noreturn) != 0 {
                decline!("no-return call");
            }
            calls += 1;
            if calls > MAX_CONDJOIN_CALLS {
                decline!("over MAX_CONDJOIN_CALLS");
            }
        }
        match instobj.get_out() {
            None => {
                // S2 allowlist for void statements: a call or a STORE.  Anything else
                // without an output is not something we can vouch for inside an
                // expression.
                if !instobj.is_call() && code != OpCode::CPUI_STORE {
                    decline!("void op that is neither a call nor a STORE");
                }
                statements += 1;
            }
            Some(outvn) => {
                // Conservative Varnode::calc_explicit, mirroring
                // `Funcdata::bb_is_complex` (funcdata_block.rs).
                let vnobj = data.vbank().get(outvn)?;
                let mut printed = false;
                if vnobj.has_no_descend() || vnobj.is_addr_tied() {
                    printed = true;
                } else {
                    let mut totalref: int4 = 0;
                    for d_op in vnobj.descend_iter() {
                        let dobj = data.obank().get(d_op)?;
                        if dobj.is_marker() || dobj.get_parent() != Some(bb) {
                            printed = true;
                            break;
                        }
                        totalref += 1;
                        if totalref > maxref {
                            printed = true;
                            break;
                        }
                    }
                }
                if printed {
                    statements += 1;
                }
            }
        }
        if statements > MAX_CONDJOIN_STMTS {
            decline!("over MAX_CONDJOIN_STMTS");
        }
    }

    // S5: a comment anchored inside this block would be dropped by the
    // COMMA_SEPARATE printer path.  Decline rather than lose it.
    if block_has_comment(data, &ops) {
        decline!("carries a comment the COMMA_SEPARATE printer would drop");
    }

    Some(statements)
}

/// Trace a condjoin decline under `KUNA_CONDJOIN_DEBUG=1` (stderr; use `decomp_dbg`
/// directly, the `kuna` CLI drops the child's stderr).
fn trace(bb: BlockId, reason: &str) {
    if std::env::var_os("KUNA_CONDJOIN_DEBUG").is_some() {
        eprintln!("[condjoin] decline {bb:?}: {reason}");
    }
}

/// Trace an admitted relaxed fold under `KUNA_CONDJOIN_DEBUG=1` (stderr), naming the
/// engine that took it — `CollapseStructure` (§8.1) or `region_structurer` (§8.2).
/// Both carry the identical gate, and which one runs on a given function depends on
/// whether the region structurer converges, so this is how a witness for either half
/// is confirmed.
pub fn trace_admit_fold(engine: &str, bl: BlockId, orblock: BlockId) {
    if std::env::var_os("KUNA_CONDJOIN_DEBUG").is_some() {
        eprintln!("[condjoin] {engine}: fold {bl:?} with complex {orblock:?}");
    }
}

/// Does any buffered analysis comment anchor to an op address inside this block?
/// (S5 — `emit_basic_block_ops` skips `emit_comment_group` entirely under
/// `COMMA_SEPARATE`, so absorbing such a block would silently drop the text.)
///
/// Reads [`Funcdata::has_pending_comment_at`], the buffer the decompile drive flushes
/// into the console `Architecture`'s `CommentDatabase` — the in-pipeline `Funcdata`'s
/// `get_arch()` is the `ArchContext` seam and cannot see the database itself.  Comments
/// produced by passes that run *after* structuring (e.g. `branchflip`'s polarity note)
/// are therefore not covered; those are kuna advisory annotations, never a correctness
/// signal.
fn block_has_comment(data: &Funcdata, ops: &[OpId]) -> bool {
    if !data.has_pending_comments() {
        return false;
    }
    for inst in ops.iter().copied() {
        let Some(o) = data.obank().get(inst) else {
            continue;
        };
        if data.has_pending_comment_at(o.get_addr()) {
            return true;
        }
    }
    false
}

/// Precompute the `condjoin`-admissible `bblocks` `BlockBasic` ids, mapped to their
/// conservatively-scored printed-statement count.
///
/// Mirrors the `complex_blocks` precomputation in
/// [`ActionBlockStructure::apply`](crate::blockaction::ActionBlockStructure) /
/// [`run_region_structurer`](crate::p8_structure::region_structurer): the structuring
/// graph is a `BlockCopy` mirror without op ownership, so the verdict is computed here
/// against the live `bblocks` op lists and keyed by the bblocks id each `BlockCopy`'s
/// `copy` pointer references.
///
/// Only ever called when `option condjoin on`; with the option off both structurers keep
/// an empty map and behave exactly as before.
pub fn compute_condfold_blocks(data: &Funcdata) -> BTreeMap<BlockId, int4> {
    let mut out: BTreeMap<BlockId, int4> = BTreeMap::new();
    let nbb = data.bblocks_get_size();
    for i in 0..nbb {
        let bb = data.bblocks_get_block(i);
        if let Some(stmts) = score_condfold_block(data, bb) {
            if std::env::var_os("KUNA_CONDJOIN_DEBUG").is_some() {
                eprintln!(
                    "[condjoin] admit {:?} @ {:#x}: {} statement(s)",
                    bb,
                    data.bblocks_block_start(bb).get_offset(),
                    stmts
                );
            }
            out.insert(bb, stmts);
        }
    }
    out
}
