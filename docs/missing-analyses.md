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

## Priority for closing the remaining gaps

1. **String-literal typing** (#6) — highest output-quality-per-effort; makes
   `puts("…")` render the text. Needs a `.rodata` string scan + `char*` typing of
   pointer constants into the type manager.
2. **DWARF** (#3) — large but high-value where present; recovers real local/param
   names and types.
3. **Demangling** (#5) — self-contained; a name-string transform at load time.
4. **PPC64/MIPS PLT + external/thunk model** (#1 remainder) — finishes import
   naming coverage and enables thunk inlining.
5. **Function-start discovery** (#4) — needed for fully stripped targets.
