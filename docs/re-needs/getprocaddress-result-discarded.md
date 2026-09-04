---
need_id: getprocaddress-result-discarded
title: a GetProcAddress result is discarded and the pointer left uninitialised
track: quality
status: open
severity: major
probe_id: p-ac20c1602ca0
acceptance_id: a-f2df446f39d5
hypothesis_status: inconclusive
credibility: 0.8
instances: 1
challenges: [65acadf3eef082e477ff5ede]
rounds: [1, 2]
first_seen_round: 1
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

Filed by a round-1 tester; gated `admitted`. Verbatim observation and replayed transcript in `.kuna-repipe/rounds/1/gate.json` (result index 9).

> **GetProcAddress result is discarded and an uninitialized pointer is called** (blocker, `65acadf3eef082e477ff5ede`)
> *What I wanted:* dataflow connecting the result of GetProcAddress to the subsequent indirect call.
> *What kuna did:* emitted `v12 = (void *)GetProcAddress(...)`, then called `(*v108)(v113,0,v27,0x30)` even though v108 was initialized to NULL and never assigned the result.

Still reproduces verbatim on main at 3ae69cbc (captain replay, round 2), in `sub_140002530` of ObfuscationFiesta.exe:

```c
    v108 = NULL;
    v12 = (void *)GetProcAddress(v107,"NtQueryInformationProcess");
    if (v12) {
      v113 = GetCurrentProcess();
      v128 = v108;
      v44 = (int *)&v25[1];
      v7 = (*v108)(v113,0,v27,0x30);
```

The store of the GetProcAddress return value into the stack slot rendered as `v108` is dropped, so
every later read of that slot sees the NULL initialiser. The SECOND site in the same function is
worse still -- the call does not even go through the NULLed local:

```c
    v98 = NULL;
    v12 = (void *)GetProcAddress(v17,"NtSetInformationThread");
    if (v12)
      (*v133)(GetCurrentThread(),v100,0,0);
```

This is a wrong-output defect, not a rendering nit: an agent reading this decompilation concludes the
program calls a null pointer, and the anti-debugging logic that the whole challenge turns on is
unreadable.

## Reproduction

Passes today == the bug is present.

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "timeout_s": 60,
  "cmd": [
    "{{KUNA}}",
    "decompile",
    "{{BIN}}",
    "sub_140002530"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "\\(\\*v108\\)\\(v113,0,v27,0x30\\)"
    ]
  },
  "cwd": "{{WORK}}",
  "repeat": 1,
  "probe_id": "p-ac20c1602ca0",
  "target": {
    "binary_rel": "bin/ObfuscationFiesta.exe",
    "binary_sha256": "b852fd5bfa65f7d71b903fb1852ae8ad03f2611637a42d501a897424647deec9",
    "binary_size": 32768,
    "binary_source": "dataset",
    "selector": "sub_140002530",
    "selector_kind": "name"
  }
}
```

## Acceptance

Fails today == the gap is open. **This is a re-authored acceptance (captain, round 2); the round-1
one was `stdout contains "(*v12)(v113,0,v27,0x30)"`, which pinned exact variable numbering and would
have stayed FAIL under a correct fix that renumbered anything.** The replacement asserts the
invariant instead, in two clauses:

- `stdout_matches` -- the indirect call must still be emitted (a "fix" that deletes the call, or
  gives up on the function, does not count).
- `stdout_absent` -- the bug SHAPE must be gone: no local is set to `NULL` and then used as an
  indirect-call target within ten lines *without an intervening assignment to that same local*. The
  tempered `(?!\s*\1 = )` repetition is what makes a correct copy-chain rendering
  (`v108 = v12; ... (*v108)(...)`) pass rather than falsely fail.

Verified by the captain at 3ae69cbc against the real output: FAILS today (bug shape present), and
PASSES on both plausible fixed renderings (`(*v12)(...)` after slot merging, and the copy-chain
form). Note `v108 = NULL;` on its own is NOT asserted absent -- the source really does initialise
the pointer, so a correct decompilation may legitimately keep that line.

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "timeout_s": 120,
  "cmd": [
    "{{KUNA}}",
    "decompile",
    "{{BIN}}",
    "sub_140002530"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "\\(\\*v\\d+\\)\\(v\\d+,0,v\\d+,0x30\\)"
    ],
    "stdout_absent": [
      "(v\\d+) = NULL;\\n(?:(?!\\s*\\1 = )[^\\n]*\\n){0,10}?[^\\n]*\\(\\*\\1\\)\\("
    ]
  },
  "cwd": "{{WORK}}",
  "repeat": 1,
  "probe_id": "a-f2df446f39d5",
  "target": {
    "binary_rel": "bin/ObfuscationFiesta.exe",
    "binary_sha256": "b852fd5bfa65f7d71b903fb1852ae8ad03f2611637a42d501a897424647deec9",
    "binary_size": 32768,
    "binary_source": "dataset",
    "selector": "sub_140002530",
    "selector_kind": "name"
  }
}
```

## Hypothesis

ADVISORY -- the builder is not bound to it. The tester guessed "stack-variable recovery split one
logical local into v12 and v108 across obfuscated control flow". The captain's replay adds one fact
the tester did not have: the second call site targets `v133`, a slot unrelated to both `v12` and the
NULLed `v98`, which reads less like a split local and more like the store being attributed to the
wrong stack offset (or dropped entirely) under this binary's control-flow obfuscation. Challenge
`65acadf3eef082e477ff5ede` is tagged `Anti-debugging` + `Control-flow obfuscation`, band 4.

## Instances

- round 1, tester `t-r1-65acadf3`, challenge `65acadf3eef082e477ff5ede` (ObfuscationFiesta.exe, PE
  x86-64), cost 3 min, severity blocker. The same tester also filed the wide-string and prototype
  observations against the same function.

## Decision log

- round 1: gated `admitted`, NOT built (round 1 closed 12 of 23). Carried into round 2.
- round 2 wave 1: dispatched, killed by the account-429 wipeout before any commit. Branch
  `feat/re-getprocaddress-result-discarded` exists locally with no PR and no `docs/features/` record;
  treat it as empty, not as salvage.
- round 2, captain tick 2026-09-04: the need had lost BOTH probe fences (frontmatter carried no
  `probe_id`/`acceptance_id`), so it was unclosable by construction -- `apply-acceptance` had nothing
  to evaluate. Both restored above; the reproduction probe keeps its round-1 identity
  `p-ac20c1602ca0` (probe ids hash cmd+expect only, so adding the `target` block did not move it),
  and the acceptance is NEW (`a-f2df446f39d5`) for the brittleness reason recorded above. A `target`
  block was added to both so the acceptance suite can resolve `{{BIN}}` from the dataset without an
  arena. The probe is NOT vendorable into `tests/cli/` -- `binary_source: dataset`, and CI has no
  dataset -- so closing this need will not add a CLI regression test; say so in the PR rather than
  faking one.
