# The decbench improvement loop

The decbench-driven continuation of the angr feature campaign (`docs/improvement-pipeline.md`).
The signal source changes — a real-binary benchmark instead of angr's testcase
corpus — but the feature factory downstream is identical: one PR per feature,
option-gated, default-on when clean, measured.

## The signal

[decbench](https://github.com/mahaloz/decbench) scores decompilers per function on a
compiled-project corpus (2026-07-27 run: **94,575 functions / 803 binaries / 39 projects ×
{O0, O2, O2-noinline}**, eleven decompilers including kuna). The campaign metric is
**GED** — CFG graph edit distance between the Joern-parsed original source and the
Joern-parsed decompiled C. Lower is better; **0 = perfect structural recovery**.
`decbench improvements` mines the per-function cases where a base decompiler beats a
target.

Where kuna stands on that run (GED, share of functions scored perfect):
**ida 36.67% · kuna 35.41% · angr 33.93% · ghidra 25.57% · binja 20.76%**. So the
campaign mines **two** reference pools — `--base angr` (the original) and `--base ida`
(the current leader) — plus the NOVEL pool below.

Decisions baked in:

- venv: `~/.virtualenvs/decbench` (angr 9.2.213 pinned, pyjoern, cfgutils, editable
  decbench). Only `rescore`/`srcsizes` need it; `mine`/`novel`/`triage`/`status` run on
  plain python3.
- results tree: `~/github/decbench/results/full_run` (override `KUNA_DECBENCH_RESULTS`).
  The benchmark ran on **stripped** copies with the function's own name relabeled
  post-hoc, so fresh decompiles must use `<opt>/<project>/stripped/<bin>` + self-relabel
  to stay comparable.
- **Verify-first is mandatory, and it is not optional bookkeeping** — see the next
  section for exactly how stale the stored output is.
- GED quirks to respect in triage: graphs > 60 nodes are **approximated** as
  |Δnodes| + |Δedges| (`approximated: true` on a case, taken from the run's own
  `ged_large_graph_audit.json`); `inf` means missing/unparseable, not "bad"; a
  degenerate source CFG (`source_nodes: 1`) means the source function is a single basic
  block — either genuinely straight-line (then extra control flow in kuna's output IS a
  real defect, usually a missed no-return) or a Joern parse failure (then the score is
  meaningless). Read the source before deciding which.

### The stored output is stale in two different ways

1. **Print normalization.** PRs #202–#211 (DIV-34…DIV-39) changed how nearly every line
   renders: `NULL` for zero pointers, compound assignment, truthy conditions, elided
   braces, inline `// slug` warnings. Textual diffs between the stored and a fresh run
   are expected and mean nothing. **Judge structure**: gotos, labels, loop shape, switch
   vs if-cascade, function boundary, missing blocks, wrong types.
2. **Mode presets did not exist when the benchmark ran.** It used shipped defaults plus
   an injected `option listing on` — which is exactly today's `--mode reliable`. Today a
   no-flag run is `--mode auto`: **`aggressive` below 500 KiB** (21 default-off options
   ON), `reliable` below 2 MiB, `fast` at 2 MiB and above (Listing/funcstart/aif OFF,
   10 s per function). So:
   - a fresh no-flag run shows **today's product default**, which is what a user sees;
   - `--mode reliable` shows **the benchmark's option surface**, which is what the score
     measured.
   Run both when they disagree, and say in the triage record which one closed the gap —
   "a code fix landed" and "the default mode got stronger" are different outcomes.
   `condfold` is default-off and is **not** in the aggressive preset, so sweep
   `--option condfold on|wide` explicitly.

## Standing requirements

Rules 1–5 of `docs/improvement-pipeline.md` → *Standing requirements* apply verbatim
(one PR per feature; end-to-end two-pass stage test; output-changing ⇒ option +
history/DIV record; speed always measured via `scripts.pipeline.timeit` with the ≤5%
budget; large work goes through a `[PROPOSAL]` draft PR). Plus:

6. **Every output-changing PR records its benchmark delta.** Run
   `scripts.decbench.rescore --case <case-id> --record docs/features/<slug>/record.json`
   (add `--siblings` when the dedup group has more instances) so the PR's
   `record.json` carries `ged_before` / `ged_after` / `ged_delta` for the exact
   mined case — the GED analog of timeit's speed block. Update the triage record's
   `status:` when a PR closes its case.

Provenance: `source_decompiler = "angr"` stays when angr's behavior is the reference;
set `inspiration = "decbench:<case-id>; <angr pass/class>; <function>"` — the
`decbench:` prefix is what `scripts.decbench.status` keys campaign features off.
Stage tests stay `tests/stages/ghangr-<slug>.xml` for angr-derived fixes;
use `ghdec-<slug>.xml` for fixes with no angr analog.

## The loop

```bash
# 0. SOURCE SIZES — one-time per results tree; decides which scores are edit
#    distances at all (needs the decbench venv; writes ~/.cache/decbench/)
~/.virtualenvs/decbench/bin/python -m scripts.decbench.srcsizes

# 1. MINE — snapshot the benchmark signal into the repo
python3 -m scripts.decbench.mine --select 30              # base angr
python3 -m scripts.decbench.mine --base ida --select 20   # base ida (current leader)
python3 -m scripts.decbench.novel --select 16             # the NOVEL pool
#   -> docs/decbench/cases[-<base>].json    the full pool, bucketed + deduped (gitignored)
#   -> docs/decbench/cases-missing*.json    kuna-failed pool (recall workstream owns it)
#   -> docs/decbench/backlog[-<base>].md    ranked human table (one row per group)
#   -> docs/decbench/novel.{json,md}        kuna-is-best-and-still-bad, ranked by warts
#   -> docs/decbench/triage-queue*.json     the stratified triage picks

# 2. TRIAGE — one agent per queue case; verify-first, then root-cause
python3 -m scripts.decbench.triage --case <case-id> --also ida,ghidra
#   write docs/decbench/triage/<case-id>.md (front-matter schema below)

# 3. CLUSTER — group feature-candidate records by root cause
#   -> docs/decbench/features.md  (the go/no-go menu: slug, cases, coverage, scope)

# 4. FIX — one branch feat/decbench-<slug> per feature, standard worker protocol

# 5. RESCORE — the benchmark gate, spliced into the feature record
~/.virtualenvs/decbench/bin/python -m scripts.decbench.rescore \
    --case <case-id> --record docs/features/<slug>/record.json --siblings

# 6. STATUS — where the campaign stands
python3 -m scripts.decbench.status
```

### Triage buckets (in `cases.json`, computed from cross-decompiler consensus)

| bucket | meaning | action |
|---|---|---|
| `kuna-specific` | kuna GED > ghidra + 2 — regression vs kuna's own ancestor | fix first, most tractable |
| `inherited` | kuna ≈ ghidra — Ghidra-family-wide gap vs the base | SAILR-style structuring features |
| `ahead` | kuna already beats ghidra, still behind the base | genuine remaining distance |
| `artifact_suspect` | kuna ≤ best(ghidra, ida, binja) + 1 while only the base = 0, or the source CFG is degenerate | verify truncation / parse failure; route to a decbench fix |

### The three pools

| pool | question it answers | miner |
|---|---|---|
| `angr` | where does angr's structuring beat ours | `mine.py` (default base) |
| `ida` | where does the current scoreboard leader beat ours | `mine.py --base ida` |
| `novel` | where is kuna already the best and the output is **still bad** | `novel.py` |

The NOVEL pool exists because a metric win says nothing about whether the pane reads
well, and because no rival holds the answer there — the fix has to be invented. It
ranks by warts in kuna's own emission (gotos, `(code *)` casts, raw register names,
`CONCAT`/`SUBPIECE` soup, `halt_baddata`), not by GED, and excludes O2 by default
because inlining detaches the decompiled CFG from the source function. **A fix that
comes out of this pool is tagged `(NOVEL)` in its commit subject** so the provenance is
unambiguous: no other decompiler showed us the answer.

### The recall pool (`cases-missing*.json`) is not a triage pool — it is the biggest one

Functions kuna produced **no output for at all** while a rival did. On the 2026-07-27
run that is **6,540 functions (7.3% of the corpus)**, every one of them a guaranteed
zero. Two named causes found so far: whole-project discovery failures (u-boot 100%,
freertos 100%, betaflight 22% — embedded/ARM), and per-function pipeline failures on
x86-64 (coreutils `main` in comm/join/timeout/uniq emits an empty body:
`structured blocks unavailable (structuring declined at a stub)`, and the same function
panics under `decompile-all`). Recall work outranks structuring work per function
recovered — check this pool before mining another structuring case.

### Per-case procedure (the triage agent)

1. `triage --case <id> --also ida,ghidra` — read every pane side by side.
2. Gap gone on fresh kuna? → `status: already-fixed` (name the PR; rescore to confirm).
   Say whether `--mode reliable` also closes it (a code fix) or only the no-flag run
   does (the aggressive default closed it).
3. Sweep relevant default-OFF options (`kuna catalog --json`, `use_when` match;
   `triage --case <id> --option <name> on`). One closes it? → `status: covered-by-option`
   (a default-flip candidate, not a new feature). `--mode aggressive` already carries 21
   of the 24 default-off options; `condfold` is not one of them.
4. Pin ONE dominant structural difference (gotos into shared tails, missing switch,
   loop shape, boundary overrun, …).
5. Metric-artifact check: approximated large CFGs, Joern parse failures on kuna's
   emission style, degenerate source CFG, truncated angr output scoring 0.
   → `status: metric-artifact` (sub-reason) or a decbench-side bug note.
6. Read the ORIGINAL SOURCE — it is what "correct structure" means, and it is in the
   results tree: `grep -l '<fn>' ~/github/decbench/results/full_run/O0/<project>/compiled/*.i`.
   Judge every pane against it, not against the reference's pane.
7. Root-cause in the reference: for angr, the venv site-packages (9.2.213 — the version
   that ran) is ground truth, `~/github/angr-dev/angr` master for readability; name the
   pass/class. For the ida and novel pools there is no readable reference — root-cause
   in kuna alone.
8. Owning kuna phase from `docs/phases.md`.
9. Scope: small option-gated pass vs `[PROPOSAL]`.
10. Check siblings reproduce the same symptom.
11. Write `docs/decbench/triage/<case-id>.md`:

```markdown
---
case_id: <id>
pool: angr | ida | novel
group_id: <project>::<function>
status: feature-candidate | covered-by-option | already-fixed | metric-artifact | needs-proposal | load-failure
tier: L|M|S|A|N
margin: <n>
fresh_verdict: <one line: what today's build actually does>
option_closing: <name or null>
feature_slug: <proposed or null>
scope: small | proposal
confidence: high | medium | low
---
## Side-by-side      (reference / stored kuna / fresh kuna, fenced)
## Source            (the original function from the .i)
## Analysis          (symptom, root cause, why the reference wins; kuna phase)
## Proposed fix      (mechanism, owning files, risks)
```

A verdict of `feature-candidate` requires ALL of: reproduced on today's build, one
named structural symptom, one named phase, and a mechanism that fits in one module.
Without the mechanism the honest status is `needs-proposal`. A disproven case is worth
more than an invented feature.

## Finding good kuna examples

The loop above mines kuna's **losses**. The same results tree also answers the mirror
question — *where does kuna already beat everyone?* — which is what stocks the landing
page's compare section (`integrations/web/compare-samples.js`, rendered at
[kuna.noelo.org](https://kuna.noelo.org) → *compare*). Ask an agent to **"find good kuna
examples"** and this is the procedure it runs.

```bash
# 1. MINE — optimized builds, medium functions, kuna beats ida and nobody beats kuna
python3 -m scripts.decbench.showcase --perfect --limit 40 \
        --dump /tmp/showcase-cands
#   --rival ghidra|binja|angr   compare against a different decompiler
#   --no-sweep                  accept beating --rival only (default: beat all four)
#   --perfect                   require kuna GED == 0 (structurally exact)
#   --min-size/--max-size       pane fit, default 22..100 decompiled lines
#   -> /tmp/showcase-cands/<case-id>/{kuna,ida,ghidra,binja,angr,source}.c + meta.json

# 2. VERIFY — does the recorded pane still reproduce on today's build?
python3 -m scripts.decbench.showcase --perfect --limit 40 \
        --dump /tmp/showcase-cands --verify        # nonzero exit on any drift

# 3. JUDGE — this is the step that decides. Never ship a mined candidate unread.
# 4. EMIT — render the picks straight out of the bundles, no retyping
python3 -m scripts.decbench.showcase --dump /tmp/showcase-cands --emit picks.json
#   picks.json = [{"case_id": "..."}, ...]   — `name` (the dropdown label) and
#   `meta` (the provenance line) are generated; override only to disambiguate.
```

The page carries **no per-sample commentary**, by design: the dropdown label is a
neutral identifier (`fn() — project binary, arch`), the line under it is provenance,
and the only claim made is the measured GED for the pair on screen. A caption telling
the reader what to notice would be the one thing there that is not machine-derived —
the panes are the argument. Keep the emitter's output free of prose.

**Step 3 is the whole job.** GED is a *filter*, not a verdict: roughly half of decbench's
cross-decompiler gaps are scoring artifacts (see *Caveats*), and a metric win says
nothing about whether kuna's pane is pleasant to read. So fan out one reviewer per
handful of candidates — each reads **all six panes in full** — then run an **adversarial
second pass** whose brief is *"this sample is about to embarrass kuna on its own front
page; find the reason"*. Both passes were what turned an 89-candidate mined pool into
the shipped samples; the first pass rejected roughly two thirds, and the skeptics killed
half of what survived.

Reject a candidate for any of:

- **Semantic wrongness** — kuna's body disagrees with `source.c` (dropped call, inverted
  test, a value read before the call that produces it). A pretty but wrong pane is fatal.
- **A rival that simply reads better.** The visitor can flip the dropdown; if Ghidra or
  angr wins that pane, the sample backfires. This is why the miner defaults to `--sweep`.
- **An invisible GED win** — kuna scores lower only because it *dropped* code the rivals
  recovered (compare pane lengths, then read for the missing block).
- **Constants kuna alone fails to resolve** — a bare `0x7f373` in a `printf` format slot,
  or a `dat_XXXX` where all four rivals print `stderr`. One or two is normal on a stripped
  binary; a pattern of it while every rival resolves is a loss the visitor sees.
- Raw register names as variables, leaked uniques, `halt_baddata`, a truncated decode.
- A trivial three-line wrapper, or a source pane that is macro soup (`source_status:
  "preprocessed"` in `meta.json` — the `.c` was unavailable and the `.i` was used).

Explicitly **not** a defect: kuna's own `/* WARNING: <pass>: ... */` transform banners
(`branchflip`, `taildup`, `earlyreturn`, `tailcalljump`, …). They are documented,
default-on provenance annotations (`docs/options.md`) — judge the code, not them.

Provenance the page must state correctly: decbench decompiled the **stripped** binary
with every decompiler, then renamed each function's own `sub_<addr>` placeholder to its
DWARF name (`_relabel_to_dwarf`). Callees stay `sub_`/`dat_` in every pane. That rename
is the *only* edit — bodies are verbatim tool output, which `--verify` re-checks against
the current build before anything ships.

## Refreshing the signal

After a future benchmark re-run: point `KUNA_DECBENCH_RESULTS` at the new tree, re-run
`srcsizes` (it reads that tree's `ged_src/` + `ged_large_graph_audit.json`), then
`mine`/`novel` — a new snapshot supersedes the old; existing triage records keep their
case ids. The mined pool JSONs are gitignored (tens of MB, regenerable); the ranked
tables, the triage queues and the triage records are committed.

## Caveats

- The Jun-29 run-binary skew: always trust the fresh decompile, not the stored one.
- `~/.cache/decbench/metric` and `~/.cache/decbench/kuna-src-cfg` are
  content-addressed — fresh inputs always recompute; never stale, safe to delete.
- The decbench working tree carries user WIP: campaign tooling only READS it; any
  decbench fix goes through a fresh branch off `origin/main` staging only named files.
- Joern is JVM-heavy: cap concurrent rescores (~8).
