---
need_id: 16-byte-vm-state
title: 16-byte VM state swaps emit as one-byte array assignments
track: quality
status: open
severity: major
probe_id: p-e4776a9db8d0
acceptance_id: a-3185ecea5bdd
hypothesis_status: upheld
credibility: 0.85
instances: 1
challenges: [605443e333c5d42c3d016f59]
rounds: [4]
first_seen_round: 4
attempts: 0
covered_by_option: null
touches: [decompiler/crates/kuna-decomp/src/p9_emit/printc.rs, decompiler/crates/kuna-decomp/src/p6_variables]
scope: small
regression_of: null
pr: null
closed_in_round: null
closing_pr: null
reject_reason: null
---

## Symptom

Faithful pseudocode for the VM's two four-word register banks.

> **16-byte VM state swaps emit as one-byte array assignments** (major, `605443e333c5d42c3d016f59`)
> Declared char arrays of length 16 but emitted v30[0] = v32[0] for a MOVAPS xmmword transfer. Initial vector zero stores also became single-byte stores. arraynotation off did not repair this; unsigned int[4] overrides still produced invalid array/scalar assignments. Native disassembly confirms full-width transfers at 0x1410-0x1428.

## Reproduction

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "target": {
    "binary_rel": "bin/KataVM_Level_1.7z.__x/KataVM_Level_1/KataVM_L1",
    "binary_sha256": "95c300aedc728b643bf97c39b5e8db88e9ddc40bf4cf337cd6c777929684a5f9",
    "binary_size": 28682,
    "binary_source": "dataset"
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
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "(?s)char\\s+(v[0-9]+)\\s*\\[\\s*16\\s*\\];.*?char\\s+(v[0-9]+)\\s*\\[\\s*16\\s*\\];.*?\\1\\s*\\[\\s*0\\s*\\]\\s*=\\s*\\2\\s*\\[\\s*0\\s*\\]\\s*;"
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
    "binary_rel": "bin/KataVM_Level_1.7z.__x/KataVM_Level_1/KataVM_L1",
    "binary_sha256": "95c300aedc728b643bf97c39b5e8db88e9ddc40bf4cf337cd6c777929684a5f9",
    "binary_size": 28682,
    "binary_source": "dataset"
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
    "exit_code": {
      "eq": 0
    },
    "stdout_absent": [
      "v[0-9]+\\s*\\[\\s*0\\s*\\]\\s*=\\s*v[0-9]+\\s*\\[\\s*0\\s*\\]\\s*;"
    ]
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- Array lvalue emission may select element zero without preserving the 16-byte access width.

## Refutation

_not yet refuted_

## Reference

- `ida-decompile load target/KataVM_Level_1.7z.__x/KataVM_Level_1/KataVM_L1` — Reference unavailable: server dcb658a893 exited with status 1 before registering. No IDA pseudocode was obtained. Kuna's disassembly independently confirms xmmword accesses.

## Instances

- `605443e333c5d42c3d016f59` (round 4, tester t-r4-605443e3)

## Decision log

- filed by cluster.py from 1 observation(s)
- split out of the round-4 `16-byte-vm-state` mega-bucket by the captain at T_DEDUP: cluster.py's key is `kind|subcommand|clause-shape`, so nine unrelated wrong-output `decompile` defects collapsed into one need whose probe covered only the first. Each carries its own probe and acceptance from its own observation.
- round 4 REFUTER: hypothesis **upheld** (was inconclusive). UPHELD at a named line, and the width survives all the way to the printer. Ran on sha 6188e204 against the witness .kuna-repipe/arena/4/605443e333c5d42c3d016f59/.../KataVM_L1. (1) The machine really does swap two xmmwords: raw bytes at 0x1410 are movdqa (%rsp),%xmm0 / movdqa 0x10(%rsp),%xmm6 / movaps %xmm6,(%rsp) / movaps %xmm0,0x10(%rsp). (2) The IR keeps the full width -- 'load addr 0x12d0' + 'print raw' in decomp_dbg gives 0x1424:3739 s0x..c488:16 = s0x..c498:16 and 0x1428:378f s0x..c498:16 = u0x10002333:16, i.e. 16-byte COPYs, and every MULTIEQUAL feeding them is :16 as well. So p2-p6 model the swap correctly; nothing upstream truncates. (3) The loss is the emitter, at printc.rs:7273 -- the whole-array 'name[index]' branch gates on 'sym_off >= 0 && (sym_off % elsize) == 0 && st.get_size() > elsize' and NEVER consults v.get_size(). For char v30[16] that is elsize=1, sym_off=0, 16>1 -> index 0 -> 'v30[0]', a one-byte lvalue for a sixteen-byte access. This is the tester's hypothesis verbatim. (4) The existing repair has a hole exactly here: the comment at printc.rs:7208 says routing plain ARRAY through push_partial_symbol_ir already fixed 'an 8-byte write at offset 0 of an undefined1[16]'. It does not fire for a FULL-width cover -- push_partial_symbol_ir's first loop test (off==0 && sz==cur.get_size()) breaks immediately with an empty stack and returns false, so a 16-byte access on a 16-byte array falls straight through to the unguarded branch. A partial access is repaired ('v30._0_4_' does appear in this same function at the 0x1470 reads); only the whole-array case is wrong. (5) FOR THE BUILDER: a bare-name fallback is NOT enough. Suppressing the subscript yields 'v13 = v30;' and 'v30 = 0;', which is still invalid C -- arrays are not assignable. The render itself has to change (a 16-byte-wide cast, or typing the location as a scalar), so this is not a one-line guard. (6) THE ACCEPTANCE IS GAMEABLE: it is stdout_absent of a regex anchored on the literal 'char vN [16]' spelling, so re-typing the two banks to 'undefined1 v30[16]' passes the probe while 'v30[0] = v32[0]' is still emitted. Tighten it at T_TRIAGE to forbid the element-zero assignment itself, independent of the declared type.
