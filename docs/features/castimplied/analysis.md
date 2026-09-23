# castimplied: analysis

## The gap

On the cast census's 4,815 functions matched by address with IDA Pro (45 binaries,
O0/O2/O2-noinline), kuna printed 45,126 casts against IDA's 37,821 (1.19x). One
of the census's ranked levers is the casts C's own conversion rules already
perform:

```c
ida : v25 = (char *)memchr(ptr, v12, v19 - (char *)ptr);
kuna: v7  = memchr(v10,(int)v4,(long)v14 - (long)v10);          // memchr takes an int

kuna: *(char *)((long)(int)(unsigned int)(unsigned char)sub_4042((int)v1) + 0x23560)
kuna: v31 = (unsigned long)v48;              // unsigned long v31; unsigned int v48;
kuna: return (unsigned long)(a1 < a0);       // unsigned long sub_afb1(long a0,long a1)
```

(sort -O0, sort -O2, ls -O0.)

## Where the casts come from

Every one of these is either a `CPUI_CAST` op `ActionSetCasts` inserted, or an
`INT_SEXT`/`INT_ZEXT` the printer renders as a cast. For the extensions the print
decision is `CastStrategyC::isExtensionCastImplied` (`p9_emit/cast.rs`, a verbatim
port of cast.cc:249). It hides an extension only when its reader is `PTRADD` or
one of thirteen integer arithmetic/comparison ops whose other operand is explicit
(or a small constant) with the same metatype. It answers no for:

- an extension at the top of a statement (`read_op == None`),
- a `CALL`, `RETURN` or `COPY` reader (no arm exists),
- a `CAST` or another extension reading it (the stacked case).

The census counted (engine lane, fmt/ls/sort/du O0): call arguments 154, statement
tops 134, `CAST` readers 47, `COPY` readers 17 of the 954 extension print events;
the IDA lane's syntactic count of the same idea on its paired corpus was 747
(assignment 238, return 57, identity 42, stacked inner 410).

## Why the rule is safe

C converts

- an argument to its parameter's type when a prototype is in scope (C11 6.5.2.2p7),
- the right operand of `=` to the type of the left (6.5.16.1p2),
- the operand of `return` as if by assignment to the return type (6.8.6.4p3),

and a conversion between integer types depends only on the value converted
(6.3.1.3). So, with the destination type spelled exactly as the cast:

- `f((T)e)`, `v = (T)e`, `return (T)e` compute what `f(e)`, `v = e`, `return e`
  compute: the implicit conversion is the same conversion.
- `(A)(B)e` computes what `(A)e` computes whenever converting `e` to `B` keeps its
  value: `(A)` then converts the same value.

Two conditions carry all of the risk, and the rule is built around them:

1. **The destination must be the one C sees.** A varargs position has no parameter
   type (the default promotions apply instead), and kuna's `formatstring` closes a
   resolved printf's varargs into ordinary per-call-site parameters; a trial
   prototype is a guess. Only a type-locked parameter of a direct call with no
   per-call-site override counts. A ternary arm's type is the arms' common type, not
   the assignment's, so a `?:` arm never counts as an assignment.
2. **The operand's C type must be the one the value test assumes.** C promotes
   `a - b` over two `unsigned char`s to `int`, where the p-code wraps; a byte loaded
   through a `char *` is a `char` in C even where the IR types it `unsigned char`;
   and a merged register variable can be declared `int *` while the varnode read at
   one site is typed `long`. The operand's type is taken only from what the printed
   text states (a declaration, a cast that stays, a truncation printed as a cast, a
   load through a pointer printed with its type); arithmetic, constants and calls
   are unknown, and a declaration that spells none of the IR's types refuses the
   drop outright. (The first cut used the IR type there and dropped
   `v36 = (unsigned long)v9;` with `int *v9`: the corpus declaration check below
   caught it.)

What never moves: float conversions, pointer conversions (either direction),
conversions to `bool`, enums, truncations (`SUBPIECE`), `truncarg`'s argument casts
(#678) and `boolbyte`'s `(bool)(unsigned char)` (#680). A sign change or a
narrowing below the chain top always stays. At the top, the conversion must also
preserve every value of its operand's type, so `(unsigned long)i` from an `int`
stays even into an `unsigned long` variable, though C would perform it.

## What IDA does that is not copied

IDA drops the cast even at a varargs position
(`printf(v3, (unsigned int)"GNU coreutils")` truncates an address), and it types
52% of parameters as a 64-bit integer, which is how it avoids `(long)p`. Neither is
copied: varargs positions keep their casts, and no type is weakened.

## Not in this change

- `(long)a * (long)b`: dropping one of the pair is legal only when the surviving
  operand is provably the wide one; left for a separate, measured change.
- Assignments through a pointer (`*p = (long)x`): the pointee's printed C type is
  not reliably the IR's (the `char *` / `unsigned char` case above).
- Globals without an integer symbol type: `decompile-project` does not declare
  globals in C, so a reader of `v = (unsigned long)dat_1a06c;` learns the
  conversion only from the cast. They keep it.
