# WS4a — the compile-side pattern-build machinery (LANDED)

This wave lands the **construction** side of SLEIGH patterns/equations/
decision-trees that the compiler drives — the prerequisite `ws4-blocker.md`
identified as missing. It is **purely additive** to the decode side already in
`kuna-sleigh`: new build methods/types alongside the existing decode code; the
decode path the 675/675 datatests depend on is untouched (verified: rust
datatests still `[675,675]`, C++ oracle `PARITY OK`).

## What landed (the 5 groups from the brief)

1. **`PatternExpression` build subclasses** (`slghpatexpress.rs`):
   `PatternValue::gen_pattern`/`gen_min_pattern` (per-leaf: `TokenField`,
   `ContextField`, `ConstantValue`, `OperandValue`, Start/End/Next2) and
   `PatternExpression::gen_min_pattern` (Binary/Unary return TRUE). The
   `getValue`/`getSubValue`/`minValue`/`maxValue`/`listValues`/`getMinMax`
   needed by `genPattern` were already on the decode side and are reused
   verbatim. `TokenField` grew two build fields (`tok_size`, `tok_index`)
   carrying the C++ `Token *`'s `getSize()`/`getIndex()` — set by
   `TokenField::new_for_build`, left `-1` by the decode factory (which nulls
   `tok` and never calls `genPattern`).

2. **`TokenPattern`** (`slghpatexpress.rs`): the token-aligned pattern builder
   — `new_true`/`new_bool`/`new_token`/`new_instruction_field`/
   `new_context_field`, `do_and`/`do_or`/`do_cat`/`common_sub_pattern`,
   `resolve_tokens`, `set_left/right_ellipsis`, `get_minimum_length`, the
   `buildSingle`/`buildBigBlock`/`buildLittleBlock` bit machinery. Token
   identity (`Token *` pointer compares) is the `BuildToken { size, bigendian,
   index }` value's `index`.

   **Fidelity note (the one subtlety):** C++ `TokenPattern`'s copy ctor /
   `operator=` run `pattern->simplifyClone()`, so every pattern *returned by
   value* from `doAnd`/`doOr`/`doCat`/`commonSubPattern` is simplified when the
   caller binds it (e.g. a trivial `ContextPattern AND InstructionPattern(true)`
   collapses to the bare `ContextPattern`). The Rust port applies
   `Pattern::simplify_clone()` to the result of each of those four methods to
   reproduce this — verified byte-identical against the C++ golden for
   `data-le-64`'s `:nop is test=1`.

3. **`PatternEquation` hierarchy** (`slghpatexpress.rs`): an arena enum
   `PatternEquation` + `EquationArena` (see "arena convention" below) covering
   `Operand`/`Unconstrained`/`Equal`/`NotEqual`/`Less`/`LessEqual`/`Greater`/
   `GreaterEqual`/`And`/`Or`/`Cat`/`LeftEllipsis`/`RightEllipsis`, with
   `gen_pattern` (incl. the `advance_combo`/`buildPattern` value-enumeration of
   the comparison equations), `resolve_operand_left` (+ `OperandResolve` and the
   `OperandResolveSink` seam for the operand mutations), and `operand_order`.

4. **The `Pattern` build/combine side** (`slghpattern.rs`): already present from
   the decode wave — `doAnd`/`doOr`/`commonSubPattern`/`simplifyClone` and the
   `PatternBlock` `intersect`/`commonSubPattern`/`shift` algebra are reused
   unchanged. WS4a adds no new code here; the `TokenPattern` layer calls them.

5. **`Constructor`/`SubtableSymbol`/`DecisionNode` build** (`slghsymbol.rs`):
   driven from the `SymbolTable` (the C++ pointer methods become
   `SymbolTable::build_subtable_pattern`/`build_constructor_pattern`/
   `order_operands`/`build_decision_tree`, because the build mutates operands and
   recurses into sibling subtables across the symbol table). The whole
   `DecisionNode` build side is ported: `add_constructor_pair`, `get_maximum_length`,
   `get_num_fixed`, `get_score` (entropy), `choose_optimal_field`,
   `consistent_values`, `split`, `order_patterns`, plus `DecisionProperties`.
   Build-side state was added to `Constructor` (`pateq: Option<EqId>`,
   `pattern`, `inerror`), `SubtableSymbol` (`pattern`, `beingbuilt`, `errors`),
   and `OperandSymbol` (mark/variable-length/offset/hand mutators) — all
   additive, default-initialized for the decode path.

## Arena-id storage convention (what WS4b consumes)

The WS2 parser threads `u32` ids for pattern equations. WS4a supplies the
storage those ids index:

- `EquationArena` (in `slghpatexpress.rs`) owns a `Vec<PatternEquation>`;
  `alloc(eq) -> EqId (=u32)` returns the id, `get(id)` borrows the node. **The
  driver (WS4b) owns one `EquationArena`** and hands the arena to
  `SymbolTable::build_subtable_pattern(table_id, arena, errs)`. Equation
  children are `EqId`s into the same arena (the C++ refcounted
  `PatternEquation *` tree → arena indices).
- `Constructor::add_equation(eq: EqId)` stores the constructor's pattern
  equation id; `get_pattern_equation() -> Option<EqId>` reads it back.
- Pattern **expressions** (`PatternExpression`) remain owned inline (the decode
  side already boxes them); only equations are arena-indexed, matching the WS2
  seam where `peq_*` actions return ids and `pexp_*` actions return owned
  expression trees.

## Freeze interfaces newly `pub` in `kuna-sleigh` (for WS4b)

| item | for |
|---|---|
| `slghpatexpress::{TokenPattern, BuildToken, EqId, EquationArena, PatternEquation, OperandResolve, OperandResolveSink}` | WS4b builds/threads equations + patterns |
| `slghpatexpress::TokenField::new_for_build` | parser builds token fields carrying token identity |
| `slghpatexpress::{PatternValue,PatternExpression}::gen_pattern/gen_min_pattern` | equation pattern generation |
| `slghsymbol::{DecisionProperties}` + `SymbolTable::{build_subtable_pattern, build_decision_tree}` | the `process()` build pipeline |
| `slghsymbol::Constructor::{add_equation,get_pattern_equation,get_pattern,set_error,is_error}` | constructor build state |
| `slghsymbol::SubtableSymbol::{is_being_built,is_error,get_pattern}` | subtable build state |
| `slghsymbol::OperandSymbol::{is_marked,set_mark,clear_mark,is_variable_length,set_variable_length,set_offset,set_offset_base,set_minimum_length,set_hand,change_local_index}` | operand offset resolution |
| `slghsymbol::{SleighSymbol::as_subtable, DecisionNode::get_field}` | inspection |
| `slghsymbol::{Constructor,SubtableSymbol}::set_built_pattern_for_test` | golden tests / out-of-band pattern injection |

## The one gap left for WS4b (recorded explicitly)

`Constructor::orderOperands` ends in C++ with `templ->changeHandleIndex(handmap)`
(re-index the p-code `ConstructTpl` handles through the new operand order). The
`ConstructTpl` arena is owned by the WS4b driver (the WS2 seam threads them as
`u32` ids the driver owns), so `SymbolTable::order_operands` performs every
operand/printpiece fix-up but **does not** touch the ConstructTpl handles. WS4b
must apply `changeHandleIndex(handmap)` to the constructor's main + named
sections after `order_operands` returns. The handmap is `operands[i]->hand`
(original index → new index); WS4b can recompute it from the reordered operand
list, or `order_operands` can be extended to return it. This is the only
build-step deferred to WS4b.

## Verification

- **Golden gate** (`rust/crates/kuna-sleigh/tests/ws4a_pattern_build_golden.rs`):
  built `Pattern`/`DecisionNode` XML-encoded and compared byte-for-byte against
  dumps from an **instrumented `/tmp` copy** of the byte-untouched C++ tree
  (`KUNA_DUMP_PATTERNS=<file> sleigh_opt <spec>`, a hook in
  `SubtableSymbol::buildDecisionTree`). Covers: `data-le-64`'s `:nop is test=1`
  (Equal + ContextField → bare `context_pat`; its 1-node decision tree); a toy
  `op8=0xf7` instruction field; a `phase=1 & op8=0xf7` AND → `combine_pat`; an
  arithmetic `op8 < 3+2` Less equation (5-way OR); and a 4-constructor subtable
  whose decision tree `chooseOptimalField` splits on a 2-bit instruction field
  (matches C++ byte-for-byte).
- **Build-side unit tests** in `slghpatexpress.rs`: `do_and`/`do_or` over token
  fields, `resolve_operand_left` Cat offsets, `operand_order`, impossible-Equal
  error.
- `cargo test --workspace --no-fail-fast` → **3733 passed, 0 failed**.
- Decoder unaffected: release `kuna-console`/`kuna-harness` build clean; C++
  oracle `PARITY OK` (207/207 + 675/675); rust datatests `[675,675]`.
- `git diff` touches no `decompiler/cpp/**`, no `docs/**` outside this directory.
