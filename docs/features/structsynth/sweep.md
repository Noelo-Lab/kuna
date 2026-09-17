# structsynth — whole-corpus `decompile-all` sweep

`kuna decompile-all <binary>` with `structsynth` off vs `param`, every changed
function classified (`docs/features/structsynth/hunks.py`), plus the two things a
body-line taxonomy cannot see: whether a statement moved anywhere
(`skeleton.py`), and whether the exported project **compiles with the layouts it
declares**. Generated from the branch build rebased onto `520443d7`; the raw arms
are not checked in (~500k lines).

The corpus is 15 binaries, deliberately not all x86-64 Linux: two ARM32
bare-metal firmwares and one PE32 DLL are in it because a type pass that only
ever ran on 64-bit gcc ELF has not been measured. Four of them — `grep`, `diff`,
`find`, `tar` — come from projects the earlier eleven did not cover, and `tar` is
the largest function count in the benchmark.

| binary | arch | functions | changed | params retyped | layouts named |
|---|---|---:|---:|---:|---:|
| fmt O2 | x86-64 ELF | 151 | 3 | 3 | 3 |
| fmt O0 | x86-64 ELF | 191 | 4 | 4 | 4 |
| ls O2 | x86-64 ELF | 404 | 70 | 114 | 35 |
| sort O2 | x86-64 ELF | 343 | 27 | 29 | 26 |
| du O2 | x86-64 ELF | 320 | 32 | 36 | 32 |
| du O0 | x86-64 ELF | 441 | 32 | 36 | 31 |
| gzip O2 | x86-64 ELF | 204 | 3 | 4 | 4 |
| bzip2 O2 | x86-64 ELF | 114 | 4 | 4 | 4 |
| grep O2 | x86-64 ELF | 449 | 43 | 48 | 40 |
| diff O2 | x86-64 ELF | 398 | 36 | 38 | 33 |
| find O2 | x86-64 ELF | 658 | 57 | 64 | 53 |
| tar O2 | x86-64 ELF | 1,125 | 106 | 114 | 93 |
| chibios O2 (`ch.elf`) | ARM32 bare metal | 398 | 30 | 28 | 24 |
| freertos O2 (`RTOSDemo.out`) | ARM32 bare metal | 126 | 8 | 9 | 9 |
| dexter O2 (`dexter.dll`) | PE32 | 109 | 0 | 0 | 0 |
| **total** | | **5,431** | **455** | **531** | |

## Does a statement move?

`skeleton.py` reduces each function to its sequence of control-flow keywords and
call tokens and compares the two arms. Over all 5,431 functions, **two** have a
skeleton delta.

### `chibios` O2 `sub_800397c` — a `for` becomes a `while`

```
off  for (v4 = **(int **)(*(int *)(v3+v9)+4); v4; v4 = *(int *)(*(int *)(*(int *)(v3+v9)+4)+v4+4)) {
on   v4 = **(int **)(*(int *)((int)v3+v10)+4);
     while (v4) { ... v4 = *(int *)(*(int *)(*(int *)((int)v3+v10)+4)+v9+4); }
```

Read by hand against both arms: the same three reads in the same order, the same
exit test, the same increment. The `for` header needs one variable to carry both
the init and the step; the type lock splits that high, and the emitter cannot
write a `for` whose step is a different variable.

### `find` O2 `sub_f620` — a `goto` to a label that is not emitted

This one is a **defect in the output**, and it is why the option ships default
off. In the `off` arm the function has two `goto label_f752;` and one
`label_f752:`. In the `on` arm one of the two predecessors is tail-duplicated,
the remaining `goto label_f752;` survives, and **the label is not emitted at
all**:

```
on    case 0x55:
        goto label_f752;                      <- no such label anywhere
      ...
      label_f770:
        v8 = (char *)sub_1bb50(v14,v9,0,v7,v13);
        sub_ece0(a0,a1->field_0x8,v8);
        return;
        v14 = (unsigned long)a3->field_0x1c;  <- the block, unlabelled and unreachable
        v13 = 1;
        v7 = 1;
        goto label_f770;
```

The statements are all still there; what is lost is the label, so the C does not
compile and the `case 0x55` path reads as going nowhere.

**It is not a new defect class.** The same shape — a `goto` whose target label
the emitter never writes — is already on `main` in nine functions of this corpus
with the option off: `du` O0 `0x5197`, `bzip2` `0x3df0`, `chibios` `0x8001538`,
`sort` `0x3ec0`, `grep` `0xeda0`, `gzip` `0xf900`, and `tar` `0x13570`,
`0x16780`, `0x5aa30`. Counting every function in both arms of all 15 binaries:
**9 with the option off, 10 with it on.** The type lock changes which blocks the
structurer duplicates, and one more function lands on the pre-existing emitter
bug. Fixing that bug is its own change; until it is fixed, a pass that reshapes
block duplication cannot be a default.

## Changed functions, by class

| class | functions |
|---|---:|
| field accesses only | 306 |
| variable renumbering only | 57 |
| variable-count delta | 32 |
| skeleton delta | 2 |
| read by hand (everything else) | 58 |

36 lines are a field access that gained a cast, which is the width disagreement
between the access and the field the layout gave that offset.

### Variable-count deltas (32)

A type-locked base changes what `merge_test_adjacent` compares, so a high can
split or fuse: 24 functions declare one more variable, 6 one fewer and 2 two
fewer.

### The 58 read by hand

Every one was read against both arms. All 58 are the same bytes through a
different spelling; the groups below sum to 58.

* **A load that was its own statement is folded into its one use** (8:
  `quotearg_n_options` in `fmt` 0x5960, `ls` 0x16700, `sort` 0x10210, `du`
  0x10130, `diff` 0x19ee0, `find` 0x23450, `grep` 0x1cc20, `tar` 0x4c550).
  `v3 = a3[1]; ... v3 | 1` becomes `v9 = a3->field_0x4 | 1`.
* **`splitdatatype` stops splitting** (10: `du` 0xdb00, `diff` 0x122a0/0x14520/
  0x17c90/0xb920, `find` 0x1cf60/0x24230, `grep` 0x1ad50, `tar` 0x454c0/0x4d650).
  Off writes `a0[0x1c] = '\0'` four times plus `a0[0x20] = '\x01'`; on writes the
  8-byte store once, because the field is 8 bytes wide. The interaction being won.
* **A repeated address becomes one temporary** (6: `tar` 0x2e4c0/0x35aa0/
  0x51e60/0x551a0, `grep` 0x7110/0x15020). `*(int8 **)(*a2 + 0x20)` tested and
  then dereferenced becomes `v2 = *(int8 **)(a2->field_0x0 + 0x20); if (v2) if (*v2)`.
  Same reads, one fewer recomputation.
* **Element rescale**: the index is divided by the element-size ratio, so the
  address is unchanged (16: `ls` 0x175c0, `du` 0xabe0, `find` 0x19e00/0x7670,
  `grep` 0x18de0, `diff` 0xbbc0, `gzip` 0x10e50, and nine `chibios` functions
  over a 0x18-wide layout).
* **A temporary holds the sum instead of the load** (4: `_obstack_begin_worker`
  in `ls` 0x14df0, `sort` 0x13fa0, `grep` 0x1b2d0, `tar` 0x4fda0).
  `v3 = *a0; *v1 = v3 + (long)v1;` becomes `v3 = a0->field_0x0 + (long)v1; *v1 = v3;`.
* **A pointer temporary stops doubling as an integer** (3: `chibios` 0x8001e0c,
  0x8001f84, 0x8001fe8).
* **A cast is added or dropped** (5: `du` 0xb550, `du` O0 0xe5e1, `find` 0x1a780,
  `grep` 0x19750, `diff` 0x8ea0 — `v2 = a0[3]; if ((char)v2)` becomes
  `v1 = a0->field_0x18;` on a `char`-declared `v1`, the truncation moving from
  the use to the declaration).
* **A signedness spelling changes** (2: `tar` 0x48e10, where a stack temporary
  becomes `uint8` and the printer writes the `(int8)` back onto the `%` operand;
  `tar` 0x36e70).
* **A shared temporary is dropped and its expression written twice** (1: `sort`
  0x7ef0. `v7 = (...)>>5; a0[2] = v7; if (v7)` becomes
  `a0->field_0x10 = (...)>>5; if ((...)>>5)`. This one is a readability *cost*.)
* **A `memmove` argument is respelled** (1: `du` O0 0xbd55.)
* **`tar` 0x14c50** (`*(char *)(V + V)` becomes `V[V]`) **and `freertos` 0x18c8**
  `vListInsert`, whose six stores are the same in the same order but whose field
  spelling the line pairing could not align.

## Does the export compile with the layouts it declares?

The body text cannot tell you whether the header and the body describe the same
object, so the check is mechanical: export the project and compile it.

**Declarations** (`offsetcheck.py`, a `_Static_assert(offsetof(struct_N,
field_0xK) == K)` per member): **0 of 352** synthesized structures across 14
exported projects declare a member the C compiler moves — 319 compiled for a
64-bit data model and the 33 of the two ARM32 projects with `-m32`. Without
`-m32` those 33 report 18 misplaced fields, because the header spells a 4-byte
target pointer `T *` and a 64-bit host compiler gives it 8 bytes. That is the
instrument, not the layout — and it is why the generated prelude now states the
target pointer width.

**Bodies** (`kuna decompile-project` + `gcc -fsyntax-only`, counting every
error): the exported project does not compile today for reasons that have
nothing to do with this option (undeclared `dat_` globals, `NULL`, jump-as-call
spellings), so the number that matters is the *delta*, and in particular
`has no member named 'field_0x…'` — the body naming a member the header does not
declare.

| binary | errors off | errors on | `has no member` off | on |
|---|---:|---:|---:|---:|
| fmt O2 | 388 | 388 | 0 | 0 |
| fmt O0 | 253 | 253 | 0 | 0 |
| ls O2 | 969 | 969 | 0 | 0 |
| ls O0 | 874 | 874 | 0 | 0 |
| du O0 | 475 | 476 | 0 | 0 |
| du O2 | 624 | 625 | 0 | 0 |
| sort O2 | 786 | 786 | 0 | 0 |
| sort O0 | 610 | 611 | 0 | 0 |
| grep O2 | 1040 | 1040 | 0 | 0 |
| gzip O0 | 679 | 679 | 0 | 0 |
| diff O2 | 1047 | 1047 | 0 | 0 |
| find O0 | 1313 | 1313 | 0 | 0 |
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
