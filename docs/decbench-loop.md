# The decbench improvement loop

The decbench-driven continuation of the angr feature campaign (`docs/improvement-pipeline.md`).
The signal source changes — a real-binary benchmark instead of angr's testcase
corpus — but the feature factory downstream is identical: one PR per feature,
option-gated, default-on when clean, measured.

## The signal

[decbench](https://github.com/mahaloz/decbench) scores decompilers per function on a
compiled-project corpus (**94,575 functions / 803 binaries / 39 projects ×
{O0, O2, O2-noinline}**, eleven decompilers including kuna). The campaign metric is
**GED** — CFG graph edit distance between the Joern-parsed original source and the
Joern-parsed decompiled C. Lower is better; **0 = perfect structural recovery**.
`decbench improvements` mines the per-function cases where a base decompiler beats a
target.

Where kuna stands after the **2026-08-03** refresh (share of functions scored perfect,
from `~/github/decbench/results/full_run/scoreboard.toml`):

| metric | scored | standings |
|---|---:|---|
| **GED** | 91,072 | ida 36.06% · **kuna 35.93%** · angr 34.20% · ghidra 25.13% · binja 20.42% |
| **overall** (all three metrics) | 94,267 | ida 38.01% · **kuna 37.91%** · angr 36.98% · ghidra 27.07% · binja 23.26% |
| **byte_match** | 90,890 | **kuna 3.41% (1st)** · ida 1.77% · angr 0.53% · ghidra 0.48% · binja 0.20% |

**Treat this tree as a mining instrument, not a publishable cross-tool leaderboard.**
kuna's column is current — re-decompiled and re-scored on 2026-08-03 from a pinned build.
Every rival's column is whatever its own last run left behind, weeks earlier. The columns
are therefore not contemporaneous, and kuna's byte_match is additionally *under*-counted on
ARM/PE at two of three optimisation levels (see *Refreshing the signal*). Use the numbers to
find cases; do not quote them as a measured cross-tool result without re-running every
decompiler on one tree.

So the campaign mines **two** reference pools — `--base angr` (the original) and
`--base ida` (the current GED leader) — plus the NOVEL pool below.

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
- **PE addresses: the pools carry RVAs, kuna wants VAs.** decbench records a PE
  function's address as an RVA (`function_results.json`, and every rival's
  `// Function: <fn> @ 0x..` marker — only kuna's own artifact writes VAs), so a mined
  case may hold either form. `triage`/`rescore`/`showcase` rebase it off the binary's
  own PE `ImageBase` before calling `kuna --addr` (`config.kuna_addr`), which differs
  per binary — `mydoom.exe`/`x0r-usb.exe` 0x400000, `dexter.dll` 0x69940000. ELF
  addresses are already VAs and pass through untouched.
- GED quirks to respect in triage: graphs > 60 nodes are **approximated** as
  |Δnodes| + |Δedges| (`approximated: true` on a case, taken from the run's own
  `ged_large_graph_audit.json`); `inf` means missing/unparseable, not "bad"; a
  degenerate source CFG (`source_nodes: 1`) means the source function is a single basic
  block — either genuinely straight-line (then extra control flow in kuna's output IS a
  real defect, usually a missed no-return) or a Joern parse failure (then the score is
  meaningless). Read the source before deciding which.

### The stored output is stale in three different ways

1. **Print normalization.** PRs #202–#211 (DIV-34…DIV-39) changed how nearly every line
   renders: `NULL` for zero pointers, compound assignment, truthy conditions, elided
   braces, inline `// slug` warnings. Textual diffs between the stored and a fresh run
   are expected and mean nothing. **Judge structure**: gotos, labels, loop shape, switch
   vs if-cascade, function boundary, missing blocks, wrong types.
2. **Mode presets did not exist when the benchmark ran.** It used shipped defaults plus
   an injected `option listing on` — which is exactly today's `--mode reliable`. Today a
   no-flag run is `--mode auto`, which picks from the input's byte length: `aggressive`
   below 500 KiB, `reliable` up to 2 MiB, `fast` at 2 MiB and above.
   **On this corpus `fast` never happens.** The largest stripped benchmark binary is O0
   `bash` at 1,513,904 bytes (1.44 MiB), so across all 803 binaries `auto` resolves to
   **768 aggressive / 35 reliable / 0 fast**. The 35 are every `bash`, `sshd`, `ssh`,
   `ssh-keygen`, `ssh-keysign`, `ssh-keyscan`, `ssh-add`, `ssh-agent`, `rsyslogd`,
   `e2fsck`, `ip`, `u-boot` and `tar` build. So:
   - a fresh no-flag run shows **today's product default**, which is what a user sees;
   - `--mode reliable` shows **the benchmark's option surface**, which is what the score
     measured;
   - on the 35 `reliable` binaries those are the same thing, so a disagreement there is a
     code change, never a mode change. On the other 768 they differ by the whole
     `aggressive` preset.
   Run both when they disagree, and say in the triage record which one closed the gap —
   "a code fix landed" and "the default mode got stronger" are different outcomes.
   `aggressive` applies 21 overrides, covering 20 of the 26 options that ship default-off;
   `condfold`, `cortexmvectors`, `ptrentry`, `paramcopyhoist`, `dwarf_lines` and
   `v850indirectbranch` are the six it does not carry, so sweep those explicitly
   (`--option condfold on|wide`). **Recount rather than copy this** — the queue adds
   options faster than the prose tracks them.
3. **The stored kuna column is a snapshot of one pinned build, and it is never HEAD.** The
   2026-08-03 refresh re-decompiled all three optimisation levels from **kuna 9623dc27**
   (`~/decbench-campaign-state/stable/PINNED_KUNA_COMMIT.txt`), which closed the worst case
   of this — round 2 opened with the O0 pool ~100 commits behind, older than the
   2026-07-27 date the docs then cited, and one whole track resolved to `already-fixed`
   purely because of it. It does not remove the rule: **verify on today's build before any
   finding counts**, and expect everything merged after the pinned commit to be missing
   from every stored pane.

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
7. **The refute brief asks "would this produce WRONG output?", not just "would it produce
   ANY output?"** — see below.
8. **Diff every changed function, not the witness** — see below.
9. **Derive every shared counter from a green build and run, never by arithmetic** — see
   *Shared counters in a busy queue* below. A rebase in this queue is not a merge problem,
   it is a measurement problem.

### A symptom is evidence; a diagnosis is a hypothesis

Round 1 filed five mechanisms and **two of them were right**. Round 2's refuters
**overturned the filed diagnosis on three of eight** — and in every one of those eight the
*symptom* stood. That ratio is the campaign's most reliable finding: the pane is data, the
root cause in the triage record is a guess until someone instruments it. Two rules follow,
and both were paid for in this round.

**"Not a no-op" is not the same as "correct."** The adversarial-refute step asks whether a
proposed mechanism would actually change the output — a useful question, and not the
dangerous one. On `paramcopyhoist` (#250) a refuter proved the proposed `Merge::trimOpInput`
hook did reach the printer, and the hook was **still the wrong site**: implementing it there
silently deleted an assignment (`e2fsck ext2fs_read_inode2` lost `v10 = a3;`, leaving an
empty `if` body and a `memcpy` reading a value that is never assigned), a second design lost
an early return in `sub_544cc`, and a third made `sub_7431a` return the wrong parameter
unconditionally. All three *fired*. So the brief needs an explicit **would this produce
wrong output?** axis alongside the reachability one — and the answer to it is a measurement,
not an argument.

**Diff every changed function, not the witness.** Each of those wrong designs **passed on
its witness** and failed only under a whole-corpus before/after sweep. For any pass that
moves, deletes or re-anchors statements, a sweep across **all** changed functions is a
standing expectation, not a nicety. Round 2's four merged output-changing PRs did this at
3,031 (#250), 12,241 (#253), 14,720 (#251) and 15,386 (#252) functions; two of the four
caught real problems that way — #250's three wrong designs, and #251's two functions in
14,720 that gain a dead partial-register shift. Automate the audit where the change has a
shape (`docs/features/itecondlist/audit.py` classifies every hunk and leaves only the
unclassifiable ones for a human).

### Shared counters in a busy queue — the merge is not the hard part

This queue ran up to four output-changing PRs concurrently, and every one of them touches
the same handful of hard-coded counters (`catalog_bytecompat.rs`, `phase_catalog.json`,
`kuna-base/src/xml.rs`' corpus count, `docs/baseline-stages.json`, the `kuna_phases`
count asserts, `tests/stages/README.md`, the DIV number in `docs/history.md`). Round 2
produced **three distinct failure shapes, and only one of them announced itself**:

| shape | what happened | why it is dangerous |
|---|---|---|
| **A conflict** | the DIV number for #257 raced from **55 → 56 → 57 → 58** as #252/#253/#254 claimed each in turn | the loud case — git stops you, and `docs/history.md` conflicts are obvious |
| **A silent identical-edit auto-merge** | `catalog_bytecompat.rs` kept `86` because **both sides had made the identical `85 → 86` edit**, so git merged cleanly and the count was one short (#254) | no conflict, no diff to review, wrong answer — caught only by running the suite |
| **A silent keep-both auto-merge** | a `docs/baseline-stages.json` auto-merge left a stale `data_footer` of `375` while the real key count was `381` (#253); and five rounds of keep-both resolution **duplicated a row in `tests/stages/README.md`** | keep-both is git's safe default and is exactly wrong for a counter or a table |

The rules that fall out, and that #255 then executed cleanly:

- **Derive every shared counter from a fresh capture and a green run on the rebased tree,
  never by arithmetic.** "Base + mine + theirs" is how you get `86` when the answer is
  `87`. Re-capture `phase_catalog.json`, re-record `docs/baseline-stages.json`
  (`kuna test --datatests --datatests-dir tests/stages --save-baseline …`), re-count the
  corpus files, regenerate `docs/options.md`. #255 verified even the byte-identical
  outcomes rather than assuming them.
- **Diff every keep-both resolution against `main` and assert you removed nothing *and*
  added nothing twice.** A duplicated table row survives every gate in this repo.
- **DIV numbers are claimed on merge, not on branch.** Re-check the number at merge and
  renumber the row plus every reference to it — `docs/history.md`, the option's
  `use_when` prose, the spec chapter, the PR body.
- Nothing here is caught by review of the *conflict*. It is caught by the build and the
  suite, so the gates must be re-run **after** the rebase, not before it.

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

| pool | question it answers | miner | size (2026-08-03) |
|---|---|---|---|
| `angr` | where does angr's structuring beat ours | `mine.py` (default base) | 3,596 cases / 1,813 groups |
| `ida` | where does the current scoreboard leader beat ours | `mine.py --base ida` | 6,057 cases / 2,687 groups |
| `novel` | where is kuna already the best and the output is **still bad** | `novel.py` | 4,842 cases / 2,301 groups |

The NOVEL pool exists because a metric win says nothing about whether the pane reads
well, and because no rival holds the answer there — the fix has to be invented. It
ranks by warts in kuna's own emission (gotos, `(code *)` casts, raw register names,
`CONCAT`/`SUBPIECE` soup, `halt_baddata`), not by GED, and excludes O2 by default
because inlining detaches the decompiled CFG from the source function. **A fix that
comes out of this pool is tagged `(NOVEL)` in its commit subject** so the provenance is
unambiguous: no other decompiler showed us the answer.

### The recall pool (`cases-missing*.json`) is not a triage pool — check it first anyway

Functions the base scored and kuna produced **no usable GED for at all**, every one of them
a guaranteed zero. On the 2026-08-03 snapshot that is **1,420 functions against
`--base angr`** (966 of them base-perfect) and **1,552 against `--base ida`**, with
**0 whole-binary failures**. It is now almost entirely embedded ARM Cortex-M:

| project | angr-base pool | ida-base pool |
|---|---:|---:|
| betaflight | 575 | 591 |
| crazyflie | 392 | 362 |
| cleanflight | 247 | 276 |
| chibios | 74 | 94 |
| nuttx | 67 | 78 |
| libopencm3 | 28 | 97 |
| everything else (x0r-usb, dexter, mydoom, libbsd, riot-os) | 37 | 54 |

Recall work outranks structuring work per function recovered — check this pool before
mining another structuring case. The three causes this file used to name are all resolved
or were never right, and none of them should be repeated:

- **"u-boot 100%, freertos 100%" was a decbench artifact, not a kuna discovery failure.**
  Neither project appears in either pool, and not because kuna improved: **angr and ida
  score nothing at all there**, so the pool cannot contain them. decbench's raw backends
  keep only functions inside `.text`, u-boot's `.text` is 936 bytes and freertos is built
  `-ffunction-sections`. Measured past the filter, kuna covers 261 of freertos's 265 and
  1,833 of u-boot's 2,968 functions (`docs/decbench/recall-measurement.md`).
- **"betaflight 22%" is stale.** betaflight is still the single largest contributor at 575
  rows, but that is 6.0% of the 11,070 betaflight functions any non-LLM rival scored.
- **The coreutils `main` failure is fixed.** `comm`/`join`/`timeout`/`uniq` `main`
  decompile to full bodies (97 / 230 / 134 / 176 lines) at their DWARF addresses on
  today's build, `decompile-all` on `comm` exits 0, and zero coreutils rows remain in
  either pool.

The measured breakdown of what is left — 98.3% of it function-entry granularity on
embedded ARM, median 24 bytes from the containing kuna entry — is
`docs/decbench/recall-measurement.md`. The approved four-step sequence against it is
**[PROPOSAL] PR #239**: step 1 `cortexmvectors` (#248) and step 2 `ptrentry` (#255) are
merged and take Cortex-M recall to **96.3%**, step 3 `tailcallentry` is in flight, step 4
(TBB/TBH) is not started. **The live sequence table with per-step measurements is a comment
on #239** — read it there; it is not duplicated in this repo, so it cannot go stale here.

### Per-case procedure (the triage agent)

1. `triage --case <id> --also ida,ghidra` — read every pane side by side.
2. Gap gone on fresh kuna? → `status: already-fixed` (name the PR; rescore to confirm).
   Say whether `--mode reliable` also closes it (a code fix) or only the no-flag run
   does (the aggressive default closed it).
3. Sweep relevant default-OFF options (`kuna catalog --json`, `use_when` match;
   `triage --case <id> --option <name> on`). One closes it? → `status: covered-by-option`
   (a default-flip candidate, not a new feature). `--mode aggressive` already carries 20
   of the 26 default-off options; `condfold`, `cortexmvectors`, `ptrentry`,
   `paramcopyhoist`, `dwarf_lines` and `v850indirectbranch` are not among them.
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

Refreshing **kuna's own column in place** (re-decompile + re-score just kuna, leaving the
rivals frozen) is a different and much more dangerous operation. The working recipe lives
outside this repo, at `~/decbench-campaign-state/`, because it carries multi-hundred-MB
pinned inputs:

| path | what |
|---|---|
| `o0-refresh/resume_from_ged.sh` | the driver — GED, byte_match, type_match, finalize |
| `o0-refresh/make_baseline_all.py` | builds the GED comparison baseline |
| `o0-refresh/restore_kuna_bytematch.py` | repairs the byte_match trap below |
| `o0-refresh/STATE.md` | what the last run did, and what it broke |
| `ged-baseline-20260727/` | the pinned published GED inputs the rivals' scores are bound to |
| `stable/kuna` + `stable/specs` | a snapshotted kuna binary and specs |

Two pinning rules it enforces, both by aborting:

- **decbench must not move off its pinned commit** (`PINNED_DECBENCH_STATE.txt`). The
  rivals' frozen GED checkpoints were produced by that evaluator; a different one silently
  re-defines the metric under them.
- **The refresh must run a snapshotted kuna binary and specs** (`KUNA_BIN`/`KUNA_SPECS`
  → `stable/`), so a concurrent `make binaries` in the main tree cannot swap the
  decompiler mid-run. A refreshed column that is half one build and half another is worse
  than a stale one.

And one baseline rule that is not obvious and cost a whole run:

> **The GED baseline must drop kuna's published scores for EVERY optimisation level that
> has been re-decompiled — not just the one you are refreshing.** `reeval_ged.eval_one`
> audits by replaying the *published* evaluator against the *on-disk* artifact and
> asserting it reproduces the published score. That assertion is only meaningful while the
> artifact is unchanged, so offering a July score as evidence for an August artifact kills
> the run: an O0-only baseline died on an O2 slice with
> `cannot reproduce published fallback 58.0: reconstructed=54`. Dropping all three costs
> nothing — the new score is computed independently.

### Trap 1: byte_match recompiles, and this machine has no cross toolchains

`reeval_bytematch.py` reuses any checkpoint that **exists** — checkpoints are never
auto-invalidated — so refreshing a decompiler's byte_match requires deleting its
checkpoints first. But byte_match **recompiles** the decompiled C with an
architecture-matched toolchain (`decbench/metrics/byte_match.py:267` — PE → MinGW,
ARM → `arm-none-eabi`, x86 → gcc). **This machine has only `gcc`**: no
`arm-none-eabi-gcc`, no MinGW. For every ARM and PE binary the metric returns
`{"skipped": True}`, the re-scorer drops it, and an **empty** checkpoint is written.

So deleting kuna's checkpoints **destroys kuna's ARM/PE byte_match while every rival keeps
theirs** — an asymmetric corruption that makes kuna look like it compiles nothing on
embedded targets. `finalize_results.py`'s coverage guard caught it (34 slices, every one
kuna at O0 on ARM/PE: betaflight 2,848 → 0, crazyflie 2,128 → 0, cleanflight 1,967 → 0)
and `restore_kuna_bytematch.py --apply` repaired them from
`pre-o0-refresh-backup/function_results.json`.

**kuna's ARM/PE byte_match at O2 and O2-noinline is still missing.** An earlier session hit
the same trap and published it with `DECBENCH_ALLOW_DROPS=1`; no local copy of those values
survives, and the 2026-07-27 published snapshot does not carry them either. kuna leads
byte_match anyway — with two of three optimisation levels under-counted on embedded targets.

> **Never pass `DECBENCH_ALLOW_DROPS=1` without reading every `[guard]` line.** The guard is
> the only thing standing between a toolchain gap and a published regression. Install the
> cross toolchains, or leave byte_match checkpoints alone on this box. (A legitimate drop
> block does exist: ~179 shrinking counters from `phoenix`, a retired decompiler still on
> disk. Any **non-phoenix** guard line means stop.)

### Trap 2: `rescore` cannot touch a PE case

The mined pools store a PE function's address as an **RVA** where kuna uses the **VA**, and
`scripts.decbench.rescore` passes `case["address_hex"]` straight to
`kuna decompile-all --addr`. Every PE case therefore fails out of the box:

```
$ kuna decompile ...O0/mydoom/stripped/mydoom.exe --addr 0x41e2     # the pool's address
error: no C output for "0x41e2"
$ kuna decompile ...O0/mydoom/stripped/mydoom.exe --addr 0x4041e2   # ImageBase + RVA
void sub_4041e2(int4 a0) { ...
```

This silently blocks PE rescoring campaign-wide: **119 PE cases** across the three current
pools (36 angr, 48 ida, 35 novel) in mydoom, minipig, x0r-usb and dexter. The same skew
fabricated 181 fake gaps in the recall measurement until it was corrected by hand
(`docs/decbench/recall-measurement.md`).

**Independently re-confirmed by #254 after the results-tree refresh and the pool re-mine** —
`address_hex` is still the RVA, so the refresh did not fix it — on the PR that moves exactly
the half of the corpus this blocks. That PR took its GED numbers via a throwaway local
address fix-up and reverted it, leaving `docs/decbench/triage-queue*.json` unmodified; do
the same rather than editing a pool by hand, because the pools are a snapshot of what
decbench recorded and a hand edit will be silently reverted by the next `mine`.

Known limitation, not yet fixed. Suggested direction: have `rescore` resolve the image base
from the loaded binary (kuna already reports absolute addresses in `functions --json`) and
add the delta when the pool row's address falls outside the loaded range — a per-case fix in
`scripts/decbench/rescore.py`, not a miner change.

## Caveats

- The Jun-29 run-binary skew: always trust the fresh decompile, not the stored one.
- `~/.cache/decbench/metric` and `~/.cache/decbench/kuna-src-cfg` are
  content-addressed — fresh inputs always recompute; never stale, safe to delete.
- The decbench working tree carries user WIP: campaign tooling only READS it; any
  decbench fix goes through a fresh branch off `origin/main` staging only named files.
  It must also stay on its **pinned commit** while the rivals' frozen checkpoints are in
  play (*Refreshing the signal*).
- Joern is JVM-heavy: cap concurrent rescores (~8) — the last refresh was killed at 8
  workers when the box fell to 3 GB available RAM alongside concurrent cargo builds.
- `rescore` cannot touch a **PE** case (RVA-vs-VA), and byte_match cannot be recomputed on
  this box for ARM or PE (no cross toolchains). Both are *Refreshing the signal*.
