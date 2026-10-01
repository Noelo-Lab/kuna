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
//!   - an exact-address `flow ... branch` override is authoritative evidence that
//!     the user wants this instruction followed intraprocedurally.  It therefore
//!     vetoes the lower-priority tail-call inference even when the destination is
//!     also a known function entry.
//!
//! ## Computed jumps with one destination
//!
//! A veneer or long-branch stub loads its target from a literal and jumps
//! through a register (`ldr r1,[pc]; bx r1`, `ldr pc,[pc,#-4]`).  Jump-table
//! recovery reads the read-only literal and yields a one-entry table.  When that
//! sole destination is another known function's entry, the jump is the same tail
//! call a direct `jmp` to it would be: [`kuna_sole_table_destination`] names the
//! destination and [`kuna_is_tail_call_table`] decides for the `CPUI_BRANCHIND`
//! under `tailcalljump on`.  The tail call is taken where following the table
//! can only reach a `halt_missing()` (the destination is outside a declared
//! extent), or where it loses nothing the copied-in body states: the callee's
//! prototype is stated (declared, DWARF, a library signature), so it fixes the
//! call's arguments, and the callee returns nothing or the caller's own output
//! is stated, so the return value reaches it.  Otherwise the flow follows the
//! table as before and the callee's body stays in the veneer.

use crate::funcdata::Funcdata;
use crate::context::OpId;
use crate::dtype::type_metatype;
use crate::fspec::{FuncProto, PrototypePieces};
use crate::jumptable::JumpTable;
use kuna_base::address::Address;
use kuna_base::error::{KunaError, KunaResult};
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

/// (kuna) The one address every entry of a recovered, non-override jump table
/// names, or `None` when the table is empty, an override, or has two targets.
pub fn kuna_sole_table_destination(jt: &JumpTable) -> Option<Address> {
    kuna_sole_destination(
        jt.is_override(),
        (0..jt.num_entries()).map(|i| jt.get_address_by_index(i)),
    )
}

/// (kuna) [`kuna_sole_table_destination`] over the table's `targets`.
pub fn kuna_sole_destination(
    is_override: bool,
    targets: impl IntoIterator<Item = Address>,
) -> Option<Address> {
    if is_override {
        return None;
    }
    let mut targets = targets.into_iter();
    let dest = targets.next()?;
    targets.all(|t| t == dest).then_some(dest)
}

/// (kuna) Parse `option tailcalljump on|direct|off` into the
/// `(tail_call_jumps, tail_call_tables)` gates: `on` recovers direct jumps and
/// one-destination computed jumps, `direct` only direct jumps.
pub fn tail_call_mode(p1: &str) -> KunaResult<(bool, bool, &'static str)> {
    match p1 {
        "" | "on" => Ok((true, true, "Tail-call jump recovery turned on")),
        "direct" => Ok((true, false, "Tail-call jump recovery limited to direct jumps")),
        "off" => Ok((false, false, "Tail-call jump recovery turned off")),
        other => Err(KunaError::parse(format!(
            "Unknown tailcalljump value: {other} (expected on|direct|off)"
        ))),
    }
}

/// (kuna) Is the computed jump `op`, whose recovered jump table has the single
/// destination `dest`, a tail call?  The gate is on, `dest` is another known
/// function's entry, and either `dest` lies outside the function's declared
/// extent or `call_states_the_body` holds (see [`tail_call_states_the_body`]).
pub fn kuna_is_tail_call_table(
    data: &Funcdata,
    op: OpId,
    gate: bool,
    dest_is_known_function: bool,
    dest_is_self: bool,
    dest_outside_extent: bool,
    call_states_the_body: impl FnOnce() -> bool,
) -> bool {
    gate
        && dest_is_known_function
        && !dest_is_self
        && data.obank().get(op).is_some_and(|o| o.code() == OpCode::CPUI_BRANCHIND)
        && (dest_outside_extent || call_states_the_body())
}

/// (kuna) What a callee's stated prototype fixes about a call to it: the
/// call's arguments, and whether the callee returns nothing.  A prototype that
/// states no return type (a `cppsig` signature: the mangling encodes none)
/// leaves the return to recovery, so it does not say the callee returns nothing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StatedCallee {
    pub returns_nothing: bool,
}

/// (kuna) The prototype pieces parked on a callee (declared, DWARF, a library
/// signature, a demangled C++ name), as a call to it gets them from
/// `ActionDefaultParams`.  A `map return` park states only the output and
/// leaves the inputs to recovery, so it states no call.  Only a stated `void`
/// return says the callee returns nothing.
pub fn stated_by_pieces(pieces: Option<&PrototypePieces>) -> Option<StatedCallee> {
    let p = pieces?;
    if p.outtype.is_none() && p.intypes.is_empty() && p.output_storage.is_some() {
        return None;
    }
    let returns_nothing =
        p.outtype.as_ref().is_some_and(|t| t.get_metatype() == type_metatype::TYPE_VOID);
    Some(StatedCallee { returns_nothing })
}

/// (kuna) The code prototype on a callee's symbol, which flow copies into the
/// call spec, when it locks the inputs.  Only a locked `void` output says the
/// callee returns nothing.
pub fn stated_by_proto(proto: Option<&FuncProto>) -> Option<StatedCallee> {
    let p = proto.filter(|p| p.is_input_locked())?;
    let returns_nothing = p.is_output_locked()
        && p.get_output_type().is_some_and(|t| t.get_metatype() == type_metatype::TYPE_VOID);
    Some(StatedCallee { returns_nothing })
}

/// (kuna) Do the pieces parked on a function lock its output when its own
/// decompile applies them?  Pieces with neither a return type nor return
/// storage (a `cppsig` signature) leave the output to recovery.
pub fn output_stated_by_pieces(pieces: Option<&PrototypePieces>) -> bool {
    pieces.is_some_and(|p| p.outtype.is_some() || p.output_storage.is_some())
}

/// (kuna) Does a tail call to `callee` state everything the callee's body,
/// copied in, would?  Its prototype fixes the call's arguments, and its return
/// value reaches the caller's: it is stated to return nothing, or the caller's
/// own output is stated.  A caller whose output is left to recovery never takes
/// a tail call's return value, so it would print `void`.
pub fn tail_call_states_the_body(callee: Option<StatedCallee>, own_output_stated: bool) -> bool {
    callee.is_some_and(|c| c.returns_nothing || own_output_stated)
}

#[cfg(test)]
#[path = "kuna_tailcalljump/tests.rs"]
mod tests;
