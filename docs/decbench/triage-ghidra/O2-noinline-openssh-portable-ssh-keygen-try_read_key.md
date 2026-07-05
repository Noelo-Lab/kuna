---
case_id: O2-noinline-openssh-portable-ssh-keygen-try_read_key
status: already-fixed
gap_survives: false
recorded_kuna_ged: 46
divergence_lever: null
proposed_new_option: null
runtime_choice: false
angr_feature: null
---
## Current kuna vs ghidra

Current kuna (`decompile-all --addr 0xc4d0`, F1 listing default ON) — body 12 lines, 2 ifs, 0 gotos, 0 labels, 0 loops, 0 switch:

```c
int8 sub_c4d0(unsigned long a0)
{
  int8 v1; // rax

  v1 = sub_22ca0(0xe);
  if (v1 == 0) {
                    /* WARNING: Subroutine does not return */
    sub_3b8d0("ssh-keygen.c","try_read_key",0x379,0,1,0);
  }
  if (sub_27180(v1,a0) == 0) {
    return v1;
  }
  sub_22830(v1);
  return 0;
}
```

Ghidra — body 12 lines, 2 ifs, 0 gotos, 0 labels, 0 loops, 0 switch:

```c
long try_read_key(undefined8 param_1)
{
  int iVar1;
  long lVar2;

  lVar2 = FUN_00122ca0(0xe);
  if (lVar2 == 0) {
                    /* WARNING: Subroutine does not return */
    FUN_0013b8d0("ssh-keygen.c","try_read_key",0x379,0,1,0,"sshkey_new failed",0);
  }
  iVar1 = FUN_00127180(lVar2,param_1);
  if (iVar1 == 0) {
    return lVar2;
  }
  FUN_00122830(lVar2);
  return 0;
}
```

Source (`ssh-keygen.i:22243`) is the same three-block shape: `sshkey_new` → `if NULL sshfatal()` (noreturn) → `if (sshkey_read()==0) return ret` → `sshkey_free; return NULL`.

Structurally the two are identical. The only nits are cosmetic: ghidra materializes the `sub_27180` result into `iVar1` before the `if` (kuna inlines the call into the condition), and kuna prints 6 of the 8 `sshfatal` args. Both emit the "Subroutine does not return" warning on the fatal call. Current-kuna GED to source is ~0, not 46.

## Divergence experiment

The recorded GED-46 came from the OLD benchmark kuna 0.1.0, which ran `decompile-all` WITHOUT the listing enabled, so `noreturn_propagate` could not conclude `sub_3b8d0` (openssh `sshfatal`) is noreturn. I reproduced that exact regression by flipping the lever OFF on current kuna:

`--option noreturn_propagate off` → kuna fails to treat `sub_3b8d0` as noreturn, flow falls through the fatal call, and it swallows the *entire following function* (`hash_to_blob` @ 0xc650) into try_read_key: a ~90-line blob with a `while(true)`, `goto label_c7bc`, raw `v15[-1]=...` stack spills, `branchflip`/`tailcalljump` warnings, and dozens of stack temporaries. That inflated blob is the GED-46 output.

Default ON (current) → the clean 12-line function above. No lever needs flipping to reach ghidra; the correct output IS the default.

## Analysis / runtime-choice verdict

Root cause: the gap was the pre-session-1 benchmark surface. `kuna decompile-all` (old) did not enable the "listing," so the `noreturn_propagate` divergence had no listing to work from and could not mark the internal `sshfatal` wrapper (`sub_3b8d0`) as no-return. Without that fact, decompilation ran off the end of `try_read_key` into `hash_to_blob`, inflating the function to GED 46.

Session-1 PR **F1** (decompile-all now enables the listing so `noreturn_propagate` collapses swallowed functions) closes this. On current kuna the fatal call is correctly no-return, the function terminates at its true boundary, and the output is structurally identical to ghidra (and to source). This is not a runtime choice or a lever-flip: the default-ON `noreturn_propagate` + F1 listing is the win, and flipping the lever OFF is what re-opens the gap. Classification: already-fixed (F1). gap_survives=false.
