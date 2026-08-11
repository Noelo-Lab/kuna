# `returndup` at O0: the slice DIV-54 never measured (2026-08-11)

DIV-54 (#246) flipped `returndup` from default-OFF to default-ON on an A/B over **52,862
functions at O2 and O2-noinline**. Its table has two rows and neither is O0. This file is
the same A/B, same harness construction, same metric, extended to **all three optimisation
levels** -- 795 slices, **85,195 scored functions**, and in particular the **32,339-function
O0 slice** that was never scored.

It exists because O0 is the campaign's current target: it is the one level where the published
columns put IDA ahead of kuna on raw count (14,983 of 32,401 against 14,709 of 32,339), and --
as section 1 shows -- the one level where a single option decides the sign of the head-to-head.
And because four independent round-4 triage records
found `returndup` over-firing there (`triage/O0-zlib-example-updatewindow.md`,
`triage/O0-zlib-example-inflateValidate.md`,
`triage/O0-cleanflight-cleanflight_DALRCF405-USBD_DataInStage.md`,
`triage/O0-crazyflie-cf2-DCD_HandleUSBSuspend_ISR.md`).

Three findings, in order of how much they should change what the repo does:

1. **At O0 `returndup` is net-negative on GED-perfect (-192) and net-positive on aggregate
   GED (-9,038, i.e. better).** The two summary statistics disagree, and the campaign's
   metric is the first one.
2. **Corpus-wide it is still strongly positive (+640 perfect over all three levels), so
   flipping the default back is the wrong move** -- it would buy 192 functions at O0 and sell
   832 at O2 and O2-noinline.
3. **One structural sub-shape carries essentially all of the O0 harm and almost none of the
   O2 benefit**: the split that permanently blocks `rule_block_or` from folding a
   short-circuit chain. Declining exactly that is worth **+665 perfect at O0 for -15 / -20 at
   O2 / O2-noinline -- +630 corpus-wide, and -2,024 aggregate GED as well.** That is a real
   gate, not an opt-level switch, and it is the one this file recommends building.

## Method

Constructed exactly like #246's ablation, extended to O0 and re-run on all three levels so
the numbers are one build and one harness throughout.

- **Two arms, one binary.** `kuna decompile-all <stripped> --json --max-fn-seconds 120`,
  once as shipped and once with `--option returndup off`, kuna at HEAD `82dd39a7`. No
  `--mode` flag, so `auto` applies -- which is what the benchmark's `kuna_raw` backend does
  (`decbench/decompilers/raw/kuna_raw.py`), and it resolves to `aggressive` on nearly every
  binary here. That the `--option` still wins over the preset on the same command line was
  verified directly on the `zlib example` witness before anything else was run.
- **decbench's population, decbench's names.** Each arm's JSON is rebuilt into a
  decbench-format artifact: for every `// Function: <name> @ <addr>` marker of the stored
  `<opt>/<project>/decompiled/kuna_<stem>.c`, the fresh record at that address, with its own
  `sub_<addr>` placeholder renamed to the marker name -- `run_benchmark._relabel_to_dwarf`'s
  convention. **All 89,135 markers over all 795 slices matched a fresh record carrying code,
  in both arms**: zero misses, zero decompile failures, zero timeouts.
- **decbench's metric.** Scoring replays `scripts/reeval_ged.py:eval_one` verbatim --
  `extract_cfgs_from_source(..., sanitize_decompiled=True)`, `resolved_source_for_binary()`
  against the tree's own `ged_src/<project>.pkl`, `GEDMetric`, non-finite dropped. The
  resulting denominators are 32,339 / 22,546 / 30,310 against decbench's published
  32,339 / 22,549 / 30,313.

Four controls, all of which had to pass before any number here was written down:

| control | O0 | O2 | O2-noinline |
|---|---|---|---|
| functions whose body the pass never changes must score identically | 23,767 / **0 differ** | 17,170 / **0 differ** | 23,587 / **0 differ** |
| slug-only diffs (`// return-dupe` banner, identical body) must score identically | 1,420 / **0 differ** | 390 / **0 differ** | 503 / **0 differ** |
| the ON arm must reproduce the published column's verdict | **99.8%** (32,263/32,339) | **99.8%** (22,502/22,546) | **99.8%** (30,240/30,310) |
| functions scored in one arm but not the other | **0** | **0** | **0** |

The published-column disagreements are HEAD-vs-pinned-`9623dc27` build drift, and they run
in kuna's favour (+76 / +44 / +70 perfect on HEAD).

Two definitions used throughout. **"Fires"** means the *body* differs between the arms, not
the text: 14.7% of O0 text diffs are the banner comment alone and are excluded (they are
control row 2). **`dGED`** in a cell means `GED(off) - GED(on)`, so **positive means
`returndup` helps that cell**, and `net perf` is perfects created minus destroyed.

Nothing in `~/github/decbench` was written to; the tree stayed on its pinned commit
`e6e6b0f6`.

## 1. What `returndup` does, per level

In #246's own columns, with O0 filled in:

| scope | n | firings | GED sum ON | GED sum OFF | GED-perfect ON | GED-perfect OFF | dPerfect |
|---|---:|---:|---:|---:|---:|---:|---:|
| **O0** | **32,339** | **8,572** | **230,291** | **239,329** | **14,785** | **14,977** | **-192** |
| O2 | 22,546 | 5,376 | 396,062 | 402,954 | 7,422 | 7,060 | +362 |
| O2-noinline | 30,310 | 6,723 | 286,500 | 294,587 | 11,129 | 10,659 | +470 |
| **all three** | **85,195** | **20,671** | **912,853** | **936,870** | **33,336** | **32,696** | **+640** |

Both directions, which is the number that matters and the one a single aggregate hides
(every one of these 4,262 functions is listed by name, with its features, in
`returndup-perfect-movers.csv` next to this file):

| scope | to perfect | off perfect | ratio | improved | worsened | McNemar z |
|---|---:|---:|---:|---:|---:|---:|
| **O0** | **1,217** | **1,409** | **0.86 : 1** | 4,934 | 3,059 | **-3.75** |
| O2 | 522 | 160 | 3.26 : 1 | 3,180 | 1,641 | +13.86 |
| O2-noinline | 712 | 242 | 2.94 : 1 | 3,811 | 2,275 | +15.22 |

O0 is not a weaker version of O2 -- it is the opposite sign, at a significance that rules out
noise. The optimized levels reproduce #246 (it measured 3.2 : 1 and +417 perfect; this run,
on a build ~30 commits newer, measures 3.1 : 1 and +832 over the same two levels).

The aggregate-GED column disagrees with the perfect column **at O0 only**: the pass moves a
lot of functions closer to the source shape (4,934 improved against 3,059 worsened, mean 4.50
removed where it wins against 4.30 added where it loses) while knocking a net 192 off exact
zero. Which statistic decides is a campaign question, not a measurement question -- the
scoreboard, the standings and every "kuna vs IDA" claim are perfect-count claims.

### Against IDA, which is why the question was asked

Head-to-head on the functions both tools have a score for, in the same frame
`features-round4.md` uses (its `[V]` figure for the pinned build is 3,067 ida-only against
2,938 kuna-only, a net -129):

| O0, 32,077 functions both scored | ida-only wins | kuna-only wins | net |
|---|---:|---:|---:|
| kuna published (pinned `9623dc27`) | 3,067 | 2,938 | **-129** |
| kuna ON, fresh HEAD (shipped) | 3,066 | 3,013 | **-53** |
| kuna OFF, fresh HEAD | 2,977 | 3,119 | **+142** |

At O0, turning this one option off is by itself the difference between trailing IDA and
leading it. At the other two levels it is the reverse, and by more:

| head-to-head | n | ida | kuna ON | kuna OFF |
|---|---:|---:|---:|---:|
| O2 | 21,046 | 6,848 | 6,590 (-258) | 6,239 (-609) |
| O2-noinline | 28,239 | 10,126 | 9,925 (-201) | 9,471 (-655) |

## 2. Is there a clean separating gate?

**Yes -- but not the one the return-value axis suggests, and only half of what the four
witnesses suggest.** #246's nine partitions were re-run per level with per-cell *perfect*
counts (it reported those only for its loop cell), plus four new ones aimed at the refuters'
re-specified separator. Most cells change sign between O0 and O2. One changes sign, carries
nearly all of the O0 harm, and carries almost none of the O2 benefit.

### The axis that looks decisive at O0 and is a trap

| cell | level | n | dGED | perf+ | perf- | net perf |
|---|---|---:|---:|---:|---:|---:|
| returns **not** all identical | O0 | 3,350 | -5,685 | 97 | 1,164 | **-1,067** |
| | O2 | 2,431 | +2,634 | 184 | 29 | **+155** |
| | O2-noinline | 2,946 | +2,770 | 304 | 64 | **+240** |
| returns all identical | O0 | 5,222 | +14,723 | 1,120 | 245 | +875 |
| | O2 | 2,945 | +4,258 | 338 | 131 | +207 |
| | O2-noinline | 3,777 | +5,317 | 408 | 178 | +230 |

The same structural cell is -1,067 at O0 and +395 at the optimized levels. So the
return-value set **is not a separator**: it is an opt-level trade dressed as one, exactly as
`features-round4.md` Killed **K3** argues on different evidence. Gating it would be a global
policy choice worth +672 perfect corpus-wide -- real, but bought by giving up 395 optimized
wins, and not defensible as "this shape is wrong".

The same is true of `void`, which is a sub-case of it (+948 / -117 / -204), and the same
holds in reverse for the merge-site shape the witnesses point at: **"the split destroyed an
`else`" is net-positive at every level** (+507 / +204 / +254), so sub-shape (ii) is not the
harmful one either. #246's loop cell reproduces exactly and stays small
(O0 -57, O2 -6, O2-noinline -6 net perfect; the O2 `dGED` of **-259** against its
published -257).

### The axis that separates on all three levels

Partition instead on whether the split **cost a short-circuit fold** -- the OFF arm's body
carries `&&`/`||` operators that the ON arm's does not, which is `rule_block_or` being
permanently blocked because the guards no longer share an out-target. This is the refuters'
re-specified sub-shape (v), the or-chain case, and it is where the harm lives:

| a short-circuit was lost | n | dGED | perf+ | perf- | **net perf** |
|---|---:|---:|---:|---:|---:|
| **O0** | 2,908 | **-3,538** | 48 | 713 | **-665** |
| **O2** | 1,638 | +1,141 | 53 | 38 | **+15** |
| **O2-noinline** | 2,023 | +373 | 91 | 71 | **+20** |

Catastrophic at O0, flat at both optimized levels -- 4.1% and 4.3% of those levels' net wins.
That is the shape of a genuine gate rather than an opt-level switch: the same predicate is
the harm at O0 and costs almost nothing where the pass pays.

`iproute2 ip::sci_complete` is the archetype. With `returndup off` the whole function
re-rolls to the source's single boolean expression and scores 0; the split removes the 0/1
select diamond before `iteboolean` and `rule_block_or` can see it, and kuna emits a
five-return cascade for GED 21:

```c
/* returndup off - GED 0 */                     /* shipped ON - GED 21 */
v1 = ((a0) || ((a1 && ((a2 || (a3))))));        if (a0) return 1;
return v1;                                      if (!a1) return 0;
                                                if (a2) return 1;
                                                if (a3) return 1;
                                                return 0;
```

`zlib inflateValidate` (a round-4 witness) is the plainer form of the same loss -- an `else`
traded for a duplicated `return 0;`.

The four round-4 witnesses split two-two across the two candidate predicates, and the two
predicates are complementary:

| witness | ON | OFF | short-circuit lost | returns not all identical |
|---|---:|---:|---|---|
| `cleanflight USBD_DataInStage` | 13 | 0 | **yes** | no |
| `crazyflie DCD_HandleUSBSuspend_ISR` | 12 | 0 | **yes** | no |
| `zlib updatewindow` | 6 | 0 | no | **yes** |
| `zlib inflateValidate` | 5 | 0 | no | **yes** |

So no single witness-derived theory covers the set, which is exactly what Killed **K6**
predicts ("the banner is not a loss signature; the sub-shapes are"). At corpus scale the
short-circuit predicate repairs **713 of the 1,409** O0 regressions while forfeiting **48** of
the 1,217 O0 wins; the return-value predicate repairs **1,164** while forfeiting **97**.

### The counterfactual, and its honest limit

Simulating a gate by taking the OFF body wherever the predicate holds (which is exact for a
whole-function gate, since "declined everywhere in this function" *is* the OFF body):

| gate -- decline the split when... | O0 | O2 | O2-noinline | **total perfect** | total GED |
|---|---:|---:|---:|---:|---:|
| *(shipped: no gate)* | 0 | 0 | 0 | **0** | 0 |
| turn the option off entirely | +192 | -362 | -470 | **-640** | +24,017 |
| the returns are not all identical | +1,067 | -155 | -240 | **+672** | -281 |
| the return is non-void | +948 | -117 | -204 | **+627** | +1,632 |
| **a short-circuit fold is lost** | **+665** | **-15** | **-20** | **+630** | **-2,024** |
| ...or the ON body nests deeper | +801 | -14 | -16 | **+771** | -2,201 |
| ...or the returns are not all identical | +1,054 | -142 | -223 | **+689** | +1,123 |

Two of these predicates are **decidable before the split** from what the pass already
computes: "non-void" is the RETURN op's input count, and "the returns are not all identical"
is the union of the constant sets `returndup_value_is_const_rec` already resolves (splitting
materialises one constant per arm, it cannot change the set). The
short-circuit predicate is **not**: this measurement identifies it by comparing the two
arms' output, so **+630 is an upper bound on what a real gate achieves**, not a promise. The
implementable form is a precondition on the shared return's predecessors -- do they form the
operand chain of a short-circuit that `rule_block_or` would otherwise fold -- and its fidelity
to this proxy is unmeasured and is the first thing an implementation must report.

### What the OFF arm costs, which the metric does not charge it for

Turning the pass off is not free of shape damage, and two of its harms are invisible to GED:

- The OFF arm **de-structures a loop in 51 of 8,572 O0 firings** (a `while( true )` that
  appears only when the split is declined) -- the mirror of the ON arm's 313.
- **147 of the 1,409 O0 panes where OFF is perfect and ON is not nest four or more levels
  deep, and 44 nest six or more** (`rsyslog setModCnf` at 18, six openssh `ssh_ecdsa_sign`
  copies at 8). GED scores a deep `else if` cascade as a perfect structural match; a reader
  does not. Any recommendation that leans on the OFF arm is buying scoreboard points with
  some of those.

## 3. What the repo should do

**Recommendation: keep the default, and build the short-circuit gate.** Concretely:

1. **Do not flip DIV-54 back.** It costs 640 GED-perfect functions across the corpus to buy
   192 at O0. The O0 result is a real defect in the *default*, but "turn it off" is a worse
   answer than "fire it less".
2. **Build the or-chain gate** as a `returndup`-local precondition (`returndup_apply`
   declines a shared return whose predecessors are the operand blocks of a short-circuit
   chain `rule_block_or` would fold). Measured ceiling **+630 GED-perfect and -2,024
   aggregate GED corpus-wide**, concentrated as +665 at O0 for -35 at the optimized levels.
   It needs no new option if it only corrects wrong output, but it changes emitted C on ~2.9k
   O0 functions, so it is a feature: option-gated, DIV row, `tests/stages/` two-pass case,
   and a full three-level re-sweep before any default flip. It is `returndup`-local, which
   is what Killed **K12** concluded the safe cut is.
3. **Do not gate on the return-value set** (uniform-value / void / distinct-returns). It is
   the largest-looking lever at O0 (+1,067) and it is an opt-level trade, not a defect
   predicate: the identical cell is a win at every level and the distinct cell is a win at
   two of three. Same verdict as **K3**, now with the O0 magnitudes attached.
4. **Correct the record in two places.** `docs/decbench/returndup-regression-triage.md`
   concludes "net-positive in every subpopulation examined" and "no discriminator exists";
   both are true only of aggregate GED at O2. On perfect count at O0, **every one of its nine
   partitions has a net-negative cell**, and the discriminator it could not find is a
   partition it did not test. `docs/decbench-loop.md` also still says the benchmark injected
   `option listing on` and ran the equivalent of `--mode reliable`; the backend appends
   `--mode` only when `DECBENCH_KUNA_MODE` is set, so the scored column is `auto`.

**What would falsify this.** The recommendation rests on one claim the measurement cannot
make: that a *pre-split* CFG predicate can recognise the or-chain shape with roughly the
fidelity of the post-hoc "a short-circuit was lost" signal. If an implementation of the gate,
swept over the same three levels, comes back below about +300 total perfect, or negative at
either optimized level, then the honest reading flips to "the harm is real but not
separable", and the choice becomes the return-value trade (+672, with its 395 optimized
losses) or nothing. Two smaller falsifiers: if a future default (a stronger `iteboolean` or
`condfold`) re-rolls those cascades after the split, the O0 cell shrinks on its own; and if
the campaign ever decides aggregate GED is the metric, `returndup` is already correct at O0
and none of this applies.

## Reproducing

This measurement is the first job of a reusable instrument, added by this work as
`scripts/decbench/optsweep.py` -- a bidirectional corpus-scale A/B for any single option
flip, with the three harness controls above built in:

```bash
~/.virtualenvs/decbench/bin/python -m scripts.decbench.optsweep \
    --option returndup off --opt O0 --opt O2 --opt O2-noinline \
    --workers 16 --joern-workers 8 --out docs/decbench/returndup-sweep
# -> report.md (headline + both directions + per project/arch), moved.csv (every moved
#    function by name, both directions), changed.txt (the requirement-8 diff set),
#    rows.json (per-function, for ad-hoc partitioning)
```

`optsweep` is a second, independent implementation of the same measurement, cross-checked
against the ad-hoc harness that produced this file: on `O0 zlib example` the two agree on
**157 of 157 functions in both arms, zero value mismatches**.

The per-function tables behind every number in this file, the partition/gate/search scripts
(`analyze.py`, `gate.py`, `search.py`, `combined.py`) and both arms' artifacts are in this
session's scratch directory
(`/tmp/claude-1000/-home-mahaloz-github-kuna/d756055a-8e91-4631-b6ae-5e5a69c3ffc2/scratchpad/`,
`returndup-o0/` and `returndup-opt2/`).

### What this measurement does not cover

- **Only the GED metric.** `byte_match` and `type_match` were not re-scored in either arm.
- **Only functions decbench already scores for kuna.** The 261 O0 functions IDA scores and
  kuna produces nothing scoreable for belong to the recall workstream and cannot move here.
- **The or-chain predicate is measured as an outcome, not as an implementable precondition**
  (see above). The uniform-return predicate is the only top candidate that is decidable
  in-pass as measured.
- **No speed measurement.** A gate that adds a CFG probe to `returndup_apply` needs its own
  interleaved timing against the 5% budget.
- **The rivals' columns are frozen.** IDA's numbers are the tree's stored scores; only kuna
  was re-decompiled. That is the same footing every campaign comparison stands on, and it is
  why the head-to-head is stated on the both-scored subset rather than as a standings claim.
