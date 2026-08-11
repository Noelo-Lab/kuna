# The `returndup` regression set: what it is, why it appeared, and why it stays (2026-08-02)

A kuna-only decbench re-run on the optimized datasets (kuna `36641d35` versus the stored
2026-07-15/16 artifacts, rivals frozen) reported **411 wins / 189 regressions** at
O2-noinline and **293 / 114** at O2, with a return/tail-duplication transform firing on the
large majority of every function whose score moved. This file is the triage of that finding.

Three questions had to be answered in order, because each one changes the next: **which**
transform fires, **why** it started firing, and whether its gate can be tightened to keep the
wins and drop the losses.

The answers: it is **`returndup`** and only `returndup`; it started firing because
**`--mode auto` became the front-end default** (DIV-40), not because anything changed
underneath it; and **no discriminator exists** — an isolating ablation puts `returndup` at
**+417 GED-perfect functions and −7,756 aggregate GED** over 52,862 functions, net positive in
every subpopulation examined. The code is unchanged.

## Method

Every number below comes from one of two measurements, both over the full O2 + O2-noinline
kuna corpus (**536 slices, 52,862 scored functions**):

- **The A/B ablation.** The same `36641d35` binary, invoked exactly as decbench's `kuna_raw`
  backend does (`decompile-all --json --max-fn-seconds 120`, no `--mode`, so `auto` applies),
  once as shipped and once with `--option returndup off`. Function names are re-mapped to
  their DWARF names by address exactly as `run_benchmark._relabel_to_dwarf` does, and each
  `.c` is scored with decbench's own `GEDMetric` against the cached project source CFGs.
  Two independent controls: the harness reproduces the published `kuna_update-passwd.c`
  **byte for byte** and its scores match `ged_new.json` on all 47 functions of that slice with
  zero mismatches; and across the corpus the **46,005 functions where `returndup` never fired
  score identically** in both arms (0 differ). The ablation therefore isolates exactly one
  transform.
- **Marker classification.** Per-function attribution off the emitted artifacts, described
  under *Which transform* below.

## Which transform: `returndup`, not `taildup`

The two passes are **indistinguishable by slug text**. `PrintC::warning_slug`
(`decompiler/crates/kuna-decomp/src/p9_emit/printc.rs`) maps `taildup:` and `returndup:` to
the *same* rendered slug, `return-dupe`. Any analysis that greps the artifacts for
`return-dupe` is measuring both at once.

They separate by **position**, because they use different warning sinks:

| pass | sink | where the slug lands |
|---|---|---|
| `returndup` | `Funcdata::warning_header` | the function's prototype line (`void xmalloc(uint8 a0) // return-dupe`) |
| `taildup` | `Funcdata::warning(text, addr)` | an interior statement line |

Classifying all 55,947 marker-declared functions that way:

| population (July → new) | n | `returndup` (header) | `taildup` (inline) |
|---|---:|---:|---:|
| win (nonzero → 0) | 704 | **608 (86%)** | 13 (2%) |
| regression (0 → nonzero) | 303 | **199 (66%)** | 8 (3%) |
| stable perfect (0 → 0) | 16,818 | 80 (0%) | 11 (0%) |
| stable imperfect (unchanged, nonzero) | 24,936 | 891 (4%) | 2,072 (8%) |
| moved, neither direction crossed 0 | 9,079 | 5,067 (56%) | 1,444 (16%) |

The raw counts settle it. The July artifacts predate DIV-39, so they carry full
`/* WARNING: … */` banners with the pass name in plain text — worth checking before trusting
any marker-based correlation, because the *rendering* changed underneath this comparison too.
Across all 536 July artifacts: **`taildup:` 5,183 lines, `returndup:` zero**. `taildup` has
been default-ON since DIV-14 and did not move. `returndup` went from **0** firings to
**6,857**.

## Why it started firing: the mode default, not DIV-47

`returndup` is default-**OFF** (DIV-18). It is switched on by the `aggressive` preset, and
since **DIV-40** the file front-ends default to `--mode auto`, which selects `aggressive` for
any input under 500 KiB — which is most of the benchmark. The stored July artifacts
(mtime 2026-07-15/16) predate the mode system entirely (`fast` #208 and `auto` #213 both
merged 2026-07-30), so that run got the shipped default and `returndup` never ran.

Direct confirmation on `base-passwd update-passwd::xmalloc` @ `0x2bc0`, the case that opened
the investigation — `--mode reliable` and `--option returndup off` produce the *same* body,
and it is the July body:

```c
/* --mode reliable  ==  --option returndup off  ==  the July artifact */
  if (!a0 && !malloc(a0)) {
    if (!malloc(1)) { __fprintf_chk(stderr,1,"Out of memory!\n"); exit(1); }
  }
  return;

/* default (auto -> aggressive) == the new artifact, GED 0 -> 9 */
  if (a0 || malloc(a0)) return;
  if (malloc(1)) return;
  __fprintf_chk(stderr,1,"Out of memory!\n");
  exit(1);
```

**#228 (DIV-47, the two restored `Cover` extensions) is refuted.** Building `#228` and its
parent `c2a37ebf` and forcing `returndup on` in both, the `returndup:` fire count is
identical on every probe binary:

| binary | pre-#228 | #228 | `36641d35` | `returndup off` |
|---|---:|---:|---:|---:|
| base-passwd `update-passwd` | 17 | 17 | 17 | 0 |
| coreutils `ls` | 52 | 52 | 52 | 0 |
| openssh `sshd` | 410 | 410 | 408 | 0 |
| cleanflight `DALRCF405` | 699 | 699 | 699 | 0 |
| zlib `minigzip` | 76 | 76 | 76 | 0 |
| gzip `gzip` | 35 | 35 | 35 | 0 |

DIV-47 changed merge decisions broadly, but it did not change which return regions
`returndup` accepts. A pass that is switched off cannot have its eligibility widened.

## The isolating ablation: `returndup` is strongly net-positive

| scope | n | GED sum ON | GED sum OFF | GED-perfect ON | GED-perfect OFF |
|---|---:|---:|---:|---:|---:|
| O2 | 22,549 | 472,422 | 476,294 | 7,211 | 7,018 |
| O2-noinline | 30,313 | 332,267 | 336,151 | 10,825 | 10,601 |
| **both** | **52,862** | **804,689** | **812,445** | **18,036** | **17,619** |

Turning `returndup` off costs **7,756 aggregate GED** and **417 GED-perfect functions**. It
creates a perfect on 608 functions and destroys one on 191 — a **3.2 : 1** ratio — and
improves 3,676 functions against 2,079 worsened. The mean GED it removes where it wins (5.32)
and the mean it adds where it loses (5.92) are comparable, so the ratio is not an artifact of
trading many small wins for a few large losses.

That is the **opposite** of the DIV-18-era measurement (−976 GED-perfect, which reverted the
default on 2026-07-05), and the reason is #137: porting angr's
`ReturnDuplicatorHigh._should_duplicate_dst` const-return gate
(`returndup_is_const_ret`, `substrate/funcdata_block.rs`) dropped the over-eager
variable-return splits that caused the 2026-07-05 revert. The revert's justification no longer
describes the code.

Attributing the reported delta: of the 704 July→new wins in scope, **608 (86%) are
`returndup`'s** and vanish if it is off; of the 303 regressions, **183 (60%)** are its and are
repaired if it is off. The other 40% of the regressions belong to the rest of the 2026-08-01
wave and are out of this file's scope.

## No discriminator exists

Nine candidate gates were tested **causally** — partition the 6,857 firings by a
binary-observable feature, then report that cell's own `GED(off) − GED(on)`. A usable gate
needs a cell that is clearly negative and carries mass. Positive numbers mean `returndup`
helps that cell.

| partition | result |
|---|---|
| split count `N` (1…8) | every bucket positive (+3,233 … +487). `N ≥ 4` is −7 on perfect count but **+1,458 on GED** |
| **void vs value return** | +4,455 (void) / +3,301 (non-void) — **both positive** |
| distinct emitted return values (1…5) | +4,615 / +2,379 / +479 / +53 / +230 — all positive |
| emitted return count (2…8) | all positive |
| function contains a `switch` | +299 (switch) / +7,457 (no switch) — both positive |
| all returns identical (pure convergence) | +4,615 / +3,141 — both positive |
| body size (<20 / 20-49 / 50-99 / ≥100 lines) | +2,702 / +3,672 / +949 / +433 — all positive |
| per project (32 projects) | 24 positive; the negatives are small and unpatterned (cronie −50, gzip −28, bash/dexter/base-passwd −15 each) |
| per opt level | +3,872 (O2) / +3,884 (O2-noinline) |

The **void-return** row deserves a note because it is the most attractive-looking theory and
it is false. `returndup_is_const_ret` short-circuits to `true` for a void return ("void return
— trivially a simple const-return region"), so a void-prototyped function bypasses angr's
const-return gate entirely — and the two clearest regressions, `xmalloc` and `do_statx`, are
both void. But void firings are **more** profitable than value firings (+4,455 vs +3,301,
+213 vs +204 net perfects). Closing that escape hatch would lose more than it gains.

The same holds for the two other theories the sample cases suggest. `USBD_StdDevReq` (8
`switch` arms all returning `0`) and `DCD_HandleUSBSuspend_ISR` (one source
`if (A && B && C && D)` split into four identical `return 1;` guards) both look like "the
epilogue is pure convergence, not distinct source returns" — but *all-returns-identical* is
+4,615 and *contains-a-switch* is +299. Both cells are net wins.

**One cell is genuinely net-negative**, and only one: the split **de-structures a loop**.

| cell | n | ΔGED (off−on) | perfects gained | perfects lost |
|---|---:|---:|---:|---:|
| broke a loop (`while( true )` appears only in the ON body) | 321 | **−257** | 0 | 4 |
| loop present, kept | 2,404 | +1,590 | 39 | 7 |
| no loop | 4,132 | +6,423 | 569 | 180 |

`cleanflight nextArg` is the archetype: giving the loop-exit edge its own private return stops
the structurer from forming the `for` header, so a two-line `for` loop becomes
`while( true ) { if (!v1) return; if (*v1 != ' ') break; … }`. That is a readability loss as
well as a GED loss, independent of the metric.

So that cell was **implemented and measured**, rather than left as a theory: decline a shared
RETURN block any of whose predecessors sits on a CFG cycle (a reachability probe next to
`returndup_is_const_ret`), built on top of `36641d35` so the shipped run is its own control,
and re-scored over the same 536 slices.

| arm | GED sum | GED-perfect |
|---|---:|---:|
| `returndup` ON (shipped) | 804,689 | 18,036 |
| `returndup` OFF | 812,445 | 17,619 |
| **`returndup` ON + loop gate** | **804,480** | **18,038** |

The gate works and is directionally right — `nextArg` keeps its `for` loop — but it is
**+2 GED-perfect and −209 aggregate GED**: 0.011% of the perfect count and 0.026% of the GED
sum. It repairs 9 of the 191 perfects `returndup` destroys while forfeiting 7 of the 608 it
creates; 655 functions improve and 403 worsen. The cause of the wash is that "predecessor on
a cycle" is far broader than the cell it targets — it removes 35% of firings on coreutils
`ls` (52 → 34) and 10% on cleanflight (699 → 629), cutting into the *loop, kept* cell (+1,590
GED, +32 perfects) that pays for it.

**It was not kept.** A +2 / −209 result does not justify a new CFG walk in the pass, a stages
testcase, a spec revision and a baseline re-record — and it is nowhere near "keep the 84% of
wins, drop the 66% of regressions". The measurement is recorded here so the next person does
not have to redo it.

## Are the regressions real?

Partly not, but not in the way that would let `returndup` off the hook.

**40% of the regressions are not `returndup` at all.** Of the eight worst regressions in the
original report, only three are (`USBD_StdDevReq` 17, `do_statx` 11, `xmalloc` 9); the other
five (`rcdeviceSend5KeyOSDCableSimualtionEvent` 18, `handle_log_close` 16, `vtxGetSettings`
16, `bmp3_set_op_mode` 13, `print_table_row` 10) carry no header `return-dupe` slug and come
from elsewhere in the wave.

**At least one of those is an improvement scored as a regression.** `coreutils kill::print_table_row`
went GED 0 → 10 because the July body silently **dropped an argument**:

```c
/* July: GED 0, and missing the description argument entirely */
  strsignal(a1);
  __printf_chk(1,"%*d %-*s %s\n",a0,a1,a2,a3);

/* new: GED 10, and correct */
  v1 = (char *)strsignal(a1);
  if (!v1) v1 = "?";
  __printf_chk(1,"%*d %-*s %s\n",a0,a1,a2,a3,v1);
```

The source is `printf(..., description ? description : "?")` — a ternary inside a call
argument, whose source CFG is flatter than the recovered `if`. GED rewards the shape, and the
July shape was wrong C.

**The `returndup` regressions, by contrast, are real shape divergences, not metric artifacts.**
Read against source, every one sampled has a source that genuinely used the merged form:

| function | source shape | what `returndup` produced |
|---|---|---|
| base-passwd `xmalloc` | nested `if` + one trailing `return` | three guard clauses |
| coreutils `do_statx` | `if (ret >= 0) { … } return ret;` | early `return` + de-indented body |
| cleanflight `USBD_StdDevReq` | `switch` with `break` + one trailing `return` | `return 0;` in all eight arms |
| cleanflight `DCD_HandleUSBSuspend_ISR` | one `if (A && B && C && D)` | four identical `return 1;` guards |
| cleanflight `nextArg` | a `for` loop | `while( true )` + an early return |
| gzip `xreallocarray` | one short-circuit `if` | three guards |

So the honest framing is not "these are not really regressions". It is: `returndup` imposes
the early-return shape whether or not the source used it, it is right about **3.2 times more
often than it is wrong**, and the fact it is wrong is not observable from the binary — the
compiler emits the same merged epilogue either way.

## Conclusions

1. **Leave `returndup` alone.** No gate tightening survives measurement. The transform is
   net-positive in every partition tested, by a wide margin.
2. **Correct the record.** `docs/modes.md` said `aggressive` enables an option "known to
   regress aggregate GED on the benchmark (DIV-18)". Measured on the current build that is
   false by 7,756 GED and 417 perfect functions; the caveat is updated and the DIV-18 row now
   points here. The same superseded −976 figure still appeared in the `returndup`
   `phases.toml` `summary`/`use_when` prose (and therefore in `docs/options.md`); **DIV-54
   refreshed both**, together with `tests/fixtures/phase_catalog.json`.
3. **`returndup` cleared the default-ON evidence bar — shipped as DIV-54.** *Corrected:* this
   file originally claimed that flipping `Architecture::duplicate_shared_returns` to `true`
   gave `datatests: 675/675` with **zero assertion movement**. Re-measured while preparing
   DIV-54, the raw flip moves **16 assertions across 9 upstream datatest files** (`ccmp`,
   `orcompare`, `condexesub`, `copytrim`, `condconst2`, `bitfields2`, `partialunion`,
   `statuscmp`, `union_datatype`) — every one of them a shared-epilogue rendering the
   upstream test pins, plus `condconst` for the w10 byte-parity oracle. 675/675 PARITY OK is
   reached the way DIV-14/17/23/25/51 reached it: a per-test `option returndup off` on those
   files, with `docs/baseline.json` untouched. The decbench result (+417 / −7,756) and the
   speed budget (worst case +2.96% interleaved) are unchanged.
4. **The loop cell is closed, not open.** Declining a split whose predecessor sits on a CFG
   cycle is the one measured net-negative subpopulation, and the gate for it was built and
   scored: +2 perfects / −209 GED, not worth the pass complexity. If it is ever revisited it
   needs a *narrower* predicate than "predecessor on a cycle" — one that identifies the loop
   *header* edge specifically — because the broad version cuts into the loop firings that pay
   for it.

## Addendum (2026-08-11): a discriminator does exist, and it is not on any axis tested above

Two of this file's conclusions are true only of **aggregate GED at O2**, which is all the
2026-08-02 ablation covered. Re-run over all three optimisation levels (795 slices,
85,195 scored functions -- `docs/decbench/returndup-o0-measurement.md`), `returndup` is
+640 GED-perfect corpus-wide but **-192 at -O0**, and on perfect count at O0 **every one
of the nine partitions above has a net-negative cell**. "Net-positive in every
subpopulation examined" and "no discriminator exists" should be read as scoped to the
data they were measured on.

The discriminator the nine partitions could not find is a partition they did not test:
whether the split **costs a short-circuit fold**. `returndup` gives every predecessor of
a shared bare-epilogue RETURN its own private return; that is the source shape for a
guard clause, but the operand blocks of a short-circuit expression are predecessors of
that same block, and the shared out-target is `CollapseStructure::rule_block_or`'s entire
precondition. `ActionReturnDup` runs in fullloop's `returnsplit` group and
`collapse_conditions` runs later, so the fold is lost permanently:

| a short-circuit was lost | n | dGED | perf+ | perf- | net perf |
|---|---:|---:|---:|---:|---:|
| O0 | 2,908 | -3,538 | 48 | 713 | **-665** |
| O2 | 1,638 | +1,141 | 53 | 38 | +15 |
| O2-noinline | 2,023 | +373 | 91 | 71 | +20 |

Catastrophic at -O0, 4% of the optimized levels' net wins -- the shape of a genuine gate
rather than an opt-level switch, and the opposite of the return-value axis this file
tested (uniform / void / distinct-returns), which is +1,067 at O0 and -395 at the
optimized levels, i.e. an opt-level trade dressed as a discriminator.

**The gate was built and shipped** as `option orchain` (DIV-69), as a **pre-split CFG
predicate** rather than the post-hoc signal above -- `p8_structure/kuna_orchain.rs
(shortcircuit_shared_targets)` replays `collapse_conditions` read-only on the bblocks CFG
and mirrors `ruleBlockOr`'s own admission test. Measured on the same corpus:

| scope | n | dPerfect | dGED |
|---|---:|---:|---:|
| O0 | 32,339 | **+611** | -2,816 |
| O2 | 22,546 | -13 | +1,232 |
| O2-noinline | 30,310 | -15 | +617 |
| **all three** | **85,195** | **+583** | **-967** |

781 functions moved to an exact structural match against 198 moved off (3.94 : 1), 92% of
the +630 the post-hoc signal prices as a ceiling, at a smaller optimized-level cost than
the ceiling itself. Predicate fidelity to that signal: 2,933 of 2,951 O0 proxy positives
recognised (99.4% recall) at 126 further firings (95.9% precision), and 94.2% of firings
reproduce the `option returndup off` body byte for byte. Full bundle:
`docs/features/orchain/`.

**Two of this file's own findings are closed by it.** The loop cell of *Conclusion 4*
(`cleanflight nextArg`: the split takes the loop-exit edge's private return and the `for`
header becomes `while( true )`) is repaired as a side effect -- `nextArg` goes GED 14 to 0
at **all three** levels -- and corpus-wide the gate removes 365 `while( true )` renderings
against 53 it creates. The narrower predicate Conclusion 4 asked for turns out not to be a
loop predicate at all. And the honest residual is unchanged: where the SOURCE wrote the
guard cascade and the compiler merged it, declining the split is wrong and nothing in the
binary says so -- `coreutils factor`, this file's flagship win, goes 0 to 12.

## Reproducing

```bash
# the two arms (kuna_raw's exact invocation; `auto` selects aggressive under 500 KiB)
kuna decompile-all <stripped> --json --max-fn-seconds 120
kuna decompile-all <stripped> --json --max-fn-seconds 120 --option returndup off

# separate the two passes that share the `return-dupe` slug
kuna decompile-all <stripped> --option returndup on --option warnstyle banner | grep -c 'returndup:'
kuna decompile-all <stripped> --option returndup on --option warnstyle banner | grep -c 'taildup:'
```

Scoring reuses decbench's `GEDMetric` and the `results/full_run/ged_src/v1` source-CFG caches
directly; see `docs/decbench-loop.md` for the campaign harness.
