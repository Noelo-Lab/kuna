# Weak MIPS import identity

Original authored GAS source, no vendor code. `identity.S` loads one undefined
weak function through its GOT entry; the mixed variant also reads a read-only
pointer initialized by a direct R_MIPS_32/R_MIPS_64 relocation to that same ELF
symbol. The GOT-only variant has no direct reference. Both restore their ABI
GOT base before returning. N32/N64 and both byte orders are included.
`page.S` tests the same weak function through `%got_page`/`%got_ofst` and tail
calls it. GNU ld 2.30 `-shared` gives that pair one global GOT entry with
initial value 0 and leaves the `GOT_OFST` field 0, in both ABIs.

Rebuild with `python3 build.py --assembler /path/to/mips64-octeon-linux-gnu-as`.
The recorded GNU assembler is 2.30 from the local SDK; provenance records source,
tool and output hashes and exact commands. Tests use an explicit generic MIPS
LE target because automatic little-endian Octeon selection is intentionally
unavailable. Equivalent original review programs were GNU-linked and executed
under the pinned QEMU Octeon68XX reference in both BE ABIs; LE is static loader
and public-assertion evidence only.
