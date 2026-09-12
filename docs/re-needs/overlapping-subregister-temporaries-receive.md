---
need_id: overlapping-subregister-temporaries-receive
title: Overlapping subregister temporaries receive duplicate local names
track: quality
status: closed
severity: major
probe_id: p-e2e8e9ceb38a
acceptance_id: a-c6276af322b1
hypothesis_status: upheld
credibility: 0.7
instances: 1
challenges: [603a9d7833c5d42c3d016c55]
rounds: [11, 12]
first_seen_round: 11
attempts: 1
covered_by_option: null
touches: [decompiler/crates/kuna-decomp]
scope: small
regression_of: null
pr: 590
closed_in_round: 12
closing_pr: "590"
reject_reason: null
---

## Symptom

Read the process-name comparison routine with unambiguous locals.

> **Overlapping subregister temporaries receive duplicate local names** (major, `603a9d7833c5d42c3d016c55`)
> 0x402820 declares identical identifiers as undefined2 and char, sometimes twice as char. Both default and ghidra naming styles exhibit collisions.

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
    "0x402820",
    "--addr"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "(?m)^  (?:char|undefined2) (v[0-9]+);[^\\n]*\\n(?:[^\\n]*\\n)*?  (?:char|undefined2) \\1;"
    ]
  },
  "target": {
    "binary_rel": "bin/Anti CrackMe V2.exe",
    "binary_sha256": "928b320c57d86b8c1647ec269fa5966ea806c1e64f195a43daeb2619e87901ea",
    "binary_size": 26112,
    "binary_source": "dataset"
  }
}
```

## Acceptance

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "cmd": [
    "{{KUNA}}",
    "decompile",
    "{{BIN}}",
    "overlap_bytes"
  ],
  "cwd": "{{WORK}}",
  "env": {
    "SLEIGHHOME": "{{SPECS}}"
  },
  "stdin": null,
  "timeout_s": 60,
  "repeat": 1,
  "target": {
    "binary_rel": "decompiler/crates/kuna-analysis/tests/fixtures/overlaplocals_i386",
    "binary_sha256": "d9a4c30516c60c55703f2822688837b723a4b9b3b8131af2b4f306233e5eebb3",
    "binary_size": 8860,
    "binary_source": "in-repo",
    "in_repo_path": "decompiler/crates/kuna-analysis/tests/fixtures/overlaplocals_i386",
    "selector": "overlap_bytes",
    "selector_kind": "name"
  },
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "(?m)^  char v1 \\[2\\]; // al$",
      "(?m)^    v1\\[0\\] = s_804a000\\[8\\];$",
      "(?m)^    v1\\[1\\] = s_804a000\\[9\\];$"
    ],
    "stdout_absent": [
      "(?m)^  (?:char|undefined2) v1;"
    ]
  },
  "notes": "Primary promoted acceptance for overlapping-subregister-temporaries-receive. The positive clauses require one compatible two-byte whole declaration and independent low/high byte references bound to it; the negative clause rejects redundant scalar declarations of that identifier. The Ghidra and dedup-off siblings pin the same invariant under both requested option surfaces."
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- Split subregister symbols may retain a shared display name.

## Refutation

_not yet refuted_

## Reference

_none recorded_

## Instances

- `603a9d7833c5d42c3d016c55` (round 11, tester t-r11-603a9d78)

## Decision log

- filed by cluster.py from 1 observation(s)
- round 11 REFUTER: hypothesis **upheld** (was inconclusive). Refuted in-tick by the captain (round 11). VERDICT UPHELD, and the emitted C is worse than the title
says: it is not valid C at all.

REPRODUCED on the dataset copy of the need's binary (challenges/603a9d7833c5d42c3d016c55/bin/Anti
CrackMe V2.exe, identical to the arena copy), current main:
`kuna decompile BIN 0x402820 --addr` -> rc 0, 100 declarations, and EIGHT names (v2..v9) are each
declared THREE times -- 24 declarations for 8 identifiers. The probe regex hits 8 times.

THE EXACT SHAPE, which pins the mechanism in three lines (v2, lines 14-16 of the output):
    undefined2 v2;
    char v2; // al
    char v2; // ah
and the only references to v2 in the whole body are `v2._0_1_` and `v2._1_1_` (lines 113-114). So the
2-byte parent is what the body actually uses, through piece syntax, and the two subregister pieces are
declared under the parent's name and NEVER referenced by that name. The filed hypothesis ("split
subregister symbols may retain a shared display name") is upheld and can be sharpened: the pieces do
not merely share the parent's name, they are redundant declarations -- so the fix may be to uniquify
the piece names OR to stop declaring pieces the emitter addresses through the parent. Leave that
choice to the builder; both flip the acceptance.

CONSEQUENCE THE TITLE UNDERSTATES: `undefined2 v2;` followed by `char v2;` is a conflicting
redeclaration. This output cannot be fed to a compiler, which puts it in the same class as the other
invalid-C need this round (sixteen-byte-simd-accesses) rather than in the cosmetic-naming class its
"minor/naming" framing suggests. Worth weighing at T_TRIAGE.

ACCEPTANCE (recipe check (v)): SOUND BUT OVER-PERMISSIVE. It is stdout_absent of the same
same-identifier-twice regex, so any fix that removes the duplicate names flips it -- no unflippable
trap like live-simd-string-copies. But the regex only covers types char|undefined2, so a partial fix
that leaves duplicates at other widths would still pass. Do not treat a green acceptance here as proof
the class is gone; re-run the duplicate-name count.

- attempt 1 BUILDER: implemented unique whole-owner declaration selection for overlapping variable
  groups and deterministic suffix allocation for remaining collisions. The allocator reserves original
  locals, parameters, global symbols, and named direct callees so a generated suffix cannot capture a
  non-local reference. Acceptance was deliberately retargeted to the independently authored in-repo
  `overlaplocals_i386` fixture and strengthened with positive whole-owner plus low/high byte-reference
  assertions; its ID therefore changed from `a-923bd9e40d90` to `a-c6276af322b1`. Ghidra-style and
  `dedupvardecls off` acceptance siblings cover the other requested surfaces. Review and merge remain
  pending; status stays open and `pr` stays null.

- attempt 1 post-fix audit: reran the exact dataset binary above at `0x402820` in default,
  Ghidra-name, and `dedupvardecls off` modes. All three returned 0 and emitted 103 declarations with
  no duplicate declared identifier. In each mode, `v4` through `v9` are each declared exactly once
  while both `._0_1_` and `._1_1_` references remain bound to that whole; repeat runs were byte-stable.
  Whole-binary JSON covered 107 functions with no top-level error or failed function and repeated at
  SHA-256 `279dc98288d96470d317318be42a50b648e9ddcdf9a423ec160986fcdfea7b28`.
  Clang accepted the extracted declaration blocks in all three modes with a generated type header,
  confirming that the conflicting local declarations are gone. Clang also accepted full project
  exports of the independently authored `overlaplocals_i386` fixture in all three modes with only an
  external-data declaration shim. A full-body compile of the dataset function was checked but is not
  claimed fixed: it still fails on unrelated, pre-existing pseudo-C gaps including scalar member forms
  such as `undefined2._0_1_`, character-array member/cast forms, and undeclared `Stack00000023`.

- attempt 1 rebase audit: `mergecheck` against `31b96e103e6dad54ae65080221e74664295bfc0e`
  reports exactly five intentional shape-C `line-removed` rejects: four stale assertion-name strings in
  `docs/baseline-stages.json` and the stale `ghdec-symbol-keyed-local-decls.xml` table-row wording in
  `tests/stages/README.md`. Those five removals are stale prose-key replacements, not dropped rows.
  Separately, the feature deliberately changes assertion coverage: `paramrefdecl` now matches the exact
  off-path `a0_1` declaration/reference and adds `#2b` for the guarded `&a0` reference;
  `symbol-keyed-local-decls` changes the whole-declaration count from 3 to 2 and the narrow-shadow count
  from 1 to 0, and adds Ghidra-name checks `#7`/`#8`. The live baseline was regenerated and remains
  797/797, the README row remains present, no upstream assertion rows are otherwise lost, and shared
  counters still re-derive as 190 settables with tier counts 61/70/59, 276 corpus files, and next
  ElementId 4164.
- closed: acceptance a-c6276af322b1 now PASSES at c42d04f1837e

- round 12 closure-index audit: a branch-local `needs reindex` is unsafe because this checkout
  contains only 136 of the authoritative index's 197 active need records. A trial regeneration
  from the root's complete working set was also rejected: that live set includes unrelated mutable
  pipeline closures and produced roughly 360 lines of semantic drift, including unrelated status,
  probe/acceptance, and PR changes. The reviewed `c42d04f` authoritative index was therefore restored
  and updated only for this closure: aggregate status counts move from 135/61 to 136/60 closed/open,
  and this need alone gains `status: closed`, round 12, PR/closing PR 590, and `closed_in_round: 12`.
  A normalized audit confirms every unrelated index row remains byte-for-byte unchanged.
