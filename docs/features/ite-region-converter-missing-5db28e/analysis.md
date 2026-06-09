# Analysis — `test_ite_region_converter_missing_break_statement::authenticate`

**Opportunity:** angr testcase `test_ite_region_converter_missing_break_statement`, function
`authenticate` in `binaries/tests/x86_64/ite_region_converter_missing_breaks` (angr 9.2.213,
entry `0x4022e5`). Compiled with `-fstack-protector` (glibc canary).

## What angr does better

Side-by-side: `angr-vs-kuna.txt`. Metrics (reference | kuna): loc `21 | 40`, **gotos `0 | 1`**,
**labels `0 | 1`**, ifs `2 | 4`, ternaries `1 | 0`.

Two things diverge:

1. **A `goto label_2417` + `label_2417:`** in kuna that angr does not have. kuna emits, inside
   the loop's match case, `v2 = 1; goto label_2417;`, where `label_2417:` is the function tail.
2. **The stack-canary epilogue is visible** in kuna and absent in angr:
   ```c
   label_2417:
     if (v6 != *(int8 *)(v4 + 0x28))   // v4 = fs base; v6 = canary saved at entry
       v2 = sub_10e0();                 // sub_10e0 == __stack_chk_fail
     return v2;
   ```
   angr's tail is simply `return ...;` — its `StackCanarySimplifier` strips the
   compiler-inserted `-fstack-protector` boilerplate.

These are the **same** gap. The canary epilogue is a single shared return point (`if canary-bad
call __stack_chk_fail; return v`). Every return path in the function must funnel into it, so the
match case (deep inside the loop) cannot `return 1` directly — the structurer emits a `goto` to
the shared tail. Remove the canary check and the tail becomes a bare `return v`, which Ghidra's
`ActionReturnSplit` then duplicates into each predecessor, eliminating the goto.

> angr renders the *other* divergence (the trailing `if (memcmp) v=1 else v=0`) as a ternary
> `cond ? 1 : 0`. **Ghidra's C printer has no ternary operator** (`printc.cc` emits if/else or a
> `(uint)` cast of a boolean), so that exact rendering is not reproducible in kuna. The canary
> strip is the reproducible, principled win and is what this feature targets.

## The exact construct (SSA evidence)

`print raw` on `authenticate`:

- **Entry (Block 0):** `tmp = *(ram, FS_OFFSET + 0x28)` then `canary_slot(s..fff0) = tmp`.
- **Exit merge (Block 10):** the return-value `RAX = MULTIEQUAL(...)` (4 inputs — the 4 return
  paths), then `ZF = canary_slot != *(ram, FS_OFFSET + 0x28)` (reload), then
  `CBRANCH -> Block 11 if ZF else Block 12`.
- **Block 11 (fail):** single `call __stack_chk_fail` (`fsub_10e0`, no-return).
- **Block 12 (ok):** `return RAX`.

## Owning stage

`docs/stage-mapping.md`: this is **S7 region/structure recovery** acting on the simplified
post-SSA CFG, just before return-split. In the real pass order (`coreaction.cc`
`ActionDatabase::universalAction`) the hook is inside `actfullloop`, immediately before
`ActionReturnSplit("returnsplit")` (right after `ActionSwitchNorm` / `ActionLowerSwitchDetect`):
SSA and call specs are settled, and the canary CBRANCH is removed *before* return-split/structuring
can synthesize the shared return point and its goto.

## Hypothesis for the kuna change

A small, option-gated Action (`ActionStripStackGuard`, option `stackguard`, default-OFF while
developing) that:

1. detects a `CPUI_CALL` whose callee is `__stack_chk_fail`, whose parent block has a single
   in-edge from a block `H` ending in `CBRANCH`, where the CBRANCH boolean is an
   `INT_EQUAL`/`INT_NOTEQUAL` whose operands both derive from a `LOAD` of `<segment> + 0x28`
   (the canary compare);
2. removes the fail edge with the stock `Funcdata::removeBranch` (converts the CBRANCH to a
   fall-through to the ok-block and patches MULTIEQUALs), then `removeUnreachableBlocks` collects
   the now-orphaned `__stack_chk_fail` block. The dead canary load/store/compare are reaped by the
   next deadcode pass in the repeating fullloop.

Inert when the option is off (default output byte-identical). Mirrors angr's `StackCanarySimplifier`.
