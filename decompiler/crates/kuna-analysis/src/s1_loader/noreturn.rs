//! No-return known-function detection — the kuna analog of Ghidra's
//! `NoReturnFunctionAnalyzer` ("Non-Returning Functions - Known").
//!
//! Ghidra ships **two** no-return analyzers:
//!
//! - **Known** (`NoReturnFunctionAnalyzer`): name-based. Every symbol whose name
//!   (leading `_`s stripped) matches a shipped list is marked no-return. This is
//!   what we port here.
//! - **Discovered** (`FindNoReturnFunctionsAnalyzer`): flow/heuristic. Counts
//!   CALL-with-no-fallthrough "indications" over the disassembled listing and
//!   marks no-return once the count reaches an evidence threshold (3). This is
//!   **infeasible at the kuna-analysis tier** — it needs the disassembled
//!   Listing / code blocks / references / FlowOverride that only exist *after*
//!   decompilation, which the analyzer tier (object view only) does not have. It
//!   is documented here as a deferred seam; see `docs/missing-analyses.md`.
//!
//! Origin (upstream Ghidra, the tree kuna was ported from):
//! - matcher: `Ghidra/Features/Base/.../analysis/NoReturnFunctionAnalyzer.java`
//!   (the `added` loop + `makeNoReturnFunction` namespace guard).
//! - name set: `Ghidra/Features/Base/data/ElfFunctionsThatDoNotReturn`, vendored
//!   verbatim at `decompiler/crates/kuna-analysis/data/ElfFunctionsThatDoNotReturn`.
//!
//! Effect: marking `exit`/`abort`/… no-return inserts an artificial halt at the
//! call site (the engine's `flow.rs` artificialHalt path), so the dead
//! fall-through after a tail `exit()` disappears from the decompiled output.

use object::read::{Object, ObjectSymbol};
use object::SymbolKind;

use crate::pass::{AnalysisCtx, AnalysisOutput, AnalysisPass, Stage};

/// The known-no-return name list, vendored verbatim from Ghidra
/// `Ghidra/Features/Base/data/ElfFunctionsThatDoNotReturn`.
const ELF_NORETURN_LIST: &str = include_str!("../../data/ElfFunctionsThatDoNotReturn");

/// Port of `NoReturnFunctionAnalyzer` ("Known"): flag every imported/defined
/// function whose name matches the shipped ELF no-return list.
pub struct NoReturnKnownPass;

/// Strip **all** leading underscores (faithful to
/// `NoReturnFunctionAnalyzer.java:84-91` — a loop, not a single strip: so
/// `__stack_chk_fail` -> `stack_chk_fail` and `_exit` -> `exit`).
fn strip_leading_underscores(s: &str) -> &str {
    s.trim_start_matches('_')
}

/// Split a (possibly namespaced) symbol name into `(base, namespace)`.
/// kuna installs `::`-split namespaces, so a demangled `Menu::exit` has base
/// `exit` in namespace `Menu`; a flat C import `exit` has no namespace.
fn base_and_namespace(name: &str) -> (&str, Option<&str>) {
    match name.rfind("::") {
        Some(i) => (&name[i + 2..], Some(&name[..i])),
        None => (name, None),
    }
}

/// Parse the vendored list into `(exact, wildcard_prefixes)` (mirrors the Java
/// reader, `NoReturnFunctionAnalyzer.java:220-245`): skip `#`/blank lines, strip
/// leading `_`, a trailing `*` makes a prefix (wildcard) entry. The shipped ELF
/// list has no wildcards (those are Rust/Golang only), but the parse is faithful.
fn parse_list(text: &str) -> (Vec<String>, Vec<String>) {
    let mut exact = Vec::new();
    let mut wildcard = Vec::new();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let line = strip_leading_underscores(line);
        if let Some(prefix) = line.strip_suffix('*') {
            wildcard.push(prefix.to_string());
        } else {
            exact.push(line.to_string());
        }
    }
    (exact, wildcard)
}

/// `true` if `name` (after stripping leading `_` from its base) matches the list,
/// honoring the namespace guard (only global / `std` — never a C++ class method
/// like `Menu::_exit`; faithful to `makeNoReturnFunction`, NoReturnFunctionAnalyzer.java:121-132).
fn name_matches(name: &str, exact: &[String], wildcard: &[String]) -> bool {
    let (base, ns) = base_and_namespace(name);
    // Namespace guard: skip class methods. Global (no namespace) is allowed; the
    // sole allowed non-global namespace is exactly `std`.
    if let Some(ns) = ns {
        if ns != "std" {
            return false;
        }
    }
    let stripped = strip_leading_underscores(base);
    if stripped.is_empty() {
        return false;
    }
    exact.iter().any(|e| e == stripped) || wildcard.iter().any(|w| stripped.starts_with(w.as_str()))
}

/// Scan an ELF object's FUNC symbols (`.symtab` + `.dynsym`) for known no-return
/// names. Emits the **original installed name** (not the stripped form) for each
/// hit so the commit's `query_global_function` resolves the FunctionSymbol that
/// actually exists. Shared by [`AnalysisPass::run`] and the unit tests.
fn scan_noreturn(file: &object::File) -> AnalysisOutput {
    let mut out = AnalysisOutput::default();
    // ELF-only list; only fires on ELF objects (the only format kuna loads).
    if !matches!(file.format(), object::BinaryFormat::Elf) {
        return out;
    }
    let (exact, wildcard) = parse_list(ELF_NORETURN_LIST);
    let mut seen = std::collections::HashSet::new();
    // The same FUNC streams `loadimage_object::from_bytes` installs as
    // FunctionSymbols: a dynamic import like `exit` exists only in `.dynsym` /
    // as a PLT stub named by `elf_plt`, so both streams must be scanned.
    for sym in file.symbols().chain(file.dynamic_symbols()) {
        if sym.kind() != SymbolKind::Text {
            continue;
        }
        let Ok(n) = sym.name() else { continue };
        let Ok(n) = String::from_utf8(crate::s1_loader::elf_plt::strip_version(n.as_bytes())) else {
            continue;
        };
        if name_matches(&n, &exact, &wildcard) && seen.insert(n.clone()) {
            out.noreturn.push(n);
        }
    }
    out
}

impl AnalysisPass for NoReturnKnownPass {
    fn stage(&self) -> Stage {
        Stage::S1
    }

    fn id(&self) -> &'static str {
        "noreturn_known"
    }

    fn run(&self, ctx: &AnalysisCtx) -> AnalysisOutput {
        scan_noreturn(ctx.file)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sets() -> (Vec<String>, Vec<String>) {
        parse_list(ELF_NORETURN_LIST)
    }

    #[test]
    fn list_parses_to_exact_names_no_wildcards() {
        let (exact, wildcard) = sets();
        assert!(wildcard.is_empty(), "ELF list ships no wildcards");
        for want in ["exit", "abort", "stack_chk_fail", "assert_fail", "pthread_exit"] {
            assert!(exact.iter().any(|e| e == want), "missing {want}");
        }
        // leading underscores are stripped at parse time
        assert!(!exact.iter().any(|e| e.starts_with('_')));
    }

    #[test]
    fn strips_all_leading_underscores() {
        let (exact, wildcard) = sets();
        assert!(name_matches("exit", &exact, &wildcard));
        assert!(name_matches("abort", &exact, &wildcard));
        assert!(name_matches("pthread_exit", &exact, &wildcard));
        // single and double leading underscores both strip to a listed name
        assert!(name_matches("_exit", &exact, &wildcard));
        assert!(name_matches("__stack_chk_fail", &exact, &wildcard)); // -> stack_chk_fail
        assert!(name_matches("__assert_fail", &exact, &wildcard)); // -> assert_fail
    }

    #[test]
    fn does_not_flag_unlisted_names() {
        let (exact, wildcard) = sets();
        for n in ["puts", "read", "main", "authenticate", "printf", "strcmp"] {
            assert!(!name_matches(n, &exact, &wildcard), "{n} must not be flagged");
        }
    }

    #[test]
    fn namespace_guard_blocks_class_methods_but_allows_std() {
        let (exact, wildcard) = sets();
        // A C++ class method `Menu::_exit` must NOT be flagged (the guard).
        assert!(!name_matches("Menu::_exit", &exact, &wildcard));
        // `std`-namespaced names are the sole allowed non-global namespace.
        assert!(name_matches("ZSt9terminatev", &exact, &wildcard));
        assert!(name_matches("std::ZSt9terminatev", &exact, &wildcard));
    }

    #[test]
    fn run_over_fauxware_flags_exit_only() {
        // Drive the full scan over the vendored fauxware bytes: it must surface
        // `exit` (a dynamic import named by elf_plt) and nothing spurious.
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/fauxware");
        let bytes = std::fs::read(path).expect("read fauxware fixture");
        let file = object::File::parse(bytes.as_slice()).expect("parse fauxware");
        let out = scan_noreturn(&file);
        assert!(out.noreturn.iter().any(|n| n == "exit"), "exit must be flagged");
        assert!(!out.noreturn.iter().any(|n| n == "puts"), "puts must not be flagged");
        assert!(!out.noreturn.iter().any(|n| n == "read"), "read must not be flagged");
    }
}
