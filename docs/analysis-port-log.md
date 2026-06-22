# Analysis-tier port log

A running record of porting Ghidra's Java **analyzer / loader tier** (the "Run
Analysis" layer) into the Rust [`kuna-analysis`](../decompiler/crates/kuna-analysis)
crate, feature by feature, each with a testcase. This is the *process* log; the
gap inventory + roadmap is [`missing-analyses.md`](missing-analyses.md), and the
crate's design is in its `lib.rs` / `pass.rs`.

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

## Work-list

Status: ✅ done · ⬜ gap (to port) · 🟡 inherited (engine already does it) ·
⛔ out-of-scope / infeasible-at-this-tier.

| St | Pass id | Analysis (Ghidra) | Diff | Testcase |
|----|---------|-------------------|------|----------|
| ✅ | `plt-got` | PLT/GOT import names (`ElfDefaultGotPltMarkup`) | — | fauxware `0x400510→puts` |
| ✅ | `symtab-dynsym` | `.symtab`/`.dynsym` function reader | — | `fixture_funcsyms` |
| ✅ | `foundation` | generic `AnalysisOutput` commit seam | med | bootstrap_from_elf commits with no funcsym regression |
| ✅ | `noreturn_known` | No-return known list (`NoReturnFunctionAnalyzer`) | easy | fauxware `rejected`: no dead code after `exit(1)` |
| ⬜ | `s1-demangle` | Demangling (`GnuDemanglerAnalyzer` + Rust) | easy | `demangle("_ZN3foo3barEv")=="foo::bar"` |
| ⬜ | `s1-strings` | String-literal detection (`StringsAnalyzer`) | med | fauxware `main`: `puts("Username: ")` |
| ⬜ | `s1-libproto` | Library prototype seeding (`ApplyDataArchiveAnalyzer`) | med | `printf` arg0 typed `char *` |
| ⬜ | `s1-dwarf` | DWARF names+types (`DWARFAnalyzer`) | hard | cet_pie: real fn names + ≥1 typed param |
| ⬜ | `s1-entry-disc` | Function entry discovery (`EntryPointAnalyzer`) | hard | stripped: decompile entry without `--addr` |
| ⬜ | `s1-eh-frame` | `.eh_frame` FDE starts (entry oracle) | hard | C++ fixture: FDE starts ⊆ discovered entries |
| ⬜ | `arm-mips-markers` | ARM `$t`/MIPS `$gp` mapping symbols | med | (needs ARM/MIPS fixture, not vendored) |
| 🟡 | `switch-recovery` | `DecompilerSwitchAnalyzer` | — | the engine **is** this (S2 jump-tables ported) |
| 🟡 | `const-prop` | `ConstantPropagationAnalyzer` | — | engine does its own SSA const-prop (S3) |
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
