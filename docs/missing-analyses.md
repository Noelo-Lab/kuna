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

The live, normative stage model is in [`stages.md`](stages.md) /
[`stage-model.md`](stage-model.md); every upstream *decompiler* module is mapped
to a stage in [`stage-mapping.md`](stage-mapping.md). The items below are the
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

Modules are grouped by the stage they feed (mostly P0/S1), mirroring the
`kuna-decomp` stage-folder scheme: `loadimage_object.rs` (the real-ELF
`LoadImage` backend), `s1_loader/` (PLT/GOT markup — `elf_plt.rs` — and, planned,
the `.symtab`/`.dynsym` reader and no-return detection), and one folder per
planned analysis (`s1_strings/`, `s1_dwarf/`, `s1_demangle/`, `s1_entry/`,
`s1_protos/`).

Each analysis implements **`kuna_analysis::pass::AnalysisPass`** — the
generalization of the de-facto `elf_plt` contract: a focused module that reads
the object and produces a flat, deduplicated `AnalysisOutput` of *facts*
(symbols, entries, no-return names, read-only ranges), never panicking and never
failing — it only ever contributes more knowledge. `run_analyses()` merges all
enabled passes; the merged output is then committed once into the engine at the
bootstrap seam (`kuna-console`'s `engine::bootstrap_from_elf`). Each pass's
`id()` registers in `stages.toml` like the `kuna_*` sub-stage fixes, so it
appears in `kuna catalog --json` and is flippable per-decompilation via
`--option <id> on|off` (and can default off, keeping the XML datatest gate — which
never constructs an `ObjectLoadImage` — untouched).

> Status: the crate, the `AnalysisPass` interface, and the relocated PLT/GOT path
> (`elf_plt` + `loadimage_object` + fixtures, moved out of `kuna-sleigh`) are in
> place. The PLT path still commits inline in `loadimage_object`; lifting it onto
> the `AnalysisPass`/`commit` seam lands with the first new pass (string-literal
> detection). The per-analysis roadmap with testcases is at the bottom of this file.

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

**kuna now:** [`kuna-sleigh/src/elf_plt.rs`](../decompiler/crates/kuna-sleigh/src/elf_plt.rs)
reconstructs `got_slot → name` from the dynamic relocations and decodes each
`.plt*` stub's GOT reference per architecture (x86-64, x86-32, AArch64, ARM32,
RISC-V; classic, CET `.plt.sec`, PIE, and stripped layouts). Matches feed the
existing loader symbol stream as named `FunctionSymbol`s, so `query_call`
resolves them. Model depth is "correct names"; the full external-location/thunk
object model (below) is deferred.

**Still a gap within this area:**
- PPC64 (ELFv2 `.plt` is a data table; call stubs are synthesized in `.text`) and
  MIPS (`.MIPS.stubs` + `$gp`-relative GOT) have no regular decodable `.plt` code
  section — left as documented seams (names not recovered, behavior unchanged).
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

## 5. Demangling (C++ / Rust / Go / Swift) — ⛔ Gap

**Ghidra:** the demangler analyzers turn `_ZN3foo3barEv` → `foo::bar()`.

**kuna:** emits the raw mangled `.dynstr`/`.symtab` string. (The vendored
`beanstalk_cpp` fixture would show mangled import names.) Note: kuna's loader
splits names on `::` into namespaces, so an *already-demangled* name nests
correctly — but kuna does no demangling itself. C imports (`puts`, `read`) are
unaffected.

## 6. String-literal detection — ⛔ Gap (partly masked by the engine)

**Ghidra:** a string analyzer finds NUL-terminated strings in `.rodata` and
types the references as `char *`, so the decompiler prints `puts("Username: ")`.

**kuna:** prints the raw pointer constant (`puts(0x400915)`), because nothing has
typed `0x400915` as a string. The engine's type inference (🟡, below) can turn a
pointer into `char *` from *usage*, but it does not materialize the literal text.

## 7. Switch / jump-table recovery — 🟡 Inherited (core) / ⛔ refinement gap

The decompiler's jump-table machinery is ported (it is part of the engine, S2 +
feedback). What Ghidra adds at the application layer is *re-running* table
recovery after type recovery to refine case ranges with aggregate-type info; that
post-typing refinement loop is not part of the standalone engine.

## 8. Library prototype seeding (signatures for `printf`, `malloc`, …) — ⛔ Gap

**Ghidra:** ships parsed C headers / data-type archives (`.gdt`) and applies them
so imports get correct prototypes and parameter types.

**kuna:** infers calling convention and parameter storage from *usage* (S4/S5,
🟡). Correct names now resolve, but an import's argument *types* are still
inferred, not seeded from a known libc signature.

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

Each row is a future `AnalysisPass` (or extension) under `kuna-analysis/src/s1_*`.
Difficulty: **easy** = self-contained byte/string transform, no new heavy dep;
**med** = needs an engine API or a parser; **hard** = a new subsystem (a
debug-format reader or a discovery loop). Vendored fixtures live in
`decompiler/crates/kuna-analysis/tests/fixtures/` (`fauxware`, `cet_pie_x86_64`,
`stripped_dynamic_x86_64`).

| # | Analysis | Stage | Diff | Concrete testcase (fixture → assertion) |
|---|----------|-------|------|------------------------------------------|
| ✅ | PLT/GOT import names | S1 | done | **fauxware**: `0x400510→puts`, no symbol at `0x0`, no `@` in names (`kuna-analysis` tests + console e2e) |
| 1 | String-literal detection + `char*` typing | S1 | easy-med | **fauxware**: `kuna decompile fauxware main` stdout contains `"Username: "`, `"Password: "` (not raw `0x40xxxx`) |
| 2 | Demangling (Itanium C++ / Rust) | S1 | easy-med | unit: `demangle("_Z3fooi") == "foo(int)"`; + small `g++ -c` fixture → mangled symbol resolves to its demangled form |
| 3 | No-return detection | S1 | easy | **fauxware** `authenticate` calls `exit`: no dead fall-through after the call; unit: `exit`/`abort` flagged from the import-name set |
| 4 | DWARF debug-info | S1 | hard | **cet_pie_x86_64** (has `.debug_info`): recovered function names + ≥1 typed parameter appear (not `param_1`) |
| 5 | Library prototype seeding | S1/P0 | med | **fauxware**: first `printf` arg typed `char *` from a seeded libc signature (composes with #1 → `printf("Password: ")`) |
| 6 | Function-start / entry discovery | S1 | hard | **stripped_dynamic_x86_64**: discovered entry set includes the real `main`/entry, decompilable without a supplied address |
| 7 | External / thunk object model | S1 | hard | **fauxware**: PLT thunk to `puts` modeled as a thunk (tail-call inlined), not a standalone `sub_` |
| 8 | Arch markers (ARM/Thumb `$t`, MIPS `$gp`, x86 purge) | S1 | med | needs an ARM/MIPS fixture (not yet vendored): a Thumb function decodes as Thumb from its `$t` mapping symbol |
| 9 | Jump-table post-typing refinement | S2 (feedback) | hard | needs a switch-heavy fixture (not yet vendored): refined case count matches the typed table after a second pass |

### Do first (the simplest-to-easy, highest-impact)

1. **String-literal detection** (#1) — highest output-quality-per-effort; a
   `.rodata` NUL-terminated scan + a `char[]` typed-data symbol; no new dependency.
2. **Demangling** (#2) — a self-contained name transform; unit-testable with no
   binary; composes with kuna's existing `::` namespace split.
3. **No-return detection** (#3) — a fixed libc import-name set + the existing
   `FuncProto` no-return flag; removes spurious dead code after `exit`/`abort`.

All three are easy/easy-med, test against already-vendored fixtures (or pure unit
tests), and never touch the XML datatest parity path — both gates stay green.
