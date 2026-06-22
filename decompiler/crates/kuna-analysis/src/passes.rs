//! The default program-prep pass list + the driver entry point the bootstrap
//! seam calls.
//!
//! [`default_passes`] is the registry of enabled analyses, in stage order. Each
//! pass's [`AnalysisPass::id`](crate::pass::AnalysisPass::id) is its gate name;
//! the passes here are **bound to the real-ELF bootstrap path only** (the XML
//! `<binaryimage>` datatest path never calls [`run_default_analyses`]), so the
//! 675/675 datatest parity oracle is structurally untouched regardless of what
//! is enabled — see the downstream-compatibility note in
//! `docs/missing-analyses.md`.

use kuna_decomp::architecture::Architecture;

use crate::loadimage_object::ObjectLoadImage;
use crate::pass::{run_analyses, AnalysisCtx, AnalysisOutput, AnalysisPass};

/// The default program-prep passes, in stage order.
///
/// These run at load time over the parsed ELF and produce an additive
/// [`AnalysisOutput`] the console commits into the engine. Adding a new analysis
/// is: implement [`AnalysisPass`] in an `s1_*` module, then add it here.
pub fn default_passes() -> Vec<Box<dyn AnalysisPass>> {
    vec![
        // S1 loader: known no-return functions (exit/abort/…). Mirrors Ghidra's
        // default-on `NoReturnFunctionAnalyzer`.
        Box::new(crate::s1_loader::noreturn::NoReturnKnownPass),
        // S1 strings: NUL-terminated ASCII string-literal detection. Mirrors
        // Ghidra's `StringsAnalyzer` (min length 5, require-NUL-end).
        // S1 library prototypes: seed common libc signatures (puts(char*), …) so
        // call arguments get typed. Mirrors Ghidra's `ApplyDataArchiveAnalyzer`.
        // THIS is what renders string literals in kuna: typing a call argument
        // `char *` lets the printer's pointer-char-constant path read the readonly
        // bytes via the StringManager and emit `puts("Username: ")`.
        Box::new(crate::s1_protos::LibProtoPass),
        // S1 entry discovery: find function entry points for stripped targets —
        // ELF e_entry, DT_INIT/DT_FINI + INIT_ARRAY/FINI_ARRAY pointer tables,
        // `.eh_frame` FDE pcBegin starts, the x86-64 `_start`→`main` libc-start
        // idiom, and conservative prologue byte patterns. Fuses Ghidra's
        // EntryPointAnalyzer/ExternalEntryFunctionAnalyzer/FunctionStartAnalyzer
        // + the GccExceptionAnalyzer `.eh_frame` FDE oracle into one additive pass
        // (the commit seam's `out.entries` arm names + adds each discovered VMA,
        // idempotent against the funcsym stream). After LibProtoPass so prototypes
        // are seeded first. Always-on, like noreturn/libproto.
        Box::new(crate::s1_entry::EntryDiscoveryPass),
        // S1 DWARF: recover function/global names and TYPED function signatures
        // from `.debug_*` sections (the kuna analog of Ghidra's `DWARFAnalyzer`).
        // Registered AFTER LibProtoPass so for any name both emit, the DWARF
        // (real source) prototype wins (last-write in set_function_prototype_pieces).
        // Skips cleanly on a non-DWARF binary. Subtask-3 (DW_OP_fbreg stack-local
        // ScopeLocal map) is a deferred engine change — see s1_dwarf docs.
        Box::new(crate::s1_dwarf::DwarfPass),
        // S1 strings (StringLiteralPass) is implemented + tested but **disabled by
        // default**: kuna's printer renders a constant that maps to a *named* global
        // symbol as that symbol's NAME (`s_400915`), which SHADOWS the string-literal
        // path. So planting a `char[N]` data symbol (Ghidra's StringsAnalyzer
        // mechanism) actually *blocks* `puts("Username: ")` in kuna, where the literal
        // instead comes from type-driven rendering (libproto/usage `char *` typing +
        // readonly + StringManager). Re-enabling it requires teaching the printer to
        // render a pointer to a readonly char-array symbol as the literal (the Ghidra
        // behavior) — a deferred printer change. See docs/analysis-port-log.md.
        // Box::new(crate::s1_strings::StringLiteralPass { min_len: 5 }),

        // S1 address tables (AddrTablePass) is implemented + tested but **disabled
        // by default**, matching Ghidra's `AddressTableAnalyzer.setDefaultEnablement(false)`.
        // It scans .rodata/.data for a run of consecutive pointer-width values that
        // all land inside an executable section (an absolute address/jump table) and
        // emits Data symbols + a readonly range. It is OFF because (a) Ghidra ships
        // it off (parity); (b) a pointer-run scanner over-accepts — any coincidental
        // run of in-range aligned values reads as a "table" (real false-positive
        // risk), and the relocation guard that defends it is weak on non-PIE EXEC
        // (absolute .text pointers carry no relocation). NOTE this is NOT switch
        // recovery (that is the INHERITED S2 engine machinery, s2_lift/jumptable.rs)
        // and NOT the roadmap-#9 post-typing refinement (that is an engine S2 feedback
        // change behind the Override::queryMultistageJumptable seam) — it is only the
        // application-layer absolute-pointer-table discovery. See docs/analysis-port-log.md.
        // Box::new(crate::s1_addrtable::AddrTablePass { min_run: 2 }),

        // DELIBERATELY ABSENT — `AggressiveInstructionFinderAnalyzer` (AIF, + the ARM
        // variant). Not ported: it is a *post-disassembly* speculative gap-filler that
        // requires a fully-populated Listing/FunctionManager (≥20 found functions) +
        // a PseudoDisassembler — none of which exist at the kuna-analysis tier (which
        // runs *before* decompilation). It is off-by-default upstream
        // (`setDefaultEnablement(false)`) and its sound output (new entries) is
        // subsumed by `s1-entry-disc` + `s1-eh-frame`. Documented ⛔ infeasible-at-tier,
        // same as `FindNoReturnFunctionsAnalyzer` (see s1_loader/noreturn.rs and
        // docs/analysis-port-log.md). No `AnalysisPass` impl exists for it.
    ]
}

/// Parse `bytes` as an object file, build an [`AnalysisCtx`], and run every
/// [`default_passes`] pass, returning the merged [`AnalysisOutput`].
///
/// This keeps `object::File` an internal concern of `kuna-analysis` (the crate
/// that already depends on `object`): the console hands in the image bytes, the
/// opened [`ObjectLoadImage`], and the resolved [`Architecture`], and gets back
/// flat facts to commit. A parse failure yields an empty output (the analyses
/// are additive and never fail — the funcsym stream is committed separately).
pub fn run_default_analyses(
    bytes: &[u8],
    image: &ObjectLoadImage,
    arch: &Architecture,
) -> AnalysisOutput {
    let Ok(file) = object::File::parse(bytes) else {
        return AnalysisOutput::default();
    };
    let ctx = AnalysisCtx { file: &file, image, arch };
    run_analyses(&ctx, &default_passes())
}
