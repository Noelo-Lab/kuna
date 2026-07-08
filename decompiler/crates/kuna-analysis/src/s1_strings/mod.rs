//! String-literal detection — the kuna analog of Ghidra's `StringsAnalyzer`
//! ("ASCII Strings"), `Ghidra/Features/Base/.../plugin/core/string/StringsAnalyzer.java`.
//!
//! ## Why this lives in kuna
//!
//! kuna ports Ghidra's C++ *decompiler*; the detection of a NUL-terminated string
//! at a data address (and the `char[N]` type it gets) is done by Ghidra's Java
//! *analyzer* tier, which the port never carried over. Without it a constant
//! pointer into `.rodata` (e.g. `0x400915`) prints as a bare integer instead of
//! the literal `"Username: "`. This pass detects the strings and the commit seam
//! (`kuna-console`'s `commit_analysis_output`) places a typelocked `char[N]` data
//! symbol at each, so the engine+printer (already ported) can render the literal.
//!
//! ## Algorithm (faithful to `StringsAnalyzer` defaults)
//!
//! - **Scan set**: `getLoadedAndInitializedAddressSet` ∩ accessible blocks — i.e.
//!   allocated + initialized sections (`SHF_ALLOC`, not `.bss`). This is `.rodata`,
//!   `.data`, `.text`, … — NOT only read-only. See [`scan_strings`].
//! - **`minStringLength`** = **5** (`MinStringLen.LEN_5`); **`requireNullEnd`** =
//!   **true**; **`startAlignment`** = **1**; **`allCharWidths`** = false (ASCII /
//!   1-byte only — UTF-16/32 is a documented seam, skipped).
//! - **Char set** (`AsciiCharSetRecognizer.contains`): `0x20..=0x7e || \r || \n ||
//!   \t`. See [`is_string_char`].
//! - **Matcher** (`MinLengthCharSequenceMatcher`): a run of in-charset bytes ended
//!   by a NUL is null-terminated; ended by any other out-of-charset byte is
//!   rejected (since `requireNullEnd`). The NUL is NOT counted toward the minimum
//!   length. Emit if `visible_len >= min_len`. See [`scan_run`].
//!
//! ## LOSS vs Ghidra (documented divergence)
//!
//! Ghidra additionally scores each candidate with an n-gram trigram model
//! (`StringModel.sng` / `NGramUtils`) to reject false positives (random
//! printable runs that are not real text). That model file is **not in the tree**
//! and is **not portable**, so this pass OMITS it and substitutes the weaker
//! "printable + NUL-terminated + min-len-5" test. The effect is *over*-acceptance
//! relative to Ghidra (a random printable run that happens to be NUL-terminated
//! and ≥5 chars is accepted) — harmless for real string literals, which all pass.
//! Recorded in `docs/missing-analyses.md` / `docs/analysis-port-log.md`.

use object::read::{Object, ObjectSection};
use object::SectionKind;

use crate::pass::{AnalysisCtx, AnalysisOutput, AnalysisPass, Phase, StringFact};

/// The default minimum visible string length (`MinStringLen.LEN_5`).
const DEFAULT_MIN_LEN: usize = 5;

/// `AsciiCharSetRecognizer.contains`: a byte is "string content" iff it is a
/// printable ASCII char (`0x20..=0x7e`) or one of CR/LF/TAB. Faithful to the
/// 1-byte (ASCII) recognizer — UTF-16/32 widths are not handled here.
fn is_string_char(b: u8) -> bool {
    (0x20..=0x7e).contains(&b) || b == 0x0d || b == 0x0a || b == 0x09
}

/// Run the `MinLengthCharSequenceMatcher` over one contiguous byte slice mapped at
/// virtual address `vma`, returning a [`StringFact`] per NUL-terminated run of
/// in-charset bytes whose visible length is `>= min_len`.
///
/// The matcher accumulates a run of [`is_string_char`] bytes; when it hits a NUL
/// it emits the run if long enough (the run is "null-terminated"); when it hits
/// any other out-of-charset byte the run is rejected (because `requireNullEnd` is
/// the default). The NUL is not counted toward `min_len`, but it *is* counted in
/// the emitted `len` (the `char[N]` array length = visible_len + 1).
fn scan_run(data: &[u8], vma: u64, min_len: usize) -> Vec<StringFact> {
    let mut out = Vec::new();
    let mut run_start: Option<usize> = None;
    for (i, &b) in data.iter().enumerate() {
        if is_string_char(b) {
            if run_start.is_none() {
                run_start = Some(i);
            }
            continue;
        }
        // Out-of-charset byte: closes the current run (if any).
        if let Some(start) = run_start.take() {
            let visible_len = i - start;
            // requireNullEnd: only a NUL terminator makes the run a string.
            if b == 0 && visible_len >= min_len {
                out.push(StringFact {
                    addr: vma + start as u64,
                    len: (visible_len + 1) as u32, // + the trailing NUL
                });
            }
        }
    }
    // A run that reaches the end of the slice without a NUL is rejected
    // (requireNullEnd): no emission for a trailing `run_start`.
    out
}

/// Is `sec` part of the loaded+initialized image — i.e. worth scanning for a
/// string literal (`getLoadedAndInitializedAddressSet`)?
///
/// This is the per-format arm of the "allocated + initialized" test. The
/// **ELF arm is byte-identical to before** (`SHF_ALLOC`), so ELF output is
/// unchanged. The PE/COFF and Mach-O arms generalize the same notion through the
/// neutral [`SectionKind`] + each format's flag bits, so a PE `.rdata` / Mach-O
/// `__cstring` is scanned and a PE `puts("hello")` recovers its literal.
fn is_loaded_initialized<'a>(sec: &impl ObjectSection<'a>) -> bool {
    // `.bss`-style uninitialized memory has no file content to scan, on every
    // format (the neutral signal `object` derives from the section type/flags).
    if matches!(sec.kind(), SectionKind::UninitializedData) {
        return false;
    }
    match sec.flags() {
        // ELF: SHF_ALLOC (the section occupies memory at runtime) — unchanged, so
        // the default ELF path is byte-identical. A non-allocated section
        // (`.comment`, a non-ALLOC `.symtab`) is not scanned.
        object::SectionFlags::Elf { sh_flags } => {
            const SHF_ALLOC: u64 = 0x2;
            sh_flags & SHF_ALLOC != 0
        }
        // PE / COFF: every section in a PE/COFF image is mapped (there is no
        // SHF_ALLOC analog), so an initialized, readable section is in the loaded
        // image. Skip uninitialized-data sections via the COFF flag too (the
        // `SectionKind` guard above already caught `.bss`).
        object::SectionFlags::Coff { characteristics } => {
            use object::pe::{IMAGE_SCN_CNT_UNINITIALIZED_DATA, IMAGE_SCN_MEM_READ};
            characteristics & IMAGE_SCN_CNT_UNINITIALIZED_DATA == 0
                && characteristics & IMAGE_SCN_MEM_READ != 0
        }
        // Mach-O: section R/W/X permissions live in the enclosing segment, not the
        // section flags — but the section *type* marks the zero-fill (`.bss`)
        // sections, already excluded by the `SectionKind` guard above. So any
        // non-zero-fill Mach-O section with file content is in the loaded image
        // (`__cstring`, `__text`, `__const`, `__data`, …).
        object::SectionFlags::MachO { flags } => {
            use object::macho::{S_GB_ZEROFILL, S_ZEROFILL, SECTION_TYPE};
            let sec_type = flags & SECTION_TYPE;
            sec_type != S_ZEROFILL && sec_type != S_GB_ZEROFILL
        }
        // Any other format flag set: be conservative and skip (no string scan).
        _ => false,
    }
}

/// Scan every allocated + initialized section of `file` for NUL-terminated ASCII
/// strings ≥ `min_len` visible chars (`getLoadedAndInitializedAddressSet`).
///
/// Included iff the section is in the loaded+initialized image
/// ([`is_loaded_initialized`]) and its `data()` is non-empty: `.bss`
/// ([`SectionKind::UninitializedData`]) and any non-loaded metadata section is
/// skipped. The string addresses are `section_vma + run_start`. Format-agnostic
/// (ELF/PE/COFF/Mach-O), with the ELF path byte-identical to before.
fn scan_strings(file: &object::File, min_len: usize) -> Vec<StringFact> {
    let mut out = Vec::new();
    for sec in file.sections() {
        if !is_loaded_initialized(&sec) {
            continue;
        }
        let data = match sec.data() {
            Ok(d) if !d.is_empty() => d,
            _ => continue,
        };
        out.extend(scan_run(data, sec.address(), min_len));
    }
    out
}

/// Port of `StringsAnalyzer` ("ASCII Strings"): detect NUL-terminated ASCII
/// string literals in the loaded+initialized image and emit a [`StringFact`]
/// (a `char[N]` data object) per find.
pub struct StringLiteralPass {
    /// Minimum visible string length (`StringsAnalyzer.minStringLength`,
    /// default [`DEFAULT_MIN_LEN`]).
    pub min_len: usize,
}

impl Default for StringLiteralPass {
    fn default() -> Self {
        StringLiteralPass { min_len: DEFAULT_MIN_LEN }
    }
}

impl AnalysisPass for StringLiteralPass {
    fn phase(&self) -> Phase {
        Phase::P1
    }

    fn id(&self) -> &'static str {
        "strings"
    }

    fn run(&self, ctx: &AnalysisCtx) -> AnalysisOutput {
        AnalysisOutput { strings: scan_strings(ctx.file, self.min_len), ..Default::default() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn charset_recognizer() {
        // printable ASCII + the three whitespace escapes are content.
        assert!(is_string_char(b'A'));
        assert!(is_string_char(b' '));
        assert!(is_string_char(0x7e)); // '~', top of printable range
        assert!(is_string_char(b'\r'));
        assert!(is_string_char(b'\n'));
        assert!(is_string_char(b'\t'));
        // NUL, other control bytes, and high bytes are not content.
        assert!(!is_string_char(0x00));
        assert!(!is_string_char(0x01));
        assert!(!is_string_char(0x1f));
        assert!(!is_string_char(0x7f));
        assert!(!is_string_char(0x80));
        assert!(!is_string_char(0xff));
    }

    #[test]
    fn matcher_basic() {
        // \x00 Hi \x00 Username:_ \x00 \x01\x02 abcdefgh \x00
        // - "Hi"        (len 2)  rejected (< min 5)
        // - "Username: " (len 10) accepted, NUL-terminated -> StringFact len 11
        // - "abcdefgh"  (len 8)  accepted, NUL-terminated -> StringFact len 9
        let data = b"\x00Hi\x00Username: \x00\x01\x02abcdefgh\x00";
        let vma = 0x1000u64;
        let out = scan_run(data, vma, 5);
        assert_eq!(out.len(), 2, "two strings should be detected");

        // "Username: " begins at index 4 (after \x00 Hi \x00).
        let username_off = 4u64;
        assert_eq!(out[0], StringFact { addr: vma + username_off, len: 11 });

        // "abcdefgh" begins after "...: \x00\x01\x02": index 4+11+2 = 17.
        let abc_off = (4 + 11 + 2) as u64;
        assert_eq!(out[1], StringFact { addr: vma + abc_off, len: 9 });
    }

    #[test]
    fn non_terminated_run_rejected() {
        // A long printable run with no NUL terminator (reaches end of slice) is
        // rejected by requireNullEnd.
        let out = scan_run(b"abcdefghij", 0, 5);
        assert!(out.is_empty(), "non-NUL-terminated run must be rejected");
        // Terminated by a non-NUL out-of-charset byte is also rejected.
        let out2 = scan_run(b"abcdefghij\x01", 0, 5);
        assert!(out2.is_empty(), "run ended by a non-NUL byte must be rejected");
    }

    #[test]
    fn min_length_boundary() {
        // Exactly 5 visible chars + NUL -> accepted (len 6).
        let out5 = scan_run(b"abcde\x00", 0x2000, 5);
        assert_eq!(out5, vec![StringFact { addr: 0x2000, len: 6 }]);
        // 4 visible chars + NUL -> rejected (< min 5).
        let out4 = scan_run(b"abcd\x00", 0x2000, 5);
        assert!(out4.is_empty(), "4-char string must be rejected at min 5");
    }

    #[test]
    fn scan_strings_over_fauxware() {
        // End-to-end fixture-level scan: the StringsAnalyzer port must surface the
        // three cited literals at their .rodata addresses.
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/fauxware");
        let bytes = std::fs::read(path).expect("read fauxware fixture");
        let file = object::File::parse(bytes.as_slice()).expect("parse fauxware");
        let out = scan_strings(&file, DEFAULT_MIN_LEN);

        let has = |addr: u64, len: u32| out.contains(&StringFact { addr, len });
        // "SOSNEAKY"   @ 0x4008d0 (8 visible + NUL = 9)
        assert!(has(0x4008d0, 9), "SOSNEAKY @ 0x4008d0 len 9 not detected: {out:?}");
        // "Username: " @ 0x400915 (10 visible + NUL = 11)
        assert!(has(0x400915, 11), "\"Username: \" @ 0x400915 len 11 not detected: {out:?}");
        // "Password: " @ 0x400920 (10 visible + NUL = 11)
        assert!(has(0x400920, 11), "\"Password: \" @ 0x400920 len 11 not detected: {out:?}");
    }

    #[test]
    fn scan_strings_over_pe_finds_hello() {
        // PR-10: the format-agnostic string scan must fire on a PE — the
        // `puts("hello")` literal lives in `.rdata` (`SectionKind::ReadOnlyData`,
        // `SectionFlags::Coff`), which the generalized `is_loaded_initialized`
        // now scans (the old `SHF_ALLOC`-only gate skipped every non-ELF section).
        // "hello" @ 0x140009000 (5 visible + NUL = 6).
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/pe_imports.exe");
        let bytes = std::fs::read(path).expect("read pe_imports.exe");
        let file = object::File::parse(bytes.as_slice()).expect("parse pe_imports.exe");
        assert_eq!(file.format(), object::BinaryFormat::Pe, "fixture is a PE");
        let out = scan_strings(&file, DEFAULT_MIN_LEN);
        assert!(
            out.contains(&StringFact { addr: 0x140009000, len: 6 }),
            "\"hello\" @ 0x140009000 len 6 not detected in PE .rdata: {out:?}"
        );
    }

    #[test]
    fn elf_section_selection_is_unchanged() {
        // Parity guard: the ELF arm of `is_loaded_initialized` must match the old
        // `SHF_ALLOC`-only test exactly (the default ELF path is byte-identical).
        // Every SHF_ALLOC fauxware section is selected; `.comment` (no ALLOC) and
        // `.bss` (UninitializedData) are not.
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/fauxware");
        let bytes = std::fs::read(path).expect("read fauxware fixture");
        let file = object::File::parse(bytes.as_slice()).expect("parse fauxware");
        const SHF_ALLOC: u64 = 0x2;
        for sec in file.sections() {
            let old = !matches!(sec.kind(), SectionKind::UninitializedData)
                && match sec.flags() {
                    object::SectionFlags::Elf { sh_flags } => sh_flags & SHF_ALLOC != 0,
                    _ => false,
                };
            assert_eq!(
                is_loaded_initialized(&sec),
                old,
                "ELF section selection changed for {:?}",
                sec.name()
            );
        }
    }
}
