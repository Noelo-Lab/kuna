# structsynth: a same-typed integer pair is an element pointer

A follow-up to `structsynth` (#655). The rule lives in
`p5_types/kuna_structsynth.rs (is_element_run)`, is part of `accepts`, and acts
only under `--option structsynth param`; the default arm is byte-identical to
`main`.

## 1. The symptom

`coreutils` `factor`'s `powm2` is `powm2(uintmax_t *r1m, const uintmax_t *bp,
const uintmax_t *ep, const uintmax_t *np, uintmax_t ni, const uintmax_t *one)`:
double-limb arithmetic that reads `x[0]` and `x[1]` of every pointer. The type
lattice already gets those pointers right, and `structsynth param` overwrote them:

```
$ kuna decompile-all O2/coreutils/stripped/factor --addr 0x3190 --option structsynth param
main:   unsigned long sub_3190(unsigned long *a0,struct_0 *a1,struct_1 *a2,struct_0 *a3,unsigned long a4,struct_0 *a5)
branch: unsigned long sub_3190(unsigned long *a0,unsigned long *a1,unsigned long *a2,unsigned long *a3,unsigned long a4,unsigned long *a5)
```

`struct_0` there is `{ unsigned long field_0x0; unsigned long field_0x8; }`: the
same bytes as `unsigned long[2]`, under a record name. These were six of the
seven rows `structsynth param` lost on the 444-slice `type_match` sweep
(`powm2`/`millerrabin2` at O0, O2 and O2-noinline).

## 2. The rule

The uniform-run rule already declines three or more equal-width offsets on a
grid, and two below pointer width. A pair of pointer-sized fields was kept on
width alone, because such pairs are usually records. The census below splits
them by what they hold:

* **pointer pairs**: 35 layouts, 30 of them DWARF structure pointers
  (`hash_table`'s `bucket`/`bucket_limit`, list links). Kept.
* **same-typed integer pairs**: 93 layouts: 47 structure pointers, 29 `void *`,
  12 integer pointers (all `factor`, `uintmax_t`), 5 untyped.

A pair now declines when all three hold:

1. every *raw* access, before the widest-wins merge and the layout prune, is one
   width `w` at exactly `0, w, 2w, ...` (an access the prune drops still counts
   as heterogeneous evidence);
2. every field the pass would declare has the same integer metatype (`int`,
   `uint` or unknown, so a signed/unsigned pair is kept);
3. the base already points at an integer or `undefined` of width `w`: the
   element pointer says everything the structure would.

Clause 3 is why `tar` O0 `0x6c7f5`'s fourth argument, a `char *` read as two
words, keeps its structure: that is the only same-typed integer pair the rule
leaves, and DWARF agrees it is a record.

## 3. Measured

Every number is `main` with `--option structsynth param` (d3617d64 control
build) against this branch with the same option, unless marked otherwise.

**Default arm.** `decompile-all` text and `--json` over 16 binaries (coreutils
`fmt ls sort du factor shred`, `tar`, `find`, at O0 and O2) are byte-identical
to `main`: 32 of 32 files.

**What declines** (JSON diff over the same 16 binaries, joined to the census):
743 `struct_N *` arguments become 651. All 92 that left were same-typed integer
pairs: 46 DWARF structure pointers, 29 `void *`, 12 integer pointers, 5 untyped.
No argument gained a structure, and no other shape moved. The branch census
(`arrays_census.py` on the `arrays_census.diff` build) is 654 bases: 1
same-typed integer pair, 35 pointer pairs, 13 signedness-mixed pairs, 605 other.

**`type_match`, 444 slices, 10,748 functions** (`scripts.decbench.typesweep`,
param vs off, same binary per run):

| | perfect off -> param | aggregate off -> param | off perfect | worse |
|---|---|---|---:|---:|
| main | 959 -> 958 | 3037.19 -> 3035.69 | 1 | 6 |
| branch | 959 -> 959 | 3037.19 -> 3037.05 | 0 | 1 |

The six `factor` rows are gone. The one left is `shred` O2-noinline `passname`:
a `char name[7]` filled by `builtin_strncpy(name, "random", 7)`, i.e. a 4-, a
2- and a 1-byte store. That is the documented mixed-width character-buffer
false positive, not a pair. Controls: 9504/10748 agree with the published
verdict in both runs, and functions with byte-identical variables in the two arms
scored identically (0 differ).

**Layout precision/recall/F1** (`layoutscore.py`, fields only, fmt/ls/sort/du
O0+O2, 9,443 DWARF fields):

| | TP / predicted | precision | recall | F1 |
|---|---:|---:|---:|---:|
| main | 704 / 787 | 0.8945 | 0.0746 | 0.1376 |
| branch | 684 / 767 | 0.8918 | 0.0724 | 0.1340 |

This is the cost. On those eight binaries the rule drops 20 claimed fields, and
all 20 were on a real structure at the right offset and size (`stat`'s
`st_dev`/`st_ino`, `timespec`, `hash_entry`). Those parameters score the same on
`type_match` either way, because `struct_N` never matches a DWARF name.
Counting parameters, the rule gives up 46 DWARF structure pointers to restore
12 integer pointers, and the 12 are the only ones `type_match` can score.

**TRex Fig. 6 prioritized score** (`scripts.decbench.structscore --trex`,
factor/fmt/ls/sort/du/find at O0 and O2, 17,105 DWARF variables): pooled mean
2.6604 -> 2.6597 (points lost 57,124 -> 57,136). `factor` goes up (O0 4.1726 ->
4.1861, O2 1.7190 -> 1.7330) because a `uintmax_t *` now passes the struct
step, and `ls`/`sort`/`du`/`find` go down by the same kind of step in the other
direction (`stat *` read as `long *` fails it). `fmt` does not move. The
two effects net out to -12 points over 17,105 variables.

**False-positive census** (`fprate.py census`, fmt/ls/sort/du O0+O2): 292
retyped parameters -> 274. Structure pointers 182 -> 172, `void *`/other
pointer 71 -> 63, no DWARF type 39 -> 39, non-pointer 0 -> 0.

**Whole-corpus hunks** (`arrays_hunks.py`, the same 16 binaries,
`arrays_hunks.txt`): 556 functions change. 484 are ledger renumbering only (a
pair that no longer mints a structure shifts every later `struct_N`); 71 are
the element pointer restored exactly (`struct_N *aK` becomes the integer
pointer, `aK->field_0x0` becomes `*aK`, `aK->field_0x8` becomes `aK[1]`, and
`&aK[n]` over the 16-byte structure becomes `&aK[2n]`); and 1 is read by hand.
That one is `tar` O0 `wsnode_insert` @0x41265, where a cast moves between two
stores (`a1->field_0x0 = a2; a2->field_0x8 = (int8 *)a1;` becomes
`*a1 = (int8)a2; a2->field_0x8 = a1;`). No skeleton delta and no
variable-count delta. No function in any arm has a `goto` without its label.

**Speed**: SPEED_PLACEHOLDER

## 4. The default-on evaluation (not flipped here)

Numbers for flipping `structsynth` to `param` by default on this branch. The
default is still `off`.

* `make test` with the default flipped: 675/675, PARITY OK.
* `make test-stages` with the default flipped: 2 assertions move, PEBNAMES-X86
  #6 and #8. The x86 `fs`-segment TEB base becomes `struct_0` with
  `field_0x0`/`field_0x30`, which are `NT_TIB.ExceptionList` and
  `ProcessEnvironmentBlock`: the intended field rendering. `main` with the
  option on moves the same two. ELFMAIN, which moved in the #655 evaluation, no
  longer does on `main` either.
* `type_match`: 959 -> 959 perfect, aggregate -0.14 (-0.005%), 1 worse row
  (`passname`, above).
* Whole-corpus: the `find` O2 `sub_f620` missing-label hunk that decided #655
  is gone. #674 fixed the emitter, and the dangling-`goto` count is 0 in all
  three arms over 16 binaries.
* Speed: FLIP_PLACEHOLDER
* Layout: the flip is worth +684 correctly placed fields (0 -> 0.1340 F1) on
  the eight layout binaries. `type_match` cannot see them.
