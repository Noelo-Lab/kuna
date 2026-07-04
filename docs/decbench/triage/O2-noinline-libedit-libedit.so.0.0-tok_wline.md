---
case_id: O2-noinline-libedit-libedit.so.0.0-tok_wline
group_id: libedit::tok_wline
status: metric-artifact
tier: A
margin: 164
bucket: ahead
fresh_reproduces: true
fresh_ged: 164
option_closing: null
root_cause_family: angr-truncation-artifact
feature_slug: null
scope: none
angr_ref: null
kuna_stage: null
---
## Side-by-side

angr (stored) — complete, all 30 lines:

```c
// Function: tok_wline @ 0x1ac30
typedef struct struct_0 {
    int field_0;
} struct_0;

typedef struct struct_1 {
    unsigned long long field_0;
} struct_1;


unsigned int tok_wline(struct_0 *ptr, struct_0 **p, unsigned int *ptr4, struct_1 **ptr5, unsigned int *ptr6, unsigned int *ptr7)
{
    unsigned int v13[2];  // rax
    unsigned long long v19;  // rdx

    ptr[17].field_0 = ptr[17].field_0 & 4294967293 | 1;
    switch ((unsigned int)v19)
    {
    case 0:
        ptr[16].field_0 = 2;
        v13 = *((long long *)&ptr[8]);
        break;
    case 1:
        goto LABEL_0x41acd0;
    case 2:
        goto LABEL_0x41aff6;
    case 3:
        goto LABEL_0x41afd0;
    case 4:
        goto LABEL_0x41afe0;
    default:
        goto LABEL_0x41ae28;
    }
}
```

kuna (stored, run-time) — first ~55 of 222 lines (cut: the remaining ~170 lines are the full tokenizer body — quote/backslash/newline case handling, the `realloc` grow loops, the 5-way quote-state `switch(v15)`, and the shared append/continue tail):

```c
// Function: tok_wline @ 0x1ac30
unsigned long tok_wline(void *a0,void *a1,void *a2,void *a3,uint4 *a4,uint4 *a5)

{
  ...
  v20 = 0xffffffff;
  v18 = 0xffffffff;
  v9 = (int4 *)*a1;
  v14 = (int4 *)a1[1];
  if ((int4 *)a1[2] <= v9) goto label_1ad19;
label_1ac7f:
  v7 = *v9;
  if (v9 == v14) goto label_1ad2b;
  v15 = (uint8)*(uint4 *)&a0[8];
  v5 = v7 + -0x27;
  v21 = 0;
  v14 = v9;
  v6 = v7;
  if (v7 == 0x27) goto label_1ad4a;
label_1ac97:
  v17 = (uint4)v18;
  v13 = (uint4)v15;
  if (v21 || SBORROW4(v6,0x27) != v5 < 0) {
    if (v7 == 0x22) {
      *(uint4 *)((int8)a0 + 0x44) = *(uint4 *)((int8)a0 + 0x44) & 0xfffffffd | 1;
      if (5 <= v13) {
        return 0xffffffff;
      }
      v7 = *(int4 *)(v15 * 4 + 0x2d96c);
                    /* WARNING: Treating indirect jump as call */
      return (*(code *)((int8)v7 + 0x2d96c))();
    }
    if (v7 <= 0x22) {
      if (v7 == 0) {
label_1af20:
        if (5 <= *(uint4 *)&a0[8]) {
          return 0xffffffff;
        }
        v7 = *(int4 *)((uint8)*(uint4 *)&a0[8] * 4 + 0x2d958);
                    /* WARNING: Treating indirect jump as call */
        return (*(code *)((int8)v7 + 0x2d958))();
      }
      if (v7 == 10) {
        v8 = *(uint4 *)((int8)a0 + 0x44) & 0xfffffffd;
        *(uint4 *)((int8)a0 + 0x44) = v8;
        if (v13 == 3) {
          *(uint4 *)((int8)a0 + 0x44) = v8 | 2;
          goto label_1aff6;
        }
        ...
      }
    }
  }
  /* ... ~170 more lines: backslash case, quote-state switch(v15) with
     cases 0-4 + default goto label_1ae28, realloc grow loops, append tail ... */
label_1ae28:
  return 0xffffffff;
}
```

kuna (fresh, current build, commit d12ef72) — byte-identical to the stored kuna output (same 222 loc / 19 gotos / 14 labels / 1 switch / 5 cases / 35 ifs / 2 loops), so it is not re-quoted here.

Structural metrics:

| output | loc | gotos | labels | switches | cases | ifs | loops |
|--------|---|---|---|---|---|---|---|
| angr(stored) | 30 | 5 | 0 | 1 | 5 | 0 | 0 |
| kuna(stored) | 222 | 19 | 14 | 1 | 5 | 35 | 2 |
| kuna(fresh) | 222 | 19 | 14 | 1 | 5 | 35 | 2 |

## Analysis

**Symptom.** angr scored GED 0 and kuna 164 on `tok_wline` (libedit's wide-char
tokenizer, ~200 lines of source with a character loop and a 5-way quote-state
switch). Fresh kuna reproduces the stored output byte-for-byte, and a rescore
confirms fresh GED is still 164 (approximated).

**The score is an artifact, on both sides of the comparison.**

1. *angr's output is truncated/broken.* Its entire 30-line body is a switch on
   `v19` — a variable that is **declared but never assigned** — whose cases
   `goto` five labels (`LABEL_0x41acd0` etc.) that **do not exist anywhere in
   the emitted text**. There is no loop and no if; the function ends right
   after the switch. ghidra (182) and ida (183) both recover the same full
   ~220-line body kuna does, so the real function is unambiguously large.
   phoenix's 0 is the same angr engine with a different structurer — not
   independent confirmation.

2. *The source CFG is degenerate.* The mandatory rescore reports
   `source_nodes: 1`, and it matched the source to
   `.../O0/libedit/compiled/chared.i` — the **wrong translation unit**
   (`tok_wline` lives in tok.c, where it is defined through libedit's
   `FUN(tok,line)`/`TYPE()` macro layer, so the literal name `tok_wline` never
   appears at a function definition in un-expanded source). Joern therefore has
   no real ground-truth CFG: against a 1-node source graph the approximated
   score `|dNodes|+|dEdges|` simply *rewards the smallest output*. angr's
   unparseable 30-line stub lands at 0; kuna's complete 222-line function is
   charged 164 for existing.

**Why no option sweep.** No kuna option can honestly close this gap — the only
way to score 0 here is to emit a near-empty function. Kuna's number is, if
anything, the best of the three engines that actually decompiled the function
(164 vs ghidra 182 / ida 183).

**Genuine (non-scoring) kuna note.** Kuna's output contains three
`/* WARNING: Treating indirect jump as call */` sites (tables at
`0x2d944/0x2d958/0x2d96c`): the compiler split the quote-state dispatch into
several computed-goto jump tables and kuna recovers only one of them as a
switch. That is the known jumptable guard-bound family
(`switchmodbound`/`switchguardbound`, PR #60) — already tracked, irrelevant to
this case's score, and shared with ghidra/ida.

**Siblings.** `O2-libedit-libedit.so.0.0-tok_wline` (@ 0x1c070): identical
artifact — same 30-line angr stub (uninitialized switch head, five dangling
gotos), same 222-line kuna output, same recorded 0-vs-164. One triage covers
the whole group `libedit::tok_wline`.

## Proposed fix

No kuna change. Decbench-side bugs to file:

1. **Invalidate degenerate-source cases.** When `source_nodes == 1`, the GED is
   meaningless; the case should be scored NaN/excluded, not treated as a valid
   0-vs-164 margin. (This one is compounded by wrong source-file matching:
   `tok_wline` was matched to `chared.i`; macro-generated definitions like
   libedit's `FUN(tok,line)` need preprocessed-source (`.i`) function-name
   resolution against the *correct* TU.)
2. **Reject unparseable/truncated decompiler output.** angr's block `goto`s
   five labels that are not defined in its own emission and switches on a
   never-assigned variable — a cheap validity check (all goto targets defined,
   body non-trivially smaller than every other engine's) would flag it instead
   of rewarding it with a perfect score.
