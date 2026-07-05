---
case_id: O2-noinline-openssh-portable-sshd-sshd_hostkey_sign
status: angr-feature-pending
gap_survives: true
recorded_kuna_ged: 73
divergence_lever: null
proposed_new_option: null
runtime_choice: false
angr_feature: noreturn-robustness
---

## Current kuna vs ghidra

Ghidra: clean ~45 LOC. It concludes the internal log-and-die wrapper `FUN_0017a7b0`
(= source `sshfatal`) is no-return — every call site carries
`/* WARNING: Subroutine does not return */` — so each of the four branches terminates and
the function ends at `return 0`.

```c
undefined8
sshd_hostkey_sign(long param_1,long param_2,undefined8 param_3, ... )
{
  undefined4 uVar1;
  int iVar2;
  undefined8 uVar3;

  uVar1 = *(undefined4 *)(param_1 + 0x83c);
  if (DAT_00223018 == 0) {
    if (param_2 == 0) {
      iVar2 = FUN_001648a0(DAT_00223024,param_3,param_4,param_5,param_6,param_7,param_8);
      if (iVar2 != 0) {
        uVar3 = FUN_00150a60(iVar2);
                    /* WARNING: Subroutine does not return */
        FUN_0017a7b0("sshd.c","sshd_hostkey_sign",0x93d,1,1,uVar3,"agent sign failed",uVar3);
      }
    }
    else {
      iVar2 = FUN_0015ddd0(param_2, ... ,uVar1);
      if (iVar2 < 0) {
                    /* WARNING: Subroutine does not return */
        FUN_0017a7b0("sshd.c","sshd_hostkey_sign",0x938,1,1,0,"privkey sign failed",param_4);
      }
    }
  }
  else if (param_2 == 0) {
    iVar2 = FUN_0013f110(param_1,param_3, ... ,uVar1);
    if (iVar2 < 0) {
                    /* WARNING: Subroutine does not return */
      FUN_0017a7b0("sshd.c","sshd_hostkey_sign",0x932,1,1,0,"pubkey sign failed",param_1);
    }
  }
  else {
    iVar2 = FUN_0013f110(param_1,param_2, ... ,uVar1);
    if (iVar2 < 0) {
                    /* WARNING: Subroutine does not return */
      FUN_0017a7b0("sshd.c","sshd_hostkey_sign",0x92d,1,1,0,"privkey sign failed",param_6);
    }
  }
  return 0;
}
```

kuna (current main, `decompile-all --addr 0x11230`, F1 listing default-ON): **153 LOC, 3 gotos,
3 labels**, and it *overruns the function boundary* — `sub_7a7b0` (the same `sshfatal` wrapper)
is treated as **returning**, so control falls through past every fatal call into the adjacent
function's body (`check_ip_options` @ 0x11490): `getpeername`/`getsockopt`, a `__snprintf_chk`
loop, the `"Connection from %.100s ..."` string, and a trailing `__stack_chk_fail()`. The tail
references undeclared locals (`v5`, `v13`, `v14`, `v15`, `v16`, `v19`) — invalid C.

```c
uint8 sub_11230(int8 a0,char *a1, ... )
{
  ... 20+ locals ...
  v4 = (uint8)*(uint4 *)(a0 + 0x83c);
    /* WARNING: branchflip: flipped negated guard for linearity ... */
  if (dat_123018 != 0) {
    ...
      v4 = sub_7a7b0("sshd.c","sshd_hostkey_sign",0x92d,1,1,0);
      goto label_112d0;              // <-- sshfatal treated as RETURNING
    ...
    v4 = sub_7a7b0("sshd.c","sshd_hostkey_sign",0x932,1,1,0);
label_113f0:
    *(uint8 *)((int8)v9 + -8) = v4;  // falls through into next function's frame setup
    ...
  }
  ...
  sub_7a7b0("sshd.c","check_ip_options",0x5a0,0,1,0);   // <-- spilled into check_ip_options
  goto label_11668;
  ...
  v3 = getpeername(v3,&v10[-0x408],&v10[-0x40c]);        // <-- adjacent function body
  ...
label_11668:
    /* WARNING: Subroutine does not return */
  *(void *)&v11[-8] = 0x1166d;
  __stack_chk_fail();
}
```

Source (O0 `sshd.i`, `sshd_hostkey_sign`): four branches, each ending in `sshfatal(...)` on the
error path, then `return 0`. `sshfatal` is a no-return log-and-die wrapper (logs, then
`cleanup_exit`/`exit`).

## Divergence experiment

The symptom is a **no-return overrun** (a no-return call rendered as returning), so the only
relevant levers are the ones that *add* no-return knowledge; flipping them OFF can only make the
overrun worse or leave it unchanged.

- `--option noreturn_propagate off`  ->  **153 LOC, identical overrun.** No change: kuna never
  concluded `sub_7a7b0` no-return in the first place, so turning propagation off does nothing.
- No `--option X off` among the 19 default-ON divergences reduces the gap — the fix is *more*
  no-return knowledge, not reverting a divergence. `branchflip` fires cosmetically at the top
  but is orthogonal to the overrun.

Winning lever: **none.** The F1 listing default is already ON in `decompile-all` (that is the
benchmark surface here), yet the gap survives it.

## Analysis / runtime-choice verdict

Root cause: kuna fails to conclude the **internal** function `sub_7a7b0` (= `sshfatal`,
FUN_0017a7b0) is no-return. `sshfatal` is a multi-level wrapper — it logs and then reaches
`cleanup_exit`/`exit`, so its no-return-ness must be discovered by analyzing `sshfatal` (and the
`cleanup_exit` it tail-calls) and propagating. Ghidra's flow analysis reaches that conclusion and
marks every call site "Subroutine does not return"; kuna's analysis does not, even with the
listing enabled and `noreturn_propagate` default-ON. Because the four `sshfatal` calls are then
treated as returning, control falls straight through the function epilogue into the next
function's body (`check_ip_options` @ 0x11490), producing the 153-line invalid-C overrun that
scores GED 73 against ghidra's 45-line 0.

This is not a divergence-lever case: no default-ON angr option, flipped OFF, recovers ghidra's
shape — flipping the no-return levers off leaves the overrun byte-identical. It is not a
metric-artifact: the source CFG is a real four-branch tree and kuna's output is genuinely wrong
(undeclared variables, wrong function boundary). It is not a plain default-fixable bug in the
usual sense either — it is exactly the gap the pending session-2 **`noreturn-robustness`** angr
feature targets: *"internal no-return wrapper still not concluded no-return even with listing on"*
(the reach/cold-tail/multi-level-wrapper case). Once kuna robustly concludes internal
log-fatal wrappers like `sshfatal` are no-return, `noreturn_propagate` (already default-ON) will
collapse each branch and the overrun disappears, converging on ghidra's form.
