---
case_id: O2-noinline-libedit-libedit.so.0.0-ce_inc_search
group_id: libedit::ce_inc_search
status: metric-artifact
tier: L
margin: 294
bucket: kuna-specific
fresh_reproduces: true
fresh_ged: 294
option_closing: null
root_cause_family: angr-truncation-artifact
feature_slug: null
scope: none
angr_ref: null
kuna_stage: null
---

## Side-by-side

Structural metrics (from the triage dump):

| output | loc | gotos | labels | switches | cases | ifs | loops | ternaries |
|--------|---|---|---|---|---|---|---|---|
| angr(stored) | 46 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| kuna(stored) | 392 | 21 | 17 | 1 | 6 | 53 | 7 | 1 |
| kuna(fresh)  | 392 | 21 | 17 | 1 | 6 | 53 | 7 | 1 |

angr (stored) — the COMPLETE function body as stored (typedefs trimmed). Note it is
pure straight-line code that ends abruptly after `v10 = v7;` — no return, no branch,
no loop, for a function every other decompiler recovers ~290 CFG nodes/edges from:

```c
unsigned int ce_inc_search(struct_1 *ptr, unsigned int a1)
{
    void* ptr9;  // rdi
    unsigned long long v7;  // rax
    unsigned int v8;  // ebx
    struct_0 *ptr6;  // rsi
    unsigned long long v10;  // rax
    unsigned int v0;  // [bp-0x64]
    void* v1;  // [bp-0x60]
    unsigned int v2;  // [bp-0x58]

    v1 = ptr->field_58;
    v2 = g_439634;
    ptr9 = ptr->field_60;
    v0 = ptr->field_470;
    v7 = ptr->field_4a0;
    v8 = a1;
    ptr6 = ptr->field_498;
    v10 = v7;
}
```

kuna (stored == fresh, byte-identical; first ~55 of 392 lines shown, rest cut — a full,
plausible decompilation of the real ~200-line libedit `search.c:ce_inc_search` with the
main REPL loop, the keymap `switch` (6 cases), and the search-string rebuild loops):

```c
unsigned int ce_inc_search(int8 a0,int4 a1)

{
  /* ...27 local declarations cut... */
  v8 = dat_39634;
  v4 = *(int8 *)(a0 + 0x4a0);
  v28 = *(void *)(v26 + 0x28);
  v5 = *(void *)(a0 + 0x58);
  v2 = *(int4 *)(a0 + 0x470);
  v22 = *(void **)(a0 + 0x60);
  if (*(void **)(a0 + 0x68) <= &v22[v4 + 6]) {
    return 6;
  }
  v20 = *(void **)(a0 + 0x498);
  v15 = v4;
  v18 = a1;
  if (v4 == 0) goto label_149a5;
label_14800:
  v15 = v15 * 4;
  v23 = dat_39634;
  do {
    v24 = (int4 *)0x2ccc0;
    *v22 = 10;
    if (v18 == 0x18) {
      v24 = (int4 *)0x2ccd0;
    }
    /* ... prompt-rebuild loops ... */
    *v24 = 0;
    sub_13cc0(a0);
    if (el_wgetc(a0,&v19) != 1) {
      return sub_b940(a0,0);
    }
    v27 = 1;
    switch(*(void *)(*(int8 *)(a0 + 0x400) + ((uint8)v19 & 0xff))) {
      case 4:
      case 0x22:
        /* ... */
      default:
        /* ... 0x17 / 0x1b / 7 key handling ... */
      case 6:
      case 9:
        /* ... append char to search buffer ... */
      case 0x25:
        /* ... */
      case 0x26:
        v7 = 1;
        v18 = 0x18;
    }
    /* ... redisplay + recursive v9 = ce_inc_search(a0,v18) tail ... */
  } while( true );
}
```

Rescore (fresh, Joern) output:

```c
// scripts.decbench.rescore --case O2-noinline-libedit-libedit.so.0.0-ce_inc_search
// ged_before/after: 294.0 (approximated=true, delta 0.0)
// source_file: results/full_run/O0/libedit/compiled/chared.i
// source_nodes: 1        <-- degenerate source CFG: score is meaningless
```

## Analysis

- **Symptom**: recorded angr GED 0 vs kuna 294. But ghidra (288) and ida (293) cluster
  tightly with kuna, and the only 0-scorers are angr and phoenix — the same engine.
  Despite `bucket: kuna-specific` / `artifact_suspect: false` in the case JSON, this is
  the textbook benchmark-artifact signature, and the rescore confirms it.
- **Root cause, part 1 — degenerate source CFG**: the scorer matched the source function
  in `O0/libedit/compiled/chared.i`, but `chared.i:5599` contains only the **prototype**
  (`el_action_t ce_inc_search(EditLine *, int);` pulled in from `el.h`); the definition
  lives in `search.c` / `search.i` in the same directory. Joern's CFG for a declaration
  is 1 node → `source_nodes == 1` → the GED degenerates to the `|dNodes|+|dEdges|`
  approximation against an empty graph, i.e. each decompiler is scored on **its own CFG
  size**, not on similarity to the source.
- **Root cause, part 2 — angr truncation rewarded**: angr's stored output is truncated —
  46 lines of straight-line field loads with no return and zero control flow (a partial /
  failed decompilation), so its CFG is ~1 node, 0 edges → GED ≈ 0 against the degenerate
  source. Every decompiler that actually recovered the function (kuna 294, ida 293,
  ghidra 288) is penalized by exactly its graph size. angr does not "win" — its failure
  mode happens to match the scorer's failure mode.
- **kuna is fine here**: fresh kuna (commit d12ef72) reproduces the stored output
  byte-identically — a complete, structurally sensible decompilation of a genuinely large
  recursive REPL function (switch recovered, loops recovered, 21 gotos in line with
  ghidra's rendering of the same CFG). No option sweep is warranted: no kuna option can
  (or should) change a score that only rewards emitting *less* of the function.
- **Sibling** `O2-libedit-libedit.so.0.0-ce_inc_search`: identical artifact — same
  truncated 46-line straight-line angr block, same kuna 392 loc / 21 gotos, same recorded
  294; one shared decbench bug covers the whole group `libedit::ce_inc_search`.

## Proposed fix

No kuna change. Two decbench-side bugs to file:

1. **Source matcher picks declarations** (primary): the source-function locator matched
   the prototype in `chared.i` instead of the definition in `search.i` (same directory).
   Fix: when resolving a function to a `.i` file, skip Joern matches whose method has no
   body / a 1-node CFG and keep searching; and/or treat `source_nodes == 1` as
   "unscorable" and exclude the case from margins instead of recording 0-vs-N.
2. **Truncated decompiler output stored as valid**: the angr backend stored a partial
   function (no return statement, ends mid-block after a call-free prologue). The backend
   (or the scorer) should flag bodies with zero control flow *and* no return for a
   known-large function as decompilation failures rather than scoring them.

Also worth tightening: the `artifact_suspect` heuristic returned false here even though
the pattern is exact (angr==phoenix==0, ghidra/ida/kuna clustered at ~290); it should
treat phoenix as non-independent of angr and flag this constellation.
