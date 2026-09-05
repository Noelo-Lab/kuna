//! (kuna) `kuna_funcboundflow` — bound fall-through at a known function entry
//! (S2 flow classification).  Informed by IDA / Ghidra, both of which bound a
//! function's decompilation to its own body and never let ordinary fall-through
//! spill into the next function.
//!
//! ## The structural gap
//!
//! kuna's flow follower ([`FlowInfo`](crate::flow)) runs with `baddr=0,
//! eaddr=~0` (the whole address space is in-bounds) and treats `CALL`/`CALLIND`
//! as fall-through: flow stops only at a `RETURN` or when the callee is *known*
//! no-return (`query_call_no_return`).  A kuna `FunctionSymbol` is an entry
//! address with **no extent**, so nothing else bounds the walk.
//!
//! When a function's last act is a `call` to a no-return routine that kuna
//! cannot prove no-return — in a stripped, statically-linked binary that means
//! the unnamed `exit`/`abort`/`__stack_chk_fail` bodies and every app-level
//! `die()`/`throw` wrapper built on them — the compiler emits the `call` with no
//! trailing `ret` (just inter-function alignment padding) and starts the next
//! function.  kuna follows the padding's fall-through straight into that next
//! function's entry and decodes **its** body into the current one, so the
//! following function is emitted twice: once correctly, and once as a garbage
//! tail of its predecessor.  Measured on a static-pie build, ~36% of the
//! application functions end in such a `call` and are corrupted this way.
//!
//! ## What this module owns
//!
//! [`kuna_should_bound_at_entry`] is the *decision* only: has fall-through
//! reached the entry of another known function (so the walk has run off the end
//! of the current function)?  The truncation itself — planting a no-return
//! artificial `RETURN` in place of the fall-through edge — is driven by
//! `flow.rs` at the fall-through-push site of `process_instruction`, mirroring
//! the `check_for_flow_modification` no-return-halt idiom.  This is the same
//! hook class as [`kuna_tailcalljump`](crate::kuna_tailcalljump) (a
//! `FlowEnvironment` predicate consulted inside the flow walk, gated by an
//! `Architecture` bool flag).
//!
//! The **gate** `glb->funcbound_flow` (`option funcboundflow`) and the
//! **entry resolution** `query_call(next).is_some()` (is `next` a known function
//! entry?) plus the **self-entry** check (`next == fd.getAddress()`) are resolved
//! by the caller (`decompile_drive.rs` / `flow.rs`) and passed in.

use kuna_base::marshal::ElementId;

/// Marshaling element `<funcboundflow>` (kuna).  ElementIds live in the 4000+
/// range (4122; `orchain` raced this id from the same reasoning and has been renumbered.
/// Do not read the previous high-water mark by hand -- `scripts.repipe.counters --check`
/// derives the next free id and fails on a duplicate).
pub const ELEM_FUNCBOUNDFLOW: ElementId = ElementId::new("funcboundflow", 4122);

/// (kuna) Has fall-through reached a foreign function entry, so the current
/// function's flow should be truncated here?
///
/// Fires iff:
///   - `gate` is on (`glb->funcbound_flow`);
///   - `next_is_known_function` — the fall-through target is the entry of a
///     known function (resolved via `query_call(next)`; includes PLT thunks);
///   - `!next_is_self` — the target is NOT the current function's own entry.
///
/// A forward fall-through only reaches a function *entry* when it has run off
/// the end of the current function (compilers never place two functions so that
/// one falls through into another's entry except after a no-return call); an
/// ordinary intra-function fall-through lands on a mid-function address where no
/// function entry sits, and so never matches.  The self-entry guard keeps a
/// (pathological) fall-through back to the current entry from truncating the
/// function against itself.
pub fn kuna_should_bound_at_entry(
    gate: bool,
    next_is_known_function: bool,
    next_is_self: bool,
) -> bool {
    // gate (opt-in / DIV default)
    if !gate {
        return false;
    }
    // the fall-through target must be another function's entry
    next_is_known_function && !next_is_self
}

#[cfg(test)]
#[path = "kuna_funcboundflow/tests.rs"]
mod tests;
