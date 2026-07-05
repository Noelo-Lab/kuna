---
case_id: O2-noinline-cronie-crond-overwrite_database
status: genuine-bug
gap_survives: true
recorded_kuna_ged: 13
divergence_lever: regionstructure
proposed_new_option: null
runtime_choice: false
angr_feature: null
---
## Current kuna vs ghidra

Source (`database.i`, `overwrite_database`) is a plain top-tested loop:

```c
for (u = old_db->head; u != NULL; u = nu) {
  if (DebugFlags & 0x10) printf("\t%s\n", u->name);
  nu = u->next;
  unlink_user(old_db, u);
  free_user(u);
}
*old_db = *new_db;
```

**Current kuna (all defaults ON) — INVALID C:**

```c
void sub_5780(int8 *a0,int8 *a1)
{
  int8 *v1;
  int8 v2;
  int8 *v3;

  if ((dat_112a0 & 0x10) != 0) {
    __printf_chk(1,"unlinking old database:\n");
  }
  v3 = (int8 *)*a0;
  if ((int8 *)*a0 != (int8 *)0x0) {
    while (
    if (dat_112a0 & 0x10) != 0 {__printf_chk(1,0xc801,v3[2])
    }v1 = (int8 *)*v3, sub_5750(a0,v3), sub_8050(v3), v1 != (int8 *)0x0) {
      v3 = v1;
    }
  }
  v2 = a1[1];
  *a0 = *a1;
  a0[1] = v2;
  a0[2] = a1[2];
  return;
}
```

The `while ( ... )` **condition contains an `if` statement plus a comma-expression body**. This does not compile — it is the classic invalid-C-in-`while`-condition shape. LOC ~24, gotos 0, but the single loop is malformed.

**Ghidra — clean top-tested `while`, VALID:**

```c
void overwrite_database(undefined8 *param_1,undefined8 *param_2)
{
  undefined8 *puVar1;
  undefined8 uVar2;
  undefined8 *puVar3;

  if (((byte)DAT_001112a0 & 0x10) != 0) {
    __printf_chk(1,"unlinking old database:\n");
  }
  puVar3 = (undefined8 *)*param_1;
  while (puVar3 != (undefined8 *)0x0) {
    if (((byte)DAT_001112a0 & 0x10) != 0) {
      __printf_chk(1,&DAT_0010c801,puVar3[2]);
    }
    puVar1 = (undefined8 *)*puVar3;
    FUN_00105750(param_1,puVar3);
    FUN_00108050(puVar3);
    puVar3 = puVar1;
  }
  uVar2 = param_2[1];
  *param_1 = *param_2;
  param_1[1] = uVar2;
  param_1[2] = param_2[2];
  return;
}
```

LOC ~26, gotos 0, 2 `if`, 1 clean `while`. GED 0 vs source.

## Divergence experiment

Levers flipped OFF (single `--addr 0x5780`, `timeout 120`):

| lever | result |
|---|---|
| `regionlooprefine` off | still INVALID (identical to default) |
| `loopbreak_recovery` off | still INVALID |
| `branchflip` off | still INVALID |
| **`regionstructure` off** | **VALID C** (see below) |

`regionstructure` OFF is the necessary lever — the loop refinement is gated inside the SAILR region structurer, so `regionlooprefine`/`loopbreak_recovery` off alone do nothing while `regionstructure` is on.

**`regionstructure` off — VALID, but do-while shape (not ghidra's top-tested while):**

```c
  v3 = (int8 *)*a0;
  if ((int8 *)*a0 != (int8 *)0x0) {
    while( true ) {
      if ((dat_112a0 & 0x10) != 0) {
        __printf_chk(1,0xc801,v3[2]);
      }
      v1 = (int8 *)*v3;
      sub_5750(a0,v3);
      sub_8050(v3);
      if (v1 == (int8 *)0x0) break;
      v3 = v1;
    }
  }
```

Valid C, same body/operations as ghidra, but wraps the loop as `if (v3 != 0) { while(true){ ... if(v1==0) break; ...} }` (Ghidra's native structurer form) instead of ghidra's rotated top-tested `while (v3 != 0)`.

## Analysis / runtime-choice verdict

**genuine-bug** (invalid-C-in-`while`-condition class, `#122`, not fully fixed for this loop shape).

Root cause: the SAILR region structurer (`regionstructure`, default-on, which drives the cyclic loop-refinement `regionlooprefine`) is *trying* to produce exactly ghidra's clean top-tested `while (v3 != 0)` — that intent is correct and is what wins on the majority. But on this shape its rendering **folds the entire loop body, including a nested `if` statement, into the `while` condition**, emitting uncompilable C (a statement inside an expression context). This is the same defect class `#122` addressed for while-conditions; it survives here because the body contains a nested `if`.

This is NOT a runtime choice: a runtime choice requires both renderings to be valid so an agent can pick per source-shape. Here the default output is **invalid C** — never defensible on any function, majority or not. The fix belongs on-default: when the refined loop body cannot be reduced to a pure expression condition (e.g. it contains a nested `if`), `regionstructure` must fall back to the `while(true){ ...; if(cond) break; }` form it already emits correctly when the feature is off — rather than splicing a statement into the condition. `regionstructure off` is only a workaround (it disables SAILR entirely, regressing the majority), which is why `runtime_choice=false`.

kuna is uniquely worse here: ghidra, and kuna-with-`regionstructure`-off, both produce valid C for this trivial `for` loop; only the on-default path is uncompilable, inflating the Joern GED.
