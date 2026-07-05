---
case_id: O2-noinline-iproute2-ip-check_duparg
status: already-fixed
gap_survives: false
recorded_kuna_ged: 116
divergence_lever: null
proposed_new_option: null
runtime_choice: false
angr_feature: null
---
## Current kuna vs ghidra

Both are structurally identical: 11 LOC, 1 `if`, 1 early `return`, 1 no-return call
(`duparg2`) with the `WARNING: Subroutine does not return` comment. 0 gotos, 0 labels,
0 loops, 0 switch on both sides. The only differences are cosmetic type spelling
(`uint4 *`/`uint8`/`uint1` vs `uint *`/`ulong`/`byte`) and variable names — both
normalized away by the Joern-stripped GED view.

Current kuna (`decompile-all --addr 0x688b0`, F1 listing default on):
```c
void sub_688b0(uint4 *a0,uint4 a1,unsigned long a2,unsigned long a3)

{
  if ((*a0 >> ((uint8)a1 & 0x3f) & 1) == 0) {
    *a0 = *a0 | (uint4)(1L << ((uint1)a1 & 0x3f));
    return;
  }
                    /* WARNING: Subroutine does not return */
  duparg2(a2,a3);
}
```

Ghidra:
```c
void check_duparg(uint *param_1,uint param_2,undefined8 param_3,undefined8 param_4)

{
  if ((*param_1 >> ((ulong)param_2 & 0x3f) & 1) == 0) {
    *param_1 = *param_1 | (uint)(1L << ((byte)param_2 & 0x3f));
    return;
  }
                    /* WARNING: Subroutine does not return */
  duparg2(param_3,param_4);
}
```

Source (`iplink_geneve.i`) confirms the shape — guarded early-return, then a no-return
`duparg2`:
```c
static void check_duparg(__u64 *attrs, int type, const char *key, const char *argv)
{
 if (!(((*attrs) & (1L << (type))) != 0)) {
  *attrs |= (1L << type);
  return;
 }
 duparg2(key, argv);   /* no-return */
}
```

## Divergence experiment

Not a "flip-off closes the gap" case — the current default already matches ghidra. To
reproduce the recorded GED-116 output (the OLD kuna 0.1.0 the benchmark ran, before F1)
I flipped the responsible lever OFF:

- `--option noreturn_propagate off` -> kuna no longer concludes `duparg2` is no-return,
  falls through past the call, and pulls in a ~90-line overrun from the following
  function (the gtp/geneve parse loop: `strcmp` chains, `addattr*`, `__stack_chk_fail`,
  labels `label_68bb3`/`label_68bb8`/...). That inflated body is exactly what scored 116.

No lever needs flipping to reach ghidra's form on current kuna — the default is correct.

## Analysis / runtime-choice verdict

Root cause of the stale gap: the benchmark ran kuna 0.1.0 with `decompile-all` not
enabling the disassembly listing, so `noreturn_propagate` could not conclude the internal
wrapper `duparg2` (which tail-calls the exit path) was no-return. Kuna therefore treated
the `duparg2` call as returning, decompiled the fall-through, and absorbed the next
function's body — an ~11-line function ballooning to ~90 lines of unrelated control flow,
GED 116.

Session-1's F1 fix makes `decompile-all` (the benchmark surface) enable the listing by
default, so `noreturn_propagate` now runs and correctly marks `duparg2` no-return. Current
kuna collapses to the exact ghidra shape (11 LOC, one guard, no overrun). This is a plain
already-fixed case attributable to F1 (listing-on so `noreturn_propagate` is effective);
the gap does not survive on current main. No runtime choice, no pending angr feature, no
metric artifact.
