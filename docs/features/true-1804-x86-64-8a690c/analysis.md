# Analysis — `test_decompiling_true_1804_x86_64 :: usage`

- **Opportunity:** `test_decompiling_true_1804_x86_64::usage`
- **Binary:** `/home/mahaloz/github/angr-dev/binaries/tests/x86_64/true_ubuntu1804`
- **Function:** `usage` (x86_64 @ `0x401610`)
- **angr version:** 9.2.213
- **Owning stage:** S7/S8 — `blockaction` (Stage 16, *Structuring*): the
  CollapseStructure/TraceDAG region-collapse engine + ConditionalExecution
  (`condexe`) + goto/schema selection.

## What angr does better

The full side-by-side is in `angr-vs-kuna.txt`. Metrics (reference | kuna):

| metric | angr | kuna |
|---|---|---|
| loc | 112 | 121 |
| gotos | 2 | 3 |
| labels | 2 | 3 |
| ifs | 6 | 9 |
| loops | 2 | 2 |

The pipeline "recovery-failure marker" signal here is a **false positive** — it
fires on the `/* WARNING: Subroutine does not return */` comment kuna emits before
the tail-call `exit(a0)`, which is a normal Ghidra no-return annotation, not a
structuring abort. The real gap is two structuring-quality differences:

### 1. Duplicated if/else tail (the dominant difference)

kuna's trailing block is:

```c
label_17b8:
  v4 = *(int8 *)&v6[0x18];          // angr: v28
  if (v4 == 0) {
    __printf_chk(1, "...online help...", ...);   // (a)
    v3 = setlocale(5,0);                          // (b)
    if (v3 != 0) {
      v1 = strncmp(v3,"en_",3);                   // (c)
      if (v1 != 0) { v4 = 0x4764; goto label_18e8; }
    }
    v4 = 0x4764; v3 = " invocation";
    __printf_chk(1, "...Full documentation...", ...);   // (d)
  }
  else {
    __printf_chk(1, "...online help...", ...);   // (a)  DUPLICATE
    v3 = setlocale(5,0);                          // (b)  DUPLICATE
    if (v3 != 0) {
      v1 = strncmp(v3,"en_",3);                   // (c)  DUPLICATE
      if (v1 != 0) {
label_18e8:
        __printf_chk(1, "...Report translation bugs...", ...);
      }
    }
    __printf_chk(1, "...Full documentation...", ...);   // (d)  DUPLICATE
    v3 = ""; if (v4 == 0x4764) v3 = " invocation";
  }
```

The online-help printf `(a)`, the `setlocale` `(b)`, the `strncmp` `(c)` and the
Full-documentation printf `(d)` are emitted **twice** — once in each arm of
`if (v28 == 0)`. angr emits each of them **once** and threads the two genuinely
distinct continuations through shared labels:

```c
    if (v28) { ...; goto LABEL_401822_or_4018e8; }
    else     { ...; LABEL_4018e8: report_bugs; LABEL_401822: full_documentation; }
```

This is angr's structurer doing condition-aware **region deduplication** (its
"ITE region converter" / condition-processing): two predecessors are converged onto
one labelled successor instead of cloning the shared continuation. kuna inherits
Ghidra's collapse-based structurer, which clones the shared tail into both arms — so
it pays an extra `if`, extra lines, and one extra goto/label.

### 2. Loop shape

The inlined `rep cmps` / strcmp loop is emitted by kuna as

```c
  do { ... goto label_17af } while (v15);
label_17af:
  if ((!v14 && !v15) == v14) goto label_17b8;
```

(a goto *out* of the loop) where angr renders a do/while terminated by a `break`.
This is again `blockaction` goto-vs-`break` schema selection.

## Hypothesis / why this is not a one-Action fix

Both differences are products of the **structurer itself**, not of the
pre-structuring dataflow. There is no lifted-CFG artifact that "shares" the
duplicated tail — sharing requires CollapseStructure/TraceDAG to converge two
predecessors onto one labelled block, a decision that lives *inside* the collapse
engine. Unlike `kuna_loweredswitch` (which fabricates an S2 `BRANCHIND`+`JumpTable`
and lets the *unchanged* structurer render it), there is no manufacturable upstream
artifact here. Closing the gap means a condition-aware ITE-dedup region converter —
a new pass *type* touching `s8_structure`/`blockaction` region collapse.

→ **Scope: LARGE.** Routed to a draft `[PROPOSAL]` PR per Hard rule 7; see
`proposal.md`.
