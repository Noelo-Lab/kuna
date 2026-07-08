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
//!   adds a per-compiler list to the match set for an ELF whose compiler is
//!   detected. The `compiler name="rustc"` arm adds `RustFunctionsThatDoNotReturn`;
//!   the `compiler id="golang"` arm adds `GolangFunctionsThatDoNotReturn`; the base
//!   `ElfFunctionsThatDoNotReturn` always applies. kuna mirrors this: when
//!   [`crate::s1_sourcelang::detect_compiler`] reports `Rustc` (resp. `Go`), the
//!   Rust wildcard list (vendored at `data/RustFunctionsThatDoNotReturn`) (resp.
//!   the Go list at `data/GolangFunctionsThatDoNotReturn`) is parsed **in addition**
//!   to the base ELF list. See [`NoReturnKnownPass::for_compiler`].
//!
//! Effect: marking `exit`/`abort`/… no-return inserts an artificial halt at the
//! call site (the engine's `flow.rs` artificialHalt path), so the dead
//! fall-through after a tail `exit()` disappears from the decompiled output. For
//! a Rust binary this additionally elides the dead code after a tail
//! `core::panicking::panic` / `handle_alloc_error` / `rust_begin_unwind`; for a Go
//! binary, after a tail `runtime.gopanic` / `runtime.throw` / `runtime.goexit`.

use object::read::{Object, ObjectSymbol};
use object::SymbolKind;

use crate::pass::{AnalysisCtx, AnalysisOutput, AnalysisPass, NoReturnFact, Phase};
use crate::s1_sourcelang::Compiler;

/// The known-no-return name list, vendored verbatim from Ghidra
/// `Ghidra/Features/Base/data/ElfFunctionsThatDoNotReturn`.
const ELF_NORETURN_LIST: &str = include_str!("../../data/ElfFunctionsThatDoNotReturn");

/// The PE (Windows-CRT) / Mach-O (macOS) no-return name list — the per-format arm
/// of `noReturnFunctionConstraints.xml` for the non-ELF executable formats. The
/// shared C names (`exit`/`abort`/…) also appear in [`ELF_NORETURN_LIST`], but a
/// PE/COFF/Mach-O selects THIS list instead of the ELF one (the base C names are
/// duplicated here so a PE still flags `exit`), plus the Windows-CRT specifics
/// (`__fastfail`/`_invoke_watson`/`quick_exit`/…) the ELF list does not name.
const PE_MAC_NORETURN_LIST: &str = include_str!("../../data/PeMacFunctionsThatDoNotReturn");

/// Port of `NoReturnFunctionAnalyzer` ("Known"): flag every imported/defined
/// function whose name matches the shipped ELF no-return list.
///
/// `compiler` selects which (if any) per-compiler no-return list is matched **in
/// addition** to the base ELF list, faithful to `noReturnFunctionConstraints.xml`'s
/// per-`<compiler>` arms for the ELF executable format:
/// - [`Compiler::Rustc`] → also match `RustFunctionsThatDoNotReturn`
///   (`compiler name="rustc"`),
/// - [`Compiler::Go`] → also match `GolangFunctionsThatDoNotReturn`
///   (`compiler id="golang"`),
/// - everything else (`Gcc`/`Clang`/`Unknown`) → base ELF list only, as Ghidra
///   does for an ELF with no per-compiler arm.
///
/// The default ([`NoReturnKnownPass::elf`]) matches only the base ELF list.
#[derive(Clone, Copy, Default)]
pub struct NoReturnKnownPass {
    /// The detected source-language compiler, selecting the extra per-compiler
    /// no-return list (Rust / Golang) to fold into the base ELF list.
    pub compiler: Compiler,
}

impl NoReturnKnownPass {
    /// The base ELF pass (no per-compiler list) — Ghidra's default for an ELF
    /// with no matching `<compiler>` arm. Equivalent to
    /// [`NoReturnKnownPass::for_compiler`] with `Gcc`/`Clang`/`Unknown`.
    pub fn elf() -> Self {
        NoReturnKnownPass { compiler: Compiler::Unknown }
    }

    /// The pass for a Rust-detected ELF: base ELF list + the Rust wildcard list.
    pub fn rust() -> Self {
        NoReturnKnownPass { compiler: Compiler::Rustc }
    }

    /// The pass for a Go-detected ELF: base ELF list + the Golang list.
    pub fn golang() -> Self {
        NoReturnKnownPass { compiler: Compiler::Go }
    }

    /// The pass for the given detected [`Compiler`] — the kuna analog of selecting
    /// the matching `<compiler>` arm of `noReturnFunctionConstraints.xml`.
    pub fn for_compiler(compiler: Compiler) -> Self {
        NoReturnKnownPass { compiler }
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
/// `compiler` selects the extra per-compiler no-return list to fold into the base
/// ELF list (faithful to `noReturnFunctionConstraints.xml`'s per-`<compiler>`
/// arms): `Rustc` → `RustFunctionsThatDoNotReturn`, `Go` →
/// `GolangFunctionsThatDoNotReturn`, everything else → base ELF list only.
fn scan_noreturn(file: &object::File, bytes: &[u8], compiler: Compiler) -> AnalysisOutput {
    let mut out = AnalysisOutput::default();
    // Per-format base list (PR-10): an ELF gets the vendored ELF list (+ the
    // per-compiler Rust/Go widening); a PE/COFF/Mach-O gets the PE/Mac list
    // (Windows-CRT + macOS no-return names, with the shared C names duplicated so
    // a PE still flags `exit`/`abort`). The names are format-neutral; only *which*
    // shipped list applies differs — the kuna analog of
    // noReturnFunctionConstraints.xml's per-executable-format arms.
    let (exact, wildcard) = match file.format() {
        object::BinaryFormat::Elf => {
            let (mut e, mut w) = parse_list(ELF_NORETURN_LIST);
            // Per-compiler source language (ELF only): append the matching list,
            // faithful to the `<compiler>` arms of noReturnFunctionConstraints.xml.
            if let Some(list) = match compiler {
                // Rust: the panic/abort/oom wildcard list.
                Compiler::Rustc => Some(crate::s1_sourcelang::rust_noreturn_list()),
                // Go: the runtime.gopanic/throw/goexit/… exact-name list.
                Compiler::Go => Some(crate::s1_sourcelang::golang_noreturn_list()),
                // No per-compiler arm for the ELF format (Gcc/Clang/Unknown).
                Compiler::Gcc | Compiler::Clang | Compiler::Unknown => None,
            } {
                let (extra_exact, extra_wildcard) = parse_list(list);
                e.extend(extra_exact);
                w.extend(extra_wildcard);
            }
            (e, w)
        }
        // PE / COFF / Mach-O: the Windows-CRT + macOS no-return list.
        object::BinaryFormat::Pe
        | object::BinaryFormat::Coff
        | object::BinaryFormat::MachO => parse_list(PE_MAC_NORETURN_LIST),
        // Any other (unsupported) format: no list, no facts.
        _ => return out,
    };
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
    for p in crate::s1_loader::format::resolve_imports(file, bytes) {
        let Ok(n) = String::from_utf8(p.name) else { continue };
        emit(&mut out, p.addr, n);
    }
    out
}

impl AnalysisPass for NoReturnKnownPass {
    fn phase(&self) -> Phase {
        Phase::P1
    }

    fn id(&self) -> &'static str {
        "noreturn_known"
    }

    fn run(&self, ctx: &AnalysisCtx) -> AnalysisOutput {
        scan_noreturn(ctx.file, ctx.bytes, self.compiler)
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

    /// (kuna DIV-21) The genuinely-unconditional no-return libc/BSD names added beyond
    /// upstream's ELF list — the `err`/`errx` family + `quick_exit`/`__assert_perror_fail`/
    /// `__chk_fail`/`__libc_fatal`. Each ALWAYS exits, so a `die()`/`fatal()` wrapper ending
    /// in one is no-return; without them its callers overrun into the next function (the
    /// same class as the `error()` boundary overrun). `warn`/`warnx` RETURN and must stay OUT.
    #[test]
    fn div21_unconditional_noreturn_family_present_warn_absent() {
        let (exact, wildcard) = sets();
        for want in ["err", "errx", "verr", "verrx", "errc", "verrc", "quick_exit"] {
            assert!(name_matches(want, &exact, &wildcard), "DIV-21: {want} must be no-return");
        }
        // underscore-stripped glibc internals
        assert!(name_matches("__assert_perror_fail", &exact, &wildcard));
        assert!(name_matches("__chk_fail", &exact, &wildcard));
        assert!(name_matches("__libc_fatal", &exact, &wildcard));
        // the warn family RETURNS — a false positive here would drop live caller code.
        for nope in ["warn", "warnx", "vwarn", "vwarnx"] {
            assert!(!name_matches(nope, &exact, &wildcard), "{nope} RETURNS; must NOT be no-return");
        }
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
    fn pe_mac_list_flags_windows_and_shared_noreturns() {
        // PR-10: the PE/Mac list flags the shared C no-returns (exit/abort/
        // quick_exit) AND the Windows-CRT specifics (__fastfail/_invoke_watson),
        // and the leading-underscore strip applies (`_exit` -> `exit`).
        let (exact, wildcard) = parse_list(PE_MAC_NORETURN_LIST);
        for want in ["exit", "abort", "quick_exit"] {
            assert!(name_matches(want, &exact, &wildcard), "PE/Mac list must flag {want}");
        }
        // The Windows-CRT fast-fail / Watson names (leading `__`/`_` stripped).
        assert!(name_matches("__fastfail", &exact, &wildcard), "must flag __fastfail");
        assert!(name_matches("_invoke_watson", &exact, &wildcard), "must flag _invoke_watson");
        assert!(name_matches("_exit", &exact, &wildcard), "_exit must strip to exit");
        // An ordinary function is never flagged.
        assert!(!name_matches("printf", &exact, &wildcard), "printf must not be flagged");
        assert!(!name_matches("main", &exact, &wildcard), "main must not be flagged");
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
        // base ELF pass (Gcc — no per-compiler list) is what fauxware (a C
        // binary) gets.
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/fauxware");
        let bytes = std::fs::read(path).expect("read fauxware fixture");
        let file = object::File::parse(bytes.as_slice()).expect("parse fauxware");
        let out = scan_noreturn(&file, &bytes, Compiler::Gcc);
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
        let out = scan_noreturn(&file, &bytes, Compiler::Gcc);

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

    /// The Golang no-return list must match a `runtime.*` panic/throw/exit symbol
    /// ONLY when the pass is in Go mode (compiler detected as Go) — never for a C
    /// ELF and never under the Rust arm. The Go gating contract, mirroring
    /// `rust_list_gated_on_rust_detection`.
    #[test]
    fn golang_list_gated_on_go_detection() {
        // A representative spread of Go runtime no-return names (the dotted form
        // Go's `.symtab`/`.gopclntab` uses — no `::`, so the namespace guard does
        // not fire; the whole string is matched against the exact set).
        let go_names = [
            "runtime.gopanic",
            "runtime.goPanicIndex",
            "runtime.throw",
            "runtime.fatalthrow",
            "runtime.goexit.abi0",
            "runtime.abort.abi0",
            "runtime.sigpanic",
            "runtime.panicmem",
        ];

        // Base ELF list (a C ELF): none of the Go runtime names are flagged.
        let (e_elf, w_elf) = parse_list(ELF_NORETURN_LIST);
        for n in go_names {
            assert!(!name_matches(n, &e_elf, &w_elf), "C ELF must not flag {n}");
        }

        // ELF + Rust list (a Rust ELF): still none of the Go runtime names match
        // (the Go list is selected only under the Go arm — the gating is
        // per-compiler, not "all extra lists at once").
        let (r_exact, r_wildcard) = parse_list(crate::s1_sourcelang::rust_noreturn_list());
        let mut e_rust = e_elf.clone();
        let mut w_rust = w_elf.clone();
        e_rust.extend(r_exact);
        w_rust.extend(r_wildcard);
        for n in go_names {
            assert!(!name_matches(n, &e_rust, &w_rust), "Rust ELF must not flag {n}");
        }

        // ELF + Golang list (a Go ELF): every runtime no-return name is flagged.
        let (g_exact, g_wildcard) = parse_list(crate::s1_sourcelang::golang_noreturn_list());
        let mut e_go = e_elf.clone();
        let mut w_go = w_elf.clone();
        e_go.extend(g_exact);
        w_go.extend(g_wildcard);
        for n in go_names {
            assert!(name_matches(n, &e_go, &w_go), "Go ELF must flag {n}");
        }
        // the leading-underscore strip still applies (a `_`-prefixed dotted name
        // is rare in Go, but the parser/matcher are uniform).
        assert!(name_matches("_runtime.gopanic", &e_go, &w_go));

        // an ordinary Go user function (not in the runtime list) is never flagged,
        // even in Go mode (no over-acceptance from the dotted-name shape).
        assert!(!name_matches("main.main", &e_go, &w_go));
        assert!(!name_matches("runtime.printlock", &e_go, &w_go));
        // the base ELF entries still apply under the Go arm (exit/abort/…).
        assert!(name_matches("exit", &e_go, &w_go));
    }

    /// The per-compiler selector wires the right list into `scan_noreturn`: the
    /// `compiler` field on `NoReturnKnownPass` picks Rust vs Go vs base. This pins
    /// the constructor contract (`for_compiler`/`golang`/`rust`/`elf`).
    #[test]
    fn compiler_field_selects_extra_list() {
        use crate::s1_sourcelang::Compiler;
        assert_eq!(NoReturnKnownPass::elf().compiler, Compiler::Unknown);
        assert_eq!(NoReturnKnownPass::rust().compiler, Compiler::Rustc);
        assert_eq!(NoReturnKnownPass::golang().compiler, Compiler::Go);
        assert_eq!(NoReturnKnownPass::for_compiler(Compiler::Go).compiler, Compiler::Go);
        // default is the base ELF pass (no per-compiler list).
        assert_eq!(NoReturnKnownPass::default().compiler, Compiler::Unknown);
    }

    // --- real-Go end-to-end (hermetic via runtime `go build`, skips if absent) ---
    //
    // Go ELF binaries are unavoidably large (~1.1 MB un-stripped — they embed the
    // whole runtime), too large to vendor comfortably; a *stripped* Go binary
    // (~750 KB) keeps `.go.buildinfo` (so detection fires) but drops `.symtab`
    // entirely (so there is no `runtime.gopanic` FUNC symbol for the matcher to
    // flag). So instead of vendoring a fixture, this test BUILDS a tiny real Go
    // program at runtime (guarded on `go` being on PATH — skips cleanly otherwise),
    // proving both halves on a genuine Go binary: detection (`Compiler::Go`) AND
    // matching (`runtime.gopanic` flagged under the Go arm, NOT under the C arm).
    // The list-parse/matching logic itself is pinned hermetically by
    // `golang_list_gated_on_go_detection` (no fixture, always runs).

    /// `true` if a runnable `go` toolchain is on PATH (so the runtime build can
    /// proceed; otherwise the e2e test is skipped, like the ARM-link follow-up).
    fn go_available() -> bool {
        std::process::Command::new("go")
            .arg("version")
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
    }

    #[test]
    fn real_go_binary_detected_and_flags_runtime_gopanic() {
        if !go_available() {
            eprintln!("skipping real-Go e2e: `go` not on PATH");
            return;
        }
        use crate::s1_sourcelang::detect_compiler;

        // Build a tiny, self-contained Go program in an isolated temp dir, with a
        // private GOCACHE/GOPATH so the build is hermetic and never touches the
        // user's environment. Un-stripped (the default) so `runtime.*` FUNC
        // symbols survive in `.symtab` for the matcher.
        let dir = std::env::temp_dir().join(format!("kuna_go_noreturn_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("mkdir temp go dir");
        std::fs::write(dir.join("main.go"), "package main\nfunc main() { println(\"hi\") }\n")
            .expect("write main.go");
        let out_bin = dir.join("go_hello");

        let status = std::process::Command::new("go")
            .args(["build", "-o"])
            .arg(&out_bin)
            .arg("main.go")
            .current_dir(&dir)
            .env("GOOS", "linux")
            .env("GOARCH", "amd64")
            .env("GOCACHE", dir.join(".gocache"))
            .env("GOPATH", dir.join(".gopath"))
            .env("CGO_ENABLED", "0")
            .status();

        // A build failure (e.g. a sandboxed `go` that can't write its cache) is a
        // skip, not a test failure — the hermetic matching contract is covered
        // elsewhere. Only proceed when we actually produced a binary.
        let built = matches!(&status, Ok(s) if s.success()) && out_bin.exists();
        if !built {
            eprintln!("skipping real-Go e2e: `go build` did not produce a binary ({status:?})");
            let _ = std::fs::remove_dir_all(&dir);
            return;
        }

        let bytes = std::fs::read(&out_bin).expect("read built go binary");
        let file = object::File::parse(bytes.as_slice()).expect("parse go binary");

        // (1) Detection: a real Go ELF must detect as `Compiler::Go`
        // (`.go.buildinfo` / `.note.go.buildid`).
        assert_eq!(detect_compiler(&file, &bytes), Compiler::Go, "go binary must detect as Go");

        // (2) Matching under the Go arm: `runtime.gopanic` (a defined FUNC in
        // `.symtab`) is flagged no-return, carrying its real code address.
        let go_out = scan_noreturn(&file, &bytes, Compiler::Go);
        let gopanic = go_out
            .noreturn
            .iter()
            .find(|f| f.name == "runtime.gopanic")
            .expect("runtime.gopanic must be flagged no-return under the Go arm");
        assert_ne!(gopanic.addr, 0, "runtime.gopanic fact must carry its code address");
        // a few more headline runtime no-return names are present too.
        for n in ["runtime.throw", "runtime.goexit.abi0"] {
            assert!(
                go_out.noreturn.iter().any(|f| f.name == n),
                "{n} must be flagged under the Go arm"
            );
        }
        // an ordinary Go user function is NOT flagged (no over-acceptance).
        assert!(
            !go_out.noreturn.iter().any(|f| f.name == "main.main"),
            "main.main must not be flagged"
        );

        // (3) The C arm (Gcc — no per-compiler list) does NOT flag the Go runtime
        // names: the widening is gated on Go detection. (The base ELF list could
        // in principle still match a name like `exit`, but `runtime.gopanic` is
        // Go-only, so this isolates the gating.)
        let c_out = scan_noreturn(&file, &bytes, Compiler::Gcc);
        assert!(
            !c_out.noreturn.iter().any(|f| f.name == "runtime.gopanic"),
            "runtime.gopanic must NOT be flagged under the C arm (Go list is gated)"
        );

        let _ = std::fs::remove_dir_all(&dir);
    }
}
