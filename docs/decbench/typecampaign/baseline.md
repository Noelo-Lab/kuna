# Type-recovery campaign — baselines

Numbers every lane in the campaign is measured against. Each section says exactly
what produced it, so a later run can disagree with it on purpose rather than by
accident.

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
pools cannot be overwritten by a type run.

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
| O0 | fmt | 413 | 4 | **3.884** | 3.540 | 7 | 409/413 | 294/409 | 290/294 | 260/290 | 209/260 | 142/209 |
| O0 | ls | 1,859 | 39 | **3.541** | 3.260 | 14 | 1820/1859 | 1241/1820 | 1229/1241 | 1075/1229 | 696/1075 | 522/696 |
| O0 | sort | 1,510 | 25 | **3.549** | 3.280 | 14 | 1485/1510 | 1023/1485 | 1007/1023 | 861/1007 | 577/861 | 406/577 |
| O0 | du | 1,434 | 42 | **3.631** | 3.363 | 12 | 1392/1434 | 1022/1392 | 1009/1022 | 858/1009 | 542/858 | 384/542 |
| O2 | fmt | 422 | 231 | **1.716** | 1.578 | 8 | 191/422 | 135/191 | 132/135 | 114/132 | 94/114 | 58/94 |
| O2 | ls | 1,600 | 968 | **1.437** | 1.343 | 28 | 632/1600 | 457/632 | 441/457 | 379/441 | 239/379 | 151/239 |
| O2 | sort | 1,214 | 672 | **1.605** | 1.485 | 18 | 542/1214 | 381/542 | 364/381 | 292/364 | 224/292 | 146/224 |
| O2 | du | 1,177 | 673 | **1.601** | 1.486 | 19 | 504/1177 | 377/504 | 363/377 | 285/363 | 220/285 | 136/220 |

Read the steps, not only the mean. At O0 nearly every GT variable is paired and
the loss is concentrated in one step: `is_c_struct` → `sign_ignored_primitive`
drops 260→209 on fmt and 1075→696 on ls, which is the pointer/struct gap the
metric lane is already chasing. The last step is comparatively cheap once its
ground truth is right — 142 of 209 on fmt, 68% — so **signedness is not where
this score is lost**, and a signedness PR should expect a small `mean` move and
no `mean 0-5` move at all. At O2 55–61% of GT variables have nothing to pair
with and the mean collapses; the O0→O2 drop is the same shape TRex reports for
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
