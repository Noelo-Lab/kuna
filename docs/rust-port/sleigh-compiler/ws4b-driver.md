# WS4b — the `SleighCompile` driver: first end-to-end `.sla`

This wave lands the `SleighCompile` driver (`rust/crates/kuna-slacomp/src/slgh_compile.rs`)
on top of WS1–WS5 + WS4a, producing the **first `.sla` compiled entirely in Rust**
whose decompressed element stream is byte-identical to C++ `sleigh_opt`.

## Result

- **`data-le-64` and `data-be-64` compile byte-identical** (decompressed element
  stream, per LOSS-010) against the committed C++ `.sla`.  These are the only two
  of the 148 vendored specs with **no semantic p-code section** (a single
  `:nop is test=1 unimpl` constructor: one context field, one `EqualEquation`, no
  RTL body).
- Gate: `cargo test --workspace --no-fail-fast` → 0 failures (incl.
  `tests/ws4b_compile_e2e.rs`); rust datatests `[675,675]`
  (`KUNA_ENGINE=rust python -m kuna.run_tests --datatests`); release
  `kuna-console`/`kuna-harness` build clean.  No `decompiler/cpp/**` or `docs/**`
  (outside this dir) touched.

## What the driver implements (the full definition half + lifecycle)

`SleighCompile` composes `kuna_sleigh::SleighBase` (was the interface-freeze `()`)
and owns the compile state: the WS4a `EquationArena`, a pattern-expression arena,
the with-block stack, the constructor map `(subtable_id, ct_index)`, the context
table, error/warning counters, and the file/line/preproc state.

Implemented end-to-end and exercised by the data specs:
`setEndian`/`predefinedSymbols` (const/OTHER/unique spaces + inst_start/next/next2/
epsilon + root subtable), `setAlignment`, `newSpace`, `defineVarnodes`,
`defineToken`/`addTokenField`, `addContextField` + `calcContextLayout`/
`calcContextVarLayout`, `attachValues`/`attachNames`/`attachVarnodes`,
`addUserOp`, `defineBitrange` (byte-aligned), `newTable`, `newOperand`,
`createConstructor`/`addSyntax`/`buildConstructor` (no-section path),
`pushWith`/`popWith` + `collectAndPrependPattern`, the `peq_*`/`pexp_*` equation/
expression builders (into the arenas), and the `process()` pipeline:
`checkNops`/`checkCaseSensitivity` → `checkConsistency` (trivial w/o p-code) →
`buildPatterns` (WS4a `build_subtable_pattern` + the deferred
`changeHandleIndex` handmap applied to the ConstructTpl arena) →
`buildDecisionTrees` (WS4a `build_decision_tree`) → `checkUniqueAllocation` →
`symtab.purge` → WS5 `encode`.

## Additive `kuna-sleigh` changes (the WS4b prerequisites that were missing)

All purely additive; the decoder path (675/675) is untouched.

| addition | file | why |
|---|---|---|
| `SleighBase` compile-side build API (`symtab`/`symtab_mut`, `set_root`/`get_root`, `add_template`/`template_mut`/`templates`, `create_predefined_spaces`, `new_processor_space`, `set_default_code_space`, endian/alignment/unique-base, `indexer_mut`, `num_sections`/`unique_allocatemask`) | `sleighbase.rs` | the driver builds into the base |
| `SymbolTable::purge`/`renumber`/`replace_symbol` (LOSS-001, previously unported) | `slghsymbol.rs` | `process()`'s final `symtab.purge`; `attach*`'s `replaceSymbol` |
| `SleighSymbol::remap_symbol_refs` + `OperandValue::set_table_id` | `slghsymbol.rs`/`slghpatexpress.rs` | **the key fix** — see below |
| `Constructor::get_handmap` + stash in `order_operands` | `slghsymbol.rs` | the WS4a-deferred `changeHandleIndex` seam |
| `SymbolTable::find_symbol_by_id_mut`, `SleighSymbol::as_subtable_mut`, `SubtableSymbol::get_constructor_mut` | `slghsymbol.rs` | mutable build access |

### The renumber cross-reference fix (the one subtlety)

C++ symbols reference each other by **pointer**, so `SymbolTable::renumber` (run by
`purge`) can rewrite `sym->id`/`scopeid` without breaking any reference.  The kuna
port stores those references **by id**, so renumber must additionally rewrite every
**encoded** symbol-id cross-reference: `ContextSymbol.vn` (ATTRIB_VARNODE),
`VarnodeListSymbol.varnode_table` (ELEM_VAR ids), `OperandSymbol.triple`
(ATTRIB_SUBSYM), `OperandValue.table_id` (ATTRIB_TABLE), and `Constructor.parent`
(ATTRIB_PARENT).  This was the *only* byte-divergence on `data-le-64`: the
`test` ContextSymbol encoded the **pre-purge** id of `contextreg` (12) instead of
its renumbered id (6).  `remap_symbol_refs` fixes it; both data specs then match.

## Gaps left for WS6/WS7 (the unported p-code section path)

Every spec besides the two data specs has constructors with semantic RTL, which
needs machinery `kuna-sleigh` does not yet port.  The driver's section/p-code
`ParserActions` (`enter_section`, `set_result_varnode`, `rtl_add_oplist`,
`pcode_create_*`, `create_macro`/`build_macro`, `self_define`, `define_operand`,
`constrain_operand`, `context_mod`/`context_set`, the `sizedstar`/`jumpdest`/
`varnode`/`intvn`/`export` builders) are stubbed with a `pcode_unported` panic
carrying the action name; they are never reached by the data specs.

Concretely missing in `kuna-sleigh` (must land before WS6 can compile a real ISA):
- `Constructor::setMainSection`/`setNamedSection`/`markSubtableOperands`,
  `ConstructTpl::fillinBuild`, `forceExportSize`, `finalizeSections`
  (`slgh_compile.cc:3436`).
- `BitrangeSymbol`/`SectionSymbol`/`MacroSymbol` symbol kinds (`slghsymbol.cc`).
- The full `ConsistencyChecker` size/truncation/optimize passes
  (`slgh_compile.cc:215-1776`) — trivial here (zero ops), real once sections exist.
- `checkUniqueAllocation` crossbuild shifting (`slgh_compile.cc:3638`) — only fires
  with `crossbuild` directives.

The arena seam the driver threads (`u32` ids for pattern equations / pattern
expressions / ConstructTpl handles) is the correct interface and stays unchanged;
WS6 fills the section arena the same way the equation arena is filled here.
