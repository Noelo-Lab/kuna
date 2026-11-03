# Bitfield pull regressions

Original x86-64 fixtures for PR #1495's review cases: a whole-field value used
after a store, an aliasing byte write or a call; a value used on multiple control
flow paths; by-value whole-field, multiple-field and mixed integer/float
arguments; and a swap of unequal-width fields.
`hi_save` also checks a true five-bit field, whose read was already reordered
on main. The aliasing test writes through `unsigned char *`, which may legally
access the containing object's representation.

Run `python3 build.py` here to rebuild both objects. `provenance.json` records
the compiler, commands and hashes. The DWARF object carries the compiler's
layout; the debug-free object exercises the C parser through CLI type/prototype
assertions. CLI tests compile the emitted functions alongside separately named
native functions from `review.c` and compare their return values and stores.
