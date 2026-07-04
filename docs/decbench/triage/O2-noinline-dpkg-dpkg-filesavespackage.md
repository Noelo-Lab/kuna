---
case_id: O2-noinline-dpkg-dpkg-filesavespackage
group_id: dpkg::filesavespackage
status: metric-artifact
tier: M
margin: 99
bucket: inherited
fresh_reproduces: false
fresh_ged: 25
option_closing: null
root_cause_family: angr-truncation-artifact
feature_slug: null
scope: none
angr_ref: null
kuna_stage: null
---
## Side-by-side

angr (stored) — the COMPLETE block, not trimmed; note zero control flow:

```c
// Function: filesavespackage @ 0x9930
void filesavespackage(unsigned long a0, unsigned long a1, unsigned long a2)
{
    unsigned long long v6;  // r15
    unsigned long long v7;  // r14
    unsigned long long v8;  // r13
    unsigned long long v9;  // r12
    unsigned long long v10;  // rbx
    unsigned long long v0;  // [bp-0x30]
    unsigned long long v1;  // [bp-0x20]
    unsigned long long v2;  // [bp-0x18]
    unsigned long long v3;  // [bp-0x10]
    unsigned long long v4;  // [bp-0x8]

    v4 = v6;
    v3 = v7;
    v2 = v8;
    v1 = v9;
    v0 = v10;
    sub_40994f();
    return;
}
```

kuna (stored, run-time) == kuna (fresh, current build d12ef72) — byte-identical, shown once:

```c
// Function: filesavespackage @ 0x9930
uint8 filesavespackage(int8 a0,int8 *a1,int8 *a2)

{
  unsigned long v1; // rax
  int8 *v2;
  unsigned long v3; // rax
  uint8 v4; // r15
  
  v1 = sub_2add0(a1,3);
  sub_21820(0x40,"filesavespackage file \'%s\' package %s",*(void *)(*(int8 *)(a0 + 8) + 8),v1);
  v2 = *(int8 **)(*(int8 *)(a0 + 8) + 0x18);
  if (((v2 != (int8 *)0x0) && (*v2 != 0)) && ((*a1 == v2[2] || (*a2 == v2[2])))) {
    v4 = 1;
    sub_21820(0x40,"filesavespackage ... diverted -- save!");
  }
  else {
                    /* WARNING: branchflip: flipped negated guard for linearity (positive condition, if/else arms swapped) */
    if ((*(uint1 *)(*(int8 *)(a0 + 8) + 0x30) & 2) != 0) {
      v4 = 0;
      sub_21820(0x40,"filesavespackage ... in new archive -- no save");
    }
    else {
      v1 = sub_252a0();
      while (v2 = (int8 *)sub_252c0(v1), v2 != (int8 *)0x0) {
        v3 = sub_21820(0x40,"filesavespackage ... also in %s",sub_2add0(v2,3));
        if ((a2 != v2 && a1 != v2) && ((*(int4 *)((int8)a1 + 0x54) != 1 || (*v2 != *a1)))) {
          sub_21820(0x40,"filesavespackage ...  is 3rd package");
          if ((char)v2[0x2f] != '\0') {
            sub_21820(0x40,"filesavespackage ...  taken -- no save");
            sub_252e0(v1);
            return CONCAT71(SUB87(v3,1),a2 == v2 || a1 == v2) & 0xffffffff;
          }
          sub_21820(0x40,"process_archive ... already disappeared!");
        }
      }
      v4 = 1;
      sub_252e0(v1);
      sub_21820(0x40,"filesavespackage ... not taken -- save !");
    }
  }
  return v4;
}
```

Structural metrics: angr(stored) 21 LOC / 0 ifs / 0 loops; kuna(stored) == kuna(fresh) 41 LOC / 4 ifs / 1 loop / 0 gotos.

## Analysis

Symptom: recorded angr GED=0 vs kuna GED=99 (margin 99), with ghidra=99 / ida=97 / phoenix=0.

This is a decbench scoring artifact, on two independent grounds:

1. **angr's stored output is a truncated lift, not a decompilation.** Its entire body is
   five callee-save register spills followed by a single opaque call `sub_40994f()` and
   `return`. With angr's default 0x400000 rebase, 0x40994f = 0x9930 + 0x1f =
   `filesavespackage+0x1f` — angr stopped lifting right after the prologue and rendered
   the *rest of the function's own body* as a call to an anonymous sub. The ground-truth
   source (`archives.i` line 8050, dpkg `src/main/archives.c`) has a divert check, a
   flags check, and a package-iterator `while` loop with three `continue` paths —
   real control flow that kuna, ghidra, and ida all recovered (they cluster at 97–99)
   and angr entirely lacks. `phoenix=0` is the same angr engine (same truncated lift,
   different structurer), so it is not independent confirmation. A one-basic-block stub
   cannot legitimately score GED 0 against a source CFG that the rescore measures at
   **21 nodes** — the recorded 0 comes from a broken scoring state at benchmark time.

2. **The recorded kuna 99 is metric drift.** Fresh kuna (commit d12ef72) produces output
   byte-identical to the stored run-time output, yet the rescore recomputes GED = **25**
   and flags `METRIC-DRIFT: recorded 99 vs recomputed 25`. Identical code scoring 99 then
   and 25 now means the recorded per-case numbers for this group came from a different
   (bad) source-CFG/scoring state, consistent with the 0/97/99/99 spread above.

Why existing kuna options don't apply: nothing is wrong on the kuna side to cover. Fresh
kuna's structure (1 loop, 4 ifs, 0 gotos) already mirrors the source shape; the residual
GED 25 vs a 21-node source is expression-level noise (the `CONCAT71(SUB87(...))` return
expression and the branchflip'd early-return restructure), not a missing structuring
pass, and is far from the recorded 99. No option sweep was warranted for an artifact.

Siblings: `O2-dpkg-dpkg-filesavespackage` (the only sibling) shows the *identical*
artifact — angr's stored block is the same prologue-only stub calling `sub_408d7f()` =
0x8d60 + 0x1f = `filesavespackage+0x1f` (same +0x1f truncation offset), and kuna
stored == fresh at 41 LOC / 4 ifs / 1 loop. The whole group's margin is the same artifact.

Note: the case JSON had `artifact_suspect: false`, but the ghidra/ida-cluster-vs-
angr-engine-only-zero signature plus the visibly truncated angr body is exactly the
historical artifact pattern; the flag was a false negative here.

## Proposed fix

No kuna work. Decbench-side bug to file:

- The stored angr/phoenix outputs for group `dpkg::filesavespackage` (both O2 and
  O2-noinline) are prologue-only truncated stubs (tail-call to `func+0x1f`); their GED 0
  should be invalidated or the group rescored end-to-end. A cheap detector: flag any
  decompilation whose only call target resolves inside the *same* function's own address
  range (self-truncation stub), or any GED 0 where the candidate CFG has 1 node but
  source_nodes > 1.
- The recorded kuna/ghidra/ida ~99 values for this group predate a scoring fix
  (recomputed kuna GED = 25 on identical output); the group's stored scores should be
  regenerated with the current Joern/scoring pipeline.
