---
case_id: O2-noinline-bzip2-bzip2recover-bsGetBit
status: already-fixed
gap_survives: false
recorded_kuna_ged: 72
divergence_lever: null
proposed_new_option: null
runtime_choice: false
angr_feature: null
---
## Current kuna vs ghidra

Current kuna (`decompile-all --addr 0x1be0`, F1 listing default ON): 18 code lines,
0 gotos, 0 labels, 3 ifs, 0 loops, 3 returns.

```c
uint4 sub_1be0(void *a0)

{
  int4 v1; // eax
  
  if (1 <= *(int4 *)((int8)a0 + 0xc)) {
    v1 = *(int4 *)((int8)a0 + 0xc) + -1;
    *(int4 *)((int8)a0 + 0xc) = v1;
    return *(int4 *)&a0[1] >> ((uint1)v1 & 0x1f) & 1;
  }
  v1 = getc(*a0);
  if (v1 == -1) {
    if (*(int4 *)__errno_location() == 0) {
      return 2;
    }
                    /* WARNING: Subroutine does not return */
    sub_1ab0();
  }
  *(int4 *)&a0[1] = v1;
  *(void *)((int8)a0 + 0xc) = 7;
  return v1 >> 7 & 1;
}
```

Ghidra: 18 code lines, 0 gotos, 0 labels, 3 ifs, 0 loops, 3 returns.

```c
uint bsGetBit(undefined8 *param_1)

{
  int *piVar1;
  int iVar2;
  
  if (0 < *(int *)((long)param_1 + 0xc)) {
    iVar2 = *(int *)((long)param_1 + 0xc) + -1;
    *(int *)((long)param_1 + 0xc) = iVar2;
    return *(int *)(param_1 + 1) >> ((byte)iVar2 & 0x1f) & 1;
  }
  iVar2 = getc((FILE *)*param_1);
  if (iVar2 == -1) {
    piVar1 = __errno_location();
    if (*piVar1 == 0) {
      return 2;
    }
                    /* WARNING: Subroutine does not return */
    FUN_00101ab0();
  }
  *(int *)(param_1 + 1) = iVar2;
  *(undefined4 *)((long)param_1 + 0xc) = 7;
  return iVar2 >> 7 & 1;
}
```

The two are structurally identical — same guard, same early return in the guarded
block, same `getc` / `iVar2 == -1` / nested errno guard / early `return 2`, same
`/* WARNING: Subroutine does not return */` noreturn cut on `sub_1ab0` /
`FUN_00101ab0`, same two field stores and final shifted return. Remaining diffs are
pure rendering: type spellings (`uint4/int4/int8/uint1` vs `uint/int/long/byte`),
variable names, `1 <= x` vs `0 < x`, and kuna inlining `__errno_location()` where
ghidra spills it to `piVar1` (one statement). None of these change the CFG.

## Divergence experiment

The recorded GED-72 gap was produced by OLD kuna 0.1.0, which decompiled without the
listing enabled, so the internal noreturn wrapper `sub_1ab0` (bzip2's `panic`/error
abort) was NOT concluded no-return. I reproduced that stale state on current kuna with
`--option noreturn_propagate off`:

```c
uint8 sub_1be0(void *a0)
{
  ...
  sub_1ab0();
  if (*(char *)&v6[2] == 'w') {          // <-- entire bsClose body falls through
    ...
    do { v4 = v4 + 1; v5 = v5 * 2; } while (v4 != 8);   // loop
    ...
    if (putc(v5 & 0xff,v1) == -1) goto label_1cc0;      // goto + label
    ...
  }
  ...
  /* WARNING: tailcalljump: recovered tail call -> 0x00001160 */
  free(v6);
  return v2;
}
```

With `noreturn_propagate off` the function balloons to ~50 lines with a do/while loop,
a `goto label_1cc0`, and code from the following functions (bsClose, plus a
strlen/`.bz2` suffix check) swallowed in via the un-cut fall-through after `sub_1ab0()`.
That inflated, wrong-CFG shape is exactly the GED-72 the benchmark recorded.

Flipping any divergence lever OFF only *opens* the gap — the default-ON configuration
is the one that matches ghidra. No lever needs flipping.

## Analysis / runtime-choice verdict

Root cause: **F1 (decompile-all now enables the listing so `noreturn_propagate`
collapses swallowed no-return tails)**. Old kuna 0.1.0 ran the benchmark without the
listing, so `sub_1ab0` was not concluded no-return; the fall-through past it merged the
whole next function (bsClose et al.) into bsGetBit, inflating GED to 72. Current kuna
(listing default ON, `noreturn_propagate` default-ON) cuts the tail at `sub_1ab0()`
exactly like ghidra, yielding a byte-for-CFG-identical structure. The gap is closed;
this is already-fixed, not a runtime choice or a bug.
