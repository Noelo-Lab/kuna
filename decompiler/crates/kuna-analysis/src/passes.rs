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
use crate::s1_sourcelang::Compiler;

/// The default program-prep passes, in stage order.
///
/// These run at load time over the parsed ELF and produce an additive
/// [`AnalysisOutput`] the console commits into the engine. Adding a new analysis
/// is: implement [`AnalysisPass`] in an `s1_*` module, then add it here.
///
/// This is the back-compat entry: it builds the pass list as if the source
/// language were [`Compiler::Unknown`] (no Rust widening). The real bootstrap
/// path uses [`passes_for`] with the detected compiler.
pub fn default_passes() -> Vec<Box<dyn AnalysisPass>> {
    passes_for(Compiler::Unknown)
}

/// Build the program-prep pass list for a detected source-language
/// [`Compiler`]. This is the kuna analog of Ghidra's source-language-gated
/// analyzer selection (`SourceLanguageAnalyzer` records the IDs;
/// language-specific analyzers gate on them). Today the only gate is the
/// no-return list widening (`noReturnFunctionConstraints.xml`'s per-`<compiler>`
/// arms: the `rustc` arm adds the Rust list, the `golang` arm adds the Go list);
/// future Rust/Go-specific passes plug in here with one line.
pub fn passes_for(compiler: Compiler) -> Vec<Box<dyn AnalysisPass>> {
    vec![
        // S1 loader: known no-return functions (exit/abort/…). Mirrors Ghidra's
        // default-on `NoReturnFunctionAnalyzer`. For a Rust binary, also match the
        // Rust wildcard list (panic/handle_alloc_error/rust_begin_unwind/…); for a
        // Go binary, also match the Golang list (runtime.gopanic/throw/goexit/…).
        // `for_compiler` selects the matching `<compiler>` arm
        // (noReturnFunctionConstraints.xml), base ELF list only otherwise.
        Box::new(crate::s1_loader::noreturn::NoReturnKnownPass::for_compiler(compiler)),
        // S1 strings (StringLiteralPass): NUL-terminated ASCII string-literal
        // detection. Mirrors Ghidra's `StringsAnalyzer` (min length 5,
        // require-NUL-end). Plants a typelocked `char[N]` data symbol (`s_<addr>`)
        // at each detected `.rodata` string. ENABLED by default since the printer
        // change (the readonly-char-array literal route in
        // `s9_emit/printc.rs::op_ptrsub_ir`): a pointer to a readonly char-printable
        // array symbol now renders as the string LITERAL (Ghidra behavior), so the
        // data symbol and the literal coexist instead of the symbol name shadowing
        // it. See docs/analysis-port-log.md (the strings/printer increment).
        Box::new(crate::s1_strings::StringLiteralPass { min_len: 5 }),
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
        // S1 ARM/Thumb decode-mode markers: paint the SLEIGH `TMode` context
        // variable from ARM mapping symbols (`$t`/`$a`) + the STT_FUNC odd-address
        // (LSB=1 ⇒ Thumb) convention, so Thumb code decodes as Thumb. The kuna
        // analog of ARM's `ARM_ElfExtension.evaluateElfSymbol` + the later
        // `ArmSymbolAnalyzer`. ARM-only: the pass returns an empty output on every
        // other language (its `canAnalyze == processor==ARM` gate), and the
        // commit seam additionally swallows a "TMode not registered" error, so
        // this is a strict no-op for every non-ARM binary (the parity gates are
        // structurally untouched). Always-on, like noreturn/libproto/entry.
        Box::new(crate::s1_loader::arm_markers::ArmMarkerPass),
        // S1 MIPS `$gp` recovery: seed `t9 = func_entry` as a tracked register
        // value at each MIPS function entry (the PIC `jalr t9` ABI convention), so
        // a PIC prologue's `addu gp,gp,t9` folds to the real `$gp` and
        // `$gp`-relative GOT/`.sdata` loads resolve. The kuna analog of Ghidra's
        // `MipsAddressAnalyzer` (the `assumeT9EntryAddress` default-on path). Unlike
        // ARM markers this is a register-VALUE seed (TrackedSet/TrackedContext), not
        // a decode-mode context bit. MIPS-only: the pass returns an empty output on
        // every other language (its `canAnalyze == processor==MIPS` gate), and the
        // commit seam additionally swallows a "t9 not found" error, so this is a
        // strict no-op for every non-MIPS binary (the parity gates are structurally
        // untouched). Always-on, like noreturn/libproto/entry/arm_markers;
        // `--option mips_gp off` restores the un-tracked (raw `$gp`) rendering.
        Box::new(crate::s1_loader::mips_markers::MipsMarkerPass),
        // S1 MIPS16 ISA_MODE decode-mode markers: paint the SLEIGH `ISA_MODE`
        // context variable at each MIPS16e/microMIPS function entry (marked by the
        // STT_FUNC LSB-set address OR `st_other & 0xf0` = STO_MIPS_MIPS16/MICROMIPS)
        // so the body decodes in the alternate ISA instead of being misdecoded as
        // MIPS32. The exact MIPS analog of ARM's `$t`/STT_FUNC-LSB → `TMode`
        // painting (arm_markers) — a decode-mode context BIT, not a register value
        // like `mips_gp`'s `t9`. The kuna analog of Ghidra's
        // `MIPS_ElfExtension.applyIsaMode`. MIPS-only: the pass returns an empty
        // output on every other language (its `processor==MIPS` gate), and the
        // commit seam additionally swallows an "ISA_MODE not registered" error
        // (the same context-paint arm the ARM `TMode` paints use), so this is a
        // strict no-op for every non-MIPS binary (the parity gates are structurally
        // untouched). Always-on, like arm_markers/mips_gp; `--option mips_isa off`
        // restores the un-painted (MIPS32-misdecoded) rendering.
        Box::new(crate::s1_loader::mips_markers::MipsIsaModePass),
        // S1 DWARF: recover function/global names and TYPED function signatures
        // from `.debug_*` sections (the kuna analog of Ghidra's `DWARFAnalyzer`).
        // Registered AFTER LibProtoPass so for any name both emit, the DWARF
        // (real source) prototype wins (last-write in set_function_prototype_pieces).
        // Skips cleanly on a non-DWARF binary. Subtask-3 (DW_OP_fbreg stack-local
        // ScopeLocal map) is a deferred engine change — see s1_dwarf docs.
        Box::new(crate::s1_dwarf::DwarfPass),
        // S1 call-fixups: tag each function whose name matches a cspec call-fixup
        // `<target>` (e.g. the `-pg` `mcount`/`__fentry__` profiling stubs) so the
        // engine replaces the CALL with the fixup body. The kuna analog of Ghidra's
        // default-on `CallFixupAnalyzer` (the install half; the flow-repair half is
        // engine-internal at this tier — see s1_callfixup docs). After LibProtoPass
        // (the fixup tags the *callee* function, independent of prototype seeding).
        // Always-on, like noreturn/libproto/entry; `--option callfixup off` restores
        // the un-fixed rendering.
        Box::new(crate::s1_callfixup::CallFixupPass),

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
    // Source-language detection runs once, before pass selection, and shapes the
    // pass list (the kuna analog of `SourceLanguageAnalyzer` running early and
    // gating the language-specific analyzers).
    let compiler = crate::s1_sourcelang::detect_compiler(&file);
    let ctx = AnalysisCtx { file: &file, image, arch };
    run_analyses(&ctx, &passes_for(compiler))
}

/// Like [`run_default_analyses`], but keep each pass's output keyed by its
/// [`AnalysisPass::id`] instead of merging.
///
/// This is the per-run-gating entry: the console stashes the per-pass split at
/// load and commits only the **enabled** passes' facts at `read symbols` (after
/// the per-pass `--option <id> on|off` flags have been applied). The id is the
/// pass's `id()` string, which doubles as its settable-option name (`stages.toml`
/// + `KUNA_OPTION_NAMES`). A parse failure yields an empty list (additive, never
/// fails). Pairs preserve pass order so the commit order is deterministic.
pub fn run_default_analyses_per_pass(
    bytes: &[u8],
    image: &ObjectLoadImage,
    arch: &Architecture,
) -> Vec<(&'static str, AnalysisOutput)> {
    let Ok(file) = object::File::parse(bytes) else {
        return Vec::new();
    };
    let compiler = crate::s1_sourcelang::detect_compiler(&file);
    let ctx = AnalysisCtx { file: &file, image, arch };
    passes_for(compiler)
        .iter()
        .map(|pass| (pass.id(), pass.run(&ctx)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pass::AnalysisPass;

    fn ids(passes: &[Box<dyn AnalysisPass>]) -> Vec<&'static str> {
        passes.iter().map(|p| p.id()).collect()
    }

    /// `passes_for(Unknown)` MUST be exactly today's `default_passes()` contents
    /// — the no-Rust default must never silently drop a pass (the guard the
    /// sourcelang brief calls for).
    #[test]
    fn unknown_matches_default_passes() {
        assert_eq!(ids(&passes_for(Compiler::Unknown)), ids(&default_passes()));
        // Both must still carry the always-on analysis passes.
        let want = ["noreturn_known", "libproto"];
        for id in want {
            assert!(
                ids(&default_passes()).contains(&id),
                "default_passes must include {id}"
            );
        }
    }

    /// Rust/Go vs non-Rust/Go selection differs ONLY in the no-return pass
    /// variant (the per-compiler list it folds in), not the pass set — the same
    /// ids in the same order across all detected compilers.
    #[test]
    fn all_compilers_have_same_pass_ids() {
        let base = ids(&passes_for(Compiler::Gcc));
        for c in [Compiler::Rustc, Compiler::Go, Compiler::Clang, Compiler::Unknown] {
            assert_eq!(ids(&passes_for(c)), base, "{c:?} pass ids must match the base set");
        }
    }

    /// The arch-marker passes are registered (always-on): ARM `TMode` painting,
    /// MIPS `$gp` (`t9`) recovery, and MIPS16 `ISA_MODE` painting. All gate
    /// internally on their architecture, so listing them here is safe on every
    /// binary.
    #[test]
    fn arch_marker_passes_registered() {
        let d = ids(&default_passes());
        assert!(d.contains(&"arm_markers"), "arm_markers must be registered");
        assert!(d.contains(&"mips_gp"), "mips_gp must be registered");
        assert!(d.contains(&"mips_isa"), "mips_isa must be registered");
    }
}
