# Analysis-tier port log

A running record of porting Ghidra's Java **analyzer / loader tier** (the "Run
Analysis" layer) into the Rust [`kuna-analysis`](../decompiler/crates/kuna-analysis)
crate, feature by feature, each with a testcase. This is the *process* log (what's
done); the gap inventory is [`missing-analyses.md`](missing-analyses.md); the
**execution plan for the remaining gaps** (dependency-ordered waves + per-task
sub-agent briefs + fixtures + fan-out protocol) is
[`analysis-port-plan.md`](analysis-port-plan.md). Crate design: its `lib.rs` / `pass.rs`.

> **Why this exists.** kuna ports Ghidra's *decompiler* (the SLEIGH "deep
> decompiler"). The Ghidra application additionally runs ~140 Java analyzers that
> *prepare* a program before the decompiler sees it. A standalone port inherits
> the decompiler but not those analyzers — so library calls render as
> `sub_400510(...)` instead of `puts(...)`, string constants as raw addresses,
> dead code after `exit()`, etc. This effort closes that gap, one analyzer at a
> time, in `kuna-analysis`.

## How a pass is structured (the seam)

Each analysis is an [`AnalysisPass`](../decompiler/crates/kuna-analysis/src/pass.rs):
a focused, read-only, never-failing producer of *facts*
(`AnalysisOutput { symbols, entries, noreturn, readonly, strings, prototypes, call_fixups }`) over an
`AnalysisCtx { file: &object::File, image: &ObjectLoadImage, arch: &Architecture }`.
`kuna_analysis::passes::default_passes()` lists the enabled passes;
`run_default_analyses(bytes, image, arch)` runs them and returns the merged
output. The console commits the merged output **once**, in
[`engine.rs::commit_analysis_output`](../decompiler/crates/kuna-console/src/engine.rs),
called from `bootstrap_from_elf` after the funcsym stream is installed.

**Binding:** analyses run **only on the real-ELF bootstrap path**
(`bootstrap_from_elf`, reached when `load file` sees `\x7fELF`). The XML
`<binaryimage>` datatest path (`bootstrap_program`) never constructs an
`ObjectLoadImage` and never calls `run_default_analyses`, so the 675/675 datatest
parity oracle is structurally untouched no matter what passes are enabled. This
binding is also what keeps kuna a faithful drop-in for a Ghidra front-end (see
[Downstream compatibility](#downstream-compatibility)).

## Per-fact-kind commit API (verified)

| Fact | Engine API | Notes |
|---|---|---|
| function symbol | `find_create_scope_from_symbol_name` → `find_function` (dedup) → `add_function` + `ConsoleProgram::register_symbol` | the existing funcsym path; idempotent overlap no-op |
| data symbol / string | `types().get_type_char(get_size_of_char())?` → `get_type_array(len, ch)?` → `add_symbol_mapped` → `set_attribute(typelock)` | all `TypeFactory` methods are **fallible** (`?`) |
| no-return | `find_function_across_scopes(addr)` (by **address**) → `Database::set_function_no_return(sid, true)`, name path `query_global_function(name)` as fallback | address is stable across demangle (Increment 11); NOT the private `Architecture::set_function_no_return(name,..)` |
| readonly range | `Database::set_property_range(varnode_flags::readonly, a1, a2_open)` | now applied on the ELF path too (was XML-only); load-bearing for string rendering |
| entry point | `name_function(addr)` → `add_function` + `register_symbol` (the `map function` recipe) | commit side is solved; *discovery* is the hard part |
| call-fixup | rebuild target→fixup map from `arch.pcodeinjectlib.injection` → `pcodeinjectlib.base.get_payload_id(CALLFIXUP_TYPE, fixup)` → `query_global_function(name)?` → guard `Database::function_inject_id_for_symbol(sid) < 0` → `Database::set_function_inject_id(sid, injectid)` | the `IfcFixupApply` body driven by the analyzer; the `<0` guard is Ghidra's `getCallFixup()==null`; the inherited inject/weave path then applies the fixup at flow time |

## Is this every Ghidra analyzer? (complete inventory)

**No — and most should not be ported.** Ghidra ships **142** non-test `*Analyzer.java`
classes. Only a minority are program-analysis steps relevant to a *standalone ELF
decompiler*; the rest are per-CPU listing helpers, non-ELF format loaders, or infra. Full
accounting (every one of the 142 falls in exactly one bucket):

### Core program-analysis tier — `Features/{Base,Decompiler,GnuDemangler,FunctionID,DecompilerDependent}` (~57)

**Ported (5) + PLT:** `NoReturnFunctionAnalyzer` (base ELF list + **all** ELF
per-compiler arms of `noReturnFunctionConstraints.xml`: Rust — Increment 7, Go —
Increment 14), `GnuDemanglerAnalyzer`, `ApplyDataArchiveAnalyzer`, `StringsAnalyzer`
(re-enabled by default since the printer change — see Increment 12),
`SourceLanguageAnalyzer` (detection half — see Increment 4); plus
`ElfDefaultGotPltMarkup` (loader code, done pre-this-effort).

**🟡 Inherited — the ported engine already does this, no analyzer needed (~11):**
`DecompilerSwitchAnalyzer`, `DecompilerFunctionAnalyzer`, `DecompilerCallConventionAnalyzer`
(these literally *run the decompiler* — kuna **is** that engine), `ConstantPropagationAnalyzer`
(S3 SSA value-set), `StackVariableAnalyzer` (S5), `FunctionAnalyzer` ×2 (proto/param recovery, S4),
`SharedReturnAnalyzer`, `SharedReturnJumpAnalyzer`, `SegmentedCallingConventionAnalyzer`.

**⬜ Real gaps worth porting (deferred, in the work-list):** `DWARFAnalyzer`,
`EntryPointAnalyzer` (+ `ExternalEntryFunctionAnalyzer`, + the 5 `BytePatterns/FunctionStart*`
below), `GccExceptionAnalyzer` (.eh_frame), `FormatStringAnalyzer` (printf varargs typing),
`ArmSymbolAnalyzer` ($t/$a), `CallFixupAnalyzer`, `FormatStringAnalyzer`.

**🟡 Ported-but-disabled-by-default:** `StringsAnalyzer` (Increment 3) and
`AddressTableAnalyzer` (Increment 4) are both faithfully ported + unit-tested but
**registered commented-out** in `default_passes()` — off by default for Ghidra parity
(`AddressTableAnalyzer.setDefaultEnablement(false)`) and false-positive risk. They are
inert until a future per-run `--option <id> on` increment.

**⛔ Out of scope / infeasible-at-this-tier (documented):** `FindNoReturnFunctionsAnalyzer`
(flow heuristic — no pre-decompile listing), `AggressiveInstructionFinderAnalyzer` (+ ARM —
needs a post-disassembly Listing/FunctionManager/PseudoDisassembler + ≥20 found functions;
off-by-default upstream; subsumed by `s1-entry-disc` + `s1-eh-frame` — Increment 4), the
operand/reference markup family (`OperandReferenceAnalyzer`, `DataOperandReferenceAnalyzer`,
`ScalarOperandAnalyzer`, `ElfScalarOperandAnalyzer` — all operate on the disassembled
Listing/ReferenceManager, which does not exist at this tier; their products are subsumed by
`s1-strings`/`s2`-jumptables/`s1-entry-disc`, and the one relevant idea is blocked by the same
printer/MapGlobals shadowing as `s1-strings` — Increment 4), `CreateThunkAnalyzer` (thunk object model),
`ExternalSymbolResolverAnalyzer` (multi-program project), `CondenseFillerBytesAnalyzer`,
`EmbeddedMediaAnalyzer`, `FidAnalyzer` (Function-ID fingerprinting subsystem),
`X86FunctionPurgeAnalyzer` (Win32 stdcall), `Golang{String,Symbol}Analyzer`,
`MingwRelocationAnalyzer`; non-ELF format loaders `Coff*`, `Macho*`, `Pef*`,
`PortableExecutableAnalyzer`, `AppleSingleDoubleAnalyzer`, `CliMetadataTokenAnalyzer`, `ElfAnalyzer`
(the loader wrapper); plus abstract bases (`AbstractAnalyzer`, `Analyzer`, `Abstract*`,
`HeadlessAnalyzer`, `MySwitchAnalyzer`).

### Processor-specific — `Processors/*` (31) — ⛔ out of scope (per-CPU listing helpers)

`X86`, `Arm`, `Mips{Address,Pre,Symbol}`, `Sparc{,EarlyAddress}`, `PowerPCAddress`, `RISCVAddress`,
`SH4{Address,EarlyAddress}`, `Pic{12,16,17c7xx,18,24DInit}`, `PicSwitch`,
`Hexagon{,PrologEpilog,Thunk,UnsupportSemantic}`, `Java`/`Jvm`/`AbstractJava`, `eBPFSyscall`,
`Motorola68K`, `NDS32`, `Loongson`, `HCS12Convention`, `AARCH64PltThunk`, `Toy`. These are per-arch
address/switch/reference heuristics at the **listing** level; kuna's SLEIGH engine handles
decoding, and PLT/thunk naming is done generically.

### Other `Features/*` — format/OS/lang specific (~52) — mostly ⛔

`FileFormats/*` (Android/iOS/Mach-O/DEX/ext4/DTB/… ~38), `MicrosoftCodeAnalyzer/*` (PE/RTTI/TEB/SEH),
`MicrosoftDemangler`, `Objective-C/*`, `PDB/*`, `Swift/*` — all non-ELF. **Partially relevant:**
`BytePatterns/FunctionStart*` (5 — byte-pattern function-start discovery, folds into the deferred
`s1-entry-disc`); `Rust/RustDemanglerAnalyzer` (**already covered** — kuna's demangle pass handles
Rust via `rustc-demangle`); `Rust/RustStringAnalyzer` (🟡 detection ported in Increment 4; the
str-slice split is infeasible-at-tier — see the work-list `ruststring` row).

### Framework/Build (2) — ⛔ `JitDataFlowBlockAnalyzer` (JIT infra), `SkeletonAnalyzer` (template).

**Summary:** of 142 analyzers, **~8 ported** (PLT, no-return, demangle, libproto, DWARF,
entry-disc/eh-frame, source-language + Rust no-return; plus `StringsAnalyzer`/`AddressTableAnalyzer`
ported-but-disabled-by-default) + ~11 inherited by the engine + the few remaining real
ELF-relevant gaps deferred to Wave 2/3 (format-string, ARM/MIPS markers, callfixup, the DWARF
stack-locals subtask, the printer change to re-enable strings) + ~112 out of scope for a
standalone ELF decompiler (per-CPU helpers, non-ELF formats, Windows/Go/Swift/ObjC, infra —
including AIF and the operand/reference markup family, both ⛔ infeasible-at-tier, Increment 4).
The work-list below tracks the relevant set; this section is the evidence it's exhaustive.

## Work-list

Status: ✅ done · ⬜ gap (to port) · 🟡 inherited (engine already does it) ·
⛔ out-of-scope / infeasible-at-this-tier.

| St | Pass id | Analysis (Ghidra) | Diff | Testcase |
|----|---------|-------------------|------|----------|
| ✅ | `plt-got` | PLT/GOT import names (`ElfDefaultGotPltMarkup`) | — | x86: fauxware `0x400510→puts`; **AArch64 e2e (Increment 19)**: linked `plt_aarch64`, `main` `bl 0x4004d0`/`0x4004e0` render `puts("hello")`/`printf("%d\n",…)` (the `adrp x16;ldr x17` veneer decode); RISC-V (Increment 20); ARM (Increment 18); **SPARC e2e (Increment 24)**: linked `plt_sparc64`, `main` calls `0x2021c0`/`0x2021a0` render `puts("hello")`/`printf("%d\n",…)` (the `sethi %g1` veneer; SPARC's JMP_SLOT `r_offset` IS the stub addr); **PPC64 ELFv2 e2e (Increment 26)**: linked `plt_ppc64le`, `main` `bl 0x680`/`0x660` render `puts(...)`/`printf(...)` (the `.text`-synthesized `addis r12,r2,@ha; ld r12,@l` TOC-relative stub decode); **MIPS o32 e2e (Increment 27)**: linked `plt_mips32`, `main`'s `$gp`-relative GOT calls render `puts(...)`/`printf(...)` (no `.plt`/`R_MIPS_JUMP_SLOT`; the dynamic-symbol GOT layout `DT_MIPS_LOCAL_GOTNO`/`DT_MIPS_GOTSYM`/`DT_PLTGOT` names the `.MIPS.stubs` stub) |
| ✅ | `symtab-dynsym` | `.symtab`/`.dynsym` function reader | — | `fixture_funcsyms` |
| ✅ | `foundation` | generic `AnalysisOutput` commit seam | med | bootstrap_from_elf commits with no funcsym regression |
| ✅ | `noreturn_known` | No-return known list (`NoReturnFunctionAnalyzer`) | easy | fauxware `rejected`: no dead code after `exit(1)` |
| ✅ | `s1-demangle` | Demangling (`GnuDemanglerAnalyzer` + Rust) | easy | `cpp_mangled` `main`: call renders `foo::Bar::baz(...)` |
| ✅ | `s1-libproto` | Library prototype seeding (`ApplyDataArchiveAnalyzer`) | med | fauxware `main`: `puts("Username: ")`, `puts("Password: ")`; `rejected`: `printf("Go away!")` |
| ✅ | `s1-strings` | String-literal detection (`StringsAnalyzer`) | med | ENABLED by default since the printer change — its planted `char[N]` symbol coexists with the literal: the printer renders a pointer to a readonly char-printable array symbol as the literal (Ghidra behavior). fauxware `main`: `puts("Username: ")`/`puts("Password: ")` with `s_400915` registered. See Increment 12 |
| ✅ | `s1-dwarf` | DWARF names+types (`DWARFAnalyzer`) via gimli | hard | dwarf_stripped: `add_values`/`compute`/`main` recovered (no .symtab); cet_pie: `elaborate_debug_symbol`'s param typed `char *` (subtasks 1+2; **subtask-3 stack-locals deferred**, engine change) |
| ✅ | `s1-entry-disc` | Function entry discovery (`EntryPointAnalyzer`/`FunctionStartAnalyzer`) | hard | stripped_dynamic: `sub_1405` (main) decompiles without `--addr` (Increment 5); dynamic INIT/FINI elements named `_INIT_<i>`/`_FINI_<i>`/`_DT_INIT` per `ElfProgramBuilder` (Increment 22); cross-arch `_start`→`main` idiom for AArch64/ARM/RISC-V — stripped `main` recovered + decompiled, ARM via the discovery-derived Thumb `TMode=1` paint (Increment 23) |
| ✅ | `s1-eh-frame` | `.eh_frame` FDE starts (entry oracle, `GccExceptionAnalyzer`) | hard | fauxware: FDE starts ⊆ discovered entries (7 starts incl. `_start`/`main`) (Increment 5) |
| ✅ | `sourcelang` | Source-language / compiler detection (`SourceLanguageAnalyzer`) | easy | `s1_sourcelang::detect_compiler`: `rust_hello` ⇒ `Rustc` (`.comment` + `_ZN…17h…E`), `fauxware`/`cpp_mangled` ⇒ `Gcc` (Increment 7) |
| ✅ | `s1-rust-golang-noreturn` | Rust + **Go** no-return list selection (`noReturnFunctionConstraints.xml` `rustc` + `golang` arms) | easy | `RustFunctionsThatDoNotReturn` (Increment 7) **and** `GolangFunctionsThatDoNotReturn` (Increment 14) vendored + parsed per detected compiler; `ZN4core9panicking5panic17h*` flagged for Rust-only, `runtime.gopanic`/`runtime.throw`/`runtime.goexit.abi0` for Go-only, neither for a C ELF |
| 🟡 | `ruststring` | Rust str-slice split (`RustStringAnalyzer`) | med | **detection ported** (shares `s1_sourcelang`); the **split is infeasible-at-tier** (needs post-disasm interior refs + a populated ReferenceManager — same wall as `FindNoReturnFunctionsAnalyzer`). Documented, no split code (Increment 7) |
| ✅ | `arm-mips-markers` | ARM `$t`/`$a`+STT_FUNC-LSB → `TMode` (`ARM_ElfExtension`/`ArmSymbolAnalyzer`); MIPS `$gp` | hard | **ARM done** (Increments 8/17/18): `arm_thumb_le32.o` → `TMode=1` for `$t.0`@`0x0` + STT_FUNC LSB normalized to `0x0`/`0x14`; commit-arm paints `TMode` via `set_variable`, no-ops on non-ARM (fauxware byte-identical); Thumb-FUNC re-home (Increment 17). **Decode e2e now done in-container** (Increment 18): the LINKED `arm_thumb_linked_le32` ET_EXEC (`arm-linux-gnueabihf-gcc` in `kuna-dev`) Thumb-decodes `compute` → `a0 * 3 + 7` and `_start`'s CALL to the even entry resolves to `compute(` — no wiring fix needed. **MIPS `$gp`** done (Increment 17); **MIPS16 `ISA_MODE` out of scope** (needs a MIPS16 fixture) |
| ✅ | `s1-formatstring` (A+B) | printf/scanf varargs typing (`FormatStringParser` + `FormatStringAnalyzer`) | xhard | **A done** (Increment 9) — `s1_formatstring::parse_output_types("%d %s")` ⇒ `[Int, CharPtr]`, full conversion+length-modifier tables, `*`/`%%`/positional `%n$`, malformed no-panic. **B done** (Increment 14) — the decompile→inspect→override→re-decompile loop in `IfcDecompile`: walks `CALL` ops, classifies printf/scanf callees (`apply::classify_variadic_call`), reads the format constant at the format slot, builds a per-call-site `PrototypePieces` override (`apply::build_override_pieces`), re-decompiles. **Gated OFF** (`--option formatstring on`, Ghidra `setDefaultEnablement(false)`). `fmt_x86_64`: `printf("%d %s\n",a0,(char *)*a1)` typed vs default `(uint8)a0,*a1`. **Cross-arch done** (Increment 25) — `fmt_aarch64`/`fmt_arm`/`fmt_riscv64` all type identically with one `--option formatstring on`; AArch64/RISC-V worked unchanged, ARM needed a scoped `readonlypropagate` flip (its format pointer is a read-only literal-pool LOAD), still under the `formatstring` gate (`verify_formatstring_crossarch.rs`) |
| 🟡 | `addrtable` | Absolute address-table discovery (`AddressTableAnalyzer`) | med | implemented + tested but **disabled by default** (Ghidra `setDefaultEnablement(false)` + false-positive risk); scanner finds the 8-entry table @ `0x402008` in `switchtab_x86_64`. See Increment 4 |
| ✅ | `callfixup` | Auto-apply cspec call-fixups (`CallFixupAnalyzer`, install half) | med | `mcount_x86_64`: `main`'s `-pg` `call mcount` is **dissolved** — body becomes `return 0;` + `Function: mcount replaced with injection: mcount`. Pass matches FUNC names to cspec `<callfixup><target>`; commit tags inject id (the inherited inject/weave path applies it). Flow-repair half infeasible-at-tier (LOSS). See Increment 8 |
| 🟡 | `switch-recovery` | `DecompilerSwitchAnalyzer` | — | the engine **is** this (S2 jump-tables ported) |
| 🟡 | `const-prop` | `ConstantPropagationAnalyzer` | — | engine does its own SSA const-prop (S3) |
| ⛔ | `s1-aif` | Aggressive Instruction Finder (`AggressiveInstructionFinderAnalyzer` + ARM) | xhard | needs post-disassembly Listing/FunctionManager/PseudoDisassembler + ≥20 found functions — not at this tier; off-by-default upstream; folds into `s1-entry-disc` + `s1-eh-frame`. Increment 4 |
| ⛔ | `operand-refs` | Operand/scalar reference markup (`OperandReferenceAnalyzer`, `DataOperandReferenceAnalyzer`, `ScalarOperandAnalyzer`, `ElfScalarOperandAnalyzer`) | easy | no Listing/ReferenceManager at this tier; products subsumed by `s1-strings`/`s2`-jumptables/`s1-entry-disc`; the one relevant idea (scalar→`char*`) is blocked by the same printer/MapGlobals shadowing as `s1-strings`. Increment 4 |
| ⛔ | `s1-noreturn-discovered` | `FindNoReturnFunctionsAnalyzer` (flow heuristic) | hard | needs pre-decompile listing/flow — not at this tier |
| ⛔ | `thunk-model` | thunk/external object model | hard | needs `ExternalLocation`/S2-S4 internals |
| ⛔ | `x86-purge` | `X86FunctionPurgeAnalyzer` | — | Linux ELF x86 is cdecl; engine infers `extrapop=0` |
| ✅ | `golang-symbols` | Go pclntab function-name recovery (`GolangSymbolAnalyzer`, name half) | hard | Increment 34: `s1_pclntab::GoPclntabPass` parses the embedded pclntab (go1.2/1.16/1.18/1.20 magics) and emits a `SymFact{Function}` per Go function, so `main.main`/`runtime.*`/package fns render named instead of `sub_<addr>`. Go-gated (registered only for `Compiler::Go`, like the Go no-return list); reuses the existing symbol commit arm (no new arm). Name-recovery half only — the RTTI/type/signature subsystems stay out of scope |
| ⛔ | `extern-resolver` | `ExternalSymbolResolverAnalyzer` | — | needs multi-program Ghidra project context |
| ⛔ | `filler-bytes` | `CondenseFillerBytesAnalyzer` | — | listing cosmetics; never reaches decompiler output |
| ⛔ | format-specific | Swift/ObjC/PDB/RTTI/Mach-O/PE | — | out of scope for an ELF decompiler |

**Do-first order:** `noreturn_known` (done) → `s1-demangle` → `s1-strings` →
`s1-libproto`.

## Downstream compatibility

> *The question: if we add analyzer logic into the kuna engine, does it still
> work as a drop-in replacement for Ghidra's C++ decompiler under a Ghidra
> front-end (the UI / headless driving it over the XML callback protocol)?*

**Verdict: no, it does not break the drop-in — provided passes stay bound to the
standalone real-ELF path (which they are).** Rationale, verified against the
Ghidra source:

1. **The C++ decompiler protocol is pull-based.** A Ghidra front-end drives the
   decompiler as `ArchitectureGhidra`, which overrides every info subsystem to
   *query the client*: `ScopeGhidra` forwards `findFunction`/`findExternalRef`/
   `findCodeLabel` and its `addSymbolInternal` literally throws
   `"unimplemented"` — the decompiler is structurally forbidden from inventing
   global symbols in that mode. `LoadImageGhidra` marshals byte/readonly
   requests; `GhidraStringManager` asks the client for string data. So in
   front-end mode the decompiler **receives** symbols/types/strings/noreturn and
   **produces** only the recovered function. It never discovers program-wide facts.

2. **kuna's analyzer facts enter at exactly one seam the protocol never uses.**
   `bootstrap_from_elf` is reached only on `\x7fELF` magic; the XML path
   (`bootstrap_program`, the structural analog of a front-end handing in
   everything) never calls `run_default_analyses`.

3. **Every commit API is additive to the same tables the funcsym path already
   writes** (`add_function`/`add_symbol_mapped`/`set_function_no_return`/
   `set_property_range`/`register_symbol`). No new wire element, no format change,
   no pass-order change. A downstream consumer would see a richer-but-well-formed
   symbol/type table and identical decompiler output *shape*.

**Recommendation (the segregation contract):** keep analyses (a) bound to the
`ObjectLoadImage` backend, not the engine; (b) additive, never authoritative
(front-end facts must win; analyzer facts only fill holes — the `find_function`
overlap no-op already enforces this); (c) a future `ArchitectureGhidra`-equivalent
leaf would use the wire symbol stream and skip analyzers entirely. A regression
guard asserting `run_default_analyses` is never invoked on the XML path is a
recommended follow-up (today the structural separation + `make test` PARITY is
the de-facto guard).

## Open architectural decision: per-run option gating (✅ RESOLVED — Increment "option-gating")

`docs/missing-analyses.md` promises each pass be flippable via `--option <id> on|off`
and discoverable in `kuna catalog`. There was a real ordering wrinkle (**conflict #4**):
the analysis commit happened in the `load file` body (`bootstrap_from_elf`), but `kuna
decompile` emits `option <id> <val>` *after* `load file`, so a per-run gate would be a
no-op as wired. **This is now resolved** (see the "option-gating" Increment below): the
per-pass `AnalysisOutput` is stashed on the program at load, the commit is deferred to
`IfcReadSymbols` (`read symbols`), the CLI `build_script` emits the `option` lines
**before** `read symbols`, and the commit consults each pass's per-id enable flag and
drops a disabled pass's facts. All eight pass ids are registered settable options
(`stages.toml` + `KUNA_OPTION_NAMES`), default-on (except `addrtable`, off — Ghidra
parity), so behavior is unchanged when no `--option` is passed. The pre-existing
per-name surface `--option noreturn <name>` still works as a manual override.

## Increments

### Increment 1 — foundation + no-return ✅

- **Foundation:** `bootstrap_from_elf` now reads the image bytes once, runs
  `run_default_analyses`, applies read-only markup (new on the ELF path), and
  commits the merged `AnalysisOutput` via the new `commit_analysis_output`
  (function/data/entry/no-return fact kinds all wired; only no-return has a
  producer in this increment).
- **`noreturn_known`** ported from `NoReturnFunctionAnalyzer` ("Known") + the
  verbatim `ElfFunctionsThatDoNotReturn` list (vendored at
  `kuna-analysis/data/`). Faithful matcher: strip **all** leading `_`, exact-then-
  wildcard match, namespace guard (global/`std` only). The flow-based "Discovered"
  analyzer (`FindNoReturnFunctionsAnalyzer`) is documented infeasible at this tier
  (needs a pre-decompile listing/flow model).
- **Result:** fauxware `rejected` went from a wall of dead fall-through to:
  ```c
  void rejected(void)
  {
    printf(0x40090c);
                      /* WARNING: Subroutine does not return */
    exit(1);
  }
  ```
- **Tests:** 5 unit tests (`kuna-analysis` `noreturn::tests`); `make test`
  **PARITY OK** (675/675); `make rust-test` green.

### Increment 2 — demangling + strings (fanned out, parallel) ✅

Two passes implemented concurrently in isolated worktree agents (both Opus), then
integrated sequentially with gates after each.

**`s1-demangle`** — port of `GnuDemanglerAnalyzer`. The decisive finding: Ghidra's
GNU/Itanium demangler is **not Java** — it shells out to a native `demangle`
(libiberty `cplus_demangle`). So "line-faithful port" has nothing to transcribe for
Itanium; mirroring Ghidra means *calling a 3rd-party demangler* — exactly a crate dep.
Added `cpp_demangle` 0.5.1 (Itanium) + `rustc-demangle` 0.1.27 (Rust legacy + v0) as a
documented dependency-substitution LOSS (same move as BFD→`object`). New `s1_demangle`
module: `demangle_raw` (full `foo(int)`) + `demangle_name` (name-only `foo::bar`).
The **name-only reduction is mandatory** — kuna's `::` splitter
(`find_create_scope_from_symbol_name`) splits the *entire* string, so a leftover
signature/template `::` would create junk scopes. Applied at funcsym build in
`loadimage_object.rs` (all 3 sources, after `strip_version`). 11 unit tests + a real
`g++`-built `cpp_mangled_x86_64` fixture. Scoped LOSS: name+namespace only, not
signature/param-type application (kuna recovers those from usage, S4/S5).

**`s1-strings`** — port of `StringsAnalyzer`. New `s1_strings` module: a faithful
transcription of `MinLengthCharSequenceMatcher` + `AsciiCharSetRecognizer` (printable
`0x20..=0x7e`+CR/LF/TAB, min length **5**, require-NUL-end, ASCII/1-byte only). Scans
allocated+initialized sections; emits `StringFact{addr,len}` (new `AnalysisOutput`
field). The commit arm types each as a typelocked `char[N]` data symbol. **LOSS:** the
n-gram trigram false-positive filter (`StringModel.sng`) is unportable (model not in
tree) — substituted "printable+NUL+min-5", which over-accepts; harmless for real
literals, documented. 5 unit + fixture tests.

**Honest rendering verdict (strings):** `puts("Username: ")` does **not** render yet —
the call shows `puts(s_400915)` (the constant now references the typed `char[]` global
instead of a bare `0x400915`, a real improvement). The inline literal needs the call
argument typed `char *`, which comes from a library prototype (`puts(char*)`) — i.e.
the **`s1-libproto`** pass. This exactly matches the research's conflict #8: a
typelocked `char[N]` data symbol alone does not retype the call-site constant; readonly
markup (foundation) is necessary but insufficient without the proto. So strings is a
faithful, complete `StringsAnalyzer` port; the headline literal is gated on libproto
(next increment).

**Engine fix uncovered by demangle — cross-scope call resolution.** Demangle placing
`foo::Bar::baz` into the nested `foo::Bar` scope exposed a latent limitation: the call
resolver (`decompile_drive.rs::query_call` + the no-return/inline/inject variants) only
searched the **global** scope's address map (`find_function(global, addr)`), so a
namespaced callee resolved to `sub_<addr>` (worse than the mangled name). C++
`Scope::queryFunction(Address)` spans the scope tree. Added
`Database::find_function_across_scopes` + `function_display_name_across_scopes`
(qualified name) + no-return/inline/inject cross-scope variants, and switched the four
`query_call*` resolvers to them. Now `main` renders `foo::Bar::baz(&v1,0x2a)`
(fully-qualified) and `__stack_chk_fail()` is correctly no-return. Identical for flat
global names, so the 675 datatests are unaffected.

- **Known cross-pass seam (RESOLVED in Increment 11):** the no-return pass matched on
  *raw object-symbol* names while demangle renamed the *funcsym*; a no-return C++ symbol
  (e.g. `_ZSt9terminatev`) is installed as `std::terminate`, so the no-return commit's
  `query_global_function("_ZSt9terminatev")` missed it. Harmless for the common case
  (C libc no-return imports are unmangled), but real for C++. **Fixed by approach (A):**
  the fact now carries the symbol **address** (`NoReturnFact { addr, name }`) and the
  commit resolves by address via `find_function_across_scopes` (stable across demangling),
  with the name path as a fallback. See Increment 11.

- **Tests:** `kuna-analysis` 32 tests pass (noreturn 5 + strings + demangle 11 +
  fixtures); `make test` **PARITY OK** (675/675, including after the engine
  call-resolution change); `make rust-test` green.

### Increment 3 — library prototypes + the strings/printer finding ✅

**`s1-libproto`** — port of `ApplyDataArchiveAnalyzer`. Ghidra applies parsed C
header archives (`.gdt`, binary, not vendored) to give imports their signatures.
Substituted a **built-in table of ~25 common libc signatures** (`puts(char*)`,
`printf(char*,...)`, `strcmp(char*,char*)`, `malloc`, `memcpy`, …) — a faithful
minimal stand-in (documented LOSS: covers the table, not a full archive). New
`s1_protos` module: for each table entry whose name is present as a function, build
`PrototypePieces` from the arch type factory and emit it; the commit seam parks each
on its callee via `Architecture::set_function_prototype_pieces`. `ActionDefaultParams`
then copies the callee signature into the caller, typing the argument constants.

**The headline result.** With `puts` typed `int puts(char *)`, the constant `0x400915`
becomes a `char *` to readonly memory, the printer's `push_ptr_char_constant_ir` fires,
and the StringManager reads the bytes:
```c
puts("Username: ");
puts("Password: ");          // fauxware main
printf("Go away!");          // fauxware rejected
```

**Key architectural finding — how kuna renders string literals (vs Ghidra).** Ghidra's
`StringsAnalyzer` *creates a string data object* at the address, and Ghidra's decompiler
renders a pointer to it as the literal. **kuna's printer renders a constant that maps to
a named global symbol as that symbol's NAME** (`puts(s_400915)`), which *shadows* the
literal path. So planting a `char[N]` data symbol (Ghidra's mechanism, which the
`s1-strings` pass faithfully implements) actually **blocks** `puts("Username: ")` in kuna.
An A/B test confirmed it: with the strings pass on, `puts(s_400915)`; with it off (libproto
only), `puts("Username: ")`. Therefore:

- kuna renders string literals via **type-driven rendering** — a `char *`-typed constant
  (from `s1-libproto`, or from S5 usage inference) pointing at readonly memory, read by the
  already-ported `StringManager` — **not** via Ghidra's plant-a-data-symbol path.
- `s1-strings` is kept (a faithful, tested `StringsAnalyzer` port) but **disabled by
  default**, because enabling it is net-negative (it trades literals for `s_<addr>` named
  refs). Re-enabling it cleanly requires a **deferred printer change**: render a pointer to
  a readonly char-array symbol as the literal (the Ghidra behavior), so the data symbol and
  the literal coexist. Until then, `s1-libproto` + S5 usage inference cover literal rendering.

This is exactly the kind of "Ghidra mechanism that doesn't map 1:1 onto the ported engine"
the port is meant to surface: the *end result* (`puts("Username: ")`) is achieved, by a
different, more kuna-native route.

- **Tests:** `kuna-analysis` 34 tests pass; `make test` **PARITY OK** (675/675);
  `make rust-test` green.

### Increment 4 — address-table discovery (disabled) + two scope decisions ✅

Three Ghidra disassembler/analysis-tier items resolved together: one disabled-by-default
code pass and two documented ⛔ out-of-scope decisions. None can move the parity oracles
(the pass is inert by default; the other two are doc-only).

**`addrtable`** — port of `AddressTableAnalyzer` ("Create Address Tables"). New
`s1_addrtable` module: a faithful transcription of the `AddressTable.getEntry` pointer-run
scanner. `scan_address_tables(file, min_run, ptr_size)` walks the searchable
(allocated + initialized, non-executable: `.rodata`/`.data`/`.data.rel.ro`/`.got`) sections
and accepts a run of `>= min_run` consecutive pointer-width values that each (a) are `!= 0`,
(b) `>= minAddressOffset`/`MINIMUM_SAFE_ADDRESS`, (c) land inside an executable section, and
(d) pass the `isValidRelocationAddress` guard. The pass emits a **Data** symbol at each table
head + a read-only range over the body — **never** an `entries`/Function fact (faithful to the
analyzer, whose function-making code is commented out; real code targets are found by the
separate `s1-entry-disc` pass and the engine's switch recovery).

*Faithful relocation-guard activation (the one design judgement).* The guard
(`isValidRelocationAddress`) is sound only on a **relocatable / PIE** image (`ET_REL`/`ET_DYN`),
where *every* real absolute pointer is relocated. On a non-PIE `ET_EXEC` (the
`switchtab_x86_64` fixture) the binary still has a couple of dynamic relocations (GOT slots
for `__libc_start_main`/`__gmon_start__`), but those do NOT cover the program's absolute
`.rodata` switch-table pointers — so the guard is gated on `file.kind()` being
Relocatable/Dynamic. On `ET_EXEC` the guard is OFF and only the heuristic checks apply
(in-exec + min-addr + string-guard + min-run) — the over-acceptance-prone path that is exactly
why Ghidra (and kuna) ship this pass disabled.

*Result.* The unit test drives `scan_address_tables` over the vendored `switchtab_x86_64`
fixture directly (NOT via `default_passes`, which leaves the pass commented out) and confirms
it **FINDS the 8-entry absolute jump table at vma `0x402008`** (the `jmp *0x402008(,%rdi,8)`
target), with all 8 elements landing inside `.text` (pinned consts read off
`readelf -x .rodata`/`objdump -d`). A negative test confirms no spurious table over fauxware's
string data, and a threshold test confirms the 8-entry table is rejected at `min_run=64`.

*Disabled-by-default.* Registered commented-out in `passes.rs::default_passes()`, exactly like
`s1-strings`: (a) Ghidra parity (`setDefaultEnablement(false)`); (b) false-positive risk (a
pointer-run scanner over-accepts; the reloc guard is weak on non-PIE). NOTE the careful
classification this task is as much about: this is **NOT** switch recovery (that is the
INHERITED S2 engine machinery, `s2_lift/jumptable.rs`, the analog of `DecompilerSwitchAnalyzer`)
and **NOT** roadmap-#9 post-typing refinement (that is the decompiler-internal multistage
re-recovery, `recover_count > 1`, ENGINE-RESIDENT S2 behind the `Override::queryMultistageJumptable`
seam) — it is only the application-layer absolute-pointer-table discovery. #9 stays a separate,
deferred *engine* task; this increment classifies it, it does not touch it.

**`aggressive-disasm` — ⛔ doc-only decision (NOT ported).** Ghidra's
`AggressiveInstructionFinderAnalyzer` (AIF, + the ARM variant) is **infeasible at the
kuna-analysis tier and out of scope.** It is a *post-disassembly, whole-program, iterative*
speculative gap-filler that (1) requires a fully-populated Listing + FunctionManager with
**≥20 functions already found** (`AggressiveInstructionFinderAnalyzer.java:97-102`,
`MINIMUM_FUNCTION_COUNT=20`) — kuna's analysis tier runs *before* any decompilation, with no
Listing and no global function discovery; (2) builds a function-start byte-fingerprint histogram
via `SleighDebugLogger`/`getInstructionMask` and accepts a gap candidate only if its masked
prefix matches a fingerprint seen ≥4 times with matching disassembly context — kuna has no
instruction-mask/SleighDebugLogger machinery; (3) speculatively disassembles undefined gaps with
`PseudoDisassembler.followSubFlows` + `checkValidSubroutine` — kuna has no recursive-descent
PseudoDisassembler at this tier. It is also **off-by-default upstream**
(`setDefaultEnablement(false)`) and its own DESCRIPTION warns "IT MAY CREATE A LOT OF BAD CODE!"
For a decompiler that is *given* function entries, AIF's only useful output (new entries) is
delivered more soundly by `s1-entry-disc` (`EntryPointAnalyzer` + `FunctionStart*` byte patterns)
and `s1-eh-frame` (FDE starts); AIF's distinctive contribution — guessing code in gaps with no
symbol/FDE/xref — is precisely the high-false-positive case Ghidra disables by default. A naive
"port" (linear gap disassembly) would silently drop AIF's entire correctness model and is exactly
the unfaithful move to avoid. **No `AnalysisPass` impl, no `default_passes()` entry**; a Ghidra-origin
"deliberately absent" note sits beside the disabled-pass block in `passes.rs`. Mirrors how
`FindNoReturnFunctionsAnalyzer` was handled (`s1_loader/noreturn.rs`).

**`operand-refs` — ⛔ doc-only decision (NOT ported as a producing pass).** The
operand/scalar reference markup family (`OperandReferenceAnalyzer`, `DataOperandReferenceAnalyzer`,
`ScalarOperandAnalyzer`, `ElfScalarOperandAnalyzer`) is **out of scope at this tier.** All four
operate on the disassembled *Listing* + `ReferenceManager`, neither of which exists in
`kuna-analysis` (its `AnalysisCtx` is object-file + loadimage bytes only). And references are a
UI/listing artifact: kuna's decompiler consumes loadimage bytes + the symbol/type tables, **not**
the ReferenceManager (verified: no `AnalysisOutput` fact kind or commit-seam API carries a
"reference"; the kinds are symbols/entries/noreturn/readonly/strings/prototypes only). Per-analyzer:
(1) `OperandReferenceAnalyzer`'s four products are all already covered or unsurfaceable — ASCII/Unicode
strings == `s1-strings` (disabled), address/switch tables == the inherited S2 jump-table engine,
subroutine disassembly + function creation == the deferred `s1-entry-disc`, pointer creation needs a
disassembled-listing target that does not exist here AND would not surface (ActionMapGlobals is a stub,
`s9_emit/coreaction_render.rs`); (2) `DataOperandReferenceAnalyzer` == (1) minus function creation;
(3) `ScalarOperandAnalyzer` produces only listing *references* from scalar operands and is **default-OFF
for ELF** in Ghidra (`getDefaultEnablement` returns false for ELF), so porting it on the ELF path would
not even be faithful; (4) `ElfScalarOperandAnalyzer` is a *corrective* subclass that *removes* bad
`.got`/`.plt` scalar references — it only matters as a fixup of (3), and kuna already gets correct
`.plt`/`.got` naming from `elf_plt.rs`, so there is nothing to correct. The one idea with any decompiler
relevance — typing a scalar that actually points at a `.rodata` string as `char*` — is already delivered
by `s1-libproto` + S5 usage inference, and a standalone "plant a typed pointer at the constant" mechanism
is **blocked by the same printer/MapGlobals shadowing** that disabled `s1-strings` (Increment 3): kuna's
printer renders a constant mapping to a named global as that symbol's NAME, shadowing the literal. **No
producing pass added** (an optional default-OFF micro-pass was considered and declined — it is provably
net-zero/net-negative for the same reason `s1-strings` is disabled).

- **Tests:** `kuna-analysis` 41 tests pass (7 new in `s1_addrtable::tests`, incl. the
  switchtab `0x402008` headline + the fauxware negative + the `min_run` threshold guard);
  `make test` **675/675 PARITY OK**; `make test-stages` **158/158 PARITY OK**; `make rust-test`
  green. The addrtable pass is inert by default, so it cannot perturb any gate; the other two
  are doc-only.

### Increment 5 — function entry discovery + `.eh_frame` FDE oracle ✅

**`s1-entry-disc` + `s1-eh-frame`** — ported as ONE fused, additive
`EntryDiscoveryPass` (new `s1_entry` module, registered after `LibProtoPass`,
always-on like noreturn/libproto). It fills only `AnalysisOutput.entries`; the
existing commit seam (`engine.rs::commit_analysis_output` step 2: `name_function`
+ `add_function` + `register_symbol`, idempotent via `find_function`) turns each
discovered VMA into a `sub_<addr>` function — **no engine change**. The pure core
`collect_entries(&object::File) -> Vec<u64>` unions five oracles, validates each
VMA is inside an executable section, drops any already covered by a funcsym
(`.symtab`/`.dynsym` defined FUNC + PLT stubs), and dedups.

**The five oracles** (faithful subsets of Ghidra's Listing-coupled analyzers):
1. ELF `e_entry` — `EntryPointAnalyzer` external entry.
2. `DT_INIT`/`DT_FINI` + `DT_INIT_ARRAY`/`DT_FINI_ARRAY` pointer tables (read
   from the `.dynamic` section as `Elf64_Dyn` pairs; array bytes sliced from the
   containing section) — the loader-seeded external entry points.
3. **`.eh_frame` FDE `pcBegin`** (`scan_eh_frame_starts`) — a hand-port of the
   CIE/FDE walk + DW_EH_PE decoder (`GccExceptionAnalyzer` +
   `ehFrame/{Cie,FrameDescriptionEntry}` + `DwarfDecoderFactory`), scoped to
   FDE-start extraction (NOT CFI/LSDA). The CIE augmentation `z`/`R`/`L`/`P`/`S`
   walk extracts the FDE pointer-encoding byte; the FDE→CIE back-pointer is
   `(o+4) - cieId`. No `gimli`; ~150 lines, faithful per the brief.
4. x86-64 `_start`→`main` libc-start idiom: the `lea rdi,[rip+disp]` before
   `call *__libc_start_main@GOT` carries `main` (the disassembly-free stand-in
   for the general call-target sweep, infeasible at this tier).
5. x86-64 gcc prologue byte patterns (`FunctionStartAnalyzer` port:
   `DittedBitSequence` matcher + a minimal bare-`<funcstart/>` set).

**Headline result.** On `stripped_dynamic_x86_64` (PIE, `.symtab` stripped) the
pass discovers `main` at 0x1405 (via BOTH the libc-start idiom AND an FDE start)
even though no symbol names it. The e2e gate (`verify_s1_entry.rs`) decompiles it
by name with NO `--addr`:
```c
unsigned long sub_1405(int4 a0,void *a1) {
  if (a0 <= 1) { fprintf(dat_4020,"Usage: %s <binary>\n",*a1); ... }
  else { v1 = sub_1357(a1[1]); }  // sub_1357 is another discovered entry
  return v1;
}
```
On `fauxware` (`.eh_frame` present) the FDE oracle yields all seven known starts
(0x400500 `_start`, 0x400664 `authenticate`, 0x4006ed `accepted`, 0x4006fd
`rejected`, 0x40071d `main`, 0x4007e0 `register_tm_clones`, 0x400870) — the four
funcsyms are a proper subset (the oracle property), and `collect_entries` then
skips those funcsyms, emitting only the genuinely-new starts.

**LOSS / scope** (documented in the module): general undirected call-target sweep
is infeasible at the analyzer tier (no Listing) — substituted by oracle 4 + 5;
the `after="defined"`/`validcode` pattern post-rules are dropped (no
PseudoDisassembler); oracles 4–5 are x86-64-only in v1 (other arches no-op;
1–3 are arch-independent); 64-bit-length and `indirect`/forward-referencing-CIE
eh_frame records are skipped (never in the fixtures); static-image base-0 PIE
assumption for array-pointer / absptr decode (kuna never rebases).

**Function-count note:** the pass DOES add functions on a symboled binary (the
new FDE/idiom starts with no symbol, e.g. `register_tm_clones`). No rust-test
asserts a post-bootstrap total function COUNT, so nothing needed updating —
`verify_w11_elf_loader` (synthetic single-`add` ELF: no entry/eh_frame/dynamic →
discovery no-op) and `verify_w11_elf_plt_names` (asserts named imports, not a
count) are unaffected.

- **Tests:** `kuna-analysis` 43 tests pass (8 new `s1_entry` unit tests over the
  fixtures); new `verify_s1_entry.rs` e2e gate green; `make test` **PARITY OK**
  (675/675); `make test-stages` **PARITY OK** (158/158); `make rust-test` green.

### Increment 6 — DWARF names + typed signatures (`DWARFAnalyzer`) via gimli ✅

**`s1-dwarf`** — port of `DWARFAnalyzer` (driver `DWARFAnalyzer.added()` →
`DWARFFunctionImporter.importFunctions` + `DWARFFunction.read` + `DWARFDataTypeImporter.getDataType`).
When a binary carries `.debug_*` sections the compiler has already recorded the
source function names, parameter names, and *types*; this pass reads them and
installs them onto kuna's symbol + type tables. New `s1_dwarf` module
(`crates/kuna-analysis/src/s1_dwarf/mod.rs`), registered AFTER `LibProtoPass` in
`passes.rs::default_passes()` (so a DWARF prototype wins over a libproto guess for
the same name). Pure additive S1 pass — read-only over `ctx.file` + `ctx.arch`,
skips cleanly (empty output, never fails) on a non-DWARF binary.

**Dependency-substitution LOSS.** Ghidra hand-rolls a complete DWARF reader in
`ghidra.app.util.bin.format.dwarf.*` (DWARFProgram, DebugInfoEntry, DIEAggregate,
DWARFAbbreviation, StringTable). kuna substitutes **`gimli`** (the de-facto Rust
DWARF reader, MIT/Apache) for that parser *wholesale* — the same move as BFD →
`object`. Added `gimli = { version = "0.33", default-features=false,
features=["read","std"] }` to `[workspace.dependencies]` (+ `gimli.workspace=true`
in kuna-analysis), Cargo.lock committed for offline-gate safety. We use gimli's
high-level `attr_string`/`attr_address` accessors so the DWARF-5 `strx`/`addrx`
indirections (cet_pie is DWARF 5) resolve correctly.

**Subtask 1 (names + globals).** Each *defined* `DW_TAG_subprogram` (one with
`DW_AT_low_pc`; declaration-only DIEs — no low_pc / `DW_AT_declaration` — are
skipped so the pass never fights libproto for the same imports) emits a
`SymFact{Function}` at its entry VMA; each CU-top-level `DW_TAG_variable` with a
`DW_OP_addr` location emits a `SymFact{Data}`. Reuses the existing commit arm 1
(no engine change). Headline win on the new vendored `dwarf_stripped_x86_64`
fixture (FUNC names ONLY in DWARF — `.symtab` stripped): `add_values`@0x401136,
`compute`@0x401153, `main`@0x401198 are recovered and decompile by name instead of
`FUN_`/`sub_`.

**Subtask 2 (typed signatures — the headline).** A DIE→`Datatype` mapper (faithful
reduction of `DWARFDataTypeImporter.getDataType`'s tag switch) maps base_type
(by `DW_AT_encoding`: signed→TYPE_INT, unsigned→TYPE_UINT, signed/unsigned_char→
char, float→TYPE_FLOAT, boolean→TYPE_BOOL), pointer_type, typedef/const/volatile/
restrict (transparent pass-through), array_type, struct/union (named opaque),
enum (→underlying int) against the arch's `TypeFactory`. A **recursion cap of
depth 3** (port of `trackRecursion`) survives struct→ptr→struct type cycles. For
each defined subprogram a `PrototypePieces{name, outtype, intypes, innames,
first_var_arg_slot, output_storage:None}` is built from the return-type DIE +
`DW_TAG_formal_parameter` children (`DW_TAG_unspecified_parameters` sets the
vararg slot) and emitted into `AnalysisOutput.prototypes` — parked on the callee
via the existing commit arm 5, exactly the path `s1-libproto` proved (no engine
change). `ActionDefaultParams` then copies the DWARF callee signature into each
caller. Verified end-to-end on cet_pie: `main` calls
`elaborate_debug_symbol((char *)a1[1])` — the `char *binary` parameter type
(DWARF `pointer → const → char`) typing the call argument, and the `int` return
giving `int4 v1`, where the engine alone would emit `undefined8`/`long`. (cet_pie
is non-stripped, so the *names* come from `.symtab`; DWARF's value here is the
types.) Note: a `char *` (8-byte, register-width) applies and renders cleanly; a
declared 4-byte `int` parameter passed in an 8-byte register stays `int8` after
propagation (standard engine width behavior, not a DWARF-application failure — the
return-type lock and pointer typing both verifiably apply).

**Subtask 3 (DONE — named, typed stack locals; see Increment 14).** Originally
deferred; now resolved without a shared-engine-path change. Each defined
subprogram's direct `DW_OP_fbreg` `DW_TAG_variable`/`DW_TAG_formal_parameter`
children emit a `LocalFact{func_addr, name, type_, stack_offset}`
(`stack_offset = call_frame_cfa + fbreg`); the commit parks them by function entry
VMA and `IfcDecompile` re-seeds each into the rebuilt `Funcdata`'s `ScopeLocal` as
a `typelock|namelock` stack symbol via the existing `map addr`/`seed_mapped_symbols`
carry-across path. Full write-up in **Increment 14** below.

**Faithful losses (DOC).** Skip `DW_TAG_label`, `DW_TAG_call_site`,
inlined-subroutine, lexical-block comments, source-info/plate comments (listing
cosmetics, zero decompiler-output payoff — same scope as the strings/demangle
losses). Type recovery is name/signature-level: a struct maps to a *named opaque*
struct (enough to render `struct foo *`), not a per-field layout. **PIE/load-bias
limitation:** kuna's loader treats `DW_AT_low_pc` as the runtime VMA verbatim (true
for cet_pie: DWARF low_pc 0x1357 == `.symtab` address); a nonzero load bias would
need a base adjustment (Ghidra's `DWARFProgram.getCodeAddress`), DOC'd in the
module.

- **Tests:** new `s1_dwarf` unit tests (snapshot-parse both fixtures, assert the
  defined-subprogram SymFacts + the `char *` param chain) + the e2e
  `verify_s1_dwarf.rs` (name recovery on dwarf_stripped, typed `char *` on cet_pie);
  `kuna-analysis` 36 tests pass; `make test` **PARITY OK** (675/675);
  `make test-stages` **PARITY OK** (158/158); `make rust-test` green.

### Increment 7 — source-language detection + Rust no-return list ✅

Combined `sourcelang` + `ruststring` (they share the Rust/compiler detector).

**`s1_sourcelang`** — port of `SourceLanguageAnalyzer` + the per-language
`*SourceLanguage.existsIn` predicates. New module
[`s1_sourcelang/mod.rs`](../decompiler/crates/kuna-analysis/src/s1_sourcelang/mod.rs)
with a pure `detect_compiler(&object::File) -> Compiler` (`Gcc`/`Clang`/`Rustc`/`Go`/
`Unknown`). Detection paths, faithful to Ghidra:
- **Rust** — `.comment` `rustc version …` record (`ElfRustSourceLanguage.java:34`'s
  `^rustc version .*$` regex, reduced to an anchored NUL-record prefix test), OR a
  Rust-mangled symbol (`_R…` v0 / `_ZN…17h<hex16>E` legacy — the same gate the loader's
  demangle uses), OR a `.rodata` `RUST_SIGNATURES` byte signature
  (`RustConstants.java:29-33` = `RUST_BACKTRACE`/`RUST_MIN_STACK`/`/rustc/`,
  `RustSourceLanguage.isRust`).
- **Go** — `.go.buildinfo` / `.note.go.buildid` section present (`GoBuildInfo.java:45`,
  `NoteGoBuildId.java:30`).
- **Gcc/Clang** — `.comment` `GCC: (…)` / `clang version …` record (a documented **kuna
  convenience**, not a faithful Ghidra SourceLanguage ID — Ghidra ships no generic C one).

The detector runs once in `passes::run_default_analyses` before pass selection (the kuna
analog of `SourceLanguageAnalyzer` running early and gating language-specific analyzers).
`default_passes()` is now `passes_for(Compiler)`; a guard unit test asserts
`passes_for(Unknown)` == `default_passes()` so the no-Rust default never silently drops a
pass. **PARTIAL scope**: only the detection half is ported — Ghidra's spec-extension /
`.gdt` data-archive application (`addSpecExtensions`, `getDataArchives`) is out of scope
(no SpecExtension subsystem in kuna), same depth as the demangle pass's name-only scope.

**`s1-rust-golang-noreturn`** (the deferred partial from Increment 1) — now **done**. The
Rust no-return wildcard list (`Ghidra/Features/Base/data/RustFunctionsThatDoNotReturn`,
vendored verbatim at `data/RustFunctionsThatDoNotReturn`) is parsed **in addition** to the
base ELF list when `detect_compiler == Rustc`, mirroring
`noReturnFunctionConstraints.xml`'s `compiler name="rustc"` arm. Wired into the existing
`NoReturnKnownPass` via a `rust: bool` field (`::elf()` / `::rust()` constructors); the
existing `*`-wildcard parser already handles the Rust forms
(`ZN4core9panicking5panic17h*`, `ZN5alloc5alloc18handle_alloc_error17h*`,
`rust_begin_unwind`, …). Gating contract is unit-tested: a `ZN4core9panicking5panic17h…E`
symbol is flagged no-return when `rust=true` but NOT for a C ELF (`rust=false`). The
fauxware exit test (`run_over_fauxware_flags_exit_only`) still passes unchanged.

**`ruststring`** — `RustStringAnalyzer`. **Detection ported** (it is exactly the
`s1_sourcelang` Rust detector, shared, not duplicated). The analyzer's actual job — the
str-slice split — is **infeasible at the kuna-analysis tier** and is *not* built (no
str-splitter code). Why: Rust `&str` literals sit byte-adjacent in `.rodata` with no NUL
separators; the only thing marking each slice boundary is an interior **code reference**
(a PC-relative `lea` of `ptr+len`). `RustStringAnalyzer` splits each over-long string at
the next `ReferenceManager.getReferenceDestinationIterator` destination — a post-disassembly
artifact the analyzer tier (object view only) does not have. This is the same wall
`FindNoReturnFunctionsAnalyzer`/`EntryPointAnalyzer` hit (see `noreturn.rs:9-15`). It is a
strict superset of the deferred `s1-strings` blocker (even with boundaries, a named
`char[N]` symbol would shadow the literal in kuna's printer, per Increment 3). LOSS
recorded; a full port would be a post-decompile feedback pass (engine feedback edge), out
of scope here.

**Fixture.** Vendored `tests/fixtures/rust_hello_x86_64` — a tiny (2576-byte)
**un**stripped `#![no_std]` rustc 1.90 PIE that carries BOTH a `rustc version` `.comment`
record and a `_ZN5nostd1m12rusty_helper17h…E` mangled symbol, so one fixture exercises both
detection paths e2e at the unit level. (No `kuna-console` `.sla` e2e was added — there is
no decompiler-output headline for *detection*; the win is the no-return list, asserted at
the pass level. An e2e showing dead-code elision after a tail Rust panic is a documented
follow-up: it needs a Rust fixture whose `main` tail-calls `core::panicking::panic`, larger
than this minimal no_std fixture.)

- **Tests:** `kuna-analysis` 44 tests pass (34 + 10 new: 7 `s1_sourcelang`, 1 `noreturn`
  Rust-gating, 2 `passes` selection guards). `make test` **PARITY OK** (675/675);
  `make test-stages` **PARITY OK** (158/158); `make rust-test` green.

### Increment 8 — ARM/Thumb mapping-symbol decode-mode painting (`arch-markers`) ✅

Port of the **ARM/Thumb decode-mode (`TMode`) painting** — the first analysis-tier fact
that touches the engine's *processor context* (not its symbol/type tables). Two distinct
upstream mechanisms, both ported as additive `ContextPaint` facts:

- **ARM mapping symbols** (`$t`/`$a`/`$d`, the ARM ELF ABI). `$t`/`$t.` marks the start of a
  Thumb run, `$a`/`$a.` an ARM (A32) run. Faithful to
  `Ghidra/Processors/ARM/.../elf/extend/ARM_ElfExtension.java:166-196`
  `evaluateElfSymbol` (`$t` → `programContext.setValue(TMode, addr, addr, 1)`; `$a` →
  `TMode=0`). Matched by **name**, regardless of `st_type` (the ABI records them as
  `STT_NOTYPE` local symbols).
- **The STT_FUNC odd-address (LSB) convention**: a Thumb function symbol is recorded at
  `entry|1`. Faithful to `ARM_ElfExtension.java:197-204` (the STT_FUNC fallback) +
  `creatingFunction:117-141` + the later `ArmSymbolAnalyzer.java:48-100` — mask the LSB and
  set `TMode=1` at the (even) entry. Emits `ContextPaint{addr: value & !1, "TMode", 1}`.

**New module** [`s1_loader/arm_markers.rs`](../decompiler/crates/kuna-analysis/src/s1_loader/arm_markers.rs)
(sibling of `noreturn.rs`), `AnalysisPass` id `arm_markers`, stage S1, registered always-on
in `passes_for` after `EntryDiscoveryPass`. **Gated on ARM** (`object::Architecture::Arm`),
the faithful analog of `ArmSymbolAnalyzer.canAnalyze:172-177` (processor==ARM &&
getRegister("TMode")!=null): on every other language the pass returns an empty output.

**New fact kind + commit arm.** `pass.rs` gains `struct ContextPaint { addr, end, var, value }`
+ `pub context_paints: Vec<ContextPaint>` on `AnalysisOutput` (+ `merge`). `engine.rs`
`commit_analysis_output` gains **step 6**: for each paint, resolve the code-space `Address`
and call `prog.arch().with_context_db_mut(|db| db.set_variable(var, &addr, value))` (or
`set_variable_region` when `end` is `Some`) — the exact analog of Ghidra's
`programContext.setValue(TMode, …)` (the same API the console `set context` command drives,
`ifacedecomp.rs:1933-1948`). `TMode` is registered by the ARM `.pspec` (`ARMt.pspec`
`<context_data><set name="TMode" val="0">`, applied by `parse_processor_config`) so painting
1 = Thumb. **Timing is correct**: the commit runs inside `bootstrap_from_elf` before any
`load function` decode (kuna decodes on demand).

**CRITICAL gate-safety** (the regression risk the plan flags). `set_variable` returns `Err`
when `TMode` is not registered (every non-ARM language). The commit arm **swallows the Err**
(`let _ = …`) — a faithful no-op mirroring `canAnalyze == false`. This is belt-and-suspenders
on top of the pass-level ARM gate (so `out.context_paints` is already empty for a non-ARM
binary). Verified non-regressing: `make test` **675/675 PARITY OK**, `make test-stages`
**158/158 PARITY OK**, `make rust-test` green — all untouched (the paint only fires on the
real-ELF ARM path; the XML datatest gates never run passes), and a non-ARM (x86-64)
`fauxware main` `decomp_dbg` decompile is **byte-identical** pre/post change (verified by
diffing against a merge-base build).

**Fixture: `.o`-unit-only (the e2e is a documented follow-up).** Built
`arm_thumb_le32.o` (904 bytes) with `clang --target=arm-linux-gnueabihf -mthumb -nostdlib
-c` (one `__attribute__((target("thumb")))` `thumb_add` + a `_start` caller; source vendored
at `tests/fixtures/arm_thumb_le32.c`). Its `.symtab` carries the `$t.0` mapping symbol at
`0x0` and STT_FUNC syms `thumb_add@0x1`, `_start@0x15` (LSB set). **No linked ARM exe** — this
host has NO ARM linker (no lld; gold/mold are x86-only builds; system `ld` rejects
`armelf_linux_eabi`), so the decode **e2e** (`kuna decompile arm_thumb… main` producing valid
Thumb-decoded C) is a documented follow-up requiring an off-host LINKED ET_EXEC/ET_DYN with
PT_LOAD (`ObjectLoadImage` reads only segments). The **symbol-scan unit test** is the
merge-blocking gate and passes: the bare `.o` (ET_REL) parses fine and the pass emits
`TMode=1` for `$t.0` (at `0x0`) and for each LSB-set FUNC normalized to even (`0x0`, `0x14`).
A non-ARM object (fauxware) emits zero paints (`non_arm_object_emits_nothing`).

**LOSS / divergences:**
- **`$d` data runs are a no-op.** Ghidra's `$d` branch lays *undefined data* (listing markup
  protecting bytes from disassembly); kuna has no `createUndefinedData` at this tier and the
  decompiler payoff is negligible. No fact emitted for `$d`/`$b`. Documented LOSS.
- **Thumb-FUNC address shift is paint-only (v1).** Ghidra *moves* a Thumb `FUNC` from
  `entry|1` to `entry` (`moveFunction`); kuna's `read_loader_symbols` installs it at the odd
  address. v1 emits only the `TMode=1` paint at the normalized even address (re-homing the
  function symbol risks double-symbols) — the function-address normalization is a follow-up.
- **MIPS `$gp` / MIPS16 `ISA_MODE` out of scope.** MIPS `$gp` is not a decode-mode paint at
  all — it is a *tracked register value* set per function entry
  (`MipsAddressAnalyzer.flowConstants`, sourced from `_mips_gp_value`), needing
  `create_set`/`get_tracked_default` at per-function granularity + a MIPS fixture (separate
  task). MIPS16 `ISA_MODE` is the exact `$t`/STT_FUNC-LSB analog
  (`MIPS_ElfExtension.applyIsaMode`) and could be added here with the identical mechanism
  (var `"ISA_MODE"`) given a MIPS16 fixture. Documented seam in `arm_markers.rs`.

- **Tests:** `kuna-analysis` 65 tests pass (3 new `arm_markers`: name classification, the
  Thumb-fact scan over the `.o`, the non-ARM-emits-nothing gate); `kuna-console` green.
  `make test` **PARITY OK** (675/675); `make test-stages` **PARITY OK** (158/158);
  `make rust-test` green.
### Increment 9 — format-string specifier parser (`FormatStringAnalyzer`, parser half A) ✅

`s1-formatstring` is split into **A (this increment — the pure parser)** and **B (the
deferred decompile-loop wiring)**, because `FormatStringAnalyzer` is genuinely
**DecompilerDependent**: typing a `printf`/`scanf` call's varargs needs the per-call-site
format-string *constant* + which arg is the format, neither of which exists at the load-time
`AnalysisCtx`/`AnalysisOutput` seam — they only materialize after the caller is lifted to
p-code. See [`analysis-port-plan.md`](analysis-port-plan.md) §`s1-formatstring`.

**A (done) — `s1_formatstring`** — a line-faithful port of the *pure logic* of
`Ghidra/Features/DecompilerDependent/.../variadic/FormatStringParser.java` (and the
`(lengthModifier, conversionSpecifier)` pair `FormatArgument.java`). New module
[`s1_formatstring/mod.rs`](../decompiler/crates/kuna-analysis/src/s1_formatstring/mod.rs),
registered only as `pub mod s1_formatstring;` in `lib.rs` (it emits **no**
`AnalysisOutput` facts and is intentionally **not** in `passes.rs` — it is a library the
future B calls). Faithful transcription of `parseFormatString` → `convertToFormatArguments`
(the length-modifier + conversion-specifier state machine, incl. `preprocessChar`,
`handleOutput/InputConversionArgument`, `skipFlags`/`skipIntegers`) →
`convertToOutputDataTypes`/`convertToInputDataTypes` →
`convertPairToDataType`/`conversionSpecifierToDataType` + the length-modification handlers
(`short/char/long/longLong/intmax_t/size_t/ptrdiff_t/longDouble`), plus the positional
`%n$` path `analyzeFormatStringWithParameters` (Ghidra's `Map<Integer,FormatArgument>`
replaced by a sparse `Vec<Option<FmtArg>>` — no `HashMap` — reproducing `convertMapToList`'s
1..=size / null-on-gap contract exactly).

Public API:
- `parse_output_types(&str) -> Vec<Spec>` (`printf`-family; `*` width/precision → an `Int`),
- `parse_input_types(&str) -> Vec<Spec>` (`scanf`-family; non-pointer values wrapped in an
  extra pointer, `*` suppresses the following arg — `convertToInputDataTypes`, `:597`),
- `parse_format_specifiers(&str)` (the brief's named alias = `parse_output_types`),
- `convert_to_format_argument_list(&str, is_output_type)` (the `FmtArg` intermediate),
- `spec_to_datatype(Spec, &dyn TypeFactory, word_size)` (the kuna `Datatype` builder, the
  analog of `s1_protos::build_ty`).

`Spec` enum = the result space of `convertPairToDataType` (Int/UInt/Short/UShort/Char/UChar/
Long/ULong/LongLong/ULongLong/Double/LongDouble/CharPtr/WideCharPtr/VoidPtr/IntPtr + the
widened `*Ptr` cousins for `%n` + the typedef-backed IntMaxT/UIntMaxT/SizeT/PtrDiffT(+Ptr)).
On any of Ghidra's "undefined behavior" branches (truncated/malformed specs, positional
gaps, double `*`) the parser returns an **empty** `Vec` (Ghidra returns `null` → no
override) — it never panics. Indexing is over a `Vec<char>` so multi-byte literal text never
corrupts the ASCII index arithmetic.

Faithfulness note found while porting: `convertPairToDataType`'s guard (`:622`) returns via
`conversionSpecifierToDataType` *before* the length-modifier switch for `s`/`c`/`S`/`C`, so
`%ls`/`%lc` map to `char *` / `unsigned char` (the wide-char branch in
`longLengthModification` is dead for the printf path) — the unit test pins this faithful
behavior, not the intuitive one. **LOSS:** kuna has no `.gdt` typedef archive, so the
extended-precision typedefs are lowered to their integer fallback per
`getIntegralPointerType`/the fabricated-typedef base type (documented stand-in matching the
`s1_protos` libc table); the scanf `double*`/`void**` second-level pointers are modeled as
`void*`.

**B (deferred — wave-3 engine change, NOT in this increment).** The
decompile→inspect→override→re-decompile loop (the analog of Ghidra's `ParallelDecompiler` +
`PcodeFunctionParser` + `HighFunctionDBUtil.writeOverride`): after the first decompile of a
caller, walk the `CALL` ops, read the format-string constant from the call arg at the
callee's fixed-param slot (`PcodeFunctionParser.java:99`), parse it with **this** module,
build a per-call-site `PrototypePieces` (callee fixed params ++ parsed varargs,
`first_var_arg_slot = -1`) and install it via the existing `pending_proto_overrides` →
`Override::insert_proto_override` plumbing, then re-decompile. That is an engine-driver
change, gate it OFF by default (matching `FormatStringAnalyzer.setDefaultEnablement(false)`),
and it needs A (this) merged first. **This increment touched ZERO engine/console/passes
surface** — `s1_formatstring/mod.rs` + one `pub mod` line + this log only.

- **Tests:** `kuna-analysis` 80 tests pass (44 + 36 new `s1_formatstring`: the brief's spec
  examples `%d %s`→[Int,CharPtr], `%5.2f`→[Double], `%ld%%%p`→[Long,VoidPtr],
  `%*d`→[Int,Int], `%2$s %1$d` positional; the full conversion + length-modifier tables; `*`
  width/precision; `%%` literal; scanf wrapping/`*`-suppression; and a malformed/truncated/
  Unicode no-panic sweep). `make test` **PARITY OK** (675/675); `make test-stages` **PARITY
  OK** (158/158); `make rust-test` green. (A is pure-additive, so the three parity gates are
  structurally untouched — formalities, all green.)
### Increment 10 — auto-apply cspec call-fixups (`CallFixupAnalyzer`) ✅

Port of Ghidra's default-on `CallFixupAnalyzer` (the **install half** only): tag every
program function whose name matches a cspec call-fixup `<target>` so the engine replaces the
CALL with the fixup body. The headline case is the gcc `-pg` profiling stub — `<callfixup
name="mcount"><target name="mcount"/>` (body `temp:1 = 0;`), the `mcount`/`__fentry__`
prologue call that should dissolve out of the decompiled output.

**Inherited vs the gap (the whole point).** kuna already had the entire *apply* path: the
cspec `<callfixup><target>` decode at bootstrap (`architecture.rs::decode_call_fixups`),
per-payload `target_symbol_names` retention on `SleighPayload::Callfixup`
(`inject_sleigh.rs`), the inject/weave flow (`func_inject_id → FuncProto::set_inject_id →
inject_sub_function`, `seams.rs`/`flow.rs`), and the per-function applier primitive
`Database::set_function_inject_id` (`database.rs`) — already driven by hand in the console
`fixup apply <fixup> <function>` command (`IfcFixupApply`, `ifacedecomp.rs`). **The gap**:
nothing auto-iterated the registered fixups' `<target>` names and tagged matching
FunctionSymbols at load. That auto-installer is exactly `CallFixupAnalyzer`; this increment
ports it and wires it to the inherited applier.

**`s1_callfixup`** — new module
[`s1_callfixup/mod.rs`](../decompiler/crates/kuna-analysis/src/s1_callfixup/mod.rs). A pure
`CallFixupPass` (`stage()=S1`, `id()="callfixup"`):
- `target_fixup_map(arch)` — the port of `getTargetFixupMap` (`CallFixupAnalyzer.java:436-459`):
  iterate `arch.pcodeinjectlib.injection`, for each `SleighPayload::Callfixup` read its
  `payload.core.name` (the fixup name) and `target_symbol_names`, build a `Vec<(target,
  fixup)>` (no `HashMap`, per convention).
- `call_fixup_name_for_function(name, map)` — the port of `getCallFixupNameForFunction`
  (`CallFixupAnalyzer.java:200-222`): strip a leading `libID_conflict_`, then probe the map
  with `name`, `"_"+name`, `"__"+name` (the underscore retries), first hit wins.
- The scan walks the same `.symtab`+`.dynsym` FUNC streams as `noreturn` (version suffixes
  stripped) and emits a `CallFixupFact { func_name }` per match (the *installed* name, so the
  commit's `query_global_function` resolves the real symbol — same contract as `noreturn`).

**Fact + commit.** New `AnalysisOutput::call_fixups: Vec<CallFixupFact>` (+ `merge`),
`CallFixupFact { func_name: String }` in `pass.rs`. Commit step 6 in
`engine.rs::commit_analysis_output` mirrors `IfcFixupApply` driven by the analyzer: rebuild
the target map, re-derive the fixup name, `get_payload_id(CALLFIXUP_TYPE, fixup)` → inject
id, `query_global_function(name)` → sid, **guard** `function_inject_id_for_symbol(sid) < 0`
(Ghidra's `getCallFixup()==null`, `CallFixupAnalyzer.java:89` — never clobber a hand-applied
fixup), then `set_function_inject_id(sid, injectid)`. A name with no registered fixup or no
matching FunctionSymbol or an already-set fixup is a silent no-op. One tiny additive engine
accessor added: `Database::function_inject_id_for_symbol(sid)` (the by-id companion of the
existing setter, for the guard).

**Registration.** `Box::new(crate::s1_callfixup::CallFixupPass)` in `passes_for`, always-on
(Ghidra `setDefaultEnablement(true)`, `CallFixupAnalyzer.java:51`).

**Fixture.** Vendored `tests/fixtures/mcount_x86_64` — a static, non-PIE `gcc -pg -O0`
binary (`.debug_*` stripped, `.symtab` kept) whose `main` (0x401795) prologue emits a direct
`call mcount` to the weak `mcount` FUNC (0x44a710); also carries `__fentry__` (0x44a770).
**Static on purpose**: a *dynamic* `-pg` build resolves `mcount` to an *indirect* GOT call
(`call *…(%rip)`) with no named-`mcount` symbol at the target, so the name-matched fixup
can't bind — only the static build emits a direct `call mcount` to a real symbol. Static
glibc makes it ~896 KB (larger than the other fixtures; unavoidable for a self-contained
direct-`call mcount` target).

**e2e proof.** `verify_s1_callfixup.rs`: bootstrap the fixture (runs the pass + commit), then
`load function main` → `decompile` → `print C`. Before the fixup `main` shows `mcount();`;
after (the default) the body is `return 0;` with the engine marker `Function: mcount replaced
with injection: mcount`. The test asserts the marker is present AND no `mcount();` call line
survives. This is the one place the "inject machinery is inherited" claim is proven all the
way to a dissolved call.

- **LOSS / scope.** Only the *install* half is ported. The flow-repair half
  (`setNoFallThru`/`clearAndRepairFlows`/non-returning fallthrough clearing,
  `CallFixupAnalyzer.java:93-175,234-434`) and thunk-following (`addInThunkedFunctionsToList`,
  177-198) are **post-disassembly listing/FlowOverride** work with no pre-decompile Listing at
  this tier — kuna handles fixup fallthrough inside the engine's flow path. Same constraint as
  `FindNoReturnFunctionsAnalyzer` (`noreturn.rs:9-15`). Documented in the module doc comment.
- **Divergence (no LOSS to parity).** The analysis-tier passes are always-on and **not**
  individually flippable via `--option` — they have an `id()` for the pass interface, but no
  pass id (`noreturn_known`/`libproto`/`callfixup`/…) is in
  `kuna_decomp::options::KUNA_OPTION_NAMES` (those are all S2+ decompiler sub-stage
  assertions). So `--option callfixup off` is silently ignored, like `--option
  noreturn_known off` would be — consistent with every Wave-1 pass. Per-run pass gating is the
  separately-tracked deferred decision (open architectural decision §, conflict #4).

- **Tests:** `kuna-analysis` 49 tests pass (44 + 5 new `s1_callfixup`); `kuna-console`
  +1 e2e gate (`verify_s1_callfixup`). `make test` **PARITY OK** (675/675);
  `make test-stages` **PARITY OK** (158/158); `make rust-test` green.

### Increment 11 — resolve no-return by address (the no-return × demangle seam) ✅

Closes the **Known cross-pass seam** flagged in Increment 2: the no-return pass matched on
*raw object-symbol* names while demangle renamed the *funcsym* before install, so a mangled
C++ no-return import (e.g. `_ZSt9terminatev`, installed as `std::terminate` in scope `std`)
never attached — the commit's `query_global_function("_ZSt9terminatev")` missed the renamed
symbol.

**Approach (A) — resolve by ADDRESS (chosen).** The address is the stable key: demangling
only changes the *name*, never the install address. Approach (B) (re-demangle the name in the
commit + match across scopes) was rejected as fragile — it duplicates the demangle logic in a
second place and still relies on string equality through two spellings. Address resolution
reuses the exact cross-scope, address-keyed resolver the call resolver already uses
(`Database::find_function_across_scopes`, added in Increment 2), so it is the minimal,
faithful fix.

- **`pass.rs`** — `AnalysisOutput::noreturn` changed from `Vec<String>` to
  `Vec<NoReturnFact>` where `NoReturnFact { addr: u64, name: String }`. `merge()` is unchanged
  (still `extend`). The fact carries the original (pre-demangle) name as the fallback key.
- **`s1_loader/noreturn.rs`** — `scan_noreturn` now emits the **install address** alongside
  the name. The matcher (strip leading `_`, exact + wildcard, `std`-only namespace guard) is
  unchanged. The decisive detail: a no-return *import* like `exit` / `_ZSt9terminatev` is
  **UND** in `.symtab`/`.dynsym` (`address()==0`); its real FunctionSymbol is installed at the
  **PLT stub** by `elf_plt`. So the scan now mirrors the loader's three install streams: the
  defined `.symtab`/`.dynsym` funcs (skipping `addr==0`, as the loader does) **plus**
  `elf_plt::resolve_plt_imports` (emitting the **stub** address under the raw `.dynstr` name).
  That stub address is exactly where the demangled `std::terminate` funcsym lives.
- **`engine.rs::commit_analysis_output`** (no-return arm) — resolve by address first
  (`find_function_across_scopes(addr)` → sid), falling back to the name path
  (`query_global_function(name)`) only when nothing is installed at `addr` (or `addr==0`).
  Address resolution finds the demangled/namespaced funcsym; the name fallback preserves the
  old behavior for any future emitter that has only a name. The `s1_sourcelang` Rust-no-return
  wiring is untouched (it flows through the same fact shape) and the fauxware `exit` regression
  still works (now resolved by its PLT-stub address, with the name path as belt-and-braces).

**Fixture.** Vendored `tests/fixtures/cpp_noreturn_x86_64` (+ source
`cpp_noreturn_x86_64.cpp`), built `g++ -O0 -no-pie -fno-pic`. `fail()` (`_Z4failv` →
demangled `fail`, `0x401196`) tail-calls `std::terminate()`; `main` also has a `throw` (→
`__cxa_throw`). `nm`/`objdump` confirm `_ZSt9terminatev`/`__cxa_throw` are UND `.dynsym`
imports whose PLT stubs are `_ZSt9terminatev@plt`=`0x401070`, `__cxa_throw@plt`=`0x4010a0`.

**Proof (the fix is load-bearing).** With the address path the unit test
`noreturn::tests::cpp_mangled_noreturn_emits_plt_stub_address` asserts the facts carry the
stub addresses (`0x401070`/`0x4010a0`) under the raw names; the e2e
`verify_noreturn_demangle.rs` decompiles `fail` and gets `void fail(void)` with the
`Subroutine does not return` warning on `std::terminate()` and **no** dead fall-through.
Manually disabling the address path (forcing the old name-only resolution) regresses to `int4
fail(int4)` with `std::terminate()` returning and spurious dead code after it — the contrast
that confirms the seam.

- **Divergence / scope.** Same always-on, not-`--option`-flippable status as every Wave-1/2
  pass (the deferred per-run gating, conflict #4) — no change here.
- **Tests:** `kuna-analysis` +1 unit (`cpp_mangled_noreturn_emits_plt_stub_address`) and a
  hardened fauxware assertion (the `exit` fact now carries a non-zero PLT-stub address);
  `kuna-console` +1 e2e gate (`verify_noreturn_demangle`). `make test` **PARITY OK**
  (675/675); `make test-stages` **PARITY OK** (158/158); `make rust-test` green.
### Increment 12 — printer renders a pointer-to-readonly-char-array symbol as a string literal; re-enable `s1-strings` ✅

The Wave-3 printer change that resolves the Increment 3 finding: re-enable `StringLiteralPass`
by teaching the C printer to render a pointer to a readonly char-printable **array symbol** as
the string LITERAL (Ghidra behavior), so the planted `char[N]` data symbol and the literal
**coexist** instead of the symbol name shadowing it.

**A/B root-cause pin (done before any edit).** Built the merge-base `kuna`, instrumented the
three candidate render arms in `s9_emit/printc.rs` (the constant-pointer arm in
`push_vn_explicit_ir`, the HighVariable-bound-name arm, the SPACEBASE `op_ptrsub_ir` arm) plus
`push_ptr_char_constant_ir`, and ran fauxware `main` with `StringLiteralPass` **OFF vs ON**:
- **OFF:** the `puts` argument is a plain constant pointer; it reaches the **constant-pointer
  arm** (`TYPE_PTR`, `sub.is_char_print()==true`) → `push_ptr_char_constant_ir`
  (`readonly==true`) → `puts("Username: ")`.
- **ON:** the planted typelocked `char[N]` symbol (`s_400915`) promotes the same constant into
  a global SPACEBASE `PTRSUB(spacebase, 0x400915)` reference (confirmed in `print raw`:
  `->(#0x0,#0x400915)`), so the value now arrives at the **SPACEBASE arm**
  (`name=s_400915, sym_off=0, sym_type=TYPE_ARRAY`), whose `sym_off<=0` branch emits the bare
  `pushSymbol` name → `puts(s_400915)`. **The SPACEBASE arm was the exact shadowing point** —
  not the HighVariable arm (the plan's other candidate). The constant-pointer arm and
  `push_ptr_char_constant_ir` were genuinely never reached when strings is ON.

**The faithful fix (mirrors C++ `PrintC::pushConstant`'s TYPE_PTR → `pushPtrCharConstant`
arm, `decompiler/cpp/printc.cc:1842-1880`).** In `op_ptrsub_ir`'s SPACEBASE arm
(`s9_emit/printc.rs`), before the bare-name `pushSymbol`, add a TIGHTLY-guarded literal route:
when the reference is a whole-symbol (`sym_off==0`), pointer-value (`!arrayvalue`) reference
whose Symbol is a **readonly** (`is_read_only`) char-printable (`is_char_print()` on the array
**element**) **TYPE_ARRAY**, route through the same `push_ptr_char_constant_ir` literal path
the constant arm uses; on success, return (no `&`, no subscript, no name). Every other symbol
render is byte-unchanged — the guard fires only for a readonly char-printable array spacebase
symbol, which the XML datatest corpus never produces (it runs no analyses, plants no such
symbol). `push_ptr_char_constant_ir` was refactored to take the pointer width as `ptr_size:
int4` (was `ct: &Datatype` used only for `ct.get_size()`) so the SPACEBASE arm can pass the
spacebase varnode's width; the one existing caller passes `ct.get_size()` — render-identical.

**Re-enable.** Uncommented `Box::new(crate::s1_strings::StringLiteralPass { min_len: 5 })` in
`passes.rs::passes_for` (now ENABLED, always-on like the other Wave-1/2 passes) and updated the
disabled-rationale comment.

**Result (the win).** With `StringLiteralPass` ON: fauxware `main` → `puts("Username: ")` /
`puts("Password: ")`, `accepted` → `puts("Welcome to the admin console, trusted user!")`,
`rejected` → `printf("Go away!")` — all literals, with the `s_400915`/`s_400920` data symbols
registered in the symbol table (coexistence). And — proving the literal comes from the planted
symbol via the SPACEBASE route, NOT from libproto's `char*` typing — a string passed to a
**non-libproto'd** function (a hand-built `myhandler("a non-prototyped readonly string", …)`
fixture, `myhandler` absent from the libproto table) ALSO renders as the literal. Before this
change that same call would have rendered the bare `s_<addr>` name.

- **Divergence/LOSS:** none to the parity oracles — all three gates byte-identical. The change
  is a render-coexistence fix only; `StringLiteralPass` logic is unchanged. The render is now
  closer to Ghidra (literal AND data symbol coexist) than the prior libproto-only route, but
  the *output* for the covered cases is identical to the strings-OFF render, so no datatest /
  stage assertion moves.
- **Tests:** new `kuna-console` e2e gate `verify_s1_strings.rs` (asserts `s_400915` is in the
  symbol table AND `puts("Username: ")`/`puts("Password: ")` render, NOT `puts(s_4009…)`);
  `kuna-analysis` 88 tests pass. `make test` **PARITY OK** (675/675 — byte-identical);
  `make test-stages` **PARITY OK** (158/158 — byte-identical); `make rust-test` green.

### Increment 13 — per-run `--option <id> on|off` gating of the analysis passes (conflict #4) ✅

Makes each `kuna_analysis` pass flippable per decompilation and discoverable in `kuna
catalog`, resolving the deferred **conflict #4** (the commit-timing ordering bug above).

**The commit-timing change (the core fix).** The analysis-pass commit moved OUT of the eager
`bootstrap_from_elf` body and INTO the (previously stub) `IfcReadSymbols` handler (`read
symbols`). At load, `bootstrap_from_elf` now calls the new
`kuna_analysis::passes::run_default_analyses_per_pass` (returns `Vec<(&'static str,
AnalysisOutput)>` — per-pass split keyed by `AnalysisPass::id`) and **stashes** that on the
`ConsoleProgram` (`pending_analysis` + the captured `analysis_code_space`) instead of
committing. `read_loader_symbols()` + the readonly loader markup stay eager where they were
(they are NOT gated passes; the invariant "commit AFTER `read_loader_symbols`" is preserved —
the deferred commit runs strictly later). `ConsoleProgram::commit_pending_analysis` drains the
stash, keeps only the **enabled** passes' outputs (per-id flag via `analysis_pass_enabled`),
merges them in pass order, and runs the existing `commit_analysis_output`. `IfcReadSymbols`
calls it; `IfcFuncload`/`IfcAddrrangeLoad` call it as a fail-open safety commit (a hand
session that skips `read symbols` still gets the default-on facts before any decode).

**The build_script reorder.** `kuna-cli`'s `build_script` now emits the `option <id> <val>`
lines **before** `read symbols` (was after), so a per-run gate is set before the commit reads
it. Upstream/printer options are order-independent w.r.t. `read symbols`, so this is safe.

**The option registration (option (a) — names in `KUNA_OPTION_NAMES`).** Eight settable rows
added to `stages.toml` (`noreturn_known`, `libproto`, `strings`, `entry_disc`, `arm_markers`,
`dwarf`, `callfixup`, `addrtable`) — the option id IS the pass's `id()`. Each has **no
`live_field`** (they flip a plain `analysis_*` bool, not a printer/engine "live value"
reader), so the codegen `live_from_arch`/`live_value` returns `None` for them; their live
`current` state is surfaced console-side via the hand-written `kuna_live_value`. The same
eight names were added to `kuna_decomp::options::KUNA_OPTION_NAMES` and eight `on_off!` arms to
`Architecture::set_kuna_option`, backed by eight new `analysis_*: bool` fields on
`Architecture` (defaulted in `reset_defaults_internal`: all **on** except `addrtable` **off**,
matching Ghidra's `AddressTableAnalyzer.setDefaultEnablement(false)`). Choosing option (a)
keeps `kuna catalog --check` (which cross-checks `KUNA_OPTION_NAMES` ↔ the catalog) green.

**Defaults / parity.** All-on (except addrtable) = the prior always-on behavior, so the
default render is unchanged. Bound to the real-ELF path only: the XML datatest path stashes
nothing, so the gated commit is a structural no-op there.

**Result (the proof).** `kuna decompile fauxware main` — UNCHANGED (`puts("Username: ")`
etc.). `kuna decompile fauxware rejected --option noreturn_known off` → the no-return is NOT
applied: `exit()` renders as returning and the dead fall-through reappears (vs the default
clean `exit(1)` terminator). `kuna catalog --json` lists all 8 pass options; `kuna catalog
--check` → `catalog OK`.

- **Divergence/LOSS:** none to the parity oracles — default behavior unchanged. The per-pass
  gating granularity is the pass-output split (a disabled pass's *whole* output is dropped);
  this is faithful to Ghidra's per-analyzer enablement. No new analysis logic.
- **Tests:** count tests bumped (settable 23→31: `stages.toml` header, `kuna_stages` tests,
  `catalog_bytecompat`), the `stage_catalog.json` byte-compat fixture regenerated (31 rows),
  the live-reader allowlist test extended for the 8 no-`live_field` gates. Two e2e gates that
  inspected committed facts directly after `bootstrap_from_elf` (`verify_s1_entry`,
  `verify_s1_strings`) now call `commit_pending_analysis()` first (the new contract). `make
  test` **PARITY OK** (675/675); `make test-stages` **PARITY OK** (158/158); `make rust-test`
  green; `kuna catalog --check` **catalog OK**.

### Increment 14 — DWARF named, typed stack locals (`s1-dwarf` subtask 3) ✅

The deferred third DWARF subtask, now resolved. Each defined `DW_TAG_subprogram`'s
direct `DW_TAG_variable`/`DW_TAG_formal_parameter` children that carry a single
`DW_OP_fbreg` (frame-base-relative) stack location are recovered as named, typed
stack locals and installed into the owning function's `ScopeLocal`, so a `-g`
binary's `int accumulator` renders instead of `local_10`. Port of
`DWARFFunctionImporter.processSubprogram`'s commit of `dfunc`'s locals +
`DWARFVariable.readLocalVariableStorage` (the `DW_OP_fbreg`→stack-varnode
resolution).

**Spike finding — the install seam IS cleanly reachable (no shared-engine-path
change).** A function's `ScopeLocal` (`varmap.rs`) is per-`Funcdata` and rebuilt
fresh each decompile — there is *no* persistent function-local scope in the global
symboltab. BUT the kuna console already carries hand-typed `map addr` stack symbols
across that rebuild: it snapshots them off the old `Funcdata` (`mapped_symbol_specs`)
and re-installs each into the freshly-built scope via
`Funcdata::seed_mapped_symbols` → `ScopeLocal::add_symbol(name, type, stackAddr,
usepoint)` + `set_attribute(typelock|namelock)`, threaded through
`decompile_func_full_with_override_dyn`'s `mapped_symbols` argument
(`infra/decompile_drive.rs`). The DWARF locals plug straight into that proven path —
they are just more `(name, type, stackAddr, flags)` entries in `mapped_symbols`. So
subtask 3 needed **zero** change to `decompile_drive`/`Funcdata`/`ScopeLocal`; only
a new fact + a console park/lookup.

**The `DW_OP_fbreg`→stack-offset conversion (faithful).** `cet_pie`'s
`DW_AT_frame_base` is `DW_OP_call_frame_cfa`, so a `DW_OP_fbreg N` location is
`CFA + N`. Ghidra's `DWARFExpressionEvaluator` resolves `DW_OP_call_frame_cfa` to a
stack varnode at the per-language **static `call_frame_cfa`** constant
(`DWARFRegisterMappings.getCallFrameCFA`, read from the processor's `<arch>.dwarf`
`<call_frame_cfa value="N"/>`), then `DW_OP_fbreg` adds `N`:
`stack_offset = call_frame_cfa + fbreg`. kuna transcribes the constant table for the
ELF arches it can produce (x86-64=8 — verified against `x86-64.dwarf`; x86=4;
AArch64/ARM/SPARC/PowerPC=0; RISC-V64=8); an arch with no entry SKIPS fbreg locals
(additive — names+types still apply). Grounded on `cet_pie`: `binary` fbreg -40 →
stack -32, `file` -32 → -24, `elf_header` -24 → -16, matching the disassembly's
`-0x18`/`-0x10`/`-0x8(%rbp)` slots (CFA = rbp+16).

**Wiring.** New `LocalFact{func_addr, name, type_, stack_offset}` +
`AnalysisOutput.locals` (`pass.rs`). `DwarfPass` (`s1_dwarf/mod.rs`) decodes the
single-`DW_OP_fbreg` location (a new `simple_fbreg_location` + a self-contained
SLEB128 reader — gimli exposes SLEB128 only on its streaming cursor), applies the
arch CFA, types each via the existing DIE→`Datatype` mapper, and emits a `LocalFact`
per direct fbreg child (`collect_fbreg_locals`). The commit (`engine.rs`
`commit_analysis_output` arm 8) parks them on `ConsoleProgram.dwarf_locals` keyed by
entry VMA; `ConsoleProgram::dwarf_locals_for(addr)` builds the stack-space `Address`
(wrapping the signed offset, the `map addr s0xffff…e4` convention) into the
`map addr` spec shape; `IfcDecompile` appends `dwarf_locals_for(entry)` to
`mapped_symbols` before the decompile drive runs. A hand-set `map addr` still wins
(`add_symbol`'s overlap arm skips an already-mapped slot). Gated on the `dwarf` pass
flag → real-ELF DWARF path only; the XML datatest path parks none (parity
structurally untouched). No `HashMap` (a `Vec` keyed by linear filter).

**Faithful losses (DOC).** Only **direct** children (a lexical-block-/
inlined-subroutine-nested local is skipped — the same listing-cosmetic scope as the
already-skipped labels/call-sites) and only the **single-`DW_OP_fbreg`** location
form (a composite/register/multi-op location is left to the engine). A local whose
stack slot the engine eliminates never renders — which is exactly what happens to
`cet_pie`'s own locals: they are write-once spill slots the stack/copy analysis
removes (their DWARF symbols install correctly but bind no Varnode). Hence a
dedicated fixture proves the render.

- **Tests:** new `stacklocal_x86_64` fixture (`+.c`) — `compute_sum` with an
  address-taken local that survives; new `s1_dwarf` unit tests (`read_sleb128`
  signed-LEB128 decode; `cet_pie_fbreg_locals_and_cfa_offsets` asserts the three
  fbreg→stack offsets + the x86-64 CFA constant); new e2e
  `verify_s1_dwarf::stacklocal_renders_dwarf_named_typed_locals` (`compute_sum`
  renders `int accumulator`/`int counter`, no `local_10`/`local_c`).
  `kuna-analysis` 91 tests pass; `verify_s1_dwarf` 3/3; `make test` **PARITY OK**
  (675/675); `make test-stages` **PARITY OK** (158/158); `make rust-test` green.
### Increment 15 — Golang no-return list + completeness sweep ✅

Mirrors the Rust no-return work (Increment 7) for **Go**, then sweeps the full analyzer
inventory one more time for any remaining feasible-at-tier ELF analysis.

**`s1-rust-golang-noreturn` (the Go half) — now done.** The Golang no-return list
(`Ghidra/Features/Base/data/GolangFunctionsThatDoNotReturn`, vendored verbatim at
[`data/GolangFunctionsThatDoNotReturn`](../decompiler/crates/kuna-analysis/data/GolangFunctionsThatDoNotReturn))
is parsed **in addition** to the base ELF list when `detect_compiler == Go`, faithful to
`noReturnFunctionConstraints.xml`'s `compiler id="golang"` arm for the ELF executable format
(the exact structural sibling of the `compiler name="rustc"` arm). The list is all **exact**
dotted `runtime.*` names (`runtime.gopanic`, `runtime.throw`, `runtime.goexit.abi0`,
`runtime.fatalthrow`, `runtime.goPanicIndex`, `runtime.abort`, `runtime.sigpanic`, … — **no**
`*` wildcards, unlike the Rust list); the existing `noreturn.rs` list parser handles them
unchanged (they land in the `exact` set). Go's dotted names carry no `::`, so the namespace
guard never fires — `runtime.gopanic` matches as a whole-string exact name.

**Wiring (generalized, not bolted-on).** The previous `NoReturnKnownPass { rust: bool }` was
generalized to `NoReturnKnownPass { compiler: Compiler }` so the per-compiler list selection
mirrors Ghidra's per-`<compiler>` arm selection exactly: `Rustc` → Rust list, `Go` → Go list,
`Gcc`/`Clang`/`Unknown` → base ELF list only. New constructors `::golang()` / `::for_compiler(c)`
join `::elf()` / `::rust()` (kept). `passes.rs::passes_for` now builds the pass with
`NoReturnKnownPass::for_compiler(compiler)` (one line, replacing the `if is_rust()` ternary —
the same pass id `noreturn_known`, so the Increment-13 `--option noreturn_known on|off` gate
and `kuna catalog` are untouched; Go vs Rust is an internal data-file selection keyed on
detection, faithful to Ghidra running one analyzer with per-compiler data files).
`s1_sourcelang` exposes `golang_noreturn_list()` next to `rust_noreturn_list()`; `Compiler`
now derives `Default` (= `Unknown`) so the pass struct can `#[derive(Default)]`.

**Fixture decision: runtime-built real-Go e2e (no vendored fixture), + hermetic synthetic
matching tests.** Go ELF binaries are unavoidably large — a `go build` output is **~1.1 MB**
un-stripped (the whole runtime is statically embedded) and still **~750 KB** stripped. Worse,
the size/coverage tradeoff is forced: a *stripped* Go binary keeps `.go.buildinfo` (so
detection fires) but drops `.symtab` entirely (so there is **no** `runtime.gopanic` FUNC symbol
for the matcher to flag), while only the *un-stripped* 1.1 MB build carries both. Rather than
vendor a 1.1 MB blob (3× the largest existing fixture, `mcount_x86_64` at 896 KB), the e2e
**builds a tiny real Go program at test runtime** (`go build` into an isolated temp dir with a
private GOCACHE/GOPATH; `go 1.18.1` is on the host) — guarded on `go` being on PATH and on the
build succeeding, **skipping cleanly** otherwise (the same "needs an off-host toolchain" posture
as the ARM-link e2e follow-up in Increment 8). On a genuine Go binary it asserts **both** halves:
`detect_compiler == Go` AND `runtime.gopanic` (+ `runtime.throw`, `runtime.goexit.abi0`) flagged
no-return **under the Go arm** carrying real code addresses, but **NOT** under the C arm (the
gating contract) and `main.main` never flagged (no over-acceptance). The list-parse/matching
logic itself is pinned **hermetically** (no fixture, always runs) by
`golang_list_gated_on_go_detection` (a `runtime.*` name spread flagged under the Go arm, never
under the C/Rust arms) + `golang_noreturn_list_carries_runtime_names` +
`compiler_field_selects_extra_list`. So the merge-blocking gates need no `go` and no fixture;
the real-binary proof runs wherever `go` is present (it is here, and the e2e passed).

**Completeness sweep (the user's "any other analyses to do?").** Cross-checked the full
inventory (this doc's "Is this every Ghidra analyzer?" §) against the ported set
(Increments 1–14) by re-reading the candidate Ghidra Java sources at `GHIDRA_REV`. Findings:

- **Implemented here:** the Golang no-return list (above) — the last per-compiler ELF
  no-return arm. ELF now matches **all three** `noReturnFunctionConstraints.xml` ELF lists
  (base + rustc + golang).
- **One near-miss, REPORTED as a follow-up (not implemented — cosmetic, not small):** Ghidra's
  ELF loader names each `.init_array`/`.fini_array`/`.preinit_array` **element** `_INIT_<i>` /
  `_FINI_<i>` / `_PREINIT_<i>` (`ElfProgramBuilder.createDynamicEntryPoints`). kuna's `s1_entry`
  oracle 2 **already discovers** those element addresses as entry points (Increment 5) — it just
  names them `sub_<addr>` (the generic commit-seam name) instead of `_INIT_<i>`. Delivering the
  Ghidra names would require changing the `AnalysisOutput.entries` fact from `Vec<u64>` to a
  named-entry shape (`Vec<(addr, Option<name>)>` or a new fact kind) **and** the commit seam,
  touching all five oracles + their tests — a cross-cutting fact-type change, **not** purely
  additive/small. Its payoff is listing-cosmetic: it only renames a handful of *stripped*-binary
  init/fini thunks (on a non-stripped binary those already have real `.symtab` names, which
  `collect_entries` correctly skips); the decompiled C **body** is identical either way. This is
  the same "listing-naming nicety, deferrable" class as the `$d` data-run markup (Increment 8)
  and the source-info comments (Increment 6). Left as a documented follow-up.
- **Confirmed already-covered / nothing-to-do:** `ExternalEntryFunctionAnalyzer` (its
  external-entry *set* is what the ELF loader seeds + `s1_entry` discovers; its disassembly-
  validation half is post-disasm — infeasible at tier); symbol-versioning `.gnu.version`/`_d`/
  `_r` (Ghidra lays it as listing **comments** only and keeps the *base* name — kuna's
  `elf_plt::strip_version` already captures the sole decompiler-relevant behavior; VERDEF/VERNEED
  are never parsed for naming); `STT_GNU_IFUNC` (Ghidra has **no** special handling — type 10 is
  unrecognized, no `_ifunc` marking — so there is nothing to port); IRELATIVE/COPY relocations
  (Ghidra creates no symbol from them; reloc-driven naming does not exist — the equivalent value
  is captured by plain symbol-table emission, already done); `__libc_csu_init` (an ordinary
  `.symtab` symbol, no special case); TLS / `STT_TLS` (Ghidra itself skips it —
  "Unsupported Thread-Local Symbol not loaded").
- **Confirmed out-of-scope / infeasible (unchanged):** `MachOFunctionsThatDoNotReturn` /
  `PEFunctionsThatDoNotReturn` (non-ELF — the XML scopes them to Mach-O/DYLD/PE; ELF never uses
  them); `ExternalSymbolResolverAnalyzer` (multi-program Ghidra-project operation — links to
  *sibling* library programs; a single-object load tier has no such context); the GOT/PLT
  disassembly engine in `ElfDefaultGotPltMarkup.process` (post-disasm Listing/PseudoDisassembler
  — kuna already does the feasible PLT/GOT **naming** at load); `EmbeddedMediaAnalyzer` (feasible
  at tier but decompiler-irrelevant — image-byte markup); and the large/listing-coupled
  subsystems already classified ⛔ (Go pclntab name recovery `Golang{String,Symbol}Analyzer`,
  FID, `FindNoReturnFunctionsAnalyzer`, AIF, the operand/reference markup family).

**Bottom line of the sweep:** with the Go no-return list in, **every feasible-at-tier,
decompiler-relevant ELF analyzer is ported.** The only remaining items are (a) the one
cosmetic init/fini-array **naming** follow-up above, (b) the previously-documented engine-side
deferrals (DWARF subtask-3 stack-locals, format-string-B varargs override loop, the ARM e2e /
Thumb-FUNC re-home / MIPS `$gp` — all needing an engine change or an off-host fixture), and
(c) the genuinely out-of-scope/huge subsystems (Go pclntab recovery, FID, non-ELF formats,
multi-program resolution). Nothing else is both feasible at this tier and worth implementing.

- **LOSS / scope.** Same as the Rust arm: only the **Known** name-based no-return matching is
  ported. The Golang list's leading comment notes some entries (`runtime.abort`,
  `runtime.systemstack_switch`) "have bytes that can cause undefined instruction errors in
  Ghidra" — a *disassembly*-side concern with no analog at the (pre-disasm) load tier; kuna marks
  them no-return by name regardless, which is the intended effect. The upstream file's six
  trailing-whitespace duplicate lines (e.g. `runtime.panicIndex ` next to `runtime.panicIndex`)
  are vendored **verbatim** and harmlessly collapse to the same entry under the parser's
  per-line `.trim()` (idempotent — the `exact` set is membership-tested, not counted).
- **Divergence:** none to the parity oracles — the Go list only widens matching on a real Go
  ELF (the XML datatest path runs no analyses), and the pass id / option / catalog are unchanged.
- **Tests:** `kuna-analysis` **93 tests pass** (88 + 5 new: `golang_noreturn_list_carries_runtime_names`,
  `golang_list_gated_on_go_detection`, `compiler_field_selects_extra_list`,
  `real_go_binary_detected_and_flags_runtime_gopanic`, and the generalized
  `all_compilers_have_same_pass_ids`). `make test` **PARITY OK** (675/675); `make test-stages`
  **PARITY OK** (158/158); `make rust-test` green; `kuna catalog --check` **catalog OK**.

### Increment 16 — format-string varargs typing (`FormatStringAnalyzer`, half B, gated off) ✅

The deferred half of Increment 9: the decompile→inspect→override→re-decompile loop that
*applies* the parser's output to a printf/scanf call's variadic arguments. `FormatStringAnalyzer`
is genuinely **DecompilerDependent** — the per-call-site format-string constant + which arg is
the format only exist after the caller is lifted to p-code — so this is the console-driver loop
(the kuna analog of Ghidra's `ParallelDecompiler` + `PcodeFunctionParser` +
`HighFunctionDBUtil.writeOverride`), **not** a load-time `AnalysisOutput` pass.

**SPIKE finding: the override + re-decompile seam is cleanly reachable.** Every primitive
already existed (the `override prototype <addr> <decl>` console command is the hand-driven
analog): `pending_proto_overrides` (ifacedecomp.rs, `fn-name → [(callpoint Address,
PrototypePieces)]`) → `Override::insert_proto_override` (overrides.rs) → `applyPrototype` at
flow time (flow.rs `build_call_specs`). The only new code is the pcode-walk + the parse + the
loop. **No new fspec/override surface, no engine-internal action — the parity gates' XML
datatest path never touches this loop.**

**What landed:**
- **Application logic** (pure, unit-tested) in
  [`s1_formatstring/apply.rs`](../decompiler/crates/kuna-analysis/src/s1_formatstring/apply.rs):
  `classify_variadic_call(name)` — Ghidra `VARIADIC_SUBSTRINGS = {"printf","scanf"}` substring
  test + the `INPUT_FUNCTION_SUBSTRING = "scanf"` output/input choice (`FormatStringAnalyzer.java`
  `:42`/`:59`/`:127`/`:273`); `build_override_pieces(name, outtype, fixed_param_types,
  format_specs, types, word_size)` — the analog of `createParameters`/`initSignature` (`:292`/`:313`):
  callee fixed params ++ format-derived types, `first_var_arg_slot = -1` (the override is the now
  *fixed* signature, no longer varargs — matching the plain `FunctionDefinitionDataType` Ghidra
  installs via `writeOverride`).
- **The decompile loop** in `kuna-console`'s `IfcDecompile`
  (`extract_format_string_overrides` + the gated re-decompile): after the first decompile, walk
  the Funcdata's `CALL` ops (`obank().iter_alive` / `FuncCallSpecs`), classify each callee,
  require `is_dotdotdot()` + a fixed format param, read the format constant at the format slot
  (`getInput(getParameterCount())`, `PcodeFunctionParser.java:99`) — resolving the kuna IR's
  derived format-pointer varnode through its defining `PTRSUB`/`PTRADD`/`COPY`/`CAST`/`INT_ADD`
  op (`resolve_const_pointer`) — read the NUL-terminated bytes via `read_loadimage_value`
  (`read_cstring`), parse with Increment 9, build the per-call-site override, push it onto
  `pending_proto_overrides`, and re-decompile once.
- **Gate** `formatstring` (default **off**, Ghidra `FormatStringAnalyzer.setDefaultEnablement(false)`):
  a new `Architecture::analysis_formatstring` bool (architecture.rs), registered in `stages.toml`
  (`change_kind = "analysis-enablement"`) + `KUNA_OPTION_NAMES` + `set_kuna_option`. **Unlike the
  other `analysis_*` flags it does not gate a load-time pass** — `IfcDecompile` reads it after the
  first decompile to decide whether to run the loop. Default-off ⇒ the loop is skipped entirely
  and the decompile is byte-identical.

**LOSS:** per-call-site override only (no listing-data markup / bookmarks); `size_t`/`intmax_t`
width-approximated (inherited from Increment 9's `spec_to_datatype`, kuna has no `.gdt` typedefs);
the format constant must survive as a constant pointer into readable memory at the call site
(Ghidra's `searchForHiddenFormatStrings` non-constant path is not ported — those calls get no
override, the faithful "no override" outcome). x86-64 ELF first cut; the call-arg-slot read is
arch-generic via the recovered proto.

- **Fixture:** `fmt_x86_64` (vendored, source + README) — `int main(int argc,char**argv){
  printf("%d %s\n", argc, argv[0]); return 0;}`, `gcc -no-pie -fno-stack-protector -O0`, not
  stripped (`main`=0x401136, `printf@plt`=0x401040, `"%d %s\n"` @ `.rodata` 0x402004).
- **Tests:** `kuna-analysis` 92 (the 3 new `classify_variadic_call` cases + Increment 9's parser);
  a new `kuna-console` e2e gate
  [`verify_s1_formatstring.rs`](../decompiler/crates/kuna-console/tests/verify_s1_formatstring.rs):
  `--option formatstring on` ⇒ `printf("%d %s\n",a0,(char *)*a1)` (the `%d` arg an `int` with no
  `(uint8)` widening, the `%s` arg cast to `char *`); default-off ⇒ the untyped
  `printf("%d %s\n",(uint8)a0,*a1)`. `make test` **PARITY OK** (675/675); `make test-stages`
  **PARITY OK** (158/158); `make rust-test` green; `kuna catalog --check` **catalog OK** (the new
  `formatstring` option discoverable, default `off`).

### Increment 17 — MIPS `$gp` recovery (`t9` tracking) + the ARM Thumb-FUNC re-home ✅

Closes the two deferred siblings of the Increment 8 `arch-markers` frontier: MIPS `$gp`
recovery (the register-VALUE tracking case) and the ARM Thumb-FUNC address re-home.

**SPIKE finding — the tracked-register-VALUE API is fully present and reachable.** kuna ports
Ghidra's `TrackedSet`/`TrackedContext`/`ContextDatabase` register-value machinery 1:1
(`kuna-sleigh/src/globalcontext.rs`): `get_tracked_default()`, `create_set(addr1,addr2)`,
`get_tracked_set(addr)`, `clone_trackbase()`. The console `set track <reg> <val> [start end]`
(`ifacedecomp.rs` `IfcSettrackedrange`) drives it; the per-function `build_arch_handle`
snapshots the track base into the seam (`seam.tracked_sets = clone_trackbase()`,
`architecture.rs`), and **`ActionConstbase`** (S3, `s3_dataflow/coreaction_early.rs`) consumes
it — emitting `COPY #val -> reg` at the entry block for each tracked register, which constant
propagation then folds. This is **distinct** from the context-BIT machinery Increment 8 used
(`set_variable`/`TMode` decode-mode painting); it seeds a register *value*, not a decode mode.
There is even an existing datatest (`tests/datatests/gp.xml`) that drives it by hand
(`set track t9 0x0 …` → recovers `printf("Hello",a0)`). **Reachable from the commit/bootstrap
path**: `commit_analysis_output` runs at `read symbols`, before any `load function` decode, so
seeding the track base there is correctly timed for the per-function snapshot.

**Why `t9 = func_entry`, not `gp = _gp` (the key design call).** Ghidra's `MipsAddressAnalyzer`
recovers `$gp` with two register-value mechanisms: (1) seed `gp = _gp` per function
(`checkForGlobalGP` + `flowConstants:243`, default-on), and (2) the `assumeT9EntryAddress`
default-true convention (`unknownValue:481-494`) — an *unknown* `t9` read is assumed to be the
function entry (`t9 == entry` because a MIPS function is reached via `jalr t9`). kuna ports
**(2)**: seed `t9 = func_entry` per function, so a PIC prologue's `lui gp; addiu gp; addu
gp,gp,t9` folds to the real `$gp`. **(1) is deliberately NOT ported** here, because kuna's
`ActionConstbase` is the *unconditional* `COPY #val -> reg` form (it lacks Ghidra's *lazy*
`unknownValue` propagator that only assumes a value when the register is genuinely unknown and
`clearRegister`s it after a PIC call). Empirically, an unconditional `COPY #_gp -> gp` at entry
is **counterproductive on a PIC function**: such a function recomputes `gp` from `t9`, and a
pre-seeded `gp` poisons the value (the `$gp`-relative load then fails to resolve). Seeding `t9`
is always correct (the ABI guarantees `t9 == entry` on entry) and never poisons a non-PIC
function (which ignores `t9`). The `gp`-direct seam (for a non-PIC function that loads `$gp` off
the stack and never recomputes it) is a documented LOSS — it needs the lazy-`unknownValue`
propagator.

**New module** [`s1_loader/mips_markers.rs`](../decompiler/crates/kuna-analysis/src/s1_loader/mips_markers.rs)
(sibling of `arm_markers.rs`), `AnalysisPass` id `mips_gp`, stage S1, registered always-on in
`passes_for` after `ArmMarkerPass`. **Gated on MIPS** (`object::Architecture::Mips|Mips64`),
the faithful analog of `MipsAddressAnalyzer.canAnalyze == processor==MIPS`: empty output on
every other language. Emits one `TrackedRegFact { func_addr, reg:"t9", value:func_addr }` per
defined STT_FUNC (UND/zero-address imports skipped). `recover_gp_value` additionally reads the
`_mips_gp_value`/`_gp`/`_GP` symbol (`checkForGlobalGP`) and the MIPS GP-bias invariant
`_gp == .got + 0x7ff0` is cross-checked — exposed for the documented `gp`-direct follow-up.

**New fact kind + commit arm.** `pass.rs` gains `struct TrackedRegFact { func_addr, reg, value }`
+ `pub tracked_regs: Vec<TrackedRegFact>` on `AnalysisOutput` (+ `merge`). `engine.rs`
`commit_analysis_output` gains **step 7**: for each fact, resolve the register varnode
(`get_register_varnode`) and seed the value over `[func_addr, func_addr+1)` via `create_set` +
`TrackedContext` push — the exact `IfcSettrackedrange` body. **CRITICAL gate-safety** (same
shape as the Increment-8 context-paint swallow): `get_register_varnode` returns Err when the
register is undefined by the active language (`t9` on x86-64); the arm `continue`s on that Err —
a faithful no-op, belt-and-suspenders on top of the pass-level MIPS gate.

**Fixture (LINKED, e2e works in-env).** Built
[`mips_gp_le32`](../decompiler/crates/kuna-analysis/tests/fixtures/mips_gp_le32) (7684-byte
dynamic ET_DYN, `mipsel-linux-gnu-gcc -O1 -no-pie`, source vendored). Unlike the ARM `.o`, this
host **has** a MIPS toolchain, so the decode e2e runs in-env. **The proof**: `kuna decompile
mips_gp_le32 _init` — the PIC `_init`@`0x4004cc` computes `gp = _gp_disp + t9`; with `mips_gp`
**on** (default) the `$gp`-relative GOT load resolves (`*(int4 *)(v1 /* t9 */ + 0x10b94)` →
`dat_411060`, a concrete GOT slot); with **`--option mips_gp off`** it regresses to the
unresolved `*(int4 *)(v1 + 0x10b94)`. (`main`/`bump` have a local `lui gp; addiu gp` so their
output is unchanged either way — gp is already constant-folded.)

**ARM Thumb-FUNC re-home (the second deferred sibling).** `arm_markers.rs` now *also* emits a
`SymFact { addr: entry & !1, name, kind: Function }` for each LSB-set STT_FUNC, so a Thumb
function recorded at `entry|1` is **also** known at the even `entry` — the address the `TMode=1`
paint and the engine's decode use — so `load function <name>` / a CALL resolves there. Ghidra
*moves* the symbol (`moveFunction`/`setElfSymbolAddress`); kuna's additive `SymFact` makes the
even entry known without removing the odd-address install (the commit seam's symbol arm is
idempotent — it skips an already-installed function — so the two coexist with no double-symbol
collision). Verified on `arm_thumb_le32.o`: `thumb_add`@`0x1` re-homed to `0x0`, `_start`@`0x15`
to `0x14`. The Increment-8 "paint-only (v1)" LOSS is now resolved.

**ARM decode e2e remains BLOCKED off-host.** This host has NO ARM linker (no lld/aarch64-ld/
arm-ld), so a LINKED ARM ET_EXEC/ET_DYN with PT_LOAD cannot be built in-env (`ObjectLoadImage`
reads only segments). The ARM `.o` unit test stands; the linked-exe Thumb-decode e2e is an
off-host follow-up. (The MIPS e2e, by contrast, runs in-env because the MIPS toolchain — incl.
a linker — is present.)

**Option / catalog.** `mips_gp` (default-on) added like the other analysis-pass gates:
`stages.toml` row + `KUNA_OPTION_NAMES` + the `analysis_mips_gp` flag (`architecture.rs`,
default-on in `reset_defaults_internal`) + the `engine.rs` `analysis_pass_enabled` arm.
`kuna catalog --check` **catalog OK** (settable count 31→32; `stage_catalog.json` fixture +
the two count tests regenerated). `docs/assertions.md` regenerated (single-line addition).

**LOSS / divergences:**
- **The `gp`-direct seam is not ported.** Only `t9 = func_entry` is seeded (see the design
  note). A non-PIC MIPS function that loads `$gp` off the stack and never recomputes it would
  need the `gp = _gp` seed, which needs Ghidra's lazy-`unknownValue` propagator (kuna's
  `ActionConstbase` is unconditional and would poison PIC functions). `recover_gp_value` is in
  place for when that propagator lands. Documented LOSS in `mips_markers.rs`.
- **MIPS16 `ISA_MODE` still out of scope** (the exact ARM `$t`/STT_FUNC-LSB analog — a *context
  bit*, `MIPS_ElfExtension.applyIsaMode`); deferred, needs a MIPS16 fixture (documented in
  `arm_markers.rs`).

**Result (the proof).** `kuna decompile mips_gp_le32 _init` resolves the `$gp`-relative load
(`dat_411060`); `--option mips_gp off` regresses it (unresolved). `kuna decompile fauxware main`
(x86-64) — byte-identical (the MIPS pass is a no-op on a non-MIPS object). `make test`
**675/675 PARITY OK**; `make test-stages` **158/158 PARITY OK**; `make rust-test` green; `kuna
catalog --check` **catalog OK**.

- **Tests:** `kuna-analysis` +5 (3 `mips_markers`: the `t9` scan, `_gp`/bias recovery, the
  non-MIPS-emits-nothing gate; 1 `arm_markers` re-home assertion; 1 `passes` arch-marker
  registration). `kuna-decomp` count tests bumped (settable 31→32) + `stage_catalog.json`
  fixture regenerated.

### Increment 22 — `_INIT_<i>`/`_FINI_<i>` array-element naming ✅

**`s1-entry-disc`** — the cosmetic follow-up flagged by the Increment 15 completeness sweep:
give the dynamic INIT/FINI entries their Ghidra loader names instead of the generic `sub_<addr>`,
faithful to `ElfProgramBuilder.createDynamicEntryPoints`:

- `DT_INIT_ARRAY` element `i` → `_INIT_<i>`, `DT_FINI_ARRAY` element `i` → `_FINI_<i>`
  (Ghidra's `baseName + i`, where `i` is the element index in the array);
- the single `DT_INIT`/`DT_FINI` → `_DT_INIT`/`_DT_FINI` (Ghidra's `"_" + dynamicEntryType.name`).

`DT_PREINIT_ARRAY` (`_PREINIT_<i>`) is wired as the faithful seam (constant + `base_name`
threading) but emits nothing — kuna does not currently *discover* preinit-array elements as
code entries, and adding that would change the discovery set (out of scope here).

**Shape choice — additive overlay, NOT a reshape.** `AnalysisOutput.entries` stays `Vec<u64>`;
a new parallel `entry_names: Vec<(u64, String)>` overlay carries the names (`pass.rs`, `merge`
extended). `s1_entry::collect_entries` is **byte-identical** — the discovery seam is untouched —
and `dynamic_entry_points` is now exactly `dynamic_entry_points_named(file).map(|(a,_)| a)`, a
guaranteed-equal projection (unit-tested: `dynamic_entry_points_is_named_projection`). The
overlay is filtered to the VMAs that actually survive collection, so a name for a dropped
(funcsym-covered) entry is never emitted. This is the lowest-risk shape: WHICH entries are found
provably does not change, so the 675/158 XML oracles (which never construct an `ObjectLoadImage`)
are structurally untouched, and the five other oracles + the address-table pass leave the overlay
empty (→ existing `sub_<addr>` behavior).

**Commit seam** (`engine.rs::commit_analysis_output` step 2): each discovered VMA consults the
overlay; a hit names the function with the Ghidra name, a miss falls back to `name_function`
(`sub_<addr>`) exactly as before. The idempotent `find_function` no-op is kept, so a non-stripped
binary's real `.symtab`/`.dynsym` name still wins — only genuinely new, never-symboled
array-element starts take the `_INIT_<i>` names.

**Flag.** Purely additive on the always-on `entry_disc` pass (no new option, no settable-count
bump): `--option entry_disc off` disables the whole discovery pass (names and all) as before.

**Result (the proof).** On the vendored stripped PIE `stripped_dynamic_x86_64` (`.symtab`
stripped): `.init_array`[0]→0x1240 and `.fini_array`[0]→0x1200 now register as `_INIT_0` /
`_FINI_0`, and `DT_INIT`@0x1000 / `DT_FINI`@0x1464 as `_DT_INIT` / `_DT_FINI`. BEFORE,
`kuna decompile stripped_dynamic_x86_64 sub_1240` resolved; AFTER it errors (`no function
"sub_1240"`) and `kuna decompile … _INIT_0` decompiles a real body. `make test` **675/675 PARITY
OK**; `make test-stages` **158/158 PARITY OK**; `make rust-test` green.

- **Tests:** `kuna-analysis` +3 (`dynamic_entry_names_stripped`,
  `collect_entry_names_matches_collected_entries`, `dynamic_entry_points_is_named_projection`);
  the existing `dynamic_entry_points_stripped` / `collect_entries_*` discovery tests pass
  **unchanged** (byte-identical discovery). `kuna-console` +1 e2e
  (`verify_s1_entry::dynamic_init_fini_elements_get_ghidra_names`: `_INIT_0`/`_FINI_0`/`_DT_INIT`/
  `_DT_FINI` resolve and `_INIT_0` decompiles; `sub_1240` no longer resolves).
### Increment 20 — RISC-V64 PLT import-name end-to-end (linked fixture) ✅

Proves the `elf_plt.rs` RISC-V PLT-veneer decoder end-to-end on a **real, linked,
dynamically-linked RISC-V64 executable**. Increment 1 (`plt-got`) already carried the RISC-V
(RV32/RV64) arm of the per-arch PLT decoder (`decode_riscv`: the 16-byte
`auipc t3,hi; l[wd] t3,lo(t3); jalr t1,t3; nop` GNU `ld` import veneer) plus a synthetic
unit test pinning its arithmetic (`riscv_plt_decode`, positive + sign-extended-negative lo12),
but the proof had run only on x86 fixtures — no linked RISC-V binary existed in-tree to
exercise the full ELF parse → GOT-name-map → stub-decode → funcsym install → decode → print
path. A dev container now carries a RISC-V64 toolchain + linker, so this increment vendors a
linked fixture and adds the e2e gate.

**No `elf_plt.rs` change was needed.** The RISC-V decoder, the `build_got_name_map`
(`R_RISCV_JUMP_SLOT` → `.dynsym` name), and the `bootstrap_from_elf` arch auto-detect
(`RISCV:LE:64:default` → `riscv.lp64d.sla`) all resolve the imports as-is. The increment is
purely **additive coverage** (one linked fixture + one console e2e), so the XML datatest oracle
is structurally untouched (the `<binaryimage>` path never constructs an `ObjectLoadImage`).

**Fixture (`plt_riscv64`, 8520 bytes, source `plt_riscv64.c` vendored alongside).** A dynamic
RISC-V64 PIE built with `riscv64-linux-gnu-gcc 11.4.0` (`-O0`, not stripped). `main` (`0x6b8`)
calls `puts("hello")` (`puts@plt`=`0x5e0` → GOT slot `0x2020`) and `printf("%d\n", argc)`
(`printf@plt`=`0x5f0` → GOT slot `0x2028`); both are `R_RISCV_JUMP_SLOT` relocations in
`.rela.plt` naming `puts`/`printf`, and the stubs are the textbook
`auipc t3,0x2; ld t3,-N(t3); jalr t1,t3; nop` veneer the decoder recognizes (the compressed
`main` body also decodes — RVC). The build host's `kuna-dev` image ships `libc6-riscv64-cross`
(shared libs) but not the dev package; the fixture build installs `libc6-dev-riscv64-cross`
(headers + crt1) in the same root container invocation (see fixtures/README.md provenance).

**Result (the proof).** `kuna decompile plt_riscv64 main`:

```c
unsigned long main(int4 a0)

{
  puts("hello");
  printf("%d\n",(int8)a0);
  return 0;
}
```

The PLT imports render as `puts`/`printf` (not `sub_5e0`/`sub_5f0`), and the `.rodata` string
constants are recovered too. `make test` **675/675 PARITY OK**; `make test-stages`
**158/158 PARITY OK**; `make rust-test` green.

- **Tests:** `kuna-console` +1 e2e (`verify_riscv64_plt.rs`,
  `riscv64_plt_calls_are_named_in_decompiled_c`) — modeled on `verify_w11_elf_plt_names.rs`
  (x86), same specs-absent skip guard; it asserts `main` decoded (real, not skipped) and that
  `puts(`/`printf(` are named while `sub_5e0`/`sub_5f0` are gone. This retires the
  RISC-V half of the "decode e2e proven only on x86" caveat; the ARM-link e2e remains the lone
  off-host blocker (no in-env ARM linker).
### Increment 19 — AArch64 PLT import-name end-to-end (linked fixture) ✅

**What.** Proved the `elf_plt` AArch64 path (`src/s1_loader/elf_plt.rs::decode_aarch64`) resolves
imports **end-to-end on a real, linked, dynamic AArch64 executable** — until now that decoder was
only unit-tested against synthetic veneer bytes (the x86 paths had a real-binary console gate,
`verify_w11_elf_plt_names`, but AArch64 had no linked fixture). A dev container with the AArch64
toolchain + linker (`aarch64-linux-gnu-gcc 11.4.0`) finally makes the linked build possible in-env
(unlike the still-blocked **ARM** decode e2e — no ARM/Thumb linker on host).

**Fixture.** `decompiler/crates/kuna-analysis/tests/fixtures/plt_aarch64` (9056 bytes, ET_EXEC,
Machine AArch64, dynamic — has `.plt` + `.rela.plt` + DT_PLTGOT), built from the vendored
`plt_aarch64.c` (`main` calls `puts("hello")` then `printf("%d\n", argc)`):
`aarch64-linux-gnu-gcc -O0 -no-pie plt_aarch64.c -o plt_aarch64`. Standard GNU `ld` 16-byte
veneer (`adrp x16, GOT_page; ldr x17,[x16,#lo12]; add x16,x16,#lo12; br x17`). Pinned VMAs:
`main`=`0x400604`, `puts@plt`=`0x4004d0` (GOT `0x411018`), `printf@plt`=`0x4004e0` (GOT
`0x411020`); both `R_AARCH64_JUMP_SLOT` in `.rela.plt`.

**e2e.** `decompiler/crates/kuna-console/tests/verify_aarch64_plt.rs`, modeled on the x86 gate:
`bootstrap_from_elf(plt_aarch64, "", [specs])` (the ELF machine → `AARCH64:LE:64:v8A` resolves
automatically) → `load function main` → `decompile` → `print C`; asserts the body names `puts(`
and `printf(` and the old `sub_4004d0`/`sub_4004e0` PLT-stub placeholders are gone. Same
specs-absent skip guard (needs the built `AARCH64.sla`). The test **actually decodes** (does not
skip) on a tree with the spec compiled — it exercises the A64 decoder over `main`.

**No `elf_plt.rs` fix needed.** The existing `decode_aarch64` already handled this binary's
veneer layout correctly; the increment is purely additive (new fixture + new e2e + doc/log). The
BEFORE/AFTER was confirmed by temporarily stubbing the AArch64 arm: with it disabled `main`
renders `sub_4004d0("hello"); sub_4004e0(0x400670,a0);`; with it enabled (the shipped behavior)
`puts("hello"); printf("%d\n",(uint8)a0);`.

**Result (the proof).**
```c
unsigned long main(uint4 a0)
{
  puts("hello");
  printf("%d\n",(uint8)a0);
  return 0;
}
```
`make test` **675/675 PARITY OK**; `make test-stages` **158/158 PARITY OK**; `make rust-test`
green. Additive real-ELF-path change → the XML datatest/stage gates are structurally untouched
(the `<binaryimage>` bootstrap never runs the ELF loader). `kuna-console` +1 test
(`verify_aarch64_plt`).

### Remaining work (essentially complete)

> The build-plan for the analyzers that stay out of reach at this tier
> (infeasible-at-tier, non-ELF, or huge-subsystem — AIF, the operand/reference
> markup family, Go pclntab, FID, PE/Mach-O loaders) — i.e. what kuna would have
> to BUILD first (notably a shared post-disassembly Listing tier), with effort
> sizing, engine seams, and verdicts — is in
> [`analysis-port-buildplan.md`](analysis-port-buildplan.md).

Waves 1–3 (Increments 4–13) **and the entire deferred frontier are done**: DWARF subtask-3
stack locals (14), Golang no-return + completeness sweep (15), format-string-B varargs typing
(16, gated off), and MIPS `$gp` recovery + ARM Thumb-FUNC re-home (17). The completeness sweep
confirms **every feasible-at-tier, decompiler-relevant ELF analyzer is ported** (ELF matches all
three `noReturnFunctionConstraints.xml` lists; the per-compiler/per-arch passes cover the rest).

The deferred frontier is now closed:
- **ARM decode e2e** — ✅ **done in-container** (Increment 18, reinforced by Increment 23). The
  earlier off-host block (no ARM linker on the build host) is lifted by the `kuna-dev`
  container's `arm-linux-gnueabihf-gcc`: the LINKED `arm_thumb_linked_le32` ET_EXEC fixture
  Thumb-decodes `compute` (`a0 * 3 + 7`) through the full pipeline (`verify_arm_thumb_decode.rs`),
  proving the `arm_markers` `TMode` paint + Thumb-FUNC re-home drive a correct decode — no wiring
  fix was needed. Increment 23 additionally exercises the linked ARM/Thumb decode through the
  cross-arch `_start`→`main` path (`entrymain_arm`, `verify_crossarch_entry_main.rs`).
- **`_INIT_<i>`/`_FINI_<i>` array-element naming** — ✅ **done (Increment 22)**: the array-element
  naming follow-up flagged by the Increment 15 sweep, delivered as an additive `entry_names`
  overlay (no reshape of the `entries` fact).

Everything else is inherited by the engine or genuinely out-of-scope for an ELF decompiler
(non-ELF formats, Go pclntab, FID — see the table + inventory).

### Increment 23 — cross-arch `_start`→`main` idiom (AArch64/ARM/RISC-V) ✅

**`s1-entry-disc` oracle 4** — extend the `_start`→`main` libc-start idiom (previously x86-64-only,
a documented LOSS) to **AArch64, ARM/Thumb, and RISC-V**, so a STRIPPED non-x86 PIE (where
`.eh_frame` may be absent or not cover `main`) still recovers `main` without a symbol. Same
disassembly-free, targeted-byte-decode style; gated per arch on `file.architecture()`; the
resolved VMA is validated inside an executable section before emitting; additive to
`AnalysisOutput.entries` (real-ELF path only — rides the always-on `entry_disc` pass, no new flag).

- **The shared modern-PIE shape.** Unlike x86-64 (which carries `main` as a PC-relative
  *immediate* in `lea rdi,[rip+disp]`), the PIE crt1 of all three other arches loads `main`
  *indirectly* from a GOT slot bearing an `R_*_RELATIVE` relocation whose **target is `main`'s
  VMA**. So the cross-arch decode is two steps: (1) byte-decode the arg0-setup in `_start` to the
  GOT-slot VMA, (2) resolve it through a `got_slot → relative_target` map (`relative_targets`):
  the RELA `addend` (AArch64/RISC-V) or the in-place GOT word for ARM REL (`has_implicit_addend`).
  - **AArch64** (`main` in x0): `adrp x0,page ; ldr x0,[x0,#lo12]` → slot
    `(adrp&!0xfff)+page_off+lo12` (the A64 `adrp`/`ldr` decode mirrors `elf_plt::decode_aarch64`,
    keyed to x0). Fixture: slot `0x10ff0` → `main`@`0x714`.
  - **RISC-V** (`main` in a0): `auipc a0,hi20 ; ld a0,lo12(a0)` → slot `auipc+((hi20<<12)+sext(lo12))`
    (mirrors `elf_plt::decode_riscv`, keyed to a0=x10; scanned at 2-byte steps for the RVC mix).
    Fixture: slot `0x2030` → `main`@`0x608`.
  - **ARM/Thumb** (`main` in r0): GOT-relative `ldr r0,[GOT_base,r0]`. Rather than simulate the
    fragile two-load+add GOT-base computation, use the invariant that the GOT base **is** the
    `.got` section address: for each small PC-relative literal-pool word in the `_start` window,
    candidate slot `= .got + off`; if that slot's RELATIVE target lands in an exec section it is
    `main` (the RELATIVE-map + exec-section cross-check is self-validating — a unique winner only).
    The ARM `main` pointer carries the **Thumb mode bit** in bit 0 (`main|1`); masked to `0x4d8`
    for the entry, AND the discovery pass emits a `TMode=1` `ContextPaint` at `0x4d8` (the analog
    of `arm_markers`' STT_FUNC-LSB → `TMode=1`, derived from the GOT pointer LSB — a stripped
    binary has no `$t` mapping symbol). Without that paint the engine decodes the Thumb body as
    A32 and emits a degenerate `void {return;}` stub.

- **Fixtures (vendored, <7 KB each, shared source `entrymain.c`).** `entrymain_{aarch64,arm,riscv64}`,
  built in `kuna-dev` (`<triple>-gcc -O0 -fno-asynchronous-unwind-tables -fno-unwind-tables
  -fvisibility=hidden … && strip`). Two non-obvious flags are load-bearing: `-fvisibility=hidden`
  (without it `main` is a `.dynsym` GLOBAL FUNC — strip removes it on AArch64/ARM but NOT on
  RISC-V, where `.dynsym` survives strip, so oracle 4 couldn't be shown to contribute it);
  `-fno-*-unwind-tables` isolates oracle 4 from the FDE oracle (AArch64/RISC-V keep crt1 FDEs but
  none cover `main`; ARM's `.eh_frame` is empty). RISC-V needs `libc6-dev-riscv64-cross` installed
  in-container first (documented in the fixtures README provenance).

- **Tests.** `kuna-analysis` +4 unit tests (`libc_start_main_idiom_{aarch64,arm,riscv}` — each
  pins the GOT slot + RELATIVE target + recovered `main`; `collect_entries_crossarch_includes_main`
  — proves oracle 4 specifically contributes `main` for each arch, `main` is non-funcsym + in an
  exec section). e2e `kuna-console/tests/verify_crossarch_entry_main.rs` (+3, actually run — not
  skipped): bootstrap each stripped fixture, commit analysis, `load function sub_<main>` →
  `decompile` → `print C` with NO `--addr` → a real `return a0` body. The x86-64 oracle-4 path
  (`libc_start_main_idiom_stripped`, `verify_s1_entry`) is unchanged (now dispatched through the
  per-arch `libc_start_main_target`).

- **Coverage / LOSS.** All three target arches succeed end-to-end (stripped `main` recovered AND
  decompiled). MIPS/PPC `_start` idioms remain a follow-up (those arches no-op). Oracle 5
  (prologue patterns) stays x86-64-only. Oracles 1–3 are arch-independent.

  | arch | `_start` | idiom | GOT slot | `main` | e2e body |
  |---|---|---|---|---|---|
  | AArch64 | `0x600` | `adrp x0;ldr x0,[x0,#4080]` | `0x10ff0` (RELA addend) | `0x714` | `unsigned int sub_714(unsigned int a0){return a0;}` |
  | ARM/Thumb | `0x3dd` | `.got`+`0x28` GOT-rel `ldr r0` + `TMode=1` | `0x10ff8` (REL in-place) | `0x4d8` | `unsigned int sub_4d8(unsigned int a0){return a0;}` |
  | RISC-V | `0x550` | `auipc a0;ld a0,-1318(a0)` | `0x2030` (RELA addend) | `0x608` | `int8 sub_608(int4 a0){return (int8)a0;}` |

### Increment 21 — MIPS16 ISA_MODE decode-mode painting ✅

Closes the last sibling of the Increment 8 `arch-markers` frontier: MIPS16 `ISA_MODE` painting
— the exact MIPS analog of ARM Thumb `TMode` (a decode-mode context BIT, unlike Increment 17's
`$gp`/`t9` tracked-register VALUE). Increment 17 deferred it for lack of a MIPS16 fixture; the
dev container now ships `mips-linux-gnu-gcc 10.3.0` (with `-mips16`).

**Spec finding — `ISA_MODE` exists and is reachable on the default MIPS language.** The MIPS
SLEIGH `contextreg` (`mips.sinc:411`) defines `ISA_MODE=(1,1)` ("=1 Decode using alternate
ISA, variable") behind `@ifdef ISA_VARIANT`. The `mips32{le,be}.slaspec` BOTH `@define
ISA_VARIANT` and `@include "mips16.sinc"`, so the `.sla` carries MIPS16 decode. The MIPS16
constructors gate on `ISA_MODE=1 & RELP=1` (`mips16.sinc:227+`); MIPS32 on `ISA_MODE=0`
(`mips32Instructions.sinc:11`, `@define AMODE "ISA_MODE=0"`). **Crucially `RELP=1` is already
fixed globally** by the default `mips32.pspec` (`<set name="RELP" val="1">`), so painting only
`ISA_MODE=1` at a function entry is sufficient to flip THAT function to MIPS16 — exactly the
Thumb `TMode` situation. A bare MIPS ELF resolves to `MIPS:{BE,LE}:32:default:default`
(`loadimage_object.rs:598`) → `mips32{be,le}.sla`, which has all this. **The context var name
is `ISA_MODE`** (no gap; the spec fully supports MIPS16 decode).

**Toolchain finding — modern binutils marks MIPS16 via `st_other`, not the LSB.** Unlike the
ARM-Thumb `entry|1` assumption in the task brief, `mips-linux-gnu-gcc 10.3.0`'s linker records a
`__attribute__((mips16))` function's STT_FUNC at the **even** entry with `st_other & 0xf0 ==
STO_MIPS_MIPS16 (0xf0)` (the `[MIPS16]` readelf flag), NOT an LSB-set odd value. This is exactly
the upstream `MIPS_ElfExtension.applyIsaMode` (`:412-432`), which enables `ISA_MODE=1` for
EITHER an LSB-set symbol value (re-homed to even) OR `st_other & 0xf0` ∈ {STO_MIPS_MIPS16,
STO_MIPS_MICROMIPS}. **Both branches ported** — faithful to `applyIsaMode`.

**New pass** [`MipsIsaModePass`](../decompiler/crates/kuna-analysis/src/s1_loader/mips_markers.rs)
(sibling of `MipsMarkerPass` in the same module), `AnalysisPass` id `mips_isa`, stage S1,
registered always-on in `passes_for` after `MipsMarkerPass`. **Gated on MIPS**
(`object::Architecture::Mips|Mips64`), the analog of `MIPS_ElfExtension.canHandle ==
processor==MIPS`: empty output on every other language. For each defined STT_FUNC carrying the
alternate-ISA marker it emits `ContextPaint { addr: entry & !1, var: "ISA_MODE", value: 1, end:
None }` — **REUSING the existing context-paint commit arm** (`engine.rs::commit_analysis_output`
step 6, the same arm the ARM `TMode` paints use; the Err-swallow makes it a no-op off-MIPS where
`ISA_MODE` is unregistered) — and, for the LSB-set form only, a re-home `SymFact { addr: entry &
!1, name, kind: Function }` (the MIPS analog of Increment 17's Thumb-FUNC re-home; the
`st_other` form already records the symbol at the even value, so no re-home is needed there).
`st_other` is read through the generic `ObjectSymbol::flags()` → `SymbolFlags::Elf { st_other }`.

**Registration (per the `mips_gp` precedent exactly).** `stages.toml` row (`mips_isa`,
default-on, S1/code-data-partition) + `KUNA_OPTION_NAMES` + `analysis_mips_isa` flag on
`Architecture` (default-on in `reset_defaults_internal`) + the `set_kuna_option` arm + the
`engine.rs` `analysis_pass_enabled` arm + the console `kuna_live_value` arm. Count tests bumped
(settable 33→34; `},\n` 32→33; PASS_GATES +`mips_isa`) + `stage_catalog.json` fixture
regenerated (34 rows) + `docs/assertions.md` regenerated. `kuna catalog --check` **catalog OK**.

**Fixture (LINKED, e2e works in-env).** Built
[`mips16_le32`](../decompiler/crates/kuna-analysis/tests/fixtures/mips16_le32) (1584-byte
freestanding ET_EXEC, big-endian, `mips-linux-gnu-gcc -mips16 -O1 -no-pie -nostdlib
-ffreestanding`, source vendored). Freestanding because the container has the MIPS runtime libc
but no `libc6-dev` (no `crt1.o`/headers); a decode fixture needs no runtime. `m16_square` is a
genuine MIPS16 leaf (`mult a0,a0; mflo v0; jr ra; addiu v0,3`, 8 bytes) at even entry `0x400130`
with `st_other = STO_MIPS_MIPS16`.

**The proof.** `kuna-console/tests/verify_mips16_isa.rs` bootstraps the fixture against the MIPS
spec, commits the analysis (the `ISA_MODE` paint), loads + decompiles `m16_square`:

```c
// AFTER (mips_isa on, MIPS16-decoded):        // BEFORE (mips_isa off, MIPS32 misdecode):
int4 m16_square(int4 a0)                       void m16_square(void)
{                                              {
  return a0 * a0 + 3;                            return;
}                                              }
```

The painted form cleanly recovers `a0 * a0 + 3`; the un-painted form reads the 8 MIPS16 bytes as
2 garbage MIPS32 words and produces an empty `void` body. The two renderings DIFFER — the option
genuinely flips the decode. **The e2e actually decoded MIPS16 (not skipped).**

**Result.** `make test` **675/675 PARITY OK**; `make test-stages` **158/158 PARITY OK**;
`make rust-test` green; `kuna catalog --check` **catalog OK**. Additive real-ELF-path change —
the XML datatest gates are structurally untouched (the `<binaryimage>` path never runs analysis
passes).

- **Tests:** `kuna-analysis` +2 (`mips_markers`: the MIPS16 paint at the even entry, the
  st_other-no-rehome check; the non-MIPS-emits-no-ISA-paints gate; `passes` arch-marker
  registration updated). `kuna-console` +2 (`verify_mips16_isa`: the MIPS16-decode assertion +
  the on/off-differs toggle). `kuna-decomp` count tests bumped (settable 33→34) +
  `stage_catalog.json` fixture regenerated.

With this, **all three `arm-mips-markers` siblings are done** (ARM `TMode`, MIPS `$gp`, MIPS16
`ISA_MODE`); the work-list row is 🟢. The only remaining arch-marker item is the ARM decode e2e
(blocked off-host — no ARM linker).
### Increment 18 — ARM Thumb decode end-to-end (linked fixture) ✅

The deferred Increment-8/17 e2e: prove the analysis-tier `arm_markers` `TMode` paint +
Thumb-FUNC re-home actually drive a **correct Thumb decode through the full pipeline** on a
**LINKED** ARM executable (not the bare `.o` the unit test used). Increments 8 and 17 proved
the *paint* (`$t`/`$a`+STT_FUNC-LSB → `TMode=1` `set_variable`) and the *re-home* (even-address
`SymFact`) only on `arm_thumb_le32.o` (ET_REL); the decode e2e was blocked because the build
host had no ARM linker. The **`kuna-dev` container** now provides `arm-linux-gnueabihf-gcc`, so
the e2e is no longer off-host.

**Fixture (LINKED ET_EXEC).** `arm_thumb_linked_le32` (1080 bytes), built in-container:
`arm-linux-gnueabihf-gcc -mthumb -static -nostdlib -e _start arm_thumb_linked_le32.c -o
arm_thumb_linked_le32` (arm-linux-gnueabihf-gcc 11.4.0). `readelf -h`: Type **EXEC**, Machine
**ARM**; `readelf -l`: one **LOAD R E** segment at `0x10000` (so kuna's `ObjectLoadImage`, which
reads only segments, loads it — unlike the bare `.o`). `readelf --syms`: the `$t` mapping symbol
@`0x100b8` (NOTYPE LOCAL), `compute` FUNC @`0x100b9` (LSB-set Thumb `entry|1` → even decode
`0x100b8`), `_start` FUNC @`0x100d7` (→ even `0x100d6`). `compute` is non-trivial (`x*3 + 7`: the
Thumb `lsls #1; add; adds #7`) so a correct Thumb decode is visibly distinct from an ARM-mode
(A32) misdecode of the same bytes (garbage). Source vendored alongside; provenance line added to
`tests/fixtures/README.md`.

**The e2e (`kuna-console/tests/verify_arm_thumb_decode.rs`).** Modeled on
`verify_w11_elf_plt_names.rs` (same `bootstrap_from_elf` → `load function` → `decompile` → `print
C` drive + the specs-absent skip guard). Two proofs in one gate:
1. **TMode paint** — `load function compute` (which fires `commit_pending_analysis`, painting the
   stashed `TMode=1` over the ContextDatabase **before** the decode) Thumb-decodes to
   `return a0 * 3 + 7;`. The arithmetic IS the Thumb-mode proof (an A32 misdecode can't produce it).
2. **Thumb-FUNC re-home** — `load function _start` renders the `bl` to `compute`'s **even** entry
   (`0x100b8`, where execution lands) by name as `compute(5)`. Without the re-home's even-address
   `SymFact` the engine symbol table would only know `compute` at the odd `0x100b9`, so the CALL
   would render an anonymous `sub_<addr>`. The test asserts `compute(` is present and `sub_100b8`
   is not. (The loader-symbol list `lookup_symbol` still returns the raw odd `entry|1` — the test
   pins that real behavior too; the decode at the odd address still lands on the even bytes with
   `TMode=1` painted.)

**No wiring fix needed.** The `arm_markers` pass + the commit seam (`commit_analysis_output`
step 6, `set_variable` over the ContextDatabase) + the SLEIGH translator already decode Thumb
correctly end-to-end. The test ran a **real decode** (the ARM `ARM8_le.sla` is built; the skip
guard was not hit). Equivalently via the CLI: `kuna decompile arm_thumb_linked_le32 compute` →
`return a0 * 3 + 7;` and `… _start` → `compute(5)`.

**Result (the proof).** `make test` **675/675 PARITY OK**; `make test-stages` **158/158 PARITY
OK**; `make rust-test` green (incl. the new `verify_arm_thumb_decode`). The change is purely
additive on the real-ELF path (a new fixture + a new console test + the `arm_markers`/commit code
unchanged), so the XML datatest oracle is structurally untouched.

- **Tests:** `kuna-console` +1 (`verify_arm_thumb_decode::arm_thumb_compute_decodes_in_thumb_mode`).
  New fixture `arm_thumb_linked_le32` (+ source). No engine/option/catalog change.

### Increment 24 — SPARC PLT import-name recovery ✅

Adds SPARC (SPARC v9, 64-bit, big-endian) to the per-arch PLT decoder in
`elf_plt.rs` (`decode_sparc`) and proves it end-to-end on a **real, linked,
dynamically-linked SPARC64 executable**. SPARC was the last common Linux/SysV
machine with a regular `.plt` whose imports kuna still rendered as `sub_<addr>`;
its arch auto-detect (`sparc:BE:64:default` → the SPARC `.sla`) was already wired
in `loadimage_object::language_id_for`, but `decode_plt_section` had no SPARC arm
(it hit the `_ => {}` PPC/MIPS seam fallthrough), so the GOT-name map never reached
the call targets.

**Why SPARC is the odd one out (and why the decoder is the simplest of the set).**
Every other supported arch decodes the stub's indirect jump to find a *separate*
`.got` slot, then matches that slot against `build_got_name_map`. SPARC doesn't
work that way: its `.rela.plt` `R_SPARC_JMP_SLOT` relocation `r_offset` **is the
PLT entry address itself** — the dynamic linker rewrites the in-place 32-byte stub
at resolution time rather than redirecting through a GOT word (confirmed against
Ghidra's `SPARC64_ElfRelocationHandler`, which materializes the resolved
`sethi/or/sllx/jmpl` sequence directly into the relocation address). So
`build_got_name_map`'s keys are *already* the call targets the decompiler sees.
`decode_sparc` walks the `.plt` in 32-byte strides, gates each entry on the
canonical `sethi %hi(...),%g1` veneer head (BE `0x03xxxxxx`: op=00, rd=%g1,
op2=0b100), and records any entry that is a known relocation — the stub address
and the map key are one and the same (`record(entry, entry, …)`). The 4-slot
(`0x80`-byte) reserved PLT0 header and the non-symbol `__gmon_start__` slot are not
relocation keys, so they fall out by the same self-correcting cross-check the other
arches use.

**Fixture (`plt_sparc64`, source `plt_sparc64.c` vendored alongside).** A dynamic
SPARC v9 / ELF64 / big-endian **EXEC** built with `sparc64-linux-gnu-gcc -O0` (not
stripped). `main` (`0x100750`) calls `puts("hello")` (`puts@plt`=`0x2021c0`) and
`printf("%d\n", argc)` (`printf@plt`=`0x2021a0`); both are `R_SPARC_JMP_SLOT`
relocations in `.rela.plt` whose `r_offset` equals the entry address, and each
`.plt` entry is the textbook 32-byte `sethi %hi(...),%g1; b,a %xcc,<resolver>;
nop*6` veneer the decoder recognizes (the 4-slot `0x80`-byte PLT0 header precedes
the imports). The `kuna-dev` image ships `sparc64-linux-gnu-gcc` but not the SPARC
libc dev package; the fixture build installs `libc6-dev-sparc64-cross` (headers +
crt1) in a single `--user root` container invocation (see fixtures/README.md
provenance), exactly as the RISC-V port did.

**Result (the proof).** `kuna decompile plt_sparc64 main`:

```c
unsigned long long main(int4 a0)

{
  puts("hello");
  printf("%d\n",(int8)a0);
  return 0;
}
```

The PLT imports render as `puts`/`printf` (not `sub_2021c0`/`sub_2021a0`), and the
`.rodata` string constants are recovered too. This is a **clean win**, not a
documented seam — SPARC's `.plt` is regular and statically decodable. `make test`
**675/675 PARITY OK**; `make test-stages` **158/158 PARITY OK**; `make rust-test`
green (incl. the new `verify_sparc_plt` + the `sparc_plt_decode` unit test). The
change is purely additive on the real-ELF path (a new `decode_sparc` arm + a new
fixture + a new console e2e), so the XML datatest oracle is structurally untouched
(the `<binaryimage>` path never constructs an `ObjectLoadImage`).

- **Tests:** `kuna-analysis` +1 unit (`elf_plt::tests::sparc_plt_decode`, the
  32-byte strider over a header + three synthetic veneers); `kuna-console` +1 e2e
  (`verify_sparc_plt.rs::sparc_plt_calls_are_named_in_decompiled_c`) modeled on
  `verify_riscv64_plt.rs`/`verify_w11_elf_plt_names.rs`, same specs-absent skip
  guard, asserting `main` decoded (real, not skipped) and `puts(`/`printf(` named
  while `sub_2021c0`/`sub_2021a0` are gone. New fixture `plt_sparc64` (+ source).
  No engine/option/catalog change. This retires the SPARC `sub_<addr>` row in the
  `plt-got` work-list and the SPARC line of missing-analyses §1.


### Increment 25 — format-string varargs typing: cross-arch coverage (AArch64/ARM/RISC-V) ✅

Closes the Increment 16 follow-up — its half-B override loop was an "x86-64 ELF first cut"
("the call-arg-slot read is arch-generic via the recovered proto"). This increment proves
(and, for ARM, fixes) `--option formatstring on` on the other three first-class arches.

**Three new fixtures, same C as `fmt_x86_64`** (`int main(int argc,char**argv){
printf("%d %s\n", argc, argv[0]); return 0;}`), built in the `kuna-dev` container,
**un**stripped: `fmt_aarch64` (`aarch64-linux-gnu-gcc`), `fmt_arm` (32-bit Thumb,
`arm-linux-gnueabihf-gcc`), `fmt_riscv64` (RVC/lp64d, `riscv64-linux-gnu-gcc`), all
`-O0 -fno-stack-protector` (gcc 11.4.0). The RISC-V dev package (`crt1.o` + headers) is not
in the base image — the build recipe `apt-get update && apt-get install
libc6-dev-riscv64-cross` (README provenance). Pinned consts per arch: AArch64 `main`=0x754,
`printf@plt`=0x630, format `.rodata` vma 0x7a8 (`adrp x0,0; add x0,x0,#0x7a8`); ARM
`main`=0x504, `printf@plt`=0x3e4, format vma 0x5cc (`ldr r3,[pc,#20]`→`.word 0xb0`@0x52c;
`add r3,pc`: pc 0x51c + 0xb0); RISC-V `main`=0x668, `printf@plt`=0x5a0, format vma 0x6a8
(`auipc a0,0x0; addi a0,a0,32`: pc 0x688 + 32).

**Finding — AArch64 and RISC-V worked unchanged.** Both materialize the format address
directly into a register (`adrp+add` / `auipc+addi`), so the call's format-arg varnode is
already a constant pointer and the Increment-16 `resolve_const_pointer` (through
PTRSUB/PTRADD/COPY/CAST/INT_ADD) reads it as-is. `--option formatstring on` ⇒
`printf("%d %s\n",a0,(char *)*a1)` on the first try (default-off: AArch64
`printf("%d %s\n",(uint8)a0,*a1)`, RISC-V `printf("%d %s\n",(int8)a0,*a1)`).

**ARM needed a fix — the read-only literal-pool addressing form.** On ARM the format address
is loaded *PC-relatively from a read-only literal pool* (`ldr r3,[pc,#k]; add r3,pc`), so the
format-arg varnode is a memory LOAD, not a constant; default-off it renders the unresolved
`printf((char *)(dat_52c + 0x51c),a0,*a1)`. The literal-pool LOAD only constant-folds under
`readonlypropagate` (`Funcdata::fillin_read_only`) — with `--option readonly on` the first
decompile already produces `u0x… = ->(#0x0,#0x5cc)` (a clean PTRSUB the existing resolver
handles). This is **not** a `resolve_const_pointer` gap — the value is genuinely a read-only
memory load the engine declined to fold.

**The fix (small, additive, gated under `formatstring`).** `IfcDecompile` now enables
`readonlypropagate` *for the duration of a `formatstring`-on decompile* and restores the prior
value afterward (and on the `has_no_code` early-return). This is faithful to Ghidra's
`FormatStringAnalyzer`, whose decompile reads constant strings out of read-only memory; it
keeps the feature self-contained on ARM with a single `--option formatstring on` (no separate
`--option readonly on`). **Gate-safe:** the toggle is inert (value unchanged) unless
`formatstring` is on (default off), so every parity gate — which never sets `formatstring` —
is byte-identical. With the fix, ARM `--option formatstring on` ⇒
`printf("%d %s\n",a0,(char *)*a1)`.

**Result — all four arches type correctly with a single `--option formatstring on`:**
`printf("%d %s\n",(uint8)a0,*a1)` (or the arch's untyped default) → `printf("%d
%s\n",a0,(char *)*a1)` (the `%d` arg a plain `int`, no widening cast; the `%s` arg cast to
`char *`).

- **Tests:** a new console e2e gate
  [`verify_formatstring_crossarch.rs`](../decompiler/crates/kuna-console/tests/verify_formatstring_crossarch.rs)
  — 6 tests (off+on × AArch64/ARM/RISC-V), **all ran** (the per-arch `.sla` are built; the
  specs-less skip guard was not hit). The existing `verify_s1_formatstring` (x86-64) still
  passes. No new option/catalog surface (reuses the existing `formatstring` + `readonly`
  flags). `make test` **675/675 PARITY OK**; `make test-stages` **158/158 PARITY OK**;
  `make rust-test` green; `kuna catalog --check` **catalog OK**.
- **Fixtures:** `fmt_aarch64`, `fmt_arm`, `fmt_riscv64` (+ sources, README provenance).
- **Engine change:** one scoped `readonlypropagate` flip in `kuna-console`'s `IfcDecompile`,
  entirely under the `formatstring` gate.


### Increment 26 — PowerPC64 PLT import-name recovery (clean port, not a seam) ✅

Recover **PPC64 ELFv2 (little-endian)** libc import names so a call renders `puts`/`printf`,
not `sub_<addr>`. PowerPC was listed in `docs/missing-analyses.md` §1 as a *documented seam*
("ELFv2 `.plt` is a data table; call stubs are synthesized in `.text`") — but, like SPARC
(PR #20), investigating the real layout showed it is **fully tractable at the static load
tier**. This is a clean port, not a documented seam.

**The layout (why it looked like a seam, and why it isn't).** On PPC64 ELFv2 there is no `.plt`
*code* section: `readelf -S` shows `.plt` as **NOBITS** (the runtime GOT for imports), and the
`.rela.plt` `R_PPC64_JMP_SLOT` relocations name `.plt` *data* slots (`0x1fef0`=puts,
`0x1fef8`=printf in the fixture). The call stubs are synthesized inline in `.text` by the linker.
`main` (`0x8bc`) `bl`s to a stub at `0x680` (puts) / `0x660` (printf), each of the canonical
ELFv2 shape:

```
    std   r2,24(r1)         ; 0xf8410018  save caller TOC
    addis r12,r2,off@ha     ; r12 = TOC_base + (off@ha << 16)
    ld    r12,off@l(r12)    ; r12 = *(.plt slot) = *(TOC + (off@ha<<16) + off@l)
    mtctr r12
    bctr
```

The decoded `.plt` slot ties straight back to the `R_PPC64_JMP_SLOT` name. The one piece the
other arches don't need is the **TOC base**: ELFv2 fixes `r2` at `.got + 0x8000` (the `.TOC.`
symbol). For the fixture `.got`=`0x1ff00`, so TOC=`0x27f00`, and the puts stub's
`addis r12,r2,-1; ld r12,32752(r12)` reconstructs `0x27f00 + (-1<<16) + 32752 = 0x1fef0` —
exactly the JMP_SLOT offset. No `.TOC.` symbol lookup is needed; `.got + 0x8000` is the ELFv2
invariant.

**The decoder (`s1_loader/elf_plt.rs`).** `resolve_plt_imports` already builds `got_slot → name`
from *all* symbol-bearing dynamic relocations (the JMP_SLOTs are in it — `object` 0.39 parses the
PPC64 relocs as `kind=Unknown target=sym#N` at the slot offset, which the existing
`build_got_name_map` handles unchanged). Added a PowerPC arm: after the `.plt*`-section scan,
`decode_ppc_text` computes the TOC base (`.got` vma + `0x8000`), reads `.text`, and
`decode_ppc64_stubs` scans for the 5-instruction stub (`std r2,24(r1)` + `addis r12,r2` +
`ld r12,(r12)` DS-form + `mtctr r12` + `bctr`), reconstructs the slot, and `record`s the stub
entry (the `std r2,24(r1)` address — what `bl` targets) against the matched name. It reads words
honoring file endianness (`is_little_endian`), so a PPC32/BE arm can slot in later; the gating is
`arch == PowerPc64` only (PPC32 secure-PLT has a different stub shape — left a seam).

**Why it integrates cleanly.** The `.symtab` carries synthetic `0000001b.plt_call.puts@@…`
symbols *at the stub addresses*, but `object` classifies them `SymbolKind::Unknown` (not `Text`),
so `loadimage_object` step 1 skips them — leaving the stub addresses free for the PLT decoder
(step 2) to register the clean `puts`/`printf`. No name collision, no ordering hazard.

**Fixture (LINKED dynamic PPC64le ELFv2 PIE).** `plt_ppc64le` (~21 KB), built in-container:
`powerpc64le-linux-gnu-gcc -O0 plt_ppc64le.c -o plt_ppc64le` (libc6-dev-ppc64el-cross installed
`--user root` for crt1/headers). `main` calls `puts("hello")` + `printf("%d\n", argc)`. Source +
provenance vendored in `tests/fixtures/`.

**e2e (`kuna-console/tests/verify_ppc64_plt.rs`).** Modeled on `verify_riscv64_plt.rs`: auto-detect
`PowerPC:LE:64:default` → `bootstrap_from_elf` → `load function main` → `decompile` → `print C`,
with the same specs-absent skip guard. The test **ran a real decode** (the `ppc_64_le.sla` is built;
the skip was not hit) and asserts `main` resolves, `puts(`/`printf(` are named, and `sub_680`/`sub_660`
are gone.

**BEFORE → AFTER** (`kuna decompile plt_ppc64le main`):
```
  before:  sub_680(v1 + 0x17c);            sub_660(v2 + -0x274c0,(int8)a0);
  after:   puts((char *)(v1 + 0x17c));     printf((char *)(v2 + -0x274c0),(int8)a0);
```

**Result.** `make test` **675/675 PARITY OK**; `make test-stages` **158/158 PARITY OK**;
`make rust-test` green (incl. the new `verify_ppc64_plt` + the `ppc64_plt_decode` unit test). Purely
additive on the real-ELF path (new PowerPC arm + new fixture + new console test); the XML datatest
oracle is structurally untouched.

- **Tests:** `kuna-analysis` +1 unit (`elf_plt::tests::ppc64_plt_decode`, LE real-fixture bytes +
  a BE word-layout case). `kuna-console` +1 e2e (`verify_ppc64_plt`). New fixture `plt_ppc64le`
  (+ source). No engine/option/catalog change.


### Increment 27 — MIPS PLT/stub import-name recovery (linked fixture) ✅

**Goal.** Recover MIPS libc import names so a call renders `puts`/`printf`, not
`sub_<addr>`. The other arches (x86-64/32, AArch64, ARM32, RISC-V; SPARC in PR
#20) decode a regular `.plt` code section whose stub references a
`got_slot → name` map built from the dynamic relocations. **MIPS o32 has neither:**
the GNU toolchain emits **no `.plt` and no `R_MIPS_JUMP_SLOT` relocations** — it
uses the classic `.MIPS.stubs` + `$gp`-relative GOT lazy-binding layout.

**What the toolchain produced (investigated, not assumed).** `mips-linux-gnu-gcc
-O0` on a `main` calling `puts`/`printf` → big-endian MIPS32 ET_EXEC with:
`readelf -r` = *"There are no relocations in this file."*; a `.MIPS.stubs` PROGBITS
section of `lw $t9,-32752($gp); move $t7,$ra; jalr $t9; li $t8,<dynidx>` lazy
resolver stubs; and the `DT_MIPS_*` dynamic tags `DT_MIPS_LOCAL_GOTNO=6`,
`DT_MIPS_GOTSYM=5`, `DT_PLTGOT=0x411020`. So this is layout **(b)** from the task
brief (classic `.MIPS.stubs`), NOT a modern regular-`.plt` + `R_MIPS_JUMP_SLOT`
build. **This is a clean port, not a documented seam** — the names ARE recoverable
statically.

**The MIPS GOT correspondence (the cr0.org "MIPS Multi-GOT" layout Ghidra cites).**
The dynamic linker resolves a positional slice of the GOT in symbol-table order:
dynamic symbols `[GOTSYM, dynsym_count)` map 1:1, in order, to GOT entries starting
at index `LOCAL_GOTNO`: `got_index(i) = LOCAL_GOTNO + (i - GOTSYM)`. For an
undefined FUNC import the GOT slot's **static contents** are the address of that
import's `.MIPS.stubs` stub (and that same stub address is the symbol's
`st_value`). A call site is `lw $t9, off($gp); jalr $t9` — an indirect call through
the GOT slot. Cross-checked on the fixture: `puts` (dynidx 7 → got_index 8 → slot
`0x411040`) holds `0x400800`; `printf` (dynidx 8 → got_index 9 → slot `0x411044`)
holds `0x4007f0` — exactly the `dat_411040`/`dat_411044` the decompiler reads.

**The port — two halves, both faithful to `MIPS_ElfExtension`
(`Ghidra/Processors/MIPS/.../elf/extend/MIPS_ElfExtension.java`).**
1. **Naming** (`elf_plt::resolve_mips_imports`, the `fixupGot` analog): walk the
   external-symbol GOT window, read each FUNC import's GOT slot, and name the
   **stub address** (the slot's static contents = `refAddr` in Ghidra's
   `createExternalFunctionLinkage(symName, refAddr, gotEntryAddr)`). Gated on
   `Mips | Mips64`, runs *before* the relocation-driven path (which would early-
   return on MIPS's empty `build_got_name_map`). Off-MIPS: empty, no-op.
   (In practice the `.symtab`/`.dynsym` UND-import path already registers the stub
   address too — `object` reports the MIPS UND import with a nonzero `st_value`
   pointing at the stub — so the two agree on the same `addr → name`.)
2. **Constant-folding the indirect call** (`elf_plt::mips_got_const_ranges` +
   `ObjectLoadImage::const_ranges` → `get_readonly`, the `setConstant` analog): the
   GOT external slots are reported as constant ranges so the engine can fold the
   `lw $t9, off($gp)` load (whose address is already `$gp`-constant from Increment
   17's `$gp` recovery) to the stub address; `bootstrap_from_elf` turns on
   `readonlypropagate` **for MIPS only** so `ActionVarnodeProps::fillinReadOnly`
   performs the fold and `ActionDeindirect` resolves the now-constant call target
   to the named stub function. Scoped to MIPS (`arch_type.starts_with("MIPS:")`),
   so non-MIPS ELF output is unchanged, and `option readonly off` restores the raw
   `(*(code *)(dat_411040 & ...))(...)` GOT-load rendering.

**BEFORE / AFTER (`kuna decompile plt_mips32 main`).**
```text
before:  (*(code *)(dat_411040 & 0xfffffffe))("hello");
         (*(code *)(dat_411044 & 0xfffffffe))(0x400888,v1);
after:   puts("hello");
         printf(0x400888,v1);
```
(The `printf` format-string pointer `0x400888` not rendering as a string literal
is a separate string-rendering concern, independent of import naming; `puts`'s
`"hello"` already renders.)

**Fixture (LINKED ET_EXEC, big-endian).** `plt_mips32` (7580 bytes), built in the
`kuna-dev` container: `mips-linux-gnu-gcc -O0 plt_mips32.c -o plt_mips32` (the
`-O0` keeps the calls **plain** `puts`/`printf`; `-O1`+ pulls in glibc's fortified
`__printf_chk`). `libc6-dev-mips-cross` is installed in the build container (the
image ships only the runtime libc). `main`@`0x400700`, `puts` stub `0x400800`,
`printf` stub `0x4007f0`. Source + the exact single-invocation build command
vendored in `tests/fixtures/README.md`.

**The e2e (`kuna-console/tests/verify_mips_plt.rs`).** Modeled on
`verify_riscv64_plt.rs` (same `bootstrap_from_elf` → `load function main` →
`decompile` → `print C` drive + the specs-absent skip guard). Asserts the loader
resolves `puts`/`printf`/`main`, the body decodes (`out.contains("main")` — a real
big-endian MIPS decode, not a skip), the call sites render `puts(`/`printf(`, and
the raw `dat_411040`/`dat_411044` GOT-slot loads are gone. **The test RAN (not
skipped)** — `mips32be.sla` is built.

**Result (the proof).** `make test` **675/675 PARITY OK**; `make test-stages`
**158/158 PARITY OK**; `make rust-test` green (incl. the new `verify_mips_plt` +
two `loadimage_object` MIPS unit tests). Purely additive on the real-ELF path (a
new fixture + a new console test + the MIPS arm in `elf_plt`/`loadimage_object` +
the MIPS-only `readonlypropagate` toggle in `bootstrap_from_elf`), so the XML
datatest oracle is structurally untouched (no XML path constructs an
`ObjectLoadImage` or reaches `bootstrap_from_elf`).

- **Engine:** `elf_plt.rs` (+`resolve_mips_imports`/`mips_got_const_ranges`/
  `mips_external_got_entries`/`read_mips_dynamic_tags`/`read_word_at_vma`),
  `loadimage_object.rs` (+`const_ranges` field, threaded through `get_readonly`
  /`adjust_vma`), `engine.rs` (MIPS-only `readonlypropagate = true`). No
  option/catalog change (`readonly` is a pre-existing `option`; the MIPS default is
  set in the loader path, user-overridable via `option readonly off`).
- **Tests:** `kuna-console` +1 (`verify_mips_plt`); `kuna-analysis` +2
  (`mips_stub_imports_resolve_to_named_functions`,
  `mips_got_const_ranges_cover_external_slots`). New fixture `plt_mips32` (+ source).

### Increment 28 — Listing/xref tier PR0: recursive-descent core + instruction/flow/xref/function model ✅

**Goal.** Land the **keystone** of "scope-B": a post-disassembly,
program-wide **recursive-descent disassembler** + an instruction / cross-reference
/ discovered-function model, built by *reusing* the ported SLEIGH decoder and a
*lifted copy* of the S2 flow classifier. This is the first PR of a multi-PR tier
whose canonical spec is the new **`docs/listing-tier-design.md`** (transcribed
from the design fan-out). The keystone unlocks Ghidra analyzers kuna cannot port
today (foremost the "Discovered" `FindNoReturnFunctionsAnalyzer`), which need a
disassembled `Listing`/`ReferenceManager`/`FunctionManager` the analysis tier has
never had.

**Scope (PR0 ONLY).** The core lands as a self-contained module
`kuna-analysis/src/listing/` and is **not wired into the engine**: no
`--option listing` flag (PR1), no `AnalysisCtx::listing` field / driver build
(PR1-PR2), no `context.rs` ARM/MIPS decode-paint (PR5). The CodeUnit partition
*queries* are PR3, but the `CodeUnit` type and the `covered`/`exec_ranges` fields
are defined now. So this PR touches **no engine path the XML datatests use** — the
module is dormant until a later PR invokes it.

**The subsystem.**
- `model.rs` — the §2 data model: `Insn` (addr/len/fall_through/flow/flows/
  mnemonic/lazy-pcode), `FlowType` (the faithful `FlowType` predicate set:
  is_call/is_jump/is_terminal/is_computed/is_indirect/is_conditional/
  has_fallthrough) + `FlowKind`, `RawOp`, `Reference`/`RefKind` (Call/Code
  populated; Data/Read/Write reserved), `DiscoveredFunction` (ordered
  `BTreeMap` model), `CodeUnit` (Instruction/Data/Undefined partition class).
- `decode.rs::decode_one` — drives `Translate::one_instruction` (`translate.rs:472`)
  with a capturing `PcodeEmit` (`translate.rs:166`) for `(len, ops)` and
  `print_assembly` (`translate.rs:481`) with a capturing `AssemblyEmit` for the
  mnemonic; builds the `Address` in the default code space.
- `classify.rs::classify` — a faithful transliteration of
  `s2_lift/flow.rs::xref_control_flow` (`flow.rs:1039-1185`), honoring the three
  gotchas: (1) a constant-space `in0` is p-code-relative (tested via
  `is_constant()` first — never a VMA target); (2) fall-through is decided by the
  **last** op (BRANCH/BRANCHIND/RETURN ⇒ no fall-through); (3) delay slots are
  already folded into `len` (no re-decode). BRANCHIND/CALLIND are recorded as
  computed/indirect with NO static target (deferred jump-table resolution).
- `walk.rs` — the two-level recursive-descent: an outer function worklist
  (CALL/CALLIND direct targets become new function entries + `RefKind::Call`
  edges) and an inner per-function instruction worklist (branch/fall-through →
  same-function successors + `RefKind::Code` edges), with `BTreeMap`-membership
  dedup, an exec-range bounds gate, and decode-error = stop-this-path.
- `mod.rs` — the `Listing` facade + `Listing::build(file, image, arch, translate,
  seeds)` and the read-only query API (instruction/xref/ordered-function).

The `s1_entry` seed helpers `executable_sections`/`existing_function_addrs`/
`in_executable_section` were promoted to `pub(crate)` so PR2 can build the seed set.

**Tests (both layers REAL, not skipped).**
- **Classifier unit tests** (`classify.rs`, 9 cases): hand-built `RawOp` lists
  (no Translate) pin `classify` against the `flow.rs` rules — CBRANCH (two
  successors, ConditionalBranch, is_conditional), unconditional BRANCH (target,
  no fall-through), CALL (target, is_call, falls through), RETURN (terminal),
  BRANCHIND (computed/indirect, no static target), CALLIND (computed, falls
  through), a **constant-space BRANCH in0** (intra-insn, NOT a target), and the
  last-op fall-through rule.
- **Real-decode integration test** (`kuna-console/tests/verify_listing_core.rs`,
  modeled on the `verify_*.rs` bootstrap pattern): `bootstrap_from_elf` over the
  vendored x86-64 `fauxware` fixture gives a built `Translate`
  (`prog.arch().translate()`) + code space (`prog.arch().manage()`); seeds =
  `collect_entries ∪ {main}`; `Listing::build` is driven end-to-end and the §7
  PR0 criteria asserted: `instruction_at(main)` has a plausible len + non-empty
  mnemonic; the `je`@0x4007bb yields two successors and `ConditionalBranch`; the
  `ret`@0x4007d4 is terminal (`fall_through == None`); the `call`@0x4007ae records
  a `RefKind::Call` edge to `authenticate`@0x400664 and seeds it as a function;
  `function_count() >= seeds`. **The test RAN (not skipped)** — `x86-64.sla` is
  built; it recovered **108 instructions, 11 functions, 16 call edges** from 2
  seeds, with `0x40071d: PUSH` (Fallthrough), `0x4007ae: CALL → 0x400664`,
  `0x4007bb: JZ` (ConditionalBranch, target 0x4007c9, fall 0x4007bd),
  `0x4007d4: RET` (Return).

**Result (the proof).** `make test` **675/675 PARITY OK**; `make test-stages`
**158/158 PARITY OK**; `make rust-test` green (incl. the 9 classifier unit tests
+ the new `verify_listing_core` console test). Structurally additive on a dormant
module (no engine invocation, no flag, no `AnalysisCtx` change), so the XML
datatest oracle is untouched — the keystone is not yet on any decompilation path.

- **New:** `docs/listing-tier-design.md` (the tier spec); `kuna-analysis/src/listing/`
  (`mod.rs`/`model.rs`/`decode.rs`/`classify.rs`/`walk.rs`) + `pub mod listing;`
  in `lib.rs`; `kuna-console/tests/verify_listing_core.rs` (+ `object` dev-dep).
- **Changed:** `s1_entry/mod.rs` — three seed helpers promoted to `pub(crate)`.
- **Behind the (forthcoming) `--option listing` flag** (PR1) and **not yet wired
  into the engine** (PR2).

### Increment 29 — Listing/xref tier PR1: the `--option listing` flag (default-off) ✅

PR1 of the Listing/xref tier (`docs/listing-tier-design.md` §5.1, §7 PR1): wire
the `--option listing` flag end-to-end, default-OFF, as pure plumbing. No Listing
is built yet (PR2 invokes the keystone from the engine); the flag is inert.

**What's wired (the `addrtable`/`formatstring` template, followed exactly).**
- `infra/architecture.rs`: the `analysis_listing: bool` field (in the `analysis_*`
  block after `analysis_formatstring`), `false` in the constructor, `self.analysis_listing
  = false;` in `reset_defaults_internal`, and the
  `"listing" => on_off!(analysis_listing, "Listing/xref disassembly tier")` arm in
  `set_kuna_option`.
- `p0_knowledge/options.rs`: `"listing"` added to `KUNA_OPTION_NAMES`.
- `kuna-console/src/engine.rs`: `"listing" => arch.analysis_listing,` in
  `analysis_pass_enabled` (explicit — the match fails OPEN with `_ => true`).
- `kuna-console/src/kuna_console.rs`: `"listing" => on_off(conf.analysis_listing),`
  in `kuna_live_value` (the console-side live `current` reader for the catalog),
  matching the analysis-pass-gate block.
- `kuna-decomp/stages.toml`: a `[[settable]]` analysis-enablement row
  (`option="listing"`, `values="on|off"`, `default="off"`,
  `change_kind="analysis-enablement"`, no `live_field`); the stale header count
  comment fixed (`settable=31` → `35`).
- `kuna-analysis/src/pass.rs`: `pub listing: Option<&'a Listing>` added to
  `AnalysisCtx` (imported from `crate::listing::Listing`); always `None` for now —
  both `AnalysisCtx { ... }` construction sites in `passes.rs` set `listing: None`.
- `kuna_stages/tests.rs`: `settable_count_is_34` → `settable_count_is_35` (35 =
  23 stage-model knobs + 12 analysis-tier gates); the `live_value` suppressed-set
  test (PASS_GATES + 11→12 comment) and the `emit_catalog_json` trailing-comma
  count (33 → 34) updated.
- `docs/assertions.md` regenerated via `kuna catalog --markdown` (the new
  `listing` row appended).

**Proof.**
- `kuna catalog --check` → **catalog OK** (the catalog documents exactly the 35
  registered kuna options; cross-checks against `KUNA_OPTION_NAMES` in-process).
  Settable count: **34 → 35**.
- The bumped `settable_count_is_35` unit test passes; all 22 `kuna_stages` tests
  green.
- New stages testcase `tests/stages/kuna-listing-flag.xml` (an infra test):
  decompiles the GH-558 fixture twice — once under the default pipeline, once with
  `option listing on` — and asserts the source-form C (`if (x <= 8)`) renders
  **identically** in both passes (default-off parity; the flag parses, round-trips,
  and is inert on the XML path). `docs/baseline-stages.json` regenerated
  (158 → 159 passing keys; only the new `KUNA-LISTING #1` key added).

**Result.** `make test` **675/675 PARITY OK**; `make test-stages` **159/159
PARITY OK**; `make rust-test` green; `kuna catalog --check` **OK**. Pure plumbing
+ default-off ⇒ the engine paths are byte-identical (the Listing is never built;
`ctx.listing` is always `None`), so the 675/675 datatest oracle and the real-ELF
bootstrap are untouched. The flag is inert until PR2 builds the Listing.

- **Changed:** `infra/architecture.rs`, `p0_knowledge/options.rs`,
  `p0_knowledge/kuna_stages/tests.rs`, `kuna-decomp/stages.toml`,
  `kuna-console/src/engine.rs`, `kuna-console/src/kuna_console.rs`,
  `kuna-analysis/src/pass.rs`, `kuna-analysis/src/passes.rs`, `docs/assertions.md`,
  `docs/baseline-stages.json`, `tests/stages/README.md`.
- **New:** `tests/stages/kuna-listing-flag.xml`.

### Increment 30 — Listing/xref tier read API (code-unit partition + function model + xref queries)

**What.** The read/query surface over the PR0-built `Listing` — design
`docs/listing-tier-design.md` §6 (consumer API) + §7 **PR3** (code-unit partition
+ ordered function model) **and PR4** (xref read API), combined into one increment
since both finalize read queries over the already-built model and would otherwise
self-conflict on `listing/mod.rs`. Purely additive read API: no engine path
touched, still no `--option` flag (PR1) and no engine invocation (PR2) — the
module stays dormant. The walk (PR0) already fills the `insns`/`refs_to`/
`refs_from`/`funcs` maps + `covered`/`exec_ranges`; this increment locks the
public read API on top.

**Code-unit partition (PR3).** `code_unit_at(vma)` is the partition the walk
leaves behind (the design's `partition_code_units`): the start-or-interior of a
decoded instruction is `CodeUnit::Instruction(start)` (ordered interior lookup,
`insns.range(..=vma).next_back()` + an `[addr, addr+len)` span test); any other
VMA inside an `executable_sections` range is `CodeUnit::Undefined`; everything
outside the exec ranges is conservatively `CodeUnit::Data` (the keystone has no
per-byte symbol-typed-data model, so "Data" = "not in an exec range" — the
conservative split A's "fell into data" / AIF's gap walk need). Plus
`is_data`/`is_undefined`/`num_instructions` and `first_undefined_after(vma)` (a
forward scan skipping each instruction's byte span — the AIF gap walk). `exec_ranges`
is now sorted at build for the forward scan.

**Ordered function model (PR3).** `function_containing(vma)` is the ordered
interval lookup `funcs.range(..=vma).next_back()` (nearest preceding function
*entry*, the faithful analog of Ghidra's ordered FunctionManager); `function_at`
(exact entry), `next_function_after(vma)` (`range(vma+1..).next()`), `function_count`.

**Xref read API (PR4).** `refs_to`/`refs_from` (slices over the bidirectional
multimap), `ref_source_iter()` (every distinct reference *source* VMA, ascending —
the call-site worklist), `has_refs_to`, `ref_count_to`. Ordering/dedup is **locked
at build**: each bucket is sorted (`refs_to` by source VMA then kind, `refs_from`
by target VMA then kind) and de-duplicated on the full `(from, to, kind)` triple,
so a target referenced twice from one site contributes one edge and `ref_count_to`
equals the number of distinct referencing sites. Signatures match §6 verbatim so
the future discovered-no-return consumer transliterates cleanly.

**Testcase.** New `kuna-console/tests/verify_listing_queries.rs` reuses the PR0
`fauxware` bootstrap/fixture, seeds `collect_entries ∪ {main, __libc_csu_init}`,
builds the `Listing`, and asserts §7 PR3 + PR4: **partition** — `code_unit_at(main
0x40071d)=Instruction`, an interior byte of the 3-byte `mov %rsp,%rbp`@0x40071e
maps to that insn, the non-exec image base 0x400000 is `Data`, and
`first_undefined_after(main)` (if Some) is genuinely Undefined / not an insn start;
**function model** — `function_containing(0x40073e)=main 0x40071d`,
`next_function_after(main)=__libc_csu_init 0x4007e0` (self-consistent with the
ordered function set), `function_count==functions().count()`; **xref** —
`refs_to(authenticate 0x400664)=[0x4007ae Call]`, `ref_source_iter()` includes the
call site and is strictly ascending, `ref_count_to(authenticate)` Call count == 1
(the sole call site), `refs_from(0x4007ae)` carries the outgoing Call, and the
terminal `ret`@0x4007d4 files no outgoing edge. **The test RAN (not skipped)** —
built `x86-64.sla`; it recovered **185 instructions, 16 functions, 169 ref
sources**, with `function_containing(0x40073e)=0x40071d`,
`next_function_after(0x40071d)=0x4007e0`, `refs_to(0x400664)=[(0x4007ae, Call)]`,
`ref_count_to(authenticate)=1`.

**Result (the proof).** `make test` **675/675 PARITY OK**; `make test-stages`
**158/158 PARITY OK**; `make rust-test` green (incl. the new `verify_listing_queries`
console test). Structurally additive read API on a dormant module (no engine
invocation, no flag, no `AnalysisCtx` change) — the XML datatest oracle is
untouched; the keystone is not yet on any decompilation path.

- **New:** `kuna-console/tests/verify_listing_queries.rs`.
- **Changed:** `kuna-analysis/src/listing/mod.rs` — the partition / function-model
  / xref read API + the build-time `finalize_refs` ordering/dedup; `exec_ranges`
  sorted at build.

### Increment 31 — Listing/xref tier PR2: engine-invoke (build the Listing at bootstrap, flag-gated)

PR2 of the Listing/xref tier (`docs/listing-tier-design.md` §1.3, §7 PR2): invoke
the keystone from the engine at bootstrap, flag-gated. PR0 built `Listing::build`
(unit-tested in isolation); PR1 wired the `--option listing` flag end-to-end with
`AnalysisCtx.listing: Option<&Listing>` always `None`. PR2 makes the build fire —
behind the default-OFF flag — and populates `ctx.listing`.

**What's wired.**
- `kuna-analysis/src/passes.rs`: threaded `translate: &dyn Translate` into both
  drivers — the live `run_default_analyses_per_pass` and (for signature symmetry)
  `run_default_analyses`. In each, before the pass loop:
  `let listing = arch.analysis_listing.then(|| Listing::build(&file, image, arch, translate, &seeds));`
  then `ctx.listing = listing.as_ref()` (replacing PR1's literal `listing: None`).
  The `Listing` is owned by the driver and outlives the pass loop (same lifetime
  shape as `file`), borrowed read-only via `ctx.listing`.
- The seed set is a new `pub fn listing_seeds(file)` =
  `existing_function_addrs(file) ∪ collect_entries(file)`, `in_executable_section`
  -filtered, sorted/deduped (design §3.1), built from the `pub(crate)` `s1_entry`
  helpers. Exposed `pub` so the cross-crate `verify_listing_*` gates can build the
  *exact* seed set the live driver uses.
- `kuna-console/src/engine.rs`: the driver call (`bootstrap_from_elf`) passes the
  engine's real decoder as the 4th arg — `sleigh.base().unwrap().translate()`
  (a `&Sleigh`, coerced to `&dyn Translate`; `Sleigh: Translate`). The `.sla` is
  loaded and the loadimage attached at this point, so a flag-gated build can decode
  through it. The decode reads bytes through the engine's attached loader (driven by
  `translate`), not through the `image` parameter (which stays unused, `_image`).

**Parity safety (the key proof).** `arch.analysis_listing` defaults `false`, so
`.then(...)` is `None` ⇒ no decode work, `ctx.listing == None`, byte-identical to
today. With the flag on the Listing is built but **no pass consumes it yet** (the
first consumer is PR6), so the decompiler output is unchanged. The build runs only
on the real-ELF bootstrap path (`run_default_analyses_per_pass`'s sole caller is
`bootstrap_from_elf`); the XML datatest path never reaches the analysis driver, so
the 675/675 and 159/159 oracles are structurally untouched.

**Build-at-load timing (honest note).** The analysis driver runs once inside
`bootstrap_from_elf` (`load file`), and the live CLI emits `option listing on`
*after* `load file` (the `--option` lines precede `read symbols`, not `load file`),
so the build-at-load does not fire through the live CLI today. Wiring the flag to
fire the build is PR6's concern (the first consumer). PR2's job is the wiring +
the parity proof + a direct exercise of the build-at-load body through the engine's
real `Translate`.

**Proof (specs built; tests ran, not skipped).**
- `verify_listing_core.rs` — new test `listing_build_through_engine_driver_seeds`:
  bootstraps `fauxware`, builds the driver's exact seed set via `listing_seeds`, and
  runs `Listing::build` through the engine's real `&dyn Translate`. Result:
  **21 seeds → 21 functions, 228 instructions**; `function_count() > 0` and
  `instruction_at(entry).is_some()` (and `instruction_at(main).is_some()`) all hold.
  This proves the engine-driven build path (vs PR0's throwaway-loadimage test).
- `verify_listing_parity.rs` — new test `listing_on_is_byte_identical_to_off`:
  decompiles `fauxware main → print C` with the flag off (default) and with
  `--option listing on`, and asserts the C is **byte-identical** (437 bytes,
  identical). The Listing is built behind the flag but consumed by no pass yet.

**Result.** `make test` **675/675 PARITY OK**; `make test-stages` **159/159 PARITY
OK**; `make rust-test` green; `kuna catalog --check` **OK**. The flag is default-off
so the parity oracles are structurally untouched; with the flag on the build runs
but changes no output (payoff comes with the first consumer, PR6).

- **Changed:** `kuna-analysis/src/passes.rs`, `kuna-console/src/engine.rs`,
  `kuna-console/tests/verify_listing_core.rs`.
- **New:** `kuna-console/tests/verify_listing_parity.rs`.

### Increment 32 — Listing/xref tier PR5: ARM/MIPS context-paint in the walker

The Listing's recursive-descent walker decodes through `Translate::one_instruction`
**outside** the decompiler's normal `commit_analysis_output` paint path, so an
alternate-ISA function (ARM Thumb / MIPS16) was misdecoded: a Thumb `compute` read
as A32, a MIPS16 `m16_square` read as MIPS32 — garbage. PR5 paints the correct
decode-mode context (`TMode` / `ISA_MODE`) into the engine's `ContextDatabase`
**before** the walk decodes, so alt-ISA functions decode correctly. Internal to
`Listing::build` (default-OFF behind `--option listing`), so it is parity-safe and
only active when the Listing is built. x86-64 (no decode-mode context) is untouched.

**How the per-address decode mode is obtained (reused, not re-derived).**
`listing/context.rs::ContextPainter::new(file)` **calls into the existing marker
logic** — `s1_loader::arm_markers::scan_arm_markers` (ARM `$t`/`$a` mapping symbols
+ STT_FUNC-LSB → `TMode`) and `s1_loader::mips_markers::scan_mips_isa_markers`
(STT_FUNC-LSB / `STO_MIPS_MIPS16` `st_other` → `ISA_MODE`) — and reuses their
`ContextPaint` facts verbatim. Both scans were promoted `fn` → `pub(crate)`; they
self-gate on the object architecture (ARM-only / MIPS-only), so on x86-64 the
painter collects **zero** paints and is a no-op.

**Mechanism (mirrors the commit seam exactly).** `ContextPainter::paint_all` applies
every collected paint via `Architecture::with_context_db_mut` →
`db.set_variable(var, addr, value)` / `set_variable_region(...)` — the *same* calls
`engine.rs::commit_analysis_output` step 6 makes. `set_variable` fills each mode
from its marker up to the next change point, so painting only the markers covers
every address the walk visits. The walk paints once at its start (before any
`decode_one`), then decodes against the already-painted context DB. Gate-safe: an
unregistered context variable (a faithful no-op for a language that does not define
it) is swallowed via the dropped `Result`, the same swallow the commit seam relies on.

**Wiring.** `walk::walk` takes `arch: &Architecture` + `painter: &ContextPainter`
and paints before the worklist loop; `Listing::build_with_meta` constructs the
painter from the `object::File` and threads both into the walk.

**Proof (specs built with `make specs`; the gate RAN, not skipped).**
`kuna-console/tests/verify_listing_context.rs` bootstraps each fixture for a real
`Translate`, builds the Listing seeded at the alt-ISA function's entry, and asserts
the seed decodes in the alternate ISA — with a **control**: the same seed decoded
through a fresh bootstrap's raw `Translate` (no Listing, no paint = the default
mode), asserting the painted decode DIFFERS.
- **ARM Thumb** (`arm_thumb_linked_le32`, `compute` @ 0x100b8): the Listing decodes
  `push` (len **2**, Thumb); the un-painted A32 control reads `addlt` (len **4**) —
  a different, garbage instruction. The body carries the `*3` shift (`lsl`).
- **MIPS16** (`mips16_le32`, `m16_square` @ 0x400130): the Listing decodes `mult`
  (len **2**, MIPS16) → `mflo` (2) → `jr`+delay-slot (4, the §4.3-gotcha-3 fold) =
  the 8-byte body; the un-painted MIPS32 control **fails to decode the bytes at all**
  ("Unable to resolve constructor") — the strongest control: they simply are not
  MIPS32.

**Result.** `make test` **675/675 PARITY OK**; `make test-stages` **159/159 PARITY
OK**; `make rust-test` green (incl. the 2 new `verify_listing_context` cases). The
paint is internal to the default-OFF Listing build, so the parity oracles are
structurally untouched (no new `--option`, catalog unchanged).

- **Changed:** `kuna-analysis/src/listing/mod.rs`, `kuna-analysis/src/listing/walk.rs`,
  `kuna-analysis/src/s1_loader/arm_markers.rs` (`scan_arm_markers` → `pub(crate)`),
  `kuna-analysis/src/s1_loader/mips_markers.rs` (`scan_mips_isa_markers` → `pub(crate)`).
- **New:** `kuna-analysis/src/listing/context.rs`,
  `kuna-console/tests/verify_listing_context.rs`.

### Increment 33 — Listing/xref tier PR6: discovered-no-return consumer (the first Listing consumer) ✅

The payoff. The first analyzer built on the Listing/xref model — the kuna analog of
Ghidra's `FindNoReturnFunctionsAnalyzer` ("Non-Returning Functions — Discovered").
It finds **custom** no-return wrappers the static name-lists cannot catch (a local
`die()` that tail-calls `exit`), by **flow evidence** over the program-wide Listing,
and marks them no-return through the EXISTING `NoReturnFact` commit seam. Two parts:
a build-timing fix (PART A) + the consumer (PART B).

**PART A — the build-timing fix (the mechanism).** PR2 flagged it: the live CLI emits
`option listing on` *after* `load file`, but the analysis driver
(`run_default_analyses_per_pass`) runs *at load*, so a Listing gated on
`arch.analysis_listing` never builds through the CLI (the flag is still default-off
at load). The fix mirrors the **Increment 13** deferred-commit precedent. At load,
`bootstrap_from_elf` now also **stashes the image bytes + path** on `ConsoleProgram`
(`analysis_image`); the *build* of the Listing — and the run of any Listing-consumer
pass — is **deferred to `commit_pending_analysis`** (reached at `read symbols`, AFTER
the CLI's `option` lines). There, gated on `arch.analysis_listing` (now in effect),
the stashed bytes are re-parsed, the Listing is built with funcsym names +
Known-no-return seed metadata (`Listing::with_noreturn_seeds`), the new
`passes::run_listing_consumers` runs the consumer over it, and its (per-pass gated)
facts merge into the same `AnalysisOutput` the deferred commit already commits. The
consumer's facts thus flow through the **unchanged** `commit_analysis_output`
no-return arm (`find_function_across_scopes`/`query_global_function` →
`set_function_no_return`); flow-repair / dead-code elimination is INHERITED from the
engine. Default (listing off) ⇒ the whole deferred block is skipped ⇒ byte-identical
to today; the XML path stashes no image ⇒ structural no-op.

**PART B — the consumer (`s1_noreturn_disc`, id `noreturn_disc`, default-OFF).** A
new gated `AnalysisPass` whose `run` short-circuits to empty when
`ctx.listing.is_none()`. The evidence-tally fixpoint over the Listing (design §6 +
Ghidra):
- **Rule 1 (call-site evidence):** a callee is no-return when **≥3** of its call
  sites (`refs_to(callee)` Call edges) show *no valid fall-through* — the byte after
  `call` is not a decoded instruction start (`!is_instruction_start`), or is data
  (`is_data`), or is *another* function's entry (`function_at(fall)`), or the call
  itself has no fall-through (a lowered tail jump). (Ghidra's evidence threshold = 3.)
- **Rule 2 (fixpoint promotion):** a function whose every terminal path is a call to
  an already-discovered-no-return function, *with no returning path* (no `RETURN`
  instruction in its span), is itself no-return — so a wrapper-of-a-wrapper converges.
- Skips callees already modeled no-return (`function_at(t).has_no_return`, seeded from
  the Known pass) or call-fixup'd. Emits `NoReturnFact { addr, name }` (NO new commit
  arm).

**The END-TO-END (the whole point — real, not faked).** Fixture
`noreturn_disc_x86_64` (`+.c`), `gcc -O1 -no-pie -fno-pic`: a `static void
die(const char*)` that `fprintf`s then `exit(1)`, NOT marked
`__attribute__((noreturn))`, called from **four** functions (`compute_a/b/c/d`). At
`-O1` gcc emits nothing after each `call die` — the byte after the call is the next
function's entry — so all four call sites show no-valid-fall-through (≥3). `compute_a`,
decompiled:

*BEFORE (default, flags off)* — `die` is treated as returning, so the decompiler
follows the fall-through past `call die` through `compute_b`/`compute_c`/`compute_d`
into `main` (massive dead code merged into one body):
```c
undefined16 compute_a(int4 a0,void *a1,unsigned long a2)
{ ... if (0 <= a0) { ...; return v3; }
  v13 = 0x40200f;
  die();                       /* treated as returning */
  if (v13 != 0) { ... return v9; }
  v14 = "b: zero"; die("b: zero");
  ...                          /* compute_c, compute_d, main bodies all inlined */
  v5 = compute_a(); ... __printf_chk(1,"%d %d %d %d\n",...); return v2 << 0x40; }
```

*AFTER (`--option listing on --option noreturn_disc on`)* — `die` is concluded
no-return; the engine's inherited repair eliminates the dead code:
```c
int4 compute_a(int4 a0)
{ if (0 <= a0) { return a0 * 2; }
                    /* WARNING: Subroutine does not return */
  die("a: negative"); }
```

**Gating invariants (the parity proof).** Verified directly: `noreturn_disc on` +
`listing off` ⇒ identical to default (no Listing to read); `listing on` +
`noreturn_disc off` ⇒ identical to default (Listing built, consumer gated off). Only
BOTH flags on changes output. So the default path is byte-identical to today.

**Result (specs built; tests ran, not skipped).** New e2e
`verify_noreturn_disc.rs` (3/3): the before/after assertion (output differs,
flags-on carries the no-return terminator, default does not), the flags-off baseline,
and the `die`-tail-calls-`exit` sanity. `make test` **675/675 PARITY OK**;
`make test-stages` **159/159 PARITY OK**; `make rust-test` green; `kuna catalog
--check` **catalog OK**. settable_count **35 → 36** (the `noreturn_disc` gate);
`stage_catalog.json` fixture + `docs/assertions.md` regenerated; the count-fixture
ripple (kuna_stages `settable_count_is_36`, `…suppressed_for_14`,
`…brackets_and_commas` 35 commas, `catalog_bytecompat` 36) updated.

- **Divergence/LOSS:** none to the parity oracles — default behavior unchanged
  (default-OFF, real-ELF-path-only, gated by BOTH `listing` and `noreturn_disc`). The
  consumer is a flow heuristic, hence default-off behind its own flag (faithful to
  Ghidra's `FindNoReturnFunctionsAnalyzer.setDefaultEnablement(false)`). Computed/
  indirect calls contribute no evidence (no static callee); jump-table resolution
  stays deferred (design §8).
- **New:** `kuna-analysis/src/s1_noreturn_disc/mod.rs`,
  `kuna-console/tests/verify_noreturn_disc.rs`, the `noreturn_disc_x86_64` fixture
  (`+.c`).
- **Changed:** `kuna-analysis/src/{lib.rs, passes.rs, listing/mod.rs}`,
  `kuna-console/src/engine.rs`, `kuna-decomp/src/infra/architecture.rs`,
  `kuna-decomp/src/p0_knowledge/options.rs`, `kuna-decomp/stages.toml`,
  `kuna-decomp/src/p0_knowledge/kuna_stages/tests.rs`,
  `kuna-decomp/tests/{catalog_bytecompat.rs, fixtures/stage_catalog.json}`,
  `docs/assertions.md`.

### Increment 35 — Multi-format loader PR-1: the `ObjectFormat` seam (ELF lifted verbatim)

**Premise.** kuna is ELF-only; the goal is "a decompiler for most file formats"
(PE/Mach-O/COFF). The canonical spec for the whole expansion is now
`docs/multiformat-loader-design.md` (the per-format PR plan, §0–§9, every file:line
re-verified against the live tree). This is **PR-1**: a PURE REFACTOR that lifts
today's ELF-only load logic behind a trait with ZERO behavioral change — the
foundation every later format-PR plugs into.

**The seam.** New `kuna-analysis/src/s1_loader/format/` — `mod.rs` (the
`ObjectFormat` trait, `FormatKind`, `ImportSym`, `detect()`, and a free
`resolve_imports(file, bytes)` dispatch fn) and `elf.rs` (`ElfFormat`). `ElfFormat`
is **today's logic moved verbatim**: `section_bits` = the old `section_kind_flags`
body; `resolve_imports` calls the unchanged `elf_plt::resolve_plt_imports`;
`compiler_model` returns the same `gcc`/`default` tokens; `const_ranges` =
`mips_got_const_ranges`. Format knowledge lives in the impl, not sprinkled through
shared passes.

**The three substituted points in `ObjectLoadImage::from_bytes`** (everything else
verbatim): (B) the non-ELF reject → `let fmt = format::detect(&file)?;`; (C) the
section snapshot → `fmt.section_bits(...)`; (F) the PLT call → `fmt.resolve_imports(
&file, bytes)` (plus the MIPS const-ranges → `fmt.const_ranges(...)`). (D) `language_id_for`
now threads the `compiler_model` from the seam — but the ELF id strings are
**byte-identical** (ELF→`gcc`/`default`), so it's a structural change with no output
change. The arch→stem match and `strip_version`/MIPS arms are unchanged. The id-string
shape is now composed by a shared `compose_language_id`, reused by `language_id_for`
and the new `elf_language_ids()` enumerator so the resolve-in-DB gate cannot drift.

**The two other call sites** — `s1_entry::existing_function_addrs` and
`s1_loader::noreturn::scan_noreturn` — now call the free `format::resolve_imports(
file, bytes)` (no format branch; just the dispatch fn). `bytes` is threaded via a new
`AnalysisCtx.bytes` field (all three ctx-construction sites already owned `bytes`) and
a `bytes` param on `existing_function_addrs`/`collect_entries`/`listing_seeds`; the ELF
resolver ignores it (it reads sections off `file`), but the seam carries it for the
PE/Mach-O typed-reparse PR-2 needs.

**ELF-only-reachable.** `detect()` rejects everything but ELF (PE/Mach-O/COFF impls
and the `object` pe/macho/coff features are PR-2), and the engine dispatch still routes
only `\x7fELF` to the object loader — so the rejected arm is unreachable on the live
path and nothing PE/Mach-O is constructible from a real binary yet.

**Faithfulness proof (the whole point).** `make test` **675/675 PARITY OK**;
`make test-stages` **159/159 PARITY OK**; `make rust-test` green — all UNCHANGED. Every
existing ELF fixture (`fauxware`, `cet_pie_x86_64`, `stripped_dynamic_x86_64`,
`plt_mips32`, `cpp_mangled_x86_64`, the cross-arch entry/plt fixtures, …) now runs
through the trait path and is byte-identical. New tests: a kuna-analysis unit
(`ElfFormat::compiler_model`/`section_bits` reproduce the old tokens/bits exactly) and
a console gate `verify_elf_language_ids.rs` (every id `language_id_for` can produce —
x86-64/x86-32/ARM/AArch64/MIPS/RISCV/SPARC/PPC, both endiannesses where the vendored
`.ldefs` declare a stem — resolves in `scan_language_database`).

- **Divergence/LOSS:** none to the parity oracles — the XML datatest path never reaches
  the object loader, and the ELF arm is the old code lifted verbatim. The only textual
  change is the non-ELF reject message (`detect()`'s "unsupported object format" vs the
  old "not an ELF object"), and that arm is unreachable on the live path (the engine
  never hands a non-ELF to the object loader), so no output changes.
- **One chokepoint that didn't lift as cleanly as the plan expected:** §1.3 framed the
  free `resolve_imports(file, bytes)` as a drop-in at the two non-`from_bytes` call
  sites, but `bytes` was not in scope at `existing_function_addrs`/`scan_noreturn`
  (their callers carry `file`-only). Threading it required an `AnalysisCtx.bytes` field
  + a `bytes` param down `existing_function_addrs`/`collect_entries`/`listing_seeds`
  (and the cross-crate `verify_listing_*` callers). Mechanical, faithful (ELF ignores
  `bytes`), and exactly the "thread bytes as needed" the plan called for — just a wider
  ripple than the "three substituted lines" framing suggested.
- **New:** `kuna-analysis/src/s1_loader/format/{mod,elf}.rs`,
  `kuna-console/tests/verify_elf_language_ids.rs`, `docs/multiformat-loader-design.md`.
- **Changed:** `kuna-analysis/src/{loadimage_object.rs, pass.rs, passes.rs,
  s1_loader/mod.rs, s1_loader/noreturn.rs, s1_entry/mod.rs}`,
  `kuna-console/tests/{verify_listing_core.rs, verify_listing_queries.rs}`.

### Increment 36 — Multi-format loader PR-2: object PE/Mach-O/COFF features + format dispatch (behind --experimental-formats)

**Premise.** PR-1 (Increment 35) lifted the ELF-only load logic behind the
`ObjectFormat` seam with ELF the only constructible impl. This is **PR-2**: turn
on `object`'s PE/Mach-O/COFF readers, add the three sibling format skeletons, and
wire the engine dispatch to route their magics to the object loader — all
**behind `--experimental-formats`** so the default path stays ELF-only and
byte-identical. The `object` parse features were the hard blocker (chokepoint A,
design §1.1); flipping them on + the dispatch is the scaffolding every later
format-PR plugs into.

**`object` features (chokepoint A).** `decompiler/Cargo.toml`: the `object` crate
features `["read_core","elf","std"]` → `+["pe","macho","coff"]`, so
`object::File::parse` physically recognizes PE/Mach-O/COFF. **`Cargo.lock`
unchanged** — the new features pull no new crate versions or transitive deps (the
PE/Mach-O/COFF readers share `object`'s existing deps), the offline-gate-safe
outcome. `cargo build --offline` confirms zero downloads.

**The three skeletons** (`s1_loader/format/{pe,macho,coff}.rs`). Each implements
the four `ObjectFormat` methods: `kind()`; `compiler_model()` per §2 — `PeFormat`
→ `windows`, `MachOFormat` → `gcc` (x86-64 is System V AMD64, the *same* cspec
Ghidra labels `gcc` — NOT an invented token) / `default` (arm64), `CoffFormat`
→ `windows` (COFF objects are overwhelmingly MSVC); `section_bits()` — PE/COFF
share a `coff_section_bits` helper keyed on the COFF `Characteristics`
(`IMAGE_SCN_MEM_EXECUTE`/`_WRITE`/`CNT_UNINITIALIZED_DATA`/`CNT_CODE`), Mach-O
keys on the neutral `SectionKind` + the section-type/`S_ATTR_PURE_INSTRUCTIONS`
bits (Mach-O carries no per-section RWX — that lives in the segment initprot);
`resolve_imports()` returns **empty** (real IAT/`__stubs` naming is PR-4/PR-7).
`detect()` now constructs all four (the PR-1 todo/error arms removed).

**Dispatch (chokepoint E).** `engine.rs`: `bootstrap_from_elf` → renamed
`bootstrap_from_object` (it was already 95% format-neutral; all callers + doc
references updated, incl. the cross-crate `verify_*` test imports). The bare
`ELF_MAGIC` test → `is_object_binary(&bytes)` per §1.4 (ELF + the four Mach-O
byte-orders + fat `0xcafebabe` + `MZ` for PE + a known-`IMAGE_FILE_MACHINE_*`
prefix for bare COFF). **By default `is_object_binary` admits ONLY ELF** — the
non-ELF magics are gated on `experimental_formats_enabled()` (the
`KUNA_EXPERIMENTAL_FORMATS` env var). So the default `load file` dispatch is
byte-identical: every non-ELF input still routes to the XML branch exactly as
before, and the XML/`bootstrap_program` branch (Invariant 1) is untouched.

**The flag.** `--experimental-formats` is exposed two ways for one mechanism: the
`KUNA_EXPERIMENTAL_FORMATS` env var (read live in `bootstrap_from_file`, so the
in-process e2e and the `decomp_dbg`/`decomp_test_dbg` binaries all honor it), and
a `kuna decompile --experimental-formats` CLI flag that exports it onto the spawned
`decomp_dbg` subprocess. The env var is the clean seam across the subprocess
boundary the CLI driver uses.

**Compiler-model fallback (§2.2).** If a format's chosen model id isn't vendored
for an arch (e.g. a hypothetical `ARM:...:windows` with no `ARM_win.cspec`), the
loader composes a *fallback* id (the same stem with the model dropped to the
per-arch default), exposed via `ObjectLoadImage::fallback_arch_id()`;
`bootstrap_from_object` retries with it when the primary doesn't resolve and no
explicit `--target` was given — wrong calling-convention details beat no
decompile. ELF carries no fallback (its primary already uses the default model),
so the established path is unaffected. (For the four headline arches every
PE/Mach-O id resolves *directly* — the fallback is the safety net, not the path.)

**Fixtures (built in-container today, no new packages).** `pe_min.obj` (`clang
-target x86_64-pc-windows-gnu -c` → an Intel amd64 COFF object, `format=Coff`) and
`macho_min.o` (`clang -target x86_64-apple-macos11 -c` → a relocatable Mach-O,
`format=MachO`), both in `kuna-analysis/tests/fixtures/`. Section flags + function
offsets pinned from the `object`-crate view (`the_answer`@0x0, `helper`@0x60 PE /
`_helper`@0x50 Mach-O).

**Tests (both ran, not skipped — the `x86` `.sla` is built).**
- **Format unit tests** (in `pe.rs`/`macho.rs`/`coff.rs`): each `compiler_model`
  returns the right token per arch; each `section_bits` maps an exec section to
  CODE|READONLY and a writable section to not-READONLY (8 tests).
- **Language-id resolves gate** (`verify_elf_language_ids.rs`, extended): every id
  the PE/Mach-O/COFF loaders produce resolves in `scan_language_database` (or its
  §2.2 fallback does), and the headline ids resolve *directly* — PE/COFF →
  `x86:LE:64:default:windows`, Mach-O → `x86:LE:64:default:gcc` /
  `AARCH64:LE:64:v8A:default`. New producers `pe_language_ids`/`macho_language_ids`
  /`coff_language_ids` derived from the same `compose_language_id` the loader uses.
- **e2e** (`kuna-console/tests/verify_object_formats.rs`, 2 tests): with the flag
  on, each fixture (1) parses (no "not an ELF object"), (2) maps sections —
  `section_snapshot()` shows a CODE|READONLY `.text`/`__text`, (3) selects the
  right spec (PE→windows, Mach-O→`gcc`, asserted on the exact id), (4) disassembles
  real x86-64 mnemonics out of `.text`/`__text`. Imports empty (PR-4/PR-7). Each
  test also proves the **default-off** path: with the env var unset the same
  fixture is rejected (routed to the XML branch), confirming the gating.

**Result.** `make test` **675/675 PARITY OK**; `make test-stages` **159/159 PARITY
OK**; `make rust-test` green (incl. the 8 unit + 2 language-id + 2 e2e tests). The
default `kuna decompile` on an ELF (`fauxware main`) is **byte-identical** with and
without `--experimental-formats` — the flag is a pure no-op on ELF (ELF is always
admitted).

- **Divergence/LOSS:** none to the parity oracles — the XML datatest path never
  reaches the object loader (Invariant 1, untouched), and the non-ELF magics are
  flag-gated off by default, so default dispatch is byte-identical. The PR-1 reject
  message and the new PE/Mach-O/COFF arms are reachable only under
  `--experimental-formats`.
- **New:** `kuna-analysis/src/s1_loader/format/{pe,macho,coff}.rs`,
  `kuna-console/tests/verify_object_formats.rs`,
  `kuna-analysis/tests/fixtures/{pe_min.obj, macho_min.o}`.
- **Changed:** `decompiler/Cargo.toml`,
  `kuna-analysis/src/{loadimage_object.rs, s1_loader/format/mod.rs}`,
  `kuna-console/src/engine.rs`, `kuna-cli/src/{main.rs, decompile.rs}`,
  `kuna-console/tests/verify_elf_language_ids.rs`, the `bootstrap_from_elf`→
  `bootstrap_from_object` rename across `kuna-console/{src/ifacedecomp.rs,
  tests/verify_*.rs}` + `kuna-analysis/src/{pass.rs, listing/decode.rs}` +
  `kuna-analysis/tests/fixtures/README.md`, `docs/analysis-port-log.md`.

### Increment 37 — Multi-format loader PR-3+4: PE linked-exe loading + IAT import naming (the PE headline)

**Premise.** PR-2 (Increment 36) proved PE/Mach-O *objects* parse + map + select
the right spec behind `--experimental-formats`, with imports empty. This is the
**PE headline** (PR-3 ∪ PR-4): a **linked Windows PE32+** exe loads and decompiles
a function, and its libc imports render **named** (`puts`/`printf`) — the first
non-ELF binary kuna names imports on. PR-3 (loader proof) needed no new loadimage
fix — `object` surfaces the PE ImageBase+RVA transparently, so the linked exe
loads through the same `bootstrap_from_object` path the PE object did; PR-4 (IAT
naming) is the real new logic, in a new `s1_loader/pe_iat.rs`.

**The IAT walk (`s1_loader/pe_iat.rs`, design §3.2).** `PeFormat::resolve_imports`
(was empty) now calls `resolve_pe_imports(file, bytes)`. It branches on
`FileKind::Pe32`/`Pe64` to re-parse with the typed `PeFile32`/`PeFile64` (the
neutral `object::File::imports()` gives name+library but **no slot address** — the
typed parser is required, design §3.2 risk row), then walks the Import Directory
pairing the INT (`original_first_thunk`, the names) and IAT (`first_thunk`, the
slot table) **in lockstep**: the `i`-th name belongs to the slot at
`image_base + first_thunk_rva + i*ptr` (`ptr`=8 for PE32+, 4 for PE32). Each slot
→ `ImportSym { addr: slot_va, name }` — exactly the ELF GOT-slot analog, so the
engine constant-folds a `call [slot]` to the name (arch-independent, no per-arch
stub decode). `Import::Ordinal(n)` synthesizes `<DLL-stem>_Ordinal_<n>` (§3.5).
`file.exports()` are registered additively as funcsyms. Pure/total/never-error:
non-PE / no-import-dir / unparsable → empty `Vec`.

**The MinGW thunk wrinkle (the load-bearing PR-4 piece).** MinGW routes a *direct*
`call thunk` to a one-instruction veneer `jmp [rip+disp]` (`FF 25 <disp32>`) that
jumps through the IAT slot — so naming only the slot leaves `call thunk` rendering
`sub_<thunk>`. `pe_iat` therefore *also* decodes the `FF 25` veneers over the
executable sections and names the **thunk entry** with its target slot's import
name. The decode is the same `FF 25` recovery as the ELF x86-64 PLT
(`elf_plt::decode_x86_64`): x64 RIP-relative (`slot = next_insn + disp32`) / x86
absolute (`slot = disp32`), self-correcting (a veneer whose target isn't a known
import is left alone). x86-only (the opcode is x86); other arches no-op the thunk
pass (the IAT-slot naming still covers their indirect calls).

**Fixtures (MinGW, in-container — `x86_64-w64-mingw32-gcc` shipped by `kuna-dev`).**
`pe_imports.c` = a `main` that calls `puts("hello")` + `printf("%d\n", argc)`.
- `pe_imports.exe` (non-stripped, `-O1`) — the headline. ImageBase `0x140000000`,
  `main`@`0x140001592`, the `puts` thunk@`0x140007240` (`FF 25` → `__imp_puts`
  IAT slot@`0x14000d33c`), the local MinGW `printf` wrapper@`0x140001550`.
- `pe_imports_stripped.exe` (`-s`) — the PR-4 proof: symbols gone, so the call to
  the `puts` thunk is named **only** by the IAT walk + thunk decode.

**Before → after** (the headline; `kuna decompile … --experimental-formats`):
On the **stripped** PE (the clean PR-4 delta — `main` at `0x140001592 --addr`):
```c
// BEFORE (resolve_imports empty)        // AFTER (PR-4 IAT/thunk naming)
sub_140007240(0x140009000);              puts(0x140009000);
sub_140001550(0x140009006,a0);           sub_140001550(0x140009006,a0);  // local printf wrapper, not an import — correctly unnamed
```
On the **non-stripped** `pe_imports.exe` (`main` by name) the call renders
`puts(0x140009000)` / `printf(0x140009006,a0)` — here the COFF symtab already
names the thunk/wrapper, and PR-4 is additive (it covers the IAT-slot path a
`call [slot]` form would take). `main` calls `puts` **via the `0x140007240` thunk**
(a direct `call thunk`), named by the `FF 25` veneer decode → `__imp_puts` slot
→ `puts`; `printf` is a *local* MinGW wrapper (named from the symtab, not an
import).

**Decompile speed.** `pe_imports.exe main`: **~0.12–0.25 s** wall (`time kuna
decompile`, end-to-end incl. the `decomp_dbg` subprocess spawn); the stripped
`main` by addr is the same range. On par with the ELF baseline (`fauxware main`,
~0.15 s on the same box) — both are dominated by process startup + the tiny
function's decompile, and the timings overlap inside the measurement noise. PE
loading is **not** materially slower than ELF: the IAT walk is a single
Import-Directory scan + one `FF 25` sweep over `.text` (O(imports + text bytes),
once at load).

**Tests (the e2e RAN, not skipped — the `x86` `.sla` is built).**
- **`pe_iat` unit tests** (synthetic, mirroring `elf_plt.rs`'s decoder tests, 4):
  the `FF 25` thunk decode both widths (x64 RIP-relative + x86 absolute), an
  unknown-slot veneer left alone, and the ordinal-name synthesis
  (`ws2_32.dll`+115 → `ws2_32_Ordinal_115`).
- **e2e** (`kuna-console/tests/verify_pe_imports.rs`, 2): (1) the non-stripped
  linked exe loads (Windows x86-64 spec), resolves `main`/`puts`, and decompiles
  `main` to `puts(`/`printf(`; (2) the stripped exe names the `puts` thunk via the
  IAT walk (renders `puts(`, not `sub_140007240`) while leaving the local `printf`
  wrapper `sub_<addr>`. Test 1 also proves the **default-off** path (PE rejected
  without the flag). A `Mutex` serializes the two tests' env-var-sensitive bodies
  (`KUNA_EXPERIMENTAL_FORMATS` is process-global).

**Result.** `make test` **675/675 PARITY OK**; `make test-stages` **159/159 PARITY
OK**; `make rust-test` green (incl. the 4 unit + 2 e2e). Default-off ⇒ ELF
byte-identical (the PE path is unreachable without `--experimental-formats`; the
ELF/XML oracles never reach the object loader — Invariant 1). **After this, PR-5
(COFF) and the on-by-default flip for PE are the remaining PE-arm work.**

- **Divergence/LOSS:** none to the parity oracles (the non-ELF path is
  flag-gated; the XML datatest path never reaches the object loader).
- **New:** `kuna-analysis/src/s1_loader/pe_iat.rs`,
  `kuna-console/tests/verify_pe_imports.rs`,
  `kuna-analysis/tests/fixtures/{pe_imports.exe, pe_imports_stripped.exe}` (+ the
  `pe_imports.c` source recorded in the fixtures README).
- **Changed:** `kuna-analysis/src/s1_loader/{mod.rs, format/pe.rs}`,
  `kuna-analysis/tests/fixtures/README.md`, `docs/analysis-port-log.md`.

### Increment 38 — Multi-format loader PR-5: COFF object support

**Premise.** PR-3+4 (Increment 37) delivered the PE *image* headline (a linked PE32+
loads + names IAT imports). PR-5 closes the PE arm with the **object** case: a
pre-link **COFF object** (`.obj`) loads under `--experimental-formats` and
decompiles a function **resolved by its COFF-symbol-table name**. A COFF object has
**no IAT and no resolved imports** — externals are unresolved *symbols*, not
addresses — so the value here is the COFF symtab (defined function names) + the
format-agnostic passes (strings / demangle / no-return-name / protos) riding a COFF
object, **not** import naming.

**Object vs image (design §3.6, documented in the `CoffFormat` module doc).**
`object` reports two things as `BinaryFormat::Coff`: a pre-link **object**
(`.obj`/`.o`, the common case and what `CoffFormat` is for) and, rarely, a
COFF-flavored **image**. A *normal* linked PE reports as `BinaryFormat::Pe` and
routes through the PE arm (`PeFormat`, the IAT-naming path) — never reaching
`CoffFormat`. `CoffFormat::resolve_imports` returns **empty** by design (the PR-2
skeleton, now finalized): a pre-link object has nothing to resolve imports *to*, so
the undefined external (a `puts` symbol) is simply absent from the funcsym set.
Section flags reuse the PE arm (`pe::coff_section_bits` — PE and COFF share the
COFF `Characteristics` model); `compiler_model` → `windows` (COFF objects are
MSVC-flavored), with the `compose_language_id` fallback to `gcc`/`default` if an
arch lacks a `windows` id.

**The one real code change — the defined-function-at-VMA-0 fix.** A relocatable
`.obj` places its first defined function at section-relative VMA 0
(`compute @ .text+0`). The defined-funcsym source in `loadimage_object.rs` (source
#1) used to skip import placeholders on `addr == 0` (the ELF UND convention,
`st_value == 0`) — which would **silently drop** `compute`. The skip now keys off
`Symbol::is_undefined()` instead: the format-faithful predicate that still drops the
undefined `puts` external while keeping a legitimately-defined COFF function at VMA
0. **Byte-identical on every ELF** — verified against the `object`-crate view of
`fauxware` (all UND import syms `puts@@GLIBC`/`printf@@GLIBC`/… are exactly the
`is_undefined()` ones, every defined function is `is_undefined()==false`) and the
relocatable-ELF object `arm_thumb_le32.o` (first function at `0x1`, not 0). Applied
to the `.dynsym` source (#3) too for consistency (no behavioral effect — COFF has no
dynamic symbols, and ELF dynsym UND entries are `is_undefined()`). The 13
`loadimage_object` ELF funcsym units pass unchanged.

**Fixture (`coff_obj.obj`, Intel amd64 COFF, <1 KB — `clang` in `kuna-dev`, no new
packages).** `clang -target x86_64-pc-windows-gnu -O1 -c coff_obj.c`. `coff_obj.c`
= `int compute(int x){ return x*3+1; }` / `int run(int n){ const char *s="hi";
puts(s); return compute(n)+(int)s[0]; }`. Confirmed via container
`x86_64-w64-mingw32-objdump -t`/`-d`: COFF symtab carries `compute`@`.text`+0x0
(`lea (rcx,rcx,2),eax; add 1` = `x*3+1`, Windows x64 first arg in rcx), `run`@+0x10,
`puts` an **undefined** external (section 0, an `IMAGE_REL_AMD64_REL32` `puts`
reloc on the `call`), and the `"hi"` literal in `.rdata` (size 3, the string pass's
input).

**Before → after** (`kuna decompile coff_obj.obj compute --experimental-formats`):
```c
// BEFORE (PR-2/PR-5 absent)              // AFTER (this PR)
.obj → "not an ELF object" (rejected)     int4 compute(int4 a0) { return a0 * 3 + 1; }
```
The `.obj` was rejected (without the flag it still routes to the XML branch, which
cannot parse it — the default-off proof). After PR-5 it loads with the Windows
x86-64 spec, and `compute`/`run` resolve **by COFF-symtab name** and decompile
(`run` → `a0 * 3 + 0x69`; the `puts(s)` external call has no resolved address/proto
so the unresolved-side const-folds, faithful for a pre-link object).

**Decompile speed.** `coff_obj.obj compute` (by name): **~0.15–0.16 s** wall
(`/usr/bin/time kuna decompile`, end-to-end incl. the `decomp_dbg` subprocess spawn,
3-run median 0.16 s, 51 MB RSS). On par with the ELF baseline (`fauxware main`,
~0.15 s) and the PE headline (Increment 37, ~0.12–0.25 s) — dominated by process
startup + spec load + the tiny function's decompile; COFF-object load is a single
section/symbol snapshot (no IAT walk), so it adds nothing over ELF.

**Tests (the e2e RAN, not skipped — the `x86` `.sla` is built).**
- **e2e** (`kuna-console/tests/verify_coff_object.rs`, 2 tests): (1) the COFF object
  loads (Windows x86-64 spec), `compute`/`run` resolve as funcsyms, the **undefined
  `puts` does NOT** (no IAT, §3.6), and `load function compute` → `print C`
  decompiles `compute` named to `* 3 + 1`; (1) also proves the **default-off** path
  (the `.obj` rejected without `--experimental-formats`). (2) the second defined
  function `run` also resolves by name + decompiles (the §4 symbol-source table
  carries *every* defined function, not just the first). A `Mutex` serializes the
  two tests' `KUNA_EXPERIMENTAL_FORMATS`-sensitive bodies (process-global env).
- The existing `verify_object_formats.rs` (PR-2) already covers `pe_min.obj`
  parse/map/disassemble; this gate adds the **named-function-by-symtab** half PR-5
  is about.

**Result.** `make test` **675/675 PARITY OK**; `make test-stages` **159/159 PARITY
OK**; `make rust-test` green (incl. the new 2-test e2e). Default-off ⇒ ELF
byte-identical (the COFF path is unreachable without `--experimental-formats`; the
XML datatest path never reaches the object loader — Invariant 1; the
`is_undefined()` funcsym change is byte-identical on every ELF). **This completes
the PE arm of the multi-format loader (PE image + PE/COFF object); Mach-O is the
remaining format.**

- **Divergence/LOSS:** none to the parity oracles (the non-ELF path is flag-gated;
  the XML datatest path never reaches the object loader; the funcsym predicate
  change is ELF-byte-identical).
- **New:** `kuna-console/tests/verify_coff_object.rs`,
  `kuna-analysis/tests/fixtures/coff_obj.obj` (+ the `coff_obj.c` source recorded in
  the fixtures README).
- **Changed:** `kuna-analysis/src/loadimage_object.rs` (the `is_undefined()`
  funcsym skip), `kuna-analysis/src/s1_loader/format/coff.rs` (module doc finalized),
  `kuna-analysis/tests/fixtures/README.md`, `docs/analysis-port-log.md`.

### Increment 34 — Go pclntab function-name recovery (GolangSymbolAnalyzer)

The kuna analog of Ghidra's `GolangSymbolAnalyzer` (the **name-recovery** half).
Previously `golang-symbols` was classified ⛔ ("large subsystem, Go-only");
greenlit and now done. A Go binary embeds a `pclntab` (the "PC → line/name"
table) so the runtime can produce stack traces; because the runtime needs it, it
**survives stripping**. Without parsing it, every Go function decompiles as
`sub_<addr>`.

**BEFORE.** A (stripped) Go binary's functions render `sub_<addr>` — kuna never
parsed the pclntab, and a `-ldflags=-s -w` build has no `.symtab` at all, so
there is no name source for `main.main`, `main.compute`, `runtime.*`, ….

**AFTER.** `main.main`, `main.compute`, and the `runtime.*` set render NAMED,
recovered from the pclntab — even on a stripped binary where the pclntab is the
*only* possible name source.

**The pass (`s1_pclntab::GoPclntabPass`, id `gopclntab`, default-ON).** A new
`AnalysisPass`:
- **Locate the header:** the `.gopclntab`/`gopclntab` section start; else the
  `runtime.pclntab` symbol; else a validated byte scan of `.rodata`/
  `.data.rel.ro`/`.noptrdata`/`.data` (the kuna analog of
  `GoPcHeader.getPcHeaderAddress` + `findPcHeaderAddress`). Each candidate is
  confirmed by `probe_header` (the `isPcHeader` predicate: pad bytes 0,
  `minLC ∈ {1,2,4}`, `ptrSize ∈ {4,8}`, magic matches a known version read
  LE-then-BE — that LE/BE probe is also how endianness is recovered).
- **Parse across versions:** the four header magics — go1.2 `0xfffffffb`, go1.16
  `0xfffffffa`, go1.18 `0xfffffff0`, go1.20 `0xfffffff1` (1.19/1.21+ share the
  1.20 magic and layout). The layout differs by family:
  - **go1.18+:** the header carries `textStart`; the functab is `(entryoff: u32,
    funcoff: u32)` pairs (stride 8); entry PC = `textStart + entryoff`; the
    `_func` at `funcdata_base + funcoff` has `nameoff: i32` at offset 4 (after the
    `entryoff: u32`).
  - **go1.16/1.17:** no `textStart`; the functab is `(entry: uintptr, funcoff:
    uintptr)` (stride `2*ptrSize`); entry PC is the **absolute** `entry`; `nameoff`
    is at offset `ptrSize` in the `_func`.
  - **go1.2…1.15:** the legacy direct-functab layout — 8-byte header, `nfunc`
    (uintptr) at +8, then the `(entry, funcoff)` pairs; the name table IS the
    pclntab blob (no separate `funcnametab`), `nameoff` indexing the whole blob.
  Names are NUL-terminated UTF-8 read from `funcnametab + nameoff`. Inline
  sentinels (`entryoff == 0xffffffff`) and the `go:buildid`/`go.buildid` names are
  dropped (`GoFuncData.isInline` / `FUNCNAMES_TO_IGNORE`). The byte layouts were
  taken from Ghidra's `GoPcHeader`/`GoModuledata`/`GoFunctabEntry`/`GoFuncData` and
  cross-checked against the Go runtime source (`runtime/symtab.go`,
  `runtime/runtime2.go`) at go1.18.
- **Defensive (the never-fail contract):** an unknown magic, a malformed header,
  an out-of-range offset, a truncated table, or a missing section all yield an
  **empty** output — never a panic, never an error. An `nfunc` cap (2M) and
  checked-add arithmetic guard a corrupt header.
- **Emits** a `SymFact { addr: func_entry, name, kind: Function }` per function via
  the **EXISTING** symbol commit arm (engine.rs `commit_analysis_output`, no new
  arm). That arm is idempotent (`find_function(..).is_none()`), so a non-stripped
  Go binary's real `.symtab` name still wins — only a *stripped* binary's
  `sub_<addr>` functions take the recovered name.

**Gating (the parity proof).** Registered in `passes_for` **only when**
`detect_compiler == Go` (the same `.go.buildinfo`/`.note.go.buildid` gate the Go
no-return list uses, Increment 15) — so on every non-Go binary the pass is
*structurally absent* from the pass set, not merely a runtime no-op. The pass is
additionally a settable (`--option gopclntab on|off`, default-on); off suppresses
the recovery. Real-ELF Go path only ⇒ the XML datatest oracle is **structurally
untouched** (no XML binary is Go; the analysis tier never runs on the
`<binaryimage>` path).

**Tests (both ran, not skipped).**
- **Hermetic parser units** (`s1_pclntab/tests.rs`, 12 tests, no `go` needed —
  the merge-blocking gate): header probe (all four magics, LE+BE, the four
  malformation rejections), full functab→`_func`→`funcnametab` decode for
  **go1.18** (relative `entryoff`), **go1.16** (absolute `entry`), and **go1.2**
  (legacy blob-indexed names), inline-sentinel + `go:buildid` skipping, big-endian
  decode, and the truncated-table / unknown-magic defensiveness.
- **Real-Go e2e** (`kuna-console/tests/verify_go_pclntab.rs`, 2 tests): builds a
  tiny Go program (`func compute(x int) int` called from `main`) at runtime via
  `go build -ldflags=-s -w` (STRIPPED) in an isolated temp dir with a private
  `GOCACHE`/`GOPATH` (hermetic; guarded on `go` on PATH + the build succeeding,
  **skips cleanly** otherwise — Increment 15's pattern). Asserts `main.main`,
  `main.compute`, and `runtime.main` are recovered as NAMED functions from the
  pclntab on the stripped binary (where the pclntab is the only name source), and
  that `--option gopclntab off` suppresses the recovery (`main.compute` absent).
  **The e2e RAN (not skipped)** with go1.18 + the built x86 `.sla`.

**Result.** `make test` 675/675 PARITY OK; `make test-stages` 159/159 PARITY OK;
`make rust-test` green (incl. the 12 parser units + the 2-test e2e); `kuna catalog
--check` catalog OK. settable_count **36 → 37** (the `gopclntab` gate);
`stage_catalog.json` fixture + `docs/assertions.md` regenerated; the count-fixture
ripple (`settable_count_is_37`, `…suppressed_for_15` PASS_GATES +1,
`emit_catalog_json_static_form_brackets_and_commas` 36 commas,
`fixture_has_all_37_settables`) updated.

- **Divergence/LOSS:** none to the parity oracles — the pass is Go-gated and
  real-ELF-path-only, so the default (non-Go) path is byte-identical to before. The
  scope is the **name-recovery** half only; Ghidra's `GolangSymbolAnalyzer` also
  recovers RTTI types, method definitions, stack-trace signatures, and source-file
  maps — those subsystems stay out of scope (an ELF decompiler needs the
  entry-PC→name mapping; the rest is Go-runtime-specific markup).
- **New:** `kuna-analysis/src/s1_pclntab/{mod.rs, tests.rs}`,
  `kuna-console/tests/verify_go_pclntab.rs`.
- **Changed:** `kuna-analysis/src/{lib.rs, passes.rs}`,
  `kuna-console/src/{engine.rs, kuna_console.rs}`,
  `kuna-decomp/src/infra/architecture.rs`,
  `kuna-decomp/src/p0_knowledge/options.rs`, `kuna-decomp/stages.toml`,
  `kuna-decomp/src/p0_knowledge/kuna_stages/tests.rs`,
  `kuna-decomp/tests/{catalog_bytecompat.rs, fixtures/stage_catalog.json}`,
  `docs/{assertions.md, analysis-port-log.md}`.

### Increment 39 — Multi-format loader PR-6+7: Mach-O linked loading + __stubs import naming (the Mach-O headline)

**Premise.** PR-2 (Increment 36) proved Mach-O *objects* parse + map + select the
right spec behind `--experimental-formats`, with imports empty. This is the
**Mach-O headline** (PR-6 ∪ PR-7): a **linked Mach-O** exe loads and decompiles a
function, and its libc import renders **named** (`printf`) — the second non-ELF
binary kuna names imports on (after PE), and the simplest analog. PR-6 (loader
proof) needed **no new loadimage fix** — `object` surfaces the Mach-O
`vmaddr`/`LC_MAIN` transparently, so the linked exe loads through the same
`bootstrap_from_object` path the Mach-O object did, and PR-2's
`MachOFormat::{section_bits, compiler_model}` (x86-64→`gcc`, the System V AMD64
cspec; arm64→`default`) were already finalized. PR-7 (the `__stubs` walk) is the
real new logic, in a new `s1_loader/macho_stubs.rs`.

**The `__stubs` walk (`s1_loader/macho_stubs.rs`, design §3.3).**
`MachOFormat::resolve_imports` (was empty) now calls `resolve_macho_imports(file,
bytes)`. It branches on `FileKind` (`MachO64`/`MachO32`; a fat `0xcafebabe`/`bf`
selects one slice — prefer x86-64, then arm64, then first — and re-dispatches,
design §3.4) and re-parses with the typed `MachHeader64`/`32` (the neutral
`object::File` view drops the section `reserved1`/`reserved2` and the flat
indirect-symbol slice the walk needs). One pass over the load commands harvests
`LC_SYMTAB` (names), `LC_DYSYMTAB` (the indirect-symbol index array), and every
`LC_SEGMENT`'s sections (canonical `Segment::from_command` / `command.symtab()` /
`command.dysymtab()` dispatch). For each `S_SYMBOL_STUBS` section it names the
stub *entry* at `sec.addr + i*reserved2`; for each `S_LAZY/NON_LAZY_SYMBOL_POINTERS`
section (the GOT analog) it names the pointer slot at `sec.addr + i*ptr`. The
name is the `LC_SYMTAB` symbol at the indirect index (via the built-in
`Section::indirect_symbols` window). Filtering matches Ghidra
`processIndirectSymbols`: skip `INDIRECT_SYMBOL_LOCAL|ABS` and `n_strx == 0`,
strip the leading `_` (`_printf`→`printf`, the `strip_version` analog).
Pure/total/never-error: non-Mach-O / no `LC_DYSYMTAB` / unparsable → empty `Vec`.
`file.exports()` are appended additively (underscore-stripped). **Arch-independent
— no instruction decode**: the named address is pure section metadata.

**How `main` reaches `printf` (and why naming the stub entry suffices).** Unlike
the PE IAT slot / ELF GOT slot (a pointer the engine constant-folds), a Mach-O
`bl`/`callq` targets the `__stubs` entry **directly** — verified on both
fixtures: x86-64 `callq 0x1000005cc`, arm64 `bl 0x1000005a0`. So there is no
pointer to fold and no per-arch stub decode; naming the stub *entry* turns
`sub_<stub>(` into `printf(`. (The `__la_symbol_ptr`/`__got` slot naming is the
defensive ELF-GOT-slot mirror, for a `-fno-plt`-style folded indirect call.)

**Fixtures (linked in-container with `ld64.lld`, not vendored-prebuilt).** Bare
`clang` (no macOS SDK) compiles the Mach-O object today; for the *link* the
rustup-bundled `ld64.lld` (an LLD darwin flavor) produces a classic
`S_SYMBOL_STUBS` indirect-symbol layout — so no vendored-prebuilt stub was needed
(the design's fallback). `macho_imports.c` = `int compute(int n){return n*3+7;}
int main(int argc,char**argv){ printf("%d\n", compute(argc)); return 0; }`,
linked for two arches (`-undefined dynamic_lookup -e _main`):
- `macho_imports` (x86-64, 16 KB): `_compute`@`0x1000005a0`, `_main`@`0x1000005b0`,
  `printf` stub@`0x1000005cc`.
- `macho_imports_arm64` (arm64, 49 KB): same source, proving arch-independence.

**Before → after** (the headline; `kuna decompile … --experimental-formats`),
x86-64 `_main`:
```c
// BEFORE (resolve_imports empty)        // AFTER (PR-7 __stubs naming)
sub_1000005cc(0x1000005ee,a0 * 3 + 7);   printf(0x1000005ee,a0 * 3 + 7);
return 0;                                return 0;
```
arm64 `_main` renders `printf(0x1000005d0)` identically (named the same way; the
arm64 codegen folds the arg differently). The defined `_main` keeps its leading
`_` (it comes from the `file.symbols()` funcsym source, not the stub resolver).

**Decompile speed.** `macho_imports _main`: **~0.3–0.8 s** wall (`time kuna
decompile`, end-to-end incl. the `decomp_dbg` subprocess spawn + the `.sla`
load), **~0.10 s user** CPU for the decompile itself; `macho_imports_arm64`
~0.35 s wall. On par with the ELF/PE baselines — both dominated by process
startup + the tiny function. The `__stubs` walk is a single load-command pass +
per-section indirect-table slice (O(load cmds + indirect entries), once at load),
materially cheaper than the PE IAT walk (no `FF 25` `.text` sweep — Mach-O calls
the stub directly).

**Tests (the e2e RAN, not skipped — the `x86`/`AARCH64` `.sla` are built).**
- **`macho_stubs` unit tests** (synthetic, mirroring `elf_plt.rs`/`pe_iat.rs`, 3):
  the leading-underscore strip (`_printf`→`printf`, `__Znwm`→`_Znwm`, no-op on a
  clean name); the stub-entry naming + `INDIRECT_SYMBOL_LOCAL` filtering against a
  synthetic `S_SYMBOL_STUBS` window (modeling the x86-64 fixture: base
  `0x1000005cc`, `reserved2`=6); the symbol-pointer-slot naming (`base + i*ptr`).
- **e2e** (`kuna-console/tests/verify_macho_imports.rs`, 3): (1) the x86-64 linked
  exe loads (SysV/gcc spec), resolves `_main`/`printf`, decompiles `_main` to
  `printf(`; (2) decompiling `_main` **by VMA** (no symtab reliance) still names
  the `0x1000005cc` stub — the clean PR-7 proof; (3) the arm64 exe names `printf`
  identically (arch-independence). Test 1 also proves the **default-off** path
  (Mach-O rejected without the flag). A `Mutex` serializes the env-var-sensitive
  bodies (`KUNA_EXPERIMENTAL_FORMATS` is process-global).

**Result.** `make test` **675/675 PARITY OK**; `make test-stages` **159/159
PARITY OK**; `make rust-test` green (incl. the 3 unit + 3 e2e). Default-off ⇒ ELF
byte-identical (the Mach-O path is unreachable without `--experimental-formats`;
the ELF/XML oracles never reach the object loader — Invariant 1). **After this,
PR-8 (fat/universal + arm64e) and the on-by-default flip for Mach-O are the
remaining Mach-O-arm work.** No loadimage fix was needed.

- **Divergence/LOSS:** none to the parity oracles (the non-ELF path is
  flag-gated; the XML datatest path never reaches the object loader).
- **New:** `kuna-analysis/src/s1_loader/macho_stubs.rs`,
  `kuna-console/tests/verify_macho_imports.rs`,
  `kuna-analysis/tests/fixtures/{macho_imports, macho_imports_arm64}` (+ the
  `macho_imports.c` source recorded in the fixtures README).
- **Changed:** `kuna-analysis/src/s1_loader/{mod.rs, format/macho.rs}`,
  `kuna-analysis/tests/fixtures/README.md`, `docs/analysis-port-log.md`.

### Increment 40 — Multi-format loader PR-10: un-gate the format-agnostic passes for PE/Mach-O

**Premise.** PRs 3+4 (PE) and 6+7 (Mach-O) named imports on non-ELF binaries, but
the binary still rendered like a wire dump: `puts(0x140009000)`, `printf(0x…, a0)`,
dead code after a tail `exit`. The *logic* of several S1 analysis passes is
format-agnostic — they were merely **gated** `BinaryFormat::Elf` (or keyed on a raw
`SHF_*` flag the `object` crate only fills for ELF). PR-10 drops those gates and
generalizes the flag checks so a PE/Mach-O gets **string literals**, **typed libc
args**, and **no dead code** — not just named imports. **One pass per commit**, each
with a fixture assertion; ELF byte-identical (the default, non-experimental path).

**The headline before/after (PE — `pe_imports.exe`, `int main(){ puts("hello");
printf("%d\n", argc); }`).**

```text
                                  before (PR-7)          after (PR-10)
  string literal (s1_strings):    puts(0x140009000)      puts("hello")
  typed arg     (s1_protos):      printf(0x140009006,a0) printf("%d\n", a0 & 0xffffffff)
```

`s1_strings` recovers the `.rodata`/`.rdata` "hello" (≥5 chars) as a typelocked
`char[6]` symbol; `s1_protos` types `printf`'s first arg `char *`, so even the
short `"%d\n"` (3 chars, below the string-scan min-5) renders via the printer's
constant-string route. **Mach-O** (`macho_imports`, `printf("%d\n", compute(argc))`)
renders `printf("%d\n", …)` (was `printf(0x1000005ee, …)`) — the typed `char *` arg
plus marking `__cstring` READONLY. **No-return on a PE**: `__tmainCRTStartup`'s tail
`exit(…)` is on the new PeMac no-return list, so it is marked `/* WARNING:
Subroutine does not return */` and the dead fall-through after it is elided
(`--option noreturn_known off` restores the dead `*dat… = 1; v14 =
__tmainCRTStartup(); return v14;`).

**Per-pass status (each verified against the live code).**

| Pass | §5.2 gate today | PR-10 change | Status |
|---|---|---|---|
| `s1_strings` | `SectionFlags::Elf { sh_flags } & SHF_ALLOC` | per-format `is_loaded_initialized` (ELF `SHF_ALLOC` unchanged; PE/COFF mapped+readable; Mach-O non-zero-fill) | **un-gated cleanly** |
| `s1_protos` | `BinaryFormat::Elf` early-return | drop gate; union `present_function_names` with the §3 `resolve_imports` names (stripped-PE / Mach-O `__stubs` imports) | **un-gated cleanly** |
| `noreturn.rs` (Known) | `BinaryFormat::Elf` early-return | per-format base list: ELF list (+Rust/Go) vs a new vendored `PeMacFunctionsThatDoNotReturn` (exit/abort/quick_exit + `__fastfail`/`_invoke_watson`/…) | **un-gated + new list** |
| `s1_callfixup` | `BinaryFormat::Elf` early-return | drop gate (a `<callfixup>` `<target>` is a function NAME; the map comes from the loaded arch's cspec) | **un-gated cleanly** |
| `s1_addrtable` | `BinaryFormat::Elf` early-return + `SectionFlags::Elf` exec/searchable | drop gate; per-format `is_executable_section`/`is_searchable_section` (ELF `SHF_*` unchanged; PE `IMAGE_SCN_MEM_EXECUTE`/readable; Mach-O `S_ATTR_PURE_INSTRUCTIONS`/`SectionKind::Text`). Still off-by-default (Ghidra parity). | **un-gated cleanly** |
| `s1_noreturn_disc` | (format-neutral; consumes the Listing) | confirmed it runs on a PE — the Listing builds on a PE/Mach-O because `s1_entry::executable_sections` already falls through to `SectionKind::Text` for non-ELF | **already neutral, confirmed** |

**One small loader touch-up (not a gate-drop).** The Mach-O `__cstring`
(`SectionKind::ReadOnlyString`) was not marked READONLY by
`MachOFormat::section_bits` (PR-6 only handled `ReadOnlyData`/`Text`), so the
printer's constant-string route could not read a typed `char *` arg's bytes.
Adding the `ReadOnlyString` arm (a read-only string section *is* read-only) is the
prerequisite that completes the Mach-O `printf("%d\n", …)` literal — folded into
the `s1_protos` commit.

**Residual gaps (documented, not forced).** None block the headline.
- The Mach-O `%d\n` is below the string-scan min-5, so `s1_strings` never plants a
  symbol for it; it renders only via the typed-`char *` route. The Mach-O *string-
  literal* half of `s1_strings` is therefore not directly proven by a fixture
  (no Mach-O fixture carries a ≥5-char literal); the **logic** is exercised by the
  PE literal + the shared `is_loaded_initialized` Mach-O arm unit-covered.
- `s1_addrtable` stays **off by default** (Ghidra `setDefaultEnablement(false)`),
  so its generalization is logic-only (no default output change on any format).
- The deeper `s1_entry` entry-oracles (`.pdata`/TLS for PE, `LC_MAIN`/`__eh_frame`
  for Mach-O) remain §5.3 / PR-12+PR-13 work — out of scope here; the Listing tier
  only needs `executable_sections`, which already falls through to `SectionKind::Text`.

**Decompile speed.** `pe_imports.exe main`: **~0.25–0.44 s** wall (`time kuna
decompile`, incl. the `decomp_dbg` subprocess spawn + `.sla` load); ELF
`fauxware main` ~0.22–0.46 s — on par. The PR-10 passes add only an extra
`resolve_imports` call in `s1_protos`'s name union (already computed for naming)
and the per-format flag branches (O(sections)); no measurable overhead.

**Tests (the e2e RAN, not skipped — the `x86`/`AARCH64` `.sla` are built).**
- **Unit tests** (per pass, in-crate): `s1_strings` finds "hello" in the PE `.rdata`
  + an ELF-section-selection-unchanged parity guard over fauxware; `s1_protos` PE/
  stripped-PE/Mach-O present-name coverage; `noreturn` PeMac-list matching
  (`__fastfail`/`_invoke_watson`/`_exit`→`exit`); `s1_callfixup` scan-runs-over-PE;
  `s1_addrtable` scan-runs-over-PE+Mach-O (exec ranges resolve).
- **e2e** (`kuna-console/tests/verify_multiformat_passes.rs`, 4, all RAN): the PE
  headline (`puts("hello")` + `printf("%d\n", …)`); the Mach-O `printf("%d\n", …)`;
  the PE `exit` dead-code elision + `noreturn_known off` restore; `s1_noreturn_disc`
  runs on a PE (Listing builds, consumer completes). A `Mutex` serializes the
  `KUNA_EXPERIMENTAL_FORMATS` (process-global) bodies.

**Result.** `make test` **675/675 PARITY OK**; `make test-stages` **159/159 PARITY
OK**; `make rust-test` green (incl. the new unit + 4 e2e). Default-off ⇒ ELF
byte-identical (the non-ELF path is unreachable without `--experimental-formats`;
the XML datatest path never reaches the object loader — Invariant 1; every ELF
flag arm is unchanged). **After this, the agnostic-pass bucket (§5.2) is done; PR-11
(`s1_dwarf` for Mach-O/MinGW-PE), PR-12/13 (`s1_entry` per-format), and PR-14
(`s1_sourcelang` per-format) are the remaining quality follow-ups.**

- **Divergence/LOSS:** none to the parity oracles (the non-ELF path is flag-gated).
- **New:** `kuna-analysis/data/PeMacFunctionsThatDoNotReturn`,
  `kuna-console/tests/verify_multiformat_passes.rs`.
- **Changed:** `kuna-analysis/src/s1_strings/mod.rs`,
  `kuna-analysis/src/s1_protos/mod.rs`,
  `kuna-analysis/src/s1_loader/noreturn.rs`,
  `kuna-analysis/src/s1_loader/format/macho.rs`,
  `kuna-analysis/src/s1_callfixup/mod.rs`,
  `kuna-analysis/src/s1_addrtable/mod.rs`, `docs/analysis-port-log.md`.
