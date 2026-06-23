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
//! - source-language selection: `Ghidra/Features/Base/data/noReturnFunctionConstraints.xml`
//!   adds `RustFunctionsThatDoNotReturn` to the match set for an ELF whose
//!   compiler is detected as `rustc`. kuna mirrors this: when
//!   [`crate::s1_sourcelang::detect_compiler`] reports `Rustc`, the Rust wildcard
//!   list (vendored at `data/RustFunctionsThatDoNotReturn`) is parsed **in
//!   addition** to the base ELF list. See [`NoReturnKnownPass::rust`].
//!
//! Effect: marking `exit`/`abort`/… no-return inserts an artificial halt at the
//! call site (the engine's `flow.rs` artificialHalt path), so the dead
//! fall-through after a tail `exit()` disappears from the decompiled output. For
//! a Rust binary this additionally elides the dead code after a tail
//! `core::panicking::panic` / `handle_alloc_error` / `rust_begin_unwind`.

use object::read::{Object, ObjectSymbol};
use object::SymbolKind;

use crate::pass::{AnalysisCtx, AnalysisOutput, AnalysisPass, NoReturnFact, Stage};

/// The known-no-return name list, vendored verbatim from Ghidra
/// `Ghidra/Features/Base/data/ElfFunctionsThatDoNotReturn`.
const ELF_NORETURN_LIST: &str = include_str!("../../data/ElfFunctionsThatDoNotReturn");

/// Port of `NoReturnFunctionAnalyzer` ("Known"): flag every imported/defined
/// function whose name matches the shipped ELF no-return list.
///
/// `rust` selects whether the Rust no-return wildcard list is matched **in
/// addition** to the base ELF list — set it when the source language is detected
/// as Rust (faithful to `noReturnFunctionConstraints.xml`'s `compiler
/// name="rustc"` arm). The default ([`NoReturnKnownPass::elf`]) matches only the
/// base ELF list, as Ghidra does for a non-Rust ELF.
#[derive(Clone, Copy, Default)]
pub struct NoReturnKnownPass {
    /// Also match the vendored `RustFunctionsThatDoNotReturn` wildcard list.
    pub rust: bool,
}

impl NoReturnKnownPass {
    /// The base ELF pass (no Rust list) — Ghidra's default for a non-Rust ELF.
    pub fn elf() -> Self {
        NoReturnKnownPass { rust: false }
    }

    /// The pass for a Rust-detected ELF: base ELF list + the Rust wildcard list.
    pub fn rust() -> Self {
        NoReturnKnownPass { rust: true }
    }
}

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
/// names. Emits a [`NoReturnFact`] carrying the symbol **address** alongside its
/// **original installed name** (not the stripped form) for each hit. The address
/// is the stable resolution key: the demangle pass renames the funcsym before it
/// is installed, so a mangled C++ no-return symbol (`_ZSt9terminatev`) is
/// installed as `std::terminate` — the commit resolves it by address
/// (`find_function_across_scopes`) and falls back to the name only when nothing is
/// installed at that address. Shared by [`AnalysisPass::run`] and the unit tests.
///
/// `rust` adds the vendored `RustFunctionsThatDoNotReturn` wildcard list to the
/// match set (faithful to `noReturnFunctionConstraints.xml`'s `rustc` arm).
fn scan_noreturn(file: &object::File, rust: bool) -> AnalysisOutput {
    let mut out = AnalysisOutput::default();
    // ELF-only list; only fires on ELF objects (the only format kuna loads).
    if !matches!(file.format(), object::BinaryFormat::Elf) {
        return out;
    }
    let (mut exact, mut wildcard) = parse_list(ELF_NORETURN_LIST);
    // Rust source language: append the Rust wildcard list (panic/abort/oom/…).
    if rust {
        let (r_exact, r_wildcard) = parse_list(crate::s1_sourcelang::rust_noreturn_list());
        exact.extend(r_exact);
        wildcard.extend(r_wildcard);
    }
    let mut seen = std::collections::HashSet::new();
    let mut emit = |out: &mut AnalysisOutput, addr: u64, n: String| {
        if name_matches(&n, &exact, &wildcard) && seen.insert((addr, n.clone())) {
            out.noreturn.push(NoReturnFact { addr, name: n });
        }
    };

    // Mirror exactly the three FUNC streams `loadimage_object::from_bytes`
    // installs as FunctionSymbols, so the emitted ADDRESS is the install address
    // the commit's `find_function_across_scopes` resolves against.
    //
    // 1. `.symtab` + `.dynsym` **defined** functions (`sym.address()`). A defined
    //    C++ no-return method surfaces here at its real code address. A UND import
    //    entry has `address()==0` (the funcsym install skips these too); its real
    //    FunctionSymbol comes from the PLT stub (arm 2), so skip the 0 address
    //    here rather than emit a useless `addr==0` fact.
    for sym in file.symbols().chain(file.dynamic_symbols()) {
        if sym.kind() != SymbolKind::Text {
            continue;
        }
        let addr = sym.address();
        if addr == 0 {
            continue; // UND / absolute import stub, not a code address
        }
        let Ok(n) = sym.name() else { continue };
        let Ok(n) = String::from_utf8(crate::s1_loader::elf_plt::strip_version(n.as_bytes())) else {
            continue;
        };
        emit(&mut out, addr, n);
    }

    // 2. PLT stubs (`elf_plt::resolve_plt_imports`): a dynamic import like `exit`
    //    or `_ZSt9terminatev` has a UND (`address()==0`) `.dynsym` entry — its
    //    only real FunctionSymbol is the PLT stub `elf_plt` names. Emitting the
    //    STUB address (not 0) is what lets the address path resolve a *demangled*
    //    import: `_ZSt9terminatev` is installed at the stub as `std::terminate`,
    //    so a name lookup of the raw mangled string misses, but the stub address
    //    matches. The name is matched on the raw (pre-demangle) `.dynstr` form,
    //    exactly as the `.symtab`/`.dynsym` arm does.
    for p in crate::s1_loader::elf_plt::resolve_plt_imports(file) {
        let Ok(n) = String::from_utf8(p.name) else { continue };
        emit(&mut out, p.addr, n);
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
        scan_noreturn(ctx.file, self.rust)
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
        // `exit` (a dynamic import named by elf_plt) and nothing spurious. The
        // base ELF pass (rust=false) is what fauxware (a C binary) gets.
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/fauxware");
        let bytes = std::fs::read(path).expect("read fauxware fixture");
        let file = object::File::parse(bytes.as_slice()).expect("parse fauxware");
        let out = scan_noreturn(&file, false);
        let exit = out.noreturn.iter().find(|f| f.name == "exit").expect("exit must be flagged");
        // The emitted address is the PLT-stub install address (a real code
        // address, never the UND `.dynsym` 0), so the commit resolves it by
        // address as well as by name.
        assert_ne!(exit.addr, 0, "exit fact must carry the PLT-stub install address");
        assert!(!out.noreturn.iter().any(|f| f.name == "puts"), "puts must not be flagged");
        assert!(!out.noreturn.iter().any(|f| f.name == "read"), "read must not be flagged");
    }

    /// The cross-pass seam fix: a C++ binary whose `.dynsym` carries the mangled
    /// no-return import `_ZSt9terminatev` (demangled `std::terminate`) and
    /// `__cxa_throw`. Both are UND in `.dynsym` (address 0); their real
    /// FunctionSymbols are installed at the PLT stub addresses (by `elf_plt`,
    /// then demangled). The scan must therefore emit each fact with the **stub
    /// install address** (non-zero) under its raw (pre-demangle) name, so the
    /// commit's address path resolves the renamed `std::terminate` funcsym. A
    /// name-only commit (`query_global_function("_ZSt9terminatev")`) would miss it.
    #[test]
    fn cpp_mangled_noreturn_emits_plt_stub_address() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/cpp_noreturn_x86_64");
        let bytes = std::fs::read(path).expect("read cpp_noreturn fixture");
        let file = object::File::parse(bytes.as_slice()).expect("parse cpp_noreturn");
        let out = scan_noreturn(&file, false);

        // The mangled std::terminate import is flagged under its RAW name (the
        // funcsym is later demangled to `std::terminate`; the scan matches the
        // raw `.dynstr` form against the vendored list entry `ZSt9terminatev`).
        let term = out
            .noreturn
            .iter()
            .find(|f| f.name == "_ZSt9terminatev")
            .expect("_ZSt9terminatev must be flagged no-return");
        // PINNED: the `_ZSt9terminatev@plt` stub is at 0x401070 in this fixture
        // (objdump -d -j .plt). Emitting this address (not the UND 0) is the fix.
        assert_eq!(term.addr, 0x401070, "must carry the terminate PLT-stub address");

        // __cxa_throw (raw `__cxa_throw` -> `cxa_throw` in the list) likewise.
        let throw = out
            .noreturn
            .iter()
            .find(|f| f.name == "__cxa_throw")
            .expect("__cxa_throw must be flagged no-return");
        assert_eq!(throw.addr, 0x4010a0, "must carry the __cxa_throw PLT-stub address");

        // No spurious flags (e.g. main / fail / the C++ method must not be hit).
        for spurious in ["main", "_Z4failv", "_ZN3app5Guard8throw_itEi"] {
            assert!(
                !out.noreturn.iter().any(|f| f.name == spurious),
                "{spurious} must not be flagged"
            );
        }
    }

    /// The Rust wildcard list must match a Rust panic symbol ONLY when the pass
    /// is in Rust mode (compiler detected as `rustc`) — never for a C ELF. This
    /// is the gated-list contract the sourcelang task delivers.
    #[test]
    fn rust_list_gated_on_rust_detection() {
        let panic = "ZN4core9panicking5panic17h0123456789abcdefE";

        // Base ELF list: no wildcards, so the Rust panic symbol is NOT flagged.
        let (e_elf, w_elf) = parse_list(ELF_NORETURN_LIST);
        assert!(!name_matches(panic, &e_elf, &w_elf), "C ELF must not flag a Rust panic");

        // ELF + Rust list: the `ZN4core9panicking5panic17h*` wildcard matches it.
        let (r_exact, r_wildcard) = parse_list(crate::s1_sourcelang::rust_noreturn_list());
        let mut e_rust = e_elf.clone();
        let mut w_rust = w_elf.clone();
        e_rust.extend(r_exact);
        w_rust.extend(r_wildcard);
        assert!(name_matches(panic, &e_rust, &w_rust), "Rust ELF must flag a Rust panic");
        // a couple more Rust no-return forms
        assert!(name_matches(
            "ZN5alloc5alloc18handle_alloc_error17hcafebabecafebabeE",
            &e_rust,
            &w_rust
        ));
        assert!(name_matches("rust_begin_unwind", &e_rust, &w_rust));
        // the leading-underscore strip still applies to the Rust forms
        assert!(name_matches("_ZN4core9panicking5panic17h0123456789abcdefE", &e_rust, &w_rust));

        // an ordinary Rust function (no panic/abort) is never flagged
        assert!(!name_matches("ZN5nostd1m12rusty_helper17h76f46bb3af543e7bE", &e_rust, &w_rust));
    }
}
