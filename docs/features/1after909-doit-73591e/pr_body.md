# `loopbreak_recovery`: recover `break;` for loop-exit gotos (port of Ghidra `scopeBreak`)

Closes the angr-vs-kuna gap on `test_decompiling_1after909_doit::doit`. angr renders the
command-processing `while` loop with **0 gotos / 0 labels** — every error/`quit` path is a
structured `break;`. kuna emitted **10 gotos / 2 labels**, nine of them `goto label_239f;` to
the loop's shared cleanup successor (the semantic `break;`) plus a synthesized `label_239f:`.

See [`analysis.md`](./analysis.md) and the side-by-side [`angr-vs-kuna.txt`](./angr-vs-kuna.txt).

## Mechanism

angr's Phoenix/SAILR structurer runs loop-successor refinement (break/continue recovery). The
upstream equivalent is Ghidra `BlockGraph::scopeBreak(-1,-1)`, called in `ActionFinalStructure`
between `finalizePrinting` and `markUnstructured` — which kuna's port had left an explicit SEAM
stub (`docs/rust-port/losses.md`).

This PR ports it as a new module `s8_structure/kuna_loopbreak_recovery.rs`: a single recursive
walk over the already-collapsed structured tree (`BlockGraph`/`BlockGoto`/`BlockIf`/`BlockSwitch`/
`BlockWhileDo`/`BlockDoWhile`/`BlockInfLoop`/`BlockMultiGoto`/`BlockCondition`), carrying `curexit`
(the fall-through block) and `curloopexit` (the innermost loop's successor). A goto whose target
equals `curloopexit` is retagged `f_goto_goto → f_break_goto`. The printer already emits `break;`
for that flag, and `markUnstructured` (run after) then suppresses the now-dead successor label.
`BlockId` equality is the faithful analog of the C++ `getIndex()` identity. It only flips
`gototype` flags on already-structured nodes — no CFG/SSA/type change.

The whole pass is gated by `Architecture::recover_loop_break`, called from
`ActionFinalStructure::apply`; the only ported-core edits are that one gated call plus the
flag/option wiring.

## Option

`option loopbreak_recovery on|off` (ElementId 4091). **DIV-7 default-on**;
`option loopbreak_recovery off` restores the byte-identical raw-`goto` rendering.

## Ablation / parity

- **0 of 675** datatest assertions change with the flip on — `make test` stays **PARITY OK**
  without regeneration.
- `make rust-test` (cargo unit + golden differential + `.sla` parity) green with the flip on.
- Speed **+0.12%** (440.2 → 440.7 ms median on `doit`, within the 5% budget).
- Clean on all gates, and it *converges* kuna toward upstream Ghidra (which runs `scopeBreak`
  unconditionally) ⇒ shipped default-on (DIV-7).

On `doit` the nine `goto label_239f;` become `break;` and `label_239f:` disappears (10/2 → 1/1
gotos/labels; the lone remaining `goto label_1dca` is a forward jump into the loop head, a
separate structural gap, correctly left untouched).

Stage test: `tests/stages/ghangr-1after909-doit-73591e.xml` (two-pass off/on).

🤖 Generated with [Claude Code](https://claude.com/claude-code)
