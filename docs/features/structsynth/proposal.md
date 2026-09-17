# structsynth — proposal

**Status: draft. This is a working prototype with measured numbers, opened for a
go/no-go rather than for merge.**

## The question

Should kuna synthesize a structure type — a layout that is not in the binary —
from the constant-offset dereferences of a pointer parameter, and ship it as
`--option structsynth param`, default off?

Everything below is measured on the branch. Nothing here is an estimate.

## What it buys

Over the eight x86-64 sweep binaries (2,168 functions), 175 functions change and
230 parameters are retyped; over the full 11-binary corpus, which adds two ARM32
bare-metal firmwares and a PE32 DLL, 213 of 2,801 functions and 267 parameters.
Every retyped parameter is one the engine already believed was a pointer.

Against DWARF, restricted to parameters that really are struct pointers
(`structscore --layout`, the RecStruct §4.1 Task-1 score), counting the fields
the pass **claims** — not the `undefined1` filler that exists only so the printer
cannot name an undeclared offset:

| | value |
|---|---|
| claimed fields that land on a real field, same offset and size | **704 of 787 = 0.894 precision** |
| fields of the real structures recovered | 704 of 9,443 = 0.075 recall |
| layout F1 | 0.0 on every binary before, 0.029 – 0.232 after |
| counting filler as a claim as well | 798 of 1,021 = 0.782 |

Read precision, not F1: the before-F1 is 0 by construction because kuna
synthesized no structures at all, so the F1 gain is a tautology dressed as a
result. **About 1 in 10 claimed fields is at an (offset, size) DWARF does not
have.** Recall is structurally low and always will be — the layout only has the
fields the one function touched.

The export round-trips at the level that matters, which is the body, not the
declaration:

| | |
|---|---|
| exported projects whose C bodies name a member the header does not declare | **0 of 14** (was 44 errors over 10) |
| synthesized structures declaring a member the compiler moves | **0 of 352**, 2,174 members |
| new gcc errors the option introduces, 14 binaries | 5, all the pre-existing `'NULL' undeclared` |

In the C, `ls` O2's `hash_rehash` @0x10f10 recovers nine of the ten members of
gnulib's `struct hash_table` — `bucket`, `bucket_limit`, `n_buckets`,
`n_buckets_used`, `tuning`, `hasher`, `comparator`, `data_freer` and
`free_entry_list`, each at its real offset, with only `n_entries` left as
filler.

## What it costs

1. **About 1 in 10 claimed fields is not a DWARF field at that offset and size.**
   704 of 787 land, 83 do not. A miss is usually a field read at a narrower width
   than it was declared, an interior member of an embedded structure, or a read
   of padding. The layouts are internally sound — every exported header compiles
   with each `offsetof` equal to the member name, and no body names a member the
   header does not declare — but they are a reading aid, not a fact.
2. **One ordinary false positive in 292 retyped parameters, and that is a count,
   not a rate.** `ls` `strmode`, a `char[12]` written as ten 1-byte stores and
   one 2-byte store: mixed widths defeat the Howard §4.5 array rule, and the wide
   store is past the narrow ones rather than over them, so the layout prune has
   nothing to collapse. The census can only adjudicate a ground truth that is not
   a pointer to an aggregate; 70 more are `void *` — `qsort` comparators that do
   read struct fields through it — which the instruments count against us, 182
   are real struct pointers, 39 are clone DIEs with no resolvable type, and
   **none** is a non-pointer. On 32-bit ARM the same census over `chibios` and
   `freertos` is 37 retyped, 29 struct pointers, 1 `void *`, 0 non-pointers.
3. **The `FILE *` class is gone, and the default flip is what removed it.** An
   earlier round counted nine DWARF `FILE *` among the retyped parameters and
   reasoned that `libctypes` could not see them because they are gnulib wrappers
   with no import prototype. The name does not come from the wrapper's own
   prototype — it comes from `int fileno(FILE *)`, propagated back from the call
   site — so with `libctypes` on by default (#658) all nine decline, and the
   census is 292 rather than 301. `fmt` `rpl_fseeko` @0x42d0 prints `FILE *a0`
   with the option on and `struct_0 *a0` only under `--option libctypes off`.
   The ordering claim stands: a pass that named `FILE *` after P5 would meet a
   type lock and lose.
4. **`type_match` goes down by 0.05%** — aggregate 1760.25 → 1759.34 over 214
   slices / 5,298 functions, 4 functions worse, 1 off perfect, 0 improved
   (re-run on the rebase; the `libctypes` flip raised the baseline from 1696.15
   and left this delta where it was). The metric compares pointee spellings by
   name, so a perfect `struct_0 *` can never intersect a ground-truth `WORD *`. The 4 are GMP limb arrays in `factor`.
   Removing them means declining every `{0:8, 8:8}` layout, which costs about a
   fifth of all the recovery, so the prototype takes the loss and stays
   default-off and out of `aggressive`.
5. **One function in 2,801 moves a statement**, and it is on ARM32: `chibios`
   `sub_800397c`, a `for` that becomes a `while` with the increment hoisted.
   Read by hand: the same reads, the same exit test, the same increment. Eleven
   functions declare one more or one fewer variable, and one (`sort` `fillbuf`)
   loses a shared temporary and writes the same expression twice.
6. **Speed: nothing measurable on a whole binary.** Interleaved min-of-15, three
   runs. Against an inert control binary where the option cannot fire (which
   moved −1.83%, +5.00% and +0.66%), every whole-binary case sits inside the
   control's own movement. The one-function case is 163 ms and moved +0.20%,
   −0.86% and +2.84% — small enough that the sign is not stable.
7. **Names are not deduped subsumptively, and that is the readability ceiling.**
   Two functions reading overlapping but unequal subsets of one real structure
   get `struct_17` and `struct_18`; only an exact layout match shares a name. It
   happens inside one statement — `du` O0 `sub_18087` (gnulib `mb_copy`) is
   `void sub_18087(struct_29 *a0, struct_30 *a1)` whose body is
   `memcpy(&a0[1], &a1[1], a1->field_0x8)`: two names for the two ends of one
   copy of one `struct mbchar`. A binary carries a few dozen names (`fmt` 3,
   `gzip` 4, `chibios` 24, `sort` 26, `du` 32, `ls` 35). A reader who cannot
   tell two names apart also cannot tell which of ten claimed fields is the one
   in ten that is wrong, so this and item 1 compound. Subsumptive dedup is the
   next change in the lane.

## What is deliberately not here

Nesting, recursion, subsumptive dedup, `structsynth all` (locals and globals),
`structarray`, and printing the recovered definitions above the function. Each is
its own change, and each is cheaper to judge once this one has a measured
baseline.

## The decision

- **Go**: land it default-off as `off|param`, accept the 0.05% `type_match` loss
  as the price of a lane the metric cannot see, and sequence the follow-ups.
- **No-go**: the prototype is self-contained — one module, one action, one
  `Funcdata` flag, one `funcdata.rs` wrapper — and reverts cleanly.
- **Go with a narrower rule**: decline every uniform-width layout including
  `{0:8, 8:8}`. That takes the `type_match` loss to zero and costs about a fifth
  of the recovered parameters. Note that it addresses neither defect above:
  `strmode` and the 83 unmatched fields are both mixed-width.
