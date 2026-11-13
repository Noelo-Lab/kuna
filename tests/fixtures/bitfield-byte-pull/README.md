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
DWARF keeps whole-byte bitfields as plain fields, so the DWARF object is only
run for the functions whose layout has no such field next to a true bitfield
(`hi_save`, `a_alias`, `a_call`, `a_phi`, `w_byval`, `m_byval`).

`shapes.c` holds five C-declared whole-byte shapes: constants compared with an
8-bit field (`job_state`), a signed byte field sign-extended from a register
(`pair_a`), one load tested several times after a call (`node_accept`), a
constant stored into a stack struct's 8-bit field (`node_store`) and a stack
struct's 8-bit field saved before a call and read again after it (`node_save`).
`shapes-no-debug.o` is `gcc -O2 -g0 -fcf-protection=none -fno-stack-protector
-c shapes.c` with gcc 15.3.0 (the `gcc:15` container), SHA-256
`a96eb8de1daecf4b02f366a3a4a1c35f01763ec99aabbc6ee4dff71fe1d388d7`.

`rmw.c` holds four true-bitfield reads from a word that the same function then
read-modify-writes, followed by a store through an aliasing `unsigned char *`
and a 64-bit return in a register pair: `pair_mid`, `pair_rest` and
`pair_inner` widen an unsigned field, and `pair_signed` sign-extends a signed
one. The machine code loads the word once and extracts the field before both
stores, so the read must print before them. `rmw-mipsbe.o` is
`mipsel-linux-gnu-gcc -EB -O2 -g -gno-record-gcc-switches -fno-pic
-mno-abicalls -fdebug-prefix-map=$PWD=. -c rmw.c -o rmw-mipsbe.o` with gcc
10.3.0 (Ubuntu 10.3.0-1ubuntu1), SHA-256
`988358943639a7823a547f723dcb19dda4f75a57c8befdc9063fb31968a98d39`.
