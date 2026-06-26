# Analysis — test_ifelseflatten_iprule :: flush_rule (angr 9.2.213)

Binary: `/home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/iprule.o` (ET_REL `.o`, x86-64)
Function: `flush_rule` @ file offset `0x1310`, size 253 bytes (`0x1310`–`0x140d`).
angr method: `test_ifelseflatten_iprule` (structuring algo `sailr`).

## What the angr test asserts

The first `if` in `flush_rule` must be a **single-scope early return**:
`if (...) return -1;` with **no `else`** and the return value `-1`. This is produced by angr's
`IfElseFlattener` region simplifier
(`angr/analyses/decompiler/region_simplifiers/ifelse.py`): when a `ConditionNode`'s
true-branch is *statement-terminating* (all endpoints return/abort) and the false-branch is
not, it drops the `else` and re-parents the false-node as a **follower** of the `if`. The net
effect is the classic early-return flattening:

```
if (cond) { ...; return X; } else { body }   ==>   if (cond) { ...; return X; } body
```

angr's rendering of `flush_rule`:

```c
unsigned int flush_rule(struct_0 *ptr) {
    ...
    if (ptr->field_0 - 28 < 0)
        return 4294967295;          // -1, single-scope early return  <-- the assertion
    parse_rtattr(&v1, 24, &ptr[1].padding_8[3]);
    ...
}
```

## What kuna produces (and the real, dominant gap)

kuna does **not** render a clean `flush_rule`. The decompiled body of `flush_rule` **merges
in the entire neighbouring function** `iprule_list_flush_or_save` (0x1410, 2059 bytes) — the
output contains `__isoc99_sscanf`, `matches(...)`, `rtnl_ruledump_req`, `new_json_obj`,
`rtnl_dump_filter_nc`, and dozens of `label_4018xx` blocks that belong to the neighbour, not
to `flush_rule`. (15 neighbour-only call sites leak into the body.)

### Root cause of the merge — fall-through past `call __stack_chk_fail`

`flush_rule` is self-contained in the raw disassembly: every `jcc`/`jmp` stays inside
`0x1310`–`0x1408`, there is a `ret` at `0x13f3`, and the canary-fail path ends at

```
1408: e8 00 00 00 00   call __stack_chk_fail   ; R_X86_64_PLT32 __stack_chk_fail-0x4
140d: 0f 1f 00         nopl (%rax)              ; padding
1410: <iprule_list_flush_or_save begins>
```

`__stack_chk_fail` is no-return, so flow should stop after `0x1408`. But in this ET_REL `.o`:
1. the call is encoded `e8 00 00 00 00` (rel32 = 0) with the target supplied **only** by the
   `R_X86_64_PLT32` relocation, so without relocation resolution kuna does not even know the
   callee is `__stack_chk_fail`; and
2. the no-return fact is not propagated to the flow-follower, so kuna assumes the call returns,
   continues at `0x140d`, walks through the `nopl` padding, and **decodes straight into
   `iprule_list_flush_or_save` at `0x1410`** — merging the two functions.

kuna already *has* the pieces (`relocobjects` loads ET_REL; the known-no-return list normalizes
`__stack_chk_fail` → `stack_chk_fail`; `noreturn_disc`/`noreturn_propagate` exist), but with all
of them enabled via the CLI the merge persists (15 leaks unchanged), and the full
`scripts.pipeline.compare` run reproduces the merge too. So the no-return fact is **not reaching
the ET_REL relocated call site** before the function's extent is decoded.

### Secondary gap — the if-else-flatten itself

Even setting the merge aside, the `flush_rule` *portion* of kuna's output structures the entry
condition as `if (field_0 >= 0x1c) { body } ... return -1` rather than angr's inverted
early-return `if (field_0 - 28 < 0) return -1; body`. Producing angr's form requires porting the
`IfElseFlattener` region simplifier — an **S7/S8 structured-tree transform**, not a peephole.

## Owning stages

- The merge: **S1 loader / analysis-tier** (ET_REL relocation resolution + no-return propagation
  to relocated call sites) — `kuna-analysis` `s1_loader::noreturn` / `s1_noreturn_propagate` +
  `relocobjects`. Not modelable as one S2 `Action`/`Rule`.
- The early-return form: **S7/S8 structuring** (`s8_structure` / region simplifiers) — a port of
  angr `IfElseFlattener`.

## Conclusion (scope)

This opportunity is **mis-framed as a single if-else-flatten Action/Rule** and is in reality two
entangled, large gaps:

1. an **ET_REL function-boundary merge** (no-return fact not propagated to a relocation-resolved
   `call __stack_chk_fail`), which alone prevents `flush_rule` from ever matching angr; and
2. a **structural if-else-flatten** region simplifier (S7/S8) port.

Neither is a single option-gated `Action`/`Rule` (Hard rule 7: touching S7 structuring/region
code, and a new analysis-tier pass type, are both *large*), and they are entangled so that fixing
only the structurer would still leave `flush_rule` merged. See `proposal.md`.
