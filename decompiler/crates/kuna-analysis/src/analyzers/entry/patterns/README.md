# Vendored Ghidra function-start byte-pattern XMLs

The `*.xml` here are vendored **verbatim** from upstream Ghidra
(`GHIDRA_REV` in `docs/UPSTREAM.md`, `cef869af04c4740a71ad31a55704045b1b0d1644`),
copied from `Ghidra/Processors/<P>/data/patterns/`. They are embedded via
`include_str!` in `mod.rs` (the analyzer tier has no spec-root file handle) and
parsed by the faithful port of `DittedBitSequence.initFromDittedStringData` +
`PatternPairSet.restoreXml`. They drive the **full byte-pattern function-start**
pass (`s1_entry::FuncStartPatternPass`, `--option funcstart_patterns on`,
default-off) — see `mod.rs`'s module doc.

| File | Upstream path | Loaded for |
|---|---|---|
| `x86-64gcc_patterns.xml` | `x86/data/patterns/` | x86-64 |
| `x86-64win_patterns.xml` | `x86/data/patterns/` | x86-64 |
| `x86gcc_patterns.xml` | `x86/data/patterns/` | i386 |
| `x86gcc_prepatterns.xml` | `x86/data/patterns/` | i386 |
| `x86win_patterns.xml` | `x86/data/patterns/` | i386 |
| `AARCH64_LE_patterns.xml` | `AARCH64/data/patterns/` | AArch64 (∅: all `<possiblefuncstart/>`) |
| `ARM_LE_patterns.xml` / `ARM_BE_patterns.xml` | `ARM/data/patterns/` | ARM (LE wired) |
| `riscv_gc_patterns.xml` | `RISCV/data/patterns/` | RISC-V |
| `MIPS_LE_patterns.xml` / `MIPS_BE_patterns.xml` | `MIPS/data/patterns/` | MIPS |
| `PPC_LE_patterns.xml` / `PPC_BE_patterns.xml` | `PowerPC/data/patterns/` | PowerPC (∅: all `<possiblefuncstart/>`) |

## Scope

Only the byte-decidable shapes are ported: a `<patternpairs>` whose post action is
an unconditional `<funcstart/>`, and a bare `<pattern>` with an unconditional
`<funcstart/>` (incl. the `*` mark-offset). The `after="defined"` / `validcode="N"`
/ `<possiblefuncstart/>` / `thunk` / `label` / `section` rules need a
`PseudoDisassembler`/`Listing` the analyzer tier does not have — a documented LOSS
(the same wall `s1_entry`'s module docs and `noreturn.rs` record). Arches whose
patternpairs are entirely `<possiblefuncstart/>` (AArch64, PowerPC) therefore yield
an empty set; their XMLs are kept (parsed, cached) so a future confirmation-machinery
port can use them without re-vendoring. The upstream `patternconstraints.xml`
language→file dispatch is replaced by `for_arch` (selection by `object`
architecture + endianness); the PLT-thunk `*_prepatterns.xml`
(`FunctionStartPreFuncAnalyzer`, `thunk="true"`) and the jump-table
`ARM_switch_patterns.xml` are out of scope and not vendored.
