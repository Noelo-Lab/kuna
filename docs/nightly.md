# The nightly run

`.github/workflows/nightly.yml` runs once a night on `main` (07:17 UTC, after the
release) and answers three questions about the day's merges:

1. **Do all the gates still pass?** The `tests` job calls `tests.yml` whole, so the
   nightly runs exactly what a push to `main` runs, workspace suite included.
2. **Did kuna's decbench scores move?** The `decbench` job decompiles a fixed sample of
   the published [decbench dataset](https://huggingface.co/datasets/noelo-lab/decbench-dataset),
   scores it with decbench's own code, and compares every function with the night before.
3. **Did kuna get slower?** The same run times every decompile, against last night's
   build on the same runner.

The `report` job puts the verdicts side by side. The full report is the `decbench
sample` job's summary on the run page; the per-function data is the run's
`decbench-nightly` artifact.

## What is measured

**The sample.** `scripts/nightly/dataset.py` reads the `optimized` config (every
O2-noinline function: 257 binaries, 35,399 functions) at a pinned dataset revision and
picks 96 binaries, project by project: each of the 40 projects contributes a binary
before any contributes a second, in an order fixed by the seed. Every function the
config scores in a picked binary is scored, which is 26,596 functions (75% of the
config) for 48% of its decompile time. Only the picked binaries and their source CFGs
are downloaded (~190 files); the dataset ships no build, so nothing is compiled.

**Fidelity to the leaderboard.** `scripts/nightly/bench.py` scores kuna the way
decbench's own benchmark driver does: kuna gets a `strip --strip-all` copy,
`kuna decompile-all <stripped> --json --max-fn-seconds 600` in its default mode, and
the output goes through decbench's kuna backend, its `_relabel_to_dwarf`, and its GED
and type_match metrics, loaded from a pinned decbench checkout rather than
reimplemented. GED compares against the dataset's published source CFGs. byte_match
is not run: it recompiles every function with the original cross toolchains.

Checked against the leaderboard: the harness run with kuna v1.515, the build the
published kuna column was scored from, over the whole `optimized` config reproduces
99.6% of the published per-function GED values exactly (31,858 of 31,986) and 97.2%
of the type_match values, and both perfect counts land within 0.3% of the published
ones. Most of the GED residue is in the dataset's exported source CFGs, not the
harness: a few binaries carry another translation unit's `main` (`coreutils/[` has
`ls`'s 312-node `main`). The residue is fixed by the pins, so it never shows up as a
change between two nights.

**Pinned inputs.** The dataset revision, the decbench commit, and the Rust Joern commit
are pinned in the workflow's `env`, so a change between two nights is kuna's. Bumping a
pin (or the sample size or seed) changes the comparison's identity; that night's report
says the scores are not comparable, and the night after compares normally again.

## How a change is judged

Scores are **paired by function**. kuna is deterministic and the sample is fixed, so
every function is measured on both nights and nothing moves by chance in the
measurement. The question is whether the functions that did move favour one direction
more than unrelated churn would:

- **perfect rate** (GED 0, type_match 1.0): McNemar's exact test on the functions that
  became perfect against those that stopped being perfect;
- **value**: the sign test on functions whose value improved against those that
  worsened.

A function decbench can measure but kuna gave no value for (not emitted, unparseable)
counts as the worst value. The four p-values are Holm-adjusted together and a change
is significant below 0.01. A significant move in the bad direction is a regression;
so is a binary that kuna newly fails on.

**Speed** is the sample's total decompile CPU time (`user + sys` of the kuna process,
one binary at a time so neighbours do not inflate it). Each binary is decompiled by
tonight's build and by last night's (`kuna-nightly-build` artifact), back to back,
alternating which goes first, so both arms see the same machine. The 95% interval is
a bootstrap over binaries. A slowdown is a regression only when the interval excludes
zero **and** the total is more than 5% slower. When last night left no build, the
latest release is the baseline; when tonight could not time a baseline at all, the
comparison falls back to last night's recorded times from a different runner, which
is reported but never called a regression.

Any regression fails the `decbench` job (and so the night), with the report saying
why. The next night compares against this one, so an intended trade-off shows red for
one night only.

## Cost

Measured on the `ubuntu-26.04` runner (AMD EPYC 7763, 4 vCPU): the `decbench` job
takes about 30 minutes, of which 3 are a cold build, 2 setup (Rust Joern, decbench,
the sample download), 22 the one-at-a-time A/B timing (~660 s of CPU per build), and
3 scoring. The `tests` job runs beside it. Timing against a near-identical build came
out at -0.1% (95% CI -1.1% .. +0.8%), so the A/B resolves changes of about 1%. The
per-function scores from that run matched a run on a different machine and Python
version exactly.

## History

Each night's artifact carries `history.jsonl`: the previous night's lines plus its
own, so the chain survives as long as a night runs within the 90-day artifact
retention. The report renders the last two weeks of it.

## Running it locally

```bash
python3 -m scripts.nightly.dataset --revision <dataset sha> --data data --sample-out out/sample.json
<decbench venv>/bin/python -m scripts.nightly.bench --sample out/sample.json --data data \
    --kuna decompiler/target/release/kuna --specs specs \
    [--baseline-kuna old/kuna --baseline-specs old/specs] \
    --decbench-root ~/github/decbench --out out
python3 -m scripts.nightly.compare --current out/results.json --previous prev/results.json \
    --report out/report.md
```

The bench needs a Python that imports `decbench` and `rust_joern` (set
`RUST_JOERN_LIBRARY` for a non-editable Rust Joern install), and the decbench checkout
it is given must be the commit that Python imports.
