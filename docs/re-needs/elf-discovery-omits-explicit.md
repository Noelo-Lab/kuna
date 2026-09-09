---
need_id: elf-discovery-omits-explicit
title: ELF discovery omits the explicit x86-64 CRT main target
track: loader
status: open
severity: major
probe_id: p-2bbc8bf1ae41
acceptance_id: a-18663b946a0d
hypothesis_status: upheld
credibility: 0.85
instances: 1
challenges: [67f9bdc38f555589f3530a85]
rounds: [12]
first_seen_round: 12
attempts: 0
covered_by_option: null
touches: [decompiler/crates/kuna-analysis]
scope: small
regression_of: null
pr: null
closed_in_round: null
closing_pr: null
reject_reason: null
---

## Symptom

Inventory the main function at 0x11a9b.

> **ELF discovery omits the explicit x86-64 CRT main target** (major, `67f9bdc38f555589f3530a85`)
> functions and decompile-all report 34 functions without main. Entry disassembly loads RDI=0x11a9b before libc startup. Address-directed decompilation works, but emits a spurious undefined16 return and four parameters even with entrymainproto on.

## Reproduction

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "timeout_s": 60,
  "cmd": [
    "{{KUNA}}",
    "functions",
    "{{BIN}}",
    "--json"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_absent": [
      "\"address_hex\": \"0x11a9b\""
    ]
  },
  "target": {
    "binary_rel": "bin/WeeperVM--Level_1",
    "binary_sha256": "8ecacb009c64989b3d269193617a742ee039ffad7e2690ed148f11312751ef03",
    "binary_size": 14112,
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
    "functions",
    "{{BIN}}",
    "--json"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "json": [
      {
        "path": "functions[*].address",
        "op": "eq",
        "value": 72347
      }
    ]
  },
  "target": {
    "binary_rel": "bin/WeeperVM--Level_1",
    "binary_sha256": "8ecacb009c64989b3d269193617a742ee039ffad7e2690ed148f11312751ef03",
    "binary_size": 14112,
    "binary_source": "dataset"
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- CRT-target discovery may not recognize this startup sequence or imported call.

## Refutation

_not yet refuted_

## Reference

- `ida-decompile load ./target/WeeperVM--Level_1 --backend ida` — Server 2851be9e60 exited with status 1 before registering. No reference pseudocode obtained.

## Instances

- `67f9bdc38f555589f3530a85` (round 12, tester t-r12-67f9bdc3)

## Decision log

- filed by cluster.py from 1 observation(s)
- split by captain at T_DEDUP from round 12's `silent-failure|functions|exit_code,json` bucket: cluster.py's signature (kind|subcommand|clause-shape) collapsed unrelated defects, so the crop was hand-partitioned one observation per need and filed via `--from-file`. See `.kuna-repipe/rounds/12/dedup/MARKER.json`; do NOT run `cluster --round 12` bare.
- round 12 REFUTER: hypothesis **upheld** (was inconclusive). UPHELD and reduced to one byte pattern. The symptom stands (34 functions, no 0x11a9b) and the filed cause -- CRT-target discovery does not recognise this startup sequence -- is exactly right; what the refutation adds is WHICH oracle and WHY, plus three closed lines.

MECHANISM. Whole-binary discovery is EntryDiscoveryPass's union of five oracles (kuna-analysis/src/analyzers/entry/mod.rs). For this image three of them are silent and the other two cannot reach main:
(1) Oracle 4, the _start->main libc-start idiom, is the one that should fire and it MISSES ON THE OPCODE FORM ALONE. x86_64_main_target (mod.rs:945) scans a 64-byte window from e_entry for literally '48 8d 3d <disp32>' (LEA RDI,[RIP+disp]) and computes main = lea_addr+7+disp. This _start hands main over as an ABSOLUTE immediate instead: e_entry is 0x10a00, and at 0x10a18 the bytes are 48 c7 c7 9b 1a 01 00 = MOV RDI,0x11a9b, immediately followed by ff 15 f3 23 00 00 = CALL qword ptr [0x12e18] (the __libc_start_main GOT call). The offending instruction is at window index 0x18 of 0x40, so the window is NOT the limiter -- the three-byte prefix test is. Same idiom, same register, same call, one encoding the matcher does not know.
(2) Oracle 3 (.eh_frame FDE pcBegin), normally the highest-value oracle on a C binary, contributes NOTHING here: readelf -S shows the image has NO .eh_frame section at all (one RWE LOAD, .text at 0x10a00). So there is no second path to main.
(3) Oracle 5 (gcc prologue byte patterns) cannot match: main's prologue at 0x11a9b is 55 31 c0 53 51 = PUSH RBP; XOR EAX,EAX; PUSH RBX; PUSH RCX -- no 'mov rbp,rsp'.

DEAD LINE CLOSED, and this is the one the last tick asked about: this is NOT a shipped-option-already story. funcstart_patterns is default-OFF, so it was the obvious candidate; 'decompile-all --option funcstart_patterns on' returns the SAME 34 functions and still no 0x11a9b. (kuna functions takes no --option, so ablate through decompile-all.)

THE LOSS IS BIGGER THAN ONE FUNCTION, which matters for ranking. Because main is never a function, nothing decodes its body, so its callees are invisible too: main calls 0x11266 at 0x11aa0, and 'xrefs --to 0x11266' reports count 0. Seeding main is what makes that subtree reachable at all.

TWO CAUTIONS FOR THE BUILDER. First, do not read 'kuna xrefs --to 0x11a9b' as proof kuna already knows about a function there: it answers with target.name sub_11a9b and a to_function block, but that is synthesized from the queried address, not evidence of an inventory entry -- 'functions' is the oracle, and it does not list it. Second, the inventory's last row is sub_112e9 with size 2230, i.e. 0x112e9..0x11b9f, which SWALLOWS 0x11a9b; expect the bounding of that run-on function to move once main is seeded, and check it rather than assuming the fix is purely additive.

Acceptance is sound: a positive json clause on functions[*].address == 72347 (0x11a9b), which cannot be satisfied by losing anything.
- round 12 CAPTAIN (B_FANOUT, pre-dispatch brief): PROMOTION CAVEAT -- this need's probe target is `binary_source: dataset`, and `verify --promote` refuses anything that is not `in-repo` (verify.py:750, "CI has no dataset"); `--force` does not override it. B_DONE can therefore CLOSE this need on the measured acceptance flip but cannot promote its probe into `tests/cli/`, so the fix ships with no regression guard unless the builder vendors an in-repo fixture (or reuses a vendored twin) in the SAME PR and points a promotable copy of the probe at it. Same refusal already blocked virtualalloc-protection-argument-disappears and arm-inventory-invents-function.
- BUILDER RESULT (worker b-r12-elf-discovery-om, no new option): hypothesis UPHELD and the refuter's mechanism is exact to the byte. `x86_64_main_target` matched `48 8d 3d` (`lea rdi,[rip+disp32]`) alone; this `_start` uses `48 c7 c7 9b 1a 01 00` = `mov rdi,0x11a9b` at 0x10a18, followed immediately by the `ff 15` `__libc_start_main` GOT call. PC-relative vs absolute is a LINK MODEL, not a different idiom, so the three non-PIE encodings (`bf imm32`, `48 c7 c7 imm32`, `48 bf imm64`) are now read as a fallback consulted only when the `lea` scan misses -- 1,788 of 1,919 swept x86-64 ELF objects carry the `lea` and are unchanged by construction. Guards, because a bare immediate is one opcode byte: a `call` (`e8` / `ff /2`) within 16 bytes, the immediate inside an executable section, and unique in the window (else a clean miss). Witness 34 -> 41 functions with 0x11a9b present; the refuter's second caution HELD -- `sub_112e9` 2230 -> 1855 and `_INIT_0` 716 -> 6, both corrections (0x10af0 really is a 6-byte `endbr64; jmp` thunk), 0 entries lost anywhere. `xrefs --to 0x11266` count 0 -> 1. Sweeps: 2 of 119 runnable fallback-reachable system binaries changed (busybox 2116 -> 2164) and 2 of 43 RE-dataset ELFs, every new start verified as a real prologue. NOT closed here: the symptom's second clause (`undefined16` + four spurious params) is a separate prototype-surface defect and `entrymainproto` is PE-only BY DESIGN, so it can never apply to this ELF. Promotion twin is the already-vendored `funcstart_patterns_x86_64`, which carried the same defect (8 -> 9 functions); no new binary added.
