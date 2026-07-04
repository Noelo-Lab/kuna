---
case_id: O2-zlib-minigzip64-deflateSetDictionary
group_id: zlib::deflateSetDictionary
status: metric-artifact
tier: M
margin: 90
bucket: inherited
fresh_reproduces: true
fresh_ged: 90
option_closing: null
root_cause_family: angr-truncation-artifact
feature_slug: null
scope: none
angr_ref: null
kuna_stage: null
---
## Side-by-side

angr (stored) — complete, 21 LOC, zero branches:

```c
// Function: deflateSetDictionary @ 0x69a0
void deflateSetDictionary(unsigned long a0, unsigned long a1, unsigned long a2)
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
    sub_4069b4();
    return;
}
```

kuna (stored, run-time) — trimmed to the structural skeleton (~50 of 101 LOC; the elided middle is straight-line hash-table/window arithmetic):

```c
// Function: deflateSetDictionary @ 0x69a0
unsigned long deflateSetDictionary(int8 *a0,int8 a1,uint4 a2)
{
  ...
  v17 = a0;
  v12 = sub_6910();
  if ((a1 == 0) || (v12 != 0)) {
    return 0xfffffffe;
  }
  v5 = v17[7];
  v12 = *(int4 *)(v5 + 0x30);
  if (v12 == 2) {
    return 0xfffffffe;
  }
                    /* WARNING: branchflip: flipped negated guard for linearity ... */
  if (v12 != 1) {
    if (*(int4 *)(v5 + 0xb4) != 0) {
      return 0xfffffffe;
    }
    v13 = *(uint4 *)(v5 + 0x50);
    *(void *)(v5 + 0x30) = 0;
    if (v13 > a2) goto label_6aa1;
    if (v12 == 0) {
      /* clear hash head table: memset + counters reset */
    }
  }
  else if (*(int4 *)(v5 + 8) != 0x2a) {
    return 0xfffffffe;
  }
  if (*(int4 *)(v5 + 0xb4) != 0) {
    return 0xfffffffe;
  }
  v7 = a0[0xc];
  a0[0xc] = sub_ddc0(v7,a1,a2);            /* adler32 over the dictionary */
  v13 = *(uint4 *)(v5 + 0x50);
  *(void *)(v5 + 0x30) = 0;
  if (v13 > a2) goto label_6aa1;
  a1 = a1 + (uint8)(a2 - v13);
  a2 = v13;
label_6aa1:
  ...
  while( true ) {
    sub_5550(v5);                          /* fill_window */
    v13 = *(uint4 *)(v5 + 0xb4);
    if (v13 <= 2) break;
    ...
    do {
      /* insert-string hash loop */
      v14 = v14 + 1;
    } while (v14 != v13);
    ...
  }
  /* restore strm->next_in/avail_in, wrap, return 0 */
  return 0;
}
```

kuna (fresh, current build d12ef72) — byte-identical to the stored output above (same 101 LOC / 2 gotos / 1 label / 10 ifs / 2 loops), so no post-Jun-29 PR changed this function.

## Analysis

Symptom: angr scored a perfect GED 0 with a 21-line, branch-free body; kuna (and ghidra 90, ida 90, binja 91) scored ~90 with a full 101-line decompilation containing 10 ifs and 2 loops — which is what zlib's `deflateSetDictionary` actually looks like. angr's stored block is obviously truncated: it is only the function prologue (5 callee-saved register spills) followed by a single call to `sub_4069b4()` — that address is exactly entry+0x14, i.e. the instruction right after the pushes. angr's CFG recovery on the stripped binary split the function at 0x69b4 and emitted the entire real body as a separate synthetic function, leaving `deflateSetDictionary` as a prologue stub. phoenix=0 is the same engine, not independent confirmation.

Confirmed by rescore (`scripts.decbench.rescore`, kuna commit d12ef72): **source_nodes = 1** — the source-side CFG is degenerate, so the GED is meaningless and minimally-sized (truncated) outputs are rewarded with 0 while every correct full recovery pays ~|nodes|+|edges| ≈ 90. Fresh kuna GED validated at 90 (before == after == recorded; no option sweep is meaningful against a degenerate reference).

Why the source CFG is degenerate: the rescore resolved the source to `/home/mahaloz/github/decbench/results/full_run/O0/zlib/compiled/adler32.i` — the wrong translation unit. `deflateSetDictionary` is *defined* in `deflate.c`/`deflate.i`; `adler32.i` (preprocessed) merely contains the **declaration** `extern int deflateSetDictionary (z_streamp strm, const Bytef *dictionary, uInt dictLength)` at line 2202, pulled in from `zlib.h`. The benchmark's source resolver matched the first file containing the identifier, hit the header declaration, and Joern produced a 1-node method graph for it. Two compounding decbench-side defects: (1) source resolution matches declarations in preprocessed `.i` files instead of definitions; (2) no guard rejects `source_nodes == 1` scores.

No kuna defect: fresh output is byte-identical to the benchmarked run, structurally in family with ghidra/ida/binja, and is a faithful decompilation of the real function (state-check guards, adler32 call, fill_window + insert-string hash loop, next_in/avail_in save/restore). The 2 gotos into `label_6aa1` mirror zlib's shared "truncate dictionary to window size" tail. No option sweep performed — no option can (or should) shrink a correct 100-line function to a 21-line stub to please a 1-node reference.

Siblings (dump only, no options): `O2-zlib-minigzip-deflateSetDictionary` — identical artifact (angr = same 21-line prologue stub calling `sub_4069b4`, kuna identical 101-LOC output). `O2-noinline-zlib-minigzip-deflateSetDictionary` — identical artifact at 0x6df0 (angr stub calls `sub_406e04` = entry+0x14, kuna identical 101-LOC output). The whole 4-case group inherits the same wrong-source + angr-split artifact.

## Proposed fix

Nothing to fix in kuna. Decbench-side bug to file, two parts:

1. Source resolution: when mapping `function → source file`, prefer the TU containing the *definition* (Joern method with a body / CFG > 1 node), not the first `.i` whose text contains the identifier — preprocessed zlib TUs all embed `zlib.h` declarations, so every `deflate.c` export is at risk of resolving to `adler32.i`&c. (This case: should be `deflate.i`, resolved to `adler32.i`.)
2. Scoring guard: treat `source_nodes == 1` as unscorable (null GED / drop the case) rather than emitting a GED that rewards truncated decompiler output; optionally also flag decompiled outputs whose CFG is a single block ending in a call to `entry+K` as suspected function-splits (angr's stub pattern here).

Re-run the `zlib::deflateSetDictionary` group (4 cases, margin 90 each) after the harness fix; expect the angr/phoenix 0s to disappear and kuna to cluster with ghidra/ida/binja against the real `deflate.i` reference CFG.
