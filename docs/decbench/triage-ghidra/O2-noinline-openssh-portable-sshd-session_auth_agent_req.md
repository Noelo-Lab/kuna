---
case_id: O2-noinline-openssh-portable-sshd-session_auth_agent_req
status: angr-feature-pending
gap_survives: true
recorded_kuna_ged: 47
divergence_lever: null
proposed_new_option: null
runtime_choice: false
angr_feature: noreturn-robustness
---

## Current kuna vs ghidra

Address confusion resolved first: ghidra loaded the image at base `0x100000`, so its
`FUN_00186cd0` / `DAT_00223458` / `FUN_00188250` are the real `0x86cd0` / `0x123458` /
`0x88250` (kuna's `sub_86cd0` etc.). Same functions — kuna's call-target resolution is
correct. The `// Function: ... @ 0x2c310` marker is decbench normalizing the entry back to
file-relative; the `FUN_*` names in the body keep the +0x100000 image base.

### ghidra (clean, ~15 LOC body, 2 ifs, 0 loops, 0 gotos)
```c
undefined8 session_auth_agent_req(undefined8 param_1,long param_2)
{
  int iVar1;
  undefined8 uVar2;
  iVar1 = FUN_00188250();
  if (iVar1 != 0) {
                    /* WARNING: Subroutine does not return */
    FUN_00186cd0(param_1,iVar1,"%s: parse packet","session_auth_agent_req");
  }
  if ((*(int *)(DAT_00223458 + 4) != 0) && (DAT_00223a24 != 0)) {
    if (DAT_00225940 != 0) { return 0; }
    DAT_00225940 = 1;
    uVar2 = FUN_0012bfc0(param_1,*(undefined8 *)(param_2 + 0x10));
    return uVar2;
  }
  FUN_0017ed20("session.c","session_auth_agent_req",0x88a,1,5,0,"agent forwarding disabled");
  return 0;
}
```
Ghidra marks the `sshpkt_fatal` call (`FUN_00186cd0`) no-return, so the `iVar1 != 0` arm
terminates and the function ends cleanly. GED 0 vs source.

### current kuna (67 LOC, 7 ifs, 3 loops, 0 gotos) — MERGED with `session_env_req`
```c
unsigned long sub_2c310(unsigned long a0,int8 a1)
{
  ...
  v1 = sub_88250();
  if (v1 == 0) {
    if ((*(int4 *)(dat_123458 + 4) != 0) && (dat_123a24 != 0)) {
      if (dat_125940 == 0) {
        v4 = *(void *)(a1 + 0x10);
        dat_125940 = 1;
        return sub_2bfc0(a0,v4);          /* tailcalljump */
      }
      return 0;
    }
    v11 = 0x2c414;
    sub_7ed20("session.c","session_auth_agent_req",0x88a,1,5,0);
    return 0;
  }
  ...
  sub_86cd0(a0,v6,"%s: parse packet","session_auth_agent_req");  /* NOT treated no-return */
  v9 = *(void *)(v7 + 0x28);              /* <-- falls through into session_env_req body */
  ...                                     /* env-var loop, sub_7f0f0, sub_8e8d0 ... */
  sub_7ed20("session.c","session_env_req",v4,0,6,0);
  free(v8);
  free(v5);
  return 0;
  ...
  sub_86cd0(v4,v2,"%s: parse packet","session_env_req");
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}
```
The whole body of `session_env_req` (@0x2c4e0) — the `SSH_ENV`/`accept_env` loop,
`free(v8)`, `free(v5)`, `__stack_chk_fail()` — is swallowed into `sub_2c310`. 67 vs ~15
lines; that inflation is the GED 47.

## Divergence experiment

The symptom is a no-return overrun / adjacent-function merge. Tested the two relevant
default-on levers with a single `--addr 0x2c310` decompile:

| lever flipped OFF | LOC | still merged (`session_env_req` present)? |
|---|---|---|
| (default, all on) | 67 | yes |
| `noreturn_propagate off` | 67 | yes |
| `noreturn_extern off` | 67 | yes |

**No lever closes the gap** — and it can't. All 19 levers are default-*on* angr features;
flipping one *off* can only remove behavior, never *add* a no-return conclusion. `noreturn_extern`
/ `noreturn_externmatch` apply to UND/PLT externs — this is an *internal* fatal wrapper.
`noreturn_propagate` propagates a conclusion but requires the callee to be concluded no-return
first, which is exactly the step that fails here. Winning lever: **none**.

## Analysis / runtime-choice verdict

Root cause is a **cascading internal no-return recognition failure**, three deep:

1. `0x869f0` (the `fatal`/`sshfatal` core) has **zero `ret`** in `0x869f0..0x86cd0`. It ends
   in `notrack jmp *%rax` (0x86a6e — an indirect tail-jump into a cleanup/`cleanup_exit`→`_exit`
   handler) and contains a backward loop (`jmp 86aea`). It is genuinely no-return, but its
   terminator is an **indirect jump**, not a `ret` and not a `call` to a syntactically-known
   noreturn (`exit`/`abort`). Kuna's noreturn detector doesn't conclude no-return through the
   indirect tail-jump, so `sub_869f0` is left as returning.
2. `0x86cd0` = `sshpkt_fatal` (variadic: reg-saves + va_list, then `call 869f0` at 0x86d60
   with NOP padding, no `ret`). Because kuna thinks `sub_869f0` returns, `sub_86cd0` "falls
   through" the NOP pad into the *next* function at `0x86d70` (a socket-write) and is
   decompiled as a returning function — so `sub_86cd0` itself is never concluded no-return.
3. `session_auth_agent_req`'s `sub_86cd0(...,"%s: parse packet",...)` call is therefore not
   no-return, so the error arm doesn't terminate; flow overruns and merges the adjacent
   `session_env_req` (@0x2c4e0). GED 47.

Ghidra concludes all three no-return (it resolves the indirect tail-jump / reach analysis),
gets GED 0. This is **not** a runtime choice and **not** a divergence: kuna's default output
is simply missing analysis ghidra has — but it maps exactly to a known, enumerated pending
session-2 angr feature, `noreturn-robustness` ("internal no-return wrapper still not concluded
no-return even with listing on — looping/cold-tail/reach cases"). The tell here is the
*indirect tail-jump + backward loop with no `ret`* terminator of the fatal core. Once kuna's
noreturn analysis is robust to that shape, `sub_869f0`→`sub_86cd0`→`session_auth_agent_req`
all conclude no-return (with `noreturn_propagate`, on by default under F1's listing) and the
`session_env_req` merge collapses, converging to ghidra's ~15-line form.

Not `genuine-bug`: a specific pending feature (`noreturn-robustness`) covers it, rather than
"no divergence explains it." Not a metric-artifact: the source CFG is real and the inflation
is a true structural over-extension, not a rendering/`!= 0` artifact.
