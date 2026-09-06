---
need_id: binary-maze-pointer-becomes
title: Binary maze pointer becomes an empty string literal
track: quality
status: open
severity: major
probe_id: p-44a61fdb36be
acceptance_id: a-79ab40a45635
hypothesis_status: overturned
credibility: 0.85
instances: 1
challenges: [60be2ad433c5d410b8842c95]
rounds: [4]
first_seen_round: 4
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

Preserve the address of the 585-byte packed maze.

> **Binary maze pointer becomes an empty string literal** (major, `60be2ad433c5d410b8842c95`)
> Initializes source pointers with an empty string, then reads beyond its terminator and compares against 0x403799. Disassembly loads 0x403550; kuna read confirms nonzero maze bytes following the initial zero row.

## Reproduction

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "target": {
    "binary_rel": "bin/Sabloom Text 6.exe",
    "binary_sha256": "5a03a2b553065aedf58f3512e1a701d2ab5e8e9365ea713bfed98f19836747e3",
    "binary_size": 242176,
    "binary_source": "dataset"
  },
  "timeout_s": 60,
  "cmd": [
    "{{KUNA}}",
    "decompile",
    "{{BIN}}",
    "sub_402020"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "=\\s*\"\"\\s*;"
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
    "binary_rel": "decompiler/crates/kuna-analysis/tests/fixtures/emptystrconst_x86_64",
    "binary_sha256": "f1e7a5da26d86d13eafe9e38d4e10db59599bdf0921edb6bfd5d9bb3872bbec6",
    "binary_size": 14512,
    "binary_source": "in-repo",
    "in_repo_path": "decompiler/crates/kuna-analysis/tests/fixtures/emptystrconst_x86_64",
    "selector": "0x401136",
    "selector_kind": "addr"
  },
  "timeout_s": 60,
  "cmd": [
    "{{KUNA}}",
    "decompile",
    "{{BIN}}",
    "0x401136",
    "--addr"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "strlen\\(\\(char \\*\\)0x402020\\)",
      "strlen\\(\"\"\\)"
    ],
    "stdout_absent": [
      "strlen\\(\"\"\\)[^\\n]*\\n[^\\n]*strlen\\(\"\"\\)"
    ]
  },
  "notes": "Vendored reduction of crackmes.one 60be2ad433c5d410b8842c95 (Sabloom Text 6.exe, sub_402020), where both pointers into the 585-byte maze at 0x403550 printed = \"\". Here maze@0x402020 opens with a zero row then blob bytes and must keep its address; merged@0x402060 is a real \"\" inside string data and must keep its quotes. The absent clause is the pre-change output: both on consecutive lines."
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- String rendering considers the first NUL without accounting for subsequent indexed reads.

## Refutation

_not yet refuted_

## Reference

- `ida-decompile load './target/Sabloom Text 6.exe'` — Exit 1: Failed to open database. No reference decompilation obtained.

## Instances

- `60be2ad433c5d410b8842c95` (round 4, tester t-r4-60be2ad4)

## Decision log

- filed by cluster.py from 1 observation(s)
- split out of the round-4 `16-byte-vm-state` mega-bucket by the captain at T_DEDUP: cluster.py's key is `kind|subcommand|clause-shape`, so nine unrelated wrong-output `decompile` defects collapsed into one need whose probe covered only the first. Each carries its own probe and acceptance from its own observation.
- round 4 REFUTER: hypothesis **overturned** (was inconclusive). REFUTED IN-TICK BY THE CAPTAIN (no refuter role exists in the launcher), on the 60be2ad4 witness, sha 6188e204, arena ".kuna-repipe/arena/4/60be2ad433c5d410b8842c95/target/Sabloom Text 6.exe".

SYMPTOM REPRODUCED AND THE DATA IS REAL. `kuna decompile "<Sabloom Text 6.exe>" sub_402020` emits `v16 = ""; v21 = "";` where the disassembly loads 0x403550, and the very next loop walks that pointer (`v16 = &v16[1]`) until `(int)v16 < 0x403799` -- 585 bytes, exactly the maze. `kuna read <bin> 0x403550 --addr --bytes 64` confirms nine 00 bytes and then `77 df 77 ff fd ff 7f ...`, so the tester's "nonzero maze bytes follow the initial zero row" is verbatim true. The address 0x403550 appears NOWHERE in the emitted C (grep: 0 hits), while 0x403511 and 0x403799 both survive as constants -- so this is the loss of one specific pointer, not of address rendering generally.

THE ONE-BIT CONTROL DISPROVES THE FILED CAUSE AND, MORE IMPORTANTLY, BREAKS THE ACCEPTANCE PROBE. .rdata is VMA 0x403000 / file 0x2400, so VA 0x403550 is file offset 0x2950. Copy the binary, poke ONE byte there from 00 to 0x41, change nothing else, re-run the same command:

    v16 = "A";
    v21 = "A";

The pointer is STILL replaced by a string literal; it is merely non-empty now. So "string rendering considers the first NUL" explains only why the literal is EMPTY -- it does not explain the defect the tester actually cares about, which is that a 585-byte binary blob's address is materialized as a string literal at all.

THIS IS THE IMPORTANT PART FOR WHOEVER BUILDS IT: THE ACCEPTANCE PROBE IS GAMEABLE. Its acceptance is `stdout_absent: = "";`. A change built exactly on the filed hypothesis -- "don't stop the literal at the first NUL" -- turns `v16 = ""` into `v16 = "\0\0\0\0\0\0\0\0\0w\xdfw..."` or into `"A"`, and the probe goes GREEN with the output still wrong and the address still gone. T_TRIAGE should tighten the acceptance to require the address to survive (`stdout_matches: 0x403550`), not merely to forbid an empty literal. As filed, this need can be closed without fixing anything.

WHAT THE EVIDENCE POINTS AT INSTEAD. v16 is declared `char *v16` and its FIRST use in the function is `v16 = a1;` followed by a genuine strlen loop over the `char *a1` parameter (lines 39-43 of the output); only afterwards is the same variable reassigned to the maze constant. The blob pointer and a real C string are sharing one variable, so the constant inherits `char *` and PrintC then does the thing it is supposed to do for a char* constant pointing at readable initialized data. The suspicious step is therefore the merge/typing that put a read-only-blob pointer into a char* live range, not the string printer's NUL rule -- and a builder should confirm that ordering before touching p9_emit at all. Note the second variable, `char *v21; // stack - 0x20`, takes its value from v16, so it is downstream of the same decision, not a second instance.

VERDICT overturned: the symptom stands and is worth fixing, the named cause is refuted by a one-byte control that leaves the defect intact, and -- the reason this refutation paid for itself -- the acceptance as written is satisfied by a cosmetic change that does not fix the bug.

- round 5 BUILDER: acceptance RETARGETED to the vendored fixture `decompiler/crates/kuna-analysis/tests/fixtures/emptystrconst_x86_64` so it promotes into `tests/cli/` (CI has no dataset), and its `expect` was re-cut to assert the discrimination rather than the absence of an empty literal: the blob pointer must keep `0x402020` AND the genuine merged `""` beside it must keep its quotes. `acceptance_id` relabelled `a-dec6e1252b85` -> `a-79ab40a45635`. The dataset witness still reproduces and still passes on the shipped build: `kuna decompile "<Sabloom Text 6.exe>" sub_402020` now emits `v16 = (char *)0x403550; v21 = (char *)0x403550;`.
- round 5 BUILDER: the captain's refutation is confirmed on the second half and extended. The naive rule the captain warned about -- suppress any zero-character literal -- is not merely gameable, it is WRONG: a whole-corpus sweep found `setlocale(6,"")` and five more genuine empty literals in the vendored coreutils fixtures alone, every one of them a linker-merged `""` stored as the tail NUL of another literal. The shipped rule therefore reads the sixteen bytes at the address and declines only where they do not read as string data. The typing story stands as filed and is NOT repaired here: the constant is `char *` because it shares a merged live range with a real `char *`, and the printer is doing the right thing for a `char *` constant; what changes is the printer's evidence test, which accepted a location on the strength of zero validated characters.
