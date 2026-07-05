---
case_id: O2-noinline-diffutils-diff-sip
status: angr-feature-pending
gap_survives: true
recorded_kuna_ged: 347
divergence_lever: null
proposed_new_option: null
runtime_choice: false
angr_feature: noreturn-robustness
---
## Current kuna vs ghidra

Current kuna default (F1 listing surface, all 19 divergences on): **99 LOC, 13 ifs, 1 while, 1 do-loop, 1 goto**.
Ghidra: **43 LOC, 4 ifs, 1 else, 0 gotos**.

The divergence is entirely one spot: the `if (_Var3 < 0)` branch after `lseek`, which calls the
internal wrapper `FUN_0010e1f0` = `pfatal_with_name` (0xe1f0). Ghidra concludes that call does not
return and truncates; kuna does not, and decompiles ~56 lines of dead fall-through as if reachable.

Ghidra (clean — the whole `_Var3 < 0` arm is a single no-return call):
```c
bool sip(int *param_1,char param_2)
{
  ...
    if (param_2 == '\0') {
      iVar1 = thunk_FUN_0010f700(*param_1,0);
      FUN_0010bf50(param_1);
      __n = *(size_t *)(param_1 + 0x2c);
      if (iVar1 != 0) {
        _Var3 = lseek(*param_1,-__n,1);
        if (_Var3 < 0) {
                    /* WARNING: Subroutine does not return */
          FUN_0010e1f0(*(undefined8 *)(param_1 + 2));
        }
        param_1[0x2c] = 0;
        param_1[0x2d] = 0;
        *(undefined1 *)((long)param_1 + 0x121) = 0;
      }
      pvVar4 = memchr(*(void **)(param_1 + 0x28),0,__n);
      return pvVar4 != (void *)0x0;
    }
  ...
}
```

Current kuna (inflated — sub_e1f0 is NOT no-return, so the fall-through blob is decompiled):
```c
        if ((int8)lseek(v1,-v3,1) <= -1) {
          v6 = *(uint4 **)&a0[2];
          sub_e1f0();                       // <-- ghidra: no-return; kuna: keeps going
          if ((int4)*v6 < 0) {
            return (uint8)*v6;
          }
          if ((v6[10] & 0xf000) != 0x8000) { ... }
          else {
            v4 = *(uint8 *)&v6[0x10];
            ...
            if ((v5 < v4) || (0x7ffffffffffffffe < v5)) {
              sub_16300();                  // taildup'd no-return tail
            }
            ...
            do { ... } while (v5 == v4);    // ~40 more lines of dead fall-through
          }
          ...
          return v4;
        }
        a0[0x2c] = 0; ...                    // ghidra's post-if cleanup, now the else-arm
```

## Divergence experiment

Symptom is a no-return overrun, so the candidate levers are the noreturn family — all default-ON:

| lever flipped OFF | sip LOC | effect |
|---|---|---|
| (default, all on)      | 99  | baseline gap |
| noreturn_propagate off | 471 | **much worse** |
| noreturn_extern off    | 99  | no change |
| noreturn_externmatch off | 99 | no change |

No lever closes the gap toward ghidra. `noreturn_propagate` is already on and already helping —
turning it off explodes sip to 471 LOC, proving the propagation is the only thing keeping the
overrun to 56 lines instead of hundreds. The remaining gap is upstream of propagation: the
wrapper `pfatal_with_name` is never *concluded* no-return in the first place, so there is no fact
to propagate. Winning lever: **none**.

## Analysis / runtime-choice verdict

Root cause: **kuna fails to conclude the internal wrapper `pfatal_with_name` (sub_e1f0) is
no-return.** That wrapper's tail is `error(2, errno, "%s", name)` — glibc `error()` with a
non-zero status calls `exit()`, so the call (and the whole wrapper) never returns. Ghidra's
discovered-no-return analysis flags the `error(2,...)` call ("Subroutine does not return"),
concludes `pfatal_with_name` is non-returning, and propagates that to every call site (sip, and
11 other callers), truncating the fall-through. kuna's own decompile of sub_e1f0 confirms the
miss: it emits code *after* the `error(2,...)` call plus a trailing `return;`, i.e. it walked the
fall-through bytes into the next function and found a reachable RET, so it decided the wrapper
returns.

Because pfatal_with_name is treated as returning, at sip's `_Var3 < 0` site kuna keeps decoding
the bytes after the call (the cold error path / next-function fall-through) as live code — the
`v6` blob with its `do…while` and extra branches — inflating 43 LOC → 99 LOC and driving the
GED to 347.

This is **not** a runtime choice. kuna's correct behavior here matches ghidra AND the source
(pfatal_with_name genuinely never returns); there is no majority of functions where treating it
as returning is better. It is a robustness gap in no-return conclusion, exactly the pending
angr feature `noreturn-robustness`: an internal no-return wrapper (tail-calling a
conditionally-exiting libc primitive like `error(status!=0,…)`) is still not concluded no-return
even with the listing on and `noreturn_propagate` active. Closing that — recognizing
`error`/`error_at_line` with a non-zero status as no-return, or a stronger discovered-no-return
pass that stops the fall-through walk at a call with no reachable RET of its own — establishes the
fact that `noreturn_propagate` then carries to sip and its 11 sibling call sites, collapsing the
inflated blob to ghidra's shape.
