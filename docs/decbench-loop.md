# The decbench improvement loop

The decbench-driven continuation of the angr feature campaign (`docs/pipeline.md`).
The signal source changes — a real-binary benchmark instead of angr's testcase
corpus — but the feature factory downstream is identical: one PR per feature,
option-gated, default-on when clean, measured.

## The signal

[decbench](https://github.com/mahaloz/decbench) scores decompilers per function on a
compiled-project corpus (full run: **110,992 functions / 806 binaries / 40 projects ×
{O0, O2, O2-noinline}**, six decompilers including kuna). The campaign metric is
**GED** — CFG graph edit distance between the Joern-parsed original source and the
Joern-parsed decompiled C. Lower is better; **0 = perfect structural recovery**.
`decbench improvements` mines the per-function cases where a base decompiler beats a
target; our pool is **angr GED = 0 while kuna GED > 0** (`--perfect-only`).

Decisions baked in:

- venv: `~/.virtualenvs/decbench` (angr 9.2.213 pinned, pyjoern, cfgutils, editable
  decbench). Only `rescore` needs it; `mine`/`triage`/`status` run on plain python3.
- results tree: `~/github/decbench/results/full_run` (override `KUNA_DECBENCH_RESULTS`).
  The benchmark ran kuna 0.1.0 (2026-06-29) on **stripped** copies of the binaries with
  the function's own name relabeled post-hoc — so **verify-first is mandatory**
  (a case may already be fixed on main) and fresh decompiles must use
  `<opt>/<project>/stripped/<bin>` + self-relabel to stay comparable.
- GED quirks to respect in triage: graphs > 60 nodes are **approximated** as
  |Δnodes| + |Δedges|; `inf` means missing/unparseable, not "bad"; a degenerate
  source CFG (`source_nodes: 1`) makes the score meaningless — that is a decbench
  bug, not a kuna gap (`bucket`/`artifact_suspect` in `cases.json` flag these).

## Standing requirements

Rules 1–5 of `docs/pipeline.md` → *Standing requirements* apply verbatim
(one PR per feature; end-to-end two-pass stage test; output-changing ⇒ option +
PROGRESS/DIV; speed always measured via `scripts.pipeline.timeit` with the ≤5%
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
# 1. MINE — snapshot the benchmark signal into the repo (commit the outputs)
python3 -m scripts.decbench.mine --select 28
#   -> docs/decbench/cases.json          the full pool, bucketed + deduped
#   -> docs/decbench/cases-missing.json  kuna-failed pool (hang workstream owns it)
#   -> docs/decbench/backlog.md          ranked human table (one row per group)
#   -> docs/decbench/triage-queue.json   the stratified triage picks

# 2. TRIAGE — one agent per queue case; verify-first, then root-cause
python3 -m scripts.decbench.triage --case <case-id>          # the full dump
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
| `inherited` | kuna ≈ ghidra — Ghidra-family-wide gap vs angr | SAILR-style structuring features |
| `ahead` | kuna already beats ghidra, still behind angr | genuine remaining distance |
| `artifact_suspect` | kuna ≤ best(ghidra, ida, binja) + 1 while only angr = 0 | verify angr truncation / degenerate source CFG; route to a decbench fix |

### Per-case procedure (the triage agent)

1. `triage --case <id>` — read angr/stored-kuna/fresh-kuna side by side.
2. Gap gone on fresh kuna? → `status: already-fixed` (name the PR; rescore to confirm).
3. Sweep relevant default-OFF options (`kuna catalog --json`, `use_when` match;
   `triage --case <id> --option <name> on`). One closes it? → `status: covered-by-option`
   (a default-flip candidate, not a new feature).
4. Pin ONE dominant structural difference (gotos into shared tails, missing switch,
   loop shape, boundary overrun, …).
5. Metric-artifact check: approximated large CFGs, Joern parse failures on kuna's
   emission style, degenerate source CFG, truncated angr output scoring 0.
   → `status: metric-artifact` (sub-reason) or a decbench-side bug note.
6. Root-cause in angr: the venv site-packages (9.2.213 — the version that ran) is
   ground truth; `~/github/angr-dev/angr` master for readability. Name the
   pass/class.
7. Owning kuna stage from `docs/phases.md` + `docs/history/stage-mapping.md`.
8. Scope: small option-gated pass vs `[PROPOSAL]`.
9. Check siblings reproduce the same symptom.
10. Write `docs/decbench/triage/<case-id>.md`:

```markdown
---
case_id: <id>
group_id: <project>::<function>
status: feature-candidate | covered-by-option | already-fixed | metric-artifact | needs-proposal | load-failure
tier: L|M|S|A
margin: <n>
fresh_ged: <n or null>
option_closing: <name or null>
feature_slug: <proposed or null>
scope: small | proposal
---
## Side-by-side      (angr / stored kuna / fresh kuna, fenced)
## Analysis          (symptom, root cause, why angr wins; angr class + kuna stage)
## Proposed fix      (mechanism, owning files, risks)
```

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
#   picks.json = [{"case_id": "...", "name": "...", "meta": "...", "note": "..."}, ...]
```

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

After a future benchmark re-run (e.g. post hang-fix): point `KUNA_DECBENCH_RESULTS`
at the new tree and re-run `mine` — a new snapshot supersedes the old; existing
triage records keep their case ids. The kuna-missing pool (`cases-missing.json`)
is owned by the run-failure workstream (see `tests/hang-repro/`), never by case
triage.

## Caveats

- The Jun-29 run-binary skew: always trust the fresh decompile, not the stored one.
- `~/.cache/decbench/metric` and `~/.cache/decbench/kuna-src-cfg` are
  content-addressed — fresh inputs always recompute; never stale, safe to delete.
- The decbench working tree carries user WIP: campaign tooling only READS it; any
  decbench fix goes through a fresh branch off `origin/main` staging only named files.
- Joern is JVM-heavy: cap concurrent rescores (~8).
