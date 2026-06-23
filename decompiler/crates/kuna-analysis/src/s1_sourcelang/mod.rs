//! Source-language / compiler detection — the kuna analog of Ghidra's
//! `SourceLanguageAnalyzer` + the per-language `SourceLanguage.existsIn`
//! extension points.
//!
//! In the Ghidra application `SourceLanguageAnalyzer.added` runs early (priority
//! `FORMAT_ANALYSIS.before()`), calls `SourceLanguageService.find(program)` to
//! collect the IDs whose `existsIn` predicate is true, and records them on the
//! program; downstream language-specific analyzers (the Rust string/demangle
//! analyzers, the Go markup, …) then gate on those IDs. kuna ports the
//! **detection half** (the `existsIn` predicates) and uses the result to shape
//! pass selection — concretely, to widen the no-return list for Rust binaries
//! (see [`rust_noreturn_list`] and `s1_loader/noreturn.rs`). The Ghidra
//! spec-extension / `.gdt`-data-archive application machinery (`addSpecExtensions`,
//! `getDataArchives`) is **out of scope** — kuna has no SpecExtension subsystem.
//! This is therefore a PARTIAL port (detection only), in the same spirit as the
//! demangle pass's name-only scope (`s1_demangle/mod.rs`).
//!
//! Origin (upstream Ghidra, the tree kuna was ported from):
//! - analyzer shell:
//!   `Ghidra/Features/Base/.../analysis/SourceLanguageAnalyzer.java`
//! - dispatcher:
//!   `Ghidra/Framework/SoftwareModeling/.../sourcelanguage/SourceLanguageService.java`
//!   (`find()` = iterate `SourceLanguage`s, collect IDs whose `existsIn` is true).
//! - Rust detection: `Ghidra/Features/Rust/.../sourcelanguage/ElfRustSourceLanguage.java:34`
//!   (`ELF_COMMENT_REGEX = "^rustc version .*$"` over `.comment`, then a `.rodata`
//!   byte-signature fallback) + `RustSourceLanguage.java` (`isRust` rodata scan,
//!   `RUST_ID = "Rust"`) + `RustConstants.java:29-33`
//!   (`RUST_SIGNATURES = {"RUST_BACKTRACE","RUST_MIN_STACK","/rustc/"}`).
//! - Golang detection: `Ghidra/Features/Base/.../golang/GoBuildInfo.java:45`
//!   (`.go.buildinfo`) + `NoteGoBuildId.java:30` (`.note.go.buildid`);
//!   `GolangElfInfoProducer.java:45-47` keys markup off exactly those two sections.
//!
//! # Faithfulness notes
//!
//! - The `^rustc version .*$` regex reduces to a line/record-prefix test: the
//!   `.comment` section is a sequence of NUL-delimited records, each a whole
//!   toolchain line, so an anchored "starts with `rustc version `" match is
//!   exact (no regex crate needed — same move as `noreturn.rs`'s `*` wildcards).
//! - The `Gcc`/`Clang` arms are a **kuna convenience**, not faithful Ghidra IDs:
//!   Ghidra ships only Rust/Swift/ObjC `SourceLanguage` extension points for ELF,
//!   there is no generic "C/GCC" one. They are derived from the same `.comment`
//!   toolchain string and exist only to give a non-Rust/Go ELF a meaningful
//!   [`Compiler`] instead of `Unknown`; nothing downstream gates on them. They are
//!   documented here so a future faithfulness audit is not misled.

use object::read::{Object, ObjectSection, ObjectSymbol};
use object::SymbolKind;

/// The Rust no-return wildcard list, vendored verbatim from Ghidra
/// `Ghidra/Features/Base/data/RustFunctionsThatDoNotReturn` (the
/// `compiler name="rustc"` arm of `noReturnFunctionConstraints.xml`). Applied
/// **in addition** to `ElfFunctionsThatDoNotReturn` when [`detect_compiler`]
/// reports [`Compiler::Rustc`].
const RUST_NORETURN_LIST: &str = include_str!("../../data/RustFunctionsThatDoNotReturn");

/// The Golang no-return list, vendored verbatim from Ghidra
/// `Ghidra/Features/Base/data/GolangFunctionsThatDoNotReturn` (the
/// `compiler id="golang"` arm of `noReturnFunctionConstraints.xml`). Applied
/// **in addition** to `ElfFunctionsThatDoNotReturn` when [`detect_compiler`]
/// reports [`Compiler::Go`]. Unlike the Rust list these are all **exact**
/// dotted `runtime.*` names (no `*` wildcards); the same list parser handles them.
const GOLANG_NORETURN_LIST: &str = include_str!("../../data/GolangFunctionsThatDoNotReturn");

/// The Rust `.rodata` byte signatures, ported verbatim from
/// `RustConstants.java:29-33` (`RustConstants.RUST_SIGNATURES`). Used as the
/// fallback detection path when the `.comment` `rustc version` record is absent
/// (e.g. a non-standard toolchain or an aggressively post-processed binary).
const RUST_SIGNATURES: [&[u8]; 3] = [b"RUST_BACKTRACE", b"RUST_MIN_STACK", b"/rustc/"];

/// ELF section names Ghidra's Golang ELF markup keys off
/// (`GoBuildInfo.java:45` / `NoteGoBuildId.java:30`).
const GO_SECTIONS: [&str; 2] = [".go.buildinfo", ".note.go.buildid"];

/// The detected source-language toolchain.
///
/// `Rustc` and `Go` are faithful Ghidra `SourceLanguage` IDs (Rust/Golang).
/// `Gcc`/`Clang` are kuna conveniences derived from the `.comment` toolchain
/// string (Ghidra has no generic C SourceLanguage). `Unknown` is the default
/// when no signal is present (matching `SourceLanguageService.find` returning an
/// empty set).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Compiler {
    /// GCC (from a `GCC: (...)` `.comment` record) — kuna convenience.
    Gcc,
    /// Clang/LLVM (from a `clang version ...` `.comment` record) — kuna convenience.
    Clang,
    /// rustc (faithful `RustSourceLanguage.RUST_ID`).
    Rustc,
    /// Go (faithful — `.go.buildinfo` / `.note.go.buildid` present).
    Go,
    /// No recognized toolchain signal.
    #[default]
    Unknown,
}

impl Compiler {
    /// `true` for [`Compiler::Rustc`] — the gate the no-return widening uses
    /// (mirrors `noReturnFunctionConstraints.xml`'s `compiler name="rustc"` arm).
    pub fn is_rust(self) -> bool {
        matches!(self, Compiler::Rustc)
    }

    /// `true` for [`Compiler::Go`].
    pub fn is_golang(self) -> bool {
        matches!(self, Compiler::Go)
    }
}

/// Detect the compiler/source-language that produced `file`.
///
/// Faithful precedence (a binary may carry several toolchain records; we report
/// the most specific source language):
/// 1. **Go** — `.go.buildinfo` / `.note.go.buildid` section present
///    (`GolangElfInfoProducer`).
/// 2. **Rust** — `.comment` has a `rustc version ...` record
///    (`ElfRustSourceLanguage` comment path), OR a Rust-mangled symbol is present
///    (`_R...` v0 / `_ZN...17h<hex>E` legacy — the loader's own demangle gate),
///    OR `.rodata` carries a [`RUST_SIGNATURES`] signature
///    (`RustSourceLanguage.isRust` rodata path).
/// 3. **Clang** — `.comment` has a `clang version ...` record (kuna convenience).
/// 4. **Gcc** — `.comment` has a `GCC: (...)` record (kuna convenience).
/// 5. **Unknown** otherwise.
///
/// Pure over `file`; non-ELF input always returns [`Compiler::Unknown`]
/// (matching `ElfRustSourceLanguage.existsIn` requiring the ELF format).
pub fn detect_compiler(file: &object::File) -> Compiler {
    // ELF-only, matching `existsIn`'s `getExecutableFormat().equals(ELF_NAME)`.
    if !matches!(file.format(), object::BinaryFormat::Elf) {
        return Compiler::Unknown;
    }

    if golang_section_present(file.sections().filter_map(|s| s.name().ok())) {
        return Compiler::Go;
    }

    // Read the `.comment` section once; it backs both the Rust and the C/C++ arms.
    let comment = file
        .section_by_name(".comment")
        .and_then(|s| s.data().ok())
        .map(<[u8]>::to_vec)
        .unwrap_or_default();

    // Rust: `.comment` `rustc version`, then mangled-symbol heuristic, then
    // `.rodata` byte signatures — three faithful detection paths.
    if comment_indicates_rust(&comment)
        || symbols_indicate_rust(file)
        || rodata_indicates_rust(file)
    {
        return Compiler::Rustc;
    }

    // C/C++ conveniences (kuna additions; not faithful Ghidra SourceLanguage IDs).
    if comment_contains(&comment, b"clang version ") {
        return Compiler::Clang;
    }
    if comment_contains(&comment, b"GCC: ") {
        return Compiler::Gcc;
    }

    Compiler::Unknown
}

/// `true` if any NUL-delimited `.comment` record starts with `"rustc version "`
/// (faithful reduction of `ElfRustSourceLanguage`'s `^rustc version .*$` regex —
/// the records are line-delimited, so an anchored prefix match is exact).
pub fn comment_indicates_rust(comment: &[u8]) -> bool {
    comment_records(comment).any(|r| record_text(r).starts_with("rustc version "))
}

/// `true` if `.comment` contains a record beginning with `needle` (the kuna
/// convenience used for the `clang`/`GCC` arms). `needle` is matched at the
/// **start** of a NUL-delimited record (a `GCC: (...)` / `clang version ...`
/// line), not anywhere in the blob.
fn comment_contains(comment: &[u8], needle: &[u8]) -> bool {
    let needle = core::str::from_utf8(needle).unwrap_or("");
    comment_records(comment).any(|r| record_text(r).starts_with(needle))
}

/// Iterate the NUL-delimited records of a `.comment` blob, skipping empties.
fn comment_records(comment: &[u8]) -> impl Iterator<Item = &[u8]> {
    comment.split(|&b| b == 0).filter(|r| !r.is_empty())
}

/// Lossy-UTF8 view of a `.comment` record, with surrounding whitespace trimmed
/// (some toolchains pad records).
fn record_text(record: &[u8]) -> &str {
    core::str::from_utf8(record).unwrap_or("").trim()
}

/// `true` if any ELF symbol name is a Rust-mangled symbol: the v0 scheme
/// (`_R...`) or the legacy scheme (`_ZN...17h<16 hex>E`). This is the
/// mangled-symbol heuristic — robust on a non-stripped Rust binary even when the
/// `.comment` record was removed. Mirrors what the loader's demangle gate
/// already recognizes (`rustc_demangle`).
fn symbols_indicate_rust(file: &object::File) -> bool {
    file.symbols()
        .chain(file.dynamic_symbols())
        .filter(|s| s.kind() == SymbolKind::Text)
        .filter_map(|s| s.name().ok())
        .any(is_rust_mangled)
}

/// `true` if `name` is a Rust-mangled symbol name. Recognizes:
/// - **v0**: `_R...` (or `__R...` with a platform leading underscore).
/// - **legacy**: `_ZN...17h<hex16>E` — the Itanium-`_ZN` form rustc emits with a
///   trailing 16-hex-digit `17h…` hash component, which distinguishes it from a
///   plain C++ `_ZN…` symbol.
pub fn is_rust_mangled(name: &str) -> bool {
    // v0: leading `_R` (allow one extra platform underscore: `__R`).
    let v0 = name.strip_prefix('_').unwrap_or(name);
    if v0.starts_with('R') && v0.len() > 1 {
        return true;
    }
    // legacy: a `17h<16 hex>E` hash tail anywhere after a `_ZN` prefix.
    let zn = name.strip_prefix('_').unwrap_or(name);
    let zn = zn.strip_prefix('_').unwrap_or(zn);
    if !zn.starts_with("ZN") {
        return false;
    }
    if let Some(pos) = name.rfind("17h") {
        let tail = &name[pos + 3..];
        // The hash is exactly 16 lowercase hex digits, then the trailing `E`.
        if let Some(hex) = tail.strip_suffix('E') {
            return hex.len() == 16 && hex.bytes().all(|b| b.is_ascii_hexdigit());
        }
    }
    false
}

/// `true` if the `.rodata` section carries any [`RUST_SIGNATURES`] byte signature
/// (`RustSourceLanguage.isRust` fallback path). A plain substring scan — the kuna
/// analog of `MemoryBytePatternSearcher` over literal byte sequences (no pattern
/// engine needed; the LOSS vs Ghidra's searcher is none for literals).
fn rodata_indicates_rust(file: &object::File) -> bool {
    let Some(data) = file.section_by_name(".rodata").and_then(|s| s.data().ok()) else {
        return false;
    };
    RUST_SIGNATURES.iter().any(|sig| contains_subslice(data, sig))
}

/// `true` if any section name in the iterator is a Golang marker section
/// (`.go.buildinfo` / `.note.go.buildid`). Factored over a `&str` iterator so it
/// is unit-testable without a Go fixture.
pub fn golang_section_present<'a>(names: impl Iterator<Item = &'a str>) -> bool {
    let mut names = names;
    names.any(|n| GO_SECTIONS.contains(&n))
}

/// Simple substring search (`haystack.contains(needle)` over bytes).
fn contains_subslice(haystack: &[u8], needle: &[u8]) -> bool {
    if needle.is_empty() || needle.len() > haystack.len() {
        return needle.is_empty();
    }
    haystack.windows(needle.len()).any(|w| w == needle)
}

/// The Rust no-return wildcard list text, for the no-return pass to parse and add
/// to its match set when the compiler is detected as Rust. The text is the
/// vendored `RustFunctionsThatDoNotReturn` (Ghidra's `compiler name="rustc"`
/// arm); it is parsed by `noreturn.rs`'s existing list parser (the `*`-suffix
/// wildcard form, e.g. `ZN4core9panicking5panic17h*`).
pub fn rust_noreturn_list() -> &'static str {
    RUST_NORETURN_LIST
}

/// The Golang no-return list text, for the no-return pass to parse and add to its
/// match set when the compiler is detected as Go. The text is the vendored
/// `GolangFunctionsThatDoNotReturn` (Ghidra's `compiler id="golang"` arm); it is
/// parsed by the same `noreturn.rs` list parser — the entries are all exact
/// dotted names (`runtime.gopanic`, `runtime.throw`, `runtime.goexit.abi0`, …)
/// with no `*` wildcards, so they land in the parser's `exact` set.
pub fn golang_noreturn_list() -> &'static str {
    GOLANG_NORETURN_LIST
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- pure predicate tests (hermetic, no fixture) ---

    #[test]
    fn comment_rust_vs_gcc_vs_clang() {
        // A real rustc `.comment` blob (NUL-delimited records, exactly the shape
        // `readelf -p .comment` shows on a rustc binary).
        let rust_comment =
            b"rustc version 1.90.0 (1159e78c4 2025-09-14)\0Linker: LLD 20.1.8\0".as_slice();
        assert!(comment_indicates_rust(rust_comment));

        // GCC / clang `.comment`s must NOT read as Rust.
        let gcc = b"GCC: (Ubuntu 11.4.0-1ubuntu1~22.04.3) 11.4.0\0".as_slice();
        assert!(!comment_indicates_rust(gcc));
        assert!(comment_contains(gcc, b"GCC: "));

        let clang = b"clang version 18.1.3 (1ubuntu1)\0".as_slice();
        assert!(!comment_indicates_rust(clang));
        assert!(comment_contains(clang, b"clang version "));

        // empty / no-comment is not Rust.
        assert!(!comment_indicates_rust(b""));
    }

    #[test]
    fn rust_mangled_name_recognition() {
        // legacy `_ZN...17h<hex16>E`
        assert!(is_rust_mangled("_ZN4core9panicking5panic17h0123456789abcdefE"));
        assert!(is_rust_mangled("_ZN5nostd1m12rusty_helper17h76f46bb3af543e7bE"));
        // v0 `_R`
        assert!(is_rust_mangled("_RNvCs1234_4core5panic"));
        // a plain C++ `_ZN...` (no `17h<hex>E` hash) is NOT Rust
        assert!(!is_rust_mangled("_ZN3foo3Bar3bazEi"));
        assert!(!is_rust_mangled("_ZSt9terminatev"));
        // not a mangled name at all
        assert!(!is_rust_mangled("main"));
        assert!(!is_rust_mangled("puts"));
        // a wrong-length hash tail is rejected
        assert!(!is_rust_mangled("_ZN4core5panic17hdeadE"));
    }

    #[test]
    fn golang_section_predicate() {
        assert!(golang_section_present(
            [".text", ".go.buildinfo", ".symtab"].iter().copied()
        ));
        assert!(golang_section_present(
            [".note.go.buildid", ".text"].iter().copied()
        ));
        assert!(!golang_section_present([".text", ".comment"].iter().copied()));
        assert!(!golang_section_present(std::iter::empty()));
    }

    #[test]
    fn rodata_signature_scan() {
        // a `.rodata`-shaped blob carrying `/rustc/` must read as Rust.
        let blob = b"....hello/rustc/1.90.0/library/core....".as_slice();
        assert!(RUST_SIGNATURES.iter().any(|s| contains_subslice(blob, s)));
        assert!(contains_subslice(blob, b"/rustc/"));
        assert!(!contains_subslice(b"no rust here", b"RUST_BACKTRACE"));
        // empty-needle / oversize-needle edge cases
        assert!(contains_subslice(b"x", b""));
        assert!(!contains_subslice(b"x", b"longer"));
    }

    #[test]
    fn rust_noreturn_list_parses_with_wildcards() {
        let text = rust_noreturn_list();
        // sanity: the vendored list carries the headline panic/abort wildcards.
        assert!(text.contains("ZN4core9panicking5panic17h*"));
        assert!(text.contains("ZN5alloc5alloc18handle_alloc_error17h*"));
        assert!(text.contains("rust_begin_unwind"));
    }

    #[test]
    fn golang_noreturn_list_carries_runtime_names() {
        let text = golang_noreturn_list();
        // sanity: the vendored Golang list carries the headline runtime no-return
        // names (all exact, dotted, no `*` wildcards).
        for want in [
            "runtime.gopanic",
            "runtime.goPanicIndex",
            "runtime.goexit.abi0",
            "runtime.fatalthrow",
            "runtime.throw",
            "runtime.abort",
            "runtime.sigpanic",
        ] {
            assert!(text.contains(want), "Golang list missing {want}");
        }
        // it is NOT the Rust list (no `*` wildcards / no `ZN…panic` forms).
        assert!(!text.contains("ZN4core9panicking5panic17h*"));
    }

    // --- fixture-backed tests (the real vendored rust binary) ---

    fn fixture(name: &str) -> Vec<u8> {
        let path = format!("{}/tests/fixtures/{}", env!("CARGO_MANIFEST_DIR"), name);
        std::fs::read(&path).unwrap_or_else(|e| panic!("read fixture {name}: {e}"))
    }

    #[test]
    fn detects_rustc_on_rust_fixture() {
        let bytes = fixture("rust_hello_x86_64");
        let file = object::File::parse(bytes.as_slice()).expect("parse rust fixture");
        assert_eq!(detect_compiler(&file), Compiler::Rustc);
        assert!(detect_compiler(&file).is_rust());
        // both the `.comment` path and the mangled-symbol path fire on this fixture.
        let comment = file
            .section_by_name(".comment")
            .and_then(|s| s.data().ok())
            .unwrap_or_default();
        assert!(comment_indicates_rust(comment), ".comment rustc path");
        assert!(symbols_indicate_rust(&file), "mangled-symbol path");
    }

    #[test]
    fn detects_gcc_on_c_fixtures() {
        for f in ["fauxware", "cpp_mangled_x86_64"] {
            let bytes = fixture(f);
            let file = object::File::parse(bytes.as_slice()).expect("parse C fixture");
            let c = detect_compiler(&file);
            assert_eq!(c, Compiler::Gcc, "{f} should detect as Gcc");
            assert!(!c.is_rust(), "{f} must not detect as Rust");
            assert!(!c.is_golang(), "{f} must not detect as Go");
        }
    }
}
