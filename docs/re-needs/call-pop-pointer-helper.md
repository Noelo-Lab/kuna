---
need_id: call-pop-pointer-helper
title: CALL/POP pointer helper decodes embedded DLL names as instructions
track: quality
status: closed
severity: major
probe_id: p-b49452c71a5a
acceptance_id: a-ed5e35be957a
hypothesis_status: upheld
credibility: 0.85
instances: 1
challenges: [5ab77f5433c5d40ad448c1c1]
rounds: [12]
first_seen_round: 12
attempts: 0
covered_by_option: null
touches: [decompiler/crates/kuna-decomp]
scope: small
regression_of: null
pr: "568"
closed_in_round: 12
closing_pr: "568"
reject_reason: null
---

## Symptom

Recover the string pointer returned by the helper at 0x4f7036.

> **CALL/POP pointer helper decodes embedded DLL names as instructions** (major, `5ab77f5433c5d40ad448c1c1`)
> The CALL targets POP EAX; INC EAX; RET. Default C instead decodes the embedded string and emits port-input operations and arbitrary memory writes. Enabling calltrampoline, calleearity and varargstackargs did not repair it. The existing flow override repairs it.

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
    "0x4f7036",
    "--addr",
    "--define-function",
    "0x4f7036"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "\\bin\\("
    ]
  },
  "target": {
    "binary_rel": "bin/CryptoME.zip.__x/CryptoME.exe",
    "binary_sha256": "bcfe3d24b0b7453c65476eb11778224b69f47efb7555db4d2240d32b966cf8bd",
    "binary_size": 139776,
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
    "dllname"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_absent": [
      "\\bin\\("
    ],
    "stdout_matches": [
      "return\\s+\"kernel32\\.dll\"\\s*;"
    ]
  },
  "target": {
    "binary_rel": "callpopret_i386",
    "binary_sha256": "f7e5bd8677a8bf0591999f9aa44da8bf6a2cc575f97f8b91e9bd13f3311b6196",
    "binary_size": 4896,
    "binary_source": "in-repo",
    "in_repo_path": "decompiler/crates/kuna-analysis/tests/fixtures/callpopret_i386"
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

_none offered_

## Refutation

_not yet refuted_

## Reference

- `ida-decompile load target/CryptoME.zip.__x/CryptoME.exe` — Reference unavailable: server exited with status 1 before registering. No reference pseudocode was obtained.

## Instances

- `5ab77f5433c5d40ad448c1c1` (round 12, tester t-r12-5ab77f54)

## Decision log

- filed by cluster.py from 1 observation(s)
- split by captain at T_DEDUP from round 12's `wrong-output|decompile|exit_code,stdout_absent` bucket: cluster.py's signature (kind|subcommand|clause-shape) collapsed unrelated defects, so the crop was hand-partitioned one observation per need and filed via `--from-file`. See `.kuna-repipe/rounds/12/dedup/MARKER.json`; do NOT run `cluster --round 12` bare.
- sibling of `push-immediate-ret-transfer` — both are CALL/RET-used-as-a-data-or-jump idiom on the same packer stub, but they are two different idioms (get-pc inline data vs push-imm/ret transfer) with two different fixes. Kept separate (Jaccard 0.0).
- round 12 REFUTER: hypothesis **upheld** (was inconclusive). REFUTED IN-TICK (captain, round 12). EVERY CLAIM THE NEED MAKES IS TRUE, INCLUDING THE THREE NEGATIVE ONES; nothing to overturn. What this adds is the byte-level witness, the exact workaround directive (the need's wording is one word off), the shipped sibling option that ALMOST covers it, and an acceptance that a correct-looking fix passes while still losing what the tester asked for.

THE IDIOM, FROM THE BYTES. 0x4f7036: `E8 52 00 00 00` = CALL 0x4f708d; `90` NOP; then ASCII 'kernel32.dll\0GetProcAddress\0LoadLibraryA\0VirtualAlloc\0VirtualFree\0VirtualProtect\0'. At 0x4f708d: `58 40 C3` = POP EAX; INC EAX; RET. The callee eats the pushed return address (0x4f703b, the NOP), increments past it to 0x4f703c = 'k', and RETs to its caller's caller. So 0x4f703b IS UNREACHABLE and the truth is `char *sub_4f7036(void) { return "kernel32.dll"; }`. CALL is a fall-through op, so kuna decodes the DLL names as instructions -- that is where `in(v13)` (port input) and the arbitrary memory writes come from.

THE THREE NEGATIVES ARE ALL REAL. calltrampoline is DEFAULT-ON and vacuous here in both directions (off emits the same three `in(` sites as the default). Its own summary explains why, and this is the useful part: it flows a call through 'a callee that throws the pushed return address away AND JUMPS BACK INTO THE INSTRUCTION STREAM' -- `lea esp,[esp+4]; <one insn>; jmp <back into the caller>`. This callee throws the return address away and RETs to the grandparent instead. SAME FAMILY, SECOND FLAVOUR: the JMP flavour shipped, the RET flavour did not. That is the coherent line for a builder, and it is why enabling calltrampoline could never have helped.

WORKAROUND -- THE NEED'S 'existing flow override' IS `callreturn`, NOT `return`. `--assert 'flow 0x4f7036 return'` changes NOTHING (byte-identical garbage). `--assert 'flow 0x4f7036 callreturn'` fully repairs the decode: `void sub_4f7036(void) { sub_4f708d(); }`. Anyone replaying this need with the obvious directive will conclude the workaround is dead; it is not.

TWO THINGS THAT WILL COST A BUILDER A DAY IF NOT SAID HERE.

(1) SECTION FLAGS ARE NOT THE DISCRIMINATOR. `kuna disassemble` at 0x4f7036 already prints '0x4f7036 is in a non-executable data section, so these are bytes, not instructions'. That is TRUE OF THE CALL ITSELF as much as of the string -- the whole packer stub lives in one non-executable-flagged section -- so any fix keyed on executability refuses the real code along with the data. The discriminator has to be the callee's stack effect, not the section bit.

(2) THE ACCEPTANCE IS SATISFIED BY A FIX THAT STILL LOSES THE ANSWER. It demands only stdout_absent `\bin\(`, and the callreturn output above -- `void sub_4f7036(void)` -- passes it while returning nothing. The tester's actual task line was 'Recover the string pointer returned by the helper at 0x4f7036'. A complete fix models EAX = 0x4f703c and emits a pointer to 'kernel32.dll'; a fix that merely stops the fall-through decode passes the probe and does not answer the question. Aim past the acceptance.

RISK OF THE INDICATED FIX: low. Treating a call whose callee provably discards the pushed return address as not-falling-through is the true semantics, not a heuristic, and the detection is a bounded decode of the callee (the same shape calleepreserves and calleeretpreserves already use). The failure mode to guard is the opposite one -- a callee that pops and then jumps back, which is calltrampoline's case and must keep its existing handling.
- round 12 captain: acceptance_id corrected a-e02886c9bd81 -> a-27130cef0fa8. The id is derived from cmd+expect; an earlier tightening rewrote the assertion without recomputing it, so the front-matter label no longer named the assertion that runs. Body unchanged -- this is a relabel only. It matters at promotion (the opportunity selector is the acceptance_id) and at dedup (cluster.py trusts a stored id over recomputing, so a re-filing of the same probe would have opened a duplicate need instead of adding an instance).
- round 12 builder: acceptance_id changed a-27130cef0fa8 -> a-ed5e35be957a when the dataset-only CryptoME.exe probe was reduced to the vendored `callpopret_i386` fixture for permanent CI promotion. The assertion is stronger at the task boundary: it requires `return "kernel32.dll";`, not merely the absence of the decoded `in(...)` operations. The original witness remains the stage-independent reproduction and still emits `return s_4f703c;` under the fix.
