# phoenix-natural-loop-region-1c12bf — analysis

## Opportunity

- angr testcase: `test_decompiling_phoenix_natural_loop_region_head_in_body :: 0x442300`
- Binary: `binaries/tests/x86_64/windows/059ef54d0a97345369d236aafb051917c50680020a1bc532236072f4d341d9e3`
  (PE32+ console x86-64, a large statically-linked **Go** Windows executable).
- angr version: 9.2.213.
- arch override: `x86_64`.

## What angr does better

angr's `RegionIdentifier` (Phoenix structuring,
`angr/analyses/decompiler/region_identifier.py`) recovers the function as a single,
clean natural loop:

```c
while (true)
{
    ...
    if (...) { ...; continue; }
    ...
    goto LABEL_442558;   // exactly ONE forced edge
    ...
}
```

The test name — *natural loop region, head in body* — pins the specific angr fix:
when Phoenix identifies a natural loop, the loop **head** node can also be reachable
from inside the loop **body** (the back-edge target is referenced mid-region). angr's
region identifier handles that case so the loop collapses into one `while (true)` with
`continue`s, leaving only one genuine `goto`.

## What kuna produces (the gap)

kuna decompiles the same function (it loads only with `--experimental-formats`; the
default loader rejects the PE) into a body shredded by gotos:

- **31** `goto label_*` / `label_*:` tokens total (see `kuna-full.txt`).
- The natural-loop head at `0x4425a0` is emitted as `label_4425a0:` with
  `goto label_4425a0;` back-edges, instead of being recovered as a loop header.
- One `while (...)` *is* formed, but most of the loop body is scattered across
  `label_4423d8`, `label_442597`, `label_442661`, `label_4426a8`, `label_4426ad`,
  `label_4426e6`, `label_442705`, `label_442711`, `label_442340`, `label_442696`,
  `label_4426a0`, … as unstructured gotos.

Side-by-side: `angr-vs-kuna.txt`. Full kuna rendering: `kuna-full.txt`.

## Owning stage

S7/S8 region-and-loop **structuring**. In kuna the printed C is produced by the
upstream Ghidra **collapse-based** block structurer
(`decompiler/crates/kuna-decomp/src/s8_structure/blockaction.rs` —
`CollapseStructure` / `LoopBody` / `TraceDAG`), *not* by a region identifier.

kuna *does* contain a faithful port of angr's `RegionIdentifier`
(`s7_regions/kuna_regionid.rs` + `kuna_regiongraph.rs`), but it is **analysis-only /
observability** (driven by the `region tree/blocks/walk` console commands over
synthetic input). It is not wired into the emit path — it appears only as a module
declaration in `lib.rs`/`mod.rs` and produces no C.

## Why this is NOT a single option-gated Action (scope = large)

The `kuna_loweredswitch.rs` template works because it manufactures an **S2 artifact**
(a `BRANCHIND` + `JumpTable`) *upstream* of the structurer and lets the existing
structurer + printer do the rest. There is **no analogous single artifact** to inject
for a generic natural-loop-head structuring improvement: the fix lives *inside* the
structurer's loop detection.

Closing the gap requires EITHER:

1. **(a)** Extending Ghidra's collapse structurer's loop/region detection
   (`blockaction.rs` `CollapseStructure`/`LoopBody`/`TraceDAG`) to recognise the
   head-in-body natural loop — a change to S7/S8 structuring **core**, well beyond a
   single gated early-return; or
2. **(b)** Wiring the analysis-only `kuna_regionid` RegionIdentifier port into the
   emit path — a **new pass type** plus new emit infrastructure.

Both paths violate Hard rule 7 (new pass type / infrastructure, S7 structuring touch
beyond a gated early-return, >3 anchor files / >1 module). The decider subagent
returned `scope: "large"` (verbatim in `record.json`).

## Conclusion

This is a legitimate, reproducible gap (angr clean loop vs kuna 31-token goto soup),
but it cannot be reduced to one Action/Rule. Per the proposal fork it is escalated as
a `[PROPOSAL]` draft PR for human go/no-go on which path (a or b) to fund. No engine
code is changed in this PR.
