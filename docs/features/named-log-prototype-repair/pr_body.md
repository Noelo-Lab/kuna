## The problem

A named prototype assertion could report `applied` while changing the wrong
program entry. PE imports can expose the same name on a data-only IAT slot and
an executable thunk; the old assertion resolver selected the first name match,
even when calls targeted the thunk. Consequently the asserted signature did not
reach those calls.

## The fix

Named prototype targets now go through `ConsoleProgram::resolve_entry`, the
same public contract used by entry-selecting CLI surfaces. A name with one
executable definition canonicalizes to that address and canonical name. Two
executable definitions reject as ambiguous. A name not present in the program
still remains pending for later discovery, and numeric target behavior is
unchanged.

This changes only assertion routing in the existing prototype source. It adds
no option, phase, catalog element, fixture, database behavior, P6/P9 behavior,
stage baseline, or history row.

## The tests

- A console integration test on the existing win32sigs PE fixture disables
  built-in signatures and proves a named three-argument `LoadLibraryExW`
  assertion produces byte-identical C to an address assertion at the executable
  thunk (`0x401010`).
- An overbreadth test on the existing libcsigs PE collision fixture proves two
  same-named executable candidates reject with both addresses in the ambiguity
  diagnostic.
- The suite also pins unresolved-name pending behavior.
- The promoted CLI probe passes 1/1 and fails against an unpatched build. The
  focused console suite passes 29/29; the full Rust workspace exits 0; spec,
  strict spec, catalog, counters, and mergecheck are clean.

The restored durable dataset acceptance is hardened to require a rename-tolerant
full assignment containing a nonempty `log(...)` call while retaining the
`log(SUB84(` absence guard. It passes on the original PolyMLP.exe witness under
its recomputed ID, `a-43296fdf37fe`.
