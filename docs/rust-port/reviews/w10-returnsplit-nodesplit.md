# ActionReturnSplit + Funcdata::nodeSplit (w10-returnsplit-nodesplit) — ACCEPT, +7

Wave (2026-06-17) integrated at rust-port `7339946` (merge of `rport/w10-returnsplit-nodesplit`
@ `1aef56c`). Verdict: **ACCEPT** (the wave's own independent verify died on a transient API
529; re-gated + spot-checked in the main loop, see gate below).

## What landed

`Funcdata::nodeSplit` (block.rs/blockaction.rs/funcdata.rs/varnode.rs, +~580 lines in
funcdata_block.rs) + `ActionReturnSplit` — the structuring pass that duplicates a shared
RETURN block back into each of its in-edges when the return-value is an op whose inputs differ
per-path, so the structurer can recover the per-arm `return <expr>;` instead of a merged
phi-node tail. Faithful transcription of the C++ `ActionReturnSplit::apply` +
`Funcdata::nodeSplit`/`nodeSplitCloneOp`/`nodeSplitCloneVarnode`/`nodeSplitRawDuplicate`/
`nodeSplitInputPatch` chain (blockaction.cc / funcdata_block.cc).

## Effect

`switchmulti.xml` 1/9 → 8/9 — the 7 gained = Switch-Multi #2/#4/#5/#6/#7/#8/#9. #3 is a
pre-existing separate seam (not regressed by this wave; it never passed). Net rust-port
400 → 407.

## Gate (re-run in the main loop)

- `cargo test --workspace`: **3666 passed, 0 failed** (the fence catcher — no fence regression).
- datatests: `[675, 407]`; full per-assertion passing-set diff vs base `b36838a`:
  **regressed-set EMPTY** (strict superset — the 7 switchmulti arms are the only delta).
- the 6 switch datatests (switchind/switchmulti/switchhide/ifswitch/switchloop/switchreturn):
  switchmulti improved (1→8), the other 5 byte-identical — no switch regression.
- oracle `python -m kuna.run_tests --baseline docs/baseline.json`: **PARITY OK**.
- C++ tree (`decompiler/cpp/*.cc/*.hh` minus the auto-linked kuna_goldengen): **untouched**.

## No special-casing

Diff grepped for function-name / address / value / type-name / register hardcoding → zero
hits. nodeSplit is a general CFG transform driven by edge/op structure; ActionReturnSplit's
predicate is the generic "return input is a per-path-divergent op" test from upstream.
