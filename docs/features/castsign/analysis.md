# castsign: analysis

## The gap

The 2026-09-23 cast census counted 596 casts, on the 4,815 functions kuna and IDA
both emit, that only convert a variable to the other signedness at its own width.
kuna declared the variable unsigned and the program reads it signed:

```c
unsigned long v1; // stack - 0x10
v1 = strlen(a0);
while ((v1 = v1 - 1, 0 <= (long)v1 && (a0[v1] == ' '))) {
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
