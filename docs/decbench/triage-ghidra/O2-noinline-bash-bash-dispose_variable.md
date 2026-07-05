---
case_id: O2-noinline-bash-bash-dispose_variable
status: metric-artifact
gap_survives: false
recorded_kuna_ged: 32
divergence_lever: null
proposed_new_option: null
runtime_choice: false
angr_feature: null
---

## Current kuna vs ghidra

Recorded scores in `results/full_run/O2-noinline/bash/evaluated/bash.toml`:
`ghidra.ged=0.0`, `ida.ged=32.0`, `kuna.ged=32.0`. Note **IDA scores the same 32 as
kuna** — only ghidra scores 0.

**Source** (`O0/bash/compiled/variables.i`, `dispose_variable`): guard `if(var==0) return;`,
then `if((attr&0x20000)==0) dispose_variable_value(var);`, `if(var->exportstr) sh_xfree(...)`,
`sh_xfree(var->name,...)`, `if(attr&1) array_needs_making=1;`, `sh_xfree(var,...)`. Source CFG
(header-stripped, current extractor): **9 nodes / 13 edges**.

**Current kuna** (fresh `decompile-all --addr 0x53c10`) — ~18 LOC, 0 gotos, 0 labels, 4 ifs
(outer guard + 3 inner), 0 loops. A **complete, faithful** decompilation:

```c
void dispose_variable(void *a0)
{
  if (a0 != (void *)0x0) {
    if ((*(uint4 *)&a0[5] & 0x20000) == 0) {
      sub_51c40(a0[1]);                        // -> dispose_variable_value(var)
    }
    if (a0[2] != 0) {
      sh_xfree(a0[2],"variables.c",0xed1);     // var->exportstr
    }
    sh_xfree(*a0,"variables.c",0xed3);         // var->name
    if ((*(uint1 *)&a0[5] & 1) != 0) {
      dat_138450 = 1;                          // array_needs_making = 1
    }
    /* WARNING: tailcalljump: recovered tail call -> introduced call to 0x0009e530 */
    sh_xfree(a0,"variables.c",0xed8);          // sh_xfree(var)
    return;
  }
  return;
}
```

**Ghidra** (stored `ghidra_bash.c`, the only definition of the name) — a **degenerate
self-recursive stub**, objectively useless (1 CFG node):

```c
void dispose_variable(void)
{
  dispose_variable();
  return;
}
```

kuna reproduces every branch and free of the source; ghidra recovered nothing. IDA's output
(0x53c10) is essentially identical to kuna's (same nested ifs, same tail-call-as-return) and
also scores 32.

## Divergence experiment

Not applicable — no divergence lever was tested because kuna's output is not the problem.
kuna's output is already the correct/complete shape (and matches IDA). Flipping any of the 19
default-ON options could only perturb an already-good result; there is no ghidra shape to
recover toward (ghidra's shape is garbage). Winning lever: **none**.

## Analysis / runtime-choice verdict

Root cause is the **GED metric's source-side CFG in the scored run, not kuna**.

- GED = `vj_ged(source_cfg, decompiled_cfg)`; lower is better; 0 = perfect (`decbench/metrics/ged.py`).
- Reproducing GED against the **correct** header-stripped source CFG (9 nodes / 13 edges)
  gives the *opposite, correct* ranking: **kuna GED = 4** (9 nodes / 11 edges, near-perfect
  structural match) vs **ghidra GED = 36** (its 1-node stub is 36 edits from source). So on a
  sound metric kuna decisively beats ghidra here.
- The recorded scores (ghidra 0, kuna/ida 32) are only possible if the **source CFG used in
  the original scoring was degenerate (~1 node)**. That is exactly the failure mode
  `scripts/reeval_ged.py` documents: original GED source CFGs were extracted from the full
  preprocessed `.i` (80-98% inlined system headers), Joern timed out / produced degenerate
  graphs, so a 1-node degenerate source matches ghidra's 1-node stub (GED~0) and penalizes any
  full decompilation (kuna/ida ~32 insertions). `reeval_ged.py` re-scores against
  header-stripped sources to fix this, and **`results/full_run/ged_new.json` is absent** — the
  fix was never applied to this tree, so `bash.toml` still carries the broken original numbers.

Signature of a metric artifact: kuna == IDA (both full, correct) are equally punished while the
uniquely-degenerate output (ghidra) is rewarded; the "gap" tracks a broken reference CFG, not
any kuna divergence, inflation, or regression. Nothing about session-1 (F1/#120/#122) or any
default-ON angr divergence is implicated — kuna's output was already correct at benchmark time
and remains correct now.

Verdict: **metric-artifact**, `gap_survives=false`. No lever, no new option, no angr feature,
`runtime_choice=false`. On a correctly-computed GED, kuna already wins this function.
