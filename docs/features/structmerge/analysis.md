# structmerge -- a record takes the fields a sibling reader proves

## The gap

`structsynth` measures one function's accesses at a time. The layout ledger then
shares one `struct_N` between two functions only when one reader's claims are a
**subset** of the other's. Two functions that read overlapping-but-different
parts of one object therefore get a name each, and each is declared to have only
the fields its own function happened to touch.

Runnable, on any machine with gcc:

```c
// two.c
struct R { char *name; long n; int flags; long tail; };
long reader_a(struct R *r) { return *r->name + r->n + r->tail; }
long reader_b(struct R *r) { return *r->name + r->n + r->flags; }
int main(int c, char **v) { struct R r = {v[0], c, c, c}; return reader_a(&r) + reader_b(&r); }
```

```
$ gcc -O1 -o two two.c && strip two
$ kuna decompile-project ./two -o out --option structsynth param
$ sed -n '/^struct struct_/,/^};/p' out/two.h; grep 'struct_. \*' out/two.h
struct struct_0 {
    char *field_0x0;
    long field_0x8;
    char field_0x10[8];
    long field_0x18;
};
struct struct_1 {
    char *field_0x0;
    long field_0x8;
    int field_0x10;
    char field_0x14[4];
};
long sub_1149(struct_0 *a0);
long sub_115d(struct_1 *a0);
```

One record, two names, and neither declaration is the record: `struct_0` has a
hole where `flags` is and `struct_1` stops before `tail`.

## The census

`docs/features/structmerge/recall.md` measures where the missing fields are, on
the eight layout builds (`fmt`, `ls`, `sort`, `du` at -O0 and -O2; 835
pointer-to-struct parameters, 9,443 DWARF fields, the layout instrument's own
denominator).

* The reading function dereferences **1,469** of the 9,443 fields, 15.6%. That
  is the per-function ceiling, and kuna already claims 817 of it correctly
  (55.6%).
* Every function taking a pointer to the same record dereferences **5,922**
  between them. 805 of the 835 parameters read a record another parameter in the
  same binary also reads.
* **23 of the 77** ground-truth records kuna types at all are given more than one
  `struct_N`; the opposite error, one `struct_N` standing for two records, is
  **1 of 136**.
* The layout prune costs 4 of the 1,469 reachable fields, so neither it nor the
  unclaimed-bytes veto it feeds is the bottleneck.

## The rule

When nothing held answers a freshly measured layout, look for a held structure
the layout was measured to AGREE with and mint the **union** of the two in its
place. The union strictly contains the held structure, so that structure is
superseded the moment the union exists and the convergence sweep moves its
readers onto it.

Agreement is the whole rule:

* every offset both claim carries the same field: offset, width, type and
  pointee spelled exactly as the ledger's field keys spell them, a self pointer
  included. "No type at all" is its own answer, never a wildcard;
* no claim of one covers bytes the other claims at a different offset;
* what they agree on is itself evidence: three shared claims, or two of which one
  is a pointer with a pointee. Every layout that reaches the ledger claims offset
  0, so the anchor is always the first word;
* the union `answers_for` both sides under the containment rule itself, so the
  growth bounds (twice the claims, four times the size), the table rule and the
  alignment-padding rule all still apply, to each reader separately;
* the reader keeps its unclaimed bytes under the union; the earlier reader is
  held to the same test by the sweep, and keeps its own record where the union
  does not fit it.

## What it is worth

Per-parameter claimed-field precision against DWARF
(`docs/features/structsynth/layoutscore.py`, the published instrument, `--option
structsynth param`):

| arm | fields only | recall | F1 | filler counted |
|---|---|---|---|---|
| off | 0.8710 (817/938) | 0.0865 | 0.1574 | 0.7861 (941/1,197) |
| **siblings** | **0.8737 (844/966)** | **0.0894** | **0.1622** | **0.7971 (974/1,222)** |

Over 177 builds in eight sets (`docs/features/structsynth/dedup_heldout.py`, the
sets the ledger's own rules were drawn from and held out against): **F1 rises on
all eight sets**, pooled true fields 53,077 -> 53,890 (+813), pooled precision
0.9330 -> 0.9312. Precision rises on three sets and falls on five, the largest
fall 0.62 points (HELDOUT2). Of the fields the merge adds, **916 of 1,105
(82.9%) are real DWARF fields**.

| set | builds | off P | on P | off F1 | on F1 |
|---|---:|---|---|---|---|
| TUNING | 8 | 0.8710 | 0.8737 | 0.1574 | 0.1622 |
| HELDOUT | 13 | 0.8976 | 0.8991 | 0.1566 | 0.1600 |
| HELDOUT2 | 22 | 0.9051 | 0.8989 | 0.1064 | 0.1081 |
| HELDOUT3 | 28 | 0.8626 | 0.8609 | 0.1035 | 0.1053 |
| HELDOUT4 | 26 | 0.9035 | 0.9019 | 0.0966 | 0.1004 |
| REVIEW1 | 24 | 0.9707 | 0.9708 | 0.2089 | 0.2105 |
| REVIEW2 | 18 | 0.9602 | 0.9587 | 0.2103 | 0.2121 |
| FRESH6 | 34 | 0.9113 | 0.9073 | 0.1126 | 0.1133 |

Record identity (`dedup_heldout.py absorb`, which asks whether a parameter
answered with a strictly larger structure was given a structure some parameter of
the same DWARF record measured): **567 absorptions over the 177 builds, 31 on a
confirmed same record, 0 on a different record**, the rest uncheckable because
the union is a shape neither reader had on the off arm.

`type_match` is blind to this by construction (a `struct_2 *` and a `struct_3 *`
score the same), and the 444-slice sweep confirms it: perfect 1,522 -> 1,522, 0
improved, 0 worsened.

## Rejected variants

All measured on the same eight builds, all below the 0.8710 precision bar:

| variant | fields-only P | recall |
|---|---|---|
| **shipped**: the union answers for both sides under `answers_for` | **0.8737** | **0.0894** |
| drop the untyped-reader clause for the union's own admission | 0.8709 | 0.0900 |
| ... and for containment in `best_of`/`is_superseded` | 0.8674 | 0.0908 |
| ... and let a typed claim answer an untyped one of the same width | 0.8661 | 0.0911 |
| asymmetric: only the reader's side is admitted, the sweep decides the other | 0.8740 | 0.0896 |

The last is a hair better than the shipped rule (+2 true fields) and was dropped:
it buys nothing for a rule that is harder to state.

## Limits

* The merge is a property of one process's ledger. `kuna decompile` numbers each
  function's records from `struct_0` again, and a `--jobs N` worker answers
  through the shard replay, which records the layout each request measured and so
  has nothing to replay for a mint of a union neither side asked for. Both behave
  as `off`, the same divergence `structsynth` already has.
* The sweep is one pass, so a union that supersedes a structure whose readers
  would themselves merge again can leave a record with two names: the ledger's
  existing non-transitivity, not a new limit.
* The ceiling the census measured is not reachable under the containment rule's
  growth bounds. A reader is never declared to hold more than twice the fields it
  measured, and most readers claim two or three. Raising that bound is what the
  three rejected variants test, and each loses precision.
