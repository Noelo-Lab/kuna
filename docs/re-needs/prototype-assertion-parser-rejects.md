---
need_id: prototype-assertion-parser-rejects
title: Prototype assertion parser rejects __stdcall
track: tooling
status: open
severity: major
probe_id: p-2bf0ba72577a
acceptance_id: a-75f3ebee0ff0
hypothesis_status: upheld
credibility: 0.3
instances: 1
challenges: [6442366033c5d43938912a85]
rounds: [7]
first_seen_round: 7
attempts: 0
covered_by_option: null
touches: [decompiler/crates/kuna-cli]
scope: small
regression_of: null
pr: null
closed_in_round: null
closing_pr: null
reject_reason: null
---

## Symptom

Specify the Windows API calling convention while repairing stack arguments.

> **Prototype assertion parser rejects __stdcall** (major, `?`)
> Rejects the address-targeted __stdcall declaration as Bad C syntax, exit 1. The declaration without __stdcall succeeds. Help, assertion documentation, and catalog revealed no documented convention override.

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
    "sub_401ba0",
    "--assert",
    "prototype 0x4050a6 void * __stdcall LoadLibraryExW(unsigned short *name,void *file,unsigned int flags)",
    "--assert-strict"
  ],
  "expect": {
    "exit_code": {
      "eq": 1
    },
    "stderr_matches": [
      "rejected",
      "Bad C syntax"
    ]
  },
  "target": {
    "binary_rel": "bin/Cube.exe",
    "binary_sha256": "6b80dcaa066884e4f7b71373f85aabcdb824d27593c1e4adf113a80e47062aeb",
    "binary_size": 193024,
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
    "sub_401ba0",
    "--assert",
    "prototype 0x4050a6 void * __stdcall LoadLibraryExW(unsigned short *name,void *file,unsigned int flags)",
    "--assert-strict"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stderr_absent": [
      "rejected"
    ]
  },
  "target": {
    "binary_rel": "bin/Cube.exe",
    "binary_sha256": "6b80dcaa066884e4f7b71373f85aabcdb824d27593c1e4adf113a80e47062aeb",
    "binary_size": 193024,
    "binary_source": "dataset"
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- The assertion parser may not support calling-convention qualifiers.

## Refutation

_not yet refuted_

## Reference

_none recorded_

## Instances

- `?` (round ?, tester ?)

## Decision log

- filed by cluster.py from 1 observation(s)
- round 7 REFUTER: hypothesis **upheld** (was inconclusive). REFUTED IN-TICK 2026-09-07 ~10:45Z by the captain. VERDICT: UPHELD, AND THE GAP IS AN EXPLICITLY-NAMED PORT STUB THAT MENTIONS __stdcall BY NAME. This is the cheapest need in the round-7 backlog and it is worth more than its 'major/cred 0.3' filing suggests -- see 3.

REPRO (same arena binary as resource-loader-c-retains: arena/7/6442366033c5d43938912a85/target/Cube.exe):
  kuna decompile $B sub_401ba0 --assert 'prototype 0x4050a6 void * __stdcall LoadLibraryExW(unsigned short *name,void *file,unsigned int flags)' --assert-strict
  -> warning: ... rejected: Bad C syntax
Drop the __stdcall and the SAME declaration is accepted. Reproduces at HEAD.

1. FILE AND LINE. decompiler/crates/kuna-console/src/grammar.rs:1362-1366, CParse::lookup_identifier:
    // glb->hasModel(nm) -> FUNCTION_SPECIFIER. The kuna Architecture has no
    // model registry, so this never fires (it would classify e.g.
    // "__stdcall" as a function specifier).  // STUB(w6-fspec-2)
    IdentClass::Identifier
Upstream classifies any identifier the Architecture knows as a prototype model into a FUNCTION_SPECIFIER token; kuna returns plain Identifier, so '__stdcall' lands where the grammar expects a type specifier and the parse dies as 'Bad C syntax'. Nothing else is missing: the REST OF THE PATH IS ALREADY PORTED -- TypeSpecifiers::function_specifier (grammar.rs:1074), the duplicate guard at 1503-1506 ('Multiple parameter models'), and merge_spec_dec copying it into TypeDeclarator::model (1455) all exist and are dead for want of the lookup. And the models themselves exist: specs/Ghidra/Processors/x86/data/languages/x86win.cspec declares __stdcall, __cdecl, __fastcall and __thiscall. So this is a registry lookup, not a feature.

2. THE ACCEPTANCE IS SATISFIABLE BY A FIX THAT DOES NOTHING -- FLAG THIS AT B_DONE. The acceptance asserts only exit 0 + stderr_absent 'rejected'. A patch that merely SWALLOWS the keyword (lexes it, drops it on the floor, keeps the default __cdecl model) passes verbatim while the declared convention is silently ignored -- which on i386 is the difference between callee-pops and caller-pops and would MIS-model the stack. Require the acceptance (or a stage test) to prove the convention is HONORED: assert a __stdcall-declared callee actually changes the caller's stack behaviour vs the same declaration without it, not just that the error stopped. Same rule the refute-tick memory records for absence needs.

3. WHY THIS NEED IS UNDERPRICED, AND THE CROSS-NEED LINK. It was filed as the tester's WORKAROUND for resource-loader-c-retains (same binary, same API, address 0x4050a6 is the LoadLibraryExW import). I measured the workaround this tick: the plain, convention-less assertion
  --assert 'prototype 0x4050a6 void * LoadLibraryExW(unsigned short *name,void *file,unsigned int flags)'
turns 'LoadLibraryExW();' into 'LoadLibraryExW((unsigned short *)(v1 + v6),file,flags);'. So a locked prototype IS the lever that recovers Win32 import arguments today, and this stub is the only thing standing between an agent and using it in the conventional spelling. Fixing it makes the second, larger need workaroundable from the CLI even before that need is fixed.

TRACK/SCOPE: tooling, small, and it stays small -- one lookup arm plus wherever TypeDeclarator::model has to reach the FuncProto. Do not let it grow into 'add a model registry to Architecture' without a proposal; the cspec already holds the models and the lookup only needs to ask the loaded ProtoModel set.
- captain r7 B_PLAN: challenges was [] so resolve_binary had no hexid and the bound target could not resolve; filled from rounds/7/gate.json (the observation that filed this need).
- round 7 REFUTER: hypothesis **upheld**. CAPTAIN r7 B_PLAN: acceptance is now TARGET-BOUND (challenges/6442366033c5d43938912a85/bin/Cube.exe, sha 6b80dcaa0668...); the record's challenges list was EMPTY, so the probe could not resolve {{BIN}} at all until I filled it from rounds/7/gate.json. Re-measured runnable and still FAILING at sha 0bd41f10: exit_code 1 and stderr still contains 'rejected'. Note credibility is only 0.3 on a single instance -- confirm the symptom reproduces before building, and note that several siblings in this family already shipped (prototype-assertion-rejects-explicit, qualified-parameter-assertions-modify, prototype-assertions-reject-ordinary are all closed), so read their PRs first: the __stdcall keyword may be one row in an existing grammar rather than new machinery.
