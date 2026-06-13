//! Port of `decompiler/cpp/unify.{cc,hh}` — **DEFERRED STUB** (W9).
//!
//! `unify` is the p-code pattern *unification engine* (`UnifyDatatype`,
//! `RHSConstant`, `ConstraintGroup`, `UnifyState`, `UnifyCPrinter`, …).  In
//! upstream Ghidra it backs the experimental rule-compiler: a `ConstraintGroup`
//! tree is matched against a `Funcdata` p-code graph, and `UnifyCPrinter`
//! re-emits a matched `ConstraintGroup` as the C++ source of a `Rule`.
//!
//! ## Why this is a stub
//!
//! The unification engine is reachable from exactly one place: the console
//! `parse rule` command (`IfcParseRule`), which is **conditionally compiled**
//! behind `#ifdef CPUI_RULECOMPILE` in `decompiler/cpp/ifacedecomp.cc`.  That
//! macro is **not** defined in the kuna build, so:
//!
//! - no datatest under `decompiler/datatests/` exercises it,
//! - no unit test under `decompiler/unittests/` exercises it,
//! - the `parse rule` command is never registered in the shipping binaries.
//!
//! It therefore has **zero oracle exposure**: there is nothing for the Python
//! harness or the XML stringmatch datatests to compare against.  Faithfully
//! porting the unification matcher + C-printer (~1.3k lines across
//! `unify.cc`/`rulecompile.cc`, plus the bison-style lexer in `RuleLexer`)
//! would add a large untested surface with no parity signal, so it is
//! **deferred** — a permanent-by-design candidate unless `CPUI_RULECOMPILE`
//! is ever enabled and the command gains an oracle.
//!
//! The single public entry point [`UnifyState::not_ported`] returns the LOSS
//! error so any caller (in practice only the deferred `parse rule` path)
//! reports the feature as unimplemented rather than silently misbehaving.

use kuna_base::error::{KunaError, KunaResult};

/// Stable explanatory string shared by the deferred rule-compiler surface, so
/// the `parse rule` console command (if it is ever registered) reports a
/// single, recognizable "unimplemented" message.
pub(crate) const NOT_PORTED: &str =
    "parse rule / rule-compiler not ported (deferred, no oracle exposure) — LOSS";

/// Placeholder for the upstream `UnifyState` (the live match state threaded
/// through `ConstraintGroup` evaluation and `RHSConstant::getConstant`).
///
/// The matching engine is deferred (see the module docs); this type exists
/// only to give the deferred `parse rule` path a typed, fail-closed entry
/// point.
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct UnifyState;

impl UnifyState {
    /// The unification engine is not ported.  Returns the LOSS error
    /// ([`NOT_PORTED`]) so the deferred `parse rule` command reports the
    /// feature as unimplemented.
    pub fn not_ported<T>() -> KunaResult<T> {
        Err(KunaError::lowlevel(NOT_PORTED))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unify_state_not_ported_reports_loss() {
        let err = UnifyState::not_ported::<()>().unwrap_err();
        assert_eq!(err.explain(), NOT_PORTED);
        assert!(err.is_lowlevel());
    }
}
