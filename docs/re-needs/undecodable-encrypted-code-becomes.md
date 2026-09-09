---
need_id: undecodable-encrypted-code-becomes
title: Undecodable encrypted code becomes an ordinary return without a warning
track: quality
status: open
severity: blocker
probe_id: p-168146b300fb
acceptance_id: a-224d0ec3cc25
hypothesis_status: upheld
credibility: 0.85
instances: 1
challenges: [5ab77f5c33c5d40ad448c65c]
rounds: [5]
first_seen_round: 5
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

Retain the decrypt loop and visibly mark incomplete control flow instead of inventing a normal return.

> **Undecodable encrypted code becomes an ordinary return without a warning** (blocker, `5ab77f5c33c5d40ad448c65c`)
> Exits 0 with case 0x40190e: return; and no warning. Kuna disassembly reports .byte 0xc6 at 0x401913; --raw places return(#0x1) at that address. JSON reports error: null. Enabling switchselector, calleearity and varargstackargs changed nothing. Also hit already-filed block-processing-panics-out at 0x401d18; not re-filed.

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
    "0x401746",
    "--addr"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "case 0x[0-9a-f]+:\\s*return;"
    ],
    "stdout_absent": [
      "WARNING"
    ]
  },
  "target": {
    "binary_rel": "bin/Defender.zip.__x/Defender.exe",
    "binary_sha256": "143b5c88d03f0a0a210ffbae1932d838bf492f1a744c9250dc082461f630c590",
    "binary_size": 15360,
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
    "0x401746",
    "--addr"
  ],
  "expect": {
    "stdout_matches": [
      "while\\s*\\("
    ],
    "stdout_absent": [
      "case 0x[0-9a-f]+:\\s*return;"
    ]
  },
  "target": {
    "binary_rel": "bin/Defender.zip.__x/Defender.exe",
    "binary_sha256": "143b5c88d03f0a0a210ffbae1932d838bf492f1a744c9250dc082461f630c590",
    "binary_size": 15360,
    "binary_source": "dataset"
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- Invalid-instruction termination may become a synthetic RETURN without preserving an incompleteness diagnostic.

## Refutation

_not yet refuted_

## Reference

- `ida-decompile decompile 0x401746 --binary ./target/Defender.zip.__x/Defender.exe --json` — No reference body obtained. Decompile required a running server; the subsequent load exited 1 before server registration. This infrastructure failure establishes no comparison of decompiler capability.

## Instances

- `5ab77f5c33c5d40ad448c65c` (round 5, tester t-r5-5ab77f5c)

## Decision log

- filed by cluster.py from 1 observation(s)
- round 5 REFUTER: hypothesis **upheld** (was inconclusive). REFUTED IN-TICK (captain, round 5). VERDICT: UPHELD -- the invalid instruction really does become a silent synthetic return -- BUT THE REFUTATION FOUND THE FIX IS CHEAPER THAN FILED, AND THAT A COMMENT-ONLY FIX DOES NOT FLIP THE ACCEPTANCE. Read both halves.

WHAT THE FUNCTION ACTUALLY IS (the tester did not say, and it changes how you read the output). sub_401746 is a SELF-DECRYPTING STUB: it stores 0x40190e to dat_4018ff, sets v2 = 0x401906, and XORs 0x30a bytes in place (`*v2 = *v2 ^ dat_406008; *v2 = *v2 ^ v4; v2 = &v2[1]; v3 -= 4`) before transferring control. The bytes at 0x40190e are CIPHERTEXT at load time, so nothing kuna decodes there is the code that runs. It reaches them via `PUSH 0x40190e / POP EBX / JMP EBX`, which kuna resolves correctly to the single constant target -- hence the degenerate `switch(0x40190e) { case 0x40190e: return; }`. The switch is not the defect; the `return;` is.

ISOLATED REPRO WITHOUT THE SWITCH (2 commands, both on the arena binary):
  kuna decompile Defender.exe 0x40190e --addr --define-function 0x40190e-0x401920
    -> void sub_40190e(void) { return; }          <- NO warning, no comment, error null
  kuna decompile Defender.exe 0x40190e --addr --define-function 0x40190e-0x401913
    -> void sub_40190e(void) // warn: Function flows out of bounds
       { return; // warn: Function flow out of bounds: r0x00401912 flows to r0x00401913 }
Same address, same bare `return;`, and the ONLY difference is whether flow ends at the range boundary or at the undecodable byte.

THE FINDING, AND IT IS THE USEFUL PART: THE DIAGNOSTIC CHANNEL ALREADY EXISTS AND IS SIMPLY NOT WIRED TO THIS TERMINATOR. kuna already carries a per-function `// warn:` on the signature and a per-statement `// warn:` on the synthesized return for out-of-bounds flow. The undecodable-instruction path produces the identical synthesized return and attaches nothing. So the need is NOT "invent an incompleteness diagnostic" (which is how the filed hypothesis reads and how a builder would scope it) -- it is "route the bad-instruction terminator through the warn channel that out-of-bounds flow already uses". Much smaller.

CONTEXT FOR THE DECODE: `kuna disassemble --addr 0x40190e --count 6` gives SCASB / CLI / SBB EBP,[EDI+0x69] / .byte 0xc6 at 0x401913. Three instructions DO decode; all three are dead-coded away (their flag/EDI/EBP results are never read), which is why the body renders empty rather than showing them. The terminator is the 0xc6 at 0x401913. Note 0x40190e is also mid-instruction relative to the linear decode (0x401909 OR EAX,0xae2a684f spans through 0x40190e) -- overlapping code, expected in a packer.

THE TRAP THAT WILL COST THE BUILDER THE ROUND: the acceptance is stdout_matches `while\s*\(` AND stdout_absent `case 0x[0-9a-f]+:\s*return;`. The `while` half ALREADY PASSES today (the decrypt loop emits `while (4 <= v3)`). So adding a `// warn:` comment to the existing `return;` DOES NOT FLIP ACCEPTANCE -- the regex matches `case 0x40190e:` followed by whitespace and `return;` regardless of a trailing comment on the same line only if the comment follows `return;`... it does, so the literal `return;` survives and the probe still matches. To close this need the synthesized RETURN itself must go: the block must terminate as an explicit incomplete/halt (a warning statement with no `return;`), not as a return decorated with a comment. Decide that first, then build.

WOULD A FIX PRODUCE WRONG OUTPUT? No -- it is strictly more truthful. Today kuna asserts a function returns normally when it has actually run off into bytes it could not decode, with error:null in --json, which is the worst possible answer for an RE agent (it reads as "analysis complete, nothing here"). Any replacement that marks the flow incomplete is a strict improvement. The one thing to guard is not regressing the very common benign case of a function whose last decoded block legitimately falls through.
round 5 TRIAGE (captain): CONFIRMED AS FILED -- quality / kuna-decomp / small / blocker. The refuter established the fix is strictly more truthful (kuna currently asserts a normal return where it ran into undecodable bytes, with error:null in --json) and pinned the acceptance trap: a `// warn:` comment on the existing `return;` DOES NOT flip the probe -- the synthesized RETURN itself must go. Guard the benign fall-through case.
- round 10 CAPTAIN (18:45Z tick, B_DRAIN off-critical-path repair): REPRODUCTION ARM TARGET BOUND. This need's `## Acceptance` arm was already bound but its `## Reproduction` arm carried no `target` block, so the arm that is supposed to assert the bug EXISTS TODAY could not run (`ProbeError: {{BIN}} used but the context supplies no bin`). That never blocked B_DONE -- closure is gated on the ACCEPTANCE arm alone, measured -- but it blinded the refuter and the builder's reproduction step. Repaired by COPYING this need's own acceptance target verbatim, which is a copy and not a guess: the need has exactly one `challenges:` entry, so both arms necessarily address the same binary, and verify.resolve_binary treats a binary_sha256 mismatch as a hard stop. Measured after the write: the reproduction arm runs (unrunnable=false) and PASSES -- a passing reproduction arm means the defect still reproduces on main at b23e01ec, so this doubles as a re-confirmation that the symptom is live. Probe ids did NOT move (probe_id_of is keyed on cmd+expect only, needs.py:269; all 32 arms across the 16 repaired needs re-hashed byte-identically). PROMOTION CAVEAT unchanged: binary_source is `dataset` and `verify --promote` refuses that verbatim -- vendor an in-repo fixture in the SAME PR or B_DONE closes the need but cannot promote its probe.
