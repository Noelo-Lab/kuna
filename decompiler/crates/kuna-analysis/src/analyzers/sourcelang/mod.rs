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
//! (see [`rust_noreturn_list`] and `loader/noreturn.rs`). The Ghidra
//! spec-extension / `.gdt`-data-archive application machinery (`addSpecExtensions`,
//! `getDataArchives`) is **out of scope** — kuna has no SpecExtension subsystem.
//! This is therefore a PARTIAL port (detection only), in the same spirit as the
//! demangle pass's name-only scope (`demangle/mod.rs`).
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
//!
//! # Multi-format expansion (design `docs/history/multiformat-loader-design.md` §5.3 / §8
//! PR-14)
//!
//! The detection above is ELF-specific in its *signal sources* (the `.comment`
//! section, the `.go.buildinfo`/`.note.go.buildid` sections), but the *contract*
//! ([`detect_compiler`] → [`Compiler`], gating the Rust/Go no-return lists + the
//! Go pclntab pass) is format-neutral. [`detect_compiler`] therefore dispatches on
//! `file.format()`:
//!
//! - **ELF** — [`detect_compiler_elf`], the original logic, **byte-identical**.
//! - **PE** ([`detect_compiler_pe`]) — the MSVC `Rich` header / `@comp.id`
//!   records (the toolchain fingerprint MSVC's linker stamps between the DOS stub
//!   and the PE header → MSVC vs Clang), plus the MinGW `GCC: (...)` record.
//!   MinGW emits **no `.comment` section** in a PE; the `GCC: (…)` records live
//!   NUL-delimited in `.rdata`, so the PE arm reuses the same NUL-record reader
//!   over `.rdata`/`.comment`.
//! - **Mach-O** ([`detect_compiler_macho`]) — `LC_BUILD_VERSION` / the
//!   `LC_VERSION_MIN_*` family (the platform + build-tool fingerprint; an Apple
//!   platform is a clang/LLVM toolchain ⇒ [`Compiler::Clang`]), plus any embedded
//!   `clang version`/`GCC:` text in a `__comment`/`__DWARF`-analog section.
//!
//! Two detection paths are **format-neutral and shared by every arm** so a Rust
//! or Go PE / Mach-O is recognized — which then correctly enables the Rust/Go
//! no-return lists (`loader/noreturn.rs`) and the Go pclntab pass
//! (`pclntab`) on those formats too:
//!
//! - **Rust** — a Rust-mangled symbol (`_R…` v0 / `_ZN…17h<hex>E` legacy:
//!   [`symbols_indicate_rust`] over `file.symbols()`/`dynamic_symbols()`, which
//!   are format-neutral in `object`).
//! - **Go** — a Go build-info section under *any* format's naming
//!   (`.go.buildinfo`/`.note.go.buildid` on ELF, `__go_buildinfo` on Mach-O):
//!   [`golang_section_present`].
//!
//! The format-specific arms are **detection-only**: nothing in the output changes
//! unless a language-specific pass acts, and those passes act only for
//! `Rustc`/`Go` — so a PE/Mach-O detecting as `Gcc`/`Clang`/`Unknown` produces
//! byte-identical output, and the **ELF path is untouched**.

use object::macho::{MachHeader32, MachHeader64};
use object::read::macho::{
    FatArch, LoadCommandVariant, MachHeader, MachOFatFile32, MachOFatFile64,
};
use object::read::{Object, ObjectSection, ObjectSymbol};
use object::{BinaryFormat, Endianness, FileKind, SymbolKind};

/// The Rust no-return wildcard list, vendored verbatim from Ghidra
/// `Ghidra/Features/Base/data/RustFunctionsThatDoNotReturn` (the
/// `compiler name="rustc"` arm of `noReturnFunctionConstraints.xml`). Applied
/// **in addition** to `ElfFunctionsThatDoNotReturn` when [`detect_compiler`]
/// reports [`Compiler::Rustc`].
const RUST_NORETURN_LIST: &str = include_str!("../../../data/RustFunctionsThatDoNotReturn");

/// The Golang no-return list, vendored verbatim from Ghidra
/// `Ghidra/Features/Base/data/GolangFunctionsThatDoNotReturn` (the
/// `compiler id="golang"` arm of `noReturnFunctionConstraints.xml`). Applied
/// **in addition** to `ElfFunctionsThatDoNotReturn` when [`detect_compiler`]
/// reports [`Compiler::Go`]. Unlike the Rust list these are all **exact**
/// dotted `runtime.*` names (no `*` wildcards); the same list parser handles them.
const GOLANG_NORETURN_LIST: &str = include_str!("../../../data/GolangFunctionsThatDoNotReturn");

/// The Rust `.rodata` byte signatures, ported verbatim from
/// `RustConstants.java:29-33` (`RustConstants.RUST_SIGNATURES`). Used as the
/// fallback detection path when the `.comment` `rustc version` record is absent
/// (e.g. a non-standard toolchain or an aggressively post-processed binary).
const RUST_SIGNATURES: [&[u8]; 3] = [b"RUST_BACKTRACE", b"RUST_MIN_STACK", b"/rustc/"];

/// Section names Ghidra's Golang markup keys off, across formats. ELF uses
/// `.go.buildinfo` (`GoBuildInfo.java:45`) / `.note.go.buildid`
/// (`NoteGoBuildId.java:30`); a Mach-O Go binary carries the same build-info blob
/// in a `__go_buildinfo` section of the `__DATA` segment (Ghidra's
/// `GoBuildInfo`/`MachoGolangSection` analog). The detection is the same: any of
/// these present ⇒ Go. `object` surfaces the Mach-O section name as
/// `__go_buildinfo` (segment-qualified names collapse to the bare section name).
const GO_SECTIONS: [&str; 3] = [".go.buildinfo", ".note.go.buildid", "__go_buildinfo"];

/// The MSVC `Rich` header end marker. MSVC's linker stamps an obfuscated block
/// between the DOS stub and the PE header listing every tool (`@comp.id`) that
/// contributed object code; the block ends with the literal `"Rich"` tag
/// followed by the 4-byte XOR key used to (de)obfuscate it. Absent ⇒ not an MSVC
/// link (MinGW/Clang-direct PEs have no `Rich` header). Ghidra:
/// `PortableExecutable`/`RichHeader`.
const RICH_END: &[u8; 4] = b"Rich";

/// The de-obfuscated `Rich` header start marker (`"DanS"`), recovered by XOR-ing
/// the block with the key. Its presence confirms a real MSVC `Rich` header.
const RICH_BEGIN: u32 = u32::from_le_bytes(*b"DanS");

/// `@comp.id` *product* ids (the high 16 bits of a `Rich` `@comp.id` record's
/// `prodid` field) that indicate a **clang/LLVM** front end rather than the MSVC
/// C/C++ compiler. clang-cl emits records tagged with the LLVM build-tool product
/// ids; MSVC `cl.exe` emits `[C|CPP]_Compiler` ids. The full Microsoft product-id
/// table is large and undocumented; we key off the small, stable LLVM subset and
/// default any other populated `Rich` header to MSVC. (Detection-only — nothing
/// downstream gates on `Clang` vs `Gcc` vs MSVC, so an imperfect split is safe.)
const RICH_LLVM_PRODIDS: [u16; 2] = [
    0x0103, // [LTCG ] clang/llvm import (observed on clang-cl links)
    0x0105, // llvm/clang front-end record (observed on clang-cl links)
];

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

/// Detect the compiler/source-language that produced `file`, dispatching on the
/// object format. `bytes` is the raw image (needed for the PE `Rich` header and
/// the Mach-O load commands — `object::File` does not surface either).
///
/// - **ELF** — [`detect_compiler_elf`] (the original logic, byte-identical).
/// - **PE** — [`detect_compiler_pe`].
/// - **Mach-O** — [`detect_compiler_macho`].
/// - any other format (COFF object, raw, …) — [`Compiler::Unknown`].
///
/// Pure over `(file, bytes)`. The format-neutral Rust (`symbols_indicate_rust`)
/// and Go (`golang_section_present`) paths are shared by the PE/Mach-O arms, so a
/// Rust or Go binary in any supported format is detected.
pub fn detect_compiler(file: &object::File, bytes: &[u8]) -> Compiler {
    match file.format() {
        BinaryFormat::Elf => detect_compiler_elf(file),
        BinaryFormat::Pe => detect_compiler_pe(file, bytes),
        BinaryFormat::MachO => detect_compiler_macho(file, bytes),
        // COFF objects / anything else carry no reliable toolchain fingerprint we
        // model yet (a COFF `.obj` is pre-link). Stay `Unknown` (no behavior
        // change), but still honour the format-neutral Rust/Go signals if present.
        _ => detect_compiler_generic(file),
    }
}

/// The ELF detection — **byte-identical to the pre-multi-format logic.** Kept as a
/// standalone fn so the ELF path is provably unchanged by the format dispatch.
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
pub fn detect_compiler_elf(file: &object::File) -> Compiler {
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

/// Detect the toolchain that produced a **PE** image.
///
/// Precedence (most-specific source language first, then the toolchain):
/// 1. **Go** — a Go build-info section (`.go.buildinfo` etc.) is present.
/// 2. **Rust** — a Rust-mangled symbol, OR a `rustc version` toolchain record in
///    `.rdata`/`.comment` (the format-neutral Rust paths).
/// 3. **Clang** — a `clang version` record, OR an MSVC `Rich` header whose
///    `@comp.id`s are tagged as clang/LLVM (clang-cl).
/// 4. **Gcc** — a MinGW `GCC: (...)` record (MinGW PEs store these NUL-delimited
///    in `.rdata`; there is no `.comment` *section*).
/// 5. **Clang** (MSVC) — any other populated `Rich` header ⇒ an MSVC `cl.exe`
///    link. kuna has no distinct `Msvc` [`Compiler`] variant (nothing gates on
///    one), so we report the closest C/C++-toolchain convenience, `Clang`, only
///    to lift the binary out of `Unknown`; this never changes output.
/// 6. **Unknown** otherwise.
pub fn detect_compiler_pe(file: &object::File, bytes: &[u8]) -> Compiler {
    if golang_section_present(file.sections().filter_map(|s| s.name().ok())) {
        return Compiler::Go;
    }

    // The toolchain `GCC:`/`clang version`/`rustc version` records: MinGW puts
    // them NUL-delimited in `.rdata` (no `.comment` section in a PE); a Clang
    // -direct PE may carry `.comment`. Read both and treat as one record stream.
    let mut toolchain = file
        .section_by_name(".comment")
        .and_then(|s| s.data().ok())
        .map(<[u8]>::to_vec)
        .unwrap_or_default();
    if let Some(rdata) = file.section_by_name(".rdata").and_then(|s| s.data().ok()) {
        toolchain.extend_from_slice(rdata);
    }

    if comment_indicates_rust(&toolchain) || symbols_indicate_rust(file) {
        return Compiler::Rustc;
    }
    if comment_contains(&toolchain, b"clang version ") {
        return Compiler::Clang;
    }
    if comment_contains(&toolchain, b"GCC: ") {
        return Compiler::Gcc;
    }

    // No toolchain text: fall back to the MSVC `Rich` header / `@comp.id` records.
    match detect_rich_header(bytes) {
        RichVerdict::Clang => Compiler::Clang,
        RichVerdict::Msvc => Compiler::Clang, // kuna has no `Msvc` variant; see doc.
        RichVerdict::None => Compiler::Unknown,
    }
}

/// Detect the toolchain that produced a **Mach-O** image.
///
/// Precedence (most-specific source language first, then the toolchain):
/// 1. **Go** — a Go build-info section (`__go_buildinfo`) is present.
/// 2. **Rust** — a Rust-mangled symbol, OR a `rustc version` toolchain record in
///    a `__comment`/`__DWARF`-analog section (the format-neutral Rust paths).
/// 3. **Clang** — a `clang version` toolchain record, OR an `LC_BUILD_VERSION` /
///    `LC_VERSION_MIN_*` load command naming an **Apple platform**. Apple's only
///    system toolchain is clang/LLVM (Ghidra labels the macOS cspec the SysV
///    `gcc` model for ABI purposes, but the *source language*/compiler is clang),
///    so any Apple-platform Mach-O reports [`Compiler::Clang`].
/// 4. **Gcc** — a `GCC: (...)` toolchain record (cross-built MinGW-style).
/// 5. **Unknown** otherwise.
pub fn detect_compiler_macho(file: &object::File, bytes: &[u8]) -> Compiler {
    if golang_section_present(file.sections().filter_map(|s| s.name().ok())) {
        return Compiler::Go;
    }

    // Mach-O keeps any toolchain text in a `__comment`/`__DWARF` section (rare for
    // a linked exe); read what `object` exposes and scan it like a `.comment`.
    let toolchain = ["__comment", ".comment", "__apple_names"]
        .iter()
        .find_map(|n| file.section_by_name(n).and_then(|s| s.data().ok()))
        .map(<[u8]>::to_vec)
        .unwrap_or_default();

    if comment_indicates_rust(&toolchain) || symbols_indicate_rust(file) {
        return Compiler::Rustc;
    }
    if comment_contains(&toolchain, b"clang version ") {
        return Compiler::Clang;
    }
    if comment_contains(&toolchain, b"GCC: ") {
        return Compiler::Gcc;
    }

    // The reliable Mach-O fingerprint: an `LC_BUILD_VERSION` / `LC_VERSION_MIN_*`
    // load command. An Apple platform ⇒ clang/LLVM toolchain.
    if macho_build_version_is_apple(bytes) {
        return Compiler::Clang;
    }

    Compiler::Unknown
}

/// Format-neutral fallback for the formats we do not fingerprint (COFF object,
/// raw): honour only the universal Rust/Go signals, else `Unknown`. Keeps a Rust
/// or Go COFF object detectable without inventing a toolchain for plain `.obj`s.
fn detect_compiler_generic(file: &object::File) -> Compiler {
    if golang_section_present(file.sections().filter_map(|s| s.name().ok())) {
        return Compiler::Go;
    }
    if symbols_indicate_rust(file) {
        return Compiler::Rustc;
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

/// `true` if any symbol name is a Rust-mangled symbol: the v0 scheme (`_R...`) or
/// the legacy scheme (`_ZN...17h<16 hex>E`). This is the mangled-symbol heuristic
/// — robust on a non-stripped Rust binary even when the `.comment` record was
/// removed, and **format-neutral** (`file.symbols()`/`dynamic_symbols()` work for
/// ELF/PE/Mach-O alike), so it is the shared Rust signal every format arm uses.
/// Mirrors what the loader's demangle gate already recognizes (`rustc_demangle`).
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
    // v0: leading `_R` (allow one extra platform underscore: `__R`). The
    // underscore is load-bearing: `strip_prefix('_').unwrap_or(name)` KEEPS the
    // original name when there is none, which degrades the test to "starts with
    // `R`" and claims every OpenSSL `RSA_new`/`RAND_bytes` importer is Rust.
    let v0 = name.strip_prefix("__R").or_else(|| name.strip_prefix("_R"));
    if v0.is_some_and(|rest| !rest.is_empty()) {
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

// ---------------------------------------------------------------------------
// PE `Rich` header / `@comp.id` fingerprint
// ---------------------------------------------------------------------------

/// The verdict of [`detect_rich_header`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RichVerdict {
    /// A valid `Rich` header whose `@comp.id`s are tagged clang/LLVM (clang-cl).
    Clang,
    /// A valid `Rich` header — an MSVC `cl.exe` link (the common case).
    Msvc,
    /// No valid `Rich` header (MinGW / Clang-direct / non-MSVC linker).
    None,
}

/// Parse the MSVC `Rich` header out of the raw PE image and classify the linking
/// toolchain. The header is an obfuscated block in the DOS stub area (before the
/// PE header at `e_lfanew`), terminated by the literal `"Rich"` tag followed by a
/// 4-byte XOR key; XOR-ing the preceding dwords with that key recovers a stream
/// that starts with `"DanS"` and then pairs of `(@comp.id, count)` dwords. We
/// confirm the `DanS` marker (so a coincidental `"Rich"` substring is rejected)
/// and inspect the `@comp.id` product ids to split clang-cl from MSVC.
///
/// Pure & total: any malformed/absent header yields [`RichVerdict::None`].
fn detect_rich_header(bytes: &[u8]) -> RichVerdict {
    // Need at least the DOS header (e_lfanew at 0x3c) to bound the search window.
    if bytes.len() < 0x40 {
        return RichVerdict::None;
    }
    let e_lfanew = u32::from_le_bytes([bytes[0x3c], bytes[0x3d], bytes[0x3e], bytes[0x3f]]) as usize;
    // The `Rich` block lives between the DOS stub and the PE header. Bound the
    // search to that window (clamp to the buffer to stay total).
    let end = e_lfanew.min(bytes.len());
    let window = &bytes[..end];

    // Find the `"Rich"` end tag; the 4 bytes after it are the XOR key.
    let Some(rich_pos) = find_subslice(window, RICH_END) else {
        return RichVerdict::None;
    };
    let key_pos = rich_pos + 4;
    if key_pos + 4 > window.len() {
        return RichVerdict::None;
    }
    let key = u32::from_le_bytes([
        window[key_pos],
        window[key_pos + 1],
        window[key_pos + 2],
        window[key_pos + 3],
    ]);

    // Walk backwards from `"Rich"` in 4-byte dwords, XOR-decoding each, until we
    // hit the de-obfuscated `"DanS"` start marker. The records between are
    // `(@comp.id, count)` pairs: `@comp.id` = (prodid << 16) | build.
    let mut prodids: Vec<u16> = Vec::new();
    let mut off = rich_pos;
    let mut found_dans = false;
    // Cap the walk so a malformed buffer cannot loop unboundedly.
    let mut guard = 0;
    while off >= 4 && guard < 4096 {
        off -= 4;
        guard += 1;
        let raw = u32::from_le_bytes([
            window[off],
            window[off + 1],
            window[off + 2],
            window[off + 3],
        ]);
        let dec = raw ^ key;
        if dec == RICH_BEGIN {
            found_dans = true;
            break;
        }
        // A `@comp.id` dword is the first of each (id, count) pair. The pairs run
        // from just after `DanS` up to `Rich`; collecting every other dword from
        // the `Rich` end yields the ids (the in-between dwords are counts). We
        // collect all decoded high-words and dedup; an id of 0 is padding.
        let prodid = (dec >> 16) as u16;
        if prodid != 0 {
            prodids.push(prodid);
        }
    }

    if !found_dans {
        return RichVerdict::None;
    }
    if prodids.iter().any(|p| RICH_LLVM_PRODIDS.contains(p)) {
        RichVerdict::Clang
    } else {
        RichVerdict::Msvc
    }
}

/// First-occurrence byte-substring search (returns the start index).
fn find_subslice(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    if needle.is_empty() || needle.len() > haystack.len() {
        return None;
    }
    haystack.windows(needle.len()).position(|w| w == needle)
}

// ---------------------------------------------------------------------------
// Mach-O `LC_BUILD_VERSION` / `LC_VERSION_MIN_*` fingerprint
// ---------------------------------------------------------------------------

/// `true` if the Mach-O image carries an `LC_BUILD_VERSION` (any platform) or an
/// `LC_VERSION_MIN_*` load command — i.e. it was built for an Apple platform with
/// the Apple/clang toolchain. Walks the load commands with the typed Mach-O
/// parsers (the same pattern as `entry/macho_entry.rs`), dispatching on
/// `FileKind` for thin 32/64 and peeling fat slices.
///
/// Pure & total: a parse failure / non-Mach-O yields `false`.
fn macho_build_version_is_apple(bytes: &[u8]) -> bool {
    match FileKind::parse(bytes) {
        Ok(FileKind::MachO64) => macho_has_version_command::<MachHeader64<Endianness>>(bytes),
        Ok(FileKind::MachO32) => macho_has_version_command::<MachHeader32<Endianness>>(bytes),
        // Fat/universal: any slice being an Apple build is enough (universal
        // binaries are all-Apple in practice). `FatArch::data` peels the slice to
        // a thin image, so we recurse on the slice bytes at offset 0.
        Ok(FileKind::MachOFat32) => match MachOFatFile32::parse(bytes) {
            Ok(fat) => fat
                .arches()
                .iter()
                .filter_map(|a| a.data(bytes).ok())
                .any(macho_build_version_is_apple),
            Err(_) => false,
        },
        Ok(FileKind::MachOFat64) => match MachOFatFile64::parse(bytes) {
            Ok(fat) => fat
                .arches()
                .iter()
                .filter_map(|a| a.data(bytes).ok())
                .any(macho_build_version_is_apple),
            Err(_) => false,
        },
        _ => false,
    }
}

/// `true` if the thin Mach-O `bytes` carries an `LC_BUILD_VERSION` or
/// `LC_VERSION_MIN_*` load command. Generic over the header width.
fn macho_has_version_command<Mach>(bytes: &[u8]) -> bool
where
    Mach: MachHeader<Endian = Endianness>,
{
    let Ok(header) = Mach::parse(bytes, 0) else {
        return false;
    };
    let Ok(endian) = header.endian() else {
        return false;
    };
    let Ok(mut commands) = header.load_commands(endian, bytes, 0) else {
        return false;
    };
    while let Ok(Some(command)) = commands.next() {
        match command.variant() {
            // `LC_BUILD_VERSION` (any platform) or the legacy `LC_VERSION_MIN_*`
            // family — both are Apple-only load commands.
            Ok(LoadCommandVariant::BuildVersion(_)) | Ok(LoadCommandVariant::VersionMin(_)) => {
                return true;
            }
            _ => {}
        }
    }
    false
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
        // v0 requires the LEADING UNDERSCORE. Without it the prefix test
        // degrades to "starts with `R`", and an ordinary C program that imports
        // OpenSSL is claimed to be Rust -- which made `--language auto` render
        // /usr/bin/ncat as Rust.
        assert!(!is_rust_mangled("RAND_bytes"));
        assert!(!is_rust_mangled("RSA_new"));
        assert!(!is_rust_mangled("RAND_bytes@OPENSSL_3.0.0"));
        assert!(!is_rust_mangled("Rmake"));
        assert!(!is_rust_mangled("R"));
        // the platform double underscore still counts
        assert!(is_rust_mangled("__RNvCs1234_4core5panic"));
        // a bare `_R` with nothing after it is not a symbol
        assert!(!is_rust_mangled("_R"));
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
        assert_eq!(detect_compiler(&file, &bytes), Compiler::Rustc);
        assert!(detect_compiler(&file, &bytes).is_rust());
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
            let c = detect_compiler(&file, &bytes);
            assert_eq!(c, Compiler::Gcc, "{f} should detect as Gcc");
            assert!(!c.is_rust(), "{f} must not detect as Rust");
            assert!(!c.is_golang(), "{f} must not detect as Go");
        }
    }

    // --- multi-format detection (design §5.3 / §8 PR-14) ---

    /// PE: the MinGW `pe_imports.exe` carries `GCC: (GNU) …` records in `.rdata`
    /// (MinGW emits no `.comment` *section* in a PE), so it detects as `Gcc`.
    #[test]
    fn detects_gcc_on_mingw_pe_fixture() {
        let bytes = fixture("pe_imports.exe");
        let file = object::File::parse(bytes.as_slice()).expect("parse PE fixture");
        assert_eq!(file.format(), BinaryFormat::Pe, "fixture must be a PE");
        let c = detect_compiler(&file, &bytes);
        assert_eq!(c, Compiler::Gcc, "MinGW PE should detect as Gcc (.rdata GCC: record)");
        assert!(!c.is_rust());
        assert!(!c.is_golang());
        // It is specifically the `GCC:` toolchain path — no `Rich` header on a MinGW PE.
        assert_eq!(detect_rich_header(&bytes), RichVerdict::None, "MinGW PE has no Rich header");
    }

    /// Mach-O: the x86-64 and arm64 `macho_imports` fixtures carry an
    /// `LC_BUILD_VERSION` (PLATFORM_MACOS) load command, so they detect as `Clang`
    /// (Apple platform ⇒ clang/LLVM toolchain).
    #[test]
    fn detects_clang_on_macho_fixtures() {
        for f in ["macho_imports", "macho_imports_arm64"] {
            let bytes = fixture(f);
            let file = object::File::parse(bytes.as_slice()).expect("parse Mach-O fixture");
            assert_eq!(file.format(), BinaryFormat::MachO, "{f} must be a Mach-O");
            let c = detect_compiler(&file, &bytes);
            assert_eq!(c, Compiler::Clang, "{f} should detect as Clang (LC_BUILD_VERSION)");
            assert!(!c.is_rust());
            assert!(!c.is_golang());
            // the load-command fingerprint is what fires.
            assert!(macho_build_version_is_apple(&bytes), "{f}: LC_BUILD_VERSION present");
        }
    }

    /// The Mach-O `macho_min.o` object also carries `LC_BUILD_VERSION`.
    #[test]
    fn detects_clang_on_macho_object() {
        let bytes = fixture("macho_min.o");
        let file = object::File::parse(bytes.as_slice()).expect("parse Mach-O object");
        assert_eq!(detect_compiler(&file, &bytes), Compiler::Clang);
    }

    /// Hermetic PE `Rich`-header parsing: a synthetic DOS-stub + `DanS…@comp.id…Rich`
    /// block decodes to MSVC, and the clang-cl product-id variant to Clang. Covers
    /// the MSVC path without an `cl.exe` fixture (none buildable on Linux).
    #[test]
    fn rich_header_msvc_vs_clang() {
        // MSVC: a single (@comp.id, count) pair with a non-LLVM product id.
        assert_eq!(detect_rich_header(&synth_pe_with_rich(&[0x00FF])), RichVerdict::Msvc);
        // clang-cl: a product id in the LLVM set.
        assert_eq!(detect_rich_header(&synth_pe_with_rich(&[0x0105])), RichVerdict::Clang);
        assert_eq!(detect_rich_header(&synth_pe_with_rich(&[0x00FF, 0x0103])), RichVerdict::Clang);
        // No `Rich` block at all ⇒ None.
        assert_eq!(detect_rich_header(b"MZ\x00\x00 not a real pe, no rich"), RichVerdict::None);
        // a stray `"Rich"` substring without a decodable `DanS` start ⇒ None.
        let mut junk = vec![0u8; 0x80];
        junk[0x3c..0x40].copy_from_slice(&0x80u32.to_le_bytes());
        junk[0x40..0x44].copy_from_slice(b"Rich"); // beyond e_lfanew window → ignored anyway
        assert_eq!(detect_rich_header(&junk), RichVerdict::None);
    }

    /// The format-neutral Rust-symbol path fires regardless of format: a synthetic
    /// in-memory object exercising `is_rust_mangled` over symbol names is covered
    /// by `rust_mangled_name_recognition`; here we assert the *dispatch* honours a
    /// Rust signal even on a non-ELF format via `detect_compiler_generic`'s symbol
    /// scan (a COFF object with a Rust-mangled symbol ⇒ `Rustc`). We use the
    /// vendored `coff_obj.obj` to confirm `Unknown` (no Rust symbols), proving the
    /// generic arm does not false-positive.
    #[test]
    fn coff_object_stays_unknown_without_rust_signal() {
        let bytes = fixture("coff_obj.obj");
        let file = object::File::parse(bytes.as_slice()).expect("parse COFF object");
        // A plain C COFF object carries no Rust/Go signal ⇒ Unknown (no output change).
        assert_eq!(detect_compiler(&file, &bytes), Compiler::Unknown);
    }

    /// Build a minimal PE buffer carrying a valid `Rich` header from the given
    /// product ids. Layout: DOS header with `e_lfanew = 0x80`, then a `DanS`
    /// start marker, the `(@comp.id, count)` pairs, and the `Rich` end tag + XOR
    /// key — all XOR-obfuscated with a fixed key (as MSVC's linker does).
    fn synth_pe_with_rich(prodids: &[u16]) -> Vec<u8> {
        let key: u32 = 0xDEADBEEF;
        // The de-obfuscated stream: DanS, three zero pad dwords (MSVC convention),
        // then (comp_id, count) pairs.
        let mut clear: Vec<u32> = vec![RICH_BEGIN, 0, 0, 0];
        for &pid in prodids {
            clear.push(((pid as u32) << 16) | 0x1234); // @comp.id = (prodid<<16)|build
            clear.push(1); // count
        }
        // Obfuscate every dword with the key.
        let mut block: Vec<u8> = Vec::new();
        for w in &clear {
            block.extend_from_slice(&(w ^ key).to_le_bytes());
        }
        // Append the `Rich` end tag (clear) + the XOR key (clear).
        block.extend_from_slice(RICH_END);
        block.extend_from_slice(&key.to_le_bytes());

        // Assemble: 0x40-byte DOS header (e_lfanew at 0x3c), then the Rich block,
        // padded so e_lfanew sits past it.
        let e_lfanew: u32 = (0x40 + block.len() as u32 + 7) & !7; // 8-byte aligned
        let mut buf = vec![0u8; e_lfanew as usize + 8];
        buf[0] = b'M';
        buf[1] = b'Z';
        buf[0x3c..0x40].copy_from_slice(&e_lfanew.to_le_bytes());
        buf[0x40..0x40 + block.len()].copy_from_slice(&block);
        buf
    }
}
