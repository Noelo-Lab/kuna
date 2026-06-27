//! Multi-format-loader PR-14 e2e gate: per-format source-language / compiler
//! detection for PE and Mach-O (design `docs/multiformat-loader-design.md` §5.3 /
//! §8 PR-14).
//!
//! `s1_sourcelang::detect_compiler` used to short-circuit to `Unknown` for any
//! non-ELF input. It now dispatches per format, so a PE / Mach-O binary's
//! toolchain is fingerprinted — and, crucially, the **format-neutral Rust/Go
//! detection paths** fire on those formats too, which is what enables the Rust/Go
//! no-return lists (`s1_loader/noreturn.rs`) and the Go pclntab pass
//! (`s1_pclntab`) on a Rust/Go PE or Mach-O.
//!
//! ## What each format detects (the headline)
//!
//! - **PE** — `pe_imports.exe` is a MinGW `x86_64-w64-mingw32-gcc` build. MinGW
//!   emits **no `.comment` section** in a PE; the `GCC: (GNU) …` toolchain records
//!   live NUL-delimited in `.rdata`. The PE arm scans `.rdata`/`.comment` ⇒
//!   `Compiler::Gcc`. (The MSVC `Rich`-header / `@comp.id` path is unit-tested
//!   hermetically in `s1_sourcelang` — no `cl.exe` fixture is buildable on Linux.)
//!
//! - **Mach-O** — `macho_imports` (x86-64) and `macho_imports_arm64` carry an
//!   `LC_BUILD_VERSION` (PLATFORM_MACOS) load command. An Apple platform ⇒ the
//!   clang/LLVM toolchain ⇒ `Compiler::Clang`.
//!
//! ## Rust/Go cross-format (the payoff the gate exists for)
//!
//! The Rust-mangled-symbol path (`_R…` / `_ZN…17h<hex>E`) and the Go build-info
//! -section path (`.go.buildinfo` / `__go_buildinfo`) are format-neutral. We
//! prove the **consequence**: when detection reports `Rustc`/`Go` (regardless of
//! which format produced the binary), `passes_for` widens the no-return set
//! (Rust) / appends the Go pclntab pass (Go). So a Rust/Go PE or Mach-O that trips
//! either neutral path automatically gets the language-specific behavior. (A real
//! Rust/Go PE/Mach-O fixture is not buildable in-container; the symbol/section
//! predicates themselves are unit-tested hermetically in `s1_sourcelang`.)
//!
//! ## Default-off / detection-only ⇒ ELF byte-identical
//!
//! Detection is pure and changes nothing unless a language-specific pass acts,
//! and those act only for `Rustc`/`Go`. A PE/Mach-O detecting as `Gcc`/`Clang`/
//! `Unknown` produces byte-identical output, and the ELF path is untouched (the
//! `s1_sourcelang` ELF tests are unchanged). This test reaches the analysis tier
//! directly (`object::File` over the fixture bytes), so it needs **no `.sla`** and
//! never skips.

use std::path::PathBuf;
use std::time::Instant;

use kuna_analysis::passes::passes_for;
use kuna_analysis::s1_sourcelang::{detect_compiler, Compiler};

fn fixtures() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .unwrap()
        .join("decompiler/crates/kuna-analysis/tests/fixtures")
}

fn read_fixture(name: &str) -> Vec<u8> {
    let path = fixtures().join(name);
    std::fs::read(&path).unwrap_or_else(|e| panic!("read fixture {name} ({path:?}): {e}"))
}

fn pass_ids(compiler: Compiler) -> Vec<&'static str> {
    // These assertions key off the COMPILER gate (gopclntab/noreturn-list widening),
    // not the format gate, so the historical ELF format is the right fixed input —
    // the Mach-O-only `objc` pass is exercised by `verify_objc.rs`.
    passes_for(compiler, object::BinaryFormat::Elf).iter().map(|p| p.id()).collect()
}

/// PE: a MinGW PE detects as `Gcc` via its `.rdata` `GCC:` records.
#[test]
fn pe_mingw_detects_as_gcc() {
    let bytes = read_fixture("pe_imports.exe");
    let file = object::File::parse(bytes.as_slice()).expect("parse PE");
    assert_eq!(file.format(), object::BinaryFormat::Pe, "fixture must be a PE");

    let t = Instant::now();
    let c = detect_compiler(&file, &bytes);
    let elapsed = t.elapsed();

    assert_eq!(c, Compiler::Gcc, "MinGW pe_imports.exe should detect as Gcc");
    assert!(!c.is_rust());
    assert!(!c.is_golang());
    eprintln!("PR-14 e2e: PE detect_compiler -> {c:?} in {elapsed:?}");
}

/// Mach-O (x86-64 + arm64): an `LC_BUILD_VERSION` (PLATFORM_MACOS) load command
/// detects as `Clang` (Apple platform ⇒ clang/LLVM).
#[test]
fn macho_detects_as_clang() {
    for f in ["macho_imports", "macho_imports_arm64"] {
        let bytes = read_fixture(f);
        let file = object::File::parse(bytes.as_slice()).expect("parse Mach-O");
        assert_eq!(file.format(), object::BinaryFormat::MachO, "{f} must be a Mach-O");

        let t = Instant::now();
        let c = detect_compiler(&file, &bytes);
        let elapsed = t.elapsed();

        assert_eq!(c, Compiler::Clang, "{f} should detect as Clang (LC_BUILD_VERSION)");
        assert!(!c.is_rust());
        assert!(!c.is_golang());
        eprintln!("PR-14 e2e: Mach-O {f} detect_compiler -> {c:?} in {elapsed:?}");
    }
}

/// The ELF arm is unchanged: the vendored Rust ELF detects as `Rustc`, a C ELF as
/// `Gcc` — exactly as before the per-format dispatch (proves the ELF byte-identity
/// invariant from the e2e side too).
#[test]
fn elf_detection_unchanged() {
    let rust = read_fixture("rust_hello_x86_64");
    let rfile = object::File::parse(rust.as_slice()).unwrap();
    assert_eq!(detect_compiler(&rfile, &rust), Compiler::Rustc, "rust ELF still Rustc");

    let c = read_fixture("fauxware");
    let cfile = object::File::parse(c.as_slice()).unwrap();
    assert_eq!(detect_compiler(&cfile, &c), Compiler::Gcc, "C ELF still Gcc");
}

/// The payoff: once detection reports `Rustc`/`Go` (on ANY format), the pass set
/// gains the language-specific behavior. This is what makes a Rust/Go PE or
/// Mach-O get the Rust/Go no-return list + Go pclntab — the gate is the
/// format-neutral `Compiler` value, not the format.
#[test]
fn rust_go_compiler_value_drives_language_passes() {
    // Go appends the pclntab pass; no other compiler carries it.
    let go = pass_ids(Compiler::Go);
    assert_eq!(go.last(), Some(&"gopclntab"), "Go compiler value appends gopclntab");
    for c in [Compiler::Gcc, Compiler::Clang, Compiler::Rustc, Compiler::Unknown] {
        assert!(
            !pass_ids(c).contains(&"gopclntab"),
            "{c:?} must not carry gopclntab"
        );
    }

    // The no-return pass is present for every compiler (its body widens for
    // Rust/Go internally — covered by the noreturn unit tests); confirm the pass
    // set is well-formed for the Rust/Go values that PE/Mach-O detection can now
    // produce.
    for c in [Compiler::Rustc, Compiler::Go] {
        assert!(pass_ids(c).contains(&"noreturn_known"), "{c:?} carries noreturn_known");
    }
}

/// The format-neutral Rust-mangled-symbol predicate is what makes a Rust PE/Mach-O
/// detect as `Rustc`. We assert it recognizes the rustc forms (the same predicate
/// `detect_compiler` runs over `file.symbols()` for every format). A real Rust
/// PE/Mach-O fixture is not buildable in-container; this proves the cross-format
/// signal hermetically.
#[test]
fn rust_symbol_path_is_format_neutral() {
    use kuna_analysis::s1_sourcelang::is_rust_mangled;
    // legacy + v0 rustc manglings
    assert!(is_rust_mangled("_ZN4core9panicking5panic17h0123456789abcdefE"));
    assert!(is_rust_mangled("_RNvCs1234_4core5panic"));
    // a plain C++ / C symbol is not Rust (so a non-Rust PE/Mach-O won't false-fire)
    assert!(!is_rust_mangled("_ZN3foo3Bar3bazEi"));
    assert!(!is_rust_mangled("printf"));
}
