---
need_id: cold-load-xref-lookup
title: Cold-load xref lookup takes about four seconds on a 466 KB ELF
track: perf
status: closed
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
closed_in_round: 2
closing_pr: "394"
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
  "repeat": 7,
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
- closed: acceptance a-6763a70e69bc now PASSES at 8ff1b4ecb1c7
- round 2 B_DONE (captain): **closed on TWO independent quiet replays, and the margin is 3.3%.**
  #394 merged as `8ff1b4ec`. B_VERIFY measured `wall_ms` median **967 ms** (reps=7) against the
  `< 1000` bar; this tick re-measured it cold at loadavg 1.15 and got **942 ms**
  (samples 923/654/970/942/971/894/972, flaky=False, exit_code 0 on all 7). The `--all` suite run
  earlier the same hour reported 1110 ms and would NOT have closed it -- that reading was taken
  while a builder held an 82%-CPU sweep. **Carry this forward: a future `regressed` flip on this
  need is a stopwatch reading until it has been re-measured with the box quiet at reps>=7.**
  Freshness was proved behaviourally rather than by a catalog row (this PR adds no option): the
  same witness command fell 3855 ms -> 969 ms across `make binaries`, a 4.0x drop.
  The promoted `tests/cli/cold-load-xref-lookup.json` is deliberately NOT this probe -- the
  acceptance targets dataset `bin/Obfuscation1` and CI has no dataset, so the vendored assertion
  is min-of-5 < 1100 ms on the in-repo `mcount_x86_64`. CI is therefore not exposed to this margin.
- round 2 wave 38 B_VERIFY flagged this need `regressed` and wave 39 B_DONE **refuted it — do not
  re-file it, do not roll anything back.** The `--all` acceptance suite ran a-6763a70e69bc at
  **repeat 1** and got 1224 ms against the `< 1000` bar; wave 38 immediately re-measured it 9 times
  on a quiet box (loadavg 2.1-3.4 of 80 cores) and got 657/660/663/663/667/911/964/971/1036 ms,
  **median 667 ms**, 1 of 9 over the bar. That is faster than the two readings that closed it
  (#394 B_VERIFY 967 ms reps=7, B_DONE 942 ms reps=7), and the merge under test (46c373ac, a P2
  jump-table option) cannot plausibly slow a cold `kuna xrefs` load. The need's own standing rule
  applied exactly as written and is worth restating: *a `regressed` flip here is a stopwatch
  reading until it has been re-measured with the box quiet at reps>=7.*
- round 2 B_DONE (captain): **the acceptance probe now carries `repeat: 7`** (was absent, i.e. 1).
  This is a STRENGTHENING of the measurement, not a relaxation of the bar — `expect` is untouched
  (`wall_ms median < 1000`) and `acceptance_id` is unchanged at `a-6763a70e69bc`, because
  `probe_id()` hashes only `cmd` + `expect`. Why it was needed: `verify.py` applies its
  timing/memory floor of 7 reps via `gate_reps()` on the **gate** path only; `acceptance_suite()`
  honours the probe's own `repeat` verbatim, so every B_VERIFY was timing this need ONCE against a
  bar its true median clears by ~33% on a box with a 657-1036 ms spread. That is how wave 38's
  phantom `regressed` happened, and left alone it would have burned a builder on a non-bug.
  **The same one-sample defect affects every timing acceptance in the backlog** (a captain-side
  fix in `verify.py` — defaulting `acceptance_suite` to `gate_reps` — is the general repair and
  belongs to a harness need, not to this record).
- round 2 B_DONE (captain), **the `repeat: 7` edit was replayed and both closures still hold.**
  `verify --acceptance-suite` on the two edited needs at 46c373ac, box quiet (loadavg 1.14):
  a-6763a70e69bc **median 662 ms** (965/662/866/654/660/673/648), flaky False, PASS; the phantom
  1224 ms of wave 38 does not reproduce anywhere in 7 runs. Suite verdict: 2 pass, 0 fail,
  0 regressed.
- round 2 wave 24 B_VERIFY flagged this need `regressed` for the **third** time and wave 25 B_DONE
  refuted it again — **do not roll back, do not re-file.** The `--all` suite at `766ead49` measured
  a-6763a70e69bc median **1088 ms** (7 reps: 1212/1001/956/1242/1315/1088/1055) against the `< 1000`
  bar, every run exit 0 with correct JSON — only the stopwatch clause failed, and the suite ran in
  `make rust-test`'s wake with loadavg still 1.69/4.30/4.19. Re-measured on the settled box at 9
  reps: 956/938/972/738/652/881/660/650/670 → **median 738 ms, max 972 ms, flaky False, PASS,
  transition `unchanged`** (JSON `/tmp/w24-cold-remeasure.json`). All nine samples clear the bar and
  the sample order is the load-decay signature. Secondary causal check: `766ead49` adds `machomain`,
  a P1 pass gated on MH_EXECUTE Mach-O images, and the probe target is an i386 ELF — the pass cannot
  execute on it. The bar was NOT relaxed and the probe was NOT touched. Note the failure mode is now
  **load contention, not sample count**: `repeat: 7` is already in force and did not prevent it, so
  the standing rule stands as written — *a `regressed` flip here is a stopwatch reading until it has
  been re-measured with the box quiet at reps>=7.*
- round 2 wave 32 B_DONE (captain): **fourth phantom `regressed` refuted at 255a96d6 — and the
  quiet-box margin is now the thing to watch.** Wave 31 re-measured on a settled box at 9 reps
  (940/656/956/961/928/654/654/961/951 → median **940.4 ms**, max 961.0, flaky False, PASS,
  transition `unchanged`; JSON `.kuna-repipe/rounds/2/coldload-remeasure-w31.json`). The bar was
  NOT relaxed. The new signal: the QUIET median has drifted 738 ms (wave 24) → 940 ms (wave 31),
  i.e. ~6% headroom under `< 1000`, so this probe will keep flagging and will eventually flag with
  the box quiet. That is either a probe-calibration question for **T_TRIAGE** or a real slow creep
  worth its own perf need — it is decided at triage with a measured `xrefs` cold-load trend, never
  by a B_VERIFY/B_DONE tick moving the bar.
