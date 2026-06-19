# WS4 — BLOCKED on unported compile-side pattern/decision machinery

This document records the WS4 driver wave's finding: **WS4 (the `SleighCompile`
driver) cannot be completed as scoped, because the compile-side pattern,
pattern-equation, and decision-tree machinery it drives was never landed by any
prior wave (WS0–WS5) and is not present anywhere in the Rust tree.**

The WS4 brief assumed that "the lexer (WS1), parser (WS2), SleighPcode+MacroBuilder
(WS3), and `.sla` encode (WS5) are ALL LANDED" and that WS4 only has to "tie them
together" by filling the ~110 `ParserActions`/`ScannerHost` stubs plus
`process()`/`run_compilation`. That is true for the **definition** half of a spec
(spaces / tokens / contexts / varnodes / attaches / symbol table) but **false** for
the **constructor / pattern / semantics** half: the objects those parser actions are
supposed to *construct* do not exist.

## What is actually missing (the prerequisite, not part of WS4 as written)

The WS2 parser threads `u32` arena ids for pattern equations, pattern expressions,
ConstructTpls, etc., and the WS2 golden tests verify only the *action trace* — the
recording mock returns **fresh dummy ids** (`tests/slghparse_golden.rs`: `fn
peq_equal(...) -> u32 { self.fresh() }`). No real pattern object is built or stored
by anything in the tree today.

Concretely, the following C++ classes/methods are **MISSING** in `kuna-sleigh`
(verified by grep — only doc-comments and trait-method *names* mention them; no
`struct`/`enum`/`impl` exists):

### 1. `PatternExpression` arithmetic + instruction-value subclasses
(`slghpatexpress.hh:144-330`, `.cc`)
- Present (decode side, reused): `PatternExpression`, `PatternValue`, `TokenField`,
  `ContextField`, `ConstantValue`, `OperandValue`.
- **Missing**: `StartInstructionValue`, `EndInstructionValue`,
  `Next2InstructionValue`, `BinaryExpression`, `UnaryExpression`,
  `PlusExpression`, `SubExpression`, `MultExpression`, `LeftShiftExpression`,
  `RightShiftExpression`, `AndExpression`, `OrExpression`, `XorExpression`,
  `DivExpression`, `MinusExpression`, `NotExpression`.
  (Built by the `pexp_*` parser actions; `genPattern`/`getValue` compile-side.)

### 2. `TokenPattern` — the token-aligned pattern builder
(`slghpatexpress.hh:23-55`, `.cc`) — entirely **missing**. This is the engine that
turns equations into `Pattern`s (`buildSingle`/`buildBigBlock`/`buildSmallBlock`,
the `&`/`|`/`;` operators, `commonSubpattern`). Everything in #3 returns one.

### 3. The whole `PatternEquation` hierarchy
(`slghpatexpress.hh:351-478`, `slghpatexpress.cc`) — entirely **missing**:
`PatternEquation` (+ `OperandResolve`, `genPattern`, `genMinPattern`,
`resolveOperandLeft`, `operandOrder`, ref-counting `layClaim`/`release`),
`OperandEquation`, `UnconstrainedEquation`, `ValExpressEquation`, `EqualEquation`,
`NotEqualEquation`, `LessEquation`, `LessEqualEquation`, `GreaterEquation`,
`GreaterEqualEquation`, `EquationAnd`, `EquationOr`, `EquationCat`,
`EquationLeftEllipsis`, `EquationRightEllipsis`.
(Built by `peq_*`, `constrain_operand`, `define_invisible_operand`, `self_define`,
and consumed by `WithBlock::collectAndPrependPattern` and `buildConstructor`.)

### 4. Constructor / subtable / decision-tree **build** side
(`slghsymbol.cc`, ~19 sites; `slghsymbol.hh:494-589`) — **missing**:
`Constructor::buildPattern`, `Constructor::orderOperands`,
`SubtableSymbol::buildPattern`, `SubtableSymbol::buildDecisionTree`, the whole
`DecisionNode` build side (`split`/`orderPatterns`/`chooseOptimalField`/`getScore`/
`getBaseInstructionByte`/…). The `DecisionNode` *decode* side exists; the *build*
side does not. `process()` calls `buildPatterns()` → `SubtableSymbol::buildPattern`
and `buildDecisionTrees()` → `SubtableSymbol::buildDecisionTree` for **every** spec,
including data-le-64's one `:nop is test=1 unimpl` constructor.

### 5. `ConsistencyChecker`
(`slgh_compile.cc:215-1776`) — must walk the `OpTpl`/`VarnodeTpl` template trees to
derive Varnode sizes, optimize p-code (COPY propagation), and check truncations.
`process()` calls it (`checkConsistency`) before `buildPatterns`. The template types
exist (semantics.rs), but the checker/optimizer is ~1500 LOC of compile-side logic
that is part of WS4's own file and depends on #1–#4 being resolvable.

## Why even data-le-64 is blocked

data-le-64 (the simplest of 148 specs, 33 lines) still contains one context field
and one constructor:

```
define context contextreg test=(0,0);
:nop is test=1 unimpl
```

Compiling it runs the full `process()` pipeline: `checkConsistency`
(ConsistencyChecker, #5) → `buildPatterns` (Constructor/SubtableSymbol::buildPattern
→ TokenPattern → the `test=1` EqualEquation's `genPattern`, #1–#4) →
`buildDecisionTrees` (DecisionNode, #4) → `checkUniqueAllocation` → encode (WS5).
The `test=1` pattern alone requires `EqualEquation` + `TokenField`/`ContextField`
`PatternValue` + `TokenPattern` + `Constructor::buildPattern`. None exist.

So **no `.sla` can be produced** — the WS4 success criterion (data-le-64 →
content-identical `.sla`) is unreachable without first landing #1–#5.

## Recommended re-sequencing (for the integrator / WS7)

WS4 should be split, and the prerequisite landed as its own wave(s) *before* the
driver:

- **WS4a (pattern machinery, in `kuna-sleigh`)** — port, with golden-dump tests
  against an instrumented `/tmp` C++ tree:
  - `slghpatexpress.cc` compile side: the `PatternExpression` arithmetic/instr
    subclasses (#1) + `TokenPattern` (#2) + the `PatternEquation` hierarchy (#3),
    incl. `genPattern`/`genMinPattern`/`resolveOperandLeft`/`OperandResolve`.
  - `slghpattern.cc` compile side: `Pattern`/`PatternBlock` build operators
    (`commonSubpattern`/`doOr`/`doAnd`/`doCat`) the equations call (the decode-side
    `Pattern` types exist; their build operators may be partial).
  - `slghsymbol.cc` compile side: `Constructor::buildPattern`/`orderOperands`,
    `SubtableSymbol::buildPattern`/`buildDecisionTree`, `DecisionNode` build side.
- **WS4b (the driver, `slgh_compile.rs`)** — then the present WS4 brief is correct:
  fill the ~110 builder stubs + `process()`/`run_compilation`, with the
  `ConsistencyChecker` (#5) ported in-file. The driver threads the WS4a objects
  through its arenas exactly as the parser already threads the `u32` ids.

The arena-id convention WS2 froze (the parser returns/threads `u32` ids the driver
owns) is the right seam and stays unchanged; WS4a just supplies the real objects the
ids index.

## Definition-half work that *is* unblocked (deferred to WS4b)

For the record, the following WS4 driver methods depend only on the symbol
table / spaces / tokens / contexts (all of which have build-side API in
`kuna-sleigh`) and are implementable without #1–#5:
`predefinedSymbols`, `setEndian`, `setAlignment`, `newSpace`, `defineVarnodes`,
`defineToken`, `addTokenField`, `addContextField`, `calcContextLayout` /
`calcContextVarLayout`, `addUserOp`, `defineBitrange`, `attachValues` /
`attachNames` / `attachVarnodes`, `newTable`, `setAllOptions`, the preprocessor
hooks (`get/set/undefinePreprocValue`, `parseFromNewFile`/`parseFileFinished`,
`read_include`), `getUniqueAddr`, and the error/warning reporters. These were left
to WS4b rather than landed piecemeal here, because (a) they cannot be exercised
end-to-end without the constructor half (no `.sla` gate to verify them against), and
(b) they require promoting ~8 `pub(crate)` `SleighBase` accessors to `pub`
(symtab/manager/templates/root/alignment getters+setters) which is better done once,
atomically, alongside WS4b so the freeze table stays coherent. A small number of
these accessor promotions will be needed; record each in `map.md`'s freeze table.

## Verification of this finding (commands run)

- `grep -rn 'pub struct .*Equation|pub enum .*Equation' rust/crates/{kuna-sleigh,kuna-slacomp}/src` → none.
- `grep -rl 'pub struct TokenPattern|pub struct PlusExpression|pub struct EqualEquation|pub struct OperandEquation'` → MISSING (all).
- `tests/slghparse_golden.rs` mock builds nothing: `peq_*`/`pexp_*` return `self.fresh()`.
- The decode-side `Pattern`/`PatternExpression`/`TokenField`/`ContextField`/
  `OperandValue` types DO exist (`slghpattern.rs`, `slghpatexpress.rs`) and are
  reused; only the *compile* side (#1–#5) is absent — consistent with the explicit
  "What is NOT ported (SLEIGH compiler side)" note at `slghpatexpress.rs:27-39`.

No code under `decompiler/cpp/**` or `docs/**` (outside this directory) was touched;
no `.sla` was special-cased; `cargo build -p kuna-slacomp` remains green at the
inherited WS5 state (the WS4 `todo!()` bodies are untouched).
