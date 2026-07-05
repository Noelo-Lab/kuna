---
case_id: O2-noinline-openssh-portable-sshd-session_close_single_x11
status: angr-feature-pending
gap_survives: true
recorded_kuna_ged: 58
divergence_lever: null
proposed_new_option: null
runtime_choice: false
angr_feature: noreturn-robustness
---

## Current kuna vs ghidra

The GED-58 gap **survives on current kuna** (F1 listing default on, `noreturn_propagate`
default on). kuna's `session_close_single_x11` is the correct ghidra body **plus the entire
next function `session_tty_list @ 0x2d420` swallowed onto its tail**. The swallow is
byte-for-byte identical to ghidra's standalone `session_tty_list` (the `DAT_00225540` /
`strncmp("/dev/",..)` / `strrchr` / `notty` block).

Ghidra (`session_close_single_x11 @ 0x2d200`, ~44 LOC, 1 `if`, 1 `while`, 0 gotos):

```c
undefined8 session_close_single_x11(undefined8 param_1,ulong param_2)
{
  ...
  lVar2 = FUN_0012aa90(param_2 & 0xffffffff);
  if (lVar2 == 0) {
                    /* WARNING: Subroutine does not return */
    FUN_0017a7b0("session.c","session_close_single_x11",0x92a,1,1,0,"no x11 channel %d",
                 param_2 & 0xffffffff);          // <-- sshfatal, concluded NO-RETURN -> flow stops
  }
  __ptr = *(int **)(lVar2 + 0xc0);
  ...                                            // loop + free-chain
  free(*(void **)(lVar2 + 0xa0));
  *(undefined8 *)(lVar2 + 0xa0) = 0;
  return 0;
}                                                // <-- function ENDS here; 0x2d420 is a new fn
```

Current kuna (`sub_2d200`, ~66-75 LOC, extra `do/while` + 3 extra `if`, 1 branchflip):

```c
unsigned long sub_2d200(unsigned long a0,int4 a1)
{
  ...
  v3 = sub_2aa90(a1);
  if (v3 != 0) {                                 // ghidra's guard, branch-flipped
    ...                                          // loop + free-chain  (CORRECT)
    return 0;
  }
  sub_7a7b0("session.c","session_close_single_x11",0x92a,1,1,0);  // sshfatal -- NOT no-return
  v2 = 0;                                        // <-- FALL-THROUGH into session_tty_list
  v6 = '\0';
  dat_125540 = '\0';
  if (0 < dat_125970) {                          // ---- all of this is session_tty_list @0x2d420
    do {
      ... strncmp((char*)v1,"/dev/",5) ... strrchr ...
    } while (v2 < dat_125970);
    if (v6 != '\0') { return 0x125540; }
  }
  sub_c9bb0(0x125540,0xd6c49,0x400);
  return 0x125540;
}
```

(Note also: kuna's decl list is missing `v5` although `v5` is used throughout — a cosmetic
`dedupvardecls` artifact, secondary to the swallow.)

## Divergence experiment

The symptom is a no-return overrun (cold `fatal` path falls through and merges the next
function), so I flipped the noreturn / call-tail levers OFF. **None closes the gap** —
`session_tty_list` is still swallowed in every variant:

| lever flipped off | swallow gone? | LOC |
|---|---|---|
| `noreturn_propagate` off | no | ~70 |
| `noreturn_extern` off | no | 75 |
| `noreturn_externmatch` off | no | 75 |
| `foldcallret` off | no | 77 |
| `tailcalljump` off | no | 75 |

This is expected: the defect is a **missing no-return conclusion**, not an over-eager
default-on transform. Levers flipped OFF only *revert* transforms; they cannot *add* the
no-return fact that would terminate the flow. So the winning lever is **none**.

## Analysis / runtime-choice verdict

Root cause is a **transitive internal-no-return chain that kuna never concludes**, verified
against the raw bytes (`objdump`):

- `_exit@plt` (0xcbf0) — known no-return leaf.
- `cleanup_exit` (0x12e40) — every path ends in `call _exit@plt` (0x12e92), with a
  `jmp 12e90` retry loop around it; it never returns.
- `sshfatal` (0x7a7b0) — saves varargs, `call 7eaa0` (do_log), `call 12e40`
  (cleanup_exit), then **no `ret`** — just `nopl` padding and the next function's `endbr64`.
  No-return because it tail-calls cleanup_exit.
- `session_close_single_x11` (0x2d200) — calls `sshfatal` in the "no x11 channel" cold
  path; the flow should terminate there.

Ghidra walks this chain (`_exit` -> `cleanup_exit` -> `sshfatal`) and prints
`WARNING: Subroutine does not return` at the call site, so it ends the function at
`return 0` and `session_tty_list @ 0x2d420` becomes its own function. kuna, even with
F1's listing enabled and `noreturn_propagate` default-on, does **not** conclude
`cleanup_exit`/`sshfatal` are no-return, so it decodes straight through the `call sshfatal`
into 0x2d420's bytes and merges the two functions — inflating the GED by 58.

The same bug recurses one level down and is directly observable: kuna's own decode of
`sub_7a7b0` (sshfatal) *itself* swallows `record_hostkey @ 0x7a850` after its
`call cleanup_exit`, for the identical reason. So the failing analysis is the intermediate
wrapper `cleanup_exit` (0x12e40): its no-return-ness requires reasoning that **all** paths
end in `_exit@plt` despite a back-edge loop (`jmp 12e90`) and cold error-handling tail. That
is exactly the pending **noreturn-robustness** shape ("internal no-return wrapper still not
concluded no-return even with listing on — looping / cold-tail / reach cases").

Not a runtime choice and not a divergence-lever case: kuna is uniquely worse here purely
because an upstream/robustness analysis is incomplete, and no default-off flip would recover
ghidra's form (you cannot toggle *on* a conclusion the analysis never reached). The fix is
strengthening kuna's no-return conclusion to be transitive through wrappers whose only
exit is a call to a no-return function even in the presence of a loop-back / cold tail — the
`noreturn-robustness` session-2 angr feature. Until then the gap stands.
