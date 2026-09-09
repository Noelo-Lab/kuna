---
need_id: linear-disassembly-silently-skips
title: Linear disassembly silently skips branch targets inside decoy instructions
track: tooling
status: open
severity: major
probe_id: p-40e05ae9eb25
acceptance_id: a-c5bcbf266674
hypothesis_status: upheld
credibility: 0.85
instances: 1
challenges: [5ab77f6633c5d40ad448cbec]
rounds: [12]
first_seen_round: 12
attempts: 0
covered_by_option: null
touches: [decompiler/crates/kuna-cli]
scope: small
regression_of: null
pr: null
closed_in_round: null
closing_pr: null
reject_reason: null
---

## Symptom

A diagnostic when linear decoding skips an in-range direct branch target, with an option to follow reachable instructions.

> **Linear disassembly silently skips branch targets inside decoy instructions** (major, `5ab77f6633c5d40ad448cbec`)
> The listing includes JMP 0x43d092 but decodes RET at 0x43d091 across that target, with empty notes. Disassembling 0x43d092 separately correctly yields LODSB. Also hit already-filed block-processing-panics-out; an explicit function boundary bypassed that panic.

## Reproduction

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "timeout_s": 60,
  "cmd": [
    "{{KUNA}}",
    "disassemble",
    "{{BIN}}",
    "0x43d060",
    "--addr",
    "--count",
    "70",
    "--json"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_is_json": true,
    "stdout_matches": [
      "JMP 0x43d092"
    ],
    "stdout_absent": [
      "\"address_hex\": \"0x43d092\""
    ],
    "json": [
      {
        "path": "notes",
        "op": "len_eq",
        "value": 0
      }
    ]
  },
  "target": {
    "binary_rel": "bin/LoaderCrackMev2.0.zip.__x/corrupt.exe",
    "binary_sha256": "bb6cd6045dff9e299f60299e8dae22abb57eafd125221bb72f3d671f138f00fc",
    "binary_size": 220798,
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
    "disassemble",
    "{{BIN}}",
    "0x43d060",
    "--addr",
    "--count",
    "70",
    "--json"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_is_json": true,
    "json": [
      {
        "path": "notes",
        "op": "matches",
        "value": "0x43d092"
      }
    ]
  },
  "target": {
    "binary_rel": "bin/LoaderCrackMev2.0.zip.__x/corrupt.exe",
    "binary_sha256": "bb6cd6045dff9e299f60299e8dae22abb57eafd125221bb72f3d671f138f00fc",
    "binary_size": 220798,
    "binary_source": "dataset"
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- The linear sweep does not diagnose instructions covering direct branch targets.

## Refutation

_not yet refuted_

## Reference

- `ida-decompile load target/LoaderCrackMev2.0.zip.__x/corrupt.exe --backend ida --project-dir notes/ida-corrupt --json` — Reference unavailable: server exited with status 1 before registering. No comparison output was obtained.

## Instances

- `5ab77f6633c5d40ad448cbec` (round 12, tester t-r12-5ab77f66)

## Decision log

- filed by cluster.py from 1 observation(s)
- split by captain at T_DEDUP from round 12's `bad-ux|?|exit_code,json,stdout_is_json` bucket: cluster.py's signature (kind|subcommand|clause-shape) collapsed unrelated defects, so the crop was hand-partitioned one observation per need and filed via `--from-file`. See `.kuna-repipe/rounds/12/dedup/MARKER.json`; do NOT run `cluster --round 12` bare.
- round 12 REFUTER: hypothesis **upheld** (was inconclusive). round 12 REFUTER (captain, in-tick): UPHELD on aa18e56d. SYMPTOM EXACT: the probe command reproduces byte-for-byte -- notes [] , 70 instructions, JMP 0x43d092 present at 0x43d08c and 0x43d08f, no row with address_hex 0x43d092. HYPOTHESIS CONFIRMED AT THE CODE LEVEL, NOT JUST INFERRED: kuna disassemble has exactly THREE note producers and none is branch-aware -- choose_view's data-section note (disassemble.rs:532-540), pool_note for a folded literal pool (:1205), image_bound_note for a listing cut short by the segment end (:962). All three are inapplicable to this region (executable section yC at 0x43d000, nothing folded, the range ends far inside the segment), so notes is structurally empty here and no code path could ever say the sweep walked over a branch target. THE FILED CAUSE IS RIGHT AND THE FIX ORACLE IS ONE PREDICATE: an instruction whose [addr, addr+size) strictly contains an in-range direct branch target that is not itself a decoded start. FOUR CORRECTIONS FOR WHOEVER BUILDS IT. (1) IT IS NOT ONE SITE, IT IS THREE TARGETS FROM FOUR BRANCHES -- I measured the whole listing: of 6 in-range direct branches, only 2 land on a decoded start; 0x43d092 (from both 0x43d08c and 0x43d08f, covered by the RET 0xebac at 0x43d091), 0x43d0ae (from 0x43d0ab, covered by 0x43d0ad), 0x43d0c1 (from 0x43d0be, covered by 0x43d0c0). All three are the identical EB 01 / EB 04 jmp-over-one-junk-byte idiom. So the diagnostic is NOT spam even on deliberately obfuscated code -- 3 notes in 70 instructions -- and a builder should not add a suppression heuristic it does not need. (2) THE REAL HARM IS FABRICATED INSTRUCTIONS, NOT A MISSING ROW, AND IT IS WORSE THAN THE TESTER SAW. File offset 0x3528c holds eb 04 90 eb 01 c2 ac eb 01 e9 90 fe c8 ... ; the decoy byte is the C2 at 0x43d091. Decoding from 0x43d0c1 instead gives XOR AL,0x92 / STOSB / LOOP 0x43d092 -- a back edge to 0x43d092, i.e. the region is a LODSB-arithmetic-STOSB decryption loop. The linear listing replaces that loop tail with CALL -0x1d119d07 and INT3, and at 0x43d0ad it prints JMP 0x4d99972, a target outside the image entirely. So the command invents a call and an out-of-image jump that the bytes do not contain; that is the wrong output to lead the PR with, not the absent address. (3) A ONE-SHOT RESTART AT THE MISSED TARGET IS NOT A FIX -- MEASURED. Disassembling 0x43d092 alone gives LODSB then JMP 0x43d096 then, at 0x43d095, JMP 0x2d0ccf2a -- garbage again, because 0x43d093's JMP skips the E9. Only following that second branch reaches the correct 0x43d096 NOP. Any follow mode must be transitive to a fixpoint over newly discovered targets, not a single re-anchor; the three desyncs each re-sync within a few bytes (0x43d0ae rejoins the linear stream at 0x43d0b3), so a fixpoint terminates cheaply here. (4) THE ACCEPTANCE IS WEAK AND SHOULD BE TIGHTENED BEFORE DISPATCH: it asserts only notes len_gt 0 on this command, so ANY note passes it -- including one about something else entirely, and including the data-view or image-bound notes if a refactor ever makes them fire here. Require the note text to name 0x43d092, and if a follow mode is added keep a clause proving the listing gained an address_hex 0x43d092 row. Note the repro probe pins stdout_absent 0x43d092, so a DEFAULT-ON follow mode retires the repro; that is expected, but it means the repro cannot double as the regression guard. TRACK AND SHAPE CONFIRMED: this is CLI-tier. Nothing in docs/options.md governs walk(); the sweep is a plain kuna-cli walk with no settable behind it, so this wants a CLI flag plus an always-on note, a DIV row, and no phases.toml row or catalog counts. Not a duplicate of its two same-round siblings: function-inventory-omits-two is EntryDiscoveryPass seeding and function-inventory-silently-lists is a zero_discovery_error predicate; this is the third distinct gap, in kuna-cli disassemble, exactly as the previous tick predicted -- the round-12 inventory trio is confirmed to be three separate fixes and must not be ranked as one.
- round 12 captain: acceptance_id corrected a-398e7d7ba3b8 -> a-c5bcbf266674. The id is derived from cmd+expect; an earlier tightening rewrote the assertion without recomputing it, so the front-matter label no longer named the assertion that runs. Body unchanged -- this is a relabel only. It matters at promotion (the opportunity selector is the acceptance_id) and at dedup (cluster.py trusts a stored id over recomputing, so a re-filing of the same probe would have opened a duplicate need instead of adding an instance).
- round 12 CAPTAIN (B_IDLE->B_PLAN tick, 09:4xZ): **PASSED OVER AT B_PLAN — ACCEPTANCE IS UNRUNNABLE, SO A BUILDER COULD NOT CLOSE IT EVEN IF IT FIXED THE BUG.** `verify --acceptance-suite --need linear-disassembly-silently-skips` on main 1c66b462 returns `unrunnable: true`, `error: "check raised: ProbeError: {{BIN}} used but the context supplies no bin"`, clauses `[]`, repeat 0 — the acceptance arm's cmd still carries the `{{BIN}}` placeholder and nothing has ever bound `target` for it. select.py ranked it #3 (13.86, tied with the whole major band) and it was dropped for `neopack-payload-requires-external` at the identical score; that substitution costs nothing in rank and buys a measurable acceptance. NOT a comment on the defect, which the round-12 refuter upheld at the code level. WHAT IT NEEDS BEFORE IT IS DISPATCHABLE: a bound probe `target` (nothing in the pipeline writes one — it is hand-bound every round), pointing at a binary reachable from the repo, with the `{{BIN}}` substitution then resolving. Until that exists this need is invisible to B_DONE: its acceptance can never flip, so a builder that fixed it perfectly would still read as unfinished.
- round 12 B_IDLE (captain): **probe target bound by hand; this need is now dispatchable.** Both
  its probe blocks used `{{BIN}}` with no `target`, so every replay died with "`{{BIN}}` used but the
  context supplies no bin" -- clauses `[]`, repeat 0 -- and two consecutive `B_PLAN` ticks had to
  skip it because an acceptance that cannot run can never flip, however good the fix. The binary is
  identified from the filing tester's own `toolcalls.jsonl`, by an argv that matches the probe cmd
  exactly, not by guessing from the arena: `kuna disassemble target/LoaderCrackMev2.0.zip.__x/corrupt.exe 0x43d060 --addr --count 70 --json`. Bound to
  `bin/LoaderCrackMev2.0.zip.__x/corrupt.exe` (sha256 `bb6cd6045dff9e299f60299e8dae22abb57eafd125221bb72f3d671f138f00fc`, 220798 bytes,
  source `dataset`, resolved as `challenges/5ab77f6633c5d40ad448cbec/<rel>`). `target` is not part of
  `probe_id`/`acceptance_id` (they hash `cmd`+`expect` only), so both ids are unchanged and no
  front-matter drifted. Replayed on unpatched main `17da9472`: **reproduction PASSES** (the defect
  is present) and **acceptance FAILS** on `json[0]` (`notes` never mentions the skipped target `0x43d092`) -- runnable, honestly failing, which is the
  dispatch precondition.
