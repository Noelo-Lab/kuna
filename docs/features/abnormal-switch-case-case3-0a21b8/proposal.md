# [PROPOSAL] abnormal-switch-case-case3 — multi-predecessor unrolled-guard jump-table recovery

**Scope: LARGE — needs human go/no-go before any implementation worker is spent.**

## The problem

angr's `test_decompiling_abnormal_switch_case_case3` (`memmove` @ `0x18003c330`,
`msvcr120.dll`) recovers a clean `switch (count)` with 17 cases. kuna loses the
switch and emits an indirect *call*:

```c
                    /* WARNING: Treating indirect jump as call */
v7 = (void *)(*(code *)((uint8)*(uint4 *)(a2 * 4 + 0x18003c410) + 0x180000000))(a0,a1);
```

The dispatch is an **image-base-relative jump table**
(`target = 0x180000000 + table[index*4]`, 17-entry RVA table at `0x18003c410`)
whose `BRANCHIND` block has **two predecessors with two different bound guards**
(`cmp $0x10 ; jbe` at function entry; `and $0x7 ; test ; jne` on the
backward-copy path that re-enters the dispatch at `0x18003c3fc`). See
`analysis.md` for the full asm + decompilation evidence.

## Why it is not a one-Action feature (Hard Rule 7)

The recovery does not fail for lack of a small helper — it fails inside the
**core S2 jump-table guard analysis**:

- `JumpBasicModel::analyze_guards` (`s2_lift/jumptable.rs:1896`) bounds the table
  by walking backward through **single-predecessor** blocks for the dominating
  guard. With `size_in() > 1` it short-circuits to `check_unrolled_guard`.
- `check_unrolled_guard` (`s2_lift/jumptable.rs:2080`) is an **empty SEAM stub**:
  `// SEAM(structuring): checkCommonCbranch + findMultiequal + liftVerifyUnroll`.
- No bound ⇒ `recover_model_basic` returns `false`
  (`s2_lift/jumptable.rs:2113`) ⇒ `flow.rs:2719` truncates the indirect jump to
  a call.

Neither gated precedent fits: `kuna_loweredswitch.rs` manufactures a table from
a *comparison cascade* (none here); `kuna_switchmodbound` presupposes the guard
walk already melded a path (it did not). Closing the gap means **porting the
upstream multi-predecessor unrolled-guard machinery** into the
`check_unrolled_guard` SEAM — modifying `JumpBasicModel` core, not a single
gated early-return. This trips Hard Rule 7 on two counts (new S2 recovery
infrastructure; touches switch-recovery core beyond a gated early-return).

## angr reference

angr's `JumpTableResolver` / "abnormal switch case" handling resolves jump
tables whose bound check is split across multiple predecessor guards. The kuna
analog is upstream Ghidra's `JumpBasic::checkUnrolledGuard` →
`BlockBasic::findMultiequal` + `JumpBasic::checkCommonCbranch` +
`liftVerifyUnroll` (`decompiler/cpp/jumptable.cc`), which kuna left as the SEAM
stub at `jumptable.rs:2080`.

## Implementation plan (multi-step)

1. **Port `check_unrolled_guard`** in `s2_lift/jumptable.rs`:
   - `checkCommonCbranch` — when the rootblock has multiple predecessors, find a
     CBRANCH common to the guard structure of each predecessor path.
   - `BlockBasic::findMultiequal` (`s2_lift/blockbasic.rs` or `block.rs`) — pair
     up the MULTIEQUAL inputs across predecessors so the switch variable's value
     range is the union of per-path guards.
   - `liftVerifyUnroll` — verify the lifted (unrolled) guard genuinely bounds the
     index on every path before accepting the bound.
2. **Bound the table** from the unioned guard so `find_normalized` yields
   `jrange.size <= maxtablesize`, then let the existing readonly-LOAD
   image-base-relative table model (`jumptable.rs:2011`) read the 17 RVAs.
3. **Gate** the whole behavior behind a new `option abnormalswitch on|off`
   (default OFF while developing; ablation decides the ship default), so default
   output stays byte-identical when off.
4. **Test harness prerequisite (go/no-go):** kuna's loader is ELF-only and
   cannot load this PE. The end-to-end testcase must use a raw **bytechunk**
   (function bytes `0x18003c330..0x18003c895` at their real addresses, proven to
   reproduce the gap in `analysis.md`). The standard pipeline before/after demo
   (`kuna decompile <pe>`) will NOT run on this target.

## Speed / risk assessment

- **Risk: HIGH.** `analyze_guards` / `find_normalized` are core to *every*
  jump-table recovery in the engine. A faithful port must keep the 675/675
  datatest parity intact (several datatests exercise jump tables). The behavior
  must be option-gated so default output is byte-identical until the ablation
  proves it clean.
- **Speed:** guard analysis runs once per BRANCHIND during S2; the unrolled-guard
  walk is bounded (`maxbranch=2`, `maxpullback=2`). Expected negligible per-table
  cost, but must be measured on a jump-table-heavy corpus before any default-on.
- **Surface:** estimated 2 ported-core files (`jumptable.rs`, a block helper) +
  `options.rs`/`architecture.rs`/`stages.toml` registration anchors, plus the
  bytechunk test. Larger than one new module.

## Proposed option

`option abnormalswitch on|off` — "Recover image-base-relative jump tables whose
bound guard is split across multiple dispatch predecessors (angr's abnormal
switch case)." `source_decompiler=angr`,
`inspiration="test_decompiling_abnormal_switch_case_case3; JumpTableResolver / abnormal switch case; 0x18003c330"`,
`change_kind=structure-recovery`.

## Recommendation

File this draft and pause for human go/no-go. On approval, re-dispatch an
implementation worker on `feat/angr-abnormal-switch-case-case3-0a21b8` to port
`checkUnrolledGuard`/`findMultiequal`/`liftVerifyUnroll` behind
`option abnormalswitch`, with the bytechunk harness for the end-to-end test.
