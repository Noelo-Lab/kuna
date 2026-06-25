# optimized-memcpy-6301a9 — analysis

**angr testcase:** `test_decompiling_optimized_memcpy` :: `0x42cca0`
**Binary:** `i386/windows/736cb27201273f6c4f83da362c9595b50d12333362e02bc7a77dd327cc6b045a`
(an MSVC CRT `memcpy`/`memmove`, Duff's-device style)
**angr:** 9.2.213

## What angr does better

The angr test asserts `d.codegen.text.count("switch") == 7`: angr recovers **all 7
jump-table switches** in this optimized memcpy. The function is built from seven
`jmp dword [reg*scale + const_table]` dispatches, where each table
(`0x42cda8`, `0x42ce24`, `0x42cd28`, `0x42cfb0`, `0x42ceb4`, `0x42cf60`, …) holds
*absolute* target addresses (PE has a fixed image base + relocations).

kuna (driven on a self-contained bytechunk of the function at `0x42cca0`, since the
i386 PE loader can't open the file directly — see *Loader note*) recovers **only 1**
switch — the single plain-stride table `switch(a2 & 3)`. The other **6** indirect
jumps fail recovery and degrade to:

```c
/* WARNING: Treating indirect jump as call */
v1 = (void *)(**(code **)(v2 * 4 + 0x42cda8))();
return v1;
```

i.e. `FlowInfo::truncateIndirectJump` (`s2_lift/flow.rs:2719`) converts the
unrecovered `BRANCHIND` to a `CALLIND`. Side-by-side capture:
[`angr-vs-kuna.txt`](angr-vs-kuna.txt).

## The exact construct

The one table that recovers is a plain `reg*4 + table`. The six that fail all have a
**transformed switch index** *and* a guard that is duplicated across the unrolled
Duff's-device ladder's merge points:

| dispatch | index transform |
|---|---|
| `v2 * -4 + 0x42cf60`        | negative stride |
| `(a2 - 4) * 4 + 0x42ce24`   | offset |
| `((uint4)v4 & 3) * 4 + 0x42ceb4` | mask |
| `(a2 & 3) * 4 + 0x42cfb0`   | mask |
| `(v2 * 4) + 0x42cda8`       | plain, but guarded at an unrolled merge |
| `((uint4)a0 & 3) * 4 + 0x42cd28` | mask |

## Owning stage and root cause

**Stage:** S2 lift, switch-model sub-stage — jump-table recovery
(`decompiler/crates/kuna-decomp/src/s2_lift/jumptable.rs`).

The index transforms themselves are **not** the blocker: `JumpBasicModel`'s
`find_unnormalized_basic` / `circlerange_pull_back` (a real `CircleRange` value-set)
/ `build_addresses_basic` (a live `EmulateFunction`) are all ported and already
handle `INT_ADD`/`INT_SUB`/`INT_AND`/`ZEXT`/`SEXT` index pull-backs.

The blocker is the **bound**. `JumpBasic::analyzeGuards` (`jumptable.rs:1897`) calls
[`check_unrolled_guard`](../../../decompiler/crates/kuna-decomp/src/s2_lift/jumptable.rs)
(`jumptable.rs:2075`) for guard blocks with `size_in() > 1` — exactly the
Duff's-device unrolled-ladder merge points here. That method is a documented
`// SEAM(structuring)` **stub** (its own comment: *"the switch still recovers via the
straight-line path (the unrolled guard only adds an extra range constraint)"*). For
this MSVC memcpy that assumption is false: the unrolled/duplicated `cbranch` guard is
the **sole** source of the table bound. With it stubbed, `analyzeGuards` returns
without the bounding guard, the value-set range stays unbounded, `findNormalized`
can't size the table, and recovery degrades to `truncateIndirectJump → CALLIND`.

`check_unrolled_guard` needs `checkCommonCbranch` + the `BlockBasic` SSA helpers
`findMultiequal` / `noInterveningStatement` / `unblockedMulti` / `liftVerifyUnroll`,
all `// SEAM(W7)` in `substrate/block.rs` (header lines 52–56).

## Hypothesis for the kuna change

Port `check_unrolled_guard` (`checkCommonCbranch` + `findMultiequal` /
`liftVerifyUnroll` / `noInterveningStatement` / `unblockedMulti`) so the unrolled-guard
range constraint is recovered and the six transformed-index tables size and recover
natively — yielding angr-like 7-switch output. This is **core S2/S7 recovery** (it
edits `analyzeGuards`/`findNormalized` beyond a gated early-return and ports new
`BlockBasic` SSA infrastructure across `jumptable.rs` + `block.rs` + `funcdata`), so it
is a **LARGE** feature under Hard Rule 7 → this bundle is a `[PROPOSAL]`, not an
implementation. See [`proposal.md`](proposal.md).

## Loader note

kuna's loader does not open this i386 Windows PE directly
(`could not build an architecture … unsupported/!recognized binary`), so the
reproduction and any stage test use a self-contained `bytechunk` of the function +
its in-`.text` jump tables (the tables and most targets live within
`[0x42cca0, 0x42d040)`; the lone external tail-call to `0x439674` is stubbed `c3`).
The loader gap is tracked separately (multi-format loader PRs #32/#35/#38/#40) and is
out of scope here.
