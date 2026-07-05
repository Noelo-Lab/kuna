---
case_id: O2-noinline-findutils-find-entry_free
status: already-fixed
gap_survives: false
recorded_kuna_ged: 27
divergence_lever: null
proposed_new_option: null
runtime_choice: false
angr_feature: null
---
## Current kuna vs ghidra

Source (`sharefile.i:8334`), the ground truth GED scores against:
```c
static void entry_free (void *pv) {
  struct SharefileEntry *p = pv;
  if (p->fp) {
    if (0 != rpl_fclose(p->fp))
      fatal_nontarget_file_error(*__errno_location(), p->name);  // noreturn
  }
  free(p->name);
  free(p);
}
```

Current kuna (`decompile-all --addr 0xe490`, all defaults):
```c
void sub_e490(void *a0)
{
  unsigned int v1; // eax
  if (*(int8 *)((int8)a0 + 0x18) != 0) {
    if (sub_17850() != 0) {
      __errno_location();
                    /* WARNING: Subroutine does not return */
      sub_e410(*(void *)CONCAT44(dat_4,v1),*(void *)((int8)a0 + 0x10));
    }
  }
  free(*(void **)((int8)a0 + 0x10));
                    /* WARNING: tailcalljump: recovered tail call -> introduced call to 0x000069b0 */
  free(a0);
  return;
}
```
LOC ~13, gotos 0, labels 0, ifs 2 (nested), loops 0, switch 0.

Ghidra:
```c
void entry_free(void *param_1)
{
  int iVar1;
  int *piVar2;
  if (*(long *)((long)param_1 + 0x18) != 0) {
    iVar1 = FUN_00117850();
    if (iVar1 != 0) {
      piVar2 = __errno_location();
                    /* WARNING: Subroutine does not return */
      FUN_0010e410(*piVar2,*(undefined8 *)((long)param_1 + 0x10));
    }
  }
  free(*(void **)((long)param_1 + 0x10));
  free(param_1);
  return;
}
```
LOC ~13, gotos 0, labels 0, ifs 2 (nested), loops 0, switch 0.

Control flow is **identical**: same outer `if(p->fp!=0)`, same inner `if(rpl_fclose!=0)`, same noreturn `fatal_nontarget_file_error`/`sub_e410` call, same trailing `free(p->name); free(p); return`. Both mark `sub_e410`/`FUN_0010e410` `/* Subroutine does not return */`. The only residual (expression-level, non-CFG) differences:
- kuna renders the errno arg as `*(void *)CONCAT44(dat_4,v1)` (lost the `__errno_location()` -> eax link, spurious `dat_4:eax` concat) where ghidra shows the clean `*piVar2`. A data-flow rendering nit, not a control-flow node.
- kuna adds a `/* tailcalljump: ... */` comment on the final `free(a0)` (tail-call recovered as a normal call — the correct, ghidra-matching shape). A comment, stripped by Joern's CFG.

Current kuna is structurally `~=` ghidra; the recorded GED-27 gap does not reproduce.

## Divergence experiment

- **`--option noreturn_propagate off`** (reproduces OLD kuna 0.1.0 pre-F1): output EXPLODES. Because `sub_e410` (fatal_nontarget_file_error) is no longer concluded no-return, decompilation runs off the end of `entry_free` and swallows the **next** function `sharefile_init` @0xe4d0 — the body fills with `malloc(0x10)`, `strdup`, `sub_1ac70(0xb,0,...)`, a `branchflip` warning, and a nested free-chain that belongs to sharefile_init, with a bogus `uint4 * sub_e490(...)` signature. This is exactly the inflated shape that scored GED 27. **This is the closed gap.**
- **`--option tailcalljump off`**: removes the tailcalljump comment but renders the final free as `(*dat_3df78)(a0)` with `/* Treating indirect jump as call */` — strictly *worse* / less ghidra-like. tailcalljump default-ON is correct here; not a lever to flip.

Winning lever to close the 27-gap: **none needed** — it is closed by the F1 default (decompile-all enables the listing, so the default-ON `noreturn_propagate` fires on the internal no-return wrapper and collapses the swallowed `sharefile_init`).

## Analysis / runtime-choice verdict

**Root cause of the stale 27:** the benchmark ran kuna 0.1.0, where `decompile-all` did not enable the listing, so `noreturn_propagate` could not conclude `fatal_nontarget_file_error` (`sub_e410`) no-return. Decompilation overran the function boundary and merged in `sharefile_init`, massively inflating the CFG (the swallowed-function symptom). Ghidra concludes the no-return itself and stops cleanly at GED 0.

**Fix already merged:** session-1 **F1** made `decompile-all` enable the listing by default, so the default-ON `noreturn_propagate` now fires and `entry_free` terminates at the correct boundary. The `noreturn_propagate off` experiment above reproduces the old blown-up output exactly, confirming F1 is the closer. Not a runtime choice, not a bug — a stale benchmark number.

`gap_survives=false`, `status=already-fixed`. The tiny residual `CONCAT44(dat_4,v1)` errno-arg artifact is an expression-rendering nit that no divergence lever toggles and does not affect the CFG; it is far below the recorded 27 and does not warrant action here.
