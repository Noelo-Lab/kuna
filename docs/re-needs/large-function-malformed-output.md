---
need_id: large-function-malformed-output
title: a large checker decompiles into malformed and prohibitively noisy C
track: quality
status: open
severity: major
probe_id: p-8355fed97a86
acceptance_id: a-2a1f5bccb422
hypothesis_status: overturned
credibility: 0.8
instances: 1
challenges: [6a3822888a86e4c2c55254fe]
rounds: [1, 2]
first_seen_round: 1
attempts: 1
covered_by_option: null
touches: [decompiler/crates/kuna-decomp/src/p2_lift]
scope: large
regression_of: null
pr: 417
closed_in_round: null
closing_pr: null
reject_reason: null
---

## Symptom

Readable pseudocode for the core checker at 0x140002d50, with coherent C++ string objects and crypto-call arguments.

> **Large checker decompiles into malformed and prohibitively noisy C** (blocker, `6a3822888a86e4c2c55254fe`)
> It emitted over 1,200 lines and 165 anonymous variables. The output included impossible conditions such as `v13 != v13 + 6`, raw synthetic stack-address writes, and badly reconstructed BCrypt calls.

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
    "0x140002d50",
    "--addr"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "if\\ \\(v13\\ !=\\ \\(unsigned\\ int\\ \\*\\)\\(\\(long\\ long\\)v13\\ \\+\\ 6\\)\\)"
    ]
  },
  "target": {
    "binary_rel": "bin/selam.exe",
    "binary_sha256": "01f288daf9ca08857b51c550a0e4b1d422724de883d984780fb8c493f2e2c859",
    "binary_size": 240128,
    "binary_source": "dataset"
  },
  "cwd": "{{WORK}}",
  "repeat": 1,
  "probe_id": "p-8355fed97a86"
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
    "0x140002d50",
    "--addr"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_absent": [
      "\\*\\(unsigned long long \\*\\)&v\\d+\\[-8\\] = 0x1"
    ],
    "stdout_matches": [
      "BCryptGenerateSymmetricKey\\(v\\d+,"
    ]
  },
  "target": {
    "binary_rel": "bin/selam.exe",
    "binary_sha256": "01f288daf9ca08857b51c550a0e4b1d422724de883d984780fb8c493f2e2c859",
    "binary_size": 240128,
    "binary_source": "dataset"
  },
  "cwd": "{{WORK}}",
  "repeat": 1,
  "probe_id": "a-2a1f5bccb422"
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- Stack-variable splitting and C++ small-string-object type recovery fail together in this optimized function.

## Refutation

**REFUTED (builder `b-r3-large-function-m`, round 3).** Neither stack-variable splitting nor C++
small-string-object type recovery is the cause, and the four filed complaints are three
independent defects.

The dominant one is an unmodelled instruction. x86 SLEIGH lowers `INT imm8` to
`intloc = swi(imm8); call [intloc]` -- a `call` with **no matching push** -- while
`x86-64-win.cspec`'s default `__fastcall` carries `extrapop="8" stackshift="8"`. Every `INT`
therefore grows RSP by 8, and kuna prints it: `(*(void *)swi(0x29))(5);` is immediately followed
by `v65 = &v64[8];`. The witness contains **eleven** `INT 0x29` (`__fastfail`, the MSVC `/GS` +
STL `_STL_VERIFY` idiom). Once two paths join with RSP offsets 8 apart, the frame stops being a
constant offset from the spacebase: stack locals degenerate into offsets off a `char *`, and each
`CALL`'s return-address push -- normally a dead store into an unmapped slot -- survives as an
explicit store. That is the filed "raw synthetic stack-address writes" (63 of them) and it is why
the BCrypt arguments are stack blobs.

Proven twice without engine code. `--assert 'flow <addr> callreturn'` on the 11 sites:
1267 -> 1171 lines, 63 -> 0 synthetic stores, 159 -> 144 declarations, and
`BCryptGenerateSymmetricKey(*(unsigned long long *)&v66[0x50],&v53,v60,v51)` becomes
`BCryptGenerateSymmetricKey(v52,&v55,v62,v54)`. Byte-patching the 30 `MOV ECX,5 ; INT 0x29` sites
to NOPs: 1121 lines, 0 synthetic stores. Across all 24 of this image's 907 functions that contain
`INT 0x29`: 2802 -> 2583 lines, 85 -> 0 synthetic stores, 558 -> 493 declarations.

**The impossible condition does NOT fall out of that fix** -- the question the B_PLAN note asked.
It is present at line 1249 today and at line 1157 with all 11 fastfail sites cut, and it is
*faithful*: MSVC really emitted `LEA RAX,[R12+6] ; CMP R12,RAX ; JZ` at `0x14000385a`. Its real
home is a second defect: 21 `REP STOSB` in this function each render as a hand-rolled 5-line fill
loop (~105 lines, ~21 induction variables, `builtin_memset` count 0), because `memsetrecover`
matches only the straight-line run-of-stores form. The `stdout_absent` clause the B_PLAN note
drafted therefore belongs on that increment, not on the fastfail one.

**The acceptance cannot be satisfied soundly and needs re-cutting.** `std::string` is unreachable
on this witness for four independent reasons: no PDB or symbols; no `basic_string` RTTI (it is not
polymorphic, so it has none -- the image's 30+ MSVC RTTI descriptors do not include it); the
function *constructs no `std::string`* (the MSVC SSO signature `_Myres=0xf` at `+0x18` /
`CMP [x+0x18],0xf ; CMOVA` appears nowhere in `0x140002d50..0x140004550`; the nearest string object
is in the **caller** at `0x140004a95`), so even a complete MSVC STL layout-recovery feature would
not fire here; and what the function actually takes is `{const char *, size_t}`, a
`std::string_view`, nameable only by guessing from the shape. Replacement clauses, both verified in
both directions on the unmodified binary: `stdout_absent` `\*\(unsigned long long \*\)&v\d+\[-8\] = 0x1`
(63 today, 0 with the fix) and `stdout_matches` `BCryptGenerateSymmetricKey\(v\d+,` (0 today, 1
with the fix).

Full evidence, the three-increment plan and the recommended first change:
`docs/features/large-function-malformed-output/proposal.md`.

## Reference

_none recorded_

## Instances

- `6a3822888a86e4c2c55254fe` (round 1, tester t-r1-6a382288)

## Decision log

- round 1: gated `admitted`, NOT built (round 1 closed 12 of 23). Carried into round 2.
- round 2 wave 20 (captain): **RESTORED FROM `.kuna-repipe/rounds/1/gate.json` + the arena
  report** and RE-GATED against merged main 509bb7a3. This record carried only a `## Symptom`
  stub, no probe and no acceptance, so it was undispatchable by `select` and unclosable by
  B_DONE; the two arms below existed all along in round-1 state and were never transcribed.
  Two-arm verdict today: `admitted` (probe PASSES, acceptance FAILS), so the gap is live and
  closable-by-probe. Target rebound from the round-1 arena path to the dataset challenge
  (`binary_source: dataset`, sha256 pinned), so the probe no longer depends on an arena that
  a later round may rebuild.
- round 2 wave 20 (captain): scope small -> LARGE: the acceptance asks for `std::string` in a 1,200-line optimized MSVC C++
  checker, i.e. C++ object/type recovery, not a local print fix. Expect a proposal fork.
- round 2 wave 20 (captain): `touches` LEFT as filed (`[decompiler/crates/kuna-decomp]`). I have measured evidence of the
  symptom but none of the owning module, and a confidently-wrong `touches` would misdirect a
  builder and mis-scope its leases. The builder should set it from its own root-cause work.
- round 2 B_PLAN (captain): **PASSED OVER for this wave in favour of
  [[simd-constant-string-initializer]]** — not deprioritised, not blocked. Reason is the
  acceptance, not the evidence: `stdout_matches: ["std::string"]` is a narrow proxy for a broad
  symptom (1,200 lines, 165 anonymous variables, impossible conditions, mangled BCrypt calls),
  and it is satisfiable by a build that spells one type `std::string` without removing any of
  the malformed C the tester actually complained about. Measured at 8799f22e: acceptance still
  FAILS (no `std::string` anywhere in the 1,267-line emission), and the headline impossible
  condition is still emitted verbatim at line 1249 — `if (v13 != (unsigned int *)((long
  long)v13 + 6))`. The clause to add is name-agnostic, since an SSA-pinned one fakes a flip the
  moment numbering shifts: `(\w+) != \(unsigned int \*\)\(\(long long\)\1 \+ 6\)` as
  `stdout_absent`, VERIFIED present today. It is deliberately NOT added yet: until the root
  cause is known it is unproven that the same mechanism removes both, and over-constraining a
  large need on a hunch is how one becomes unclosable. Add it at proposal-approval time, once
  the proposal says whether the comparison falls out of the same fix.
- round 3 (builder `b-r3-large-function-m`): hypothesis OVERTURNED, scope confirmed LARGE, proposal
  filed. `touches` narrowed to `decompiler/crates/kuna-decomp/src/p2_lift` for increment A. The
  acceptance arm is recommended for REPLACEMENT rather than augmentation -- see `## Refutation`.
- round 3 (captain): **ACCEPTANCE RE-CUT**, on the builder's refutation and re-verified
  independently at 2026-09-05 20:40 UTC. `stdout_matches: ["std::string"]` is unsatisfiable on
  this witness for the four reasons in the refutation, and satisfying it would mean guessing a
  type name. Same command, two clauses instead, both measured in both directions on the
  unmodified binary (stock 1267 lines / 63 stores / 0 BCrypt-by-value; 11 x
  `--assert 'flow <a> callreturn'` 1171 / 0 / 1): `stdout_absent`
  `\*\(unsigned long long \*\)&v\d+\[-8\] = 0x1` and `stdout_matches`
  `BCryptGenerateSymmetricKey\(v\d+,`. Both are name-agnostic, so no SSA renumbering can fake a
  flip, and the pair is absent-AND-positive, so it cannot be satisfied by emitting nothing.
  The `(\w+) != \(unsigned int \*\)\(\(long long\)\1 \+ 6\)` clause the round-2 B_PLAN
  note drafted stays OFF this acceptance: it is defect B (`REP STOSB`), a different mechanism,
  and attaching it here would make increment A unclosable.
- round 3 (captain): proposal APPROVED for **increment A only**. B1 (fold `V != V + #c`),
  B2 (`REP STOSB` -> `builtin_memset`) and C (Win32 signature table) stay documented in
  `docs/features/large-function-malformed-output/proposal.md` and are out of scope for the
  closing PR.
- round 3 (builder `b-r3-large-function-m`): increment A IMPLEMENTED and the acceptance PASSES.
  `option fastfailnoreturn` (default on, DIV-119, ElementId 4144, `p2_lift/kuna_fastfailnoreturn.rs`
  + one call site in `flow.rs::setup_callind_specs`): on a Windows image a CALLIND that reads what a
  `swi` CALLOTHER with the 1-byte constant `0x29` wrote in the same instruction is `__fastfail`, so
  its call spec is marked no-return and the halt `check_for_flow_modification` already plants for a
  named no-return callee is planted there. Witness 1267 -> 1147 lines, 63 -> 0 return-address
  stores, 160 -> 149 declarations, `BCryptGenerateSymmetricKey(v52,&v55,v62,v54)`. Whole-image
  sweep: 22 of 858 functions change, all shrinking, gotos 456 -> 390, no named call lost; four more
  Windows PEs swept, the only non-improvement being byte-identical output on the one with no
  `int 0x29`. 0/675 datatest assertions moved and 20.5% faster on the witness. The acceptance
  target is dataset-only, so `verify --promote` refused: `tests/cli/large-function-malformed-output.json`
  carries BOTH clauses verbatim against a vendored 1,536-byte synthesized PE32+
  (`decompiler/crates/kuna-analysis/tests/fixtures/fastfail_x86_64.exe`) that reproduces the same
  shape in both directions. Increments B1/B2/C remain open in `proposal.md`.
