//! (kuna) **gonoreturn** — apply the Go no-return list to pclntab-recovered
//! functions (`docs/multilang.md`, the Go pack's no-return coverage item).
//!
//! ## The structural gap this closes
//!
//! The known-no-return pass (`loader/noreturn.rs`, `noreturn_known`) matches
//! the vendored per-compiler lists against the object's **symbol tables**
//! (`.symtab`/`.dynsym` + import stubs). A production Go binary is stripped
//! (`-ldflags=-s -w`): both tables are gone, so the pass emits **zero** facts
//! and every entry of the vendored `GolangFunctionsThatDoNotReturn` —
//! `runtime.gopanic`, `runtime.panicshift`, `runtime.panicunsafestringlen`,
//! all of them — goes unused on exactly the binaries it was vendored for. The
//! fallout is structural, not cosmetic: a call to `runtime.panicshift` keeps
//! its fall-through edge, so the panic path renders as if the helper
//! returned, and any function whose stack-growth arm physically follows such
//! a call keeps its `morestack` loop (the `gomorestack` fold refuses the
//! broken shape — correctly).
//!
//! The names ARE still in the image: the pclntab names every function, and
//! the `gopclntab` pass already recovers them. This pass walks that same
//! table ([`super::recover_go_table`], same executable-range admission) and
//! matches each recovered `(addr, name)` against the same vendored list the
//! symbol-table pass uses ([`crate::sourcelang::golang_noreturn_list`], via
//! the same [`crate::loader::noreturn::name_matches`] reader), emitting
//! ordinary address-keyed [`NoReturnFact`]s. The commit installs the pclntab
//! names before the no-return arm resolves addresses (pass order), so the
//! facts land on the recovered functions whether or not a `.symtab` exists;
//! on an unstripped binary the facts are duplicates of `noreturn_known`'s and
//! the commit arm is idempotent.
//!
//! ## Gating
//!
//! Registered ONLY for a Go-detected binary (beside `gopclntab`, and like it
//! a structural no-op everywhere else). `option gonoreturn on|off`, default
//! **on**, gates the facts at the commit boundary (the per-pass analysis
//! gate, `Architecture::analysis_gonoreturn`); `option noreturn_known`
//! continues to gate only the symbol-table pass, so the two sources stay
//! separately flippable.

use crate::loader::noreturn::{name_matches, parse_list};
use crate::pass::{AnalysisCtx, AnalysisOutput, AnalysisPass, NoReturnFact, Phase};

use super::{exec_end, recover_go_table};

/// Apply `GolangFunctionsThatDoNotReturn` to the pclntab's own function list.
pub struct GoNoReturnPass;

impl AnalysisPass for GoNoReturnPass {
    fn phase(&self) -> Phase {
        Phase::P1
    }

    fn id(&self) -> &'static str {
        "gonoreturn"
    }

    fn run(&self, ctx: &AnalysisCtx) -> AnalysisOutput {
        let mut out = AnalysisOutput::default();
        let (exact, wildcard) = parse_list(crate::sourcelang::golang_noreturn_list());
        // The same table walk and executable-range admission as `gopclntab`:
        // a corrupt table cannot mark a no-return over data it never names.
        let exec = ctx.image.executable_ranges();
        let (funcs, _text_end) = recover_go_table(ctx.file);
        let mut seen = std::collections::HashSet::new();
        for f in funcs {
            if exec_end(&exec, f.addr).is_none() {
                continue;
            }
            if name_matches(&f.name, &exact, &wildcard) && seen.insert(f.addr) {
                out.noreturn.push(NoReturnFact { addr: f.addr, name: f.name });
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The vendored Go list reaches the names the symbol-table pass cannot see
    /// on a stripped binary — the headline runtime panic family included.
    #[test]
    fn vendored_list_matches_the_runtime_panic_family() {
        let (exact, wildcard) = parse_list(crate::sourcelang::golang_noreturn_list());
        for name in [
            "runtime.gopanic",
            "runtime.panicshift",
            "runtime.panicunsafestringlen",
            "runtime.panicunsafestringnilptr",
            "runtime.panicunsafeslicelen",
            "runtime.goPanicIndex",
            "runtime.panicmem",
            "runtime.throw",
            "runtime.goexit.abi0",
        ] {
            assert!(name_matches(name, &exact, &wildcard), "{name} must match");
        }
        // Ordinary Go functions must not.
        for name in ["main.main", "runtime.morestack_noctxt", "runtime.mallocgc", "fmt.Println"] {
            assert!(!name_matches(name, &exact, &wildcard), "{name} must not match");
        }
    }
}
