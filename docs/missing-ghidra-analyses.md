# Missing analyses: what a standalone decompiler port does not inherit

kuna ports Ghidra's **C++ decompiler** (the SLEIGH-based "deep decompiler"). The
Ghidra *application* additionally runs a large body of Java **loader + analyzer**
code that prepares a program *before* and *around* the decompiler — parsing the
object file, naming things, discovering functions, recovering types. The
decompiler consumes the results of that work through the `DecompileCallback` XML
interface; it does not perform it.

A standalone port therefore inherits the decompiler's transformations but **not**
the analyzers that feed it. This document inventories the gap, with emphasis on
what affects naming and symbol resolution for ELF binaries (the class of problem
that produced `sub_400510(...)` instead of `puts(...)`).

The live, normative stage model is in [`phases.md`](phases.md) /
[`history/stage-model.md`](history/stage-model.md); every upstream *decompiler* module is mapped
to a stage in [`history/stage-mapping.md`](history/stage-mapping.md). The items below are the
*application-layer* steps that sit **outside** that stage model.

## Where these live: the `kuna-analysis` crate

These analyses have a dedicated home: the **`kuna-analysis`** crate
(`decompiler/crates/kuna-analysis/`). It sits *above* `kuna-decomp` in the
dependency graph so an analysis can both read the parsed object (via
`kuna-sleigh`'s loader + the `object` crate) and seed the engine's symbol/type
tables (`kuna_decomp::{architecture, database}`):

```text
kuna-base / kuna-num -> kuna-sleigh -> kuna-decomp -> kuna-analysis -> kuna-console / kuna-cli
```

Modules are grouped by what they feed (mostly P0/S1): `loadimage_object.rs` (the
real-ELF `LoadImage` backend), `loader/` (image-format markup — `elf_plt.rs`,
relocations, PE/Mach-O stubs, no-return detection), `analyzers/` (one module per
analysis pass), and `listing/` (the disassembly/xref tier).

Each analysis implements **`kuna_analysis::pass::AnalysisPass`** — the
generalization of the de-facto `elf_plt` contract: a focused module that reads
the object and produces a flat, deduplicated `AnalysisOutput` of *facts*
(symbols, entries, no-return names, read-only ranges), never panicking and never
failing — it only ever contributes more knowledge. Each pass's `id()` registers
in `phases.toml` as a settable option (and in `KUNA_OPTION_NAMES`), so it appears
in `kuna catalog --json` and is **flippable per-decompilation via `--option <id>
on|off`** — implemented as of the analysis-port "option-gating" Increment. The
passes are **default-on** (faithful to Ghidra's default-on analyzers), except
`addrtable`, which ships **off** (Ghidra `AddressTableAnalyzer` parity).

The gating's commit timing: `bootstrap_from_elf` runs
`run_default_analyses_per_pass` and **stashes** the per-pass `AnalysisOutput` on
the `ConsoleProgram` at load (it does NOT commit eagerly). The commit is deferred
to `read symbols` (`IfcReadSymbols::commit_pending_analysis`), which runs AFTER
the CLI's `option` lines (the `build_script` emits `option …` before `read
symbols`), so a per-run gate is in effect: each pass's enable flag
(`Architecture::analysis_*`) is consulted and a disabled pass's facts are
dropped. The XML datatest path stashes nothing, so the gated commit is a no-op
there — the parity gate (which never constructs an `ObjectLoadImage`) is
structurally untouched.

> Status: the crate, the `AnalysisPass` interface, the relocated PLT/GOT path,
> the generic commit seam, **and per-run `--option` gating of every pass are in
> place**. `bootstrap_from_elf` stashes the per-pass `AnalysisOutput` and commits
> it (gated) via `engine.rs::commit_analysis_output` at `read symbols`
> (function/data/entry/no-return/strings/prototypes/context/call-fixup fact kinds
> wired). All Wave-1/2/3 passes are done (no-return, demangle, strings, libproto,
> entry-disc, ARM markers, DWARF, call-fixup) and the printer renders
> readonly-char-array symbols as string literals. The PLT path still commits
> inline in `loadimage_object` (lifting it onto the pass list is cosmetic and
> deferred). The running process log is
> **[`history/analysis-port-log.md`](history/analysis-port-log.md)**; the per-analysis roadmap
> with testcases is at the bottom of this file.

## Legend

- ✅ **Fixed** — implemented in kuna now.
- 🟡 **Inherited** — the decompiler engine already does this (ported); no
  application analyzer needed.
- ⛔ **Gap** — done by a Ghidra Java analyzer/loader; not in kuna. Degrades
  output but does not crash; a user can often supply the fact by hand.

---

## 1. PLT/GOT import resolution — ✅ Fixed (this change)

**Ghidra:** `ElfDefaultGotPltMarkup` + `ElfProgramBuilder.createExternalFunctionLinkage`
(loader) parse `.rela.plt`/`.rela.dyn`, map each GOT slot to its `.dynsym` name,
and turn each PLT stub into a thunk to an external function. The decompiler then
sees the import name at the call site.

**kuna now:** [`kuna-analysis/src/loader/elf_plt.rs`](../decompiler/crates/kuna-analysis/src/loader/elf_plt.rs)
reconstructs `got_slot → name` from the dynamic relocations and decodes each
`.plt*` stub's GOT reference per architecture (x86-64, x86-32, AArch64, ARM32,
RISC-V, SPARC; classic, CET `.plt.sec`, PIE, and stripped layouts). SPARC is the
one decoder where the stub address and the name-map key coincide: its
`R_SPARC_JMP_SLOT` `r_offset` is the PLT entry itself (the linker rewrites the
in-place 32-byte `sethi/b,a` stub at resolution time), so `decode_sparc` just
strides the `.plt` and records any `sethi %g1`-headed entry that is a known
relocation (Increment 24, `plt_sparc64` e2e). PowerPC64 ELFv2 has no `.plt`
*code* section, so its TOC-relative call stubs (synthesized inline in `.text`)
are decoded out of band (`decode_ppc_text`/`decode_ppc64_stubs`: TOC base =
`.got` vma + `0x8000`, slot = `TOC + (addis@ha << 16) + ld@l`; Increment 26,
`plt_ppc64le` e2e). **MIPS** has its own resolver
(`resolve_mips_imports`, Increment 27): the o32 ABI has no `.plt` and no
`R_MIPS_JUMP_SLOT`, so the stub→name correspondence comes from the dynamic-symbol
GOT layout (`DT_MIPS_LOCAL_GOTNO`/`DT_MIPS_GOTSYM`/`DT_PLTGOT`,
`got_index(i)=LOCAL_GOTNO+(i-GOTSYM)`), the analog of Ghidra's
`MIPS_ElfExtension.fixupGot`/`processMipsStubsSection` (`plt_mips32` e2e). Matches
feed the existing loader symbol stream as named `FunctionSymbol`s, so `query_call`
resolves them. Model depth is "correct names"; the full external-location/thunk
object model (below) is deferred.

**Still a gap within this area:**
- SPARC's regular `.plt` **is now decoded** (Increment 24) — it earlier shared the
  seam row below but turned out tractable (its `R_SPARC_JMP_SLOT` `r_offset` IS the
  stub address).
- PPC64 ELFv2's `.text`-synthesized call stubs **are now resolved** (Increment 26):
  it was grouped here as a seam but turned out tractable — the stub's
  `addis r12,r2,@ha; ld r12,@l(r12)` pair statically reconstructs the `.plt` slot
  (`TOC + (@ha<<16) + @l`) whose `R_PPC64_JMP_SLOT` reloc names the import.
- MIPS (`.MIPS.stubs` + `$gp`-relative GOT) **is now resolved** (Increment 27):
  although it has no regular `.plt` code section, the dynamic-symbol GOT layout
  (`resolve_mips_imports`, above) recovers the import names — it earlier shared
  this seam row but turned out tractable.
- x86-32 **PIC** veneers (`jmp *disp(%ebx)`) are not statically decodable without
  the runtime `%ebx` GOT pointer — skipped.
- The external/thunk **object model** (Ghidra's `ExternalLocation` + thunk
  functions) is not built; kuna registers a plain named function at the stub.
  This is invisible for ordinary call naming but matters for thunk tail-call
  inlining and external-reference rendering.

## 2. Dynamic / defined symbol coverage — ✅ Improved

The loader previously read only `.symtab` `Text` symbols and (bug) registered
undefined import entries (`puts@@GLIBC_2.2.5`, `st_value == 0`) as functions at
address `0x0`. The fix skips `st_value == 0`, strips `@VERSION` suffixes, and
also reads `.dynsym` *defined* functions so stripped-but-dynamic binaries still
get their exported names. (`loadimage_object.rs::from_bytes`.)

The **data half** of the same two tables is covered as well (GH-184): every
defined, named `STT_OBJECT` entry with a non-zero `st_size` — `@VERSION`
stripped, deduplicated by address, `.symtab` before `.dynsym` — is collected at
load (`loadimage_object.rs::data_symbols`) and installed at the analysis commit
as a named `undefined<size>` global with `namelock` only, after the DWARF
globals and the detected string literals have claimed their addresses. This is
what names a copy-relocated (`R_X86_64_COPY`) libc extern — `stderr`, `stdout`,
`stdin`, `optind`, `optarg`, `__progname` — which has a real `.bss` address and
a `.dynsym` entry but no DIE in the program's own `.debug_info`, and a
stripped/DWARF-less binary's own statics. The Ghidra analog is the ELF symbol
importer's data-symbol path (`ElfSymbolTable` → data address markup), which
names data objects independently of debug info. Gated by `--option datasyms
on|off`, default ON (DIV-26/DIV-76); the commit is in
`kuna-console/src/engine.rs::commit_analysis_output`.

## 3. DWARF debug-info recovery — ⛔ Gap

**Ghidra:** the DWARF analyzer parses `.debug_info`/`.debug_line` to recover
function names, parameter and local names, types, and source lines.

**kuna:** no DWARF parsing. Even a "not stripped with DWARF" binary yields only
ELF symbol-table names and engine-inferred types. This is the single biggest
naming/typing source kuna leaves on the table. A faithful port would be a
substantial `.debug_*` reader feeding the symbol table and type manager.

## 4. Function-start / entry-point discovery — ⛔ Gap

**Ghidra:** disassembly + function analyzers find function entries by prologue
patterns, call targets, address-table references, and thunk detection, so the
listing has functions even where no symbol exists.

**kuna:** decompiles only entry points it is given (a symbol, or `load addr`).
It does not hunt for unlabeled functions. For a stripped binary the user must
supply addresses.

**Aggressive Instruction Finder (`AggressiveInstructionFinderAnalyzer`) — ⛔
infeasible-at-tier.** AIF is a *speculative* extension of entry discovery: it guesses
code in undefined gaps via instruction-mask fingerprinting + recursive-descent
PseudoDisassembly. It is **off-by-default upstream** (`setDefaultEnablement(false)`,
warns "MAY CREATE A LOT OF BAD CODE!") and requires a post-disassembly
Listing/FunctionManager (≥20 found functions) + a PseudoDisassembler — none of which
exist at the kuna-analysis tier (which runs before decompilation). Its sound output (new
entries) is subsumed by `s1-entry-disc` + `s1-eh-frame` for kuna's given-entries model.
Documented ⛔ out-of-scope (see [`history/analysis-port-log.md`](history/analysis-port-log.md) Increment 4),
the same call as `FindNoReturnFunctionsAnalyzer`.

## 5. Demangling (C++ / Rust / Go / Swift) — ⛔ Gap

**Ghidra:** the demangler analyzers turn `_ZN3foo3barEv` → `foo::bar()`.

**kuna:** emits the raw mangled `.dynstr`/`.symtab` string. (The vendored
`beanstalk_cpp` fixture would show mangled import names.) Note: kuna's loader
splits names on `::` into namespaces, so an *already-demangled* name nests
correctly — but kuna does no demangling itself. C imports (`puts`, `read`) are
unaffected. **Status:** ✅ done (Increment 2 — Itanium C++ + Rust). The Rust
*source-language* detection that gates Rust-specific behavior (e.g. the Rust
no-return list) is a separate piece, ✅ done in Increment 4 (`s1_sourcelang`).

## 6. String-literal detection — ⛔ Gap (partly masked by the engine)

**Ghidra:** a string analyzer finds NUL-terminated strings in `.rodata` and
types the references as `char *`, so the decompiler prints `puts("Username: ")`.

**kuna:** prints the raw pointer constant (`puts(0x400915)`), because nothing has
typed `0x400915` as a string. The engine's type inference (🟡, below) can turn a
pointer into `char *` from *usage*, but it does not materialize the literal text.

**Operand/scalar reference markup (`OperandReferenceAnalyzer` family) — ⛔
out-of-scope-at-tier.** Ghidra's operand-reference analyzers (`OperandReferenceAnalyzer`,
`DataOperandReferenceAnalyzer`, `ScalarOperandAnalyzer`, `ElfScalarOperandAnalyzer`) create
listing *references* — strings, pointers, address tables — from disassembled operands. They
need the disassembled Listing + ReferenceManager, neither of which exists at this tier, and
references never reach kuna's decompiler (it reads loadimage bytes + the symbol/type tables,
not the ReferenceManager). Their products are already covered: strings → `s1-strings`
(disabled, this section), address/switch tables → §7, function creation → §4 (`s1-entry-disc`).
`ScalarOperandAnalyzer` is even default-OFF for ELF upstream, and `ElfScalarOperandAnalyzer`
only *removes* bad `.got`/`.plt` references (which `elf_plt.rs` already names correctly). The
one relevant idea — typing a scalar that points at a `.rodata` string as `char*` — is blocked
by the same printer/MapGlobals shadowing that disables `s1-strings`, and is already delivered
by `s1-libproto` + S5 usage. Documented ⛔ (see
[`history/analysis-port-log.md`](history/analysis-port-log.md) Increment 4).

## 7. Switch / jump-table recovery — 🟡 Inherited (core) / ⛔ refinement gap

The decompiler's jump-table machinery is ported (it is part of the engine, S2 +
feedback). What Ghidra adds at the application layer is *re-running* table
recovery after type recovery to refine case ranges with aggregate-type info; that
post-typing refinement loop is not part of the standalone engine.

**Two distinct application-layer items, both classified (do not conflate with the
inherited core):**

- **Absolute address-table discovery (`AddressTableAnalyzer`) — 🟡
  ported-but-disabled.** A *byte-level data scan*: walk `.rodata`/`.data` for a run of
  consecutive pointer-width values that all land in an executable section (an absolute
  jump/function-pointer table) and lay down data labels. This is **NOT** switch recovery
  (that is the inherited engine machinery above) and **NOT** the post-typing refinement
  loop below. Faithfully ported in `s1_addrtable` (the scanner finds the 8-entry table @
  `0x402008` in the `switchtab_x86_64` fixture) but **disabled by default** — Ghidra ships
  it `setDefaultEnablement(false)` and a pointer-run scanner over-accepts (false-positive
  risk). See [`history/analysis-port-log.md`](history/analysis-port-log.md) Increment 4.
- **Post-typing refinement loop (roadmap #9) — ⛔ engine S2, not analyzer-tier.** The
  decompiler-internal multistage re-recovery (`recover_count > 1`), gated behind the
  `Override::queryMultistageJumptable` engine seam. It is an *engine* (S2-feedback) change,
  not a `kuna-analysis` pass; deferred as a separate future engine task.

## 8. Library prototype seeding (signatures for `printf`, `malloc`, …) — 🟡 Substituted

**Ghidra:** ships parsed C headers / data-type archives (`.gdt`) and applies them
so imports get correct prototypes and parameter types.

**kuna:** the `.gdt` format is a binary archive kuna does not vendor, so the pass
substitutes a **built-in signature table** — `libproto` (`LibProtoPass`, the
original 27-entry minimal stand-in) plus `libcsigs` (`LibcSigsPass`, the measured
~200-entry extension). Everything the table does not name still falls back to
kuna's usage inference (S4/S5, 🟡). Residual LOSS vs a real header archive: no
struct/enum/typedef definitions travel with a signature (a `struct stat *` is a
`void *` here), the vocabulary carries only width-stable slots — a declaration
using `off_t`/`time_t`/`long long` is rejected rather than approximated — and the
table covers libc/POSIX only, not the third-party libraries a `.gdt` set would
(OpenSSL, GnuTLS, …). Building a real header-derived type database is the open
follow-up; the current table's derivation and rejected set are in
`docs/features/libcsigs/`.

## 9. Engine capabilities kuna *does* inherit — 🟡

For contrast, these need no application analyzer — they are the ported engine and
already work:

- SLEIGH instruction decoding (vendored `specs/`, `kuna-sleigh`).
- Calling-convention / prototype recovery from usage (S4, `fspec.rs`).
- Type inference and propagation, pointer/`char*` from usage (S5).
- Jump-table core recovery (S2 + feedback).
- Dead-code, SSA, simplification, and C rendering with cast insertion (S6–S9).

---

## Port roadmap (ranked, with testcases)

> For the analyzers declared **infeasible-at-tier**, **non-ELF**, or **huge
> subsystem** below (AIF, the operand/reference markup family, Go pclntab, FID,
> PE/Mach-O loaders, …), the dependency-ordered build-plan — what kuna would have
> to BUILD first to make each feasible (notably a post-disassembly Listing tier),
> with effort sizing and verdicts — is in
> [`history/analysis-port-buildplan.md`](history/analysis-port-buildplan.md).

Each row is a future `AnalysisPass` (or extension) under `kuna-analysis/src/s1_*`.
Difficulty: **easy** = self-contained byte/string transform, no new heavy dep;
**med** = needs an engine API or a parser; **hard** = a new subsystem (a
debug-format reader or a discovery loop). Vendored fixtures live in
`decompiler/crates/kuna-analysis/tests/fixtures/` (`fauxware`, `cet_pie_x86_64`,
`stripped_dynamic_x86_64`).

| # | Analysis | Stage | Diff | Concrete testcase (fixture → assertion) |
|---|----------|-------|------|------------------------------------------|
| ✅ | PLT/GOT import names | S1 | done | **fauxware**: `0x400510→puts`, no symbol at `0x0`, no `@` in names (`kuna-analysis` tests + console e2e). Per-arch e2e: x86-64 (`verify_w11_elf_plt_names`), AArch64 (`verify_aarch64_plt`), RISC-V64 (`verify_riscv64_plt`), **MIPS32** (`verify_mips_plt`: `plt_mips32` `puts`/`printf` via the `.MIPS.stubs`/GOT layout — Increment 27) |
| ✅ | **Foundation: generic commit seam** | S1 | done | `bootstrap_from_elf` runs `run_default_analyses` + `commit_analysis_output`; no funcsym regression (`make test` PARITY OK) |
| ✅ | **No-return detection** | S1 | done | **fauxware** `rejected` calls `exit`: no dead fall-through after `exit(1)` (5 unit tests + e2e). See [`history/analysis-port-log.md`](history/analysis-port-log.md) increment 1 |
| ✅ | **Demangling** (Itanium C++ / Rust) | S1 | done | `cpp_mangled` `main` renders `foo::Bar::baz(...)` (cpp_demangle + rustc-demangle; needed the cross-scope call-resolution fix). Increment 2 |
| ✅ | **Library prototype seeding** | S1 | done | **fauxware** `main`: `puts("Username: ")`, `puts("Password: ")` (libproto types arg `char*`; the route that actually renders literals in kuna). Increment 3 |
| 🟡 | String-literal detection | S1 | n/a | implemented + tested but **disabled by default**: kuna's printer renders a named `char[]` symbol as its name (`s_400915`), shadowing the literal; literals come from prototype/usage `char*` typing instead. Increment 3 |
| ✅ | **DWARF debug-info** (`DWARFAnalyzer`) | S1 | done | **cet_pie_x86_64**: `elaborate_debug_symbol`'s param recovered as `char *binary` (gimli); **dwarf_stripped**: names with no `.symtab`. Subtask-3 stack-locals deferred. Increment 6 |
| ✅ | **Function-start / entry discovery** (`EntryPointAnalyzer`) | S1 | done | **stripped_dynamic_x86_64**: `sub_1405` (main) decompiles without `--addr` (e_entry + init arrays + `.eh_frame` FDEs + `_start`→`main` idiom + prologues). Increment 5 |
| ✅ | **`.eh_frame` FDE starts** (`GccExceptionAnalyzer`, entry oracle) | S1 | done | **fauxware**: 7 FDE starts ⊇ the funcsyms; feeds entry discovery. Increment 5 |
| ✅ | **Source-language detection** (`SourceLanguageAnalyzer`) | S1 | done | **rust_hello_x86_64** ⇒ `Compiler::Rustc` (`.comment` `rustc version` + `_ZN…17h…E`); **fauxware**/**cpp_mangled** ⇒ `Gcc`. Gates the Rust no-return list. Increment 7 |
| ✅ | **Rust no-return list** (`RustFunctionsThatDoNotReturn`) | S1 | done | vendored + matched only when compiler==Rustc: `ZN4core9panicking5panic17h*` flagged for Rust, not for a C ELF. Increment 7 |
| ✅ | **Golang no-return list** (`GolangFunctionsThatDoNotReturn`) | S1 | done | vendored + matched only when compiler==Go (`noReturnFunctionConstraints.xml` `golang` arm): `runtime.gopanic`/`runtime.throw`/`runtime.goexit.abi0` flagged for Go, not for a C/Rust ELF. Real-Go e2e built at test runtime (Go ELFs are ~1.1 MB — not vendored); hermetic matching tests. Increment 14 |
| 🟡 | **Rust str-slice split** (`RustStringAnalyzer`) | S1 | n/a | detection ported (shares the source-lang detector); the **split is infeasible-at-tier** — needs post-disasm interior reference destinations + a ReferenceManager (same wall as no-return-discovered/entry-disc), and even with boundaries the printer would shadow the literal. Documented, no split code. Increment 7 |
| 🟡 | Absolute address-table discovery (`AddressTableAnalyzer`) | S1 | n/a | implemented + tested but **disabled by default** (Ghidra parity + false-positive risk): **switchtab_x86_64** — `scan_address_tables` finds the 8-entry table @ `0x402008`, all elements in `.text`. NOT switch recovery (inherited S2) and NOT #9 below. Increment 4 |
| ⛔ | Aggressive Instruction Finder (`AggressiveInstructionFinderAnalyzer` + ARM) | S1 | n/a | infeasible-at-tier: needs post-disasm Listing/FunctionManager/PseudoDisassembler + ≥20 found functions; off-by-default upstream; subsumed by entry-disc + eh-frame. Increment 4 |
| ⛔ | Operand/scalar reference markup (`OperandReferenceAnalyzer` family) | S1 | n/a | out-of-scope-at-tier: no Listing/ReferenceManager; products subsumed by strings/jumptables/entry-disc; scalar→`char*` blocked by the same printer/MapGlobals shadowing as strings. Increment 4 |
| 7 | External / thunk object model | S1 | hard | **fauxware**: PLT thunk to `puts` modeled as a thunk (tail-call inlined), not a standalone `sub_` |
| 8 | Arch markers (ARM/Thumb `$t`, MIPS `$gp`, x86 purge) | S1 | med | needs an ARM/MIPS fixture (not yet vendored): a Thumb function decodes as Thumb from its `$t` mapping symbol |
| 9 | Jump-table post-typing refinement | S2 (feedback) | hard | needs a switch-heavy fixture (not yet vendored): refined case count matches the typed table after a second pass |

### Do first — ✅ all three done (increments 1–3)

1. ~~**No-return detection**~~ — ✅ done (increment 1).
2. ~~**Demangling**~~ — ✅ done (increment 2; needed the cross-scope call-resolution fix).
3. ~~**String literals**~~ — ✅ achieved (increment 3) via **library-prototype seeding**
   (`char*` arg typing + readonly + StringManager), *not* via planting `char[]` data
   symbols — kuna's printer prefers a symbol name over the literal, so the plant-a-symbol
   `StringsAnalyzer` mechanism is implemented but disabled. See
   [`history/analysis-port-log.md`](history/analysis-port-log.md) Increment 3 for the full finding.

DWARF, entry discovery, the printer change that re-enables the strings pass, and **per-run
option gating** (each pass flippable via `--option <id> on|off`, conflict #4) are all DONE —
see [`history/analysis-port-log.md`](history/analysis-port-log.md) Increments 5–13. None of the work touches
the XML datatest parity path — both gates stay green.
