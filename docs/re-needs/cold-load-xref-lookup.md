---
need_id: cold-load-xref-lookup
title: Cold-load xref lookup takes about four seconds on a 466 KB ELF
track: perf
status: open
severity: minor
probe_id: p-6d1e0b3b78b9
acceptance_id: a-6763a70e69bc
hypothesis_status: overturned
credibility: 0.85
instances: 1
challenges: [5bd1d1bb33c5d4110a29b31e]
rounds: [2]
first_seen_round: 2
attempts: 1
covered_by_option: null
touches: [decompiler/crates/kuna-cli, decompiler/crates/kuna-analysis, decompiler/crates/kuna-sleigh]
scope: small
regression_of: null
pr: null
closed_in_round: null
closing_pr: null
reject_reason: null
---

## Symptom

Iteratively query references while triaging the checker.

> **Cold-load xref lookup takes about four seconds on a 466 KB ELF** (minor, `5bd1d1bb33c5d4110a29b31e`)
> The actual `kuna xrefs ./target/Obfuscation1 --to 0x80ba3d2 --json` invocation took 4.1303 seconds according to notes/toolcalls.jsonl. Every independent query reloads analysis state.

## Reproduction

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "timeout_s": 60,
  "cmd": [
    "{{KUNA}}",
    "xrefs",
    "{{BIN}}",
    "--to",
    "0x80ba3d2",
    "--json"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_is_json": true,
    "wall_ms": {
      "stat": "median",
      "gt": 2000
    }
  },
  "target": {
    "binary_rel": "bin/Obfuscation1",
    "binary_sha256": "5a6b66da130bc3465d8d55062500940e974d28c0d17710b2eb51d2c983e7f082",
    "binary_size": 472476,
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
    "xrefs",
    "{{BIN}}",
    "--to",
    "0x80ba3d2",
    "--json"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "wall_ms": {
      "stat": "median",
      "lt": 1000
    }
  },
  "target": {
    "binary_rel": "bin/Obfuscation1",
    "binary_sha256": "5a6b66da130bc3465d8d55062500940e974d28c0d17710b2eb51d2c983e7f082",
    "binary_size": 472476,
    "binary_source": "dataset"
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- Each CLI invocation performs a full cold load and analysis rather than reusing a project or resident analysis session.

## Refutation

_not yet refuted_

## Reference

- `ida-decompile load ./target/Obfuscation1 --backend ida --json` — The reference interface is designed around a persistent server/database, although its server could not start in this arena because it attempted to create a config lock on a read-only path.

## Instances

- `5bd1d1bb33c5d4110a29b31e` (round 2, tester t-r2-5bd1d1bb)

## Decision log

- filed by cluster.py from 1 observation(s)
- round 2 T_TRIAGE (captain): touches [] -> kuna-cli; track perf, scope small, severity minor CONFIRMED. The friction is per-query cold load (4.1s x N queries), so the cheapest real fix is a CLI batch mode (several --to in one invocation) rather than making the load faster -- which is why the edit surface is kuna-cli though the track is perf. Neither track holds counter leases, so the label costs nothing either way.
- round 2 wave 9 ATTEMPT 1 (builder `b-r2-cold-load-xref-l`, ended `failed` by its own honest report,
  rc=0, $28.24, 197 turns) -- **the work exists and is not lost**: local branch
  `feat/re-cold-load-xref-lookup`, 2 commits (`bcbad0a9` one-decode xrefs, `873d9e8c` name the
  reference-walk-discovered functions), **unpushed**, plus
  `docs/features/cold-load-xref-lookup/record.json` with the measurements. A re-dispatch must
  `RESUME_BRANCH=feat/re-cold-load-xref-lookup`, never start over. DO NOT `git branch -D` it.
- **hypothesis overturned by measurement, symptom intact.** The filed cause ("no persistent session")
  named the workflow cost, not the per-query one, so the fix it implies (a batch/session mode) would
  have left the real defect standing: one cold `kuna xrefs` decoded the program **three times**
  (154,608 instructions each) -- analysis-tier Listing walk 1.08 s + `operand_refs` linear decode
  0.58 s + `xrefs::build`'s own recursive descent 1.26 s -- because `--mode auto` promotes a sub-500 KiB
  binary to `aggressive`, a preset for the quality of emitted *C*, on a command that emits none.
  The builder also refuted three sub-hypotheses en route (per-node `ConstructTpl` deep clone ~2%,
  the `WalkCursor` breadcrumb copy, SLEIGH context-DB commits at 9 ms of 3.4 s).
- **why it did not merge, both blockers honest and neither is a bookkeeping artifact.** (1) 3418 ms ->
  1039 ms median (-69.6%), byte-identical output, all four gates green -- but the acceptance clause is
  `wall_ms median < 1000` and it measures 1090-1130 ms, with a 727-1150 ms run-to-run spread on this
  machine, so even a "pass" would be a coin flip. DO NOT RELAX THE PROBE
  ([[kuna-repipe-close-bookkeeping]]). (2) A real recall regression: dropping the Listing tier means
  AIF's gap-walk entries stop being seeds, so `kuna xrefs --from <addr>` on an AIF-only function
  answers 0 references (betaflight Cortex-M `0x806b798` 4->0, `0x801500e` 2->0; KeyVal2.exe 1->0 twice).
  `--mode aggressive` restores it. Attempt 2 needs that trade gated, not accepted.
- round 2 B_PLAN (captain), **dispatch prep for attempt 2 — the branch MOVED, read this before
  you go looking for it.** Attempt 1's two commits are intact at **`wip/re-cold-load-xref-lookup-a1`**
  (`873d9e8c`), not at `feat/re-cold-load-xref-lookup`. The rename is mechanical, not a judgement
  on the work: `spawn_builder` derives the branch as `feat/re-` + slug and `worker.sh` falls back
  to a *silent detached* worktree when that name is already taken, so leaving attempt 1 parked on
  it would have cost attempt 2 its branch. Its worktree was removed (clean, nothing uncommitted);
  the commits are only on that local branch, still unpushed. **Attempt 2 starts by taking that work
  over** — `git merge wip/re-cold-load-xref-lookup-a1` or `git reset --hard` onto it, then rebase on
  main (it is based on `e3db5512`, several merges behind) — it does NOT reimplement 487 lines that
  already measure -69.6% with byte-identical output and four green gates.
- round 2 B_PLAN (captain), **what attempt 2 actually owes**, both blockers named by attempt 1
  itself: (1) the recall regression is the real blocker and it must be *gated*, not accepted —
  dropping the Listing tier loses AIF gap-walk seeds, so `kuna xrefs --from` on an AIF-only
  function answers 0 (betaflight `0x806b798` 4->0, `0x801500e` 2->0; KeyVal2.exe 1->0 twice).
  Correct output at 3.4 s beats wrong output at 1.0 s; a fix that ships the regression will be
  rolled back at B_VERIFY. (2) the acceptance bar `wall_ms median < 1000` is ~90-130 ms below what
  attempt 1 reached and this box's run-to-run spread is 727-1150 ms, so the remaining work is a
  further real cut, not a re-measurement. **The probe is not relaxed** — same standing rule as
  `decompiling-3396-byte-main`. If attempt 2 lands a correct, gated -70% that still misses 1000 ms,
  that is a merged PR and an open need, exactly like #380.
- round 2 B_PLAN (captain): `touches` widened from `[kuna-cli]` to `[kuna-cli, kuna-analysis,
  kuna-sleigh]` to match what attempt 1's diff actually edits (`listing/xrefs.rs`,
  `listing/kuna_picbase.rs`, `sleigh.rs`). No new resource lease results — `perf` is not in
  `TRACK_RESOURCES` and none of those paths map to a counter — so this only makes the siblings'
  contracts file honest.
