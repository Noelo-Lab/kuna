---
need_id: hint-printing-call-loses
title: Hint-printing call loses its live buffer argument after an XOR loop
track: quality
status: open
severity: major
probe_id: p-89d12f7524fc
acceptance_id: a-2d1da99d5a50
hypothesis_status: overturned
credibility: 1.0
instances: 3
challenges: [5e51836e33c5d4439bb2dc24, 6a09581717539b5175d122b9, 6a148dd62b3df128c1df5c9e]
rounds: [9]
first_seen_round: 9
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

See which decoded buffer the hint printer consumes.

> **Hint-printing call loses its live buffer argument after an XOR loop** (major, `5e51836e33c5d4439bb2dc24`)
> Emits a zero-argument call after decoding the buffer. Disassembly sets RDI to the buffer at 0x400f58 and preserves it through the call at 0x401007. Both recovery options leave the argument absent; disabling calleedeadarg also changes nothing. An explicit prototype restores it. Probe passed and acceptance failed.

> **Memcpy loses its third argument with both recovery options enabled** (major, `6a09581717539b5175d122b9`)
> Emits two-argument memcpy. Enabling calleearity and varargstackargs leaves checker C identical. Kuna disassembly shows R8, RDX and RCX initialized immediately before the call to the memcpy import thunk.

> **VM state initializer loses key, length and seed arguments** (major, `6a148dd62b3df128c1df5c9e`)
> Emits init_state(v47). Assembly loads R9D with the seed, R8D with 0x20 and RDX with the key address immediately before the call. Independently decompiling the callee recovers four used parameters. Both recovery options leave the call unchanged. Their catalog descriptions cover narrower sibling-call and variadic cases, so this is not claimed as a regression.

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
    "0x400f30",
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
      "(?m)^  [A-Za-z_]\\w*\\(\\);"
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
    "binary_rel": "bin/cr4ckm3_0x.tgz.__x/cr4ckm3_0x.tar.__x/cr4ckm3_0x_/cr4ckm3_0x",
    "binary_sha256": "cf9eb23ba4e4f9fc4b86acb1501664cbdece60c3aadadc1e8a65b70ca3a75a01",
    "binary_size": 807352,
    "binary_source": "dataset"
  },
  "timeout_s": 60,
  "cmd": [
    "{{KUNA}}",
    "decompile",
    "{{BIN}}",
    "0x400f30",
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
      "(?m)^  [A-Za-z_]\\w*\\(\\);"
    ],
    "stdout_matches": [
      "sub_417f80\\([^)]"
    ]
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- The loop also consumes the buffer pointer, and single-site callee analysis does not establish its arity.
- Unlocked call-site recovery does not use the independently recovered callee signature.

## Refutation

_not yet refuted_

## Reference

- `ida-decompile decompile 0x400f30 --binary target/cr4ckm3_0x.tgz.__x/cr4ckm3_0x.tar.__x/cr4ckm3_0x_/cr4ckm3_0x --raw` — No reference obtained: no matching server, followed by a failed load whose server exited before registering.

## Instances

- `5e51836e33c5d4439bb2dc24` (round 9, tester t-r9-5e51836e)
- `6a09581717539b5175d122b9` (round 9, tester t-r9-6a095817)
- `6a148dd62b3df128c1df5c9e` (round 9, tester t-r9-6a148dd6)

## Decision log

- filed by cluster.py from 3 observation(s)
- captain T_DEDUP r9: MERGED BY HAND from 3 observations on 3 challenges/3 testers (5e51836e `0x400f30`, 6a095817 `sub_140001740` memcpy loses its 3rd argument, 6a148dd6 `sub_140007540` init_state loses key/length/seed). All three run `--option calleearity on --option varargstackargs on` and still get a truncated argument run at the call site; the witness is the ELF one because its probe needs no `--define-function` and no `funcboundflow off`. Two things for T_REFUTE: (a) the two non-witness halves lose their own acceptance clauses and will close for free, and the witness is a TOTAL loss (zero args) while the others are PARTIAL (last argument dropped) -- that may be two root causes; (b) the round-4 need `argument-recovery-options-still` (closed, 3 instances) is the same family, so if this reproduces on ITS binary it is a genuine regression and should be linked -- I did not set `regression_of` without that evidence. The 6a095817 member arrived carrying `regression_of: call-args-discarded` (closed); witness selection dropped it, which is deliberate -- verify before restoring it.
- round 9 REFUTER: hypothesis **overturned** (was inconclusive). captain T_REFUTE r9 (in-tick, no refuter role in the launcher). SYMPTOM STANDS, FILED CAUSE IS WRONG, AND THE MECHANISM IS NOW PINNED TO ONE LINE.

REPRODUCED. On 8df2a52e, `kuna decompile <arena>/cr4ckm3_0x 0x400f30` emits `sub_417f80();` at line 47 and RDI really is live: 0x400f58 `LEA RDI,[RBP-0x30]` and nothing redefines RDI before `CALL 0x417f80` at 0x401007. `--assert 'prototype sub_417f80 long sub_417f80(char *s)'` restores `sub_417f80((char *)v2);`, so the caller-side dataflow is intact and the loss is entirely in unlocked call-site recovery.

THE TESTER'S ABLATIONS ARE ALL VACUOUS AND I RE-RAN THEM. calleearity, calleearityfwd, calleearitylive, calleearitybody, calleearitycut, varargstackargs and calleedeadarg are EVERY ONE default ON, so 'I enabled it and nothing changed' proves nothing. I ran the real A/B (each OFF, plus spillargtrial reload/spill): all seven leave line 47 byte-identical. Do not re-derive this.

WHY THE HYPOTHESIS IS OVERTURNED. It says 'single-site callee analysis does not establish its arity'. kuna HAS single-site callee-body arity recovery, shipped and default-on: p4_calls/kuna_calleearitybody.rs (run is the leading argument-register run the callee proves_input, bounded by a register it proves_dead) widened by kuna_calleearitycut.rs (when the callee decode is CUT at a nested call, the register the run stops at may itself be the bound). Both fire on exactly this shape and both establish the run correctly here: the callee reads RDI at 0x417f84 (`MOV R12,RDI`) so RDI proves_input, and the decode is cut 3 instructions later at `CALL 0x4004c8` (0x417f8d) before RSI is touched, so RSI proves neither. The run {RDI} is contiguous and stops short. The rule then REFUSES on its third condition.

THE LINE: kuna_calleearitycut.rs:125, `if !is_register(&bound.addr) || !bound.caller_quiet { return false }`. caller_quiet (kuna_calleearitybody.rs:178) is 'the Varnode at this trial's slot is neither written nor constant'. The caller writes ESI at 0x400fc9 (`MOVZX ESI,byte ptr [RBP-0x1a]`) inside the unrolled XOR loop, purely as the byte-XOR key for `XOR byte ptr [R8+0x6],SIL` at 0x400fcd. RSI is the boundary register, the caller wrote it, caller_quiet is false, accepts_cut_run returns false, and the whole one-argument list is dropped.

PROVED, NOT INFERRED. I copied the binary and NOP'd the 8 bytes at file offset 0xfc9 (= vaddr 0x400fc9; LOAD maps offset 0 at 0x400000), killing the ESI write and its use and NOTHING else -- the loop still consumes the buffer pointer exactly as before. The same command on the patched copy emits `sub_417f80(v2);`. That single-variable control also kills the hypothesis's first bullet: the loop matters because it writes the BOUNDARY register, not because it consumes the buffer pointer.

WHAT THE BUILDER MUST NOT DO. Simply dropping the caller_quiet test is unsafe and the module header already says why: 'a caller that computes a value into it right before the CALL is passing a further argument whatever the callee decode can see, and truncating there would hide it.' The discriminator this case hands you is that the boundary write's value is CONSUMED BY THE CALLER (SIL is read by the XOR at 0x400fcd) before the call -- argument setup is a value used ONLY by the call, which is Funcdata::onlyOpUse in reverse. A boundary register whose value fails onlyOpUse is scratch and is quiet enough; one that passes it is a real further argument. Anything coarser (e.g. 'writes inside a loop are quiet') will fabricate arguments.

ACCEPTANCE TRAPS. (1) The acceptance is stdout_absent `(?m)^  [A-Za-z_]\w*\(\);` and line 47 is the ONLY match in the current output, so the real fix does flip it -- but so would any change that assigns the call's return value (`v1 = sub_417f80();`) while still losing the argument. Assert the argument, not the absence, when you promote this probe to tests/cli/. (2) The acceptance runs the witness ELF only.

THE DEDUP CAUTION IS CONFIRMED: THIS IS TWO ROOT CAUSES. calleearitybody/cut are eligible ONLY for 'a call that recovered nothing at all' -- a site with a self-consistent partial list is calleearitylive's and is left alone. The witness is a TOTAL loss and goes through the path above; the other two instances (6a095817 memcpy keeps 2 of 3, 6a148dd6 init_state keeps 1 of 4) are PARTIAL and therefore cannot be this rule. Fixing the witness will close the need (the non-witness halves carry no clause of their own) WITHOUT necessarily fixing them; expect them back in a later round and do not read that as a regression.

NOT A REGRESSION OF THE ROUND-4 NEED. The family named there (argument-recovery-options-still) is the same symptom class, but the operative rule here shipped after it, so I left regression_of null.
- round 9 TRIAGE: acceptance rewritten. The refuter's own trap note ("assert the argument, not the absence") applied now rather than at promotion: added stdout_matches `sub_417f80\([^)]` alongside the absence clause, so `v1 = sub_417f80();` -- a change that assigns the return value while still losing the argument -- no longer passes. The proved fixed shape (NOP'd-ESI control) is `sub_417f80(v2);`, which matches. track/touches/scope confirmed unchanged (quality, kuna-decomp, small).

### 2026-09-08T05:51Z — captain, round 9 B_PLAN: acceptance target bound

The acceptance probe carried no `target`, so the suite could not resolve `{{BIN}}` and
reported `ProbeError: {{BIN}} used but the context supplies no bin` — indeterminate, not
failing, and therefore unclosable by B_DONE however good the fix. Bound to the challenge's
own image:

    bin/cr4ckm3_0x.tgz.__x/cr4ckm3_0x.tar.__x/cr4ckm3_0x_/cr4ckm3_0x (807352 B, ELF x86-64 statically linked, stripped)

Re-run on freshly built main (3177f045) it now RUNS and FAILS, on the right clause:
`  sub_417f80();` — the zero-argument call, exactly the filed symptom

Nothing about the probe's command or expectations was changed — only the target it
resolves against. Do not re-derive this.
