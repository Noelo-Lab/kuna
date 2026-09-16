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
construction and the queue is the ranked remainder plus the artifact tier.

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
on. Harness control on the same run: 266 functions had the same variable count in
both arms and 8 of them scored differently — i.e. the option changed types
without changing the variable count, which is what `libcsigs` does.

### 3. `varcensus` — declared variables

`python3 -m scripts.pipeline.varcensus <binary> --json`. A declaration is a
2-space-indented line ending in `;` with no `=` and no `(`; a *thunk* is a
function whose body carries kuna's `jump-as-call` marker.

coreutils `fmt` -O2, **unstripped twin** (`compiled/fmt` — this is the run the
design lane's numbers came from):

| config | functions | declarations | non-thunk | single-def/single-read | `[16]` blobs |
|---|---|---|---|---|---|
| default | 152 | 332 | 295 | 31 | 23 |
| `--option foldcallret off` | 152 | 367 | 330 | 52 | 23 |

JSON surface on the same run: 731 variables (298 args, 433 stack), 326 of them
framelayout slots with no line and no address.

The design lane quotes 335 / 298 / 370 for the same three cells: its declaration
rule keeps three lines this one drops. The deltas — which is what a PR argues
from — agree exactly (+35 declarations, +21 single-def/single-read from
`foldcallret off`).

The **stripped** copies (what decbench scores) at -O2:

| binary | functions | declarations | non-thunk | single-def/single-read | `[16]` blobs | JSON vars | useless stack slots |
|---|---|---|---|---|---|---|---|
| fmt | 151 | 369 | 332 | 38 | 23 | 736 | 328 |
| ls | 404 | 1,355 | 1,278 | 178 | 33 | 2,135 | 1,016 |
| sort | 343 | 1,153 | 1,084 | 132 | 31 | 1,808 | 865 |
| du | 320 | 1,038 | 968 | 103 | 27 | 1,722 | 839 |

### 4. `structscore` — structs, the TRex score and the candidate census

`structscore <stripped>... --all`, coreutils fmt/ls/sort/du, O0 and O2.

**TRex Fig. 6 prioritized score** (0–6, mean per binary; `unpaired` is GT
variables kuna's JSON surface has nothing to pair with, which is where the
never-exported register locals land):

| opt | binary | GT vars | unpaired | mean | defined | is_c_pointer | pointer_level | is_c_struct | sign_ignored | c_primitive |
|---|---|---|---|---|---|---|---|---|---|---|
| O0 | fmt | 413 | 5 | **3.719** | 408/413 | 292/408 | 288/292 | 259/288 | 207/259 | 82/207 |
| O0 | ls | 1,859 | 44 | **3.411** | 1815/1859 | 1220/1815 | 1208/1220 | 1064/1208 | 671/1064 | 363/671 |
| O0 | sort | 1,510 | 28 | **3.386** | 1482/1510 | 995/1482 | 980/995 | 849/980 | 553/849 | 254/553 |
| O0 | du | 1,434 | 42 | **3.513** | 1392/1434 | 1010/1392 | 997/1010 | 855/997 | 534/855 | 249/534 |
| O2 | fmt | 422 | 234 | **1.654** | 188/422 | 136/188 | 133/136 | 112/133 | 93/112 | 36/93 |
| O2 | ls | 1,600 | 976 | **1.395** | 624/1600 | 452/624 | 436/452 | 374/436 | 234/374 | 112/234 |
| O2 | sort | 1,214 | 678 | **1.544** | 536/1214 | 375/536 | 359/375 | 286/359 | 217/286 | 101/217 |
| O2 | du | 1,177 | 679 | **1.554** | 498/1177 | 371/498 | 357/371 | 281/357 | 216/281 | 106/216 |

Read the steps, not only the mean: at O0 nearly every GT variable is paired and
the score is lost at the last two steps (the primitive itself), while at O2 55–61%
of GT variables have nothing to pair with at all and the mean collapses. The
O0→O2 drop is the same shape TRex reports for Ghidra (2.99→2.17).

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

**Struct-candidate census** (a base dereferenced at ≥2 distinct constant offsets):

| opt | binary | functions | with a candidate | candidates | paired to GT | already right | GT is `struct *` |
|---|---|---|---|---|---|---|---|
| O0 | fmt | 191 | 24 | 31 | 23 | 9 | 11 |
| O0 | ls | 589 | 122 | 198 | 149 | 37 | 96 |
| O0 | sort | 479 | 112 | 191 | 141 | 27 | 95 |
| O0 | du | 441 | 112 | 188 | 129 | 15 | 100 |
| **O0 total** | | **1,700** | **370 (21.8%)** | **608** | **442** | **88 (19.9%)** | **302 (68.3%)** |
| O2 | fmt | 151 | 29 | 50 | 13 | 2 | 10 |
| O2 | ls | 404 | 161 | 325 | 160 | 15 | 42 |
| O2 | sort | 343 | 89 | 191 | 67 | 14 | 47 |
| O2 | du | 320 | 100 | 204 | 72 | 9 | 56 |
| **O2 total** | | **1,218** | **379 (31.1%)** | **770** | **312** | **40 (12.8%)** | **155 (49.7%)** |

Two things this settles before any struct work starts:

* **The O0 pool really is smaller** — 21.8% of functions carry a candidate at O0
  against 31.1% at O2, and 0.36 candidates per function against 0.63. That was the
  design lane's guess and it holds. What flips the other way is how much of the
  pool is *judgeable*: 442 of 608 O0 candidates (72.7%) pair with a ground-truth
  variable against 312 of 770 (40.5%) at O2, because at O2 the base is often a
  register kuna never exports.
* **The match→miss channel is small but real**: 88 of 442 paired candidates at O0
  (19.9%) and 40 of 312 at O2 (12.8%) are variables kuna *already* types to
  DWARF's satisfaction, so a synthesis pass that fires on them spends a match. On
  the other side, 68.3% (O0) and 49.7% (O2) of paired candidates really are
  pointer-to-struct in the source. The design lane's 637 candidates / 332 of 1,218
  O2 functions is the same measurement with a narrower pattern set; this one also
  counts `B->field` and a bare `*B`, hence 770 / 379.

Reproduce (about 4 minutes for all eight binaries):

```bash
KUNA_BIN=<release kuna> SLEIGHHOME=<specs> \
~/.virtualenvs/decbench/bin/python -m scripts.decbench.structscore \
  $R/{O0,O2}/coreutils/stripped/{fmt,ls,sort,du} --all --out /tmp/structscore.json
```
