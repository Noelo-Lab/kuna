# castindex: a variable index and a byte-pointer difference stay in pointer terms

## 1. The problem

`castarith` (#722) keeps a pointer plus a *constant* in pointer terms. The same
`INT_ADD` with a *variable* addend, and every `INT_SUB` of two pointers, still reach
`ActionSetCasts` as integer arithmetic: `(long)` on the pointer, and a cast back on
the sum. On main `b3878d32e` (`kuna decompile-all <bin> --addr <a>`):

```c
// coreutils sort -O0 mergefps (sub_a5c0)            IDA at the same address
*(long *)((long)v7 + v19 * 8) = v10 + -0x20;         v39[v27] = v48 - 32;
v3 = *(unsigned long *)((long)a5 + v19 * 8);         a6[v27]
// coreutils ls -O0 print_long_format (sub_c6c8)
sub_5453(v14,(long)v24 - (long)v14);                 sub_5453(v52, sa - v52);
// grep -O0 (sub_5d23)
v2 = memrchr(a1,v13,(long)v20 - (long)a1);           memrchr(a2, v15, &string[v33] - a2);
```

## 2. Census (STEP 1): the `(long)v` residue

Instrument: the castbench counter over the 4,815 functions kuna and IDA both emit (45
binaries x O0/O2/O2-noinline), main arm `/home/mahaloz/kwt/castbench/main-b3878d32e`.
Every 64-bit integer cast of a bare variable (`(i64|u64)<var>`), classified by the
operand's declared type and its context (`census.py`). This key is wider than the
4,563 of results.md I.1, which counted one shape key; the classes are what matter.

| class | main | castindex | IDA's form | reading |
|---|---:|---:|---|---|
| pointer - pointer (`(long)p - (long)q`, either side) | 852 | 469 | `sa - v52` | **(c)** same byte pointee: converted; wider ones stay (a `>> 3` follows) |
| pointer + variable (`(long)p + v`) | 516 | 352 | `*((_BYTE *)a1 + v2)` | **(a)** byte element: converted; a byte offset read wider stays |
| pointer + scaled variable (`(long)p + v * S`) | 125 | 69 | `v39[v27]` | **(a)** `S == sizeof(T)`: converted; a record stride stays |
| pointer in a larger sum (other addend, rhs of `+`) | 498 | 361 | | the same two shapes nested |
| pointer + constant that `castarith` declines | 487 | 487 | | `table[i]` bases, non-whole offsets, a `void *` leaving the function |
| pointer used as an integer (assigned, passed, masked, compared) | 743 | 743 | `(unsigned int)&v7` | no pointer arithmetic to keep |
| int index scaled into an integer base or a constant table | 674 | 674 | `16LL * i + a1`, `off_2B200[v7]` | the base is an integer (IDA types it `long long` too) or a global the `castglobals` lane names; IDA's `16LL *` moves the width onto a literal |
| int widened for arithmetic, globals, other | 2,186 | 2,098 | | integer conversions |
| **total** | **6,081** | **5,253** | **2,761** | |

The engine view of the same ops (an env-instrumented build, every `INT_ADD` with one
pointer-typed non-constant operand and a non-constant addend, and every `INT_SUB` of
two pointers, shared set):

| verdict | ops | why |
|---|---:|---|
| index converted | 366 | a byte element indexed by the variable, or `x * sizeof(T)` |
| difference converted | 223 | two pointers to the same byte type |
| scale is not the element | 384 | `p + v - 8` read at 8 bytes, `p + i * 16 + 8`: no element divides it |
| difference of other pointees | 255 | `long *`, `void *`, mixed: the element difference is not the byte difference, and C does not subtract `void *` or two different types |
| sum used as an integer | 155 | including a sum that is another pointer's subscript |
| `void *` sum leaving the function | 48 | `castarith`'s rule: its `char *` would reach the prototype votes |
| address shared by several accesses | 41 | `castarith`'s rule |
| two pointer operands | 16 | `(long)p + (long)q` |
| unsettled store, result not `ptrdiff_t` | 12 | |
| saves no cast | 4 | the consumer needs a cast of its own anyway |

Shape (b), a record stride: a base already typed as a record gets `PTRADD`/`PTRSUB`
from P5 and never reaches this pass. The 19 aggregate targets on the shared set are
`(struct_N *)(p + i * 0x38)` sums whose base is `void *`: the record belongs to the
sum, not the base, so `p[i].field` would invent the base's record. Census only.

The brief's other witnesses: sort -O0 `*(long *)(a0 + (long)v3 * 0x10 + 8)` has an
integer base (`long a0`; IDA `16LL * i + a1 + 8`), and ls -O0 `free(*(void **)((long)v1
+ (long)v2 * 0x10 + 8))` is a 16-byte record stride read at 8 bytes: both stay. The odd
ls -O0 `*(char *)((long)a0 + (long)&v1[-1])` (`sub_116ec`, mfile_name_concat) is
`dir[dirlen - 1]` where one register variable `v1` holds `dirbase` (a `char *`) and
then `dirlen` (a `size_t`): a variable-merge problem, not a cast one. No indirect call
through a `(long)`-cast base (a jump-as-call base) appears on the shared set.

## 3. The change

A new option `castindex on|off` (P9, cast-policy), in `p9_emit/kuna_castarith.rs`
beside `castarith`, at the same seam. A separate option rather than a value of
`castarith`: the two act on disjoint ops (a constant addend; a variable addend or an
`INT_SUB`), every cast class of this campaign is ablatable on its own (`castimplied`,
`castsign`), and a pointer difference is not an offset at all.

- `rewrite_index`: an `INT_ADD` of one pointer and one non-constant becomes
  `PTRADD(base, x, #sizeof T)` when the addend is an implied `x * sizeof(T)` or
  `x << s` read only by the add (the scaling op is destroyed), or `PTRADD(base, v, #1)`
  when `T` is one byte. `T` and every refusal of it come from `castarith`'s own
  `element`, factored out (`castindex off` is byte-identical to the base on all 45
  castbench binaries). The index must be a pointer-width integer, not an enum and not
  a pass-inserted cast. An implied `SEXT`/`ZEXT` index prints bare under the `PTRADD`,
  as upstream always printed it, after the extension-input rule has made its input's
  signedness the extension's.
- Cast accounting: the rewrite happens only where it saves a cast (the index's
  extension, the base, the sum), the sum's consumers asked through the pass's own
  `get_input_cast`. Without it 10 of 1,497 hunks traded one cast for another.
- `pointer_difference`: an `INT_SUB` of two pointers to the same one-byte integer type
  (after typedefs), whose result is an integer of the pointer's width, keeps its
  operands; the result is cast against the signed pointer-width integer, the token the
  two `(long)` casts produced and C's `ptrdiff_t`, so `(unsigned long)(v8 - v18)` in an
  unsigned compare is decided exactly as before. Counted per operand as `castInput`
  would spend it.

## 4. Value preservation

- Index: the element size is the scale, C scales the subscript by the same `sizeof`,
  and C converts an integer subscript by its own type's signedness, which the pass made
  the extension's. A byte element takes the whole addend, an expression of the 8-byte
  type the integer form added.
- Difference: `p - q` of two `char *` is their byte difference, of type `ptrdiff_t`
  (`long` on LP64, `int` on ILP32, the signed pointer-width integer the `(long)` casts
  named), so every later use (a division, an arithmetic shift, a signed or unsigned
  compare, a length argument) computes the same.
- Compiled round trips (`decompile_all_cli.rs
  a_variable_index_and_a_byte_pointer_difference_round_trip_through_the_printed_c`,
  fixture `castindex_x86_64.c`, 25 functions, gcc -O0 / clang -O0 / gcc -O2 builds,
  option on and off; also compiled by hand with gcc 11 and clang 14 under
  `-Werror=int-conversion -Werror=incompatible-pointer-types`): loads of widths
  1/2/4/8 and a double, stores, a loop, `int`/`unsigned int`/`short`/`signed char`/
  `unsigned char`/`long` indexes with negative values, an `unsigned int` index
  `0x80000001` read through a 24 GiB `MAP_NORESERVE` map (a sign extension would read
  2^31 elements backward), `strchr`/`strrchr` differences divided by 3, shifted,
  compared signed and unsigned, and passed to `strnlen`; controls: a 16-byte stride read
  at 8, a 12-byte stride read at 4, a byte offset read at 8, a `long *` difference;
  and a base64 decoder whose `malloc`ed global `char *` table is indexed by input
  bytes of 0x80 and up (`'\xff'`, `'\x80'`, `'*'`) into a filler with the sign bit set,
  each quad folded into a checksum. Every program prints what its binary prints, and
  a hand-made mutant of the printed decoder with a sign-extended index, or with an
  `unsigned char` element, prints a different checksum.
- 12 unit tests in `p9_emit/kuna_castarith/tests.rs`; the stage test
  `tests/stages/kuna-castindex.xml` (7 assertions, two passes).

## 5. Measurements

castbench full (4,815 shared functions; base = the main `c960fb18d` arm, with
`castimplied`, `castarith`, `castsign` and `globalref` on):

| arm | casts | /kloc | /100 stmts | vs IDA |
|---|---:|---:|---:|---:|
| IDA | 37,821 | 155.4 | 27.0 | 1.000 |
| main | 37,477 | 196.6 | 31.2 | 0.991 |
| **castindex** | **36,617** | **192.1** | **30.5** | **0.968** |

Per level: O0 12,234 -> 11,942 (1.003 -> 0.979), O2 13,837 -> 13,539 (1.028 -> 1.006),
O2-noinline 11,406 -> 11,136 (0.937 -> 0.915). 220 functions fewer casts (860 casts),
**0 more**. Against IDA per function: fewer than IDA 1,946 -> 1,970, more 1,412 -> 1,384.
By shape: `(long)<var>` -827, `(char *)(...)` -197, `(void *)(...)` -59,
`(unsigned char *)(...)` -36, `(long *)(...)` -21, `(unsigned long *)(...)` -17.

Whole corpus (every function of the 45 binaries, both arms of one build): 776
functions change, none with any structural difference (lines, statements, gotos,
labels, loops, returns); 1,487 hunks, every one with fewer casts, the same variable
references and only an element-size scale lost: 998 subscript, 481 difference, 8 both
(`hunkclass.py`, `structcheck.py`). `castindex off` is byte-identical to main
`c960fb18d` on all 45 binaries. The flip: `default-on-evaluation.md`.

A textbook base64 decoder shows the table-lookup shape
(`gcc -O0`, the decoder's `malloc`ed `char *decoding_table` indexed by input bytes):

```c
// castindex off
v2 = (*(char *)(v8 + a0) != '=') ? (int)*(char *)((unsigned long)*(unsigned char *)(v8 + a0) + (long)decoding_table) : 0;
*(char *)(v9 + (long)v6) = (char)(v5 >> 0x10);
// castindex on
v2 = (*(char *)(v8 + a0) != '=') ? (int)((char *)decoding_table)[*(unsigned char *)(v8 + a0)] : 0;
((char *)v6)[v9] = (char)(v5 >> 0x10);
```

The `(int)` on the element and `*(char *)(v8 + a0)` (an integer `long a0` plus an
integer) are other cast classes; this option does not touch them. The round-trip
fixture carries the same decoder (section 4).
