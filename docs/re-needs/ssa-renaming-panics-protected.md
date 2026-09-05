---
need_id: ssa-renaming-panics-protected
title: SSA renaming panics on a protected function
track: quality
status: open
severity: major
probe_id: p-f29a4d139e4a
acceptance_id: a-903a7ff59249
hypothesis_status: overturned
credibility: 0.7
instances: 1
challenges: [629a286b33c5d45b75903c7a]
rounds: [3]
first_seen_round: 3
attempts: 0
covered_by_option: null
touches: [decompiler/crates/kuna-decomp/src/p3_dataflow/heritage.rs]
scope: small
regression_of: null
pr: null
closed_in_round: null
closing_pr: null
reject_reason: null
---

## Symptom

Inspect the function at 0x40ed14 without an internal dataflow failure.

> **SSA renaming panics on a protected function** (major, `629a286b33c5d45b75903c7a`)
> Emitted a failed-function placeholder and exited 1 with an empty-stack panic. Reliable mode reproduced it; decompile-all recorded a per-function error.

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
    "0x40ed14",
    "--addr"
  ],
  "expect": {
    "exit_code": {
      "eq": 1
    },
    "stderr_matches": [
      "rename_recurse: set_input_varnode \\(empty stack\\)"
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
    "0x40ed14",
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
    ],
    "stdout_matches": [
      ";"
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

- The protected instruction stream exposes an unchecked SSA invariant; exact cause unverified.

## Refutation

_not yet refuted_

## Reference

_none recorded_

## Instances

- `629a286b33c5d45b75903c7a` (round 3, tester t-r3-629a286b)

## Decision log

- filed by cluster.py from 1 observation(s)
captain T_DEDUP r3: SPLIT from obs8. Distinct panic site -- p3_dataflow/heritage.rs:3048 `rename_recurse: set_input_varnode (empty stack)` at sub_40ed14. Filed separately as block-processing-panics-out; fixing one will not fix the other.
captain T_TRIAGE r3: track CORRECTED tooling -> quality, touches CORRECTED kuna-cli -> kuna-decomp. Panic site measured on cf5234ac: p3_dataflow/heritage.rs:3048 'rename_recurse: set_input_varnode (empty stack)' on thief_crackme.exe 0x40ed14. BUILDER: STRICT BUG FIX -- do NOT add a phases.toml option row. Quality track is for the counter lease and the mandatory whole-corpus sweep: heritage rename is on every function's path, so a fix that is right here can be silently wrong everywhere else.
captain T_TRIAGE r3: repaired the missing probe/acceptance `target` block (binary_rel + sha256 + size, source dataset) -- without it {{BIN}} could not resolve and the need was unclosable by B_DONE and invisible to regression detection. Verified: acceptance now RUNS and FAILS on cf5234ac, which is the state a filed need must be in.
- round 3 REFUTER: hypothesis **overturned** (was inconclusive). measured on 904d8f88 (origin/main 14895370 + the refute CLI; no engine delta) with the release binary. THE FILED CAUSE IS WRONG ON BOTH HALVES. (a) Nothing is unchecked: an empty variable stack is the NORMAL path taken for every function input (heritage.rs:3040), and what fails is the input CREATION that follows it, so the panic text names the trigger and not the failure. kuna_inputtile::new_tiled_input returns None after discarding the real error at kuna_inputtile.rs:44 (Err(_)) -- that error is KunaError::lowlevel('Overlapping input varnodes') raised at funcdata_varnode.rs:1056, i.e. the full-size input request collides with a non-identical input varnode that already exists. (b) There is NO upstream path to port: Ghidra's Funcdata::setInputVarnode (vendored funcdata_varnode.cc) throws the same LowlevelError from the same single step-back, so kuna's port is faithful and the only divergence is that kuna raises it as a panic instead of a per-function error. A builder chasing 'a missing SSA guard' or 'an un-ported seam' will find neither. WHAT THE FIX ACTUALLY IS, AND THE WRONG-OUTPUT TRAP IN IT. kuna already reconciles ONE overlap shape -- DIV-50/#238's kuna_inputtile, for guardInput's leftover WRITE-MASKED pieces -- and this binary hits a shape that module deliberately declines (a live or straddling input). Its own header states why the write-mask restriction is what makes a mid-rename rewrite safe: a write-masked varnode is never on a VariableStack, a live input can be, so relaxing that check to 'combine whatever overlaps' destroys a varnode a live stack still holds and produces silently wrong output rather than a fixed function. That is the obvious fix and it is the wrong one. SCOPE, MEASURED. 6 of 443 functions in thief_crackme.exe, 4 of 415 in the tester's unpacked image, and 0 of 2135 functions across three ordinary binaries (Cube.exe, illusion.exe, /usr/bin/xxd) -- not a common-path defect. 'Protected' is correlation, not cause: all six failures are self-decrypting PUSHFD/POPFD/XCHG stubs, but structurally identical neighbours (sub_42bc8e and sub_42bd44, the same 91-byte stub shape) decompile cleanly, and all four --mode presets panic identically, so no configuration dodges it. NOT MEASURED: which storage overlaps which input. The flags-vs-eflags idiom in these stubs is the suspicion, unconfirmed -- one eprintln at kuna_inputtile.rs:44 would settle it, and making that error text reach the message is worth doing anyway. ACCEPTANCE STRENGTHENED: exit 0 + absent 'decompilation failed' + absent panicked/un-ported seam was absence-shaped and closable by emitting an empty body, so stdout_matches ';' was added; re-verified after the edit -- the acceptance RUNS and FAILS on 904d8f88, which is the state an open need must be in.
