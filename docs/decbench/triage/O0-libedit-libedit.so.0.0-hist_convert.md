---
case_id: O0-libedit-libedit.so.0.0-hist_convert
group_id: libedit::hist_convert
status: metric-artifact
tier: S
margin: 13
bucket: ahead
fresh_reproduces: true
fresh_ged: 13
option_closing: null
root_cause_family: degenerate-source-cfg
feature_slug: null
scope: none
angr_ref: null
kuna_stage: null
---
## Side-by-side

angr (stored, GED 0):

```c
// Function: hist_convert @ 0x11e8a
typedef struct struct_0 {
    char padding_0[1144];
    unsigned long long field_478;
    struct struct_1 *field_480;
} struct_0;

typedef struct struct_1 {
    unsigned long long field_0;
} struct_1;

long long hist_convert(struct_0 *a0, unsigned int a1, unsigned long a2)
{
    char v0;  // [bp-0x28]
    long long v1;  // [bp-0x20]

    return (a0->field_480(a0->field_478, &v0, a1, a2, a0->field_480) == 4294967295 ? 0 : ::0x40a240::ct_decode_string(v1, (struct struct_0 *)&a0[1].padding_0[96], (struct struct_0 *)&a0[1].padding_0[96]));
}
```

kuna stored (GED 13) — kuna fresh (current main, d12ef72) is byte-identical:

```c
// Function: hist_convert @ 0x11e8a
unsigned long hist_convert(int8 a0,unsigned int a1,unsigned long a2)

{
  code *v1;
  unsigned long v2;
  unsigned long v3; // rax
  char v4 [8];
  int8 v5; // fs_offset
  unsigned long v6; // stack - 0x20
  unsigned long v7; // stack - 0x10

  v7 = *(void *)(v5 + 0x28);
  v1 = *(code **)(a0 + 0x480);
  v2 = *(void *)(a0 + 0x478);
                    /* WARNING: branchflip: flipped negated guard for linearity (positive condition, if/else arms swapped) */
  if ((*v1)(v2,v4,a1,a2) != -1) {
    v3 = ct_decode_string(v6,a0 + 0x4e8);
  }
  else {
    v3 = 0;
  }
  return v3;
}
```

Actual source (`.../O0/libedit/compiled/hist.c:244-252`):

```c
static wchar_t *
hist_convert(EditLine *el, int fn, void *arg)
{
	HistEventW ev;
	if ((*(el)->el_history.fun)((el)->el_history.ref, &ev, fn, arg) == -1)
		return NULL;
	return ct_decode_string((const char *)(const void *)ev.str,
	    &el->el_scratch);
}
```

## Analysis

Symptom as recorded: angr 0, phoenix 0, ida 8, kuna 13, ghidra 21. Fresh kuna (current main)
reproduces the stored output byte-for-byte, and the rescore validates fresh GED = 13
(before == after == recorded, no options).

The score is a decbench artifact, confirmed by the mandatory rescore:

- `source_nodes: 1` — the source CFG is **degenerate** (single node), which per the campaign
  rules makes the GED meaningless.
- `source_file: .../O0/libedit/compiled/chared.i` — the scorer resolved `hist_convert` to the
  **wrong translation unit**. `chared.i` contains only the header **prototype**
  (`__attribute__((__visibility__("hidden"))) wchar_t *hist_convert(EditLine *, int, void *);`,
  line 5625, pulled in from hist.h). The real definition (a static function with an if + two
  returns) is in `hist.c:245` / `hist.i`. Joern's CFG for a declaration-only method is 1 node.

Against a 1-node source graph, every decompiler's GED is just |output CFG| — i.e. output
verbosity, not structural fidelity. angr's output is a single return-of-ternary statement
(its StackCanarySimplifier deletes the whole canary pattern, AIL expression folding inlines
the fptr/argument loads, and ITERegionConverter collapses the guard diamond into `?:`), so
it lands nearest the single node and scores 0. phoenix = 0 is the same engine, not
independent confirmation. ida (8), kuna (13), ghidra (21) simply rank by how many statements
they emit. `artifact_suspect` was false only because ida/ghidra didn't cluster with kuna —
but that spread is exactly what a verbosity metric produces.

Option sweep: no default-OFF catalog option targets the (real but unscoreable) cosmetic
differences — the sweep candidates are all switch-recovery/loader/arch-specific. The relevant
defaults (`stackguard`, `foldcallret`, `branchflip`, `dedupitetail`) are already on and none
produces a ternary return or removes the orphaned canary-init load.

Genuine (cosmetic, correctly decompiled) kuna gaps visible here, for the record — real
readability items but NOT what this 13-point margin measures:
1. Orphaned canary-init load `v7 = *(void *)(v5 + 0x28);` — `stackguard`
   (`s7_regions/kuna_stackguard.rs`) strips the check branch + handler at S7, after dead-code
   elimination, so the entry save into the canary slot survives; angr's StackCanarySimplifier
   removes both ends early.
2. Single-use loads spilled to temps (`v1`, `v2`) instead of inlined into the indirect call
   (S6 explicit-marking; `foldcallret` only folds call *returns*).
3. The guard diamond renders as if/else writing `v3` + one merged return instead of angr's
   `return cond ? A : B` (kuna has no ITERegionConverter analog; upstream Ghidra never emits
   ternaries).

Siblings: `O2-noinline-libedit-libedit.so.0.0-hist_convert` — identical symptom, identical
scores (angr 0 / kuna 13 / ghidra 19 / ida 8), kuna output byte-identical in shape; the same
wrong-TU source resolution applies (same function name, same scorer logic), so it should be
invalidated with this case.

## Proposed fix

Decbench-side bug to file: the source-CFG extractor resolves a function name to the first
`.i`/`.c` containing it and here picked `chared.i`, where `hist_convert` exists only as a
header prototype, producing a 1-node Joern CFG. Fix: prefer the TU containing a method
**with a body** (or the TU whose object actually defines the symbol), and hard-invalidate
any case where `source_nodes == 1` instead of scoring it (both `hist_convert` cases in the
libedit group should be dropped from the margin table).

No kuna change is warranted by this case. The three cosmetic items above (stackguard
orphan-load removal; single-use load folding; an ITE-region ternary pass modeled on angr's
`optimization_passes/ite_region_converter.py`) are real quality candidates but should be
pursued only if a *non-degenerate* case surfaces them, since each is output-changing and the
last needs new S8/S9 emission support.
