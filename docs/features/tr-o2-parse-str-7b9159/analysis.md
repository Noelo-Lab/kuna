# Analysis — `tr_O2.o::parse_str` (angr vs kuna)

- **angr testcase:** `test_decompiling_tr_O2_parse_str` (`angr/tests/analyses/decompiler/test_decompiler.py:3582`)
- **Binary:** `tests/x86_64/decompiler/tr_O2.o`  ·  **Function:** `parse_str` @ `0x400a20`  ·  arch `x86_64`
- **angr version:** 9.2.213
- The angr assertion is weak — `line_count > 20` ("it was failing structuring"). kuna already
  emits 354 lines, so kuna *passes that literal assertion*. The real gap surfaced by the
  pipeline is **structuring quality**.

## What angr does better

angr's SAILR/Phoenix structurer renders `parse_str` as a clean nested loop:

```c
v23 = *(ptr6);
if (v23) {
    v24 = 0; v25 = 0; v26 = ptr6;
    while (true) {
        ...
        if (v23 != 92) {
            *(ptr4) = v23; v23 = v29; v24 = v28;
            if (!v23) break;
        } else {
            ...
            switch ((char)v33) { case 0: ... case 7: ... }
        }
    }
}
```

| metric | angr (reference) | kuna |
|---|---|---|
| gotos | **11** | **37** |
| labels | **8** | **21** |
| loc | 417 | 354 |
| switches / cases | 1 / 16 | 1 / 16 |
| ifs | 33 | 32 |
| loops | 5 | 7 |

Both recover the same `switch` (16 cases) — the `loweredswitch`/jumptable side is already at
parity. The divergence is purely **goto/label density**: angr structures the function body into
`while(true){ if/else; switch }` with `break`/fall-through; kuna falls back to a goto web.

## The exact construct

kuna's distinct goto-target histogram (`grep -oE 'goto label_[0-9a-f]+'`):

```
 12  goto label_400af2     <- the loop back-edge / continuation merge (the `do {...} while(...)` tail)
  3  goto label_400e9c     <- shared error/cleanup epilogue
  3  goto label_400ba0     <- shared inner dispatch merge
  2  goto label_400fb0
  ~16 other distinct shared-block targets, ×1 each
```

The dominant pattern is **`goto label_400af2` ×12** — every `switch` case body and several
branches jump to the single loop-tail block (`*v17 = v23; ... } while (a0[...] == 0)`), i.e. a
**loop-continuation merge**. angr hoists this merge into the loop body so the cases fall through
to the latch naturally (no `goto`), producing `while(true){...}`. The rest are
shared-successor merges that angr resolves with condition-region structuring + tail duplication.

## Owning stage

S7 **region recovery** / S8 **structuring** (`docs/stage-mapping.md`):
`s7_regions/kuna_regiongraph.rs` (`KunaRegionIdentifier`, `region_structurer`),
`s8_structure/kuna_gotoreduce.rs`, `s8_structure/kuna_loopbreak_recovery.rs`. The real pass
order is in `infra/universalaction.rs` + the `coreaction_*.rs` files.

## Hypothesis & why it is not a single-Action fix

The clean output is a **cyclic** structuring result (loop refinement: hoisting the
continuation merge into the loop body, `continue`/fall-through synthesis, condition-aware loop
successor recovery). kuna's existing structuring options were tried on this exact target and
**none change it** (goto count stays 37 with each, or all, enabled):

- `gotoreduce` — tail-duplicate `if(cond) goto T` for small single-successor tails (acyclic).
- `loopbreak_recovery` — loop-exit `goto` → `break` (acyclic exit edges only).
- `regionstructure` — `region_structurer` is explicitly **acyclic** (sequence + ITE schemas +
  edge-virtualization); it does not perform cyclic loop refinement.

So closing this gap needs **new cyclic loop-refinement infrastructure** in S7/S8 (a
DreamStructurer/SAILR-style condition-region pass over loops), not a single Action/Rule like
`kuna_loweredswitch.rs`. Even eliminating all 12 back-edge gotos leaves ~25 vs angr's 11.

**Conclusion: LARGE → proposal fork (Hard Rule 7).** See `proposal.md`.
