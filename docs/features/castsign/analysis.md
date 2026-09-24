# castsign: analysis

## The gap

The 2026-09-23 cast census counted 596 casts, on the 4,815 functions kuna and IDA
both emit, that only convert a variable to the other signedness at its own width.
kuna declared the variable unsigned and the program reads it signed:

```c
unsigned long v1; // stack - 0x10
v1 = strtoul(a0,NULL,0);
if ((long)v1 <= -1)
  v1 = strtoul(a1,NULL,0);
if (0 <= (long)v1)
```

526 of the 596 are on locals and 70 on parameters. The other direction (declared
signed, read unsigned) is 20 on locals and 12 on parameters. IDA leans the other
way and pays 587 casts for it.

## Why `signedness auto` leaves them

`signedness auto` (`p9_emit/kuna_typeround.rs`) already re-declares a local signed
when every signedness-sensitive reader of it is signed. A trace of its decision for
every local that carries one of these casts (base `5458b7ab5`, the full castbench
corpus, all functions):

| why the local was not re-declared | locals | casts |
|---|---:|---:|
| mapped onto a Symbol (every stack local, at offset 0) | 254 | 460 |
| mixed evidence: an unsigned demand (ZEXT that is read, `<`/`<=` unsigned, `>>` logical, `/ %` unsigned) beside the signed one | ~330 | ~600 |
| a `PTRADD` index reader | 46 | 67 |
| a conversion to a pointer | 48 | 75 |
| a member that is an input (read before written), treated as a parameter | ~30 | ~48 |
| address taken or slot split (a second high prints the same name) | ~20 | ~30 |

The mixed-evidence locals are nearly all `-O0` register temporaries (`// eax`) that
merge unrelated values, or values that leave through a zero-extension into a 64-bit
return or a varargs slot, where the high bits are read. Neither can be re-declared
without changing what the C computes. Parameters need a prototype change, not a
declaration change: the signature, the callers' arguments and the
`decompile-project` header all spell it.

## What is safe to take

- **A stack local that covers its Symbol whole.** Its declaration line is written
  from its own high, like a register local's. The rest of the walk is unchanged.
  An address-taken slot is excluded by name: `&v` is its own high carrying the same
  name, and re-signing `v` would change the C type of `&v` under a pointer typed for
  the old declaration.
- **A pointer-width `PTRADD` index.** `p[v]` computes `p + v * size` modulo 2^64
  under either declaration. The signed reading keeps a negative index defined, and
  a narrower index reaches the `PTRADD` through an explicit extension that still
  constrains it.
- **A same-width conversion to a pointer.** gcc and clang convert by keeping the
  bits.
- **A body local with an input member.** The printer declares only body locals,
  and `retain_sole_named` drops every planned high it does not declare.

Each of them is safe only for a variable that no `+ - * <<` and no unary `-`
reads (next section), and each flip must remove a cast that prints today.

None of them touches a declaration whose type is locked. A `--assert type`, a
DWARF local (mapped `typelock|namelock` onto its frame Symbol) and a type committed
from Ghidra lock the Symbol, not the member varnodes the walk checks, so
`kuna_castsign.rs (symbol_type_locked)` reads the lock on the Symbol: the naming
pass's bind, a dynamic Symbol, and the Symbol containing each address-tied member.
The first build of this option missed it and re-declared DWARF locals the source
declares unsigned: wc `-O0` `uintmax_t words, chars` and bzip2 `fallbackQSort3`'s
`UInt32 med` printed as `long`/`int`, and an asserted `unsigned long idx` printed
as `long idx`.

These relaxations only ever declare a value signed. The mirror direction would buy
at most 20 local casts and risks IDA's overshoot.

## Arithmetic on the variable

Re-declaring `v` signed changes more than the casts at its comparisons. Every
`v + k`, `v - k`, `v * k`, `-v` and `v << k` the body prints then computes in the
signed type. Unsigned arithmetic wraps, as the binary does; signed arithmetic is
undefined on overflow, and gcc folds on that even at `-O0`, clang at `-O2`. An
earlier version of this option took these as neutral, as `signedness auto` does
for register locals, and printed C that computes a different value from the
binary at the edge of the range. gcc -O0 on

```c
unsigned long v = strtoul(s, 0, 0);
while ((long)(v - 1) >= 0 && n < 5) { v -= 2; n++; }
```

gives `unsigned long v1;` with `0 <= (long)(v1 - 1)` on main, and that earlier
version printed `long v1;` with the same condition. For `v == 2^63` the binary
loops 5 times; the printed C built with gcc -O0, gcc -O2 or clang -O2 loops 0
times, because it reads `0 <= v1 - 1` as `0 < v1`. The headline shape,
`v1 = v1 - 1, 0 <= v1`, fails the same way under gcc -O2 and clang -O2. No cast
was removed by the first flip at all, only the declaration changed.

So a high only `castsign` admits (a frame local, a body local with an input
member, or a verdict a neutral reader decided) is left alone when any of those
operators reads it, directly or through the expression its value is printed into
(`kuna_castsign.rs (can_overflow)`). For `<<` only the shifted operand counts,
not the count. `& | ^ ~`, comparisons, truncations, assignments, call arguments,
stores and `return`s cannot overflow and keep a variable admissible. On the 45
castbench binaries the earlier version flipped 234 declarations. 162 of them are
now left alone: 148 print `+ - *` on the variable (69 of those update the
variable itself, `v = v - 1`), and the rest are arithmetic the text shows as
`v4 -= v7`, or a flip that removed no cast.

The walk `signedness auto` runs for register locals still treats `+ - *` as
neutral, so its own flips can compute differently at the edge unless the C is
built with `-fwrapv` (on main, gcc -O1 `(long)(v1 + 1) <= v1` over a `long v1`).
`castsign` does not change that walk for the highs it already covers; the spec
(§9.3) states the caveat.

## A flip must remove a cast

The first version also flipped declarations whose casts all stayed: in
`0 <= (long)(v1 - 1)` the cast is on the result of the subtraction, not on `v1`,
and `(long)(0x7fffffffffffffff - v3) / 0x15180` in shadow's chage keeps its cast
too. 23 of the 234 castbench flips removed nothing. Now a high only `castsign`
admits is re-declared only when one of its declared members is read by a cast
to exactly the new declaration's type that prints today
(`kuna_castsign.rs (drops_printed_cast)`). A cast whose value is only a call
argument, the right side of an assignment or a `return` can already be left out
by `castimplied`, so it does not count. Under another conversion it does:
`SEXT816((long)v34)`, `(long)(int)v8` and `p[(int)v16]` keep a same-width sign
change that never preserves the value, and `castimplied` keeps those.

## Rejected: declared types as unsigned evidence

A call argument to a type-locked unsigned parameter, and a store through a pointer
to an unsigned integer, were tried as unsigned evidence. Neither changes what C
computes (a same-width conversion). Over the full corpus they cost 46 casts: they
undid 20 functions' existing `signedness` flips. And they got the type wrong where
it mattered. gzip `xpalloc`'s `nbytes` is gnulib's signed `idx_t`, passed to
`realloc(size_t)`; the vote kept it `unsigned long` and brought back
`SEXT816((long)v3)`. A stored-through pointer's pointee is usually typed from the
stored value itself (du's `struct timespec` fields), so that vote is circular.

## The assignment side

Re-declaring `unsigned long v18` as `long v18` would bring back the `(unsigned
long)` `castimplied` dropped from `v18 = dat_dd86c << 3;`, because the spellings no
longer match (gzip -O0 `0xd846`, `0xd0a0`). A conversion to an N-bit integer depends
only on the value modulo 2^N (C11 6.3.1.3; gcc and clang define the signed case the
same way). So `lhs = e` stores the bits `lhs = (T)e` stored whenever the
re-declared type is as wide as `T`, whatever `e`'s C type is. `castimplied` accepts
that case for a declaration the signedness rounding re-signed, when `castsign` is
on.

## Found while building the round-trip fixture (not this option)

gcc -O1/-O2 compile `long f(char *s) { long i = strlen(s) - 1; while (i >= 0 &&
s[i] == ' ') ...; return i; }` so that `rax` still holds the strlen-derived
counter at `ret`. kuna declares that function `void` with or without this option
(also on `dbe854ba3`), so the printed C drops the return value. The fixture returns
`i * 10 + 3` to keep the round trip about signedness.
