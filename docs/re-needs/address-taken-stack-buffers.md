---
need_id: address-taken-stack-buffers
title: Address-taken stack buffers lack JSON use locations
track: quality
status: closed
severity: minor
probe_id: p-570c29173fd3
acceptance_id: a-afdbfc2ff960
hypothesis_status: overturned
credibility: 0.7
instances: 1
challenges: [64c8b272b25df8732eebc2a6]
rounds: [12]
first_seen_round: 12
attempts: 1
covered_by_option: null
touches: [decompiler/crates/kuna-decomp]
scope: small
regression_of: null
pr: 619
closed_in_round: 12
closing_pr: "619"
reject_reason: null
---

## Symptom

Locate main's key-buffer uses through variable metadata.

> **Address-taken stack buffers lack JSON use locations** (minor, `64c8b272b25df8732eebc2a6`)
> The buffer at stack offset -88 has empty line_numbers and addresses despite being passed to __strcpy_chk and strcmp. The input buffer similarly lacks its scanf and strcmp uses.

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
    "_main",
    "--json"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_is_json": true,
    "stdout_matches": [
      "\"stack_offset\"\\s*:\\s*-88\\s*,\\s*\"size\"\\s*:\\s*\\d+\\s*,\\s*\"line_numbers\"\\s*:\\s*\\[\\s*\\]"
    ]
  },
  "target": {
    "binary_rel": "bin/main",
    "binary_sha256": "979555f6b20fc5f024358ef26603f4c25e8db326941f86af30726eca9fea453e",
    "binary_size": 50080,
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
    "_main",
    "--json"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_is_json": true,
    "stdout_absent": [
      "\"stack_offset\"\\s*:\\s*-88\\s*,\\s*\"size\"\\s*:\\s*\\d+\\s*,\\s*\"line_numbers\"\\s*:\\s*\\[\\s*\\]"
    ],
    "stdout_matches": [
      "\"stack_offset\"\\s*:\\s*-88\\s*,\\s*\"size\"\\s*:\\s*32\\s*,\\s*\"line_numbers\"\\s*:\\s*\\[\\s*\\d+\\s*,\\s*\\d+",
      "\"stack_offset\"\\s*:\\s*-56\\s*,\\s*\"size\"\\s*:\\s*40\\s*,\\s*\"line_numbers\"\\s*:\\s*\\[\\s*\\d+\\s*,\\s*\\d+"
    ]
  },
  "target": {
    "binary_rel": "bin/main",
    "binary_sha256": "979555f6b20fc5f024358ef26603f4c25e8db326941f86af30726eca9fea453e",
    "binary_size": 50080,
    "binary_source": "dataset"
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

_none offered_

## Refutation

_not yet refuted_

## Reference

_none recorded_

## Instances

- `64c8b272b25df8732eebc2a6` (round 12, tester t-r12-64c8b272)

## Decision log

- filed by cluster.py from 1 observation(s)
- split by captain at T_DEDUP from round 12's `wrong-output|decompile|exit_code,stdout_absent,stdout_is_json` bucket: cluster.py's signature (kind|subcommand|clause-shape) collapsed unrelated defects, so the crop was hand-partitioned one observation per need and filed via `--from-file`. See `.kuna-repipe/rounds/12/dedup/MARKER.json`; do NOT run `cluster --round 12` bare.
- round 12 REFUTER: hypothesis **overturned** (was inconclusive). OVERTURNED ON ITS FRAMING (no hypothesis was ever filed -- the section reads "_none offered_", so what is overturned is the implied cause in the title). The SYMPTOM STANDS exactly as reported: kuna decompile <arena 12/64c8b272 target/main> _main --json gives v1 char[32] at -88 and v2 char[40] at -56 with line_numbers [] and addresses [], while v3 (long, -16) carries line_numbers [7,16] and 3 addresses. "ADDRESS-TAKEN" IS NOT THE GATE, AND I HAVE A CONTROL THAT PROVES IT. Built /tmp/vuse2.c (5 lines: int x=3; scanf("%d",&x); return x+1;, gcc -O0). v1 there is an address-taken SCALAR -- __isoc99_scanf("%d",&v1) is literally the address being taken -- and it comes back FULLY POPULATED, line_numbers [5,7], addresses [4484,4521,4545]. So a stack variable can be address-taken and still get its use locations. THE REAL DISCRIMINATOR IS DIRECT-VALUE USE vs USE-THROUGH-ADDRESS, and the same control proves it from the other side: v1s list is [5,7] and LINE 6 IS MISSING -- line 6 is the scanf line, i.e. precisely the use where the variable appears only through its address. Populated = the variable appears as a direct value varnode (a read or write of the scalar itself). Empty = it only ever appears as the pointer operand of some other op. An ARRAY can never be used any other way, so arrays come out 100% empty, which is what makes the defect LOOK like "address-taken buffers". The title has the correlation and the wrong cause. NOT MACH-O-SPECIFIC -- REPRODUCES ON PLAIN ELF IN 8 LINES OF C, so no builder needs the crackme or a Mach-O toolchain. /tmp/vuse.c: char buf[32] + long sc, strcpy/strlen/strcmp. kuna decompile /tmp/vuse.elf probe --json gives v2 char[40] with line_numbers [] though the body references it on three lines, beside v3 long with [9,11,12]. Sources for both controls are 8 and 5 lines and are reproduced in this note; the .elf files in /tmp will NOT survive a reboot. THE INFORMATION IS RECOVERABLE, THIS IS NOT A "the model does not know" defect: the same JSON already carries line_mappings with 10 entries including lines 8, 10 and 11 -- exactly the lines whose emitted text references v1 and v2 -- so the line-to-address mapping exists and the printer demonstrably knows which token belongs to which symbol. TWO WAYS A BUILDER SHIPS WRONG OUTPUT HERE. (1) DO NOT BLANKET-POPULATE. Four of the seven variables in the target (local_70, local_68, local_5c, local_8) are EMPTY AND CORRECTLY SO -- they never appear in the emitted body at all; same in the control (local_50, local_10, local_8). A fix that fills in every stack variable INVENTS uses and is worse than the bug. (2) THE ACCEPTANCE IS WEAKER THAN IT LOOKS: it is stdout_absent on the regex for offset -88 with an empty list, so it also turns green if v1 is dropped from the variables array entirely, or if field ORDER changes so the pattern stops matching. Neither ships anything. Demand a positive clause -- v1s line_numbers must contain the __strcpy_chk and strcmp lines -- at the next T_TRIAGE. ONE MORE THING THE BUILDER WILL TRIP ON: the variables array is STACK-ONLY. In the ELF control the body declares "unsigned long v1; // rax" and that register variable is ABSENT from variables entirely (the array holds v3, v2, local_50, local_10, local_8). That is out of scope for this need but do not read it as a second bug.
- round 12 captain: acceptance_id corrected a-d436d807c231 -> a-afdbfc2ff960. The id is derived from cmd+expect; an earlier tightening rewrote the assertion without recomputing it, so the front-matter label no longer named the assertion that runs. Body unchanged -- this is a relabel only. It matters at promotion (the opportunity selector is the acceptance_id) and at dedup (cluster.py trusts a stored id over recomputing, so a re-filing of the same probe would have opened a duplicate need instead of adding an instance).
- closed by #619: acceptance a-afdbfc2ff960 now PASSES. Same-named stack fragments contribute use evidence only when their full byte range is contained by the reported aggregate; exact base-address constants also contribute, while partial overlaps, neighbouring slots, and unused locals remain empty.
