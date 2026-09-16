# Type-recovery campaign — baselines

Numbers every lane in the campaign is measured against. Each section says exactly
what produced it, so a later run can disagree with it on purpose rather than by
accident.

## Canonical typesweep baseline (444 slices)

Captured 2026-09-16 with the pinned binary `/home/mahaloz/kwt/_baseline/kuna` (main `809712e9`, rebuilt 18:54),
`scripts.decbench.typesweep --baseline-only`, `DECBENCH_NO_CACHE=1`, results tree `full_run_address_2026-09-11`,
projects coreutils grep gzip diffutils bzip2 findutils tar shadow × O0/O2/O2-noinline = 444 slices, 10,748 scored functions.
Control: 96.90% agreement with the tree's published per-function verdicts (10415 agree / 333 differ — the tree was scored
by the PR#48 address-mode matcher and an older kuna, so <100% is expected). Raw rows: `/home/mahaloz/kwt/_baseline/rows.json`
(re-report with `typesweep --report-only --out <dir>`).

| slice | functions | perfect | perfect % | mean |
|---|---:|---:|---:|---:|
| ALL | 10748 | 848 | 7.89% | 0.2645 |
| opt:O0 | 4286 | 612 | 14.28% | 0.4167 |
| opt:O2 | 2394 | 42 | 1.75% | 0.1330 |
| opt:O2-noinline | 4068 | 194 | 4.77% | 0.1817 |
| proj:bzip2 | 267 | 15 | 5.62% | 0.3441 |
| proj:coreutils | 6422 | 525 | 8.18% | 0.2560 |
| proj:diffutils | 420 | 43 | 10.24% | 0.3592 |
| proj:findutils | 790 | 31 | 3.92% | 0.1444 |
| proj:grep | 247 | 37 | 14.98% | 0.3614 |
| proj:gzip | 368 | 83 | 22.55% | 0.4654 |
| proj:shadow | 686 | 20 | 2.92% | 0.2619 |
| proj:tar | 1548 | 94 | 6.07% | 0.2600 |

<details><summary>per project × opt</summary>

| slice | functions | perfect | perfect % | mean |
|---|---:|---:|---:|---:|
| proj:bzip2@O0 | 108 | 14 | 12.96% | 0.5661 |
| proj:bzip2@O2 | 56 | 0 | 0.00% | 0.1847 |
| proj:bzip2@O2-noinline | 103 | 1 | 0.97% | 0.1979 |
| proj:coreutils@O0 | 2585 | 400 | 15.47% | 0.4067 |
| proj:coreutils@O2 | 1401 | 12 | 0.86% | 0.1156 |
| proj:coreutils@O2-noinline | 2436 | 113 | 4.64% | 0.1768 |
| proj:diffutils@O0 | 165 | 25 | 15.15% | 0.5209 |
| proj:diffutils@O2 | 97 | 6 | 6.19% | 0.2377 |
| proj:diffutils@O2-noinline | 158 | 12 | 7.59% | 0.2651 |
| proj:findutils@O0 | 306 | 19 | 6.21% | 0.2164 |
| proj:findutils@O2 | 194 | 1 | 0.52% | 0.0654 |
| proj:findutils@O2-noinline | 290 | 11 | 3.79% | 0.1212 |
| proj:grep@O0 | 101 | 22 | 21.78% | 0.5141 |
| proj:grep@O2 | 52 | 1 | 1.92% | 0.1881 |
| proj:grep@O2-noinline | 94 | 14 | 14.89% | 0.2932 |
| proj:gzip@O0 | 134 | 56 | 41.79% | 0.7179 |
| proj:gzip@O2 | 99 | 10 | 10.10% | 0.3167 |
| proj:gzip@O2-noinline | 135 | 17 | 12.59% | 0.3236 |
| proj:shadow@O0 | 270 | 15 | 5.56% | 0.4837 |
| proj:shadow@O2 | 131 | 2 | 1.53% | 0.1186 |
| proj:shadow@O2-noinline | 285 | 3 | 1.05% | 0.1177 |
| proj:tar@O0 | 617 | 61 | 9.89% | 0.3930 |
| proj:tar@O2 | 364 | 10 | 2.75% | 0.1473 |
| proj:tar@O2-noinline | 567 | 23 | 4.06% | 0.1876 |

</details>

Reference (E3 canonical recompute on a 14-project sample, same metric): binja 9.64% / .346, ida 7.40% / .282, kuna 6.09% / .277, ghidra 5.30% / .243.
Pre-tooling census on the same binary (fmt/ls/sort/du O2): see `/home/mahaloz/kwt/_baseline/census0.json`.
Every campaign PR reports its typesweep block against THIS binary and THIS slice set (or a subset of it).

## Census (F3 tooling)

The four instruments landed together (`scripts.decbench.mine --metric`,
`scripts.decbench.typescore`, `scripts.pipeline.varcensus`,
`scripts.decbench.structscore`). This is their smoke run and the before-numbers
the type lanes quote.

Pins for everything below:

```bash
KUNA_BIN=/home/mahaloz/kwt/_baseline/kuna          # kuna main @809712e9
SLEIGHHOME=/home/mahaloz/github/kuna/specs
DECBENCH_NO_CACHE=1                                 # set by typescore/structscore themselves
decbench main @625e892
```

Two results trees are in play and they are not interchangeable: sections 1 and 2
read stored per-function scores, so they use `results/full_run`, the tree that
carries every rival's column and the one libcsigs was measured against. Sections
3 and 4 only need binaries and their DWARF twins, so they use the freshest tree,
`results/full_run_address_2026-09-11` (whose *stored* scores come from a
non-`main` matcher and must not be mixed with a typesweep number).

### 1. `mine --metric type_match`

`python3 -m scripts.decbench.mine --base ida --metric type_match --select 20`
over `results/full_run` (the tree that carries every rival's column):

| pool | cases | dedup groups | queue |
|---|---|---|---|
| ida-perfect / kuna-imperfect on `type_match` | 1,575 | 826 | 20 (3 artifact-suspect, 17 ranked) |
| kuna-missing (no usable score) | 1,371 functions | — | 0 whole-binary failures |

Regression control: `mine --select 30` (base angr, metric GED) writes
`cases.json`, `cases-missing.json`, `backlog.md` and `triage-queue.json`
byte-identical to the pre-change tool except for the new `"metric": "ged"`
provenance key. A non-GED pool writes `-<metric>`-suffixed siblings, so the GED
pools cannot be overwritten by a type run. A case id carries no metric either, so
`triage --case` resolves one to the GED row unless `--metric type_match` asks for
the other, and says on stderr when an id is in both — mining the type pool cannot
re-point an id that is already in use (`rescore` pins `ged` outright, since every
field it writes is a GED field).

Margin tiers L/M/S are GED sizes, so on a 0..1 metric they are empty by
construction and the whole queue comes from the artifact tier plus tier `X`.
With the margins there all but tied, `X` is ordered by how many optimisation
levels the group is imperfect at and then by group id — a coverage order, not a
severity ranking. It bites: 18 of the 20 picked groups are imperfect at all
three levels, out of a pool that is 602/826 single-level.

The degenerate-source exclusion that keeps a 1-node-CFG function out of a GED
queue does **not** apply here: 609 of the 1,575 type cases (243 of the 826
groups, 29.4%) have a degenerate source CFG, and a straight-line function's
types are exactly as scoreable as any other's. 4 of the 20 queued groups are
such cases.

### 2. `typescore` — the record's benchmark block

Control run against a block that already exists:
`typescore --record docs/features/libcsigs/record.json --project bzip2`
(all three opt levels, 6 slices, `results/full_run`).

| | n | perfect_off | perfect_on | mean_off | mean_on | improved | worse |
|---|---|---|---|---|---|---|---|
| published (libcsigs, PR-time build) | 267 | 8 | 8 | 0.2617 | 0.2712 | 8 | 0 |
| re-measured here (kuna @809712e9) | 267 | 14 | 15 | 0.3345 | 0.3441 | 8 | 0 |

The rows this was measured from carry their own provenance (kuna
`sha256 9aabcce8a7fc4fdc`, decbench main @625e892, both arm values, the results
tree); a re-report under a different binary says so and still reports the corpus
the rows cover, not the one the re-report asked for.

`n`, `improved` and `worse` reproduce exactly; the absolute levels are higher
because the pinned kuna is ~75 commits newer than the one libcsigs was measured
on. `projects` carries the projects only — `pooled` is its own top-level key, as
in the libcsigs record — so anything summing `projects` cannot double-count.

The `control` block on the same run:

| field | value | what it means |
|---|---|---|
| `identical_variables_scored_differently` | **0** | the only invalidating check: both arms handed the metric byte-identical `variables[]` for 217 functions and every one scored the same |
| `retyped_functions` | 50 | the functions the option actually rewrote — the expected signal, of which 8 improved and 0 worsened |
| `off_arm_vs_published` | 195 / 72 (73.0%) | the off arm against the tree's stored verdict. It disagrees on purpose here: the tree was scored with `libcsigs` at its default (on) and with a kuna ~75 commits older |

### 3. `varcensus` — declared variables

`python3 -m scripts.pipeline.varcensus <binary> --json`. A declaration is a
2-space-indented line ending in `;` with no `=` and no `(`; a *thunk* is a
function whose body carries kuna's `jump-as-call` marker.

coreutils `fmt` -O2, **unstripped twin** (`compiled/fmt` — this is the run the
design lane's numbers came from):

`declarations` is over every function and `non-thunk` excludes the thunks;
`single-def/single-read` below is the **non-thunk** count (`varcensus` prints
both — the all-function value for the default row is 68, the non-thunk one 31):

| config | functions | declarations | non-thunk | single-def/single-read (non-thunk) | `[16]` blobs |
|---|---|---|---|---|---|
| default | 152 | 332 | 295 | 31 | 23 |
| `--option foldcallret off` | 152 | 367 | 330 | 52 | 23 |

JSON surface on the same run: 731 variables (298 args, 433 stack). 412 of them
carry no line and no address at all (`json_useless_slots`); 326 of those are
stack slots — the framelayout ones — and the other 86 are arguments.

The design lane quotes 335 / 298 / 370 for the same three cells: its declaration
rule keeps three lines this one drops. The deltas — which is what a PR argues
from — agree exactly (+35 declarations, +21 single-def/single-read from
`foldcallret off`).

The **stripped** copies (what decbench scores) at -O2:

| binary | functions | declarations | non-thunk | single-def/single-read (non-thunk) | `[16]` blobs | JSON vars | useless stack slots |
|---|---|---|---|---|---|---|---|
| fmt | 151 | 369 | 332 | 38 | 23 | 736 | 328 |
| ls | 404 | 1,355 | 1,278 | 178 | 33 | 2,135 | 1,016 |
| sort | 343 | 1,153 | 1,084 | 132 | 31 | 1,808 | 865 |
| du | 320 | 1,038 | 968 | 103 | 27 | 1,722 | 839 |

### 4. `structscore` — structs, the TRex score and the candidate census

`structscore <stripped>... --all`, coreutils fmt/ls/sort/du, O0 and O2. Every
variable here is paired to its ground truth the way `type_match` pairs it: the
binary-wide calibration shift (8 on all eight binaries), the per-function shift
only where that one aligns nothing, and decbench's type-preferring `claim()`.
That is verified rather than asserted: replaying the same eight runs through
decbench's own `_match_structured` (typesweep's verbatim recorder) pairs all
9,593 ground-truth variables to the same kuna variable, 0 differing.

**TRex Fig. 6 prioritized score** (mean per binary; `unpaired` is GT
variables kuna's JSON surface has nothing to pair with, which is where the
never-exported register locals land). The ground-truth signedness the last step
needs comes from the twin's `DW_AT_encoding`, because decbench's form list has
the qualifier stripped out of it; `mean` is the full 0–6 score and `mean 0-5`
the same score without that step, exact for every variable. `sign?` counts the
variables that reached the last step with no DWARF encoding to judge it by —
they score 5, so `mean` is a lower bound by at most that many sixths:

| opt | binary | GT vars | unpaired | mean 0-6 | mean 0-5 | sign? | defined | is_c_pointer | pointer_level | is_c_struct | sign_ignored | c_primitive |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| O0 | fmt | 413 | 4 | **3.901** | 3.557 | 8 | 409/413 | 297/409 | 293/297 | 260/293 | 210/260 | 142/210 |
| O0 | ls | 1,859 | 39 | **3.593** | 3.312 | 33 | 1820/1859 | 1271/1820 | 1259/1271 | 1092/1259 | 715/1092 | 522/715 |
| O0 | sort | 1,510 | 25 | **3.601** | 3.332 | 24 | 1485/1510 | 1052/1485 | 1036/1052 | 871/1036 | 587/871 | 406/587 |
| O0 | du | 1,434 | 42 | **3.660** | 3.392 | 18 | 1392/1434 | 1037/1392 | 1024/1037 | 863/1024 | 548/863 | 384/548 |
| O2 | fmt | 422 | 231 | **1.732** | 1.595 | 9 | 191/422 | 138/191 | 135/138 | 114/135 | 95/114 | 58/95 |
| O2 | ls | 1,600 | 968 | **1.470** | 1.375 | 38 | 632/1600 | 473/632 | 457/473 | 388/457 | 250/388 | 152/250 |
| O2 | sort | 1,214 | 672 | **1.623** | 1.503 | 21 | 542/1214 | 389/542 | 372/389 | 294/372 | 227/294 | 146/227 |
| O2 | du | 1,177 | 673 | **1.627** | 1.511 | 22 | 504/1177 | 388/504 | 374/388 | 288/374 | 224/288 | 137/224 |

An array is scored as its element type on both sides — kuna's `char[3]` against
DWARF's `char[3]` is an exact match, not a pointer-vs-`char` miss — which is
also what `type_match` does with it (`normalize_type('char[3]')` is `{'char[3]'}`,
which no pointer form matches). Scoring the two sides by different conventions
cost 333 points net over these eight binaries, 222 of them at O0. The gross
move is larger and does not stay on one step: the old convention failed
`is_c_pointer` on 115 more variables (575 points), and the variables that now
survive that step give 242 of it back further down (`is_c_struct` 207,
`c_primitive` 53, less 18 recovered at `sign_ignored_primitive`).

`GT vars` is the denominator the mean is taken over: the ground-truth variables
of every DWARF function some kuna function resolves to, which is the set the
metric scores as well. Two subprograms can carry the same name, so those
variables are counted once per resolving address (O0: ls +3, sort +9, du +7 over
the 1,856 / 1,501 / 1,427 in the raw DWARF map), and a subprogram the DIE walk
cannot name is not counted at all (O2: fmt −1, ls −9, sort −6, du −5). Both are
under 0.6% and the identity `denominator = raw + reached twice − never reached`
holds on all eight.

Read the loss in points, not in failure counts. The score stops at the first
failing step, so a variable that first fails step *i* (0-based) scores *i* and
costs `6 - i` points, and a late step can fail more often than an early one and
still be worth a fraction of it. `structscore --trex` prints this decomposition
(`loss` in the JSON), and it reconstructs the mean exactly — on fmt O0,
4×6 + 112×5 + 4×4 + 33×3 + 50×2 + 68×1 = 867 points lost out of 413×6 = 2,478,
and (2478 − 867)/413 = 3.9007:

| opt | binary | points lost | defined ×6 | is_c_pointer ×5 | pointer_level ×4 | is_c_struct ×3 | sign_ignored ×2 | c_primitive ×1 |
|---|---|---|---|---|---|---|---|---|
| O0 | fmt | 867 | 24 (3%) | **560 (65%)** | 16 (2%) | 99 (11%) | 100 (12%) | 68 (8%) |
| O0 | ls | 4,475 | 234 (5%) | **2,745 (61%)** | 48 (1%) | 501 (11%) | 754 (17%) | 193 (4%) |
| O0 | sort | 3,623 | 150 (4%) | **2,165 (60%)** | 64 (2%) | 495 (14%) | 568 (16%) | 181 (5%) |
| O0 | du | 3,356 | 252 (8%) | **1,775 (53%)** | 52 (2%) | 483 (14%) | 630 (19%) | 164 (5%) |
| O2 | fmt | 1,801 | **1,386 (77%)** | 265 (15%) | 12 (1%) | 63 (3%) | 38 (2%) | 37 (2%) |
| O2 | ls | 7,248 | **5,808 (80%)** | 795 (11%) | 64 (1%) | 207 (3%) | 276 (4%) | 98 (1%) |
| O2 | sort | 5,314 | **4,032 (76%)** | 765 (14%) | 68 (1%) | 234 (4%) | 134 (3%) | 81 (2%) |
| O2 | du | 5,147 | **4,038 (78%)** | 580 (11%) | 56 (1%) | 258 (5%) | 128 (2%) | 87 (2%) |

**At O0 the score is lost at `is_c_pointer`** — 53–65% of every binary's loss,
against 11–19% for `sign_ignored_primitive` and 4–8% for `c_primitive`. It is
the same deficit the metric lane is chasing, and it is one-directional: of fmt's
112 failures, 110 are kuna declining to be a pointer at all (`unsigned long` 68,
`undefined8` 23, `long` 17, one each of `undefined16` and `long[11]`) where
DWARF has one — `fmt(file)` and `get_paragraph(f)` as `unsigned long` against
`char *` and `FILE *`, `get_line(end_of_word)` as `undefined8` against
`Word *`. So **signedness is not where this score is lost**: a signedness PR
should expect a small `mean` move and no `mean 0-5` move at all, and a
primitive-width PR is worth at most ~19% of the O0 gap.

**At O2 the score is lost at `defined`** — 76–80% of the loss is GT variables
kuna's JSON surface has nothing to pair with at all (968 of ls's 1,600), because
at O2 the variable usually lives in a register and is never exported. A type PR
measured on the O2 rows is mostly measuring the export surface; quote the O0
rows for a typing claim. The O0→O2 drop is the same shape TRex reports for
Ghidra (2.99→2.17).

*Not comparable to the published TRex table* (different corpus, different
pairing, different normalization) — it is a before-number for kuna, not a
cross-tool result.

**Layout / nesting F1 vs DWARF** (pointer-to-struct parameters only). kuna
synthesizes no structs today, so every F1 is 0 by construction; the number to
record is the denominator:

| opt | binary | GT ptr-to-struct params | kuna params typed as a struct | GT fields | layout F1 | GT nestings | nesting F1 |
|---|---|---|---|---|---|---|---|
| O0 | fmt | 34 | 0 | 556 | 0.0 | 86 | 0.0 |
| O0 | ls | 168 | 0 | 1,761 | 0.0 | 176 | 0.0 |
| O0 | sort | 162 | 0 | 1,779 | 0.0 | 357 | 0.0 |
| O0 | du | 174 | 0 | 1,926 | 0.0 | 391 | 0.0 |
| O2 | fmt | 23 | 0 | 402 | 0.0 | 62 | 0.0 |
| O2 | ls | 79 | 0 | 800 | 0.0 | 128 | 0.0 |
| O2 | sort | 95 | 0 | 1,113 | 0.0 | 215 | 0.0 |
| O2 | du | 100 | 0 | 1,106 | 0.0 | 245 | 0.0 |

Over both levels that is **835 pointer-to-struct parameters, 9,443 GT fields and
1,660 GT nestings** (O0 alone: 538 / 6,022 / 1,010).

**Struct-candidate census** — a base accessed at ≥2 distinct byte offsets, split
by what the evidence was. Declaration and prototype lines are excluded (the `*`
in `char *v1;` is not a dereference), an index is scaled to a byte offset by the
declared element size, and a `dat_*` global counts as a base like any other.

*Field-committed candidates* — at least one `*(T *)(B ± K)` or `B->f`, which
nothing but a field access produces:

| opt | binary | functions | with a candidate | candidates | paired to GT | already right | GT is `struct *` |
|---|---|---|---|---|---|---|---|
| O0 | fmt | 191 | 3 | 5 | 3 | 0 | 3 |
| O0 | ls | 589 | 19 | 21 | 21 | 0 | 21 |
| O0 | sort | 479 | 22 | 25 | 25 | 1 | 21 |
| O0 | du | 441 | 32 | 40 | 33 | 0 | 33 |
| **O0 total** | | **1,700** | **76 (4.5%)** | **91** | **82** | **1 (1.2%)** | **78 (95.1%)** |
| O2 | fmt | 151 | 5 | 7 | 4 | 0 | 4 |
| O2 | ls | 404 | 28 | 45 | 41 | 0 | 9 |
| O2 | sort | 343 | 14 | 16 | 10 | 0 | 10 |
| O2 | du | 320 | 22 | 30 | 14 | 0 | 14 |
| **O2 total** | | **1,218** | **69 (5.7%)** | **98** | **69** | **0 (0.0%)** | **37 (53.6%)** |

*Index-only candidates* — everything whose only evidence is `B[k]` or `*B`.
`char *s; s[0]; s[1]` is in here, and so is every other string or array walk:

| opt | binary | functions | with a candidate | candidates | paired to GT | already right | GT is `struct *` |
|---|---|---|---|---|---|---|---|
| O0 | fmt | 191 | 21 | 26 | 19 | 8 | 9 |
| O0 | ls | 589 | 105 | 169 | 132 | 36 | 85 |
| O0 | sort | 479 | 94 | 156 | 119 | 29 | 81 |
| O0 | du | 441 | 89 | 131 | 93 | 13 | 75 |
| **O0 total** | | **1,700** | **309 (18.2%)** | **482** | **363** | **86 (23.7%)** | **250 (68.9%)** |
| O2 | fmt | 151 | 28 | 42 | 11 | 1 | 6 |
| O2 | ls | 404 | 143 | 274 | 122 | 14 | 34 |
| O2 | sort | 343 | 84 | 168 | 58 | 12 | 38 |
| O2 | du | 320 | 90 | 157 | 59 | 8 | 43 |
| **O2 total** | | **1,218** | **345 (28.3%)** | **641** | **250** | **35 (14.0%)** | **121 (48.4%)** |

Three things this settles before any struct work starts:

* **The pool a synthesis pass can act on is small and it is the top table.** 91
  field-committed candidates at O0 and 98 at O2 — 5% of functions, against the
  1,123 index-only bases that are 86% of everything the census picks up. Any
  claim sized off the combined 573 / 739 is sized off array walks.
* **The match→miss channel is ~zero where it matters.** Exactly 1 of 82 paired
  field-committed candidates at O0 and 0 of 69 at O2 are variables kuna already
  types to DWARF's satisfaction, so a pass that fires on this pool spends almost
  no matches. The 23.7% / 14.0% in the second table is real but it is the price
  of retyping a `char *` that is only ever indexed — which is exactly what a
  synthesis pass must not do.
* **Field-committed candidates really are structs**: 78 of 82 paired (95.1%) at
  O0 have a pointer-to-struct ground truth, against 250 of 363 (68.9%) index-only.
  At O2 both halves fall (53.6% / 48.4%) and most of the pool stops being
  judgeable at all — 316 of 641 index-only candidates are bases kuna never puts
  on the JSON surface, because at O2 the base is usually a register.

The design lane's 637 candidates over 332 of the 1,218 O2 functions is the
combined pool with a narrower pattern set; this census also counts `B->field`, a
bare `*B` and a negative displacement, hence 739 over 377 — and then splits it.

Reproduce (about 4 minutes for all eight binaries):

```bash
KUNA_BIN=<release kuna> SLEIGHHOME=<specs> \
~/.virtualenvs/decbench/bin/python -m scripts.decbench.structscore \
  $R/{O0,O2}/coreutils/stripped/{fmt,ls,sort,du} --all --out /tmp/structscore.json
```
