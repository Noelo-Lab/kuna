# Where the 8,626 unclaimed ground-truth fields are

Measured on kuna 31ac2b26e (`#711` has since raised both arms; the option's own
effect is unchanged -- see `analysis.md`). `structsynth`'s per-parameter layout
precision is 0.8710 and its recall is 0.0865: of the
9,443 DWARF fields behind the 835 pointer-to-struct parameters of the eight layout builds
(`fmt`/`ls`/`sort`/`du` at -O0 and -O2), 817 are claimed correctly and 8,626 are never
claimed at all. This is a census of the 8,626, measured rather than argued.

## How it was measured

Three views of the same eight builds are joined per parameter
([`census.py`](census.py), summary [`census-report.txt`](census-report.txt)):

* the DWARF ground truth for every pointer-to-struct parameter — `structscore.dwarf_functions`
  and `struct_layout`, the layout instrument's own denominator (835 parameters, 9,443 fields,
  reproducing the published figure exactly);
* kuna's `decompile-all --json` plus a `decompile-project` header, which says which of those
  parameters kuna typed as a record and with which members — `structscore.measure_layout`
  field for field, reproducing 817/938 fields-only and 941/1,197 filler-counted;
* an `SSCENSUS` stderr line per candidate base from a census build of the pass
  ([`census.patch`](census.patch), never committed to the engine), which says what the
  function actually dereferenced *before* the layout prune, what the prune left, and the exact
  decline reason. A base is joined to an argument index through its register storage, because
  the `FuncProto` has no store yet when the pass runs.

"Touched" below means: some pre-prune access through that parameter overlaps the byte range of
that ground-truth field. A field nobody reads cannot be recovered from the reader's own
evidence, so this is the honest ceiling.

## The two ceilings

| | fields | of 9,443 |
|---|---:|---:|
| the reading function dereferences it (per-function ceiling) | **1,469** | 15.6% |
| some function taking a pointer to the same record dereferences it (cross-function ceiling) | **5,922** | 62.7% |
| … counting only readers `structsynth` accepts today | 3,818 | 40.4% |
| … and only on the 200 parameters kuna types today | **1,451** | 15.4% |
| kuna claims correctly today | 817 | 8.7% |

**84.4% of the ground-truth fields are invisible to the function that holds the parameter.**
Against the per-function ceiling kuna already recovers 55.6% of what is there (817 of 1,469),
which is a very different picture from "recall 0.087". The headroom is not inside one
function: it is that **805 of the 835 parameters read a record some other parameter in the
same binary also reads** (131 distinct records, 101 with two or more readers, one with 35),
and each reader measures a different part of it.

## Where the unclaimed fields sit

Per untyped parameter, the reason `structsynth` declined, with the ground-truth fields behind
it, the fields that parameter's own function dereferences, and the fields its accepted
siblings dereference:

| decline reason | params | GT fields | touched here | touched by an accepted sibling |
|---|---:|---:|---:|---:|
| `gate_unsettled` (the pass never ran: the type lattice had not settled) | 217 | 2,514 | 0 | 940 |
| `no_base` (no constant-offset dereference of the parameter at all) | 140 | 2,222 | 0 | 220 |
| `fewer_than_two_offsets` | 91 | 806 | 90 | 484 |
| `no_offset_zero` | 59 | 766 | 235 | 406 |
| `array_shaped` (Howard's uniform-run rule) | 36 | 239 | 205 | 72 |
| `named_composite` (a libc or DWARF pointee already won) | 27 | 673 | 83 | 18 |
| `type_lock` | 20 | 200 | 74 | 140 |
| `not_a_pointer` | 13 | 65 | 0 | 0 |
| `gate_exceeded` (type recovery gave up) | 12 | 131 | 0 | 70 |
| `dynamic_offset` | 10 | 85 | 8 | 8 |
| `phi_reached` | 9 | 80 | 53 | 5 |
| `spacebase_or_global` | 1 | 0 | 0 | 4 |
| **total untyped** | **635** | **7,781** | **748** | **2,367** |

And on the 200 parameters kuna does type: the function dereferences 721 ground-truth fields,
717 of them survive the prune, and **37** of those are still not claimed at the end.

## What this rules out

* **The layout prune is not the bottleneck.** It costs 4 of the 1,469 reachable fields
  (1,469 touched → 1,465 surviving). The unclaimed-bytes veto it feeds (#675) is a
  spelling guard over those same bytes, so it cannot be large either.
* **Holes are not the bottleneck.** Filler is 259 of the 1,197 members; the fields-only and
  filler-counted arms differ by 124 true positives, not thousands.
* **Per-function evidence is not the bottleneck either** — it is a real ceiling at 1,469, and
  kuna is already at 55.6% of it. Every rule that loosens a single function's evidence
  (`array_shaped`, `phi_reached`, `dynamic_offset`) is bounded by 266 fields *in total*, and
  each of them exists because loosening it was measured to produce wrong layouts.

## What it points at

The one lever whose ceiling is large is **sharing what the readers of one record already
prove**. The ledger unifies two layouts only when one *contains* the other; two readers that
measured overlapping-but-different parts of the same record get a name each and each keeps only
its own fields. The census says exactly how often that happens:

* **23 of the 77 ground-truth records kuna types at all are given more than one `struct_N`.**
* The reverse error is rare: **1 of 136 `struct_N` is shared by two different ground-truth
  records**, so the current unification is precise and there is room to spend.
* Merging the claims of the readers `structsynth` already accepts, into the records it already
  installs, has a ceiling of **1,451** correct fields against today's 817 — recall 0.0865 →
  0.1537 — without typing a single new parameter.

That ceiling is a ceiling on the evidence, not on what the ledger's rules will hand out.
The containment rule declares a reader to hold at most twice the fields it measured and at
most four times its bytes, and most readers claim two or three fields, so a reader of a
ten-field record is answered with at most six of them however many siblings prove the rest.
`structmerge siblings` spends what is reachable under those bounds -- 27 more claimed
fields, precision 0.8710 -> 0.8737 on this base -- and `docs/features/structmerge/analysis.md` records
what relaxing each bound was measured to cost.

Two smaller levers sit behind the same mechanism and are worth measuring after it:
`fewer_than_two_offsets` (91 parameters whose siblings touch 484 fields) and `no_offset_zero`
(59 parameters, 406). Both ask the same question — *may a sibling's claim stand in for evidence
this reader does not have?* — and both are worth less than the merge itself.

## Per build

| build | params | typed | GT fields | correct | touched here | cross-function ceiling |
|---|---:|---:|---:|---:|---:|---:|
| fmt O0 | 34 | 4 | 556 | 11 | 21 | 179 |
| fmt O2 | 23 | 2 | 402 | 6 | 24 | 122 |
| ls O0 | 168 | 44 | 1,761 | 193 | 266 | 1,465 |
| ls O2 | 79 | 25 | 800 | 106 | 197 | 604 |
| sort O0 | 162 | 36 | 1,779 | 152 | 295 | 1,069 |
| sort O2 | 95 | 21 | 1,113 | 95 | 210 | 628 |
| du O0 | 174 | 39 | 1,926 | 139 | 258 | 1,192 |
| du O2 | 100 | 29 | 1,106 | 115 | 198 | 663 |
