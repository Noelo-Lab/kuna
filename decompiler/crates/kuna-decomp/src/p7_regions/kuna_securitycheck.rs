//! (kuna `securitycheck`) Strip rustc's bounds / slice / divide-by-zero panic
//! branches — the port of SEFCOM Oxidizer's `SecurityCheckRemover`
//! (angr `angr/rust/optimization_passes/security_check_remover.py`, whose work
//! lives in the `CFAMixin::match_call` / `CFGTransformationMixin::remove_block`
//! mixins it composes).
//!
//! ## What it strips
//!
//! rustc emits a check before every slice index, every string slice and every
//! `/`/`%` by a non-constant divisor.  Each one compiles to a conditional
//! branch to a tiny block that sets up a panic-location argument, calls one
//! `core::panicking` / `core::slice::index` / `core::str` helper, and diverges
//! (the helper never returns; the block ends in a trap):
//!
//! ```text
//!   if (len <= idx)
//!       core::panicking::panic_bounds_check(idx, len, &loc);   // no-return
//!   v = base[idx];
//! ```
//!
//! Every such branch doubles the CFG of ordinary Rust code and carries no
//! information a reader wants — Oxidizer reports the removal as the single
//! biggest CFG-size win of its whole pass set.  With the branch gone the guarded
//! access is a plain statement and the surrounding structure collapses.
//!
//! ## Why the trigger is a name, and why that keeps it inert on C
//!
//! Unlike the sibling [`kuna_stackguard`](crate::kuna_stackguard) pass — whose
//! callee is an unnamed PLT stub, so the canary has to be pinned by its
//! *compare* — every helper here is a **Rust-only symbol**.  Matching the callee
//! name is therefore both the faithful port (angr matches the same seven paths)
//! and what makes the pass structurally inert on a C binary: a C program has no
//! `panic_bounds_check`, so the scan finds nothing and the Action returns 0.
//! That is what lets it default ON with no compiler-detection channel from the
//! loader to the engine (no such channel exists, and this pass does not build
//! one).
//!
//! ## Name shapes
//!
//! The recovered callee name is whatever the demangler produced, so the match
//! has to tolerate three shapes of the same symbol:
//!
//! * the full path, `core::slice::index::slice_end_index_len_fail`;
//! * the path with generic arguments still attached,
//!   `core::slice::index::slice_start_index_len_fail<usize>` — so the match is
//!   on the path PREFIX before the first `<`, never on string equality;
//! * a name kuna installed under a shallower scope, down to the bare leaf
//!   `panic_bounds_check`, which is what the x86-64 ELF path actually produces
//!   today.
//!
//! [`is_security_check_name`] accepts a name when, after the generic-argument
//! and legacy-`::h<hash>` trim, it is one of the seven paths **or a trailing
//! `::`-component suffix of one**.  A suffix match is safe here because each of
//! the seven leaves (`panic_bounds_check`, `slice_error_fail`,
//! `panic_const_div_by_zero`, `panic_const_rem_by_zero`,
//! `slice_start_index_len_fail`, `slice_end_index_len_fail`,
//! `slice_index_order_fail`) is a reserved-looking rustc-internal identifier; a
//! sibling such as `core::str::slice_error_fail_rt` is a DIFFERENT leaf and does
//! not match.
//!
//! ## The safety envelope
//!
//! Removing a branch changes the CFG the structurer sees, so the failure arm is
//! only removed when it can carry nothing else:
//!
//! * the guard is a `CBRANCH` in a basic block with exactly two out-edges;
//! * the failure successor is a **basic** block that **diverges** —
//!   `size_out() == 0`.  That is the load-bearing condition: a block with no
//!   out-edges contributes nothing to the enclosing function's returns, so
//!   deleting it cannot change the recovered prototype or return value;
//! * the failure block contains **exactly one** call, and that call's callee
//!   name is one of the seven.  A second call (something the failure path also
//!   does) rejects the block;
//! * a block that is also reachable another way is never orphaned: the edit
//!   only severs THIS edge, and `removeUnreachableBlocks` collects the block
//!   only when no predecessor is left.
//!
//! ## The CFG surgery
//!
//! Identical to `kuna_stackguard`'s: `Funcdata::remove_branch` (CBRANCH →
//! fall-through, MULTIEQUALs patched) followed by
//! `Funcdata::remove_unreachable_blocks` to collect the orphaned panic block.
//! The panic block's argument setup — the location `lea`, the index/len copies,
//! the stack spills — dies with it through the ordinary `ActionDeadCode`
//! fixpoint on the next `fullloop` iteration.
//!
//! ## Why P7 and this schedule slot
//!
//! The pass lives in `p7_regions/` next to `kuna_stackguard` because it is the
//! same *kind* of operation on a different trigger: a pre-structuring removal of
//! one CFG edge plus its orphaned diverging handler, expressed with the same two
//! block-graph primitives, scheduled in the same `fullloop` tail so that
//! `mainloop`'s dead-code and type passes re-run over the simplified graph and
//! the structurer (P8) only ever sees the reduced CFG.  angr schedules its
//! version at `BEFORE_VARIABLE_RECOVERY`; kuna cannot edit blocks that early
//! (call specs and the `bblocks` graph are what the edit needs), and the
//! repeating `fullloop` gives the same effect — every later phase re-derives
//! from the reduced graph.
//!
//! ## The option
//!
//! `option securitycheck on|off`, default **on** (DIV-82).  The live flag is
//! [`Architecture::strip_security_check`](crate::architecture::Architecture),
//! mirrored onto [`ArchContext`](crate::context::ArchContext) because the Action
//! reads it through `data.get_arch()`.

use kuna_num::opcodes::OpCode;

use crate::action::{Action, ActionBase, ActionContext, ActionGroupList, ApplyResult};
use crate::block::BlockKind;
use crate::context::{BlockId, OpId};
use crate::funcdata::Funcdata;
use kuna_base::types::int4;

/// The rustc panic helpers a security check branches to (the seven paths of
/// Oxidizer's `SECURITY_CHECK_FUNCTIONS`, `security_check_remover.py:8`).
pub const SECURITY_CHECK_FUNCTIONS: &[&str] = &[
    "core::panicking::panic_bounds_check",
    "core::str::slice_error_fail",
    "core::panicking::panic_const::panic_const_div_by_zero",
    "core::panicking::panic_const::panic_const_rem_by_zero",
    "core::slice::index::slice_start_index_len_fail",
    "core::slice::index::slice_end_index_len_fail",
    "core::slice::index::slice_index_order_fail",
];

/// Reduce a recovered callee name to the comparable path: drop everything from
/// the first `<` (a generic-argument list or a `<T as Trait>` qualifier is not
/// part of the path) and drop a trailing legacy-mangling `::h<16 hex>`
/// component if the demangler left one on.
fn callee_path(name: &str) -> &str {
    let name = match name.find('<') {
        Some(i) => name[..i].trim_end_matches(':'),
        None => name,
    };
    match name.rfind("::h") {
        Some(i) if name.len() - i == 19 && name[i + 3..].bytes().all(|b| b.is_ascii_hexdigit()) => {
            &name[..i]
        }
        _ => name,
    }
}

/// Is `name` one of the seven rustc security-check helpers?
///
/// Accepts the full path or any trailing `::`-component suffix of it (see the
/// module docs on name shapes); rejects a longer leaf such as
/// `slice_error_fail_rt`.
pub fn is_security_check_name(name: &str) -> bool {
    let path = callee_path(name);
    if path.is_empty() {
        return false;
    }
    SECURITY_CHECK_FUNCTIONS.iter().any(|full| {
        *full == path || (full.len() > path.len() + 2 && full.ends_with(path) && {
            let cut = full.len() - path.len();
            full.as_bytes()[cut - 2] == b':' && full.as_bytes()[cut - 1] == b':'
        })
    })
}

/// The callee name at a CALL-family op, as recovered at flow time
/// (`FuncCallSpecs::get_name`, the same string the printer emits).
fn callee_name(op: OpId, data: &Funcdata) -> Option<&str> {
    let idx = data.get_call_specs_index(op)?;
    let nm = data.get_call_specs(idx).get_name();
    if nm.is_empty() {
        None
    } else {
        Some(nm)
    }
}

/// Does basic block `bb` consist of exactly one call, to a rustc security-check
/// helper?  (angr `CFAMixin::match_call` over the block's terminal call, plus
/// the "nothing else happens here" guard the mixin gets for free from AIL's
/// one-statement panic blocks.)
fn block_is_security_check_handler(bb: BlockId, data: &Funcdata) -> bool {
    let mut matched = false;
    for op in data.bb_ops(bb) {
        let oc = data.obank().get(op).expect("securitycheck: op").code();
        if !matches!(
            oc,
            OpCode::CPUI_CALL | OpCode::CPUI_CALLIND | OpCode::CPUI_CALLOTHER
        ) {
            continue;
        }
        if matched {
            return false; // a second call: the arm does more than panic
        }
        match callee_name(op, data) {
            Some(name) if is_security_check_name(name) => matched = true,
            _ => return false,
        }
    }
    matched
}

/// (kuna) Strip rustc bounds/overflow panic branches
/// (angr Oxidizer `SecurityCheckRemover`).
///
/// When `option securitycheck on` (the default), finds a CBRANCH whose failure
/// successor is a diverging block that calls one of the seven rustc panic
/// helpers, and removes that edge plus the now-orphaned handler.  Inert
/// (returns 0) when the option is off or the function has no such branch —
/// which is every function of every C binary.
pub struct ActionRemoveSecurityCheck {
    base: ActionBase,
    /// Unit-test OR-override of the `strip_security_check` gate (the scheduled
    /// instance registers `false` and reads the live flag off the ArchContext,
    /// exactly as `ActionStripStackGuard` does).
    enabled: bool,
}

impl ActionRemoveSecurityCheck {
    /// `enabled` forces the pass on regardless of the architecture gate (unit
    /// tests); the scheduled instance passes `false`.
    pub fn new(enabled: bool, g: impl Into<String>) -> ActionRemoveSecurityCheck {
        ActionRemoveSecurityCheck { base: ActionBase::new(0, "removesecuritycheck", g), enabled }
    }
}

impl Action for ActionRemoveSecurityCheck {
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
        Some(Box::new(ActionRemoveSecurityCheck {
            base: self.base.clone(),
            enabled: self.enabled,
        }))
    }

    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        if !self.enabled && !data.get_arch().strip_security_check {
            return 0; // P0 assertion not set
        }
        let size = data.bblocks_get_size();
        for i in 0..size {
            let h = data.bblocks_get_block(i);
            if !matches!(data.bblocks_ref().block(h).kind(), BlockKind::Basic(_)) {
                continue;
            }
            if data.bblocks_ref().block(h).size_out() != 2 {
                continue;
            }
            let cb = match data.bb_op_tail(h) {
                Some(op) => op,
                None => continue,
            };
            if data.obank().get(cb).expect("securitycheck: cb").code() != OpCode::CPUI_CBRANCH {
                continue;
            }
            // Either out-edge may be the panic arm (rustc emits both polarities);
            // the arm is identified by what it contains, not by the branch sense.
            let mut victim: Option<int4> = None;
            for j in 0..2 {
                let out = data.bblocks_ref().block(h).get_out(j);
                if !matches!(data.bblocks_ref().block(out).kind(), BlockKind::Basic(_)) {
                    continue;
                }
                // A diverging arm: no out-edge, so nothing downstream — and in
                // particular no return — depends on it.
                if data.bblocks_ref().block(out).size_out() != 0 {
                    continue;
                }
                if block_is_security_check_handler(out, data) {
                    victim = Some(j);
                    break;
                }
            }
            let idx = match victim {
                Some(j) => j,
                None => continue,
            };
            data.remove_branch(h, idx).expect("ActionRemoveSecurityCheck: removeBranch");
            data.remove_unreachable_blocks(false, true)
                .expect("ActionRemoveSecurityCheck: removeUnreachableBlocks");
            self.base.count += 1;
            // One check per apply; the fullloop re-invokes and self-gates.
            return 1;
        }
        0
    }
}

/// (kuna) Toggle rustc security-check branch stripping.
///
/// `on` (default, DIV-82) removes the bounds/slice/divide-by-zero panic
/// branches; `off` keeps them.  The resolved value is written into
/// [`Architecture::strip_security_check`](crate::architecture::Architecture).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SecurityCheckOption {
    /// True when rustc panic-check branches are stripped.
    pub enabled: bool,
}

impl Default for SecurityCheckOption {
    fn default() -> Self {
        SecurityCheckOption { enabled: true }
    }
}

impl SecurityCheckOption {
    /// Apply the option.
    pub fn apply(&mut self, val: bool) -> String {
        self.enabled = val;
        let prop = if val { "on" } else { "off" };
        format!("Rust security-check branch stripping turned {prop}")
    }

    /// Read the gate.
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

#[cfg(test)]
#[path = "kuna_securitycheck/tests.rs"]
mod tests;
