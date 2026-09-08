---
need_id: sha-finalization-receives-two
title: SHA finalization receives two invented arguments
track: quality
status: open
severity: major
probe_id: p-3f1dc2c6bdd2
acceptance_id: a-df87d4ade81f
hypothesis_status: upheld
credibility: 1.0
instances: 2
challenges: [65acadf3eef082e477ff5ede, 6869d705aadb6eeafb399027]
rounds: [10]
first_seen_round: 10
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

An accurate three-argument EVP_DigestFinal_ex call.

> **Windows APIs acquire spurious arguments** (minor, `65acadf3eef082e477ff5ede`)
> Emits IsDebuggerPresent(CONCAT44(dat_c,argc)), Sleep(100,0), and Sleep(300,0). Enabling calleearity and varargstackargs retains them. Explicit API prototypes remove the bogus arguments.

> **SHA finalization receives two invented arguments** (major, `6869d705aadb6eeafb399027`)
> Emits EVP_DigestFinal_ex(v3,v9,v7,a3,1). EVP_DigestInit_ex also receives two extra arguments. Recovery options change nothing. Explicit prototypes repair these calls, but other calls then acquire extra arguments, including EVP_DigestUpdate and memset.

## Reproduction

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "target": {
    "binary_rel": "bin/crackmes.rar.__x/crackmes/crackmes.exe",
    "binary_sha256": "1392171ccc244ee24dc5f55987f3992a25a1e46a1f7142b35507023d685f1212",
    "binary_size": 117760,
    "binary_source": "dataset",
    "selector": "sub_14000b970",
    "selector_kind": "name"
  },
  "timeout_s": 60,
  "cmd": [
    "{{KUNA}}",
    "decompile",
    "{{BIN}}",
    "sub_14000b970",
    "--option",
    "calleearity",
    "on",
    "--option",
    "varargstackargs",
    "on"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "EVP_DigestFinal_ex\\([^,\\n]+,[^,\\n]+,[^,\\n]+,[^,\\n]+,[^,\\n]+\\)"
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
    "binary_rel": "bin/crackmes.rar.__x/crackmes/crackmes.exe",
    "binary_sha256": "1392171ccc244ee24dc5f55987f3992a25a1e46a1f7142b35507023d685f1212",
    "binary_size": 117760,
    "binary_source": "dataset",
    "selector": "sub_14000b970",
    "selector_kind": "name"
  },
  "timeout_s": 60,
  "cmd": [
    "{{KUNA}}",
    "decompile",
    "{{BIN}}",
    "sub_14000b970",
    "--option",
    "calleearity",
    "on",
    "--option",
    "varargstackargs",
    "on"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_absent": [
      "EVP_DigestFinal_ex\\([^,\\n]+,[^,\\n]+,[^,\\n]+,[^,\\n]+,[^,\\n]+\\)"
    ]
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- Imported API names are recovered but their fixed prototypes are missing or unlocked.
- Import argument inference mistakes surviving register and stack state for parameters.

## Refutation

_not yet refuted_

## Reference

_none recorded_

## Instances

- `65acadf3eef082e477ff5ede` (round 10, tester t-r10-65acadf3)
- `6869d705aadb6eeafb399027` (round 10, tester t-r10-6869d705)

## Decision log

- filed by cluster.py from 2 observation(s)
- round 10 REFUTER: hypothesis **upheld** (was inconclusive). Refuted by experiment on crackmes.exe (6869d705) at sub_14000b970. Default emits EVP_DigestFinal_ex(v3,v9,v7,a3,1) -- 5 args for a 3-arg function. 'kuna functions --json' shows EVP_DigestFinal_ex at 0x140014568 with size 0, i.e. a body-less IAT import, so the engine has no body to derive arity from and infers it from surviving call-site register/stack state. Supplying the real signature via --assert 'prototype EVP_DigestFinal_ex int EVP_DigestFinal_ex(void *ctx,unsigned char *md,unsigned int *s)' collapses the call to EVP_DigestFinal_ex(ctx,md,v6) -- exactly 3 args, with parameter names propagated. So the filed cause (import argument inference mistakes surviving state for parameters) is correct AND the prototype plumbing already lands correctly on a size-0 import, which means a fix that attaches known import prototypes works through this path. CAVEAT for the builder: prototypes bind by NAME and this binary carries memcmp twice (a local thunk at 0x140012acc size 6 and the import at 0x140014308 size 0), so a name-keyed import type library must decide which of a duplicate pair it binds -- see the overturned verdict on comparison-calls-lose-live, where that duplication defeats the same fix.
- round 10 REFUTER: hypothesis **upheld**. EVIDENCE TRANSCRIBED (captain, tick 10:05Z) -- the upheld verdict was set by the 09:46Z tick, which recorded its experiment ONLY in rounds/10/round.json and left this decision log empty. Copying it here, where a builder actually reads it. Not re-run by me. Experiment on .kuna-repipe/arena/10/6869d705aadb6eeafb399027 (crackmes.exe): default emits EVP_DigestFinal_ex(v3,v9,v7,a3,1) -- five arguments for a three-argument function. kuna functions --json places EVP_DigestFinal_ex at 0x140014568 with SIZE 0, i.e. a body-less IAT import, so its arity is inferred from surviving call-site state. Declaring the prototype collapses the call to EVP_DigestFinal_ex(ctx,md,v6). CONCLUSION THE ORIGINAL TICK DREW: the prototype plumbing DOES reach a size-0 import, so an import type-library fix works through this path. CONTRAST WORTH KNOWING, from this tick's refutation of both-argument-recovery-options: the same symptom there came from a 552-byte STATICALLY-LINKED varargs body, not a size-0 import, so 'no prototype' has at least two distinct sources and a size-0-import fix will not cover the varargs-wrapper case.
