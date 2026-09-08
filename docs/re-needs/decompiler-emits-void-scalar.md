---
need_id: decompiler-emits-void-scalar
title: Decompiler emits a void scalar local and uses it in arithmetic
track: quality
status: open
severity: major
probe_id: p-338523ce8c44
acceptance_id: a-98656add2172
hypothesis_status: overturned
credibility: 1.0
instances: 2
challenges: [5ab77f5f33c5d40ad448c834, 6a148dd62b3df128c1df5c9e]
rounds: [9]
first_seen_round: 9
attempts: 0
covered_by_option: null
touches: [decompiler/crates/kuna-decomp]
scope: small
regression_of: null
pr: null
closed_in_round: null
closing_pr: null
reject_reason: null
---

## Symptom

A valid scalar type for the recovered AL value, even when control flow remains uncertain.

> **Decompiler emits a void scalar local and uses it in arithmetic** (major, `5ab77f5f33c5d40ad448c834`)
> The function at 0x44ac6b declares a local as void, assigns a cast-to-void call result to it, stores it through a pointer, and later casts it to char for arithmetic. Exit status is 0.

> **Allocated byte-buffer operations produce a scalar void local** (major, `6a148dd62b3df128c1df5c9e`)
> Declares void v5 and assigns byte XOR results cast to void, then casts that local back to unsigned char. A scalar void local cannot represent these operations. A tested VirtualAlloc prototype with an unsigned char * return still leaves a scalar void local, renumbered v6.

## Reproduction

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "timeout_s": 60,
  "cmd": [
    "{{KUNA}}",
    "decompile",
    "{{BIN}}",
    "0x44ac6b",
    "--addr"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "\\bvoid\\s+[A-Za-z_]\\w*\\s*;"
    ]
  }
}
```

## Acceptance

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "target": {
    "binary_rel": "bin/crackme1.zip.__x/crkme.exe",
    "binary_sha256": "9031b8200481747e7ddb5cd5fc2a74030cd04052770bfa23d857a0662a58bc06",
    "binary_size": 266240,
    "binary_source": "dataset"
  },
  "timeout_s": 60,
  "cmd": [
    "{{KUNA}}",
    "decompile",
    "{{BIN}}",
    "0x44ac6b",
    "--addr"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_absent": [
      "\\bvoid\\s+[A-Za-z_]\\w*\\s*;",
      "\\(void\\)"
    ],
    "stdout_matches": [
      "sub_44ac7a\\(\\)"
    ]
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- A void pointee or call result may have propagated into a scalar variable type.
- Possibly propagating a void pointee through byte loads instead of preserving their access width.

## Refutation

_not yet refuted_

## Reference

_none recorded_

## Instances

- `5ab77f5f33c5d40ad448c834` (round 9, tester t-r9-5ab77f5f)
- `6a148dd62b3df128c1df5c9e` (round 9, tester t-r9-6a148dd6)

## Decision log

- filed by cluster.py from 2 observation(s)
- captain T_DEDUP r9: MERGED BY HAND from 2 observations on 2 challenges/2 testers (5ab77f5f `decompile 0x44ac6b`, 6a148dd6 `decompile sub_140006280`). Both emit a `void` SCALAR local and then use it; the witness's acceptance (`\bvoid\s+[A-Za-z_]\w*\s*;` absent) is the broader of the two and subsumes the other's (`\bvoid\s+v[0-9]+\s*;`). The non-witness half has no clause of its own and would close for free -- T_REFUTE should check the two really share a root cause (a void pointee propagating into a scalar type) before a builder treats one fix as covering both.
- round 9 REFUTER: hypothesis **overturned** (was inconclusive). captain T_REFUTE r9: OVERTURNED on the falsifiable half of the hypothesis. Both instances reproduce on the release binary: 5ab77f5f/crkme.exe 0x44ac6b emits `void v4; // al`, `v4 = (void)sub_44ac7a();` and `v4 = (char)((char)v4 + '$')`; 6a148dd6/p0tp.exe sub_140006280 emits `void v5;` fed by byte XORs and stored through `v4[-2]`. (1) Bullet 2 ("propagating a void pointee through byte loads instead of preserving their access width") names a site that CANNOT be the route. LOAD/STORE type propagation goes through propagate_load_store -> propagate_from_pointer (p5_types/coreaction_infertypes.rs:567-596), which only adopts the pointee when `ptrto.get_size() == sz`. TypeVoid is built as `Datatype::new_with_align(0, 1, TYPE_VOID)` (substrate/dtype.rs:5999) — size 0 — so a void pointee can never satisfy that equality for a 1-byte load; the access width IS preserved on this path. A builder told to "preserve the access width at the byte load" would be editing a guard that already does the right thing. (2) Bullet 1 ("a void pointee or call result may have propagated into a scalar variable type") is a restatement of the symptom, not a mechanism, and is not testable as filed. (3) The locked-call-output arm is likewise guarded: infertypes.rs:268 returns None for a locked TYPE_VOID return, so a declared void-returning callee is not the route either. (4) THE TESTER'S OWN CONTROL IS VOID AND MUST NOT BE RELIED ON. The report says "a tested VirtualAlloc prototype with an unsigned char * return still leaves a scalar void local", which reads as evidence against the pointee theory. I re-ran it with --assert-strict: `--assert 'prototype VirtualAlloc unsigned char *VirtualAlloc(void *a,unsigned long long b,unsigned int c,unsigned int d)'` is ACCEPTED (the call's arity visibly drops from 5 args to 4) but the RETURN TYPE never lands — the line stays `v4 = (void *)VirtualAlloc(...)`. `--assert 'type v3 unsigned char *'` on the same local is likewise silently inert, also under --assert-strict. So no control has yet actually changed the pointee, and the pointee theory is untested rather than disproved. (5) WHERE A BUILDER SHOULD LOOK INSTEAD: the void arrives on a value varnode by some route other than load/store propagation. In both instances a `void *` is in scope and the void-typed local is a MERGED high (5ab77f5f v4 takes both a call return and a store operand; 6a148dd6 v5 takes both a direct load and a XOR result), so HighVariable type selection over merged varnodes, and PTRADD/downChain on a size-0 pointee (`v4 = &v3[2]` on a `void *`), are the two unexamined candidates. The eventual fix should be stated as "a value varnode never carries a size-0 type", which is size-checkable, rather than as anything about byte loads. (6) SHARED-ROOT-CAUSE QUESTION FROM T_DEDUP IS STILL OPEN: the two instances share the shape (a void* in scope + a 1-byte value typed void) but I did not prove one cause covers both; the non-witness half still has no acceptance clause of its own, so a witness-only fix will close this need regardless.
- round 9 TRIAGE: acceptance strengthened against the delete-the-variable fix. Measured at 0x44ac6b on the r9 arena crkme.exe with a post-#504 binary: the current output carries BOTH `void v4; // al` and `v4 = (void)sub_44ac7a();`. Added stdout_absent `\(void\)` (the cast is the same defect seen at the call site; `(void *)`/`(void **)` do not match it) and stdout_matches `sub_44ac7a\(\)` so a fix that simply drops the variable and its call cannot pass. al is one byte, so the intended type is char/unsigned char.

### 2026-09-08T06:05Z — captain, round 9 B_DRAIN: acceptance target bound

The acceptance probe carried no `target`, so the suite could not resolve `{{BIN}}` and
reported `ProbeError: {{BIN}} used but the context supplies no bin` — indeterminate rather
than failing, and therefore unclosable by B_DONE however good a fix was. Bound to the
challenge's own image (challenge `5ab77f5f`, one image, no ambiguity):

    bin/crackme1.zip.__x/crkme.exe (266240 B, sha256 9031b820…)

Re-run through `verify --acceptance-suite --need decompiler-emits-void-scalar` on the current build it now RUNS
and FAILS, on the clause that carries the filed symptom:

    stdout_absent[0] → actual "void v4;"; stdout_absent[1] → actual "(void)"

Nothing about the probe's command or expectations was changed — only the target it
resolves against.
