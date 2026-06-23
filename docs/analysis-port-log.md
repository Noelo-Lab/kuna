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
| ✅ | `plt-got` | PLT/GOT import names (`ElfDefaultGotPltMarkup`) | — | fauxware `0x400510→puts` |
| ✅ | `symtab-dynsym` | `.symtab`/`.dynsym` function reader | — | `fixture_funcsyms` |
| ✅ | `foundation` | generic `AnalysisOutput` commit seam | med | bootstrap_from_elf commits with no funcsym regression |
| ✅ | `noreturn_known` | No-return known list (`NoReturnFunctionAnalyzer`) | easy | fauxware `rejected`: no dead code after `exit(1)` |
| ✅ | `s1-demangle` | Demangling (`GnuDemanglerAnalyzer` + Rust) | easy | `cpp_mangled` `main`: call renders `foo::Bar::baz(...)` |
| ✅ | `s1-libproto` | Library prototype seeding (`ApplyDataArchiveAnalyzer`) | med | fauxware `main`: `puts("Username: ")`, `puts("Password: ")`; `rejected`: `printf("Go away!")` |
| ✅ | `s1-strings` | String-literal detection (`StringsAnalyzer`) | med | ENABLED by default since the printer change — its planted `char[N]` symbol coexists with the literal: the printer renders a pointer to a readonly char-printable array symbol as the literal (Ghidra behavior). fauxware `main`: `puts("Username: ")`/`puts("Password: ")` with `s_400915` registered. See Increment 12 |
| ✅ | `s1-dwarf` | DWARF names+types (`DWARFAnalyzer`) via gimli | hard | dwarf_stripped: `add_values`/`compute`/`main` recovered (no .symtab); cet_pie: `elaborate_debug_symbol`'s param typed `char *` (subtasks 1+2; **subtask-3 stack-locals deferred**, engine change) |
| ✅ | `s1-entry-disc` | Function entry discovery (`EntryPointAnalyzer`/`FunctionStartAnalyzer`) | hard | stripped_dynamic: `sub_1405` (main) decompiles without `--addr` (Increment 5) |
| ✅ | `s1-eh-frame` | `.eh_frame` FDE starts (entry oracle, `GccExceptionAnalyzer`) | hard | fauxware: FDE starts ⊆ discovered entries (7 starts incl. `_start`/`main`) (Increment 5) |
| ✅ | `sourcelang` | Source-language / compiler detection (`SourceLanguageAnalyzer`) | easy | `s1_sourcelang::detect_compiler`: `rust_hello` ⇒ `Rustc` (`.comment` + `_ZN…17h…E`), `fauxware`/`cpp_mangled` ⇒ `Gcc` (Increment 7) |
| ✅ | `s1-rust-golang-noreturn` | Rust + **Go** no-return list selection (`noReturnFunctionConstraints.xml` `rustc` + `golang` arms) | easy | `RustFunctionsThatDoNotReturn` (Increment 7) **and** `GolangFunctionsThatDoNotReturn` (Increment 14) vendored + parsed per detected compiler; `ZN4core9panicking5panic17h*` flagged for Rust-only, `runtime.gopanic`/`runtime.throw`/`runtime.goexit.abi0` for Go-only, neither for a C ELF |
| 🟡 | `ruststring` | Rust str-slice split (`RustStringAnalyzer`) | med | **detection ported** (shares `s1_sourcelang`); the **split is infeasible-at-tier** (needs post-disasm interior refs + a populated ReferenceManager — same wall as `FindNoReturnFunctionsAnalyzer`). Documented, no split code (Increment 7) |
| ✅ | `arm-mips-markers` | ARM `$t`/`$a`+STT_FUNC-LSB → `TMode` (`ARM_ElfExtension`/`ArmSymbolAnalyzer`); MIPS `$gp` | hard | **ARM done** (Increments 8/17/18): `arm_thumb_le32.o` → `TMode=1` for `$t.0`@`0x0` + STT_FUNC LSB normalized to `0x0`/`0x14`; commit-arm paints `TMode` via `set_variable`, no-ops on non-ARM (fauxware byte-identical); Thumb-FUNC re-home (Increment 17). **Decode e2e now done in-container** (Increment 18): the LINKED `arm_thumb_linked_le32` ET_EXEC (`arm-linux-gnueabihf-gcc` in `kuna-dev`) Thumb-decodes `compute` → `a0 * 3 + 7` and `_start`'s CALL to the even entry resolves to `compute(` — no wiring fix needed. **MIPS `$gp`** done (Increment 17); **MIPS16 `ISA_MODE` out of scope** (needs a MIPS16 fixture) |
| ✅ | `s1-formatstring` (A+B) | printf/scanf varargs typing (`FormatStringParser` + `FormatStringAnalyzer`) | xhard | **A done** (Increment 9) — `s1_formatstring::parse_output_types("%d %s")` ⇒ `[Int, CharPtr]`, full conversion+length-modifier tables, `*`/`%%`/positional `%n$`, malformed no-panic. **B done** (Increment 14) — the decompile→inspect→override→re-decompile loop in `IfcDecompile`: walks `CALL` ops, classifies printf/scanf callees (`apply::classify_variadic_call`), reads the format constant at the format slot, builds a per-call-site `PrototypePieces` override (`apply::build_override_pieces`), re-decompiles. **Gated OFF** (`--option formatstring on`, Ghidra `setDefaultEnablement(false)`). `fmt_x86_64`: `printf("%d %s\n",a0,(char *)*a1)` typed vs default `(uint8)a0,*a1` |
| 🟡 | `addrtable` | Absolute address-table discovery (`AddressTableAnalyzer`) | med | implemented + tested but **disabled by default** (Ghidra `setDefaultEnablement(false)` + false-positive risk); scanner finds the 8-entry table @ `0x402008` in `switchtab_x86_64`. See Increment 4 |
| ✅ | `callfixup` | Auto-apply cspec call-fixups (`CallFixupAnalyzer`, install half) | med | `mcount_x86_64`: `main`'s `-pg` `call mcount` is **dissolved** — body becomes `return 0;` + `Function: mcount replaced with injection: mcount`. Pass matches FUNC names to cspec `<callfixup><target>`; commit tags inject id (the inherited inject/weave path applies it). Flow-repair half infeasible-at-tier (LOSS). See Increment 8 |
| 🟡 | `switch-recovery` | `DecompilerSwitchAnalyzer` | — | the engine **is** this (S2 jump-tables ported) |
| 🟡 | `const-prop` | `ConstantPropagationAnalyzer` | — | engine does its own SSA const-prop (S3) |
| ⛔ | `s1-aif` | Aggressive Instruction Finder (`AggressiveInstructionFinderAnalyzer` + ARM) | xhard | needs post-disassembly Listing/FunctionManager/PseudoDisassembler + ≥20 found functions — not at this tier; off-by-default upstream; folds into `s1-entry-disc` + `s1-eh-frame`. Increment 4 |
| ⛔ | `operand-refs` | Operand/scalar reference markup (`OperandReferenceAnalyzer`, `DataOperandReferenceAnalyzer`, `ScalarOperandAnalyzer`, `ElfScalarOperandAnalyzer`) | easy | no Listing/ReferenceManager at this tier; products subsumed by `s1-strings`/`s2`-jumptables/`s1-entry-disc`; the one relevant idea (scalar→`char*`) is blocked by the same printer/MapGlobals shadowing as `s1-strings`. Increment 4 |
| ⛔ | `s1-noreturn-discovered` | `FindNoReturnFunctionsAnalyzer` (flow heuristic) | hard | needs pre-decompile listing/flow — not at this tier |
| ⛔ | `thunk-model` | thunk/external object model | hard | needs `ExternalLocation`/S2-S4 internals |
| ⛔ | `x86-purge` | `X86FunctionPurgeAnalyzer` | — | Linux ELF x86 is cdecl; engine infers `extrapop=0` |
| ⛔ | `golang-symbols` | Go pclntab names | hard | large subsystem; Go-only |
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

### Remaining work (essentially complete)

Waves 1–3 (Increments 4–13) **and the entire deferred frontier are done**: DWARF subtask-3
stack locals (14), Golang no-return + completeness sweep (15), format-string-B varargs typing
(16, gated off), and MIPS `$gp` recovery + ARM Thumb-FUNC re-home (17). The completeness sweep
confirms **every feasible-at-tier, decompiler-relevant ELF analyzer is ported** (ELF matches all
three `noReturnFunctionConstraints.xml` lists; the per-compiler/per-arch passes cover the rest).

Only one item remains, **non-engine / cosmetic**:
- **ARM decode e2e** — ✅ **done in-container** (Increment 18). The earlier off-host block (no
  ARM linker on the build host) is lifted by the `kuna-dev` container's `arm-linux-gnueabihf-gcc`:
  the LINKED `arm_thumb_linked_le32` ET_EXEC fixture Thumb-decodes `compute` (`a0 * 3 + 7`)
  through the full pipeline (`verify_arm_thumb_decode.rs`), proving the `arm_markers` `TMode`
  paint + Thumb-FUNC re-home drive a correct decode — no wiring fix was needed.
- **`_INIT_<i>`/`_FINI_<i>` array-element naming** — cosmetic: `s1_entry` already *discovers*
  those addresses; only the Ghidra-style names differ (vs `sub_<addr>`), and delivering them
  needs reshaping the `entries` fact. Low payoff, documented follow-up.

Everything else is inherited by the engine or genuinely out-of-scope for an ELF decompiler
(non-ELF formats, Go pclntab, FID — see the table + inventory).

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
