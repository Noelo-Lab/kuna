---
need_id: defined-payload-function-loses
title: Defined payload function loses its incoming context at a direct call
track: quality
status: open
severity: major
probe_id: p-b5e2ca6ff2cc
acceptance_id: a-7683dbcbf7fc
hypothesis_status: upheld
credibility: 1.0
instances: 2
challenges: [68bffbe9224c0ec5dcedbe7a, 69d6e9ba49fa49a2a26023a7]
rounds: [6]
first_seen_round: 6
attempts: 0
covered_by_option: null
touches: [decompiler/crates/kuna-decomp/src/p4_calls]
scope: small
regression_of: null
pr: null
closed_in_round: null
closing_pr: null
reject_reason: null
---

## Symptom

Carry incoming RCX into a defined callee that dereferences it.

> **Defined payload function loses its incoming context at a direct call** (major, `68bffbe9224c0ec5dcedbe7a`)
> With both entries defined and calleearity, varargstackargs, and calleearityfwd enabled, the call remains payload(). The callee decompiles with a context parameter. Disassembly preserves incoming RCX along this branch. An explicit prototype restores the argument.

> **XNextEvent loses its output event pointer** (major, `69d6e9ba49fa49a2a26023a7`)
> Emits XNextEvent with only the display argument despite calleearity and varargstackargs. An explicit two-argument prototype restores the event pointer.

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
    "0x14000b62c",
    "--define-function",
    "0x14000b62c",
    "--define-function",
    "0x14000ca0c=payload",
    "--option",
    "calleearity",
    "on",
    "--option",
    "varargstackargs",
    "on",
    "--option",
    "calleearityfwd",
    "on"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "payload\\(\\);"
    ]
  },
  "target": {
    "binary_rel": "bin/runtime.exe",
    "binary_sha256": "4283a5283c9d5375535da28dff9a98b78c1d660b1b44dcd53dd2c10a2eb85c75",
    "binary_size": 52736,
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
    "0x14000b62c",
    "--define-function",
    "0x14000b62c",
    "--define-function",
    "0x14000ca0c=payload",
    "--option",
    "calleearity",
    "on",
    "--option",
    "varargstackargs",
    "on",
    "--option",
    "calleearityfwd",
    "on"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_absent": [
      "payload\\(\\);"
    ],
    "stdout_matches": [
      "payload\\([^\n)]+\\);"
    ]
  },
  "target": {
    "binary_rel": "bin/runtime.exe",
    "binary_sha256": "4283a5283c9d5375535da28dff9a98b78c1d660b1b44dcd53dd2c10a2eb85c75",
    "binary_size": 52736,
    "binary_source": "dataset"
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- Callee input recovery is not feeding this caller; sibling-call recovery does not cover the single direct call.
- Missing or incomplete Xlib prototype permits the output pointer to be discarded.

## Refutation

_not yet refuted_

## Reference

_none recorded_

## Instances

- `68bffbe9224c0ec5dcedbe7a` (round 6, tester t-r6-68bffbe9)
- `69d6e9ba49fa49a2a26023a7` (round 6, tester t-r6-69d6e9ba)

## Decision log

- filed by cluster.py from 2 observation(s)
- round 6 REFUTER: hypothesis **upheld** (was inconclusive). REFUTED IN-TICK BY CAPTAIN 2026-09-07 06:2xZ, on both instances' own arena binaries with the release decompiler/target/release/kuna at fd19dc13 (never the arena bin/kuna shim). VERDICT UPHELD, and the second hypothesis bullet is the precise one: 'sibling-call recovery does not cover the single direct call'. (1) THE SYMPTOM REPRODUCES VERBATIM. The probe command on runtime.exe renders the else arm as 'payload();' while the other arm renders '(*v1)(0,0,payload,a0,0,0)' -- a CreateThread(NULL,0,payload,a0,0,NULL) whose lpParameter is the caller's own a0. (2) CALLEE INPUT RECOVERY IS NOT THE BROKEN PART -- IT ALREADY WORKS. 'kuna decompile runtime.exe 0x14000ca0c --define-function 0x14000ca0c=payload' alone yields 'unsigned long long payload(unsigned int *a0)'. kuna's own recovery says arity 1; the call site simply never consults it. So the FIRST hypothesis bullet ('callee input recovery is not feeding this caller') is true only as a restatement of the symptom, not as a cause: the recovery ran and produced the right arity. (3) WHY ALL THREE OPTIONS DECLINE, AND IT IS BY DESIGN, NOT A BUG. calleearity, calleearityfwd and calleearitylive are ALL sibling-call reconciliation (phases.toml 2470/2489/2508): each needs ANOTHER call to the same entry address in the same function whose list is already final and longer. I decoded the caller by hand -- there is exactly ONE call to payload and it is the only one there will ever be. At 0x14000b676 'LEA R8,[0x14000ca0c]' (4c 8d 05 8f 13 00 00, next 0x14000b67d + 0x138f) takes payload's ADDRESS for the CreateThread arm; at 0x14000b726 'CALL 0x14000ca0c' (e8 e1 12 00 00, next 0x14000b72b + 0x12e1) is the direct call. An address-taken LEA is not a call site, so the witness search has nothing to find and the family is inert BY CONSTRUCTION. This is the fourth quadrant of the calleearity family -- no sibling AT ALL -- and nothing in the 158-row catalog covers it. (4) THE INCOMING RCX CLAIM IS TRUE. Entry is 'MOV RBX,RCX' @0x14000b649; 'CMP dword[RCX+0x238],EDI' @0x14000b64c; 'JE +0xce' @0x14000b652 lands directly ON the call at 0x14000b726 with nothing in between. RCX is untouched from entry to CALL, so the argument really is the caller's own first parameter and 'payload(a0)' is the correct output. A fix here would NOT produce wrong output on this witness. (5) THE FIX HAS AN EXISTING, SAFE DISCRIMINATOR -- DO NOT RELAX onlyOpUse. calleearitylive already carries the machinery: probe_callee_entry_dead (shared with calleedeadarg) decides whether the callee reads a register before writing it. The missing piece is letting that body evidence stand ALONE as the witness when there is no sibling, rather than only extending a strict-prefix list. phases.toml:2479 states the hazard for the naive route in the project's own words: 'Relaxing that rule directly would fabricate an argument at every test rcx,rcx; jz; call in the corpus.' A builder that widens Funcdata::onlyOpUse instead of adding body evidence will produce wrong output at scale. (6) THE TWO INSTANCES ARE TWO DIFFERENT MECHANISMS -- SPLIT AT T_TRIAGE. The second instance (69d6e9ba, ELF 'almost') is XNextEvent, which readelf shows as 'UND FUNC GLOBAL XNextEvent' with an R_X86_64_JUMP_SLOT at 0x27eb8 -- an IMPORT with no body in the image. Both the sibling search and probe_callee_entry_dead are useless on it (phases.toml:2517 says so explicitly: 'an import has no body to decode'). Closing it needs a known-library-prototype source for Xlib, which shares nothing with the runtime.exe fix. The acceptance a-7683dbcbf7fc only covers the runtime.exe witness, so a builder that closes it buries the Xlib instance. (7) THE ACCEPTANCE ITSELF IS SOUND, unusually for this round: stdout_absent 'payload\\(\\);' PLUS stdout_matches 'payload\\([^\\n)]+\\);' -- it has the positive clause the four probes flagged in the 05:57Z tick lack, so deleting the call cannot make it green. (8) SIDE OBSERVATION, NOT THIS NEED: 'kuna disassemble runtime.exe --addr 0x14000b62c' answers 'is in a non-executable data section, so these are bytes, not instructions', and 'kuna xrefs --to 0x14000ca0c' returns count 0 despite the LEA and the CALL above. Both are real and belong to the PE-section/xref family, not here.
- round 6 T_TRIAGE: TRIAGE: touches -> kuna-decomp/src/p4_calls; scope small -> medium. INSTANCE COVERAGE WARNING: the acceptance a-7683dbcbf7fc only covers the runtime.exe witness (the single-direct-call arity gap). The second instance 69d6e9ba is XNextEvent, a UND import with no body, where both the sibling search and the body probe are useless -- it needs an Xlib known-prototype source and shares no mechanism. Closing this need will NOT fix that instance; re-file it when it recurs rather than treating the closure as covering both.
- captain r7 B_PLAN: scope medium -> small (medium fails needs.validate, so the record could not be written at all); dispatch directly and take the [PROPOSAL] route if the mechanism proves large.
- round 6 REFUTER: hypothesis **upheld**. CAPTAIN r7 B_PLAN: acceptance is now TARGET-BOUND (challenges/68bffbe9224c0ec5dcedbe7a/bin/runtime.exe, sha 4283a5283c9d...) and was re-measured runnable and still FAILING at sha 0bd41f10 -- clause 1 (exit 0) already passes, clauses 2 and 3 are the real criteria, so an exit-0 fix closes nothing. scope was 'medium', which needs.validate REJECTS (small|large only), so the record could not be written at all until I set it to small; that is a filing artifact, not a judgement that the work is trivial -- if the mechanism turns out to be a subsystem rather than a fix inside the existing calleearity/calleearityfwd/varargstackargs family, take the [PROPOSAL] route and say so.
