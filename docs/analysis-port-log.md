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
(`AnalysisOutput { symbols, entries, noreturn, readonly }`) over an
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
| no-return | `query_global_function(name)?` → `Database::set_function_no_return(sid, true)` | NOT the private `Architecture::set_function_no_return(name,..)` |
| readonly range | `Database::set_property_range(varnode_flags::readonly, a1, a2_open)` | now applied on the ELF path too (was XML-only); load-bearing for string rendering |
| entry point | `name_function(addr)` → `add_function` + `register_symbol` (the `map function` recipe) | commit side is solved; *discovery* is the hard part |

## Is this every Ghidra analyzer? (complete inventory)

**No — and most should not be ported.** Ghidra ships **142** non-test `*Analyzer.java`
classes. Only a minority are program-analysis steps relevant to a *standalone ELF
decompiler*; the rest are per-CPU listing helpers, non-ELF format loaders, or infra. Full
accounting (every one of the 142 falls in exactly one bucket):

### Core program-analysis tier — `Features/{Base,Decompiler,GnuDemangler,FunctionID,DecompilerDependent}` (~57)

**Ported (4) + PLT:** `NoReturnFunctionAnalyzer`, `GnuDemanglerAnalyzer`,
`ApplyDataArchiveAnalyzer`, `StringsAnalyzer` (disabled — see Increment 3); plus
`ElfDefaultGotPltMarkup` (loader code, done pre-this-effort).

**🟡 Inherited — the ported engine already does this, no analyzer needed (~11):**
`DecompilerSwitchAnalyzer`, `DecompilerFunctionAnalyzer`, `DecompilerCallConventionAnalyzer`
(these literally *run the decompiler* — kuna **is** that engine), `ConstantPropagationAnalyzer`
(S3 SSA value-set), `StackVariableAnalyzer` (S5), `FunctionAnalyzer` ×2 (proto/param recovery, S4),
`SharedReturnAnalyzer`, `SharedReturnJumpAnalyzer`, `SegmentedCallingConventionAnalyzer`.

**⬜ Real gaps worth porting (deferred, in the work-list):** `DWARFAnalyzer`,
`EntryPointAnalyzer` (+ `ExternalEntryFunctionAnalyzer`, + the 5 `BytePatterns/FunctionStart*`
below), `GccExceptionAnalyzer` (.eh_frame), `FormatStringAnalyzer` (printf varargs typing),
`ArmSymbolAnalyzer` ($t/$a), `CallFixupAnalyzer`, `SourceLanguageAnalyzer`.

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
Rust via `rustc-demangle`); `Rust/RustStringAnalyzer` (⬜ deferred).

### Framework/Build (2) — ⛔ `JitDataFlowBlockAnalyzer` (JIT infra), `SkeletonAnalyzer` (template).

**Summary:** of 142 analyzers, ~4 ported (+ `StringsAnalyzer`/`AddressTableAnalyzer` ported
but disabled-by-default) + ~11 inherited by the engine + the remaining real ELF-relevant gaps
deferred (DWARF, entry-disc, eh-frame, format-string, ARM/MIPS markers, callfixup, sourcelang)
+ ~112 out of scope for a standalone ELF decompiler (per-CPU helpers, non-ELF formats,
Windows/Go/Swift/ObjC, infra) — now including AIF and the operand/reference markup family,
both reclassified ⛔ infeasible-at-tier (Increment 4). The work-list below tracks the relevant
set; this section is the evidence it's exhaustive.

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
| 🟡 | `s1-strings` | String-literal detection (`StringsAnalyzer`) | med | implemented + tested but **disabled by default** — its named `char[]` symbol *blocks* literal rendering in kuna's printer; literals come from `s1-libproto` instead (see Increment 3) |
| ⬜ | `s1-dwarf` | DWARF names+types (`DWARFAnalyzer`) | hard | cet_pie: real fn names + ≥1 typed param |
| ⬜ | `s1-entry-disc` | Function entry discovery (`EntryPointAnalyzer`) | hard | stripped: decompile entry without `--addr` |
| ⬜ | `s1-eh-frame` | `.eh_frame` FDE starts (entry oracle) | hard | C++ fixture: FDE starts ⊆ discovered entries |
| ⬜ | `arm-mips-markers` | ARM `$t`/MIPS `$gp` mapping symbols | med | (needs ARM/MIPS fixture, not vendored) |
| 🟡 | `addrtable` | Absolute address-table discovery (`AddressTableAnalyzer`) | med | implemented + tested but **disabled by default** (Ghidra `setDefaultEnablement(false)` + false-positive risk); scanner finds the 8-entry table @ `0x402008` in `switchtab_x86_64`. See Increment 4 |
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

## Open architectural decision: per-run option gating (deferred)

`docs/missing-analyses.md` promises each pass be flippable via `--option <id> on|off`
and discoverable in `kuna catalog`. There is a real ordering wrinkle: the analysis
commit happens in the `load file` body (`bootstrap_from_elf`), but `kuna decompile`
emits `option <id> <val>` *after* `load file`, so the gate would be a no-op as
wired. **Resolution (deferred to a dedicated increment):** move the `option` lines
before `read symbols` in the CLI `build_script`, stash the per-pass `AnalysisOutput`
on the program at load, and commit (gated by the now-set option) inside the
currently-stub `IfcReadSymbols`. Until then, passes are **default-on and bound to
the ELF path** (faithful to Ghidra's default-on analyzers, parity-safe), but not
yet per-run toggleable via `--option`. The pre-existing per-name surface
`--option noreturn <name>` still works as a manual override.

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

- **Known cross-pass seam:** the no-return pass matches on *raw object-symbol* names
  while demangle renames the *funcsym*; a no-return C++ symbol (e.g. `_ZSt9terminatev`)
  would be installed as `std::terminate`, so the no-return commit's
  `query_global_function("_ZSt9terminatev")` would miss it. Harmless for the common
  case (C libc no-return imports are unmangled). Documented for a later fix (match
  on the installed/demangled name, or resolve no-return across scopes by the demangled
  name).

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

### Next candidates

Per the work-list, in rough priority: **DWARF** (`s1-dwarf`, biggest naming/typing
source, hard — `gimli`), **entry discovery** (`s1-entry-disc`, hard), the **printer change**
to let `s1-strings` render literals (re-enables it), the **per-run `--option` gating** of
all passes (the deferred conflict #4), and the **no-return × demangle** cross-pass seam fix
(match the installed/demangled name). Inherited/out-of-scope items need no work (see table).
