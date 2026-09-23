# kuna type-recovery campaign — handoff state (2026-09-24, stopped mid-round-H)

Everything below is measured, not assumed. Paths are on this machine. The repo is
`/home/mahaloz/github/kuna`; **the main checkout is read-only for agents — the user commits there
directly.** All campaign work happens in worktrees under `/home/mahaloz/kwt/<slug>`.

**Read next, in this order:** `/home/mahaloz/kwt/COMMON_BRIEF.md` (the operating rules every agent
gets), `docs/agents.md` in the repo (AGENTS.md/CLAUDE.md are symlinks to it), and
`docs/decbench/typecampaign/results.md` (the campaign's own results page, rounds B–G).

---

## 1. Where the campaign stands

The user asked for an extreme, long-running effort on type recovery, judged by decbench's
`type_match`, with three priorities: (1) primitives right, (2) fewer variables, (3) angr-style
struct recovery. Eight rounds have run (A–H).

**Headline, 444 benchmark slices / 10,748 functions, metric pinned to decbench `625e892`:**

| | perfect functions | mean |
|---|---:|---:|
| campaign baseline | 848 (7.89%) | .2645 |
| round C | 1,349 (12.55%) | .3403 |
| round F | 1,522 (14.16%) | .3553 |
| **round G (current main)** | **1,609 (14.97%)** | **.3688** |

kuna is **1st of five decompilers** and leads at every optimization level. On each rival's own
functions: vs binja 14.76% / .3707 against 12.28% / .3422; vs IDA 15.09% against 8.33%; vs angr
15.23% against 8.27%; vs Ghidra 14.94% against 6.84%. `ptr_char` recovery (38.0%) passed binja
(36.8%) in round G.

Merged in the campaign: roughly 45 PRs, #646 through #717. The per-round detail, every PR with its
measured effect, and the reproduce block are in `docs/decbench/typecampaign/results.md` with raw
tables in `final-c/` … `final-g/`.

**main is at `dbe854ba3`** ("structsynth claims a record's fields that a sibling function proves",
#715), which landed during the stop.

---

## 2. What was in flight when the campaign was stopped

Two workflows were stopped by the user's request: round H (`wf_7d7b113b-58c`) and the cast census
(`wf_6d8f225d-f10`). No agent of mine is running. Uncommitted work in every worktree was committed
and pushed before stopping, and **those WIP commits are ungated — treat their trees as unmeasured.**

| lane / PR | state | what is left to do |
|---|---|---|
| **#715 structmerge** | **MERGED** `dbe854ba3` | nothing |
| **#719 calleevoteperf** | open, branch `fix/calleevoteperf` head `65da6af10`, **review APPROVED (round 2)** | only the lander step. It was running when the stop came and never took the merge lock, so #719 is untouched. |
| **#718 callbacktype** | open, branch `feat/callbacktype` head `d9059c0ef` (my ungated WIP commit on top of the peer's `081fdd26d`) | round-2 review returned CHANGES with 3 required changes (below); the fix round was mid-flight |
| **structdeep** | no PR, branch `feat/structdeep` head `ab4939b87` (ungated WIP) | the implementer was mid-work; it depends on #715, which has now merged |
| **cast census** | 3 of 3 census lanes finished and pushed on `campaign/castcensus`; the planner never ran | write the plan (section 4) |
| **round-H results PR** | never started | the ledger worktree is on `campaign/ledger-8`, clean |

### #718's required changes (from the round-2 adversarial review)
1. Refuse the park when the function's own recovery found **fewer** inputs than the declaration and
   any direct call site exists — otherwise the closed list materializes arguments at those sites.
   Repros: `/tmp/rev-callbacktype-counterexamples/x7.c`, `x6.c`.
2. Add the mirror of `Decline::ReturnsAValue`: refuse when the slot's declared return is non-void
   and the body computes no return value. Repros: `x2.c` (`CONCAT44(dat_4,...)`), `x5.c`.
3. Decide and state the policy for a park that adds a parameter to a function with no direct
   call site.

---

## 3. The operating protocol (do not drop these — they are why the work has held up)

- **Every agent reads `/home/mahaloz/kwt/COMMON_BRIEF.md` first.** It carries the worktree recipe,
  the base-aware spec-linking command, build env (`CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0`),
  the gate list, PR format, commit trailers, and a per-round "state of main" section.
- **One writer per worktree.** Never `git stash` (the stash is shared across worktrees and has
  already handed one agent's work to another).
- **Every PR gets an adversarial reviewer** with two lenses (does the mechanism fire; is there
  wrong output) on binaries **disjoint** from the implementer's sweep, before any merge. Reviewers
  have caught real wrong-output defects in a large fraction of PRs, including several where every
  gate was green. This is the single most valuable rule in the campaign.
- **Gates, all of them, on the rebased tree:** `make test` (675/675 PARITY OK), `make test-stages`,
  `make check-spec` + `python3 tools/check_spec.py --strict`, `make test-cli`, `kuna catalog --check`,
  `python3 -m scripts.repipe.counters --check`, `docs/options.md` byte-fresh, and `make rust-test`
  via `/home/mahaloz/kwt/rust-test-lane.sh` (3 flock lanes; CI skips it on internal PRs, so the
  agent is that gate).
- **Merging is serialized** on `flock /home/mahaloz/kwt/.locks/merge`, inside one script that does
  rebase → re-derive counts → gates → push → CI → merge → worktree cleanup.
- **Anything that can change emitted C ships behind a named option** (a `[[settable]]` row in
  `phases.toml` + registration in `p0_knowledge/options.rs`); adding one bumps several hard-coded
  count sites — always `counters --rederive`, never arithmetic. A strict fix to wrong output needs
  no option.
- **Default-ON needs evidence:** 0 of 675 datatest assertions moved, stage moves read one by one,
  `test-cli` probes moved only with a stated reason, the 444-slice typesweep improved ≥ worse with
  every worse row read, speed within +5% (interleaved min-of-15), and a whole-corpus hunk
  classification where every changed function falls in the documented class.
- **The metric is pinned.** The decbench checkout moved to `8cfc216`, whose `type_match` drops
  `_effective_offset` and import-fails every instrument. Pin `625e892` the way
  `docs/decbench/typecampaign/final-c/pindb.py` does — `PYTHONPATH` alone is outranked by the venv's
  editable finder. **Never write into `/home/mahaloz/github/decbench/results`.**
- **`[AUTOMATED]` prefixes** every PR, issue, comment and commit. PR bodies: three short sections,
  repro first, under a screen, no internal process.

---

## 4. The user's newest ask: too many casts

Verbatim: *"One thing I am noticing across many samples is how often casting occurs in Kuna,
compared to other decompiler, specifically IDA Pro. We should work do have less casting BECAUSE our
typing system is better and requires less explicit casting because we already know what it is. Look
at IDA Pro. Note: that does not mean just hide casts, it means meaningfully remove them when
possible."*

A three-lane read-only census finished and is committed on branch `campaign/castcensus`
(worktree `/home/mahaloz/kwt/castcensus`, reports in `.scratch/{measure,engine,ida}.md` with the
scripts and raw data beside them). **The planner agent that was to turn it into a ranked
implementation plan never ran — that is the next step.** What the census established:

**The gap is real and the campaign made it worse.** On 4,815 functions matched by address across 45
binary/opt combinations: casts per 1,000 lines — IDA 155.4, angr 101.5, Ghidra 198.4, kuna at the
campaign baseline 211.3, **kuna on current main 235.2**. Per statement kuna is 1.24× IDA.

**Where they come from.** 78% of kuna's casts are `CPUI_CAST` ops from one function
(`ActionSetCasts`) on one decision (`CastStrategyC::cast_standard`). **48.6% of all casts are the
pointer↔integer round trip around a single `CPUI_INT_ADD`**, and 74% of those have a `void *` base
with a constant offset — a base type kuna itself chose (`ptrfromuse void`, #690).

**The three leading levers, all measured:**
1. **Keep pointer arithmetic in pointer terms.** 3,771 casts at 2,944 sites — about half the whole
   gap to IDA. IDA does address arithmetic in integers and casts once at the load; kuna converts a
   pointer to an integer and back, and casts twice.
2. **`ptrfromuse void` → `byte`** removes **14.5% of all casts** (5,622 → 4,808 on fmt/ls/sort/du
   O0) at **zero metric cost** (241 → 241 perfect, aggregate +1.10, 0 functions worse).
3. **Synthesize records for `void *` bases** — improves the type *and* removes two casts per site.

The `ida` lane's own conclusion is worth carrying: kuna's excess casts are mostly *the price of
kuna's better type*, not a worse one (IDA types 52% of parameters as a 64-bit integer where kuna
types 60% as a pointer). Nothing in the top five levers requires weakening a type. Where IDA has no
cast because an implicit conversion hides a real truncation, **that is a bug to not copy** — the
user's rule is that a removed cast must not change the value the program computes.

---

## 5. Ranked next work (my recommendation)

1. **Land #719** — approved, lander only. Re-run its measurement on the rebased tree first: main
   gained #715 (type-affecting) since it was measured. Discard
   `/home/mahaloz/kwt/calleevoteperf-land-artifacts/` (a stopped peer session's half-run with no
   baseline arm — see `/home/mahaloz/kwt/calleevoteperf/.scratch/ORCHESTRATOR_NOTICE.md`).
   Its `make rust-test` lane was still running when the stop came and was killed with the rest of the
   campaign's processes, so that gate is **unrun for `65da6af10`** — the lander re-runs it on the
   rebased tree anyway.
2. **Finish #718** — the three required changes above, then re-review. Note
   `/home/mahaloz/kwt/callbacktype/.scratch/ORCHESTRATOR_NOTICE.md`: two commits on that branch came
   from another session and **their gates were never run**, despite what `state.md` says.
3. **Write the cast plan** from the three census reports, then implement it lever by lever. This is
   the user's live request and the largest readability win available.
4. **structdeep** — struct layout recall is .0932 and nesting F1 .0036; 8,626 of 9,443 ground-truth
   fields are never claimed. Branch has ungated WIP.
5. **Round-H results PR** — re-measure on the final main and append a Round H section plus a
   campaign summary to `results.md` (the round-G write-up ends with the ranked levers).

**Upstream (decbench) items, worth more than any engine change and not yet acted on:**
- `Noelo-Lab/decbench#93` (open, filed by this campaign): credit any struct pointer against a
  ground-truth pointer-to-struct. Worth **+572 functions** to kuna today; needs a `pointee_kind`
  field on the decompiled side, so kuna's JSON export would need a small follow-up.
- `Noelo-Lab/decbench#94` (open): `DW_TAG_restrict_type` is unhandled, so `char *restrict` ground
  truth is recorded as `void` — 4,412 variables. Worth only ~8 functions to kuna (98% of them are
  register-only), but it is a one-line fix and it affects every tool.
- **Not filed, awaiting the user's answer:** decbench scores an inlined callee's variables as locals
  of the function they were inlined into. Excluding them would take O2 from 74 → 153 perfect and
  O2-noinline from 380 → 498 **for every decompiler**. The O2 census
  (`final-e/o2census/`) is the evidence.

---

## 6. Ceilings — what no amount of engine work will move

- **Register-resident ground truth:** 25,821 of 65,715 scored variables (39.3%) live only in
  registers. kuna exports none of them, and the metric can only match them by name anyway (binja
  gets 51 of 7,091 this way).
- **Program-defined struct names** (`hash_entry`, `fileinfo`, …): 10,559 variables carry a name no
  stripped binary holds. No decompiler recovers them; they are reachable only under decbench#93.
- **O2 generally:** 85.8% of O2 functions contain at least one unmatchable variable. On the
  functions that *can* reach perfect, kuna converts 43.8% — the same rate as at O0. The O2 number
  is the benchmark's ceiling, not kuna's weakness.

---

## 7. Traps that have cost real time

- `kuna decompile` forks `decomp_dbg`, so rebuilding only `kuna-cli` ablates nothing — build
  `-p kuna-cli -p kuna-console -p kuna-harness` or run `make binaries`.
- The console prints `int4`/`uint4`; the CLI (aggressive under 500 KiB) prints `int`/`unsigned int`.
  Never mix the two vocabularies in a string match.
- `--save-baseline` without `SLEIGHHOME` writes an **empty** baseline.
- `export A=x B=$A` on one line leaves `B` empty (this faked a red gate once).
- `gh pr merge` can fail with a GraphQL 502; `gh api -X PUT repos/<repo>/pulls/<n>/merge -f
  merge_method=squash` works. `gh pr edit` silently no-ops here — use `gh api -X PATCH`.
- A stale `"RUNNING"` line in a lane's `state.md` after a killed process is how an unmeasured commit
  gets merged. Two such lines were found during this stop; both are now corrected in place.
- `type_match` cannot see fabricated variables or arity: any feature that adds or removes
  variables/arguments must carry its own arity and phantom counters beside the metric.

---

## 8. Other sessions

A second Claude session (`kuna-type-recovery-campaign [8beaa2]`) briefly relaunched round H,
believing this session had died. It stopped everything, reverted its edits to `COMMON_BRIEF.md`, and
handed the work back. It owns no lanes. Its two useful warnings (the ungated commits on
`feat/callbacktype`, and the baseline-less measurement in `calleevoteperf-land-artifacts/`) are
recorded as `ORCHESTRATOR_NOTICE.md` files in the affected worktrees.

## 9. Resuming

Workflow scripts (each is a self-contained JS file; edit and re-invoke with `scriptPath`):
`/tmp/claude-1000/-home-mahaloz-github-kuna/5b38c0e9-b59d-4a19-b411-e2c1a4a26e10/scratchpad/round-{d,e,f,g,h}.js`
and the cast census at
`/home/mahaloz/.claude/projects/-home-mahaloz-github-kuna/5b38c0e9-b59d-4a19-b411-e2c1a4a26e10/workflows/scripts/cast-census-wf_6d8f225d-f10.js`.
Each round's script is the previous one with a new `ITEMS` list, so the lane/review/land machinery
is reusable as-is. Do **not** resume `wf_7d7b113b-58c` by run id: another session appended to its
journal, so cached results there are unreliable — start a fresh run with the remaining items.

Persistent memory for this work lives in
`/home/mahaloz/.claude/projects/-home-mahaloz-github-kuna/memory/` — `kuna-type-campaign.md` (round
by round) and `kuna-cast-density.md` (the casting ask).
