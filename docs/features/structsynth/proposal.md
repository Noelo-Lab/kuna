# structsynth — the proposal, and the default it settled on

This started as a go/no-go: should kuna synthesize a structure type — a layout
that is not in the binary — from the constant-offset dereferences of a pointer
parameter? The answer was **go, opt-in**: the pass ships as
`--option structsynth off|param`, **default `off`**, and deliberately not in the
`aggressive` preset.

Everything below is measured on the branch rebased onto `520443d7`. Nothing here
is an estimate.

## The default decision

Four criteria, all measured on the rebased build:

| criterion | measured | verdict |
|---|---|---|
| worst whole-binary speed delta ≤ +5% | **+1.34%** (`fmt` O0), against an inert control that moved +2.60% | pass |
| 444-slice `type_match` perfect ≥ off − 0.1% | **959 → 958, −0.104%** (aggregate 3037.05 → 3035.55, −0.049%) | **fail** |
| `make test` / `make test-stages` with the default flipped | **675/675** datatests unmoved; 4 stage assertions move, each an intended field rendering | pass |
| no bug hunk in the whole-corpus sweep | `find` O2 `sub_f620` | **fail** |

The accuracy miss is four-hundredths of a function wide. The bug hunk is not:
with the option on, `find`'s `sub_f620` emits `goto label_f752;` and **no
`label_f752:`**, so the C does not compile. That defect is already on `main` in
nine other functions of the sweep corpus with the option off (`du` O0 0x5197,
`bzip2` 0x3df0, `chibios` 0x8001538, `sort` 0x3ec0, `grep` 0xeda0, `gzip`
0xf900, `tar` 0x13570/0x16780/0x5aa30) — the whole-corpus count is 9 off against
10 on. The type lock changes which blocks the structurer duplicates, and one more
function lands on it. Fixing the emitter is its own change; until then a pass
that reshapes block duplication cannot be a default.

## What it buys

Over the fifteen sweep binaries (5,431 functions, including two ARM32 bare-metal
firmwares and a PE32 DLL), 455 functions change and 531 parameters are retyped.
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
| new gcc errors the option introduces, 14 projects | 5, all the pre-existing `'NULL' undeclared` |

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
2. **A character buffer whose writes are not all one width becomes a record.**
   Two instances are measured: `ls` `strmode`, a `char[12]` written as ten 1-byte
   stores and one 2-byte store, and `shred` `passname`, a `char[7]` whose
   `builtin_strncpy(a1,"random",7)` is a 4-, a 2- and a 1-byte store — which the
   option turns back into three magic constants, costing the recovered string.
   Mixed widths defeat the Howard §4.5 array rule and the wide store is past the
   narrow ones rather than over them, so the layout prune has nothing to collapse.
   In the census (292 retyped parameters over `fmt`/`ls`/`sort`/`du` at O0 and
   O2) 182 are real struct pointers, 70 are `void *` — `qsort` comparators that
   do read struct fields through it — 39 are clone DIEs with no resolvable type,
   and **none** is a non-pointer. On 32-bit ARM the same census over `chibios`
   and `freertos` is 37 retyped, 29 struct pointers, 1 `void *`, 0 non-pointers.
3. **A named pointee always wins, and that is now exercised by the corpus.**
   `libctypes` (#651, default-on since #658) declares `int fileno(FILE *)`, and
   ordinary propagation carries that name back into gnulib's wrappers, so the
   nine DWARF `FILE *` an earlier round counted among the retyped parameters all
   decline: `fmt` `rpl_fseeko` @0x42d0 prints `FILE *a0` with the option on and
   `struct_2 *a0` only under `--option libctypes off`.
4. **`type_match` goes down by 0.05%** — aggregate 3037.05 → 3035.55 over 444
   slices / 10,748 functions, 6 functions worse, 1 off perfect, 0 improved. The
   metric compares pointee spellings by name, so a perfect `struct_0 *` can never
   intersect a ground-truth `WORD *`. Six of the seven are GMP limb arrays in
   `factor`; the seventh is `shred`'s `passname` above. Removing the six means
   declining every `{0:8, 8:8}` layout, which costs about a fifth of all the
   recovery.
5. **Two functions in 5,431 move a statement**, and one of them is the bug hunk
   above. The other is ARM32: `chibios` `sub_800397c`, a `for` that becomes a
   `while` with the increment hoisted — the same reads, the same exit test, the
   same increment. Thirty-two functions declare one more or one fewer variable,
   and one (`sort` `fillbuf`) loses a shared temporary and writes the same
   expression twice.
6. **Speed: nothing measurable on a whole binary.** Interleaved min-of-15 over
   six binaries: `fmt` O2 −0.32%, `ls` O2 −1.05%, `sort` O2 −0.13%, `du` O2
   +1.18%, `fmt` O0 +1.34%, `sort` O0 +0.81%, against an inert control binary
   where the option cannot fire that moved +2.60% on the same run.
7. **Names are not deduped subsumptively, and that is the readability ceiling.**
   Two functions reading overlapping but unequal subsets of one real structure
   get `struct_17` and `struct_18`; only an exact layout match shares a name. It
   happens inside one statement — `du` O0 `sub_18087` (gnulib `mb_copy`) is
   `void sub_18087(struct_29 *a0, struct_30 *a1)` whose body is
   `memcpy(&a0[1], &a1[1], a1->field_0x8)`: two names for the two ends of one
   copy of one `struct mbchar`. A binary carries a few dozen names (`fmt` 3,
   `gzip` 4, `chibios` 24, `sort` 26, `du` 32, `ls` 35, `grep` 40, `find` 53,
   `tar` 93). A reader who cannot tell two names apart also cannot tell which of
   ten claimed fields is the one in ten that is wrong, so this and item 1
   compound. Subsumptive dedup is the next change in the lane.

## What is deliberately not here

Nesting, recursion, subsumptive dedup, `structsynth all` (locals and globals),
`structarray`, and printing the recovered definitions above the function. Each is
its own change, and each is cheaper to judge once this one has a measured
baseline.

## What would re-open the default question

Three things, in the order they would matter: the emitter defect behind
`find` `sub_f620` being fixed; a rule that declines a mixed-width character
buffer without declining a real record that tiles its bytes the same way; and
subsumptive dedup, which is what makes a few dozen `struct_N` names readable.
