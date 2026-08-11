# Round 4 grounding: where the kuna-vs-IDA gap actually is

Measured on 2026-08-10 against HEAD `82dd39a7`, before any triage was read. Round 3's
lesson was that triaging a stale pool spends the round on defects that no longer exist;
this round opens by locating the gap in the scoreboard itself so the triage sample can be
aimed rather than sampled uniformly.

All GED figures recomputed directly from `~/github/decbench/results/full_run/ged_new.json`,
which is the run's own per-function metric store. kuna's column comes from the pinned build
`9623dc27`; every rival's is frozen from its own last run. Treat the tree as a mining
instrument, not a publishable leaderboard.

## O0 is the only level where IDA holds more perfect functions

| opt | ida perfect / scored | ida % | kuna perfect / scored | kuna % | perfect deficit | scored delta |
|---|---|---:|---|---:|---:|---:|
| O0 | 14,983 / 32,401 | 46.24% | 14,709 / 32,339 | 45.48% | **+274 for ida** | -62 |
| O2 | 7,269 / 21,639 | 33.59% | 7,378 / 22,549 | 32.72% | -109 (kuna ahead) | +910 |
| O2-noinline | 10,586 / 28,880 | 36.66% | 11,059 / 30,313 | 36.48% | -473 (kuna ahead) | +1,433 |

Total perfect functions: **ida 32,838, kuna 33,146 (+308 for kuna)**.

The percentage table reads as "kuna trails at all three levels". The count table says
something different and more useful: at O2 and O2-noinline kuna's lower *rate* is a
denominator effect -- it is scored on 2,343 more functions than IDA and holds more perfect
ones at both. **O0 is the only level with a real deficit**, and it is 274 functions.

## The O0 deficit is highly symmetric, so net effect is the only thing that counts

At O0, per function, comparing perfect-vs-not:

| | count |
|---|---:|
| IDA perfect, kuna not (both scored) | 3,067 |
| kuna perfect, IDA not | 3,054 |
| IDA perfect, kuna produced nothing scoreable | 261 |
| **net** | **+274 for ida** |

The net deficit is **4% of the 6,121-function disagreement volume**. A change that flips
100 losses into wins while flipping 100 wins into losses nets zero. Standing requirement 8
("diff every changed function, not the witness") is therefore not a nicety at O0; it is the
only way to know whether a feature moved the number at all. Every round-4 candidate is
priced against BOTH populations.

Per-project net (top contributors):

| project | ida-only | kuna-unscored | kuna-only | net |
|---|---:|---:|---:|---:|
| betaflight | 256 | 169 | 222 | +203 |
| gnutls | 236 | 0 | 44 | +192 |
| coreutils | 239 | 0 | 70 | +169 |
| bash | 286 | 0 | 175 | +111 |
| crazyflie | 426 | 8 | 354 | +80 |
| zlib | 115 | 0 | 48 | +67 |
| cleanflight | 122 | 80 | 137 | +65 |

## Half the O0 deficit is recall, and it is one shape

324 functions IDA scores at O0 produce no usable GED from kuna at all -- a guaranteed zero
on every one. Probed against `kuna functions --json --mode aggressive` on today's build:

- **322 of 324 (99.4%) are INTERIOR to a kuna entry.** kuna discovered a function that
  starts before the true entry. Only 2 are entries kuna found and simply scores badly.
- Every one is ARM Cortex-M: betaflight 210, cleanflight 97, crazyflie 8, riot-os 5,
  chibios 1, nuttx 1 (plus 2 exact-entry in libbsd).
- The distance to the containing entry is not uniform:

| bytes past the containing kuna entry | 2 | 4 | 6 | 8 | <=16 | <=64 | <=256 | median |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| count | **64** | 6 | 18 | 6 | 147 | 284 | 315 | 18 |

**64 of 322 are exactly 2 bytes -- one Thumb halfword.** A general "kuna merges functions"
story does not produce a spike at exactly one instruction. On the worked witness
(betaflight `RcStatsGetFullThrottleTimeUs`, true entry `0x803d314`, kuna's entry
`sub_803d312`) both addresses decompile to an identical body, so the only thing wrong is
the entry address.

Closing that pool is worth more than any structuring feature in the round:

| O0 recall recovered perfect | kuna O0 | ida O0 |
|---:|---:|---:|
| +94 (the <=8-byte class) | 45.64% | 46.24% |
| +162 | 45.76% | 46.24% |
| +324 (all of it) | 46.02% | 46.24% |

## The structural signature of the remaining 3,067

Structural metrics of kuna's stored pane minus IDA's, over all 3,067 O0 functions where IDA
is perfect and kuna is not (paired panes, 3,067 of 3,067):

| metric | mean (kuna - ida) | median |
|---|---:|---:|
| ifs | **+0.59** | 0 |
| loc | +0.23 | +1 |
| loops | +0.03 | 0 |
| ternaries | +0.03 | 0 |
| labels | +0.01 | 0 |
| gotos | -0.00 | 0 |
| switches | -0.00 | 0 |
| cases | -0.02 | 0 |

**It is not goto density and it is not loop shape.** It is extra conditional branches. The
control population (kuna perfect, IDA not; n=2,938) shows the mirror: kuna is 3.37 LOC
shorter, with no `if` excess.

71% of the losing set is a small delta: kuna GED median 6, and 2,185 of 3,067 score <= 8.

## Two suspects tested, one killed

**`condfold` is not the lever.** 120 O0 losing-set functions, three arms:

| arm | mean if delta vs ida | functions whose if-count moved |
|---|---:|---:|
| off (default) | +0.80 | -- |
| on | +0.80 | 2 of 120 |
| wide | +0.78 | 2 of 120 |

**`returndup` is.** Same harness, both populations, ablating the default-ON pass:

| population | n | fires | mean if delta ON | mean if delta OFF | better OFF | worse OFF |
|---|---:|---:|---:|---:|---:|---:|
| IDA perfect, kuna not | 78 | 28 | 1.38 | 0.79 | **20** | 1 |
| kuna perfect, IDA not (control) | 83 | 13 | 0.51 | 0.59 | 0 | **7** |

`returndup` accounts for roughly 43% of the if-excess on the losing set. Naively scaled to
the full populations that is about +786 improved against -258 regressed.

### Every other default-ON structuring option is inert on this population

Same harness, one ablation per arm, scoring total shape distance to IDA's pane summed over
`ifs + loops + switches + gotos + labels`:

| ablation | mean shape dist (the gap, n=46) | vs baseline | closer | farther | exact match |
|---|---:|---:|---:|---:|---:|
| baseline | 1.20 | -- | -- | -- | 21 |
| **`returndup=off`** | **0.52** | **-0.67** | **10** | **1** | **27** |
| `iteregion=off` | 1.20 | +0.00 | 0 | 0 | 21 |
| `branchflip=off` | 1.20 | +0.00 | 0 | 0 | 21 |
| `gotoreduce=off` | 1.20 | +0.00 | 0 | 0 | 21 |
| `itecondlist=off` | 1.20 | +0.00 | 0 | 0 | 21 |
| `earlyreturn=off` | 1.20 | +0.00 | 0 | 0 | 21 |
| `taildup=off` | 1.20 | +0.00 | 0 | 0 | 21 |

And the price, on the population where kuna already wins:

| ablation | mean shape dist (the price, n=44) | vs baseline | closer | farther | exact match |
|---|---:|---:|---:|---:|---:|
| baseline | 0.66 | -- | -- | -- | 23 |
| `returndup=off` | 0.68 | +0.02 | 0 | 1 | 22 |
| `iteregion=off` | 0.52 | -0.14 | 6 | 0 | 29 |
| every other | 0.66 | +0.00 | 0 | 0 | 23 |

Six of the seven suspects are **exactly zero** on the losing population. `returndup` is the
only structuring default that moves it, taking structurally-exact matches from 21 to 27 of
46 (+13pp) while costing one function of 44 in the control. Scaled to the full populations
that is roughly +400 newly exact against -69 lost -- against an O0 deficit of 274.

That scaling is a proxy and must not be quoted as a result. What it does establish is where
the expensive measurement belongs, and that a global flip is almost certainly the wrong
answer: DIV-54 bought +417 GED-perfect at O2 and O2-noinline, so the shippable outcome is a
**structural gate** that separates the O0 harm, not an opt-level switch -- which kuna cannot
observe at runtime in any case.

This matters because of what DIV-54 measured. PR #246 flipped `returndup` default-ON on an
A/B of **52,862 functions across O2 and O2-noinline only** -- its own table has exactly two
rows and there is no O0 row. The pinned benchmark build `9623dc27` IS that commit, so the
stored O0 column already carries the flip. Eleven of the round-4 triage records name
`returndup` in `option_closing`, several of them repeating the belief that "the default
cannot flip, DIV-54 measured +417 GED-perfect corpus-wide" -- a number that was never
measured on this slice.

A structural proxy is not GED. The real A/B on the O0 slice is
`docs/decbench/returndup-o0-measurement.md`.

## A driver bug found while measuring

`kuna functions --mode reliable` reports **1** function on stripped `crazyflie/cf2.elf`
where `kuna decompile-all --mode reliable` recovers **2,790** (chibios `ch.elf` 5 vs 770;
betaflight 1 vs 5,797; x86-64 `coreutils/ls` agrees at 588 vs 588). `list_functions` is the
only caller passing `default_listing = false`, and that one flag gates not just the Listing
but the non-x86-64 `funcstart_patterns` and `aif` **discovery** passes, whose entire job is
to add entries. Same parity class as DIV-53 and DIV-66. PR #239's own sequence comment was
misled by it, recording "auto -> reliable builds no Listing at all (1 entry, 0% recall)" --
true of the `functions` surface only.
