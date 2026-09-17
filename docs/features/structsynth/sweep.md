# structsynth — whole-corpus `decompile-all` sweep

`kuna decompile-all <binary>` with `structsynth` off vs `param`, every changed
function classified (`docs/features/structsynth/hunks.py`), plus the two things a
body-line taxonomy cannot see: whether a statement moved anywhere
(`skeleton.py`), and whether the exported project **compiles with the layouts it
declares**. Generated from the branch build; the raw arms are not checked in
(~200k lines).

The corpus is 11 binaries, deliberately not all x86-64 Linux: two ARM32
bare-metal firmwares and one PE32 DLL are in it because a type pass that only
ever ran on 64-bit gcc ELF has not been measured.

| binary | arch | functions | changed | params retyped |
|---|---|---:|---:|---:|
| fmt O2 | x86-64 ELF | 151 | 3 | 3 |
| fmt O0 | x86-64 ELF | 191 | 4 | 4 |
| ls O2 | x86-64 ELF | 404 | 70 | 114 |
| sort O2 | x86-64 ELF | 343 | 27 | 29 |
| du O2 | x86-64 ELF | 320 | 32 | 36 |
| du O0 | x86-64 ELF | 441 | 32 | 36 |
| gzip O2 | x86-64 ELF | 204 | 3 | 4 |
| bzip2 O2 | x86-64 ELF | 114 | 4 | 4 |
| chibios O2 (`ch.elf`) | ARM32 bare metal | 398 | 30 | 28 |
| freertos O2 (`RTOSDemo.out`) | ARM32 bare metal | 126 | 8 | 9 |
| dexter O2 (`dexter.dll`) | PE32 | 109 | 0 | 0 |
| **total** | | **2,801** | **213** | **267** |

Re-run on the tree rebased onto 12c31465. `libctypes` is on by default there and
names the `FILE *` parameters this pass used to claim, so eight functions and
eight parameters left the sweep; nothing else in it moved.

## Does a statement move?

`skeleton.py` reduces each function to its sequence of control-flow keywords and
call tokens and compares the two arms. Over all 2,801 functions, **one** function
has a skeleton delta, and it is on ARM32:

* `chibios` O2 `sub_800397c`. A `for` becomes a `while` with the increment
  hoisted into the body and a pre-header assignment added:

  ```
  off  for (v4 = **(int **)(*(int *)(v3+v9)+4); v4; v4 = *(int *)(*(int *)(*(int *)(v3+v9)+4)+v4+4)) {
  on   v4 = **(int **)(*(int *)((int)v3+v10)+4);
       while (v4) { ... v4 = *(int *)(*(int *)(*(int *)((int)v3+v10)+4)+v9+4); }
  ```

  Read by hand against both arms: the same three reads in the same order, the
  same exit test, the same increment. The `for` header needs one variable to
  carry both the init and the step; the type lock splits that high, and the
  emitter cannot write a `for` whose step is a different variable.

The eight x86-64 binaries have **no** skeleton delta, which is what the earlier
round of this sweep measured and reported as a property of the pass. It is a
property of those eight binaries.

## Changed functions, by class

| class | functions |
|---|---:|
| field accesses only | 144 |
| variable renumbering only | 30 |
| variable-count delta | 11 |
| skeleton delta | 1 |
| read by hand (everything else) | 27 |

17 lines are a field access that gained a cast, which is the width disagreement
between the access and the field the layout gave that offset.

### Variable-count deltas (11)

A type-locked base changes what `merge_test_adjacent` compares, so a high can
split or fuse. `ls` 0x14ed0, `sort` 0x14080, `du` O2 0x6b70/0xadd0/0xd9f0, `du`
O0 0xc6a5 and `chibios` 0x8001d88/0x8001dbc/0x8001ddc declare one more; `du` O0
0xb89b and 0x10c3a declare one fewer.

### The 27 read by hand

Every one of these was read against both arms. All 27 are the same bytes through
a different spelling; the counts below sum to 27.

* **A load that was its own statement is folded into its one use** (4:
  `quotearg_n_options` in `fmt` 0x5960, `ls` 0x16700, `sort` 0x10210, `du`
  0x10130). `v3 = a3[1]; ... v3 | 1` becomes `v9 = a3->field_0x4 | 1`.
* **A temporary holds the sum instead of the load** (2: `_obstack_begin_worker`
  in `ls` 0x14df0 and `sort` 0x13fa0). `v3 = *a0; *v1 = v3 + (long)v1;` becomes
  `v3 = a0->field_0x0 + (long)v1; *v1 = v3;`.
* **A shared temporary is dropped and its expression written twice** (1: `sort`
  0x7ef0. `v7 = (...)>>5; a0[2] = v7; if (v7)` becomes
  `a0->field_0x10 = (...)>>5; if ((...)>>5)`. This one is a readability *cost*.)
* **Element rescale**: the index is divided by the element-size ratio, so the
  address is unchanged (12: `ls` 0x175c0 `&a0[2]` -> `&a0[1]`, struct 0x10 wide;
  `du` 0xabe0 `&a0[0xc]` -> `&a0[1]`, 0xc x 8 = 1 x 0x60; `gzip` 0x10e50
  `&v[v * 4]` -> `&v[v * 2]`; and nine `chibios` functions over a 0x18-wide
  layout — 0x8002638, 0x800269c, 0x8002718, 0x8002754, 0x80027cc, 0x800280c,
  0x800287c, 0x8002d30, 0x8002e20, where `&a0[6]`/`&a0[7]`/`&a0[8]`/`&a0[0xc]`/
  `&a0[0x10]` become `&a0[1]`, `&a0[1].field_0x4`, `&a0[1].field_0x8`, `&a0[2]`
  and `&a0[2].field_0x10`, each arithmetically the same byte).
* **A cast is added or dropped** (2: `du` 0xb550
  `(int *)*(long *)(v2+0x10)` -> `*(void **)(v2+0x10)`; `du` O0 0xe5e1
  `*(void **)((long)v2+8)` -> `(void *)v2[1]`).
* **`splitdatatype` stops splitting** (1: `du` 0xdb00. Off writes
  `a0[0x1c] = '\0'` four times plus `a0[0x20] = '\x01'`; on writes the 8-byte
  store once. The interaction being won.)
* **A `memmove` argument is respelled** (1: `du` O0 0xbd55.
  `&((char *)(v + v))[1]` becomes `&v[1]` with the base assignment split out.)
* **A pointer temporary stops doubling as an integer** (3: `chibios` 0x8001e0c,
  0x8001f84, 0x8001fe8. `v2 = a0[2]; a0[2] = v2 + 1; if (v2+1<=0)` becomes
  `v2 = a0->field_0x8 + 1; a0->field_0x8 = v2; if (v2 <= 0)`.)
* **Field spelling the line-pairing could not align** (1: `freertos` 0x18c8
  `vListInsert`. Same six stores in the same order; `a1[4] = (unsigned int)a0`
  becomes `a1->field_0x10 = &a0->field_0x0`, and `&a0[2]` becomes
  `&a0->field_0x4[4]` — an index into the filler member, offset 4 + 4 = 8.)

## Does the export compile with the layouts it declares?

The body text cannot tell you whether the header and the body describe the same
object, so the check is mechanical: export the project and compile it.

**Declarations** (`offsetcheck.py`, a `_Static_assert(offsetof(struct_N,
field_0xK) == K)` per member): **0 of 352** synthesized structures across 14
exported projects declare a member the C compiler moves. The two ARM32 projects
need `--m32`; without it 18 of 33 of their structures "fail", because the header
spells a 4-byte target pointer `T *` and a 64-bit host compiler gives it 8
bytes. That is the instrument, not the layout.

**Bodies** (`kuna decompile-project` + `gcc -fsyntax-only`, counting every
error): the exported project does not compile today for reasons that have
nothing to do with this option (undeclared `dat_` globals, `NULL`, jump-as-call
spellings), so the number that matters is the *delta*, and in particular
`has no member named 'field_0x…'` — the body naming a member the header does not
declare.

| binary | errors off | errors on | `has no member` off | on |
|---|---:|---:|---:|---:|
| fmt O2 | 374 | 374 | 0 | 0 |
| fmt O0 | 246 | 246 | 0 | 0 |
| ls O2 | 911 | 911 | 0 | 0 |
| ls O0 | 805 | 805 | 0 | 0 |
| du O0 | 409 | 410 | 0 | 0 |
| du O2 | 573 | 574 | 0 | 0 |
| sort O2 | 759 | 759 | 0 | 0 |
| sort O0 | 594 | 595 | 0 | 0 |
| grep O2 | 1015 | 1015 | 0 | 0 |
| gzip O0 | 660 | 660 | 0 | 0 |
| diff O2 | 893 | 893 | 0 | 0 |
| find O0 | 1203 | 1203 | 0 | 0 |
| chibios O2 | 1144 | 1146 | 0 | 0 |
| freertos O2 | 327 | 327 | 0 | 0 |

The five extra errors (`du` O0, `du` O2, `sort` O0, `chibios` x2) are all
`'NULL' undeclared`: the generated header never defines `NULL`, and the retyped
pointer comparison reaches that spelling in five more functions. No new error
class appears.

The earlier round of this prototype had **44** `has no member named 'field_0x…'`
errors over 10 of these binaries, `fmt` O2 among them: offsets in a hole
(`&a3->field_0x8` against an `undefined1 _pad8[32]`) and offsets past the end.
`tests/stages/structsynth-hole-member.xml` is the regression probe.
