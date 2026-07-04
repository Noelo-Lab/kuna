---
case_id: O2-dpkg-dpkg-packages
group_id: dpkg::packages
status: metric-artifact
tier: A
margin: 161
bucket: inherited
fresh_reproduces: true
fresh_ged: 161
option_closing: null
root_cause_family: angr-truncation-artifact
feature_slug: null
scope: none
angr_ref: null
kuna_stage: null
---
## Side-by-side

angr (stored) — complete, 15 lines. Note: no control flow at all, and the body is just
the callee-saved-register spills (push r13/r12/rbx) followed by a call to `sub_4144ed`
= image base 0x400000 + 0x144ed 
= `packages`+0xd — angr split the function immediately
after its own prologue and "called" the remainder as a separate sub:

```c
// Function: packages @ 0x144e0
void packages(unsigned long a0)
{
    unsigned long long v4;  // r13
    unsigned long long v5;  // r12
    unsigned long long v6;  // rbx
    unsigned long long v0;  // [bp-0x20]
    unsigned long long v1;  // [bp-0x10]
    unsigned long long v2;  // [bp-0x8]

    v2 = v4;
    v1 = v5;
    v0 = v6;
    sub_4144ed();
    return;
}
```

kuna (stored, run-time) — trimmed to the first ~55 of 100 lines (rest is the
symmetrical else-branch loop + epilogue calls):

```c
// Function: packages @ 0x144e0
unsigned long packages(int8 *a0)
{
  int8 v1;
  unsigned long v2;
  int8 v3;
  int8 *v4; // rax
  char *v6; // rsi

label_1474b:
  v5 = (void *)dcgettext(0,"you must specify packages by their own names, not by quoting the names of the files they come in",5);
  sub_265b0();
label_14768:
  v2 = *v5;
  v6 = "--%s needs at least one package name argument";
  do {
    sub_265b0(dcgettext(0,v6,5),v2);
label_1478b:
    v6 = "--%s --pending does not take any non-option arguments";
    v2 = *dat_252a78;
  } while( true );
  sub_18e70();
  v7 = 0;
  if (dat_4f3c4 == 0) {
    v7 = 4 - (uint4)(uint1)sub_117b0(0x10000);
  }
  sub_1e550(v7);
  sub_12050();
  sub_1f750();
  sub_254a0("startup packages %s",*dat_252a78);
  if (dat_4f3d0 != 0) {
    if (*a0 != 0) goto label_1478b;
    v2 = sub_2a4a0();
label_14558:
    v3 = sub_2a540(v2);
    if (v3 != 0) {
      while (v7 = *(int4 *)&dat_252a78[5], v7 == 4) {
label_14594:
        if (*(int8 *)(v3 + 0x160) == 0) goto label_14558;
label_1459e:
        if (2 <= (uint4)(*(int4 *)(v3 + 0x10) - 1U)) goto label_14558;
label_145a9:
        sub_14f60(v3);
        if (*(char *)(*(int8 *)(v3 + 0x138) + 8) != '\0') goto label_14558;
        *(void *)(*(int8 *)(v3 + 0x138) + 8) = 1;
        sub_2a8d0(0x4f3f0,v3);
        v3 = sub_2a540(v2);
        if (v3 == 0) goto label_145dd;
      }
      /* ... enqueue_pending action dispatch (if/else ladder, gotos back into
         the loop), the else-branch per-package do/while, and the epilogue
         (sub_20000/sub_13ed0/sub_18d60/sub_1eb20; return 0) ... [45 lines cut] */
```

kuna (fresh, current build d12ef72) — byte-identical to the stored block above
(same 100 loc / 13 gotos / 8 labels / 19 ifs / 3 loops; no drift since the 0.1.0 run):

```c
// Function: packages @ 0x144e0
unsigned long packages(int8 *a0)
/* ... identical to kuna(stored), omitted ... */
```

Structural metrics: angr 15 loc / 0 gotos / 0 ifs / 0 loops; kuna stored == kuna
fresh at 100 loc / 13 gotos / 8 labels / 19 ifs / 3 loops.

## Analysis

- **Symptom**: angr recorded GED 0 vs kuna 161 on dpkg `packages` (stripped, O2).
  Fresh kuna (commit d12ef72, post-SAILR-PRs) reproduces exactly — output and
  rescored GED (161) are unchanged from the 0.1.0 run.
- **The score is a benchmark artifact, confirmed on both sides**:
  1. **Degenerate source CFG**: the rescore reports `source_nodes: 1`, and the
     source file the scorer paired with this function is
     `results/full_run/O0/dpkg/compiled/archives.i` — the wrong translation unit
     (`packages` lives in packages.c; kuna's own output even contains the
     assert string `"packages.c" ... "enqueue_pending"`). A 1-node source CFG
     makes GED ≈ |decompiled CFG|: any decompiler that recovers the real
     function is charged its whole size.
  2. **angr's output is truncated**: its `packages` is only the three
     callee-saved spills plus `sub_4144ed()` — 0x400000 base + 0x144ed =
     `packages`+0xd. angr's CFG on the stripped binary split the function right
     after the push sequence and modeled the rest as a separate callee, so its
     emitted body has an essentially single-node CFG that trivially matches the
     degenerate source and scores 0. phoenix=0 is the same angr engine
     (different structurer only), not independent confirmation.
- **Why bucket=inherited is consistent**: ghidra 161 and ida 164 cluster
  exactly with kuna — every non-angr decompiler that recovered the real ~100-line
  function pays the same penalty. There is no kuna-specific structuring gap here.
- **Options**: no sweep run — no option can (or should) make kuna emit a
   truncated 15-line body; the target score is not legitimately reachable.
- **Siblings**: none (`siblings: []`).
- Secondary observation (not this case's cause, matches ghidra's score so it is
  inherited behavior): kuna's block layout places the shared error tail
  (`label_1474b`/`label_14768`/`label_1478b`, an unconditional `do{}while(true)`)
  ahead of the function entry code — worth an eye in a separate goto-structuring
  triage, but it does not affect the artifact verdict here.

## Proposed fix

No kuna change. File a decbench-side scoring bug with two parts:

1. **Source-CFG pairing/validation**: the ground-truth extractor matched dpkg
   `packages` to `O0/dpkg/compiled/archives.i` and produced a 1-node CFG. The
   scorer should (a) fix the function→TU pairing, and (b) hard-invalidate any
   case where `source_nodes <= 1` (degenerate source ⇒ score is meaningless)
   instead of letting it reward empty output.
2. **Truncation guard**: flag decompiler outputs whose CFG size is a tiny
   fraction of the peer median (angr 15 loc vs 100/161-node peers) as suspect
   rather than scoring them at face value — here angr's function-boundary
   split (`call packages+0xd`) was rewarded, not its structuring.

Scope: none (metric artifact; nothing to build in kuna).
