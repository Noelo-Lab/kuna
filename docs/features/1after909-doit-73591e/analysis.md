# Analysis — `1after909::doit`: angr recovers `break`, kuna leaves loop-exit gotos

- **angr testcase:** `test_decompiling_1after909_doit`
- **Binary:** `/home/mahaloz/github/angr-dev/binaries/tests/x86_64/1after909`  (x86_64)
- **Function:** `doit` (@ `0x401869`)
- **angr version:** 9.2.213

## What angr does better

angr renders `doit` with **0 gotos / 0 labels** — the command-processing loop is a clean
`while`, and every error/quit path inside it is a structured `break` out of the loop, after
which the shared cleanup block (fflush / free-loop / `fclose` / `puts("Bye!")` / `return`)
runs once.

kuna renders the same function with **10 gotos / 2 labels** (plus a recovery-failure marker):

| kuna construct | count | what it really is |
|---|---|---|
| `goto label_239f` | 9 | `break;` out of the `while (v12 <= 0x38e)` command loop — `label_239f` **is** the loop's fall-through successor (the cleanup block) |
| `goto label_1dca` | 1 | convergence of two preamble paths into the command-loop head |

The full side-by-side is in [`angr-vs-kuna.txt`](./angr-vs-kuna.txt). Metrics:
`gotos 0 | 10`, `labels 0 | 2`, `loc 314 | 258`.

The dominant gap is the **9 `goto label_239f`**. Each is an edge from inside the command loop
to the loop's immediate successor. Semantically these are `break;` statements. angr's
Phoenix/SAILR structurer recovers them via its loop-successor refinement (break/continue
recovery); kuna leaves them as raw gotos with a synthesized label on the successor.

## The owning stage and the exact construct

- **Stage:** **S8 structuring** — `decompiler/crates/kuna-decomp/src/s8_structure/blockaction.rs`,
  inside `ActionFinalStructure::apply`.
- The region/loop structure is already recovered correctly: the command loop is a
  `BlockWhileDo` and its exit edges carry `f_loop_exit_edge`. What is missing is the
  **print-prep pass that lowers loop-exit `BlockGoto`s to `break`/`continue`**.
- Upstream Ghidra runs, in `ActionFinalStructure`:
  ```
  graph.orderBlocks();
  graph.finalizePrinting(data);
  graph.scopeBreak(-1,-1);     // break statements   <-- THIS recovers break/continue
  graph.markUnstructured();    // gotos
  graph.markLabelBumpUp(false);// label fixup
  ```
  kuna ports `finalizePrinting` (switch cases + for-loop iterators) and `markUnstructured`,
  but **`scopeBreak` is an explicitly-recorded SEAM stub** — see
  `blockaction.rs:3493` / `:3502` (`SEAM(W7/W8): ... scopeBreak ... remain unported ...
  Recorded as losses`) and `docs/rust-port/losses.md`. The flag constants it would set —
  `block::f_break_goto` (= 2) and `block::f_continue_goto` (= 4) — already exist
  (`substrate/block.rs:157,159`) and are unused; `next_flow_after` (`block.rs:1344`),
  one of the building blocks `scopeBreak` needs, is also already present.

## Hypothesis for the kuna change

Port `BlockGraph::scopeBreak` as a recursive walk over the already-collapsed structured tree:
descend carrying the enclosing loop's break-target and continue-target front leaves, and for
each `BlockGoto` (and goto-carrying `BlockIf`/`BlockSwitch` edge) whose target front-leaf is
the enclosing loop's exit, retag its `gototype` from `f_goto_goto` to `f_break_goto` (or
`f_continue_goto` for the loop top). The printer then emits `break;`/`continue;` instead of a
`goto` + label. Invoke it in `ActionFinalStructure::apply` between `finalizePrinting` and
`markUnstructured`, gated by a new option (default-off for byte-identical output).

## Why this is a **proposal**, not a one-shot implementation

This is **S8 structuring code**, and the fix is a faithful port of an upstream multi-step
print-prep pass (`scopeBreak` + its supporting recursion in `block.rs`), not a single
option-gated early-return modelable as one `Action`/`Rule`. Under the pipeline's Hard Rule 7
(touching S7/S8 structuring beyond a single gated early-return ⇒ LARGE) and the precedent
that every prior structuring gap in this repo was filed as a `[PROPOSAL]`
(condition-aware if/else tail dedup, multi-latch loops, irreducible loops, head-in-body
loops), this gap takes the proposal fork. A decider subagent independently returned
`scope: large` — its decision is recorded in `record.json`.
