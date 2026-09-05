---
need_id: block-processing-panics-out
title: Block processing panics with an out-of-bounds index
track: quality
status: open
severity: major
probe_id: p-84fba836c289
acceptance_id: a-5d5e1d9200d1
hypothesis_status: upheld
credibility: 0.7
instances: 1
challenges: [629a286b33c5d45b75903c7a]
rounds: [3]
first_seen_round: 3
attempts: 0
covered_by_option: null
touches: [decompiler/crates/kuna-decomp/src/substrate/block.rs]
scope: small
regression_of: null
pr: null
closed_in_round: null
closing_pr: null
reject_reason: null
---

## Symptom

Inspect the function at 0x41cd08 without an internal block-processing failure.

> **Block processing panics with an out-of-bounds index** (major, `629a286b33c5d45b75903c7a`)
> Exited 1 with a block.rs panic: index out of bounds. Reliable mode reproduced it; decompile-all recorded a per-function error.

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
    "0x41cd08",
    "--addr"
  ],
  "expect": {
    "exit_code": {
      "eq": 1
    },
    "stderr_matches": [
      "index out of bounds"
    ]
  },
  "target": {
    "binary_rel": "bin/thief_crackme.exe",
    "binary_sha256": "d176b3254177ecf03709513a72a52941a6031002f64e9f3681568f03dd96f0fc",
    "binary_size": 184320,
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
    "0x41cd08",
    "--addr"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_absent": [
      "decompilation failed"
    ],
    "stderr_absent": [
      "panicked",
      "un-ported seam"
    ]
  },
  "target": {
    "binary_rel": "bin/thief_crackme.exe",
    "binary_sha256": "d176b3254177ecf03709513a72a52941a6031002f64e9f3681568f03dd96f0fc",
    "binary_size": 184320,
    "binary_source": "dataset"
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- An unchecked block invariant fails on the protected instruction stream; exact cause unverified.

## Refutation

_not yet refuted_

## Reference

_none recorded_

## Instances

- `629a286b33c5d45b75903c7a` (round 3, tester t-r3-629a286b)

## Decision log

- filed by cluster.py from 1 observation(s)
captain T_DEDUP r3: SPLIT from ssa-renaming-panics-protected, which the deterministic pass merged on kind+shape. Distinct panic site -- substrate/block.rs:648 `index out of bounds: the len is 1 but the index is 1` at sub_41cd08. Same binary, different phase.
captain T_TRIAGE r3: track CORRECTED tooling -> quality, touches CORRECTED kuna-cli -> kuna-decomp. cluster.py sends every non-`functions` crash to tooling, but the panic site is measured: substrate/block.rs:648 'index out of bounds: the len is 1 but the index is 1' (LOSS-131 seam) on thief_crackme.exe 0x41cd08. BUILDER: this is a STRICT BUG FIX -- do NOT add a phases.toml option row; a panic that yields a WARNING stub instead of C is not a judgement call. The quality track is assigned for the counter lease and the whole-corpus sweep discipline, because an unguarded index fix in ported substrate code can silently move output far from this witness.
captain T_TRIAGE r3: repaired the missing probe/acceptance `target` block (binary_rel + sha256 + size, source dataset) -- without it {{BIN}} could not resolve and the need was unclosable by B_DONE and invisible to regression detection. Verified: acceptance now RUNS and FAILS on cf5234ac, which is the state a filed need must be in.
- round 3 REFUTER: hypothesis **upheld** (was inconclusive). Captain refuted BY MEASUREMENT in-tick (main-tree kuna, 20:01 build). The filed guess -- 'an unchecked block invariant fails' -- is UPHELD, and the invariant is now named: RUST_BACKTRACE puts the panic at substrate/block.rs:648 inside FlowBlock::get_in, called from funcdata_block.rs:1334 'let inbl = self.bblocks_ref().block(parent).get_in(i)' in descend2_undef, reached via block_remove_internal(unreachable=true) <- remove_unreachable_blocks. i is the MULTIEQUAL's slot for the dead varnode (get_slot, line 1327), so 'len is 1 but the index is 1' means the phi still has 2 inputs while its parent block has 1 in-edge: MULTIEQUAL arity and parent in-degree are already desynced BEFORE descend2_undef runs. Note the unreachable arm SKIPS the whole push_multiequals + per-out-edge op_remove_input resync that the !unreachable arm does (funcdata_block.rs:1397-1419), which is where the ordering has to be looked at. TWO CORRECTIONS FOR THE BUILDER. (1) Nothing measured here is specific to the 'protected instruction stream' -- the path is generic unreachable-block removal; the packer only supplies the dead-block shape. Do not scope the fix to obfuscated input. (2) THE ACCEPTANCE PROBE CANNOT TELL A FIX FROM A COVER-UP: it only asks for exit 0 with 'panicked' absent from stderr, so a bounds guard (or a clamp/skip at block.rs:648) closes this need while emitting a phi with a dropped or mis-wired input -- silently WRONG C instead of a crash. The fix must restore the arity invariant, and the builder should diff the emitted C for sub_41cd08 against the surrounding call graph rather than trusting the absence probe.
