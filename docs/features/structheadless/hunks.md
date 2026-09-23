# structheadless: whole-corpus hunks

`hunks.py cb-off cb-on` over the 45 castbench binaries (`kuna decompile-all`,
x86-64, -O0/-O2/-O2-noinline), both arms of build e2071591f
(`--option structheadless off` against `closed`). A function is `record name
only` when it is identical once every `struct_N` number and `vN`/`aN` name is
spelled alike; `fields, declared types and casts` when every differing line
names a field or a record, is a declaration or signature, or is the same line
with its casts removed. Everything else was read.

| binary | functions | changed | record name only | fields, declared types and casts | skeleton delta | declaration-count delta | read |
|---|---:|---:|---:|---:|---:|---:|---:|
| O0/coreutils/cp.c | 530 | 71 | 40 | 29 | 0 | 1 | 1 |
| O0/coreutils/du.c | 441 | 79 | 51 | 26 | 0 | 1 | 1 |
| O0/coreutils/fmt.c | 191 | 7 | 6 | 1 | 0 | 0 | 0 |
| O0/coreutils/ls.c | 589 | 96 | 76 | 19 | 0 | 0 | 1 |
| O0/coreutils/sort.c | 479 | 57 | 44 | 12 | 0 | 0 | 1 |
| O0/coreutils/tail.c | 316 | 37 | 28 | 8 | 0 | 0 | 1 |
| O0/coreutils/wc.c | 227 | 8 | 5 | 3 | 0 | 0 | 0 |
| O0/diffutils/cmp.c | 214 | 11 | 10 | 1 | 0 | 0 | 0 |
| O0/diffutils/diff.c | 507 | 94 | 77 | 16 | 0 | 0 | 1 |
| O0/diffutils/diff3.c | 271 | 15 | 14 | 1 | 0 | 0 | 0 |
| O0/diffutils/sdiff.c | 249 | 22 | 16 | 6 | 0 | 0 | 0 |
| O0/findutils/find.c | 862 | 234 | 162 | 69 | 0 | 2 | 1 |
| O0/grep/grep.c | 642 | 137 | 81 | 53 | 0 | 1 | 2 |
| O0/tar/tar.c | 1570 | 477 | 276 | 196 | 0 | 2 | 3 |
| O2/coreutils/cp.c | 409 | 47 | 31 | 15 | 0 | 1 | 0 |
| O2/coreutils/du.c | 320 | 37 | 25 | 11 | 0 | 1 | 0 |
| O2/coreutils/ls.c | 404 | 91 | 87 | 4 | 0 | 0 | 0 |
| O2/coreutils/sort.c | 343 | 42 | 37 | 5 | 0 | 0 | 0 |
| O2/coreutils/wc.c | 182 | 4 | 3 | 1 | 0 | 0 | 0 |
| O2/diffutils/diff.c | 398 | 70 | 64 | 6 | 0 | 0 | 0 |
| O2/findutils/find.c | 658 | 178 | 91 | 84 | 0 | 2 | 1 |
| O2/grep/grep.c | 449 | 79 | 53 | 20 | 0 | 4 | 2 |
| O2/tar/tar.c | 1125 | 312 | 174 | 122 | 0 | 7 | 9 |
| O2-noinline/coreutils/cp.c | 540 | 91 | 69 | 21 | 0 | 1 | 0 |
| O2-noinline/coreutils/du.c | 451 | 55 | 36 | 18 | 0 | 1 | 0 |
| O2-noinline/coreutils/fmt.c | 201 | 7 | 6 | 1 | 0 | 0 | 0 |
| O2-noinline/coreutils/ls.c | 598 | 78 | 61 | 17 | 0 | 0 | 0 |
| O2-noinline/coreutils/sort.c | 490 | 49 | 43 | 6 | 0 | 0 | 0 |
| O2-noinline/coreutils/tail.c | 322 | 30 | 28 | 2 | 0 | 0 | 0 |
| O2-noinline/coreutils/wc.c | 233 | 8 | 5 | 3 | 0 | 0 | 0 |
| O2-noinline/diffutils/cmp.c | 214 | 8 | 7 | 1 | 0 | 0 | 0 |
| O2-noinline/diffutils/diff.c | 517 | 81 | 72 | 9 | 0 | 0 | 0 |
| O2-noinline/diffutils/diff3.c | 280 | 10 | 9 | 1 | 0 | 0 | 0 |
| O2-noinline/diffutils/sdiff.c | 249 | 19 | 13 | 6 | 0 | 0 | 0 |
| O2-noinline/findutils/find.c | 874 | 221 | 168 | 51 | 0 | 2 | 0 |
| O2-noinline/grep/grep.c | 634 | 125 | 79 | 40 | 0 | 4 | 2 |
| O2-noinline/tar/tar.c | 1585 | 463 | 261 | 197 | 0 | 3 | 2 |
| **total** | 20230 | 3450 | 2308 | 1081 | 0 | 33 | 28 |

No skeleton delta: no control-flow keyword or call moved anywhere. (The first
count showed 8, all in tar at -O2: the cast spelling `(char (*)[16])` reads as a
call token, and it became `(struct_42 *)`.)

## The 28 read by hand

| class | functions | what changed |
|---|---:|---|
| a read respelled at the same address because a local's pointer type changed | 21 | gnulib `hash_*` at -O0 in cp, du, ls, sort, tail, diff, find, grep, tar: `*(char *)(v6 + 0x10)` -> `*(char *)&v6[4]` once `v6` is the `float *` a `hash_tuning` member is (9); grep -O0 `0xeae1` `*(unsigned char *)(v + i)` -> `(unsigned char)v[i]` on a `char *` (1); tar -O2 `0x34fd0`, `0x35c30`, `0x36050`, `0x36e70`, `0x37bc0`, `0x37e80`, `0x37f00`, `0x38d40` and -O2-noinline `0x360d0`, `v[k]` <-> `*(T *)((int8)v + 8k)` (9); tar -O0 `0x43c4d`, `0x44224`, a list pointer `int8 *` -> `int8` (2, a weaker type: the function is answered by a record whose member there is an integer) |
| a callee's partial record for a value whose own reads it lacks | 1 | find -O2 `sub_c9f0`: `int *v7` -> `struct_69 *v7`; the parser-table reads at 0 and 0x10 are not in the callee's record, so they keep casts |
| the same address with the base split out | 2 | grep -O2 `0x10290`, -O2-noinline `0xf9a0`: `v16 = base + v16; *(char *)(v16 + 0x20)` -> `v6 = base; *(char *)((long)v6 + v16 + 0x20)`, the base re-read after the same call in both arms |
| an element pointer formed a line earlier | 2 | grep -O2 `0xfb20`, -O2-noinline `0xf060`: `v1 = v2 * 4; ... *(unsigned int *)(a0->f60 + v1)` -> `v1 = &a0->field_0x60[v2]; ... *v1`; the machine code (`mov 0x60(%rbp),%rax; mov (%rax,%rbx,4),%esi`) has no store between the two |
| `return NULL` spelled `return (char *)0` | 1 | tar -O2 `0x13570`, three returns; the same null, and 76 fewer casts elsewhere in the function |
| a parameter's sign follows the record member it is stored to | 1 | tar -O2-noinline `0x369f0`: `int8 a1` -> `uint8 a1`, so `a1 + 1U` prints `a1 + 1`; the same 64-bit sum |

## The 33 declaration-count deltas

| class | functions |
|---|---|
| a value typed as a record no longer shares a variable with a differently typed one (+1 `struct_N *` local) | 10: fts `fts_read` at -O0 in cp, du, find, grep (`0x12fac`, `0xbd55`, `0x1f599`, `0x1e2b9`); find -O2 `0xff90`, -O2-noinline `0xfad0`; tar -O2 `0x59300`, `0x55a50`, -O2-noinline `0x59d30`; grep -O2-noinline `0x6350` |
| gnulib `fts_stat` (-O2 and -O2-noinline in cp, du, find, grep): the `stat *` local holding the address of the embedded `struct stat` merges with the record member's `unsigned long *`, and `fstatat` receives `(stat *)v1` | 8 |
| a pointer local split off by the `void *` fallback or a member type | 5: tar -O2 `0x50640`, `0x51d40`, -O2-noinline `0x29480`, `0x4e660`, -O0 `0x740c4` |
| locals re-partitioned after a retype (a value's type now differs from what it shared a variable with, or now matches it) | 10: find -O0 `0x28423`; tar -O0 `0x6df00`; grep -O2 `0x15aa0`, `0x6a70`, `0xfd10`, -O2-noinline `0xf4a0`, `0x103f0`; tar -O2 `0x36260`, `0x375f0`, `0x37f80` |

Every one of these reads and writes the same addresses in the same order; the
declaration count moves because variable merging declines to join values of
different types. The two classes that leave a local WORSE typed -- tar's two
`int8 *` list pointers and the eight `fts_stat` `stat *` locals -- are listed in
the catalog row and in `analysis.md`. Pointer-to-integer declaration changes
over the whole corpus (`demotions.py`): 12, against 59 before the `void *`
fallback; integer-to-pointer: 43.
