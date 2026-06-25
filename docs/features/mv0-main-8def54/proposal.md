# [PROPOSAL] angr-mv0-main-8def54 — structure multi-latch loops (keep the switch inside the getopt loop)

**Status:** draft / awaiting human go-no-go. **Do not implement until approved.**

## The problem

On coreutils `mv` `main` (`/home/mahaloz/github/angr-dev/binaries/tests/x86_64/mv_0`,
fn `main` @ `0x402c40`), kuna fails to recover the `getopt_long` option-parsing
loop as a structured `while`. The loop has one header and **many latches** (each
of the 11 `switch` cases back-edges to the `getopt_long` block). kuna's S8
structurer degrades it to a goto-loop: it labels the header (`label_2d00:`),
**hoists the whole switch out of the loop** to the function tail, and synthesizes
`goto label_2d00;` from every case.

Result vs angr (which keeps the switch inside `while(true)`):

| metric | angr | kuna |
|---|---|---|
| gotos | 10 | 29 |
| labels | 9 | 16 |
| loops | 3 | 2 |

See `analysis.md` and `angr-vs-kuna.txt` for the full side-by-side.

## Why this is LARGE (not a one-Action feature)

The gap is in the **S8 collapse fixed point**
(`decompiler/crates/kuna-decomp/src/s8_structure/blockaction.rs`):
`CollapseStructure` / `TraceDAG` / `LoopBody` / `ruleBlockWhileDo` /
`emitLikelyEdges`. `ruleBlockWhileDo` collapses a loop only with a single
structured back-edge; multi-latch loops get their extra back-edges marked
unstructured and the loop falls back to `BlockGoto`, which in turn prevents the
`BlockSwitch` from nesting under the loop. A no-op gated Action cannot force the
existing collapse to keep the switch inside — the multi-latch normalization has to
happen **inside** the structuring fixed point. This touches S7/S8 region/structuring
code well beyond a single gated early-return and likely > 3 ported-core anchor
files. A decider subagent (Opus 4.8, high confidence) classified it LARGE.

## angr reference

- angr `RegionIdentifier` — already ported as
  `decompiler/crates/kuna-decomp/src/s7_regions/kuna_regionid.rs` +
  `kuna_regiongraph.rs` (S7).
- angr Phoenix/Dream-style `Structurer` / `RegionSimplifier` loop refinement:
  `_refine_loop`, break/continue insertion, single-latch normalization — the passes
  that let angr present one back-edge per loop body and render the rest as
  `continue`.

## Proposed implementation plan (multi-step)

1. **Detect** loop headers with N>1 in-loop back-edges (latches) in the S8 block
   graph, gated behind a new option (default OFF while developing).
2. **Latch unification:** route the N back-edges through a synthesized common
   latch / pre-header block so the loop presents a single structured back-edge to
   `ruleBlockWhileDo`; the per-case jumps become `continue`.
3. **Switch nesting:** teach the `BlockSwitch` path to allow case bodies whose tail
   is a loop back-edge to nest under the enclosing `BlockWhileDo` instead of being
   hoisted/gotos'd.
4. **(Alternative / larger)** route this loop shape through the already-ported S7
   `kuna_regionid` region structurer and bridge its output back into the S8 block
   model — reuses angr's loop refinement directly, but is a bigger integration.
5. Gate behind the option, add a `tests/stages/ghangr-mv0-main-8def54.xml` two-pass
   testcase (off = goto-hoist bug, default/on = structured while), regenerate the
   stage baseline, and measure decompile speed (standing requirement).

## Speed / risk assessment

- **Speed: medium risk.** Latch unification runs inside the S8 collapse fixed point
  on every function with a loop; a naive per-loop-header back-edge scan/merge can
  re-trigger the fixed point and add passes over the block graph. Must benchmark on
  the target (budget default +5%) and likely ship **default-OFF opt-in** until the
  ablation + speed gate are clean.
- **Correctness: medium-high risk.** Changing loop collapse / switch nesting can
  alter structuring on many functions. Requires the full 675-assertion ablation to
  stay PARITY OK; almost certainly default-OFF at first.
- **Anchor files:** `blockaction.rs` (collapse + switch nesting), the block-graph
  model, and option/architecture/stages.toml registration — > 3 ported-core files.

## Proposed option

- Option name: **`multilatchloop`**  (clean; decider's suggestion)
- Next free ElementId: **4021** (highest currently used is 4020).
- `change_kind`: `structure-recovery`
- `source_decompiler`: `angr`;
  `inspiration`: `test_decompiling_mv0_main; RegionIdentifier/Structurer loop refinement (single-latch normalization); main`

🤖 Generated with [Claude Code](https://claude.com/claude-code)
