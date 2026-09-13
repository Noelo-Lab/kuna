---
need_id: sixteen-byte-simd-accesses
title: Sixteen-byte SIMD accesses render as dereferenced arrays of pointers
track: quality
status: closed
severity: minor
probe_id: p-624a130d8a14
acceptance_id: a-9d336b0f3009
hypothesis_status: upheld
credibility: 0.7
instances: 1
challenges: [68b94ba38fac2855fe6fbada]
rounds: [11]
first_seen_round: 11
attempts: 0
covered_by_option: null
touches: [decompiler/crates/kuna-decomp]
scope: small
regression_of: null
pr: https://github.com/Noelo-Lab/kuna/pull/620
closed_in_round: 12
closing_pr: "620"
reject_reason: null
---

## Symptom

Unambiguous 16-byte memory accesses in the vectorized XOR loop.

> **Sixteen-byte SIMD accesses render as dereferenced arrays of pointers** (minor, `68b94ba38fac2855fe6fbada`)
> Loads and stores emit *(char *[16]), an invalid C cast to an array of sixteen pointers. Corresponding MOVDQU instructions at 0x140001440 and 0x140001474 access 16 bytes. The repaired _0_16_ lane widths remain intact; this is a separate declarator problem.

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
    "sub_1400012f0"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "\\*\\(char \\*\\[16\\]\\)"
    ]
  },
  "target": {
    "binary_rel": "bin/crackme1.exe",
    "binary_sha256": "16512154b0c419b2e4b064dbabd4932251ec5e0e11e14302d86821dca1ade1f0",
    "binary_size": 16384,
    "binary_source": "dataset"
  }
}
```

## Acceptance

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "timeout_s": 60,
  "cmd": [
    "{{KUNA}}",
    "decompile",
    "{{BIN}}",
    "sub_1400012f0"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_absent": [
      "\\*\\(char \\*\\[16\\]\\)"
    ]
  },
  "target": {
    "binary_rel": "bin/crackme1.exe",
    "binary_sha256": "16512154b0c419b2e4b064dbabd4932251ec5e0e11e14302d86821dca1ade1f0",
    "binary_size": 16384,
    "binary_source": "dataset"
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- Pointer-to-array declarator parentheses are misplaced during cast emission.

## Refutation

_not yet refuted_

## Reference

_none recorded_

## Instances

- `68b94ba38fac2855fe6fbada` (round 11, tester t-r11-68b94ba3)

## Decision log

- filed by cluster.py from 1 observation(s)
- round 11 REFUTER: hypothesis **upheld** (was inconclusive). UPHELD, and the fix is a KNOWN, ALREADY-PINNED one-line inversion -- the builder must flip an existing test, not write a new diagnosis.

REPRODUCED on main (d6105e3d): `kuna decompile <68b94ba3>/bin/crackme1.exe sub_1400012f0` rc 0, 647 lines, 7 occurrences of `*(char *[16])` (lines 191/214/264/333/389/549/606). v63 is declared `char v63 [16];` at line 82, so the intended type is pointer-to-`char[16]` and the correct C spelling is `char (*)[16]`.

HYPOTHESIS ("pointer-to-array declarator parentheses are misplaced during cast emission") IS EXACTLY RIGHT, and narrower than it reads. The cast path is innocent: printc.rs push_cast_type already does the right thing (declarator_parts -> front+back concatenated, which yields `char (*` + `)[16]` for a correct declarator). The defect is one loop in CSpeller::declarator, kuna_langc.rs:113: it walks the modifier stack BASE->OUTERMOST (`stack.iter().rev().skip(1)`), but the pending_ptr paren rule at :125 only wraps when an ARRAY sees a PRECEDING pointer -- which is the array-of-pointer ordering. C declarator binding is outermost-type-first, so the two nestings come out SWAPPED.

Walk it: ptr-to-array is stack [PTR, ARRAY, char]; rev/skip1 = [ARRAY, PTR] -> ARRAY first (pending false, no paren, back="[16]"), then PTR (front="*") -> `char *[16]`. Outermost-first [PTR, ARRAY] gives PTR (front="*", pending) then ARRAY (wraps) -> `char (*` + `)[16]`. Correct. The mirror case fixes itself: array-of-pointer [ARRAY, PTR, char] outermost-first gives back="[1]" then front="*" -> `char *a[1]`, which is right and is TODAY rendered wrong as `int4 (*a)[1]`.

*** THIS IS ALREADY A KNOWN DIVERGENCE WITH A TEST PINNING THE BUG. *** decompiler/crates/kuna-decomp/src/p9_emit/printc/tests.rs:113 `pointer_to_array_paren_inverted_divergence` asserts BOTH wrong spellings verbatim and its doc says "This test pins the actual (buggy) output so a future fix flips it deliberately." The `declarator_parts` doc-comment at printc.rs:8596 documents the CORRECT output (`int4 (*)[1]` -> `("int4 (*", ")[1]")`), which the code does not produce. So the builder's job is: reorder the walk, then FLIP the two assertions in that test and rewrite its DIVERGENCE preamble. It will otherwise go red on a test that looks unrelated to its diff.

WHAT THIS ROUND ADDS THAT THE OLD TEST DID NOT KNOW. The pinned test calls the bug "Latent: ptrtoarray.xml declares such params but never emits them as a decompiled function HEADER, so no passing assertion depends on it today." Round 11 shows it is NOT latent -- it reaches ordinary output through the CAST path on any 16-byte SIMD access, and the result is invalid C. That upgrades the need from cosmetic to the invalid-C class alongside overlapping-subregister-temporaries-receive.

WOULD A FIX BUILT ON THIS HYPOTHESIS PRODUCE WRONG OUTPUT? No. Post-fix the line reads `v63._0_16_ = *(char (*)[16])&v21[v14];` -- Ghidra's own idiom for a 16-byte lane transfer (`auVar1 = *(undefined1 (*) [16])(...)`), consistent with the `char v63 [16]` declaration. Array-valued assignment remains non-strict-C, but that is the declaration convention, not this defect, and the acceptance only demands the absence of `*(char *[16])`.

BLAST RADIUS, MEASURED, SMALL BUT NOT ZERO. declarator_parts is the single spelling seam for declarations, headers, struct fields AND `type_to_c_string` (the decbench type_match extractor), so the flip is global. In the corpus: tests/datatests/ptrtoarray.xml is the only file carrying `(*x)[N]` and all four hits are INPUTS (`map addr r0x104018 int4 (*paiGlob)[16]`, three `parse line extern ...`), never expectations -- its 10 stringmatch assertions are all body expressions (`display(&a0[1]);`, `paiGlob = &c;`, ...) and none names a declarator. So no datatest assertion should move; the pinned unit test is the one that must.

TWO WARNINGS FOR THE BUILDER. (1) The `--assert prototype`/`parse line` C PARSER side was not checked here -- if it builds ptr-to-array with the same inversion, a printer-only fix makes the two disagree; check `parse line extern void intarray(int4 (*a)[1]);` round-trips before committing. (2) `touches: [decompiler/crates/kuna-decomp]` is correct but coarse; the edit is p9_emit/kuna_langc.rs plus p9_emit/printc/tests.rs, and kuna_langrust.rs implements the same Speller trait -- confirm the Rust speller does not share the inverted walk.

- Sol-high BUILDER: reproduced the exact 16,384-byte witness at authoritative
  main `ccc82a37`: 630 output lines and seven `*(char *[16])` casts.  The repair
  yields the same 630 lines, zero invalid casts, and seven `*(char (*)[16])`
  casts; a full diff contains only those seven type-token substitutions.  The
  one-line loop reversal suggested by the refuter is too shallow for alternating
  modifiers: closing before an already accumulated suffix changes
  `(*x[2])[3]` into `(*x)[2][3]`.  `CDeclarator` instead applies modifiers
  outermost-to-base and groups the complete accumulated declarator when a
  postfix follows an ungrouped pointer.  Exact mirror and nested Datatype tests,
  plus pointer/array/function precedence cases, pin the generic rule.  All
  representative valid declarations pass strict C11 syntax compilation.  No
  option or ID is allocated: preserving a spelling that changes the recovered
  type and is invalid in a cast is not a useful policy choice.  Full corpus and
  workspace gates remain pending resource authorization.
