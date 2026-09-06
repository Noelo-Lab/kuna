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
| `already-supported` | the acceptance already passes: **the tester was wrong.** Kept as a ledger — but see "an empty bucket is not a broken gate" below before reading a zero |
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

## 4b. The standing brief: be an interface, not an oracle

The corpus is deliberately hostile — 171 of its 250 challenges carry at least one obfuscation
class, and 57 are code-virtualised. Hostile binaries are precisely where a decompiler's
automatic answer is wrong, so the interesting question this loop asks is not "why did kuna
guess wrong" but **"why could the agent not tell it the right answer"**.

That is a standing instruction to both halves of the loop, and it is in both prompts:

- **Testers** are told to file a missing *interface* whenever they think "kuna should have
  known this" — because the useful form of that complaint is almost always "kuna should have
  let me say so". Define a function boundary, override a jump table, declare a blob as data
  with a type, steer structuring, fix a prototype, make a rename persist. They are told to
  check `kuna catalog --json` first: an option that already exists is a discoverability
  problem, not a missing interface, and that is worth knowing too.
- **Builders** are told to *expose* before they invent. `kuna-console` registers ~37
  intervention-shaped commands — `map function`, `override jumptable`, `force goto`,
  `force datatype`, `parse line`, `retype`, `structure blocks` and the rest — and **none is
  reachable from the `kuna` binary**. Three very different jobs hide behind "add an
  interface": wiring something that already works (cheap, the common case), porting a
  registered stub that answers `engine integration not yet ported` (real engine work, take
  the `[PROPOSAL]` route), or adding a genuine new judgement call (that is a `phases.toml`
  option, not a subcommand).

Two rules keep the result usable by the thing it is for:

**A durable assertion beats a one-shot flag.** The phase model is
`assert(phase, anchor, type, value, strength)`, consulted on every re-run. An agent that
renames forty functions and loses them on the next invocation has gained nothing.

**Machine-readable in, machine-readable out.** Accept assertions as a file or repeatable
flags; emit JSON beside the human form.

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

### Round 1 — what actually happened, measured against those criteria

Nine challenges, three codex testers, 26 observations filed, gated on a pinned `main`.

| Criterion | Result |
|---|---|
| ≥6 needs pass the two-arm gate | **23 admitted** of 26 |
| ≥2 rejected as `already-supported`/`user-error` | **1** `already-supported`, 1 `not-reproducible`, 1 `unrunnable`. Strictly, the bar was **missed**: only one filing was refuted as *kuna was already fine*. |
| ≥1 hypothesis overturned outside the `absence` class | **yes, 3** — see below |
| ≥1 PR merged, ≥1 acceptance promoted to `tests/cli/` | **9 acceptances flipped; 4 promoted** |

The gate's sharpest moment was three near-identical filings about whole-binary JSON size
landing on three *different* verdicts — one `admitted`, one `already-supported` (its
acceptance already passed), one `not-reproducible` (its probe failed). Text dedup would have
merged all three; predicates separated them.

**Three overturned hypotheses**, all in the same shape the decbench campaign found — the
symptom was real every time and the diagnosis was wrong:

- *bogus function at `0xfe6dca9f`* — filed as a discovery heuristic misfiring. Actually
  `listing/walk.rs` runs two worklists that disagree about what counts as code: the
  instruction worklist gates every address on the executable-range universe, the function
  worklist took a direct `CALL` target unconditionally. The witness is an `e8` read one byte
  early behind an always-taken `je`.
- *`main` typed `void(void)`* — filed as "interprocedural recovery failed to propagate".
  Half right: nothing propagates *because* kuna recovers parameters from the callee's own
  body, and `main` never reads its argument registers. The fix reads the caller instead, and
  is PE-only because on ELF the CRT lives in libc and there is no in-image call site to read.
- *dialog dispatch renders `switch(0)`* — filed as a switch-recovery bug. Actually
  `loweredswitch` detects the cascade on the **simplified** graph and installs the
  `BRANCHIND` on **re-lifted raw p-code**; the two halves never see the same graph, neither
  recovery arm is checked, and the surgery commits regardless.

**Closure, measured by re-running both arms on the merged build.** The acceptance arm says
9 flipped; the probe arm says 13 bad behaviours stopped reproducing. Neither number is the
answer, and the gap between them is the finding:

| | count | |
|---|---|---|
| genuinely fixed at defaults | **12** | probe gone *and* the change is real |
| fixed, but behind a default-OFF option | 2 | `switchselector`, `linuxsyscall` — correctly still reproducing at defaults |
| still open | 9 | |
| **falsely reported fixed** | **1** | see below |

The false one is the most useful result of the round. A probe asserted
`_secret_function(v2);` for a void function called with an argument. The merged build emits
`_secret_function(v3);` — the identical defect with one renumbered local — and the probe
stopped matching, so the machine reported the bug **gone**. An over-specified *acceptance*
leaves finished work looking open; an over-specified *probe* closes work that was never done,
and nothing downstream would ever have caught it. Both prompts now say: assert the property,
never a `vN`, a `sub_<addr>`, or a whole signature line.

**What the acceptance re-run then showed, which the gate could not.** Of the 14 acceptances
that did not flip, several assert a *rendering the tester imagined* rather than the symptom
they observed: one demanded `mprotect(` where the syscall is actually `write`; one demanded
the literal token `switch(a1)` where the shipped fix correctly emits the compiler's own
if/else-if chain over the real parameter. Both underlying defects **are** fixed. This is the
`unprobeable` trade from the other direction: a probe precise enough to run is also precise
enough to over-specify, and an acceptance that over-specifies reads as an open defect
forever. Round 2's tester brief should say: assert the symptom's *absence*, not the fix's
spelling.

**A structural finding worth more than any single fix.** A quality fix ships behind a
default-OFF option, but a tester-authored acceptance always invokes kuna with defaults — so
a correct fix behind a default-OFF flag can *never* flip its own acceptance. Two of round 1's
four quality options shipped OFF and their acceptances still read as failing. The acceptance
suite must record the option set a need was closed under, or it will keep re-filing work that
is already done.

## What an adversarial review found, and where it stands

The implementation was reviewed by four independent agents, every finding re-run by a
refuter: **32 confirmed, 17 refuted**. All 32 are fixed. The list is kept here rather than in
a PR description because the next person to touch this code needs to know which mistakes were
already made in it.

The eight that were critical, and would each have made the loop unusable or unsafe:

| | |
|---|---|
| the loop was **inert** | `run.sh` ran only the housekeeping tick and never `captain.sh`, so 72 h of ticks would advance nothing past `T_IDLE`/`B_IDLE` |
| every lease and slot was **dead on arrival** | the recorded pid was the ephemeral `state …` CLI process, so `merge` granted to everyone — and `reap` failed **live** workers and released their claims |
| the probe allowlist was bypassable | basename-only matching: any file named `kuna` ran, and a tester has workspace-write |
| the sandbox hid one directory | a second full copy of all 250 flags sits in the sibling label repo, and `$HOME` held SSH keys, a GitHub token and 600 past prompts |
| the gate→cluster seam passed no observation | every clustered need would have been one empty record |
| `--merge` raced the `full-ci` registration | it could squash-merge before the workspace suite even registered, and failed *open* on a draft-query error |
| tester prose could hijack a record | an `## Acceptance` heading in a tester's own text replaced the real probe — which is then auto-closed and promoted |
| the arena shims were never on `PATH` | no tool-call log at all, and IDA's `.i64` files uncontained |

Four more came from the **first live run**, and none of them could have been found offline:
the report schema 400s in strict structured-output mode; `gate_round` did not supply `{{BIN}}`
so a correct observation was discarded as unrunnable; the IDA shim never passed
`--backend ida`, so "compare against IDA" silently compared against angr; and declib's socket
lives under `TMPDIR`, outside the tester's sandbox, so every reference call failed.

Each fix carries a regression test in `tools/repipe/smoke.sh` or `tests/cli/`. Two are worth
knowing about because they shape how you should extend this code:

- **A regex guard is not a sanitizer.** CodeQL rejected a charset gate on a URL id and was
  right to: the tainted string still flowed into a path. The accepted fix builds the path from
  `os.listdir()` and compares the id against filesystem-produced names by equality, which
  makes traversal structurally impossible rather than guarded.
- **Negative operators are universally quantified.** `absent` under `[*]` means *no* element
  has it, not *some* element lacks it. Existentially-quantified negation let a probe and its
  own negation both pass on a mixed array, which the gate reads as `already-supported` — i.e.
  it silently discards a real need.

## A conflicting PR runs no gates at all, and looks green

Round 1's PR sat for half an hour showing six green `Analyze (...)` checks and nothing else.
The `Tests` workflow had never run — not queued, not skipped, **no run object at all** — and
the reason was that `main` had moved ahead and the PR was `CONFLICTING`. GitHub does not
dispatch `pull_request` workflows when it cannot compute the merge commit, and CodeQL's
default setup is a different mechanism that runs anyway.

So the observable state of a conflicting PR is: CodeQL green, every gate absent, and
`gh pr checks` showing an all-green list. That is precisely the "all-green PR that had
executed no code" that `tests.yml`'s own header describes.

What catches it is the `missing` arm in `open_pr.sh --merge`:

```sh
WS_CONC=$(... next((r.conclusion for r in check_runs if r.name == WS_NAME), "missing") ...)
if [ "$WS_CONC" = "skipped" ] || [ "$WS_CONC" = "missing" ]; then
  echo "ERROR: '$WS_NAME' concluded '$WS_CONC' -- it did not actually run; not merging"
```

`missing` was written for a different case (a required check renamed out from under the
guard) and it caught this one. **Do not soften it to "absent means not required".** The
diagnostic to reach for first is `gh pr view <n> --json mergeable,mergeStateStatus`:
`CONFLICTING`/`DIRTY` explains an absent suite far more often than anything about the
workflow file does.

## A quota-killed builder loses its work silently

Round 2's two builders both ended `claude rc=1` about 30 minutes in, well inside
`REPIPE_BUILDER_TIMEOUT`. The reason lives only in the result JSON:

```
$ python3 -c 'import json; print(json.load(open(".kuna-repipe/logs/<wid>.result.json"))["result"])'
You've hit your session limit · resets 4:50am (UTC)
```

`is_error: true` with `subtype: "success"` — so neither the exit code nor the subtype tells a
quota kill from a model refusal or a crash. **Check the `result` string.**

One of those builders was in its `docs` phase with 618 insertions across 19 files — a new
subcommand, a promoted `tests/cli` probe, a console verify test — all uncommitted, and it was
recovered by hand. `worker.sh` now preserves that itself: on a failed session it commits the
worktree to the worker's own branch as an explicit `WIP UNFINISHED, DO NOT MERGE` snapshot
naming the phase and stating that no gate ran, and a re-dispatch onto an existing same-branch
worktree now **reuses** it instead of dying on `worktree add`.

Two things that fix deliberately does not do, both of which a first attempt did and was
rejected for:

- It never runs `git worktree remove --force` on a stale or wrong-branch directory. That flag
  is exactly what deletes a worktree holding modified and untracked files, so tidying would
  destroy the work the change exists to preserve — strictly worse than today's harmless
  failure. Those cases still fall through to the add-and-fail path.
- It is not an `EXIT` trap. An `EXIT` trap fires on SIGTERM without waiting for the foreground
  `claude` subshell, so it would stage a tree still being written.

The WIP commit is refused when the worktree is detached, mid-rebase, or no longer its own
worktree: `git commit` lands on whatever HEAD is, and a commit the branch cannot reach while
the log says otherwise is worse than no commit.

## An empty `already-supported` bucket is not a broken gate

Round 1's criteria say "≥2 rejected as `already-supported`/`user-error` — if this is zero, the
gate is not working". Round 2 reported **zero** and the gate was fine. The criterion was
obsoleted by a change made here after round 1.

A result carrying `reasons: ['probe-fail', 'acceptance-pass']` looks like already-supported —
the bad behaviour is gone *and* the desired behaviour works. Usually it is not. Round 2's one
such record:

```
probe.expect      {"stdout_matches": ["sub_418fb0\\(\\)"]}
acceptance.expect {"stdout_absent":  ["sub_418fb0\\(\\)"]}
```

**Exact polarity inverses on the same regex.** `probe-fail` and `acceptance-pass` are one
fact — `sub_418fb0()` is absent — reported twice. Relabelling it `already-supported` would
assert kuna does the desired thing when all that was observed is the symptom's absence.
`not-reproducible` is the correct, weaker claim, and the existing precedence already yields it.

The obsolescence is self-inflicted and worth naming. The tester brief now says *assert the
symptom's absence, not the fix's spelling* — the right fix for round 1's over-specified
acceptances, and it **manufactures negation-shaped arm pairs**. 11 of round 2's 22
observations have that shape. The better acceptances get by that rule, the closer this bucket
goes to zero, because a negation pair can never populate it.

So judge the gate on whether it refutes anything at all — `not-reproducible` and
`already-supported` together — and treat a zero in either alone as uninformative. A proposal
to flip the precedence for these pairs was written and **rejected** in review for this reason;
before re-litigating it, check whether the two arms are independent or complementary.

## "It did not error" is not "it worked", and kuna will not tell you

Round 3 filed five independent observations saying prototype assertions reject standard C
types. Reviewing them, I ran the assertion by hand, saw no error, and concluded the testers had
made a quoting mistake. That was wrong, and the way it was wrong is the lesson.

```
kuna decompile <bin> main --assert "prototype main int main(void)"
unsigned long main(void) { ... }          # exit 0, no error, no warning

kuna decompile <bin> main --assert "prototype main int main(void)" --json
  assertions[0].status = "rejected"       # the override was silently discarded
```

`int` is rejected; `int4` is applied. The testers were right on all five, the gate admitted all
five, and the refutation was mine. The trap: **the text surface exits 0 and prints a function
whether or not the assertion took**, so "no error" reads as success to anyone who does not think
to ask a second question. A sixth tester filed exactly this as its own observation — *text output
silently ignores a prototype override that JSON output applies*.

Two standing rules follow.

**For a reviewer refuting an observation.** Never refute on the absence of an error. Check the
state the observation is *about*: for an assertion that is `assertions[].status` in `--json`, not
the exit code and not the presence of output. If the surface has no way to report whether the
thing took, that absence is itself the defect — file it rather than concluding the tester was
confused.

**For the gate.** This is why the two-arm gate runs the probe rather than reading the report. A
human or an LLM reviewing five reports by eye reached the wrong verdict on all five; the gate
replaying the probes reached the right one on all twenty-three. When the machine and the
reviewer disagree, re-run the probe before believing the reviewer.

## The tester model can refuse the task, and that is not a kuna result

Round 3 lost a tester 78 seconds in, `codex exited rc=1`, no report, nine tool calls of
evidence gone. The cause is only in the event stream:

```json
{"type":"turn.failed","error":{"message":"This content was flagged for possible
 cybersecurity risk. ... To get authorized for security work, join the Trusted Access
 for Cyber program"}}
```

The provider declined the work. It happened on **6 of 36 tester runs**, and on **all three
attempts** at challenge `63d5a26a` — which makes that challenge systematically unmeasurable
with this tester model rather than hard. Three other runs hit it mid-session and still filed
reports, so it is not always fatal.

This matters because it silently corrupts the two numbers the loop exists to produce. A
refusal exits `1`, exactly like a crash, so it lands as a generic `failed` — and a challenge
nobody was allowed to attempt is indistinguishable from one kuna could not support. Both the
solve rate and `gave_up: kuna-blocked` drift by however often it happens.

`tester.sh` now reads the event stream and records `--phase refused` with a
`provider-refusal:` note, so `scripts.repipe.status` and the round's grading can tell it apart
from a kuna failure and from a harness fault.

**Recorded, not worked around.** Do not reword prompts to get past the classifier — the
refusal is the provider's call, and the sanctioned route is the authorization programme the
message names. If the refusal rate rises far enough to starve a round, the options are to
raise it with the provider or to run the tester on a model licensed for this work; neither is
something the loop should route around on its own.

## Working in `docs/re-needs/` while the loop is running

The loop writes need records as UNTRACKED files and only commits them at `INTEGRATE`, so
there is a long window where a round's entire evidence base exists solely in the working
tree. Two things follow, and I hit both in one session — the second within minutes of writing
down the first.

**1. Never `git add docs/re-needs/` (or any directory the loop writes).** It stages whatever
the loop has in flight. Name the files you actually changed:

```sh
for f in a b c; do git add "docs/re-needs/$f.md"; done   # not: git add docs/re-needs/
```

Staging 17 of the loop's untracked round-3 records onto a side branch made them tracked
*there* and not on `main`.

**2. That is only half of it. Once a file exists only on a branch, every `git checkout main`
DELETES it from the working tree** — correct git behaviour, silent, and it happens on a later
unrelated command rather than at the moment of the mistake. The loop reads the working tree,
so a running round loses those needs immediately. Fixing the commit does not fix the tree.

After any branch switch in this repo while the loop is live:

```sh
git checkout <branch> -- docs/re-needs/          # put them back, then verify
ls docs/re-needs/*.md | wc -l
python3 -m scripts.repipe.needs reindex
```

The damage is recoverable — an orphaned commit still holds the files, findable with
`git log --oneline --all --reflog` — but it is invisible until a count comes back wrong. In
this case 14 records vanished and the symptom was a round-3 need count reading 2 instead of
16, noticed only because the number was checked for an unrelated reason.

`docs/re-pipeline.md` has the same hazard in a milder form: the loop and an operator both
append sections, so a PR left open across a round comes back conflicting. Two PRs were closed
unmerged that way and their content had to be re-applied by hand.

## `SIGTERM` to a supervisor drains the whole loop, not that process

`run.sh` traps `SIGTERM` and does a graceful stop, which writes `.kuna-repipe/STOP`. That
file is **shared state**: every supervisor sharing the state dir sees it, transitions the
round to `DRAINING`, and stops dispatching. Removing the file does not undo it —
`DRAINING → RUNNING` is deliberately not a legal edge, so the round can only go on to
`STOPPED`.

So `kill <supervisor>` is not "stop this process". It is "end this round". To remove a
duplicate or stuck supervisor without ending the round, use `kill -9`, which skips the trap.

The situation that leads here is worth naming, because the expensive mistake was three steps
upstream: **two supervisors were running.** One had been launched hours earlier and declared
failed because `tail` could not read its log — the log path was wrong, the process was fine.
They do not corrupt anything (the captain slot lease serialises them, so one always loses the
race) but they double the tick rate and each counts budget independently.

After launching a supervisor, check that exactly one exists:

```sh
pgrep -f 'repipe/run.sh' | while read p; do
  ps -o args= -p "$p" | grep -q shell-snapshot || echo "$p"
done | wc -l          # must be 1
```

`pgrep -f` matches this shell's own command line, so a naive count reads 2-3 and hides a real
duplicate in the noise; filter the wrapper out. And never conclude a background launch failed
because its log is unreadable — check for the process.

## The quality lane is capped at one builder, and the reason may have expired

Round 4's captain found this while answering whether to run testers, and the measurement is
worth keeping: **`select -k 3` returns TWO picks**, not three, against an 11-need backlog. A
builder slot is unfillable at any backlog depth. 10 of the 11 open needs are `quality`, and

```python
TRACK_RESOURCES = {"quality": ["counter:catalog", "counter:stages-corpus", "counter:div",
                               "file:phases.toml", "file:docs/options.md"], ...}
```

means every quality need takes the same five leases, so at most one quality builder runs at a
time. With a quality-heavy backlog — which is what three rounds have produced — two of three
builder slots idle permanently. Backlog *depth* is not the throughput constraint; lease
*structure* is.

The serialisation was correct when written. Its stated reason is the silent-identical-edit
shape: "an identical `85 -> 86` edit on two branches merges CLEANLY to the wrong number",
which this pipeline hit for real in round 1 (two branches at 131 and 129 when the truth was
133).

**Two guards have since been built that did not exist then**, and both sit at merge time:

- `builder_prompt.md` takes `lease-acquire --resource merge` and then runs
  `scripts.repipe.counters --fix` INSIDE that lease, on the rebased tree — every counter is
  re-derived from the live tree rather than arithmetic, so a raced number is repaired, not
  shipped.
- `counters --check` runs in the parity-gates CI job, so a wrong number fails the build even
  if the repair is skipped.

The merge is therefore already serialised on its own lease, and the numbers are already
re-derived under it. `TRACK_RESOURCES` additionally serialises the whole BUILD — an hour of
analyze/design/code/test/docs — to protect a step that takes minutes and is protected twice.

**This is a candidate change, not a conclusion.** What would settle it:

1. Two quality builders working concurrently and merging serially, with `counters --check`
   green on main afterwards. That is the whole claim.
2. The residual risk is not the counters but the **DIV number**: two builders each claiming
   the next free one. `counters --fix` re-derives counts, not registry allocations. Check
   whether the merge-time DIV claim in `docs/improvement-pipeline.md` §4 actually reallocates,
   or only renumbers references.
3. `file:phases.toml` itself is probably safe to drop — two option rows in different places
   auto-merge, and adjacent ones conflict loudly, which is the safe failure.
4. `file:docs/options.md` is regenerated from the catalog at merge and never merged, so it
   needs regeneration, not a lease.

Do not relax this while a round is in flight, and do not relax it without (1). The payoff is
roughly 3x on quality throughput, which is where this backlog lives.

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
