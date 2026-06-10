# ADR 0003: Integer semantics — explicit wrapping, C++ width mapping

## Context

The C++ code computes constantly on `uintb` (`uint8`, a 64-bit unsigned) and
`intb`, relying on C++'s defined unsigned wraparound for masks, sign
extension, and pattern arithmetic. Rust's `+`/`-`/`*` panic on overflow in
debug builds and silently wrap in release — a ported expression that "works"
in release could still be a logic divergence, and a debug run would abort on
benign intended wraps.

## Decision

`uintb -> u64`, `intb -> i64`. A wrapping-helper trait (`wadd`, `wsub`,
`wmul`, `wneg`, `wshl`, ...) is **mandatory** in ported arithmetic: any C++
expression whose operands can legitimately wrap is transcribed with the `w*`
helpers, never bare operators. Bare operators remain only where overflow is a
genuine bug. As a wrap detector, the golden corpus is run once under a debug
(overflow-panicking) build: every panic site is a place where a bare operator
should have been a `w*` call, fixed before the golden is accepted.
`calc_mask`'s truncation semantics (size-indexed mask table, size > 8 yielding
the full mask) are preserved exactly.

## Consequences

- Wraparound becomes grep-able intent (`wadd`) rather than an implicit
  property of the build profile; release and debug builds compute identically.
- The one-time debug golden run converts "did we miss a wrap?" from an audit
  into a mechanical detector.
- Slight verbosity in hot arithmetic; accepted, since these are exactly the
  lines where the C++ semantics must be visible to a reviewer.
- Sign-sensitive C++ casts (`(intb)` on a masked value, etc.) must be ported
  as explicit sign-extension helpers, not `as` casts chosen by eye.
