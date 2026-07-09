//! (kuna) `kuna_tailcalljump` — `-O2` tail-jump recognition (S2 flow
//! classification).  Inspired by angr's tail-call handling
//! (`test_decompiling_tee_O2_tail_jumps`, `setlocale_null_androidfix`).
//!
//! ## The structural gap
//!
//! At `-O2` a function whose last action is "call X then return" is compiled to a
//! direct **tail jump** — `jmp X` instead of `call X; ret`.  When `X` is an
//! external symbol the jump targets the PLT thunk (`jmp setlocale@plt`).
//!
//! kuna's flow follower ([`FlowInfo::xref_control_flow`](crate::flow)) runs with
//! `baddr=0, eaddr=~0` (the whole address space is in-bounds), so a direct
//! `CPUI_BRANCH` to the thunk is treated as ordinary *intraprocedural* flow: it
//! `new_address()`-follows the jump INTO the PLT thunk.  The thunk's body is an
//! indirect jump through the GOT (`jmp qword [rip+X]`); jump-table recovery fails
//! and `truncate_indirect_jump` rewrites it to a `CALLIND` through the GOT pointer
//! plus a `"Treating indirect jump as call"` warning.  The thunk gets *inlined*
//! instead of the `jmp` being recognized as a tail call.
//!
//! angr instead renders `return setlocale(v1, NULL)`.  kuna already resolves a
//! normal `call setlocale@plt` to `setlocale(...)` (via
//! `FlowEnvironment::query_call` over the PLT-thunk address); the tail-jump path
//! simply never consults it.
//!
//! ## What this module owns
//!
//! [`kuna_is_tail_call_branch`] is the *decision* only: is `op` a direct
//! `CPUI_BRANCH` to the entry of another known function?  The rewrite
//! (`BRANCH` → `CPUI_CALL` + an artificial `RETURN`) is driven by `flow.rs` at the
//! `CPUI_BRANCH` arm of `xref_control_flow`, mirroring the in-file
//! `truncate_indirect_jump` / `setup_callind_specs` rewrite + halt-insert idiom.
//! This is the same hook class as [`kuna_v850indbranch`](crate::kuna_v850indbranch)
//! (a `FlowEnvironment` predicate consulted inside `xref_control_flow`, gated by an
//! Architecture bool flag).
//!
//! ## STUB(W4): the gate + the symbol-table query
//!
//!   - the **gate** `glb->tail_call_jumps` (default \b false, shipped
//!     `option tailcalljump off`): modelled by [`TailCallJumpOption`], whose
//!     `default()` is the shipped off-by-default flag.
//!   - the **callee resolution** `query_call(dest)` (is `dest` a known function
//!     entry?) and the **self-entry** check (`dest == fd.getAddress()`): both are
//!     resolved by the caller (`decompile_drive.rs`, the v850 register-name
//!     convention) and passed in as `dest_is_known_function` / `dest_is_self`.

use crate::funcdata::Funcdata;
use crate::context::OpId;
use kuna_base::marshal::ElementId;
use kuna_num::opcodes::OpCode;

/// Marshaling element `<tailcalljump>` (kuna).  ElementIds live in the 4000+
/// range (next free above `gotoreduce`'s 4100 in the 40xx block: 4101).
pub const ELEM_TAILCALLJUMP: ElementId = ElementId::new("tailcalljump", 4101);

/// (kuna) Toggle `-O2` tail-jump recognition: `tailcalljump on|off`.
///
/// STUB(W4): the flag is carried as a plain `bool` whose [`Default`] mirrors the
/// *shipped* default (`option tailcalljump off`, i.e. \b false — kept opt-in
/// default-off because default-on regresses 2 datatests (`Long double #1/#2`),
/// matching `Architecture::reset_defaults_internal`).  W9's option dispatch flips
/// `Architecture::tail_call_jumps`; [`apply`](TailCallJumpOption::apply)
/// transcribes the `onOrOff(p1)` body.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TailCallJumpOption {
    /// True if a direct `jmp` to another function's entry is recovered as a tail
    /// call (`Architecture::tail_call_jumps`).
    pub enabled: bool,
}

impl Default for TailCallJumpOption {
    /// Shipped default: `option tailcalljump off` (kept opt-in because default-on
    /// regresses 2 datatests, `Long double #1/#2`; `option tailcalljump on`
    /// recovers the tail call and logs the introduced call).
    fn default() -> Self {
        TailCallJumpOption { enabled: false }
    }
}

impl TailCallJumpOption {
    /// (kuna) Set the gate.
    pub fn apply(&mut self, val: bool) -> &'static str {
        self.enabled = val;
        if val {
            "Tail-call jump recovery turned on"
        } else {
            "Tail-call jump recovery turned off"
        }
    }

    /// Read the gate (`glb->tail_call_jumps`).
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

/// (kuna) Is `op` a direct `jmp` that should be recovered as a tail call?
///
/// Fires iff:
///   - `gate` is on (`glb->tail_call_jumps`);
///   - `op->code() == CPUI_BRANCH` (a direct, non-indirect jump);
///   - `dest_is_known_function` — the branch target is the entry of a known
///     function (resolved via `query_call(dest)`; this includes PLT thunks);
///   - `!dest_is_self` — the target is NOT the current function's own entry
///     (self-tail-recursion is left as an ordinary back-edge to keep the CFG
///     surgery narrow).
///
/// A direct branch to another function's *entry* is, by definition, a tail call;
/// ordinary intraprocedural jumps target mid-function addresses (no function
/// entry there) and so never match.
///
/// `dest_is_known_function` / `dest_is_self` are the already-resolved
/// symbol-table queries (the v850 register-name STUB convention; `decompile_drive`
/// wires the real `query_call` / `fd.getAddress()` calls).  // STUB(W4)
pub fn kuna_is_tail_call_branch(
    data: &Funcdata,
    op: OpId,
    gate: bool,
    dest_is_known_function: bool,
    dest_is_self: bool,
) -> bool {
    // gate (default-off opt-in)
    if !gate {
        return false;
    }
    // The target must be another function's entry (a tail call), not the current
    // function and not a mid-function intraprocedural jump.
    if !dest_is_known_function || dest_is_self {
        return false;
    }
    let opref = match data.obank().get(op) {
        Some(o) => o,
        None => return false,
    };
    // a direct jump only
    if opref.code() != OpCode::CPUI_BRANCH {
        return false;
    }
    true
}

#[cfg(test)]
#[path = "kuna_tailcalljump/tests.rs"]
mod tests;
