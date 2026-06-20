# WS4c — the p-code section RTL-compile path + the ConsistencyChecker (LANDED)

This wave lands the deep p-code **section** path that WS4b stubbed with a
`pcode_unported` panic, plus the full `ConsistencyChecker`, driving the compiler
from **2/148** content-identical specs to **147/148** (the lone holdout,
`hexagon`, fails earlier in the WS2 hand parser — see "Remaining gap").

The gate is the decompressed `.sla` element stream being byte-identical to C++
`sleigh_opt` (LOSS-010: the raw deflate bytes differ per backend, so content is
the gate). `python -m kuna.slacomp --all` → **147/148**.

## What landed

### kuna-sleigh (additive — decoder 675/675 + PARITY OK untouched)

- **Compiler-only symbol kinds** (`slghsymbol.cc`): `MacroSymbol`,
  `SectionSymbol`, `BitrangeSymbol`, `LabelTableSymbol` — added as `SymbolKind`
  variants with their build accessors. None has `encode`/`decode` (all are
  removed by `SymbolTable::purge` before the `.sla` is written); `get_type`,
  the `as_*`/`as_*_mut` accessors, and the purge macro-operand-local handling
  were wired in.
- **`Constructor` section build** (`slghsymbol.cc`): `setMainSection`/
  `setNamedSection` (store the base-template-arena handle), `get_operands`, and
  `SymbolTable::mark_subtable_operands` (the `markSubtableOperands` check vector)
  + `check_symbols` (the per-scope unplaced/unreferenced-label walk) +
  `get_varnode` (the `SpecificSymbol::getVarnode` family).
- **`ConstructTpl`** (`semantics.rs`): `set_num_labels`/`set_opvec`/
  `get_result_mut` for `expandMacros`/`forceExportSize`.
- **The renumber cross-reference remaps** — the subtle part.  C++ shares
  pointers; the kuna by-id port must rewrite every encoded cross-reference when
  `purge`/`renumber` compacts ids or `order_operands` reindexes operands:
  - `Constructor.operands` ids (renumber)
  - `ContextCommit.sym` id (renumber)
  - `ContextOp` embedded `OperandValue.table_id` (renumber) and `index`
    (order_operands handmap)
  - operand `defexp` embedded `OperandValue.table_id` (renumber) and `index`
    (order_operands handmap)
  Implemented via `PatternExpression::remap_table_id`/`remap_operand_index`
  (recursive over the Binary/Unary/Value tree) threaded through
  `SleighSymbol::remap_symbol_refs` (renumber) and `SymbolTable::order_operands`
  (handmap).

### kuna-slacomp (the driver + the checker)

- **`slgh_compile.rs`** — the ~80 stubbed p-code-section `ParserActions` filled:
  the statement/expr/varnode/jumpdest/sizedstar/section/label builders, all
  forwarding to the inherited `PcodeCompile` machinery; the driver now
  **implements `PcodeCompile` directly** (collapsing the C++ `SleighPcode`
  back-pointer), so it owns the unique base / label count and gets `create_op`/
  `create_store`/`assign_bit_range`/… for free.  Plus `createMacro`/`buildMacro`/
  `createMacroUse`/`compareMacroParams` (via the WS3 `MacroBuilder`),
  `createCrossBuild` + `checkUniqueAllocation` (+ the `shiftUnique*` family /
  `insertCrossBuildRegion`), `finalizeSections`/`forceExportSize`/`findSize`/
  `expandMacros`, `constrainOperand`/`defineOperand`/`defineInvisibleOperand`/
  `selfDefine`, `contextMod`/`contextSet`, `recordNop`, and `buildConstructor`.
  The heterogeneous bison semantic values are threaded through **one tagged
  `RtlValue` arena** (globally-unique ids; per-kind arenas would alias
  numerically).
- **`consistency.rs`** — the full `ConsistencyChecker` (slgh_compile.cc:215-1776)
  as inherent methods on `SleighCompile`: `testSizeRestrictions` (the per-opcode
  size switch + `recoverSize` + unnecessary `ZEXT`/`SEXT`/`SUBPIECE`→`COPY`),
  `testTruncations` (`v_offset_plus`/`adjustTruncation`), `optimizeAll` (the
  `UniqueState`/`OptimizeRecord` machinery: `getDefinitions` interval splitting,
  `findValidRule` COPY-propagation with the read/write interference checks,
  `applyOptimization`), `testLargeTemporary`, and the post-order subtable
  traversal.

## The five byte-divergence root causes fixed (the WS4c grind log)

In order discovered, each ported faithfully against an instrumented `/tmp` copy
of the byte-untouched C++ tree (built with a custom `main`→`slgh_main` shim, run
with `-y` for the XmlEncode debug stream):

1. **`getUniqueAddr` increment** — was `0x10000`, C++ `MAX_UNIQUE_SIZE == 256`.
2. **The renumber/handmap cross-ref remaps** above (operands / ContextCommit.sym
   / ContextOp + defexp OperandValue table_id & index).
3. **`pcode_create_subpiece`** — the WS2 parser threads the `integervarnode`
   *value* (not an arena id); build its constant varnode inline.
4. **`buildPatterns`** — C++ builds only from `root->buildPattern()` (which
   recurses through subtable operands); an *unreferenced* subtable is left
   pattern-less and purged.  The port was also iterating the full `tables` list
   and building each, keeping unused subtables (extra symbols → every downstream
   id shifted).
5. **`DecisionNode::orderPatterns`** — the insertion-sort break point compares
   the original item against the **partially-sorted** list (C++ `list[j]`), not
   the original `j`; comparing the original reorders conflicting/equal patterns
   (swapped `<pair>` entries).
6. **`calcContextLayout`** — `FieldContext::operator<` orders by the varnode
   symbol's **name** then `qual->low`; the port sorted by symbol **id** (e.g.
   ARM's `condit`/`itmode`).

## Result

`python -m kuna.slacomp --all` → **147/148 content-identical** (was 2).  The full
toy/data family + every real ISA except hexagon: x86/x86-64, the ARM/AARCH64
family, MIPS (32/64, R6), RISCV, SparcV9, SuperH/SuperH4, PowerPC-class, Dalvik/
JVM, 6502/65c02/Z80/z180, 8048/8051/8085, the PIC family, MSP430, AVR/avr32a,
m68k (68020/68030/68040/coldfire), tricore, V850, loongarch, nds32, etc.

## Remaining gap (1/148)

`hexagon.slaspec` fails **before** the WS4c path — a WS2 hand-parser limitation:
`SLEIGH parse error: expected }, got <`.  Hexagon's grammar uses a construct the
WS2 recursive-descent parser does not yet accept; this is unrelated to the p-code
section RTL compile and belongs to a WS2 parser follow-up, not WS4c.

## Verification

- `python -m kuna.slacomp --all` → 147/148; the two DATA + full toy family +
  x86/ARM/MIPS/RISCV/… content-identical.
- `cargo test --workspace --no-fail-fast` → 0 failures (incl. the extended
  `ws4b_compile_e2e.rs` content-identity test over ~40 representative specs).
- Decoder unaffected (all WS4c changes are additive build-side): release
  `kuna-console`/`kuna-harness` build clean; rust datatests `[675,675]`; C++
  oracle `PARITY OK` (207/207 + 675/675); `kuna-sleigh` 211 lib + ws4a golden
  tests pass.
- `git diff` (since the WS4b base) touches only `rust/crates/kuna-slacomp/**` and
  `rust/crates/kuna-sleigh/**` — no `decompiler/cpp/**`, no `docs/**` outside
  `docs/rust-port/sleigh-compiler/`.
