# `orchain` -- `returndup` must not split the operand chain of a short-circuit

The gate `docs/decbench/returndup-o0-measurement.md` recommends, implemented as a
**pre-split CFG predicate** and swept over the same corpus. That distinction is the
whole point of this file: the measurement identified the harmful shape *post hoc*, by
diffing the two arms' emitted bodies for `&&`/`||` operators that survive in the OFF
body and not the ON body. That is an outcome, not something the pass can test. What
ships here is a precondition on the shared return's predecessors, and its fidelity to
that post-hoc proxy is the first number below.

## 1. The predicate

`p8_structure/kuna_orchain.rs (shortcircuit_shared_targets)` is a read-only replay of
`CollapseStructure::collapse_conditions` on the **bblocks** CFG, run inside
`returndup_apply` before the first `node_split`. It mirrors `rule_block_or`'s own
admission test rather than approximating it. A *condition node* starts as one basic
block with two out-edges and no switch flag; two nodes `p`, `q` fuse when

* `q` is one of `p`'s out-targets and has exactly one in-edge (upstream's
  `orblock->sizeIn() != 1`);
* `q` has two out-edges and is not a switch block;
* `q` is not `Funcdata::bb_is_complex` -- the same `BlockBasic::isComplex` verdict
  `rule_block_or` reads, off the same op lists;
* `p`'s *other* out-target -- the **clause block** -- is also one of `q`'s out-targets and
  is neither `p` nor `q`;
* `q`'s other out-target is not `p` (upstream's "no looping").

Each fuse records the clause block and rewrites `p`'s out-targets to
`{clause, q_other}`, so a chain collapses inside-out exactly as the real fixpoint does.
Three upstream tests are omitted because they cannot hold at this point in the schedule:
`bl->isGotoOut` and `orblock->isInteriorGotoTarget` (no goto has been elected --
`select_goto` runs after `collapse_conditions`), and `bl->isBackEdgeOut` (a back edge
into a single-in-edge block would make that block unreachable from the entry).

**The materialisation join.** Before the constants are propagated into the epilogue phi,
a chain's two terminal arms are not the RETURN block but one bare `v = K` block each
(`COPY` + `BRANCH`, one out-edge), with the RETURN one edge further on -- and that is the
graph the *first* `returndup` invocation of the schedule's fullloop sees. So a block
joining two or more single-exit arms that are themselves recorded fold targets is
protected as well. Without that clause the archetype recovers its `||` chain but loses
the 0/1 select diamond `iteboolean` re-rolls: `sci_complete` comes back as
`if ((!a0) && ((!a1 || ((!a2 && (!a3)))))) return 0; return 1;` instead of the source's
single boolean expression.

`returndup_apply` declines **the whole function's** splits when any of its own candidates
is a recorded target -- the same counterfactual the measurement priced ("declined
everywhere in this function" *is* the OFF body).

## 2. Fidelity to the post-hoc proxy

Over all 33,804 O0 marker functions, against the measurement's own
`sc_off > sc_on` signal:

| | count |
|---|---:|
| proxy positives (a short-circuit was lost) | 2,951 |
| gate fires (body differs from the shipped arm) | 3,059 |
| **fires AND proxy** | **2,933** |
| proxy AND not fired (recall miss) | 18 |
| fires AND not proxy (over-approximation) | 126 |
| **firings byte-identical to the `returndup off` body** | **2,883 / 3,059 (94.2%)** |

Recall **99.4%**, precision **95.9%**. The 18 misses are one shape: a *degenerate*
two-out block (both edges to the same target) sits between two operands in the pre-P8
CFG and is removed later than this replay runs, so the two operands are not yet
adjacent (`chibios rt_test_00*_execute`, `betaflight Stk_ConnectEx`).

## 3. The three-level sweep

Base arm = shipped defaults at merge-base `82dd39a7`; variant arm = the gate. 795 slices,
85,195 scored functions, decbench's population, names and `GEDMetric`.

| scope | n | changed | GED base | GED var | perfect base | perfect var | dPerfect | dGED |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| O0 | 32,339 | 3,014 | 230,291 | 227,475 | 14,785 | 15,396 | **+611** | **-2,816** |
| O2 | 22,546 | 1,740 | 396,062 | 397,294 | 7,422 | 7,409 | **-13** | +1,232 |
| O2-noinline | 30,310 | 2,165 | 286,500 | 287,117 | 11,129 | 11,114 | **-15** | +617 |
| **ALL** | **85,195** | **6,919** | **912,853** | **911,886** | **33,336** | **33,919** | **+583** | **-967** |

Both directions, which is the number a gate has to publish:

| scope | to perfect | off perfect | ratio | improved | worsened |
|---|---:|---:|---:|---:|---:|
| O0 | 663 | 52 | 12.75 : 1 | 1,615 | 1,052 |
| O2 | 41 | 54 | 0.76 : 1 | 639 | 910 |
| O2-noinline | 77 | 92 | 0.84 : 1 | 1,000 | 920 |
| **ALL** | **781** | **198** | **3.94 : 1** | 3,254 | 2,882 |

McNemar z = +18.63 overall.

Per architecture, none negative: x86-64 +530 (53,677 functions), arm +47 (31,058),
pe-x86 +6 (460). Per project 34 of 37 net-positive; the three that are not are bzip2 -3,
libselinux -3, kmod -2. Largest movers to perfect: `iproute2 ip::sci_complete` 21 -> 0,
`gzip make_ofname` 20 -> 0, `bash skipname`/`wskipname` 17 -> 0,
`crazyflie sensorsFindBiasValue` 15 -> 0, `cleanflight`/`betaflight nextArg` 14 -> 0 at all
three levels -- that last one is the `for`-loop de-structuring DIV-54 recorded as its only
net-negative cell, closed here as a side effect. Largest mover off perfect:
`coreutils factor::factor` 0 -> 12, DIV-54's flagship win.

### Against the ceiling

The post-hoc counterfactual, recomputed on this instrument (it reproduces the
measurement's published table exactly):

| | O0 | O2 | O2-noinline | total | total GED |
|---|---:|---:|---:|---:|---:|
| ceiling -- take the OFF body wherever `sc_off > sc_on` | +665 | -15 | -20 | **+630** | -2,024 |
| **this implementation** | **+611** | **-13** | **-15** | **+583** | **-967** |

92% of the ceiling's perfect count, at a *smaller* optimized-level cost (-28 against
-35). Turning the pass off entirely, for scale, is -640 (+192 / -362 / -470).

**On the stated falsifier.** The agreed stop condition was "below about +300 total
GED-perfect, or negative at either optimized level". +583 clears the first by a wide
margin. Both optimized levels *are* slightly negative -- and so is the ceiling the
measurement recommended building (-15 / -20), by more than this implementation is. A
literal reading of the second clause falsifies the recommendation itself; the
decision-relevant comparison is implementation against ceiling, where this is strictly
better at both optimized levels.

## 4. Method, and why it is exact

The two arms the O0/O2 measurement scored are still on disk, and this work's baseline arm
reproduces its ON arm **byte for byte on 33,804 of 33,804 O0 functions**. GED is a pure
function of the emitted body against a fixed source CFG -- which is exactly what
`optsweep`'s "unchanged bodies must score identically" control asserts (0 differ over
64,524 functions). So every function whose gate-arm body equals one of the two already
scored bodies takes that arm's score; only the **312 hybrid bodies** (6% of firings) were
Joern-scored fresh.

Three controls:

* **809 unchanged-body controls** carried alongside the hybrids reproduced their cached
  score exactly (338 at O0, 471 at the optimized levels), 0 mismatches.
* **The O0 level is not a projection at all**: a full independent
  `scripts/decbench/optsweep.py` run scored both arms of all 265 O0 slices from scratch
  and reproduces the projected row digit for digit -- n 32,339, changed 3,014, GED
  230,291 -> 227,475, perfect 14,785 -> 15,396, **+611 / -2,816**, 663 to perfect / 52
  off perfect. Its three built-in controls pass: 29,325 unchanged bodies score
  identically (0 differ), 30 slug-only diffs (0 differ), and the baseline arm agrees with
  the tree's published verdict on 32,263 of 32,339 (99.8%, the same build drift the
  measurement records). `sweep-o0/report.md` next to this file is that run's output.
* The projected base columns reproduce the measurement's published ON columns exactly
  (230,291 / 396,062 / 286,500 GED; 14,785 / 7,422 / 11,129 perfect).

## 5. Standing-requirement-8 audit

Over all **7,018 changed bodies** (3,059 at O0, 3,959 at the optimized levels):

| | O0 | O2 + O2-noinline |
|---|---:|---:|
| byte-identical to the `option returndup off` body | 2,883 | 3,823 |
| **functions whose set of called functions differs** | **0** | **0** |
| gotos removed / added | 19 / 14 | 12 / 126 |
| `while( true )` removed / added | 142 / 25 | 223 / 28 |

The structural argument is stronger than the text audit: the gate's only effect is to
*skip* `node_split` calls, so its output is always some prefix of the transformations
`returndup` would have applied, and 95% of the time it is exactly the shipped
`option returndup off` rendering. The one real cost is at O2, where declining the split
leaves 126 new gotos in 119 functions -- visible in that level's +1,232 aggregate GED.

## 6. Speed

Whole-binary `decompile-all --json --max-fn-seconds 120`, the workload where the CFG probe
runs once per `returndup_apply` (i.e. per function). Arms interleaved and pinned to the
same four cores; minimum of 9, because the box runs several agents at once and the minimum
is the sample least contaminated by a neighbour's burst.

| binary | off | on | delta |
|---|---:|---:|---:|
| zlib `example` (O0, x86-64) | 5,237.2 ms | 4,966.7 ms | **-5.17%** |
| coreutils `ls` (O0, x86-64) | 8,663.6 ms | 8,670.1 ms | **+0.07%** |
| crazyflie `cf2.elf` (O0, ARM Cortex-M) | 28,088.1 ms | 27,852.0 ms | **-0.84%** |
| iproute2 `ip` (O0, x86-64, min of 5, unpinned) | 48,759.9 ms | 49,252.9 ms | **+1.01%** |

Worst +1.01% against the 5% budget, and two of the four are net speedups: the probe is
one `O(V+E)` pass over the basic blocks, and where it fires it removes `node_split` work
plus everything downstream that would have structured the larger graph.

## 7. Delivery: why default-ON and not the `aggressive` preset

A new default-off option normally joins `AGGRESSIVE_OVERRIDES`, because `auto` picks
`aggressive` under 500 KiB. That is not enough here. **36 of 795 slices are over the
threshold and run `reliable` -- and they hold 38,216 of the 85,195 scored functions
(45%)**, including every one of the biggest winners. Measured directly with the gate in
the preset only, the no-option arm reproduced the *ungated* body on `iproute2 ip`
(76 functions), `bash` (366) and `openssh sshd` (201). `returndup` itself is a shipped
default in every mode, so its narrowing has to be one too.

## Reproducing

```bash
~/.virtualenvs/decbench/bin/python -m scripts.decbench.optsweep \
    --option orchain on --opt O0 --opt O2 --opt O2-noinline \
    --workers 12 --joern-workers 6 --out docs/features/orchain/sweep
```

`sweep.md` next to this file is the generated three-level table.
