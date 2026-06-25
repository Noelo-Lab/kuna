# [PROPOSAL] `loopbreak_recovery` — port Ghidra `scopeBreak` to recover `break`/`continue`

**Opportunity:** `test_decompiling_1after909_doit::doit`
**Binary:** `/home/mahaloz/github/angr-dev/binaries/tests/x86_64/1after909` (x86_64), `doit` @ `0x401869`
**Proposed option:** `loopbreak_recovery`  ·  **Element id:** `4091`  ·  **Stage:** S8 structuring
**Scope:** **LARGE** (proposal fork, Hard Rule 7) — human go/no-go required before implementation.

## The problem

On `1after909::doit`, angr emits **0 gotos / 0 labels**; kuna emits **10 gotos / 2 labels**.
Nine of kuna's gotos are `goto label_239f` — error/`quit` paths that are really `break;`
statements out of the command-processing `while` loop (`label_239f` *is* the loop's
fall-through successor / cleanup block). kuna leaves them as raw gotos with a synthesized
label; angr recovers structured `break`s. See [`analysis.md`](./analysis.md) and the
side-by-side [`angr-vs-kuna.txt`](./angr-vs-kuna.txt).

This is not an irreducibility or region-recovery failure: the loop is already a `BlockWhileDo`
with `f_loop_exit_edge`-tagged exits. The missing piece is a **print-prep pass that lowers
loop-exit gotos to `break`/`continue`**.

## The angr reference

angr's Phoenix/SAILR structurer **loop-successor refinement** (break/continue recovery):
edges leaving a loop to its single successor become `break`; edges to the loop head become
`continue`.

The upstream-equivalent in Ghidra is **`BlockGraph::scopeBreak(-1,-1)`**, called in
`ActionFinalStructure` immediately after `finalizePrinting` and before `markUnstructured`.
In kuna this is an **explicitly-recorded SEAM stub**:
`s8_structure/blockaction.rs:3493` / `:3502`, `docs/rust-port/losses.md`. The flags it sets
(`block::f_break_goto`, `block::f_continue_goto`) already exist but are unused
(`substrate/block.rs:157,159`), and `next_flow_after` (`block.rs:1344`) is already present.

## Implementation plan (multi-step — why this is LARGE)

1. **Port `BlockGraph::scopeBreak` into `substrate/block.rs`** as a recursive method on the
   sblocks arena: descend the structured tree carrying the current loop's *break-scope* and
   *continue-scope* front leaves; for each `BlockGoto` / goto-carrying `BlockIf` /
   `BlockSwitch` case whose target front-leaf equals the enclosing loop's exit leaf, rewrite
   its `gototype` `f_goto_goto → f_break_goto`; equal to the loop top ⇒ `f_continue_goto`.
   May also need `getExitLeaf`/scope-front plumbing alongside the existing `next_flow_after`.
2. **Invoke it from `ActionFinalStructure::apply`** (`blockaction.rs`, between
   `finalize_*_printing()` and `mark_unstructured()`), gated by the new
   `loopbreak_recovery` arch flag — gated early-return keeps default output byte-identical.
3. **Teach the printer (`s9_emit/printc.rs`)** to emit `break;` / `continue;` for the new
   `gototype` flags instead of `goto label;` + suppress the now-dead label (Ghidra's
   `markUnstructured`/`markLabelBumpUp` interplay) — this is the part beyond a single
   early-return that makes it a structuring port, not a Rule.
4. Add option wiring: `stages.toml` `settableTable` row, `options.rs`, `architecture.rs`
   flag + reset default, `kuna_loopbreak_recovery.rs` action shim (or fold into the
   `ActionFinalStructure` call site).
5. **Tests:** `tests/stages/ghangr-1after909-doit-73591e.xml` (off ⇒ asserts the `goto
   label_239f`; default/on ⇒ asserts `break;`), baseline-stages bump; ablation over the
   675-assertion corpus (loop-exit gotos are pervasive — expect non-zero ablation churn, so
   likely **default-OFF opt-in** unless a clean DIV re-pin is justified).

This touches **S8 structuring** (`blockaction.rs`), the **substrate block tree**
(`block.rs`), **and the printer** (`printc.rs`) — >1 new module-level addition plus a
print-emit change, well beyond a single gated early-return. Hard Rule 7 ⇒ proposal.

## Speed / risk assessment

- **Speed:** negligible — one extra single-pass O(blocks) recursive walk of the structured
  tree at finalize time, once per function.
- **Risk:** medium. `break`/`continue` recovery is pervasive, so ablation churn across the
  datatest corpus is expected; the feature should ship **default-OFF opt-in** unless the
  ablation is provably clean. Mis-scoping a break target (nested loops/switches sharing a
  successor) could lower a goto to the wrong loop — the port must faithfully track the
  break/continue scope stack, exactly as upstream `scopeBreak` does. Because it changes
  emitted control flow, it must remain gated until the ablation is understood.

## Proposed provenance (for `settableTable` when implemented)

- `source_decompiler = "angr"`
- `inspiration = "test_decompiling_1after909_doit; Phoenix/SAILR loop-successor break/continue recovery (Ghidra BlockGraph::scopeBreak); doit"`
- `change_kind = "structure-recovery"`
- `summary` = recover `break`/`continue` for loop-exit / loop-back gotos instead of emitting `goto label;`.
- `use_when` = a loop's error/exit paths render as `goto <successor-label>` (angr emits `break;`).
