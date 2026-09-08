# sectionless-elf-import-relocations

## The problem

An ELF with no usable section table loses every import name. `readelf -r
--use-dynamic` still reads all eight of KataVM_L1's PLT relocations off
`PT_DYNAMIC`; kuna named none of them, so the `puts` thunk and its call sites
came out anonymous.

Reproduce with any dynamically linked binary — zeroing `e_shoff`/`e_shnum`/
`e_shstrndx` changes no mapped byte, so nothing but the section table differs:

```
$ printf '#include <string.h>\n#include <stdio.h>\nint main(void){char b[8];memcpy(b,"hi",3);puts(b);return 0;}\n' > c.c && gcc -O0 -o sec c.c
$ cp sec nosec && python3 -c "
import struct; b=bytearray(open('nosec','rb').read())
struct.pack_into('<Q',b,0x28,0); struct.pack_into('<H',b,0x3c,0); struct.pack_into('<H',b,0x3e,0)
open('nosec','wb').write(bytes(b))"

$ kuna decompile ./sec main
unsigned long main(void)
{
  char v1 [8];

  memcpy(v1,"hi",3);
  puts(v1);
  return 0;
}

$ kuna decompile ./nosec "$(nm sec | awk '$3=="main"{print "0x"$1}')" --addr
unsigned long sub_1189(void)
{
  char v1 [8];

  sub_1090(v1,0x2004,3);
  sub_1070(v1);
  return 0;
}
```

## The fix

- New `loader/elf_dynseg.rs` reads the import markup out of `PT_DYNAMIC` —
  `DT_SYMTAB`/`DT_STRTAB`/`DT_SYMENT` for the names, `DT_JMPREL` and
  `DT_RELA`/`DT_REL` for the GOT slots they attach to, `DT_PLTGOT` as the i386
  GOT anchor — translating each virtual address through the `PT_LOAD` map. That
  is how the run-time loader finds the same tables; `object`'s
  `dynamic_relocations()` cannot, because it builds its iterator by scanning the
  section table for `SHT_REL`/`SHT_RELA`.
- `elf_plt::resolve_plt_imports` consults it only when the section-driven
  resolution produced **nothing at all**. An image with an intact section table
  never reaches the new code, so this can add names but never move one.
- The PLT is not reconstructed as a range. With no section name left there is no
  honest bound for it, and a guessed sub-range of an executable segment names
  stubs off by an entry. Each executable `PT_LOAD` window goes to the existing
  per-architecture decoders whole, and their decoded-GOT-target match decides
  which instructions in it were stubs — so the relocation slots are the bound.
  Every architecture those decoders already handle is reached; recovery is
  measured on x86-64, i386, AArch64 and RISC-V. PowerPC and MIPS are excluded,
  since neither resolves through a `.plt` code section at all.

## The tests

`dynseg_plt_x86_64` (808 bytes, synthesized by the generator beside it) is a
section-table-free PIE importing `puts` and `memcpy`, with a third 16-byte-aligned
run of code right after the two stubs: eight unit tests cover the tag walk, the
truncation and oversized-`DT_PLTRELSZ` paths, and that `main` is *not* named.
The promoted probe `tests/cli/sectionless-elf-import-relocations.json` decompiles
its `puts` stub, which prints `void sub_240(void) { (*dat_318)(); }` without the
fix.

Two sweeps over the same 131 ELF images (98 repo fixtures across eight
architectures, 33 system binaries with up to 233 imports). Resolved with and
without their section table, every name the segment path produced matched the
section path's answer at the same address — 0 mismatches, 0 extra names. Resolved
unmodified against a base-commit build, `kuna functions --json` is identical on
all 131, which is the inertness claim measured rather than argued.

Gates: `make test` 675/675 PARITY OK · `make test-stages` 692/692 PARITY OK ·
`make rust-test` green · `make check-spec` OK · `make test-cli` 91/91 ·
`kuna catalog --check` OK.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
