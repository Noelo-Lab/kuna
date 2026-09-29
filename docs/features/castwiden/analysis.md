# castwiden: analysis

## The residue

After rounds I-K (`castimplied`, `castarith`, `castsign`, `globalref`, `castindex`,
`castternary`, `callpush`), kuna printed 35,588 casts on the 4,815 functions it and
IDA both emit in the 45 castbench binaries (0.941x IDA's 37,821), but 29.9 per 100
statements against IDA's 27.0, and 31.5 against 26.1 at -O2. Sign-merged, the
largest shape kuna prints more of is the 64-bit integer cast: `(long)v` /
`(unsigned long)v` +2,450, `(long)(expr)` +881, `(long)*p` +333, `(long)call()` +212.

## Census: what those casts are (engine-side)

The text shape `(long)v` does not say what the cast converts, so the census was
taken in the engine: an instrumented build (`census-instrumentation.patch`, not
part of the change) logged, for every 8-byte integer cast the printer emitted, the
op (INT_SEXT, INT_ZEXT, or a same-size `CPUI_CAST` from an integer or a pointer),
the operand's printed C type as `castimplied` knows it, and the op reading the
cast with, for a binary op, the other operand's printed type. Over the 4,815
shared functions, main 0096e984d printed:

| kind | printed |
|---|---:|
| `CPUI_CAST` pointer -> integer (`(long)p`, the `castarith`/`castindex` residue) | 2,300 |
| INT_ZEXT printed as a cast | 1,554 |
| `CPUI_CAST` sign change (`(long)v` of an `unsigned long`; 664 in comparisons, `castsign`'s domain) | 1,350 |
| INT_SEXT printed as a cast | 1,350 |
| other (`undefined8`, typedefs) | 464 |

So of the ~4,000 sign-merged excess only the 2,904 SEXT/ZEXT casts are widenings.
Where they sit (the op reading the cast):

| reader | SEXT | ZEXT | what C needs |
|---|---:|---:|---|
| `*` | 671 | 370 | 805 are `(long)i * K` beside an `int` literal: C computes `i * K` in `int` |
| `+` / `-` | 371 | 648 | 687 beside an `int` literal; about 300 beside a 64-bit operand |
| statement top (assignment) | ~130 | ~214 | 199 are globals whose declared type kuna never states (refused) |
| call argument | 95 | 65 | mostly untrusted (trial) prototypes |
| shift, left operand | 13 | 89 | must stay |
| comparison | 19 | 21 | must stay (`castsign`'s typing) |
| store | ~10 | ~40 | the pointee of the printed pointer |
| `& | ^`, `/ %`, unary `-`, stacked | ~60 | ~70 | |

The largest group, 1,640 casts, is a widened operand beside an **8-byte literal**
that kuna prints unsuffixed, so C types it `int` and the cast is what makes the
arithmetic 64-bit. IDA prints the same sites with an `LL` literal: gzip
`gen_bitlen` -O0 `*(short *)(4LL * dword_DF200[dword_DFAF8] + v17 + 2) = 0;` where
kuna printed `*(unsigned short *)(v6 + (long)*(int *)((long)dat_dfaf8 * 4 + 0xdf200) * 4 + 2) = 0;`;
find `insert_exec_ok` -O0 `*(long long *)(8LL * v16 + a3)` for kuna's
`*(long *)(a2 + (long)v15 * 8)`; ls `abformat_init` -O0 `128 * (k + 12LL * j)` for
kuna's `((long)v8 * 0xc + (long)v9) * 0x80`; gzip `build_tree` -O2 `v16 = 4LL * v10;`
for kuna's `v16 = (long)v34 * 4;`. The literal is an 8-byte constant in the IR;
printing it with the suffix of that width is its type becoming right, and C's usual
arithmetic conversions then perform exactly the widening the cast spelled.

## Why upstream keeps them

`CastStrategyC::isExtensionCastImplied` (cast.cc:249) hides an extension under
arithmetic only when the other operand is an explicit variable of the same
metatype, or a constant no wider than `int`; an 8-byte constant is refused on
purpose, because an unsuffixed token does not say its size. Every other reader
(a load, a field, an expression, a store) returns false.

## What is removable, and what is not

A cast may go only where C's own conversion performs the same conversion. For a
binary op, the usual arithmetic conversions (C11 6.3.1.8) must give the operation
the same type with and without the cast, and the operand must reach that type with
the same value: the cast's own type, or a cast that keeps the value (an integer
conversion depends only on the value converted, 6.3.1.3). The extension must also
be the one C performs on the printed operand type (SEXT of a signed type, ZEXT of
an unsigned one); a mismatch keeps its cast.

Not removable, and left alone:

- a widening beside an `int` literal, unless the literal is printed with its 8-byte
  suffix (`literal`): `i * 8` overflows where `(long)i * 8` does not;
- a zero-extension beside a signed 8-byte operand, `(unsigned long)c + l`: the bare
  operand makes the operation signed, so a comparison or a right shift of the sum
  would change (about 150 sites; lifting it needs a proof that every reader of the
  sum is sign-blind);
- a shift's left operand, a comparison (40 widenings; the 664 sign-change casts in
  comparisons belong to `castsign`), unary minus;
- both factors of `(long)a * (long)b`: one goes, the survivor keeps the product
  64-bit;
- an operand whose printed type is unknown or not the IR's (a merged variable, a
  global kuna never declares);
- a negated literal C types as unsigned (`-0x80000000`, whose C value is +2^31).

## Two pre-existing wrong-value prints found on the way

Both are unchanged by this option (it refuses both shapes) and are left for their
own fix:

- `(long)i + -0x80000000` for `(long)i - 0x80000000L`: the unsuffixed hex literal is
  `unsigned int`, so the negation is +2^31 (`castwiden_x86_64.c` `keep_neglit`,
  checked as text only).
- `(long)((1U - a0) * 0x13)` for `(long)((1 - v) * 19)`: the `1U` makes the product
  unsigned, so C zero-extends what the binary sign-extends.

## Three ways a removed cast changed how C groups or widens the text

A cast does two things besides converting: it parenthesizes its operand, and it
makes its operand 64-bit wherever C pairs it. The first version of this option
checked the IR operation a widening sat under, which is not always what C pairs it
with in the printed text. Three shapes changed the computed value:

- **One widened value read twice.** `MULT(t, t)` with `t = SEXT(i)` prints
  `(long)i * (long)i`; the printer asks about a cast per reading op, not per slot,
  so leaving out one left out both and printed the 32-bit `i * i` (gcc -O2
  `return (long)i * i;`). An op whose two inputs are the same varnode now keeps both.
- **Lost parentheses.** `p[1] + (long)(a + b)` lost its cast and printed
  `p[1] + a + b`, which C evaluates as two 64-bit additions (clang -O0; binary -2,
  printed C 4294967294 for `a = b = 0x7fffffff`). A dropped widening now prints as
  upstream's hidden extension does, whose token parenthesizes the operand whenever
  its operator binds no tighter than the reader's: `p[1] + (a + b)`.
- **Regrouped chains.** The printer writes `x + (y + z)` as `x + y + z` (the same
  associative operator on the right takes no parentheses), and C groups that as
  `(x + y) + z`. `(long)a + ((long)b + *p)` dropped both casts, `a` against the
  64-bit chain and `b` against `*p`, and printed `a + b + *p`: a 32-bit `a + b`
  (clang -O0). The left operand of `+ * & | ^` whose right operand is such a chain
  now loses its cast only when the chain's first leaf, as printed, would let it go
  (`first_leaf_fits`): `(long)a + b + *p`.

The first two were found in review; the third by `semhunks.py`, which parses both
sides of every changed line as C, types each removed cast's surroundings from the
function's own declarations, struct definitions and signatures, and checks that C
gives every operation the same type (for a chain of `+ - * & | ^` at 64 bits, the
same type at the chain's top). A textual comparison with parentheses ignored, the
first version's check, cannot see any of the three. On the review corpus printed by
the first version the classifier flags all of them (2 lost parentheses, 1 shared
widening, 3 regrouped chains); on this version, over 64 binaries (2,341 changed
functions, 6,422 changed lines), it flags nothing (`semhunks.json`).

The destination rules were also tightened: `castimplied`'s destinations widen only
for a widening to eight bytes (a 32-bit PE's `v7 = (unsigned int)(char)v12[1]` keeps
its cast) and only for an operand that prints as a C integer C extends the same way
(not `bool`, a 6-byte `undefined6` or a `CONCAT24`).
