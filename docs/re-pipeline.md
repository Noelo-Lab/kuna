# The kuna RE-friction loop

**The task**: find out where `kuna` fails an agent that is actually trying to reverse-engineer
a binary, and close those gaps — autonomously, with no human in the loop. This file is the
runbook: an agent (interactive or headless) should be able to run the whole cycle from what is
written here.

Its sibling, `docs/improvement-pipeline.md`, asks *"is kuna's emitted C worse than angr's?"*
This one asks the question that matters for an agent-first decompiler: **can an agent use it
at all?** The two loops share their scheduler, claim registry and PR opener; they differ in
where the work comes from and what "done" means.

One iteration:

1. **Test** — codex agents try to solve crackmes from `~/github/kuna-re-dataset` with kuna as
   their primary tool, and record every place it was missing, wrong, slow or costly (§2).
2. **Gate** — every recorded observation is replayed by machine. Two executable predicates
   decide whether it is real (§3).
3. **Cluster** — surviving observations collapse into needs in `docs/re-needs/` (§4).
4. **Build** — claude agents close one need each, on one of three tracks, and self-merge (§5).
5. **Verify** — the acceptance probe is re-run on merged `main`. Needs that flipped are
   closed; the next round's testers are pointed at the new surface and asked to break it (§6).

## 0. Prerequisites

```bash
make binaries && make specs
tools/repipe/run.sh --preflight        # hard-fails on anything missing
```

Preflight requires: `git`, `gh` (authed, `repo` scope), `codex`, `claude`, `python3`, a built
`kuna`, compiled `.sla`, the dataset, and `REPIPE_MIN_FREE_GB` free. It also **fails if
`bwrap` is unavailable** — see §2, containment is not optional by default.

Everything is stdlib Python run as `PYTHONPATH=$REPO python3 -m scripts.repipe.<mod>`; there is
no install step and no third-party dependency, matching `scripts/pipeline/`.

## 1. Running it

```bash
tools/repipe/run.sh                     # bounded: REPIPE_ROUNDS (default 3)
tools/repipe/run.sh --once              # exactly one full cycle
python3 -m scripts.repipe.webui --port 8787     # watch it
touch .kuna-repipe/STOP                 # graceful drain; INTEGRATE still runs
touch .kuna-repipe/PAUSE                # finish what is running, spawn nothing
touch .kuna-repipe/ABORT                # hard stop; worktrees and arenas left intact
```

| Env var | Meaning (default) |
|---|---|
| `REPIPE_MAX_AGENTS` | total concurrent LLM processes (**7** = 1 captain + 3 testers + 3 builders) |
| `REPIPE_TESTER_SHARE` | tester fraction of the non-captain slots (0.5) |
| `REPIPE_ROUND_CHALLENGES` | challenges per round (9) |
| `REPIPE_TESTER_TIMEOUT` / `REPIPE_BUILDER_TIMEOUT` / `REPIPE_CAPTAIN_TIMEOUT` | 3600 / 7200 / 1200 s |
| `REPIPE_BUILDER_USD` / `REPIPE_ROUND_USD` / `REPIPE_RUN_USD` | 25 / 150 / 1500 |
| `REPIPE_MIN_FREE_GB` / `REPIPE_HALT_FREE_GB` | stop dispatching / halt outright (250 / 60) |
| `REPIPE_SANDBOX` | `auto` \| `bwrap` \| `none` — `none` is prompt-only containment |
| `REPIPE_ENABLE_IDA` | let testers reach IDA as a logged last resort (1) |
| `REPIPE_REFUTE_MODE` | `absence-skip` — do not spend refuters on "the subcommand does not exist" |
| `REPIPE_DATASET` | the crackme corpus (`~/github/kuna-re-dataset`) |
| `KUNA_PIPELINE_STATE_DIR` | live state (`.kuna-repipe/`, gitignored) |

The agent split at other values: 2→1/1/1, 4→1/2/1, 5→1/2/2, 6→1/3/2, **7→1/3/3**, 9→1/4/4.
Below 5 the two tracks cannot overlap, so the live process count is
`captain + max(testers, builders)`. Above ~10 the merge lease, not the slot count, is the
bottleneck.

## 2. The tester

One `codex exec` session per challenge, in a sanitized arena.

```bash
ROUND=1 HEXID=64f1f7afd931496abf909525 tools/repipe/tester.sh
```

**Containment is a mount namespace, not a policy.** codex's `-s workspace-write` restricts
writes, not reads, and the dataset gives the answer away four different ways:

| Trap | Why a prompt cannot fix it | Closure |
|---|---|---|
| `meta.json` carries `ground_truth.flag` in plaintext (98 challenges) | it sits in the directory a solver is pointed at | never copied; dataset tmpfs'd out of the namespace |
| `solutions/<hexid>/*.zip` — full writeups with working keygens (168 challenges, ZipCrypto pw `crackmes.one`) | one `unzip -P` away | same |
| `extras/` is both the only task statement *and* a spoiler channel | `challenges/5ab77f5533c5d40ad448c1ea/extras/…/hints.txt` is 19 valid serials on a challenge whose `ships_source_code` is **false** | allowlist, then deny by name, extension, flag-content, and **serial shape** |
| 6 challenges ship author source | — | dropped by extension |

So the tester runs under
`bwrap --dev-bind / / --tmpfs $DATASET`, plus `network_access=false` (which also removes
"look up the writeup"), plus a post-hoc tripwire that greps the transcript for the dataset
path, the flag, `crackmes.one` and `solutions/`. A tripwire hit marks the run `contaminated`:
its observations are kept — friction is friction — but its outcome is voided.

The arena also **meters the tester**. `bin/kuna` and `bin/ida-decompile` are transparent shims
that log every call to `notes/toolcalls.jsonl` with argv, exit code and wall-clock. That log
is the pipeline's only per-call latency signal — kuna emits no timing of its own — and it makes
"how often did kuna make a tester leave" a measured number rather than a self-report.

Two things about the corpus that a harness must respect: the primary binary is **always**
`meta.json → detected.primary.path` (never a `bin/` glob — the tree preserves recursive
extraction shapes like `bin/CrackMe_3.zip.__x/CrackMe_3.exe`), and **58 of 287 shipped
binaries have no execute bit**, so every copy is `chmod 0755`.

**Giving up is a result.** `outcome: gave_up` with `gave_up_reason: kuna-blocked` is the
loudest signal this pipeline can receive, and the prompt says so.

### Grading is deliberately weak, and says so

`grade.py` returns a tiered verdict: `flag-exact` (high) · `binary-accepts` (high) ·
`verifier-agrees` (**low**) · `unverifiable`. The low tier is flagged because these
`verifier.py` files are LLM reconstructions from public writeups that were **never validated
against the binaries** — 70 exist, 34 self-test-pass, 19 raise `NotImplementedError`, 4 are
quarantined stubs, and one ends "We return True here provisionally". Only 22 of 250 challenges
are machine-checkable *and* uncontaminated.

**So the solve rate is a secondary metric.** The primary output of a tester run is probes,
graded by replay.

## 3. The two-arm gate

The one thing this loop refuses to do is let an LLM's *narrative* reach a builder. The
decbench campaign's measured result is the reason: **round 2's refuters overturned the filed
diagnosis on 3 of 8 cases while the symptom stood in all 8**, and per `docs/decbench-loop.md`
some wrong mechanisms fire, pass their witness, and ship broken output.

So every observation carries two executable predicates, and a need is admitted only if, on a
freshly built `main` at a pinned SHA:

- the **probe** — asserting the *current bad* behaviour — **PASSES**, and
- the **acceptance** — asserting the *desired* behaviour — **FAILS**.

```bash
python3 -m scripts.repipe.verify --gate --round 1 --json
```

| Outcome | Meaning |
|---|---|
| `admitted` | real, reproducible, not already possible |
| `not-reproducible` | the probe does not fire — noise, or environment |
| `already-supported` | the acceptance already passes: **the tester was wrong.** Kept as a ledger — if this bucket is ever empty, the gate is broken |
| `flaky` | the repeats disagreed. A flaky probe is not evidence |
| `unrunnable` | malformed, or the target's sha256 does not match — a probe pointed at the wrong file **refuses** rather than returning a confident false verdict |

Everything downstream follows: dedup keys off probe signatures rather than text, a builder is
done when the acceptance flips to PASS, and **the acceptance probe is promoted verbatim into
`tests/cli/` as a permanent regression test** — so every shipped need leaves a CI guard behind
it automatically.

**What this costs.** Friction with no machine-checkable predicate — "the output is hard to
read", "I lost my renames every invocation" — is not a need. It lands in
`docs/re-needs/rejected/` as `unprobeable`. The quantitative channels (`gave_up_reason`,
`minutes_lost`, `fallbacks[].why_kuna_could_not`) still capture it and the dashboard charts it,
but this pipeline will not build it. That is the trade to revisit first if the rejected pile
turns out more interesting than the backlog.

## 4. The need record

`docs/re-needs/<need_id>.md` — YAML front-matter plus fixed `##` sections, deliberately the
same dialect as `docs/decbench/triage/<case-id>.md`.

```bash
python3 -m scripts.repipe.cluster --round 1          # observations -> needs
python3 -m scripts.repipe.needs list --json
python3 -m scripts.repipe.needs rank
```

Clustering is deterministic first: the key is `(kind, kuna subcommand, acceptance clause
shape)`, with text similarity only as a tie-breaker. Bumping an existing need's `instances`
and `challenges` is pure Python, so only genuinely novel observations ever cost an agent.

`## Hypothesis` is **advisory and explicitly not binding on the builder**. It is refuted before
dispatch (except for `kind: absence`, where "there is no `xrefs` subcommand" has no
interesting root cause) and the verdict is recorded either way.

Before any need is dispatched it is checked against `kuna catalog --json`: if an existing
option closes it, it becomes `rejected` with `covered_by_option` — a default-flip candidate for
the *other* pipeline, not new work here.

## 5. The builder — three tracks

Getting the track wrong is the most likely way to waste a builder session.

| | **`tooling`** (the majority) | **`quality`** | **`perf`** |
|---|---|---|---|
| The gap is | a missing or broken capability | kuna's emitted C is wrong | it is too slow |
| Lives in | `kuna-cli`, `kuna-console`, `kuna-analysis` | `kuna-decomp/src/pN_*/kuna_<slug>.rs` | either |
| Changes emitted C | **no** | **yes** | must not |
| `phases.toml` option | **not required** — the rule is "anything that *can change emitted C* ships behind a named option", and a new subcommand cannot | **required**, plus all 8 ritual steps | only if output moves |
| Counters | none | `kuna_phases/tests.rs` (2 asserts + the tier tuple + the count-encoding *test names*), `catalog_bytecompat.rs` (3 asserts + the `phase_catalog.json` fixture), `tests/stages/kuna-catalog.xml`, `kuna-base/src/xml.rs` | none |
| Tests | cargo tests + the promoted acceptance probe. **A `tests/stages/` case would be wrong** — that README scopes the corpus to stage-model issues | two-pass `tests/stages/gh{angr,dec}-<slug>.xml` | timing probe with a stated noise floor |
| Speed bar | before/after on the touched path | `scripts.pipeline.timeit`, ≤5% | **3σ and ≥20%** — `returncopysplit` measured a −20%/−12% noise floor on byte-identical output |

Track `quality` follows `docs/improvement-pipeline.md` §3–4 and
`tools/pipeline/worker_prompt.md` §§3–8 **verbatim**; the builder prompt includes them by
reference rather than duplicating them, so the ritual has exactly one copy.

Standing requirements 7 and 8 apply unchanged to `quality`: sweep every changed function, not
just the witness; and a refuter must answer *"would this produce WRONG output?"* by building
the change and reading the diff, not by arguing.

`loader` needs (PE/Mach-O/DOS/stripped-PIE discovery) are almost always `scope: large` and
route to `[PROPOSAL]`.

## 6. Merging, and closing the loop

Merges are **serialized** behind a `merge` lease. The sequence, after taking it:

```bash
git fetch origin && git rebase origin/main          # rebase FIRST
python3 -m scripts.repipe.counters --fix            # re-derive; never arithmetic
python3 -m scripts.repipe.mergecheck --against origin/main
make test && make test-stages && make rust-test && make check-spec
decompiler/target/release/kuna catalog --check
python3 -m scripts.repipe.verify --need <id> --json  # acceptance must be PASS
tools/pipeline/open_pr.sh --merge feat/re-<slug>
```

`open_pr.sh --merge` adds the **`full-ci`** label — which is itself a CI trigger, and without
it a PR from a branch in this repo never runs the workspace suite — waits for every check, and
squash-merges over REST. It refuses to merge a draft, and is re-runnable: a `--merge` that dies
after the squash landed sees `.merged == true` and exits 0.

Three silent-merge shapes this repo has actually produced are guarded mechanically
(`mergecheck --self-test` reproduces all three in synthetic git histories):

| Shape | What happened | Guard |
|---|---|---|
| loud conflict | a DIV number raced 55→56→57→58 | claim the number at merge, rewrite every reference |
| **silent identical-edit** | both branches made the same `85 → 86` edit, git merged cleanly, the answer was 87 | re-derive every counter from a fresh capture on the rebased tree |
| **silent keep-both** | a stale `data_footer: 375` against 381 real keys; a duplicated row in a README | diff every keep-both against `origin/main`: nothing removed, nothing added twice |

**Never re-pin `docs/baseline.json`** — `mergecheck` hard-rejects it.

### "Restart the testers when the builders have fixed what they asked for"

There is no judgment in this. A need is closed **iff its acceptance probe, which failed when it
was filed, now passes on a freshly built main**:

```bash
python3 -m scripts.repipe.verify --acceptance-suite --all --json
```

A previously-closed need whose acceptance flips back becomes `regressed` and outranks
everything. The next round's testers are automatically handed the closed set with an explicit
mandate to try to break it, so the loop closes on evidence twice: a machine re-runs the
predicate, and a fresh agent attacks the new surface.

## 7. Collision avoidance

Three layers, cheapest first:

1. **Track separation.** `tooling` touches kuna-cli/console/analysis + `tests/cli/` +
   `docs/cli.md`; `quality` touches kuna-decomp + `phases.toml` + the counters +
   `docs/options.md` + `docs/history.md` + `tests/stages/`. Disjoint sets.
2. **Named leases** with a TTL and a dead-pid reaper: `merge`, `counter:catalog`,
   `counter:stages-corpus`, `counter:div`, `file:phases.toml`, `file:docs/options.md`,
   `cluster:<id>`. A lease exists where a *silent wrong merge* is possible — not where a
   trivial rebase would resolve it. Because every `quality` need needs the whole counter set,
   **at most one option-adding builder is ever in flight** with no special-casing, while
   `tooling` builders parallelize freely.
3. **Contracts.** `.kuna-repipe/contracts.json` is rendered into every builder's prompt so each
   knows what its siblings are touching, and a builder that needs someone else's file stops and
   says so instead of racing.

## 8. The captain

A Claude Code session that performs **one bounded, guarded state transition per tick** and
exits; `run.sh` re-invokes it. `captain.py` owns three machines (Supervisor / TestTrack /
BuildTrack), appends every transition to `rounds/<n>/transitions.jsonl`, and **raises and exits
2 on an illegal one** — the captain cannot talk the machine into skipping a gate.

The captain also **approves proposals**. There is no human, so a large need's design-only draft
PR is adjudicated by the captain reading `proposal.md`, the need's instance count and
credibility, and the replayed probe: approve (re-dispatch with `IMPL_PROPOSAL=1
RESUME_BRANCH=…`), reject (`blocked`, reason recorded), or defer. Approve conservatively — a
rejected proposal costs one design; a wrongly approved one costs a builder, the merge lease and
a red main.

It runs with `--disallowedTools Task` so `--max-agents` stays an honest count: every agent must
come from a slot.

## 9. Safety

- **Disk.** A cargo worktree costs 20–30 GB and has filled this machine mid-run.
  `worker.sh` now exports `CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0
  CARGO_PROFILE_TEST_DEBUG=0` and removes `target/debug` from a **trap**, so it fires on
  timeout and crash too. A disk governor stops dispatching below `REPIPE_MIN_FREE_GB` and halts
  below `REPIPE_HALT_FREE_GB`.
- **Specs.** Never `make specs` in a worktree. `KUNA_SPECS`/`SLEIGHHOME` point at the main tree,
  but they do **not** reach the cargo workspace suite (~22 targets fail "Could not find .sla
  file"), so `worker.sh` symlinks the built `.sla` in before `make rust-test`.
- **Never `git stash`** with sibling worktrees live — `refs/stash` is one shared stack and work
  has been lost that way.
- **codex sqlite.** Each tester gets `CODEX_HOME=.kuna-repipe/runs/<id>/codexhome` (with
  `auth.json` symlinked in), so rollouts and the sqlite stay inside the run instead of
  inflating `~/.codex/logs_2.sqlite`, already 270 MB over 612 rollouts. Not `--ephemeral`:
  harvest needs the transcript.
- **IDA `.i64`** files land in the arena, because `bin/ida-decompile` points declib's
  `DECLIB_SERVER_REGISTRY` and `--project-dir` there and IDA only ever sees the arena copy.
- **Crash recovery.** State is the file, not the process. `captain.py --recover` reaps dead
  pids (freeing their slots, claims and leases) and resumes at the *recorded* state.
- **The genuinely unsafe part, stated plainly.** Builders run
  `claude -p --dangerously-skip-permissions` with the network on, inside a worktree, confined
  only by convention and post-hoc checks. There is no sandbox around a builder, and that is
  inherent to "implement in the kuna repo and self-merge". What *is* contained: `main` only
  ever advances through the serialized merge step, `docs/baseline.json` is a hard reject, and
  testers — which run exploratory work against 250 unknown binaries — are network-off and
  confined to an arena holding nothing of value.

## 10. Proving it works

```bash
tools/repipe/smoke.sh              # levels 0+1: ~4 min, zero tokens, zero network
tools/repipe/smoke.sh --level 0    # ~40 s
```

Level 1 exercises the real machine against real defects: it builds arenas for the flag
challenge and the `hints.txt` trap and asserts the flag is absent, the exec bits are repaired
and `hints.txt` was dropped **by the serial-shape rule specifically**; it runs the two-arm gate
against kuna's live `count: 0, exit 0` failure and requires the probe to pass and the
acceptance to fail; it proves a misresolved target refuses instead of lying; it merges a 3-way
duplicate into one need; it proves two `quality` needs cannot dispatch together; and it curls
every dashboard route including a path-traversal attempt.

**Four false-green canaries** (in the spirit of `make test-ghidra`'s two) fail the run if: no
probe reproduced (a broken runner would "pass" everything by failing everything), the sandbox
assertions were skipped because `bwrap` was missing, the redactor dropped *every* extras file
(over-redaction hides a broken allowlist), or clustering produced nothing.

Levels 2–4 cost real money and are run by hand:

```bash
# L2 — one live tester (~20 min, ~$2). Proves codex --output-schema compliance, the
#      thread.started scrape, and that the sandbox does not break the API call.
REPIPE_MAX_AGENTS=2 ROUND=1 HEXID=$(python3 -m scripts.repipe.sample slate --round 9 -k 1 \
  --filter 'format=ELF,size<64k,machine_checkable=true' --json | python3 -c 'import json,sys;print(json.load(sys.stdin)[0]["hexid"])') \
  tools/repipe/tester.sh

# L3 — one live builder on the smallest real seed need (~2 h, ~$20).
#      `functions-json-size` is deliberately tiny: the acceptance is one `exists` clause.
REPIPE_MAX_AGENTS=2 tools/repipe/run.sh --once

# L4 — the first real round.
REPIPE_MAX_AGENTS=7 REPIPE_ROUND_USD=150 tools/repipe/run.sh --rounds 1
```

### Success criteria for round 1, stated in advance

- ≥6 needs pass the two-arm gate.
- **≥2 rejected as `already-supported`/`user-error` — if this is zero, the gate is not
  working.** With kuna's nine-subcommand surface and an eager tester, some fraction of filings
  must be user error.
- ≥1 hypothesis overturned outside the `absence` class (the decbench prior is ~35%; zero is
  evidence the refuter is rubber-stamping).
- ≥1 PR merged, and ≥1 acceptance probe flipped to PASS and promoted into `tests/cli/`.

## Machinery reference

| Piece | What |
|---|---|
| `scripts/repipe/probe.py` | the predicate evaluator; verifies `target.binary_sha256` before running so a misresolved path cannot produce a confident false verdict |
| `scripts/repipe/verify.py` | the two-arm gate, the acceptance suite, and promotion into `tests/cli/` |
| `scripts/repipe/workspace.py` + `redact.py` | the contamination-proof arena and the four-trap spoiler filter |
| `scripts/repipe/sample.py` | the stratified round slate, deterministic per `(seed, round)` |
| `scripts/repipe/grade.py` | the tiered solve verdict and the contamination tripwire |
| `scripts/repipe/needs.py` | the durable backlog; also emits `opportunities.json`'s shape so `scripts/pipeline/select.py` consumes it unchanged |
| `scripts/repipe/cluster.py` | observations → needs, deterministic first |
| `scripts/repipe/select.py` | collision-aware dispatch: resource sets, feasibility, contracts |
| `scripts/repipe/counters.py` + `mergecheck.py` | re-derive every shared counter; catch the three silent-merge shapes |
| `scripts/repipe/captain.py` | the three guarded state machines |
| `scripts/repipe/status.py` | the terminal view: round, agents, slots, leases, backlog, disk — reuses `scripts/pipeline/status.py`'s TTL-cached collector |
| `scripts/repipe/webui.py` | the dashboard: one background refresher, many viewers, zero `gh` calls per request |
| `tools/repipe/` | `run.sh`, `tester.sh`, `captain.sh`, the three prompts, `smoke.sh`, fixtures |

Reused from the angr lane rather than forked: `tools/pipeline/run.sh` and `worker.sh` (through
new env seams that all default to today's behaviour), `scripts/pipeline/state.py` (slots,
leases and `reap` added), `status.py` (TTL cache added), `open_pr.sh` (`--merge` added), and
the whole `state proposal`/`approve`/`claim-approved` + `IMPL_PROPOSAL=1` path, which needed no
change at all — the captain simply plays the human.
