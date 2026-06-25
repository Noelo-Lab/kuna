# [PROPOSAL] `earlyreturn` — angr-style early-return guard hoisting ("else simplification")

**Opportunity:** angr `test_od_else_simplification` · function `skip` · `od_gccO2.o` (x86_64)
**Proposed option:** `earlyreturn` (default-off, opt-in) · **Scope: LARGE** (human go/no-go)

## The problem

For a function whose first action is an argument-validity check, angr emits an idiomatic
early-return guard and de-indents the body; kuna wraps the entire body in an inverted
condition. See `analysis.md` and the captured side-by-side in `angr-vs-kuna.txt`.

```c
// angr (desired)                         // kuna (today)
if (!cur)                                 if (a0 != 0) {
    return 1;                                 ...whole body...
v3 = 0;                                    }
... body de-indented ...                   v14 = 1;
                                           label: if (canary) __stack_chk_fail();
                                           return v14;     // !a0 path lands here
```

The angr test (`cstyle_ifs=False`) asserts the **first** `if` is `if (!a0) { return 1; }`
with no else.

## angr reference passes

1. `optimization_passes/return_duplicator_{base,high,low}.py` — duplicate the shared
   epilogue (`__stack_chk_fail` guard + `return 1`) into the `!a0` path so it becomes a
   standalone *terminating* return.
2. `region_simplifiers/ifelse.py` `IfElseFlattener` — when the if's true-branch always
   terminates and the false-branch does not, drop the `else` and re-parent the body as a
   sibling after the `if`.

Both run on angr's mutable region tree.

## Why this is LARGE (not one option-gated Action/Rule)

- kuna has **no angr-style mutable region tree** at emit time. The if/else shape is fixed by
  Ghidra's block-graph collapse Structurer in
  `decompiler/crates/kuna-decomp/src/s8_structure/blockaction.rs` (~3600 lines, S7/S8 region
  recovery).
- Reproducing angr's output requires **two cooperating transformations** woven into that
  Structurer: (a) return/epilogue duplication into the guard path, and (b) else removal +
  body de-indent — i.e. a **new structuring pass type**, not a single `Action`/`Rule` like
  `kuna_loweredswitch.rs`.
- It touches S7/S8 structuring/region code well beyond a single gated early-return → Hard
  Rule 7 LARGE. A decider subagent independently confirmed `scope: large`.

## Proposed multi-step implementation plan (for the approved implementation worker)

1. **Pattern detection (S8).** In/after `CollapseStructure`, identify a top-level
   `BlockIf { cond, body }` whose *false* (fall-through) path reaches the function's shared
   return epilogue with no side effects other than constant return-value setup.
2. **Epilogue duplication.** Synthesize a duplicated terminating return (carrying the
   constant from the false path, e.g. `return 1`) into a new guard block `if (!cond) return
   K;` — the analogue of angr's return-duplicator.
3. **Restructure.** Replace the body-wrapping `BlockIf` with the guard followed by the
   de-indented body as siblings (analogue of `IfElseFlattener`). Preserve canary handling so
   `make test` / `make test-stages` parity is unaffected when the flag is off.
4. **Gate.** New `earlyreturn` bool on the architecture struct (default off in reset),
   registered action early-returns when off so default output is byte-identical;
   `settableTable` row + `options.rs` registration; LLM-discoverable.
5. **Verify.** Two-pass `tests/stages/ghangr-od-else-simplification-5194c9.xml` (off = body-wrapped
   bug, on = early-return guard). Ablation over the 675 datatests; default decision gated by
   the speed budget.

## Speed / risk assessment

- **Risk:** structuring rewrites are the highest-blast-radius part of the pipeline; an
  over-broad pattern could change many functions. Mitigate by a tight pattern (top-level
  single-guard + constant-return shared epilogue only) and keeping it **default-off opt-in**
  initially, with a full 675-assertion ablation before any DIV flip.
- **Speed:** one extra structuring walk per function; expected within the +5% budget but
  must be measured on the target (`skip`) before any default-on decision.
- **Loader caveat:** the opportunity binary is ET_REL (loader gap, PR #37); the
  implementation worker should validate via the link-bypass ELF or a bytechunk stage test.

## Relation to existing proposals

Distinct from `condfold` (#39: short-circuit `&&`/`||` folding + crossing-edge goto
reduction) and `elf-reloc-objects` (#37: the ET_REL loader). Same broad SAILR S7/S8
region-structuring family, but a separate, specific transformation (early-return guard
hoisting / else flattening). Not covered by any existing option (`returnpair`,
`sparcstructret`, `stackguard` are unrelated).

🤖 Generated with [Claude Code](https://claude.com/claude-code)
