---
need_id: unsigned-byte-vm-selector
title: Unsigned-byte VM selector gets an unreachable negative case label
track: quality
status: open
severity: major
probe_id: p-541ad2bd1eb0
acceptance_id: a-813d10d96aa3
hypothesis_status: upheld
credibility: 0.7
instances: 1
challenges: [605443e333c5d42c3d016f59]
rounds: [4]
first_seen_round: 4
attempts: 0
covered_by_option: loweredswitchlabels
touches: [decompiler/crates/kuna-decomp/src/p2_lift/kuna_loweredswitch.rs, decompiler/crates/kuna-decomp/src/p2_lift/kuna_loweredswitchlabels.rs, decompiler/crates/kuna-decomp/src/substrate/funcdata_block.rs]
scope: small
regression_of: null
pr: null
closed_in_round: null
closing_pr: null
reject_reason: null
---

## Symptom

A branch that executes for opcode 0x8b.

> **Unsigned-byte VM selector gets an unreachable negative case label** (major, `605443e333c5d42c3d016f59`)
> Declared an unsigned char selector, switched directly on it, and emitted case -0x75. That label cannot match the promoted unsigned byte. Disassembly at 0x14b8 shows CMP AL,0x8b and the XOR handler. switchselector on, reliable mode, and lowered_switch off did not remove the defect.

## Reproduction

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "target": {
    "binary_rel": "decompiler/crates/kuna-analysis/tests/fixtures/katavm_level1_x86_64",
    "in_repo_path": "decompiler/crates/kuna-analysis/tests/fixtures/katavm_level1_x86_64",
    "binary_sha256": "95c300aedc728b643bf97c39b5e8db88e9ddc40bf4cf337cd6c777929684a5f9",
    "binary_size": 28682,
    "binary_source": "in-repo"
  },
  "timeout_s": 60,
  "cmd": [
    "{{KUNA}}",
    "decompile",
    "{{BIN}}",
    "0x12d0",
    "--addr",
    "--option",
    "loweredswitchlabels",
    "off"
  ],
  "expect": {
    "exit_code": { "eq": 0 },
    "stdout_matches": [
      "(?s)unsigned\\s+char\\s+(v[0-9]+)\\s*;.*?switch\\s*\\(\\s*\\1\\s*\\).*?case\\s+-0x75"
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
    "binary_rel": "decompiler/crates/kuna-analysis/tests/fixtures/katavm_level1_x86_64",
    "in_repo_path": "decompiler/crates/kuna-analysis/tests/fixtures/katavm_level1_x86_64",
    "binary_sha256": "95c300aedc728b643bf97c39b5e8db88e9ddc40bf4cf337cd6c777929684a5f9",
    "binary_size": 28682,
    "binary_source": "in-repo"
  },
  "timeout_s": 60,
  "cmd": [
    "{{KUNA}}",
    "decompile",
    "{{BIN}}",
    "0x12d0",
    "--addr"
  ],
  "expect": {
    "exit_code": { "eq": 0 },
    "stdout_matches": [
      "(?s)unsigned\\s+char\\s+(v[0-9]+)\\s*;.*?switch\\s*\\(\\s*\\1\\s*\\).*?case\\s+0x8b"
    ],
    "stdout_absent": [
      "(?s)unsigned\\s+char\\s+(v[0-9]+)\\s*;.*?switch\\s*\\(\\s*\\1\\s*\\).*?case\\s+-"
    ]
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- Case formatting may sign-extend an 8-bit constant without consulting the selector type.

## Refutation

_not yet refuted_

## Reference

_none recorded_

## Instances

- `605443e333c5d42c3d016f59` (round 4, tester t-r4-605443e3)

## Decision log

- filed by cluster.py from 1 observation(s)
- split out of the round-4 `16-byte-vm-state` mega-bucket by the captain at T_DEDUP: cluster.py's key is `kind|subcommand|clause-shape`, so nine unrelated wrong-output `decompile` defects collapsed into one need whose probe covered only the first. Each carries its own probe and acceptance from its own observation.
- round 4 REFUTER: hypothesis **upheld** (was inconclusive). UPHELD, and the exact line is named. printc.rs:4248-4258 does consult a signedness flag (kuna_has_signed_labels), but the flag is set in substrate/funcdata_block.rs:2145 as signed = case_vals.iter().any(|v| v & signbit != 0) with signbit taken at the switch-VARIABLE width -- i.e. purely from whether some recovered case constant has its top bit set, never from the selector's data type nor from the signedness of the compares that built the cascade. With var_size 1 and a case value 0x8b, bit 7 is set, so EVERY label on that table renders signed and 0x8b prints as -0x75. Reproduced on sha c5f4073a: decompile 0x12d0 --addr gives unsigned char v17, switch(v17), and labels 0x1e 0x3e 0x4a 0x5d 0x7c then -0x75 -- the small labels are identical either way, which is why only one looks wrong. REFUTER WARNING FOR THE BUILDER: do NOT just delete the signed rendering. The flag exists for the getopt idiom (case -1 over a signed int), and the code comment says so; dropping it regresses that to case 0xffffffff. The fix is to derive signedness from the selector type / the SLESS-vs-LESS of the recovered compares, so a 1-byte unsigned selector keeps 0x8b. TESTER ABLATION WAS INVALID: the need says lowered_switch off did not remove the defect, but there is no such option -- it is loweredswitch. With --option loweredswitch off the switch is gone entirely (if (v17 == 0xaa) goto label_1470) in 0.57s, so the install IS the path. RESOLVED IN THE SAME TICK, no anomaly: an unrecognized --option NAME VALUE is SILENTLY ACCEPTED -- rc=0, nothing on stderr, output byte-identical to the default. Matrix on this command: loweredswitch off removes the switch; lowered_switch, LOWERED_SWITCH, loweredswitc and zzzz all leave it standing at rc=0 with an empty stderr. So the tester's ablation never ran, and the silent-accept is itself a tooling-track defect worth filing: any tester A/B with a typo'd option name reads as 'the option changed nothing'.
- candidate builder: vendored the exact 28,682-byte witness unchanged and retargeted both arms to its in-repo path so the probes are runnable without the private dataset.
- candidate builder: changed the reproduction command to explicit `--option loweredswitchlabels off`, which preserves the historical negative-label behavior after the default-on repair, and strengthened acceptance to require `case 0x8b` while forbidding a negative case on the unsigned-byte switch. Because command/expect identity changed, the derived IDs move from `p-ada30c2012ba` / `a-08dc77c2147e` to `p-541ad2bd1eb0` / `a-813d10d96aa3`.
- candidate builder: corrected `covered_by_option` and `touches` to the P2 recovery/metadata path; the generic P9 numeric printer remains untouched.
