//! The default program-prep pass list + the driver entry point the bootstrap
//! boundary calls.
//!
//! [`default_passes`] is the registry of enabled analyses, in stage order. Each
//! pass's [`AnalysisPass::id`](crate::pass::AnalysisPass::id) is its gate name;
//! the passes here are **bound to the real-ELF bootstrap path only** (the XML
//! `<binaryimage>` datatest path never calls [`run_default_analyses`]), so the
//! 675/675 datatest parity oracle is structurally untouched regardless of what
//! is enabled — see the downstream-compatibility note in
//! `docs/missing-analyses.md`.

use kuna_decomp::architecture::Architecture;
use kuna_sleigh::translate::Translate;

use crate::loadimage_object::ObjectLoadImage;
use crate::pass::{run_analyses, AnalysisCtx, AnalysisOutput, AnalysisPass};
use crate::sourcelang::Compiler;

/// The default program-prep passes, in stage order.
///
/// These run at load time over the parsed ELF and produce an additive
/// [`AnalysisOutput`] the console commits into the engine. Adding a new analysis
/// is: implement [`AnalysisPass`] in a module under `analyzers/`, then add it here.
///
/// This is the back-compat entry: it builds the pass list as if the source
/// language were [`Compiler::Unknown`] (no Rust widening) on a non-PE image (no
/// MSVC-RTTI pass). The real bootstrap path uses [`passes_for`] with the detected
/// compiler + the parsed object's format.
pub fn default_passes() -> Vec<Box<dyn AnalysisPass>> {
    // A neutral non-PE/non-Mach-O format (`Elf`): the back-compat entry never
    // carries the PE-only `rtti` pass nor the Mach-O-only `objc` pass, so the
    // default set is byte-identical to before those passes.
    passes_for(Compiler::Unknown, object::BinaryFormat::Elf)
}

/// Build the program-prep pass list for a detected source-language [`Compiler`] and
/// the parsed object's `format`. This is the kuna analog of Ghidra's
/// source-language-gated AND loader-format-gated analyzer selection
/// (`SourceLanguageAnalyzer` records the IDs; `RttiAnalyzer.canAnalyze` gates on a
/// PE/Microsoft program). The gates here are:
/// - the no-return list widening (`noReturnFunctionConstraints.xml`'s
///   per-`<compiler>` arms: the `rustc` arm adds the Rust list, the `golang` arm
///   adds the Go list, plus the Go-only `gopclntab` pass),
/// - the MSVC RTTI / vftable pass (`rtti`), registered ONLY on a PE image (the
///   Microsoft C++ ABI is a PE concern — `BinaryFormat::Pe`), so a non-PE binary's
///   pass set is byte-identical to before this pass existed, and
/// - the Objective-C method-name pass (`objc`), registered ONLY on a Mach-O image
///   (the Objective-C runtime metadata is a Mach-O concern — `BinaryFormat::MachO`),
///   so a non-Mach-O binary's pass set is byte-identical to before this pass existed.
pub fn passes_for(compiler: Compiler, format: object::BinaryFormat) -> Vec<Box<dyn AnalysisPass>> {
    let mut passes: Vec<Box<dyn AnalysisPass>> = vec![
        // S1 loader: known no-return functions (exit/abort/…). Mirrors Ghidra's
        // default-on `NoReturnFunctionAnalyzer`. For a Rust binary, also match the
        // Rust wildcard list (panic/handle_alloc_error/rust_begin_unwind/…); for a
        // Go binary, also match the Golang list (runtime.gopanic/throw/goexit/…).
        // `for_compiler` selects the matching `<compiler>` arm
        // (noReturnFunctionConstraints.xml), base ELF list only otherwise.
        Box::new(crate::loader::noreturn::NoReturnKnownPass::for_compiler(compiler)),
        // S1 strings (StringLiteralPass): NUL-terminated ASCII string-literal
        // detection. Mirrors Ghidra's `StringsAnalyzer` (min length 5,
        // require-NUL-end). Plants a typelocked `char[N]` data symbol (`s_<addr>`)
        // at each detected `.rodata` string. ENABLED by default since the printer
        // change (the readonly-char-array literal route in
        // `p9_emit/printc.rs::op_ptrsub_ir`): a pointer to a readonly char-printable
        // array symbol now renders as the string LITERAL (Ghidra behavior), so the
        // data symbol and the literal coexist instead of the symbol name shadowing
        // it. See docs/history/analysis-port-log.md (the strings/printer increment).
        Box::new(crate::strings::StringLiteralPass { min_len: 5 }),
        // S1 library prototypes: seed common libc signatures (puts(char*), …) so
        // call arguments get typed. Mirrors Ghidra's `ApplyDataArchiveAnalyzer`.
        // THIS is what renders string literals in kuna: typing a call argument
        // `char *` lets the printer's pointer-char-constant path read the readonly
        // bytes via the StringManager and emit `puts("Username: ")`.
        Box::new(crate::protos::LibProtoPass),
        // S1 entry discovery: find function entry points for stripped targets —
        // ELF e_entry, DT_INIT/DT_FINI + INIT_ARRAY/FINI_ARRAY pointer tables,
        // `.eh_frame` FDE pcBegin starts, the x86-64 `_start`→`main` libc-start
        // idiom, and conservative prologue byte patterns. Fuses Ghidra's
        // EntryPointAnalyzer/ExternalEntryFunctionAnalyzer/FunctionStartAnalyzer
        // + the GccExceptionAnalyzer `.eh_frame` FDE oracle into one additive pass
        // (the commit boundary's `out.entries` arm names + adds each discovered VMA,
        // idempotent against the funcsym stream). After LibProtoPass so prototypes
        // are seeded first. Always-on, like noreturn/libproto.
        Box::new(crate::entry::EntryDiscoveryPass),
        // S1 `.eh_frame` LSDA landing-pad discovery (`eh_frame_full`): the
        // GccExceptionAnalyzer full `.gcc_except_table` markup. For each FDE, follow
        // the CIE `L` augmentation to its LSDA pointer in `.gcc_except_table`,
        // decode the call-site table, and emit each exception-handler landing pad
        // (catch/cleanup block, reached only by the unwinder) as a discovered
        // function entry — net-new code targets `EntryDiscoveryPass`'s FDE-pcBegin /
        // prologue / libc-start oracles never see (a landing pad sits mid-function).
        // Registered always (the facts are computed + stashed at load), but the
        // commit is GATED by `--option eh_frame_full on` (default-OFF,
        // output-changing: it ADDS entries) via `engine.rs::analysis_pass_enabled`.
        // A default run therefore commits nothing here and the discovery set is
        // byte-identical to the FDE-pcBegin-only behavior. After EntryDiscoveryPass
        // (this is the deeper `.eh_frame` markup). The DW_CFA_* call-frame
        // instructions are NOT recovered — kuna's S5/S7 frame analysis already
        // recovers the stack frame from the code, so CFI is inherited, not rebuilt.
        Box::new(crate::entry::EhFrameLsdaPass),
        // S1 full byte-pattern function starts (FuncStartPatternPass): the faithful
        // port of Ghidra's `FunctionStartAnalyzer` over the ENTIRE vendored pattern
        // corpus (`entry/patterns/*.xml`: the `<patternpairs>` pre/post sequences
        // + bare `<funcstart/>` patterns, x86/x86-64 headline + AArch64/ARM/RISC-V/
        // MIPS/PPC). Unlike `EntryDiscoveryPass`'s always-on minimal oracle 5 (three
        // bare x86-64 prologues), this applies the full set with the upstream
        // pre/post matching: a candidate is a start iff a postpattern matches at it
        // AND a prepattern matches the bytes immediately before it. Default-**OFF**
        // (output-changing: discovers more functions): registered always, but its
        // facts are dropped at commit unless `--option funcstart_patterns on`
        // (`engine.rs::analysis_pass_enabled` reads `arch.analysis_funcstart_patterns`,
        // default false), so a default run is byte-identical. After EntryDiscoveryPass
        // (its discoveries are a superset; the commit boundary dedups against the entries
        // EntryDiscoveryPass already emits). See `entry::FuncStartPatternPass`.
        Box::new(crate::entry::FuncStartPatternPass),
        // S1 ARM/Thumb decode-mode markers: paint the SLEIGH `TMode` context
        // variable from ARM mapping symbols (`$t`/`$a`) + the STT_FUNC odd-address
        // (LSB=1 ⇒ Thumb) convention, so Thumb code decodes as Thumb. The kuna
        // analog of ARM's `ARM_ElfExtension.evaluateElfSymbol` + the later
        // `ArmSymbolAnalyzer`. ARM-only: the pass returns an empty output on every
        // other language (its `canAnalyze == processor==ARM` gate), and the
        // commit boundary additionally swallows a "TMode not registered" error, so
        // this is a strict no-op for every non-ARM binary (the parity gates are
        // structurally untouched). Always-on, like noreturn/libproto/entry.
        Box::new(crate::loader::arm_markers::ArmMarkerPass),
        // S1 MIPS `$gp` recovery: seed `t9 = func_entry` as a tracked register
        // value at each MIPS function entry (the PIC `jalr t9` ABI convention), so
        // a PIC prologue's `addu gp,gp,t9` folds to the real `$gp` and
        // `$gp`-relative GOT/`.sdata` loads resolve. The kuna analog of Ghidra's
        // `MipsAddressAnalyzer` (the `assumeT9EntryAddress` default-on path). Unlike
        // ARM markers this is a register-VALUE seed (TrackedSet/TrackedContext), not
        // a decode-mode context bit. MIPS-only: the pass returns an empty output on
        // every other language (its `canAnalyze == processor==MIPS` gate), and the
        // commit boundary additionally swallows a "t9 not found" error, so this is a
        // strict no-op for every non-MIPS binary (the parity gates are structurally
        // untouched). Always-on, like noreturn/libproto/entry/arm_markers;
        // `--option mips_gp off` restores the un-tracked (raw `$gp`) rendering.
        Box::new(crate::loader::mips_markers::MipsMarkerPass),
        // S1 MIPS16 ISA_MODE decode-mode markers: paint the SLEIGH `ISA_MODE`
        // context variable at each MIPS16e/microMIPS function entry (marked by the
        // STT_FUNC LSB-set address OR `st_other & 0xf0` = STO_MIPS_MIPS16/MICROMIPS)
        // so the body decodes in the alternate ISA instead of being misdecoded as
        // MIPS32. The exact MIPS analog of ARM's `$t`/STT_FUNC-LSB → `TMode`
        // painting (arm_markers) — a decode-mode context BIT, not a register value
        // like `mips_gp`'s `t9`. The kuna analog of Ghidra's
        // `MIPS_ElfExtension.applyIsaMode`. MIPS-only: the pass returns an empty
        // output on every other language (its `processor==MIPS` gate), and the
        // commit boundary additionally swallows an "ISA_MODE not registered" error
        // (the same context-paint arm the ARM `TMode` paints use), so this is a
        // strict no-op for every non-MIPS binary (the parity gates are structurally
        // untouched). Always-on, like arm_markers/mips_gp; `--option mips_isa off`
        // restores the un-painted (MIPS32-misdecoded) rendering.
        Box::new(crate::loader::mips_markers::MipsIsaModePass),
        // S1 DWARF: recover function/global names and TYPED function signatures
        // from `.debug_*` sections (the kuna analog of Ghidra's `DWARFAnalyzer`).
        // Registered AFTER LibProtoPass so for any name both emit, the DWARF
        // (real source) prototype wins (last-write in set_function_prototype_pieces).
        // Skips cleanly on a non-DWARF binary. Subtask-3 (DW_OP_fbreg stack-local
        // ScopeLocal map) is a deferred engine change — see dwarf docs.
        Box::new(crate::dwarf::DwarfPass),
        // S1 DWARF source lines: parse `.debug_line` and surface each
        // instruction's `file:line` as a `Comment::user2` on the decompiled
        // output (the kuna analog of Ghidra's `DWARFLineInfoCommentScript`).
        // Default-OFF (`--option dwarf_lines on`): it CHANGES the output (adds
        // comment lines), so unlike the names/types `dwarf` pass it is opt-in.
        // Registered after `DwarfPass` (it reuses the same `.debug_*` sections).
        Box::new(crate::dwarf::DwarfLinesPass),
        // S1 call-fixups: tag each function whose name matches a cspec call-fixup
        // `<target>` (e.g. the `-pg` `mcount`/`__fentry__` profiling stubs) so the
        // engine replaces the CALL with the fixup body. The kuna analog of Ghidra's
        // default-on `CallFixupAnalyzer` (the install half; the flow-repair half is
        // engine-internal at this tier — see callfixup docs). After LibProtoPass
        // (the fixup tags the *callee* function, independent of prototype seeding).
        // Always-on, like noreturn/libproto/entry; `--option callfixup off` restores
        // the un-fixed rendering.
        Box::new(crate::callfixup::CallFixupPass),

        // S1 address tables (AddrTablePass) is implemented + tested but **disabled
        // by default**, matching Ghidra's `AddressTableAnalyzer.setDefaultEnablement(false)`.
        // It scans .rodata/.data for a run of consecutive pointer-width values that
        // all land inside an executable section (an absolute address/jump table) and
        // emits Data symbols + a readonly range. It is OFF because (a) Ghidra ships
        // it off (parity); (b) a pointer-run scanner over-accepts — any coincidental
        // run of in-range aligned values reads as a "table" (real false-positive
        // risk), and the relocation guard that defends it is weak on non-PIE EXEC
        // (absolute .text pointers carry no relocation). NOTE this is NOT switch
        // recovery (that is the INHERITED S2 engine machinery, p2_lift/jumptable.rs)
        // and NOT the roadmap-#9 post-typing refinement (that is an engine S2 feedback
        // change behind the Override::queryMultistageJumptable hook) — it is only the
        // application-layer absolute-pointer-table discovery. See docs/history/analysis-port-log.md.
        // Box::new(crate::addrtable::AddrTablePass { min_run: 2 }),

        // S1 scalar/operand reference markup (OperandRefsPass) is implemented +
        // tested but **gated off by default** AND runs DEFERRED (not in this
        // load-time list) — the kuna analog of Ghidra's `ScalarOperandAnalyzer` /
        // `ElfScalarOperandAnalyzer`. Like the Listing/xref tier (PR6), it must run
        // at the deferred commit point (`read symbols`) rather than at load: it
        // linear-decodes the executable sections via the engine `Translate`, and the
        // program loadimage is only attached to the engine's `Sleigh` AFTER this
        // load-time pass list runs (`set_loader` in `bootstrap_from_object`), so a
        // load-time decode finds no bytes. It is therefore driven from
        // [`run_operand_refs`] in `commit_pending_analysis`, gated on
        // `analysis_operand_refs` (`--option operand_refs on`). OFF by default
        // because (a) Ghidra ships the producing analyzer `setDefaultEnablement` =
        // `!isElf` (disabled for every ELF) and the ELF subclass only *removes* bad
        // `.got`/`.plt` refs kuna never creates; (b) its one useful product (a
        // `.rodata` string typed as `char*`) is already delivered by the always-on
        // `strings` + libproto/S5 typing for the common case; (c) a per-instruction
        // immediate scan over-accepts. See docs/history/analysis-port-buildplan.md §1.2.

        // `AggressiveInstructionFinderAnalyzer` (AIF) is NOT a pure-`ctx` pass here:
        // it is the third *Listing/xref consumer* (`aif`, the sound substitute the
        // buildplan §1.3 prescribed, gated off-by-default like upstream's
        // `setDefaultEnablement(false)`). It speculatively decodes the undefined gaps
        // the Listing left, which needs the live SLEIGH decoder, so it is driven by
        // `aif::run_aif` inside `run_listing_consumers` (below), not from this
        // load-time pure-ctx list. The `AggressiveInstructionFinderPass` `AnalysisPass`
        // impl exists only for the `aif` gate identity (its `run` is a no-op). See
        // `aif/mod.rs` + docs/history/analysis-port-log.md.
    ];

    // S1 Go pclntab function-name recovery (GoPclntabPass): when the binary is Go
    // (`detect_compiler == Go`, via `.go.buildinfo`/`.note.go.buildid` — the same
    // gate the Go no-return list uses), parse the embedded `pclntab` and emit a
    // `SymFact { kind: Function }` per Go function, so `main.main`/`runtime.*`/
    // package functions render NAMED instead of `sub_<addr>`. The kuna analog of
    // Ghidra's `GolangSymbolAnalyzer` (name-recovery half). Go-only: registered
    // ONLY for `Compiler::Go`, so every non-Go binary's pass set is byte-identical
    // to before (the no-op is structural, not a runtime check). `--option
    // gopclntab off` suppresses it; the existing symbol commit arm installs the
    // facts idempotently (a real `.symtab` name still wins, so only a *stripped*
    // Go binary's `sub_<addr>` functions take the recovered name).
    if compiler.is_golang() {
        passes.push(Box::new(crate::pclntab::GoPclntabPass));
    }

    // S1 MSVC RTTI / vftable recovery (RttiPass): on a Windows PE, parse the
    // CompleteObjectLocator → RTTI3/2/1 → RTTI0 graph in `.rdata`/`.data`, demangle
    // each `.?A…@@` class name, and emit `<Class>::vftable` / `<Class>::RTTI_*`
    // labels + the recovered class names (`Box`/`Shape`). The kuna analog of
    // Ghidra's `RttiAnalyzer` (a Microsoft-PE analyzer). PE-only: registered ONLY
    // for `BinaryFormat::Pe`, so every non-PE binary's pass set is byte-identical to
    // before (the no-op is structural, not a runtime check — and the pass ALSO
    // self-gates on PE in `run`). DEFAULT-OFF: its facts are computed at load but
    // committed only when `--option rtti on` (`engine.rs::analysis_pass_enabled`
    // reads `arch.analysis_rtti`, default false), so a default run is byte-identical
    // and bound to the real-PE path (the XML datatest oracle is untouched).
    if format == object::BinaryFormat::Pe {
        passes.push(Box::new(crate::rtti::RttiPass));
    }

    // S1 Mach-O Objective-C metadata recovery (ObjcMetadataPass): when the binary
    // is a Mach-O, walk the `__objc_*` metadata (classlist → class_t → class_ro_t
    // → method_list_t) and rename each IMP function `-[Class sel]` / `+[Class sel]`
    // (the FID-precedent label-gated rename) + emit `_OBJC_CLASS_$_<name>` /
    // selector symbols. The kuna analog of Ghidra's ObjcTypeMetadataAnalyzer
    // (name-recovery half). Registered ONLY for a Mach-O binary, so every non-Mach-O
    // target's pass set is byte-identical to before (the gate is structural, like
    // the Go pclntab pass). Default-OFF (`--option objc`, the fid precedent): its
    // facts are computed at LOAD but COMMITTED only when the `objc` gate is on
    // (`engine.rs::analysis_pass_enabled`), so a default run is byte-identical.
    // Selectors are plain ASCII — no demangler needed. x86-64, no-chained-fixups
    // path (the arm64 + LC_DYLD_CHAINED_FIXUPS resolver is the deferred PR-O0/O2).
    if format == object::BinaryFormat::MachO {
        passes.push(Box::new(crate::objc::ObjcMetadataPass));
    }

    // S1 PE PDB metadata recovery (PdbPass): on a Windows PE, read the CodeView
    // fingerprint (`{guid, age, path}`), locate the external `.pdb` (tier-1: the
    // `kuna_pdb_path` env var, the fid `kuna_fid_db` precedent), fingerprint-gate
    // it (the supplied `.pdb`'s guid/age must match the PE's CodeView record — never
    // apply a wrong/stale PDB, the FID full-hash-match discipline), and on a match
    // walk the global symbols (S_PUB32/S_GPROC32) to RENAME each stripped FUN_*/sub_*
    // function to its real name (`out.fid_names`, the label-gated rename) + emit the
    // function symbol (`out.symbols`). The kuna analog of Ghidra's
    // PdbUniversalAnalyzer (name-recovery half). PE-only: registered ONLY for
    // `BinaryFormat::Pe`, so every non-PE binary's pass set is byte-identical to
    // before (the gate is structural, and the pass ALSO self-gates on PE in `run`).
    // DEFAULT-OFF: its facts are computed at load but committed only when `--option
    // pdb on` (`engine.rs::analysis_pass_enabled` reads `arch.analysis_pdb`, default
    // false) AND a fingerprint-matching `.pdb` is supplied via `kuna_pdb_path`, so a
    // default run is byte-identical and bound to the real-PE path (the XML datatest
    // oracle is untouched). Types/typed-locals/lines are the deferred PR-P2/P3.
    if format == object::BinaryFormat::Pe {
        passes.push(Box::new(crate::pdb::PdbPass));
    }

    passes
}

/// Build the Listing/xref tier's seed set from a parsed object: the union of
/// the real funcsym entries (`entry::existing_function_addrs`) and the
/// discovered entry points (`entry::collect_entries`), restricted to
/// executable sections, sorted and deduped (design §3.1).
///
/// Both halves are already exec-section-filtered upstream (`collect_entries`
/// filters; `existing_function_addrs` are real FUNC syms), but we apply the
/// `in_executable_section` gate to both to be robust against a funcsym pointing
/// at a non-exec address.
///
/// `pub` so the cross-crate `verify_listing_*` gates can build the *exact* seed
/// set the live driver uses (the build-through-engine proof), instead of
/// reconstructing it from the `pub(crate)` `entry` helpers.
pub fn listing_seeds(file: &object::File, bytes: &[u8]) -> Vec<u64> {
    let execs = crate::entry::executable_sections(file);
    let mut seeds: Vec<u64> = crate::entry::existing_function_addrs(file, bytes)
        .into_iter()
        .chain(crate::entry::collect_entries(file, bytes))
        .filter(|&vma| crate::entry::in_executable_section(&execs, vma))
        .collect();
    seeds.sort_unstable();
    seeds.dedup();
    seeds
}

/// The Listing/xref-tier **consumer** passes — those that read the built Listing
/// (`ctx.listing`) instead of (or in addition to) the parsed object. They are
/// kept OUT of [`passes_for`] because they only run when the Listing has been
/// built, which (per the build-timing fix) happens at the deferred commit point
/// (`read symbols`), not at load — see [`run_listing_consumers`].
///
/// The consumers are the discovered-no-return analyzer (`noreturn_disc`, the kuna
/// analog of Ghidra's `FindNoReturnFunctionsAnalyzer`), the structural no-return
/// propagation analyzer (`noreturn_propagate`, the kuna analog of angr's CFGFast
/// call-graph no-return propagation), and the FID fingerprint matcher (`fid`, the
/// kuna analog of Ghidra's FID identification analyzer — re-identify a stripped
/// function by full-hash fingerprint). Each is still gated by its own
/// `--option <id> on|off` flag at commit time
/// (`engine.rs::analysis_pass_enabled`), so a default run skips it.
fn listing_consumer_passes() -> Vec<Box<dyn AnalysisPass>> {
    vec![
        Box::new(crate::noreturn_disc::NoReturnDiscoveredPass),
        Box::new(crate::noreturn_propagate::NoReturnPropagatePass),
        Box::new(crate::fid::FidPass),
    ]
}

/// Build the Listing/xref tier and run the Listing **consumer** passes over it,
/// returning each consumer's output keyed by its [`AnalysisPass::id`] (the same
/// per-pass-split shape as [`run_default_analyses_per_pass`]).
///
/// This is the deferred half of the build-timing fix (PR6). The non-Listing
/// passes run at load (`run_default_analyses_per_pass`), but the Listing build is
/// gated on `--option listing on`, a flag the live CLI sets *after* `load file`
/// (before `read symbols`). So the Listing — and any pass that reads it — must be
/// built/run at the deferred commit point, when the flag is finally in effect.
/// The console calls this from `commit_pending_analysis` (reached at `read
/// symbols`), gated on `arch.analysis_listing`; it parses `bytes`, builds the
/// Listing with funcsym names + Known-no-return/call-fixup seed metadata, and runs
/// every [`listing_consumer_passes`] pass.
///
/// A parse failure (or no exec ranges) yields an empty list (additive, never
/// fails). Bound to the real-ELF path: the XML datatest path never calls this, so
/// the parity oracles are structurally untouched.
///
/// `noreturn_seeds`/`callfixup_seeds` are the addresses the load-time Known
/// no-return / call-fixup passes flagged (so the Listing's `DiscoveredFunction`s
/// carry `has_no_return`/`call_fixup`, letting the consumer skip already-modeled
/// callees and seed the fixpoint's terminal set faithfully).
pub fn run_listing_consumers(
    bytes: &[u8],
    image: &ObjectLoadImage,
    arch: &Architecture,
    translate: &dyn Translate,
    noreturn_seeds: &[u64],
    callfixup_seeds: &[u64],
) -> Vec<(&'static str, AnalysisOutput)> {
    let Ok(file) = object::File::parse(bytes) else {
        return Vec::new();
    };
    let mut seeds = listing_seeds(&file, bytes);
    // (kuna, recursive-descent discovery) When the prologue-pattern pass is active
    // (`funcstart_patterns`, default-ON for non-x86-64 on `decompile-all`, DIV-20),
    // seed the recursive-descent walk with its `<patternpairs>` function starts too — not
    // just the entry oracles. On a STRIPPED ARM binary the entry oracles seed only the ELF
    // entry, so the walk discovers ~nothing; seeded with the prologue starts it explores
    // the CALL graph out of every discovered function and finds call-only targets that
    // have no recognizable prologue (Ghidra's disassembler-driven recursive descent).
    // betaflight STM32F405: the walk grows 1 -> 1470 discovered functions (Ghidra 1822).
    // Gated by the same flag, so x86-64 (funcstart_patterns off) is unchanged.
    if arch.analysis_funcstart_patterns {
        let execs = crate::entry::executable_sections(&file);
        seeds.extend(
            crate::entry::full_pattern_starts(&file)
                .into_iter()
                .filter(|&vma| crate::entry::in_executable_section(&execs, vma)),
        );
        seeds.sort_unstable();
        seeds.dedup();
    }
    let seed_names = funcsym_names(&file);
    let funcsym_seeds = crate::entry::existing_function_addrs(&file, bytes);
    let mut listing = crate::listing::Listing::build_with_meta(
        &file,
        image,
        arch,
        translate,
        &seeds,
        &funcsym_seeds,
        &seed_names,
    );
    // (kuna, Stage-2 ARM discovery) Raw, UNPAIRED Thumb-prologue gap seeding — the
    // angr `CFGFast._func_addrs_from_prologues()` mirror. After the first walk, scan
    // for canonical LR-saving Thumb prologues (`PUSH {..,lr}` / `PUSH.W {..,lr}`)
    // that landed in an UNDEFINED gap (never epilogue-paired by `<patternpairs>`,
    // never reached by a direct BL, and skipped by AIF's cursor-advancing gap-walk),
    // validate each with `check_valid_subroutine`, and RE-SEED the walk with the
    // survivors so it expands each into a full function + discovers its callees. The
    // guards (gap-only via the walk's coverage, `check_valid_subroutine`, and the
    // body-claim dedup) keep precision (angr measured the raw prologues at ~93%).
    // Gated by the same `funcstart_patterns` flag (ARM-only inside
    // `raw_thumb_prologue_seeds`), so x86-64 (funcstart_patterns off) is unchanged
    // and every non-ARM binary is a strict no-op. betaflight STM32F405: recovers the
    // ~483 PUSH-prologue functions the `<patternpairs>` matcher structurally misses.
    if arch.analysis_funcstart_patterns {
        if let Some(code_space) = arch.manage().get_default_code_space() {
            let raw = crate::aif::raw_thumb_prologue_seeds(
                &file,
                &listing,
                translate,
                std::rc::Rc::clone(code_space),
                listing.exec_ranges(),
            );
            if !raw.is_empty() {
                let before = seeds.len();
                seeds.extend(raw);
                seeds.sort_unstable();
                seeds.dedup();
                // Only re-walk when the scan genuinely added new seeds.
                if seeds.len() != before {
                    listing = crate::listing::Listing::build_with_meta(
                        &file,
                        image,
                        arch,
                        translate,
                        &seeds,
                        &funcsym_seeds,
                        &seed_names,
                    );
                }
            }
        }
    }
    // (kuna, Stage-3 ARM discovery) Code-pointer-table gap seeding — the
    // reference-based (vtable / callback / Cortex-M ISR-handler) mirror of Ghidra's
    // `ArmThumbFunctionTableScript` / angr's `_seg_list` pointer scan. After Stages
    // 1-2 have covered the statically-reachable + canonical-LR-prologue population,
    // scan every allocated section for 4-byte code pointers (Thumb bit set) into an
    // UNDEFINED gap whose target BOTH begins with a stack-frame prologue AND passes
    // `check_valid_subroutine`, and RE-SEED the walk with the survivors so it expands
    // each into a full function + discovers its callees. The corroborating pointer
    // reference is what safely admits a non-LR-push prologue (`SUB SP` / `PUSH {..}`
    // no-LR / `VPUSH`) that the LR-only Stage-2 matcher structurally skips; the triple
    // guard (gap-only + prologue + valid-subroutine) keeps precision at ~100% and never
    // chases the angr-over-discovery trap (crazyflie's misread descriptor table).
    // Gated by the same `funcstart_patterns` flag (ARM-only inside
    // `code_pointer_table_seeds`), so x86-64 (funcstart_patterns off) is byte-identical
    // and every non-ARM binary is a strict no-op. Measured recovery (real, ground-truth
    // functions, zero false starts): cf2 +3, usart-stdio +1, betaflight +8.
    if arch.analysis_funcstart_patterns {
        if let Some(code_space) = arch.manage().get_default_code_space() {
            let ptr = crate::aif::code_pointer_table_seeds(
                &file,
                &listing,
                translate,
                std::rc::Rc::clone(code_space),
                listing.exec_ranges(),
            );
            if !ptr.is_empty() {
                let before = seeds.len();
                seeds.extend(ptr);
                seeds.sort_unstable();
                seeds.dedup();
                if seeds.len() != before {
                    listing = crate::listing::Listing::build_with_meta(
                        &file,
                        image,
                        arch,
                        translate,
                        &seeds,
                        &funcsym_seeds,
                        &seed_names,
                    );
                }
            }
        }
    }
    // Seed the function model's no-return / call-fixup flags from the load-time
    // Known passes so the consumer skips already-modeled callees and the fixpoint
    // treats a Known-no-return callee as terminal.
    let listing = listing.with_noreturn_seeds(noreturn_seeds, callfixup_seeds);
    let ctx = AnalysisCtx { file: &file, bytes, image, arch, listing: Some(&listing) };
    let mut out: Vec<(&'static str, AnalysisOutput)> = listing_consumer_passes()
        .iter()
        .map(|pass| (pass.id(), pass.run(&ctx)))
        .collect();

    // The Aggressive Instruction Finder gap-walk (`aif`, the third Listing
    // consumer) is NOT a pure-`ctx` pass: it speculatively decodes undecoded gap
    // bytes, so it needs the live SLEIGH decoder (the upstream builds its own
    // `PseudoDisassembler`). Drive it here with the same `translate`/code-space the
    // Listing build held, keyed by its `aif` id so the deferred commit gates it via
    // `analysis_pass_enabled` exactly like the pure consumers. A no-op (empty
    // `entries`) when there is no code space.
    if let Some(code_space) = arch.manage().get_default_code_space() {
        let mut aif_out = AnalysisOutput::default();
        aif_out.entries = crate::aif::run_aif(
            &listing,
            translate,
            std::rc::Rc::clone(code_space),
            listing.exec_ranges(),
        );
        out.push(("aif", aif_out));
    }

    // (kuna, recursive-descent discovery) Promote the walk's discovered functions — the
    // CALL targets it followed from the (prologue-seeded) roots — to committed function
    // entries. This is the commit step that turns the `walk.rs` two-level worklist
    // recursive descent into actual functions (Ghidra's disassembler-driven
    // CreateFunctionCmd analog); the commit arm names them `sub_<addr>` and dedups against
    // the already-committed set. Gated by `funcdisc_recursive` → the same
    // `analysis_funcstart_patterns` flag, so x86-64 (funcstart_patterns off) is byte-identical.
    if arch.analysis_funcstart_patterns {
        let mut rd_out = AnalysisOutput::default();
        rd_out.entries = listing.functions().map(|(&vma, _)| vma).collect();
        out.push(("funcdisc_recursive", rd_out));
    }
    out
}

/// Build an [`AnalysisCtx`] and run the **deferred** scalar/operand reference-markup
/// pass ([`crate::operand_refs::OperandRefsPass`]), returning its output keyed by
/// the pass id (`"operand_refs"`).
///
/// Like the Listing tier (PR6), this pass must run at the deferred commit point
/// rather than at load: it linear-decodes the executable sections through the
/// engine `Translate`, and the program loadimage is only attached to the engine's
/// `Sleigh` *after* the load-time pass list runs (`set_loader` in
/// `bootstrap_from_object`). A load-time decode would find no bytes (every decode
/// fails). The console calls this from `commit_pending_analysis` (reached at `read
/// symbols`, after `set_loader`), gated on `arch.analysis_operand_refs`
/// (`--option operand_refs on`). `ctx.listing` is `None` — the pass does its own
/// linear decode, independent of the Listing tier (which never populates the data
/// references this pass needs; see `docs/history/listing-tier-design.md` §2.2).
///
/// A parse failure yields an empty output (additive, never fails). Bound to the
/// real-ELF path: the XML datatest path never calls this, so the parity oracles
/// are structurally untouched.
pub fn run_operand_refs(
    bytes: &[u8],
    image: &ObjectLoadImage,
    arch: &Architecture,
) -> AnalysisOutput {
    let Ok(file) = object::File::parse(bytes) else {
        return AnalysisOutput::default();
    };
    let ctx = AnalysisCtx { file: &file, bytes, image, arch, listing: None };
    crate::operand_refs::OperandRefsPass.run(&ctx)
}

/// Extract `(addr, name)` for every text/function symbol in the object — the name
/// overlay the Listing seeds its `DiscoveredFunction`s with, so a discovered
/// callee resolves to its real name (e.g. the static `die` wrapper) instead of
/// `sub_<addr>`. ASCII/UTF-8 names only (a non-UTF-8 name is skipped).
fn funcsym_names(file: &object::File) -> Vec<(u64, String)> {
    use object::read::{Object, ObjectSymbol};
    use object::SymbolKind;
    let mut out: Vec<(u64, String)> = Vec::new();
    for sym in file.symbols().chain(file.dynamic_symbols()) {
        if sym.kind() != SymbolKind::Text {
            continue;
        }
        let addr = sym.address();
        if addr == 0 {
            continue;
        }
        if let Ok(name) = sym.name() {
            if !name.is_empty() {
                out.push((addr, name.to_string()));
            }
        }
    }
    out
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
    translate: &dyn Translate,
) -> AnalysisOutput {
    let Ok(file) = object::File::parse(bytes) else {
        return AnalysisOutput::default();
    };
    // Source-language detection runs once, before pass selection, and shapes the
    // pass list (the kuna analog of `SourceLanguageAnalyzer` running early and
    // gating the language-specific analyzers).
    let compiler = crate::sourcelang::detect_compiler(&file, bytes);
    // Listing/xref tier (design §1.3): built once, before the pass loop, only
    // when `--option listing on` (default-off ⇒ `None` ⇒ no decode work, byte
    // -identical to today). Owned here so it outlives the pass loop, borrowed
    // read-only by every consumer pass via `ctx.listing`.
    let seeds = listing_seeds(&file, bytes);
    let listing = arch
        .analysis_listing
        .then(|| crate::listing::Listing::build(&file, image, arch, translate, &seeds));
    let ctx = AnalysisCtx { file: &file, bytes, image, arch, listing: listing.as_ref() };
    let format = file.format();
    run_analyses(&ctx, &passes_for(compiler, format))
}

/// Like [`run_default_analyses`], but keep each pass's output keyed by its
/// [`AnalysisPass::id`] instead of merging.
///
/// This is the per-run-gating entry: the console stashes the per-pass split at
/// load and commits only the **enabled** passes' facts at `read symbols` (after
/// the per-pass `--option <id> on|off` flags have been applied). The id is the
/// pass's `id()` string, which doubles as its settable-option name (`phases.toml`
/// + `KUNA_OPTION_NAMES`). A parse failure yields an empty list (additive, never
/// fails). Pairs preserve pass order so the commit order is deterministic.
pub fn run_default_analyses_per_pass(
    bytes: &[u8],
    image: &ObjectLoadImage,
    arch: &Architecture,
    translate: &dyn Translate,
) -> Vec<(&'static str, AnalysisOutput)> {
    let Ok(file) = object::File::parse(bytes) else {
        return Vec::new();
    };
    let compiler = crate::sourcelang::detect_compiler(&file, bytes);
    // Listing/xref tier (design §1.3): built once, before the pass loop, only
    // when `arch.analysis_listing` (i.e. `--option listing on`). Default-off ⇒
    // `.then(...)` is `None` ⇒ no decode work, `ctx.listing == None`, and the
    // real-ELF bootstrap is byte-identical to today. The `Listing` is owned here
    // (same lifetime shape as `file`), outlives the pass loop, and is borrowed
    // read-only by every consumer pass via `ctx.listing`.
    let seeds = listing_seeds(&file, bytes);
    let listing = arch
        .analysis_listing
        .then(|| crate::listing::Listing::build(&file, image, arch, translate, &seeds));
    let ctx = AnalysisCtx { file: &file, bytes, image, arch, listing: listing.as_ref() };
    let format = file.format();
    passes_for(compiler, format)
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

    /// The neutral format every existing pass-set test uses: the `rtti` pass is
    /// PE-gated and the `objc` pass is Mach-O-gated, so an ELF (neither) format
    /// gives the byte-identical pre-rtti/pre-objc set.
    const NON_PE: object::BinaryFormat = object::BinaryFormat::Elf;

    /// `passes_for(Unknown, non-PE)` MUST be exactly today's `default_passes()`
    /// contents — the no-Rust default must never silently drop a pass (the guard the
    /// sourcelang brief calls for).
    #[test]
    fn unknown_matches_default_passes() {
        assert_eq!(ids(&passes_for(Compiler::Unknown, NON_PE)), ids(&default_passes()));
        // Both must still carry the always-on analysis passes.
        let want = ["noreturn_known", "libproto"];
        for id in want {
            assert!(
                ids(&default_passes()).contains(&id),
                "default_passes must include {id}"
            );
        }
    }

    /// All NON-Go compilers select the same pass set (Rust differs only in the
    /// no-return pass's folded-in list, not the pass *ids*). Go additionally
    /// carries the `gopclntab` pass (Go-only, appended last); every other compiler
    /// is byte-identical to the Gcc base.
    #[test]
    fn non_go_compilers_have_same_pass_ids() {
        let base = ids(&passes_for(Compiler::Gcc, NON_PE));
        for c in [Compiler::Rustc, Compiler::Clang, Compiler::Unknown] {
            assert_eq!(ids(&passes_for(c, NON_PE)), base, "{c:?} pass ids must match the base set");
        }
    }

    /// The Go pass set is exactly the base set plus the `gopclntab` pass appended
    /// (the Go-only function-name recovery); no other compiler carries it.
    #[test]
    fn go_adds_only_gopclntab_pass() {
        let base = ids(&passes_for(Compiler::Gcc, NON_PE));
        let go = ids(&passes_for(Compiler::Go, NON_PE));
        assert_eq!(&go[..base.len()], &base[..], "Go set is the base set + extras");
        assert_eq!(go.last(), Some(&"gopclntab"), "Go appends the gopclntab pass");
        assert_eq!(go.len(), base.len() + 1, "Go adds exactly one pass");
        // No non-Go compiler carries the gopclntab pass.
        for c in [Compiler::Gcc, Compiler::Rustc, Compiler::Clang, Compiler::Unknown] {
            assert!(!ids(&passes_for(c, NON_PE)).contains(&"gopclntab"), "{c:?} must not carry gopclntab");
        }
    }

    /// The MSVC RTTI pass (`rtti`) is registered ONLY on a PE image — never on a
    /// non-PE one — so a non-PE pass set is byte-identical to before the pass
    /// existed (the parity-safety contract).
    #[test]
    fn rtti_pass_is_pe_gated() {
        // PE: the rtti pass is appended (after any compiler-specific extras; the
        // `pdb` pass follows it, so rtti is no longer strictly last).
        let pe = ids(&passes_for(Compiler::Clang, object::BinaryFormat::Pe));
        assert!(pe.contains(&"rtti"), "PE pass set must carry the rtti pass");
        // Non-PE: never carried, for every format + compiler.
        for fmt in [object::BinaryFormat::Elf, object::BinaryFormat::MachO, object::BinaryFormat::Coff] {
            for c in [Compiler::Gcc, Compiler::Clang, Compiler::Go, Compiler::Unknown] {
                assert!(
                    !ids(&passes_for(c, fmt)).contains(&"rtti"),
                    "{c:?}/{fmt:?} must not carry rtti"
                );
            }
        }
    }

    /// The PE PDB pass (`pdb`) is registered ONLY on a PE image — never on a non-PE
    /// one — so a non-PE pass set is byte-identical to before the pass existed (the
    /// parity-safety contract). It is appended last (after the rtti pass).
    #[test]
    fn pdb_pass_is_pe_gated() {
        // PE: the pdb pass is appended last (after the rtti pass).
        let pe = ids(&passes_for(Compiler::Clang, object::BinaryFormat::Pe));
        assert!(pe.contains(&"pdb"), "PE pass set must carry the pdb pass");
        assert_eq!(pe.last(), Some(&"pdb"), "pdb is appended last on a PE");
        // A PE carries BOTH the rtti and pdb metadata passes (rtti then pdb).
        let rtti_i = pe.iter().position(|&p| p == "rtti");
        let pdb_i = pe.iter().position(|&p| p == "pdb");
        assert!(rtti_i < pdb_i, "rtti precedes pdb on a PE");
        // Non-PE: never carried, for every format + compiler.
        for fmt in [object::BinaryFormat::Elf, object::BinaryFormat::MachO, object::BinaryFormat::Coff] {
            for c in [Compiler::Gcc, Compiler::Clang, Compiler::Go, Compiler::Unknown] {
                assert!(
                    !ids(&passes_for(c, fmt)).contains(&"pdb"),
                    "{c:?}/{fmt:?} must not carry pdb"
                );
            }
        }
    }

    /// The `objc` pass is registered ONLY for a Mach-O binary (the Mach-O-format
    /// gate, like `gopclntab`'s Go gate): a Mach-O target carries it, every other
    /// format's pass set is byte-identical to before.
    #[test]
    fn objc_pass_registered_only_for_macho() {
        let elf = ids(&passes_for(Compiler::Clang, object::BinaryFormat::Elf));
        let macho = ids(&passes_for(Compiler::Clang, object::BinaryFormat::MachO));
        assert!(!elf.contains(&"objc"), "ELF must not carry the objc pass");
        assert!(macho.contains(&"objc"), "Mach-O must carry the objc pass");
        // Mach-O adds exactly the one objc pass over the ELF set (the rtti pass is
        // PE-only, so a Mach-O target adds only objc over the ELF base).
        assert_eq!(macho.len(), elf.len() + 1, "Mach-O adds exactly the objc pass");
        assert_eq!(macho.last(), Some(&"objc"), "objc is appended last");
        // No non-Mach-O format carries it.
        for f in [object::BinaryFormat::Elf, object::BinaryFormat::Pe, object::BinaryFormat::Coff] {
            assert!(!ids(&passes_for(Compiler::Clang, f)).contains(&"objc"), "{f:?} must not carry objc");
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
