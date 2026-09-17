# mulblob — whole-corpus before/after sweep

`kuna decompile-all` in both arms over 12 binaries from the frozen decbench tree
(`results/full_run_address_2026-09-11`), O0 / O2 / O2-noinline:

```
O0  coreutils/fmt      O2  coreutils/fmt       O2-noinline coreutils/fmt
O0  grep/grep          O2  grep/grep
O0  gzip/gzip          O2  gzip/gzip
O0  bzip2/bzip2        O2  bzip2/bzip2
O2  tar/tar            O2  diffutils/diff      O2  findutils/find
```

**4,535 functions decompiled; 22 change.** Every other function is
byte-identical, in both directions.

## What changes in the 22

Each changed function was compared whole, not hunk by hunk: the declaration
block as a multiset of type spellings with the names stripped, and the body
after normalizing local numbering (`vN` -> `V`), the two spellings of a widened
operand (`vN._0_16_` and `ZEXT816(x)` -> `OPND`), and the half-fill statements
that exist only to build a blob.

| | |
|---|---|
| 16-byte blob declarations | 114 -> 11 |
| declaration multiset delta, pooled over the 22 | exactly 103 `char V [16]` removed, **nothing added** |
| functions whose declaration block grows | **0** |
| bodies identical under that normalization | 12 of 22 |
| bodies that differ further | 10 |

The 10 differ in one way, and it is the same way every time: with the blob gone,
a widened operand read by two printed expressions has two readers of its own, so
P6 gives it a name and the reads use the name.

```
off   v8._8_8_ = 0;  v8._0_8_ = v29;  v18._8_8_ = 0;  v18._0_8_ = v27;
      v29 = SUB168(v8._0_16_ * v18._0_16_,0);
      if (SUB168(v8._0_16_ * v18._0_16_,8)) {

on    v2 = ZEXT816(v10);
      v10 = SUB168(v2 * ZEXT816(v8),0);
      if (SUB168(v2 * ZEXT816(v8),8)) {
```
(coreutils `fmt -O2` `xstrtoumax`, the loop at 0x7853; four statements and two
declarations become one of each.)

Two of them additionally lose a nesting level: `else { if (c) { … } }` collapses
to `else if (c) { … }` once the two half-fill statements no longer make the
block complex enough to keep the brace. Same statements, one level less.

The 11 surviving blobs are that same residue in declaration form: the operand is
explicit, so it renders as `char v3 [16]; ... v3 = ZEXT816(v8);` — one
declaration and one statement instead of one declaration and two. Assigning a
16-byte value to an array-typed local is not compilable C, but neither is the
`v3._8_8_ = 0;` pair it replaces: both arms print a pseudo-C spelling for a
value with no C type, and this arm prints one statement fewer. Every one of the
11 is read.

## The decisive check: the p-code is unchanged

Textual hunk classification cannot by itself prove that no statement moved, so
the six largest changed functions were also dumped with `print raw` in both arms
and compared op by op, after dropping the ops that *are* the widening
representation (the `CPUI_PIECE`, the `COPY #0` feeding it, and the `INT_ZEXT`
that replaces them):

```
fn  binary / addr                       ops off -> on   non-representation ops
1   O2 coreutils/fmt   0x74d0 (xstrtoumax)  368 -> 328   identical but for 20 operand COPYs
2   O0 coreutils/fmt   0x7b53                30 ->  27   identical but for 2 operand COPYs
3   O2 diffutils/diff  0x17190               51 ->  47   identical but for 2 operand COPYs
4   O2 tar/tar         0x4eec0              466 -> 422   identical but for 22 operand COPYs
5   O2 grep/grep       0xcec0              2980 -> 2972  identical but for 4 operand COPYs
6   O2 findutils/find  0x26060              369 -> 329   identical but for 20 operand COPYs
```

The only ops that disappear are the `COPY`s that existed solely to give the
`PIECE` a Varnode for each half (`u0x000a7300 = RBX`). **No arithmetic, branch,
load, store, call or return op differs at any address in any of the six.**
(fn2 additionally shows one `(cast)` op where the off arm had two operand COPYs:
`ActionSetCasts` inserting a rendering cast, not a value change.)  Pooled over
the six: 70 operand COPYs disappear and one rendering cast appears; every other
core op is identical, address for address.

## Compile-and-run oracle

`gcc -O2` fixture with two 64x64->128 multiply shapes (a plain `widemul` writing
both halves, and the overflow-checked multiply-accumulate of the `xstrtoumax`
idiom), decompiled in both arms, both renderings transcribed back into C behind
`SUB168` / `ZEXT816` / `CARRY8` shims and run against the original source:

```
cases=10081 mismatches=0
```

81 edge pairs over {0, 1, 2, 2^32-1, 2^32, 2^63-1, 2^63, 2^64-1, 0xdeadbeefcafebabe}
plus 10,000 xorshift triples. Both halves of the product, and the
overflow verdict, agree with the source in both arms and with each other.
