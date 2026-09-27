# castwiden: plan

## The seam

A print-time decision, like `castimplied` and `castternary`; the IR is not changed.

- `p9_emit/kuna_castwiden.rs (plan)` decides, once per arithmetic op
  (`+ - * / % & | ^`), which widened operand keeps no cast and which 8-byte literal
  operand takes a size suffix. `kuna_castimplied.rs (ImpliedCasts::widen_plan)`
  caches it for the function (cleared by `ImpliedCasts::begin`).
- `printc.rs (PrintC::implied_cast_drops)` asks `ImpliedCasts::widen_drops` for the
  extension arms and the `CPUI_CAST` arm; a dropped cast pushes its operand bare,
  so precedence is the enclosing operator's.
- `printc.rs (PrintC::widen_suffix)` asks for the constant being pushed on the
  integer path and `IntegerLiteral::suffix_as_long` appends `L`/`LL`, or `UL`/`ULL`
  for an unsigned target (`-4UL` is the unsigned value of the bits `-4` spells).
- `castimplied`'s destination rule (`ImpliedCasts::decide`) takes three more cases
  under `castwiden`: a non-value-keeping widening into a destination of exactly its
  spelling (`converts_exactly`), a value-keeping one into an integer of the cast's
  width (`fits_dest`), and a store through a printed pointer (`store_pointee`).

## Types

The other operand's C type is a set of promoted types (`castternary`'s
`INT/UINT/LONG/ULONG` and `usual`), derived from the printed text only:
a declared variable (`ImpliedCasts::explicit_type`), a conversion that stays, a
literal (both bases: the printer picks decimal or hex by magnitude, and the two
give a literal between 2^31 and 2^64 different types, so both are allowed for),
a load through a pointer printed as a declared variable, a cast, a structure field
(`find_truncation` at the `PTRSUB` offset, the printer's own lookup) or an element
of such a base, and a nested arithmetic op after its own plan (`expr_set`, depth
8). Everything else is unknown and keeps the cast.

## Value

For every type `b` the other operand may have: `usual(T, b) == usual(promote(S), b)`
and (that type is `T` or the cast keeps `S`'s values). The literal case also
requires the suffixed literal to be exactly `T`, the operation's type to stay
what it was, and no negated unsigned literal. Round trips: `castwiden_x86_64.c`
(gcc -O0, clang -O0, gcc -O2 builds) printed with each value, compiled by gcc and
clang at -O0 and -O2.

## Options considered

- A new value of `castimplied`: its readers are destinations; this is a new reader
  class (arithmetic), with its own risk (the operation's type), and a literal
  retyping `castimplied` does not do. A separate option lets each be evaluated and
  flipped on its own, as `castternary` was.
- Typing the literal from the start (P9 `markExplicitLongSize` for every 8-byte
  literal): prints suffixes where no cast is saved. The suffix is printed only
  where it replaces a cast.
- `(unsigned long)c + l`: needs a proof that every reader of the sum is sign-blind;
  left for a follow-up.
