//! Port of `decompiler/cpp/rulecompile.{cc,hh}` — **DEFERRED STUB** (W9).
//!
//! `rulecompile` is the experimental *rule compiler*: it owns `RuleLexer` (a
//! hand-written lexer for the rule DSL), `RuleCompile` (the parser/builder that
//! assembles a [`crate::unify`] `ConstraintGroup` from a rule file), and
//! `RuleGeneric` (a user-configurable `Rule` read in from a file).  The
//! end-to-end feature lets a user write a p-code transformation rule in a small
//! DSL and have the decompiler emit the equivalent C++ `Rule` source.
//!
//! ## Why this is a stub
//!
//! `rulecompile` is reachable from exactly one place: the console `parse rule`
//! command (`IfcParseRule::execute`), which is **conditionally compiled** behind
//! `#ifdef CPUI_RULECOMPILE` in `decompiler/cpp/ifacedecomp.cc`.  That macro is
//! **not** defined in the kuna build, so the command is never registered and
//! nothing in the test corpus drives it:
//!
//! - no datatest (`tests/datatests/`) and no unit test
//!   (`decompiler/unittests/`) exercises it, and
//! - the experimental rule grammar has **no recorded oracle** for the Python
//!   harness or the XML stringmatch datatests to compare against.
//!
//! With zero oracle exposure, a faithful transcription of the lexer + parser +
//! `UnifyCPrinter` would add a large, unverifiable surface and is therefore
//! **deferred** (a permanent-by-design candidate; see [`crate::unify`]).
//!
//! The single public entry point [`RuleCompile::not_ported`] returns the LOSS
//! error so the deferred `parse rule` path reports the feature as unimplemented
//! rather than silently producing nothing.

use kuna_base::error::{KunaError, KunaResult};

use crate::unify::NOT_PORTED;

/// Placeholder for the upstream `RuleCompile` (the rule-file parser/builder).
///
/// The rule compiler is deferred (see the module docs); this type exists only
/// to give the deferred `parse rule` console command a typed, fail-closed entry
/// point.
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct RuleCompile;

impl RuleCompile {
    /// The rule compiler is not ported.  Returns the LOSS error
    /// ([`NOT_PORTED`]) so the deferred `parse rule` command reports the
    /// feature as unimplemented.
    ///
    /// This mirrors the single C++ call site
    /// (`IfcParseRule::execute`, guarded by `#ifdef CPUI_RULECOMPILE`): instead
    /// of running the lexer/parser and `UnifyCPrinter`, the kuna build fails
    /// closed here.
    pub fn not_ported<T>() -> KunaResult<T> {
        Err(KunaError::lowlevel(NOT_PORTED))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rule_compile_not_ported_reports_loss() {
        let err = RuleCompile::not_ported::<()>().unwrap_err();
        assert_eq!(err.explain(), NOT_PORTED);
        assert!(err.is_lowlevel());
    }

    // --- w9-con-rulecompile-unify verifier (adversarial) ---

    /// `RuleCompile` and `UnifyState` are two halves of the same deferred
    /// subsystem (rulecompile.cc builds a unify ConstraintGroup); the doc
    /// claims a single, recognizable "unimplemented" message. Verify both
    /// entry points yield byte-identical errors, not two drifting strings.
    #[test]
    fn w9_con_rulecompile_unify_shared_loss_message() {
        let from_rulecompile = RuleCompile::not_ported::<()>().unwrap_err();
        let from_unify = crate::unify::UnifyState::not_ported::<()>().unwrap_err();
        assert_eq!(from_rulecompile, from_unify);
        assert_eq!(from_rulecompile.explain(), NOT_PORTED);
    }
}
