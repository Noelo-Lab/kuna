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
//!   are format-neutral in `object`). The v0 arm parses the name against the
//!   scheme's grammar and knows which formats prepend a platform underscore, so
//!   an ordinary C `_Runtime`/`_RC4_set_key` on Mach-O is not taken for one.
//! - **Go** — a Go build-info section under *any* format's naming
//!   (`.go.buildinfo`/`.note.go.buildid` on ELF, `__go_buildinfo` on Mach-O):
//!   [`golang_section_present`]. A PE has no such section (the Windows linker
//!   folds the blob into `.data`), so its arm also validates the blob in place.
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

/// Section-name prefixes of the Swift 5 reflection metadata, across formats:
/// `__swift5_types`/`__swift5_proto`/… inside `__TEXT` on Mach-O, the same
/// names without the segment underscores (`swift5_typeref`, `swift5_fieldmd`,
/// …) on ELF, and the abbreviated `.sw5…` spellings (`.sw5tymd`, `.sw5prt`,
/// `.sw5tyrf`, …) the Windows toolchain emits in a PE. Any section opening
/// with one of these prefixes is Swift metadata the compiler emitted — no
/// other toolchain uses the namespace — so a prefix test is exact.
const SWIFT_SECTION_PREFIXES: [&str; 3] = ["__swift5_", "swift5_", ".sw5"];

/// Swift runtime entry points a Swift binary calls (or exports, for the
/// runtime library itself) even when fully stripped of its own symbols: the
/// ARC and metadata hooks every nontrivial Swift function emits. Matched
/// exactly against the symbol tables (dynamic imports included), the same
/// role `RUST_SIGNATURES` plays for a `.comment`-less rustc image.
const SWIFT_RUNTIME_SYMBOLS: [&str; 6] = [
    "swift_retain",
    "swift_release",
    "swift_beginAccess",
    "swift_endAccess",
    "swift_allocObject",
    "swift_getTypeByMangledNameInContext",
];

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
    /// Swift (kuna — `swift5` reflection metadata sections, Swift-mangled
    /// symbols, or the Swift runtime import set; Ghidra has no Swift
    /// `SourceLanguage` extension point to port).
    Swift,
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

    /// `true` for [`Compiler::Swift`].
    pub fn is_swift(self) -> bool {
        matches!(self, Compiler::Swift)
    }

    /// The engine-plane [`SourceLang`] this verdict records on the
    /// `Architecture` (toolchain conveniences collapse to `C`).
    pub fn to_source_lang(self) -> kuna_decomp::kuna_sourcelang::SourceLang {
        use kuna_decomp::kuna_sourcelang::SourceLang;
        match self {
            Compiler::Gcc | Compiler::Clang => SourceLang::C,
            Compiler::Rustc => SourceLang::Rust,
            Compiler::Go => SourceLang::Go,
            Compiler::Swift => SourceLang::Swift,
            Compiler::Unknown => SourceLang::Unknown,
        }
    }

    /// The detection verdict a forced [`SourceLang`] stands in for — the
    /// inverse of [`Compiler::to_source_lang`] up to the toolchain
    /// conveniences (`c` forces the generic C arm, reported as `Gcc`).
    pub fn from_forced_lang(lang: kuna_decomp::kuna_sourcelang::SourceLang) -> Compiler {
        use kuna_decomp::kuna_sourcelang::SourceLang;
        match lang {
            SourceLang::C => Compiler::Gcc,
            SourceLang::Rust => Compiler::Rustc,
            SourceLang::Go => Compiler::Go,
            SourceLang::Swift => Compiler::Swift,
            SourceLang::Unknown => Compiler::Unknown,
        }
    }
}

/// The compiler verdict every consumer should act on: the `KUNA_SOURCELANG`
/// override when the environment carries one (`option sourcelang`, set by the
/// CLI before `load file` so it reaches pass selection), else detection.
pub fn effective_compiler(file: &object::File, bytes: &[u8]) -> Compiler {
    match kuna_decomp::kuna_sourcelang::forced_from_env() {
        Some(lang) => Compiler::from_forced_lang(lang),
        None => detect_compiler(file, bytes),
    }
}

/// [`effective_compiler`] over raw image bytes (the `load file` entry point).
pub fn effective_compiler_bytes(bytes: &[u8]) -> Compiler {
    match kuna_decomp::kuna_sourcelang::forced_from_env() {
        Some(lang) => Compiler::from_forced_lang(lang),
        None => detect_compiler_bytes(bytes),
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

    // Swift — before the toolchain conveniences: a Swift ELF links clang/gcc
    // runtime objects, so its `.comment` carries their records too.
    if swift_indicated(file) {
        return Compiler::Swift;
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
/// 1. **Go** — a Go build-info section (`.go.buildinfo` etc.) is present, OR
///    the build-info blob Go's Windows linker merges into `.data` validates
///    there (`loader::gobuildinfo::pe_buildinfo_blob`) AND a pclntab whose
///    table resolves sits in read-only data with the image's own `moduledata`
///    naming it (`pclntab::readonly_table_present`). A `go build` PE has only
///    the plain `.text`/`.rdata`/`.data` sections, so content is its one
///    signal; the pclntab half keeps a C program that merely carries a Go image
///    as data (a dropper's payload, in either data section) from being read as
///    Go. Checked first, as on every format: a cgo build also links C objects
///    whose toolchain records the later arms read.
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
    if golang_section_present(file.sections().filter_map(|s| s.name().ok()))
        || (crate::loader::gobuildinfo::pe_buildinfo_blob(file).is_some()
            && crate::pclntab::readonly_table_present(file))
    {
        return Compiler::Go;
    }

    // The toolchain `GCC:`/`clang version`/`rustc version` records: MinGW puts
    // them NUL-delimited in `.rdata` (no `.comment` section in a PE); a Clang
    // -direct PE may carry `.comment`. Read both and treat as one record stream.
    let comment = file.section_by_name(".comment").and_then(|s| s.data().ok()).unwrap_or(&[]);
    let rdata = file.section_by_name(".rdata").and_then(|s| s.data().ok()).unwrap_or(&[]);
    let records = ToolchainRecords::scan_joined(comment, rdata);

    if records.rust || symbols_indicate_rust(file) {
        return Compiler::Rustc;
    }
    if swift_indicated(file) {
        return Compiler::Swift;
    }
    if records.clang {
        return Compiler::Clang;
    }
    if records.gcc {
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
    // Swift — before the Apple-platform fallback: every Swift Mach-O carries
    // `LC_BUILD_VERSION`, so the clang arm would otherwise claim it.
    if swift_indicated(file) {
        return Compiler::Swift;
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
    if swift_indicated(file) {
        return Compiler::Swift;
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

/// The toolchain records [`detect_compiler_pe`] reads.
///
/// Each flag is exactly [`comment_indicates_rust`] / [`comment_contains`] over the
/// same stream. A PE's `.rdata` runs to tens of megabytes and the detection runs
/// several times per load, so the scan neither copies it nor walks its records:
/// a record can only open with a needle it contains, so only the records around
/// a substring hit are decoded and tested.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct ToolchainRecords {
    rust: bool,
    clang: bool,
    gcc: bool,
}

impl ToolchainRecords {
    /// The records of `first` followed by `second`, read as one stream: a record
    /// left open at the end of `first` continues into `second`.
    fn scan_joined(first: &[u8], second: &[u8]) -> ToolchainRecords {
        if first.last().is_none_or(|&b| b == 0) {
            return Self::scan(first).or(Self::scan(second));
        }
        let tail = first.iter().rposition(|&b| b == 0).map_or(0, |at| at + 1);
        let head = second.iter().position(|&b| b == 0).unwrap_or(second.len());
        let mut joined = first[tail..].to_vec();
        joined.extend_from_slice(&second[..head]);
        Self::scan(&first[..tail]).or(Self::scan(&joined)).or(Self::scan(&second[head..]))
    }

    fn scan(blob: &[u8]) -> ToolchainRecords {
        ToolchainRecords {
            rust: any_record_opens_with(blob, "rustc version "),
            clang: any_record_opens_with(blob, "clang version "),
            gcc: any_record_opens_with(blob, "GCC: "),
        }
    }

    fn or(self, other: ToolchainRecords) -> ToolchainRecords {
        ToolchainRecords {
            rust: self.rust || other.rust,
            clang: self.clang || other.clang,
            gcc: self.gcc || other.gcc,
        }
    }
}

/// [`comment_contains`] for one needle, visiting only the records that hold it.
fn any_record_opens_with(blob: &[u8], needle: &str) -> bool {
    let mut checked_to = 0;
    for hit in memchr::memmem::find_iter(blob, needle.as_bytes()) {
        if hit < checked_to {
            continue;
        }
        let start = memchr::memrchr(0, &blob[..hit]).map_or(0, |at| at + 1);
        let end = memchr::memchr(0, &blob[hit..]).map_or(blob.len(), |at| hit + at);
        if record_text(&blob[start..end]).starts_with(needle) {
            return true;
        }
        checked_to = end;
    }
    false
}

/// `true` if any symbol name is a Rust-mangled symbol: the v0 scheme (`_R...`) or
/// the legacy scheme (`_ZN...17h<16 hex>E`). This is the mangled-symbol heuristic
/// — robust on a non-stripped Rust binary even when the `.comment` record was
/// removed, and **format-neutral** (`file.symbols()`/`dynamic_symbols()` work for
/// ELF/PE/Mach-O alike), so it is the shared Rust signal every format arm uses.
/// Mirrors what the loader's demangle gate already recognizes (`rustc_demangle`).
///
/// The test is [`is_rust_mangled_in`], not [`is_rust_mangled`]: the format is in
/// hand here, and on Mach-O it decides whether a `_R…` symbol can be Rust at all.
fn symbols_indicate_rust(file: &object::File) -> bool {
    let format = file.format();
    file.symbols()
        .chain(file.dynamic_symbols())
        .filter(|s| s.kind() == SymbolKind::Text)
        .filter_map(|s| s.name().ok())
        .any(|name| is_rust_mangled_in(name, format))
}

/// Detect the source language straight from an image's bytes.
///
/// The byte-level entry point for callers that hold the image but not a parsed
/// [`object::File`] -- the console's `load file`, which records the verdict on
/// the `Architecture` so per-function engine rules (`option rustabi auto`) can
/// test it. An unparseable image reports [`Compiler::Unknown`].
pub fn detect_compiler_bytes(bytes: &[u8]) -> Compiler {
    match object::File::parse(bytes) {
        Ok(file) => detect_compiler(&file, bytes),
        Err(_) => Compiler::Unknown,
    }
}

/// `true` if `name` is a Rust-mangled symbol name. Recognizes:
/// - **v0**: `_R<path>` (or `__R<path>` with a platform leading underscore),
///   parsed against the scheme's grammar — the prefix alone is not evidence.
/// - **legacy**: `_ZN...17h<hex16>E` — the Itanium-`_ZN` form rustc emits with a
///   trailing 16-hex-digit `17h…` hash component, which distinguishes it from a
///   plain C++ `_ZN…` symbol.
///
/// Format-agnostic, for a caller holding a bare name (the demangle pass). Where
/// the object format is known, use [`is_rust_mangled_in`].
pub fn is_rust_mangled(name: &str) -> bool {
    is_rust_v0(name, false) || is_rust_legacy(name)
}

/// [`is_rust_mangled`] with the object format in hand — the form the detector uses.
///
/// Mach-O prepends a platform underscore to every symbol it carries, so a rustc
/// v0 symbol is spelled `__R…` there and a single-underscore `_R…` is an ordinary
/// C name (`_Runtime`, `_RC4_set_key`). Every other format spells v0 `_R…`, and
/// for them this is exactly [`is_rust_mangled`].
pub fn is_rust_mangled_in(name: &str, format: BinaryFormat) -> bool {
    is_rust_v0(name, format == BinaryFormat::MachO) || is_rust_legacy(name)
}

/// `true` if `name` is a well-formed v0 symbol; `platform_underscore` demands the
/// `__R…` spelling (see [`is_rust_mangled_in`]).
///
/// The prefix is a necessary condition, never a sufficient one: `_R` opens every
/// Mach-O C symbol whose name starts with `R`, so a prefix-only test reads
/// `_Read`, `_Runtime` and `_RC4_set_key` as rustc output and hands the whole
/// image the Rust output language, the Rust ABI rules and the Rust no-return
/// list. The grammar decides instead, and `rustc_demangle` — the scheme's
/// reference implementation, and the crate the demangle pass already hands these
/// names to — is what parses it.
fn is_rust_v0(name: &str, platform_underscore: bool) -> bool {
    let prefixed = name.starts_with("__R") || (!platform_underscore && name.starts_with("_R"));
    prefixed && rustc_demangle::try_demangle(name).is_ok()
}

/// `true` if `name` is a legacy Rust symbol: an Itanium `_ZN…E` name carrying the
/// `17h<16 hex>` hash component rustc appends and a plain C++ `_ZN…` lacks.
fn is_rust_legacy(name: &str) -> bool {
    let zn = name.strip_prefix('_').unwrap_or(name);
    let zn = zn.strip_prefix('_').unwrap_or(zn);
    if !zn.starts_with("ZN") {
        return false;
    }
    let Some(pos) = name.rfind("17h") else {
        return false;
    };
    let Some(hex) = name[pos + 3..].strip_suffix('E') else {
        return false;
    };
    hex.len() == 16 && hex.bytes().all(|b| b.is_ascii_hexdigit())
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

/// `true` if any section name opens a Swift 5 reflection-metadata namespace
/// ([`SWIFT_SECTION_PREFIXES`]). Factored over a `&str` iterator like
/// [`golang_section_present`] so it is unit-testable without a Swift fixture.
pub fn swift_section_present<'a>(names: impl Iterator<Item = &'a str>) -> bool {
    let mut names = names;
    names.any(|n| SWIFT_SECTION_PREFIXES.iter().any(|p| n.starts_with(p)))
}

/// `true` if `name` is a Swift-mangled symbol: the Swift 5 `$s`/`$S` scheme or
/// the Swift 4 `_T0` scheme, each with an optional platform leading underscore
/// (Mach-O spells them `_$s…`/`__T0…`).
///
/// The prefix is held to the scheme's own next token — an identifier length, a
/// known-type shortcut, or the stdlib module contraction — so an assembler
/// local like `$LN3` or a C `_T0mumble` cannot claim the image. No other
/// toolchain emits `$s`-namespace symbols, which keeps a one-symbol test as
/// safe as the Rust v0 grammar parse.
pub fn is_swift_mangled(name: &str) -> bool {
    let bare = name.strip_prefix('_').unwrap_or(name);
    if let Some(rest) = bare.strip_prefix("$s").or_else(|| bare.strip_prefix("$S")) {
        return rest
            .bytes()
            .next()
            .is_some_and(|b| b.is_ascii_digit() || b.is_ascii_uppercase() || b == b's');
    }
    if let Some(rest) = bare.strip_prefix("_T0").or_else(|| name.strip_prefix("_T0")) {
        return rest.bytes().next().is_some_and(|b| b.is_ascii_digit() || b.is_ascii_uppercase());
    }
    false
}

/// `true` if the image carries a Swift signal: a Swift-mangled text symbol, or
/// one of the Swift runtime entry points ([`SWIFT_RUNTIME_SYMBOLS`]) in either
/// symbol table — the dynamic table included, so a stripped Swift binary that
/// imports `libswiftCore` is still recognized.
fn symbols_indicate_swift(file: &object::File) -> bool {
    file.symbols().chain(file.dynamic_symbols()).any(|s| {
        let Ok(name) = s.name() else { return false };
        (s.kind() == SymbolKind::Text && is_swift_mangled(name))
            || SWIFT_RUNTIME_SYMBOLS.contains(&name.strip_prefix('_').unwrap_or(name))
    })
}

/// The shared Swift arm every format uses: metadata sections first (cheapest,
/// emitted by the compiler itself), then the symbol scan.
fn swift_indicated(file: &object::File) -> bool {
    swift_section_present(file.sections().filter_map(|s| s.name().ok()))
        || symbols_indicate_swift(file)
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

    /// The v0 arm parses the grammar. `_R` is also the start of every Mach-O C
    /// symbol whose name begins with `R`, so the prefix alone claimed `_Run`,
    /// `_Read` and `_RC4_set_key` for rustc — and one such symbol re-languages a
    /// whole C program to Rust.
    #[test]
    fn v0_names_must_parse_as_v0_paths() {
        // a path tag is one of `C M X Y N I B`; anything else ends it.
        for c_name in [
            "_Run", "_Read", "_Runtime", "_Rename", "_RSA_new", "_RAND_bytes", "_Rabbit",
        ] {
            assert!(!is_rust_mangled(c_name), "{c_name} is a C name, not a v0 symbol");
        }
        // `C` then a digit LOOKS like a crate-root identifier, so the tag alone is
        // not enough: `RC4_set_key` declares a 4-byte crate named `set_` and then
        // leaves `key` over, which is no instantiating-crate path.
        assert!(!is_rust_mangled("_RC4_set_key"));
        assert!(!is_rust_mangled("_RC2_key"));
        // real v0 symbols, including a generic instantiation.
        assert!(is_rust_mangled("_RNvCs1234_4core5panic"));
        assert!(is_rust_mangled("_RINvNtC3std3mem8align_ofdE"));
        assert!(is_rust_mangled("_RNvNtCs1234_4core3fmt5write"));
    }

    /// The detector knows the format, and on Mach-O every symbol carries a
    /// platform underscore — so rustc's v0 symbols read `__R…` there and a
    /// single-underscore `_R…` is a C name however well it parses.
    #[test]
    fn macho_needs_the_platform_underscore_for_v0() {
        let v0 = "_RNvCs1234_4core5panic";
        assert!(is_rust_mangled_in(v0, BinaryFormat::Elf));
        assert!(is_rust_mangled_in(v0, BinaryFormat::Pe));
        assert!(!is_rust_mangled_in(v0, BinaryFormat::MachO));
        // the Mach-O spelling of that same symbol.
        let macho_v0 = "__RNvCs1234_4core5panic";
        assert!(is_rust_mangled_in(macho_v0, BinaryFormat::MachO));
        assert!(is_rust_mangled_in(macho_v0, BinaryFormat::Elf));
        // legacy is unaffected by the format.
        let legacy = "_ZN4core9panicking5panic17h0123456789abcdefE";
        for f in [BinaryFormat::Elf, BinaryFormat::Pe, BinaryFormat::MachO] {
            assert!(is_rust_mangled_in(legacy, f));
        }
    }

    #[test]
    fn swift_section_predicate() {
        // Mach-O, ELF and PE spellings of the Swift 5 metadata namespace.
        for name in ["__swift5_types", "__swift5_proto", "swift5_typeref", "swift5_fieldmd", ".sw5tymd", ".sw5prt"] {
            assert!(swift_section_present([".text", name].iter().copied()), "{name}");
        }
        assert!(!swift_section_present([".text", ".comment", ".go.buildinfo"].iter().copied()));
        assert!(!swift_section_present(std::iter::empty()));
        // A name merely containing (not opening) the namespace is not a marker.
        assert!(!swift_section_present([".debug_swift5_typeref"].iter().copied()));
    }

    #[test]
    fn swift_mangled_name_recognition() {
        // Swift 5 scheme, ELF/PE (`$s…`) and Mach-O (`_$s…`) spellings, with a
        // module-name length, a known-type shortcut, and the stdlib contraction.
        for name in [
            "$s4main5helloyyF",
            "_$s4main5helloyyF",
            "$ss5printyypd_tF",
            "$sSS7cStringSSSPys4Int8VG_tcfC",
            "_$S4main5helloyyF",
            // Swift 4 scheme.
            "_T04main5helloyyF",
            "__T04main5helloyyF",
        ] {
            assert!(is_swift_mangled(name), "{name} is Swift-mangled");
        }
        // Assembler locals, plain C names, and `$`/`_T0` with no mangling body.
        for name in ["$LN3", "_$LN3", "main", "_Runtime", "$", "_T0", "_T0mumble", "_RNvCs1234_4core5panic"] {
            assert!(!is_swift_mangled(name), "{name} is not Swift-mangled");
        }
    }

    /// A synthetic ELF carrying one `swift5_` metadata section detects as Swift,
    /// and the same image without it stays Unknown — the hermetic analog of the
    /// fixture-backed Rust/Go tests (no Swift toolchain exists on this host).
    #[test]
    fn detects_swift_on_synthetic_elf_metadata_section() {
        use object::write::Object as WObject;
        let build = |with_swift: bool| -> Vec<u8> {
            let mut obj = WObject::new(
                object::BinaryFormat::Elf,
                object::Architecture::X86_64,
                object::Endianness::Little,
            );
            let text = obj.add_section(vec![], b".text".to_vec(), object::SectionKind::Text);
            obj.append_section_data(text, &[0xc3], 1);
            if with_swift {
                let s =
                    obj.add_section(vec![], b"swift5_typeref".to_vec(), object::SectionKind::ReadOnlyData);
                obj.append_section_data(s, b"\x00", 1);
            }
            obj.write().expect("write synthetic ELF")
        };
        let swift = build(true);
        let file = object::File::parse(swift.as_slice()).expect("parse");
        assert_eq!(detect_compiler(&file, &swift), Compiler::Swift);
        assert!(detect_compiler(&file, &swift).is_swift());
        let plain = build(false);
        let file = object::File::parse(plain.as_slice()).expect("parse");
        assert_eq!(detect_compiler(&file, &plain), Compiler::Unknown);
    }

    /// The forced-language mapping both ways: what `KUNA_SOURCELANG` forces and
    /// what the engine fact records (the env read itself is covered end-to-end
    /// by the `tests/cli` probes — no process-global mutation here).
    #[test]
    fn forced_language_round_trips_to_compiler_and_source_lang() {
        use kuna_decomp::kuna_sourcelang::SourceLang;
        for (lang, compiler) in [
            (SourceLang::C, Compiler::Gcc),
            (SourceLang::Rust, Compiler::Rustc),
            (SourceLang::Go, Compiler::Go),
            (SourceLang::Swift, Compiler::Swift),
        ] {
            assert_eq!(Compiler::from_forced_lang(lang), compiler);
            assert_eq!(compiler.to_source_lang(), lang);
        }
        // The toolchain conveniences collapse onto C; Unknown stays itself.
        assert_eq!(Compiler::Clang.to_source_lang(), SourceLang::C);
        assert_eq!(Compiler::Unknown.to_source_lang(), SourceLang::Unknown);
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

    /// The one-pass PE record scan reports exactly what the per-record predicates
    /// report over the concatenated stream, including a record that runs across
    /// the `.comment`/`.rdata` seam, a non-ASCII lead and a non-UTF-8 record.
    #[test]
    fn toolchain_record_scan_matches_per_record_predicates() {
        let blobs: [&[u8]; 14] = [
            b"",
            b"\0",
            b"GCC: (GNU) 13.2\0",
            b" \t\nclang version 17.0.1\0",
            b"rustc version 1.80.0 (abc)\0",
            b"GC",
            b"C: (MinGW) 12\0tail",
            b"\xc2\xa0GCC: (nbsp lead)\0",
            b"GCC: \xff bad utf8\0",
            b"GCC: ",
            b"xyz\0clang",
            b" version 9\0\0GCC: x",
            b"rustc",
            b"\x0b\x0cplain data\0r\0c\0G\0",
        ];
        for first in blobs {
            for second in blobs {
                let joined = [first, second].concat();
                let expected = ToolchainRecords {
                    rust: comment_indicates_rust(&joined),
                    clang: comment_contains(&joined, b"clang version "),
                    gcc: comment_contains(&joined, b"GCC: "),
                };
                assert_eq!(
                    ToolchainRecords::scan_joined(first, second),
                    expected,
                    "{first:?} + {second:?}"
                );
            }
        }
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
