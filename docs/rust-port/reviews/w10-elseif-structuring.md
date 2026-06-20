# Else-if structuring + comment rendering (w10-elseif-structuring) — ACCEPT, +16

Wave (2026-06-17) integrated at rust-port (merge of `rport/w10-elseif-structuring` @ `bfa7ade`,
base c2245d3/435). Verdict: **ACCEPT**. The session's largest single wave.

## What landed (four faithful roots)

1. **`ActionPreferComplement`** was a no-op seam (`blockaction.rs:3327`). Ported
   `BlockIf::preferComplement`/`getSplitPoint`/`flipInPlaceTest`/`flipInPlaceExecute` across the
   dual sblocks/bblocks arena (`funcdata_block.rs`; C++ block.cc:3141/2365/2391/3038,
   blockaction.cc:2140) + closed the `opFlipInPlaceExecute` seam (funcdata_op.cc:1296).
2. **PendingBrace** — the EmitNoMarkup/printc path had none. Added it to `EmitBase`
   (`prettyprint.rs`) + the `emitBlockIf` else-if logic (`printc.rs`, C++ printc.cc:3027). This is
   the `else if` vs `else {` decision.
3. **`markUnstructured`** was a W7/W8 seam → no goto-label statements. Ported
   `BlockGraph::mark_unstructured` (`block.rs`; C++ block.cc:1250/2904/3115/3654), wired into
   `ActionFinalStructure`.
4. **CommentSorter** was ported but never wired into the printer → instruction/block comments
   unrendered. Wired `SorterFuncdata` + `setup_function_list` +
   `emitCommentBlockTree/emitCommentGroup/emitLineComment` (`printc.rs`; C++ printc.cc:3388/3404,
   printlanguage.cc:596).

## Effect

435 → 451 on its base; +16 on the integrated 449 tree → **465**. Gained: Else-if
#1/#2/#3/#4/#5/#6/#11/#14 (elseif 6/14 → 14/14), Loop comment #1-5 (0/5 → 5/5), Copy trim #7,
For-loop var used #2, Inlining #9.

## Gate (re-run at integration onto 449)

- `cargo test --workspace` (debug): 0 failures. The `verify_w10_inline_body` count was reconciled
  to **9/12** (Inlining #4 from the stack-frame wave + #9 from this wave both pass — neither
  branch alone foresaw the combined count).
- datatests: `[675, 465]`; passing-set diff vs base 449: **regressed-set EMPTY** (gained exactly
  the 16 above). printc.rs auto-merged with the landed string-literal/longdouble/heap-string
  changes (different regions).
- switch cluster 8/16/3 unchanged; oracle **PARITY OK**; C++ tree byte-untouched.

## No special-casing

General structuring/printing ports matching C++ `BlockIf::preferComplement` / PendingBrace /
`markUnstructured` / CommentSorter; no name/address/value hardcoding. `emitLabel` full
entry-address naming + `scopeBreak`/`markLabelBumpUp`/`orderBlocks` remain W7/W8 seams (the
affected `Else-if #12` checks label presence, not text).
