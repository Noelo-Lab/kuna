---
need_id: local-type-assertion-target
title: Local type assertion cannot target the register temporary printed by kuna
track: tooling
status: open
severity: major
probe_id: p-cf0aec905051
acceptance_id: a-bb33af961c53
hypothesis_status: upheld
credibility: 0.7
instances: 1
challenges: [69761b7a39e9c4d85c2f9fc1]
rounds: [12]
first_seen_round: 12
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

Repair the VM temporary through the documented --assert type interface.

> **Local type assertion cannot target the register temporary printed by kuna** (major, `69761b7a39e9c4d85c2f9fc1`)
> The emitted temporary v6 is absent from the variables inventory. Its type assertion is rejected with No symbol named. Switching to namestyle ghidra and targeting the emitted uVar6 also fails.

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
    "sub_1005350",
    "--option",
    "calleearity",
    "on",
    "--option",
    "varargstackargs",
    "on",
    "--assert",
    "type v6 unsigned long *",
    "--json"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "json": [
      {
        "path": "assertions[0].status",
        "op": "eq",
        "value": "rejected"
      }
    ],
    "stderr_matches": [
      "No symbol named:"
    ]
  },
  "target": {
    "binary_rel": "bin/graphy-release.zip.__x/graphy",
    "binary_sha256": "1fb1f75b6a3939e3d80a25ca65aeb092d62a6968a53bd1db499a7cee864bed42",
    "binary_size": 40472,
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
    "sub_1005350",
    "--option",
    "calleearity",
    "on",
    "--option",
    "varargstackargs",
    "on",
    "--assert",
    "type v6 unsigned long *",
    "--json"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "json": [
      {
        "path": "assertions[0].status",
        "op": "eq",
        "value": "applied"
      }
    ],
    "stderr_absent": [
      "No symbol named:"
    ]
  },
  "target": {
    "binary_rel": "bin/graphy-release.zip.__x/graphy",
    "binary_sha256": "1fb1f75b6a3939e3d80a25ca65aeb092d62a6968a53bd1db499a7cee864bed42",
    "binary_size": 40472,
    "binary_source": "dataset"
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- The assertion resolver may only see backed symbols while emission invents names for register temporaries.

## Refutation

_not yet refuted_

## Reference

_none recorded_

## Instances

- `69761b7a39e9c4d85c2f9fc1` (round 12, tester t-r12-69761b7a)

## Decision log

- filed by cluster.py from 1 observation(s)
- split by captain at T_DEDUP from round 12's `missing-capability|decompile|exit_code,json,stderr_absent` bucket: cluster.py's signature (kind|subcommand|clause-shape) collapsed unrelated defects, so the crop was hand-partitioned one observation per need and filed via `--from-file`. See `.kuna-repipe/rounds/12/dedup/MARKER.json`; do NOT run `cluster --round 12` bare.
- round 12 REFUTER: hypothesis **upheld** (was inconclusive). round 12 REFUTER (captain, tick 05:3xZ) -- VERDICT UPHELD, and the control run pins the namespace boundary exactly. Refuted jointly with vm-temporary-declared-as (same challenge, same function sub_1005350, one shared measurement); read that need's log for the type half.

THE CONTROL, on aa18e56d, seven single-assert runs of kuna decompile <graphy> sub_1005350 --assert "<D>" --json, all exit 0:
  type v6  unsigned long *  -> REJECTED, "No symbol named: v6"    (register temp, // rax)
  name v6  vmtop            -> REJECTED, "No symbol named: v6"    (so it is not a type-directive quirk)
  type v2  char[16]         -> REJECTED, "No symbol named: v2"    (register temp)
  type v10 unsigned long *  -> APPLIED                            (stack slot, 512 bytes)
  type v12 unsigned long *  -> APPLIED                            (stack slot at -0x38)
  type a0  unsigned long *  -> APPLIED                            (parameter, by its C-printed name)
  type local_30 unsigned long * -> REJECTED, "No symbol named: local_30"
That is the whole boundary in one table. Everything the local scope backs with a Symbol resolves; every register temporary the emitter prints does not. The hypothesis as filed -- the resolver only sees backed symbols while emission invents names for register temporaries -- is exactly what the table says. UPHELD, and unusually cleanly for this round: the discriminator is a pair of runs differing only in whether the target lives in a stack slot.

THE DOCUMENTATION ALREADY PROMISES THE CAPABILITY, which raises this above a missing-feature ask. kuna decompile --help gives as its worked examples of the assert vocabulary, verbatim: "type v2 char[16]" and "name v2 credbuf". In this function v2 is a register temporary and BOTH of those exact forms are rejected. The help text teaches the failing form. Whoever takes this should treat the help text as the specification the code must meet, not as prose to be edited down to match the code.

TWO CORRECTIONS TO THE FILED SYMPTOM, both measured, neither fatal to it:
1. "The emitted temporary v6 is absent from the variables inventory" is TRUE and is stronger than filed: the --json variables array for this function holds 12 entries -- 3 args and 9 stack slots -- and contains ZERO register variables. v1 through v9 and v11, every register temporary the C body declares, are all absent. So this is not v6 specifically; kuna exports no handle for any register-resident local in any function.
2. But the inventory is NOT the resolver's namespace, and a builder who assumes it is will build the wrong thing. local_30 IS in the JSON inventory and is still rejected, while a0 is NOT in the JSON inventory under that name (the JSON calls the three parameters param_1, param_2, param_3 while the C body prints them a0, a1, a2) and resolves fine. There are three distinct name spaces here -- what the C body prints, what --json reports, and what --assert resolves -- and no two of them agree. Exporting the register temps into the JSON inventory alone will NOT make them assertable.

WHAT THE FIX HAS TO BE, since the two above rule out the cheap versions: the assert resolver has to be able to name a HighVariable that has no backing Symbol, either by adopting the emitter's printed identifier as a lookup key or by accepting a storage-based target form. There is no such form today -- the vocabulary line in --help is function, typedef, prototype, data, param, return, name, type, comment, flow, readonly, volatile, and none of them takes a register or an SSA site. Doing this by minting a real ScopeLocal Symbol for each register high is the obvious route and is also the risky one: it changes what p6 believes about the local scope, which is the merge machinery this round has repeatedly warned builders away from.

ACCEPTANCE IS SOUND AND NOT GAMEABLE, but it is narrow. It demands assertions[0].status == applied for exactly "type v6 unsigned long *" with no "No symbol named:" on stderr. Two notes. First, it is coupled to the emitted identifier staying v6 -- any unrelated change that renumbers the temporaries turns this need falsely regressed, so whoever closes it should add a second clause on a differently-named temp or on the register-temp count in --json. Second, applying this exact assertion is also a plausible route to closing the sibling vm-temporary-declared-as, so the two needs should be dispatched to ONE builder or explicitly kept apart; closing this one makes the sibling's defect user-workaroundable without fixing it.

DEAD LINE CLOSED: the filed probe pins --option calleearity on and --option varargstackargs on. Measured irrelevant -- every run in the table above omitted both options entirely and reproduced the same rejections and the same applies. Drop them before dispatch; they aim a builder at call/argument recovery, which is this round's recurring wrong turn.
