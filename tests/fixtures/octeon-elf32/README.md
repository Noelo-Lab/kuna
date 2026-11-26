# Big-endian Octeon ELF32 images without an N32 ABI

`calls.c` holds `crc16`, a CRC-16 (polynomial 0x1021, initial value 0) whose
bit-15 test GCC compiles to the Octeon `bbit0` at `0x10000018`, its caller
`verify`, and `describe`, which saves `ra` with `sd`, extracts fields with
`exts` and `dext`, and passes five arguments to `emit`, the fifth in the O64
stack slot at SP + 32. `cop2.S` is a genuine COP2 load, `lwc2 $15,-6($3)`,
the encoding family Octeon reuses for `bbit0`. `imports.c` is PIC code that
calls `free` and `strlen` through the GOT and switches through a jump table;
`provider.c` stands in for the C library at link time.

| Image | `e_flags` | ABI metadata | Expected selection |
| --- | --- | --- | --- |
| `o64.elf` | 0x808b2001 (Octeon, O64, MIPS64r2) | `.MIPS.abiflags`, soft-float `.gnu.attributes` | `MIPS:BE:64:octeon-32addr:o64` |
| `noabi.elf` | 0x808b0001 (Octeon, MIPS64r2, no ABI) | GCC's `.mdebug.abiO64` marker only | the same, with a note that O64 is assumed |
| `bare.elf` | 0x808b0001 | none | the same, with the note |
| `mdebug32.elf` | 0x808b0001 | `.mdebug.abi32` (O32 code) | generic selection, as before, with a note that the marker contradicts O64 |
| `cop2.elf` | 0x80002801 (generic MIPS64r2, O64) | `.MIPS.abiflags` | generic `MIPS:BE:64:64-32addr:o64`, `lwc2` |
| `pic.so` | 0x808b2007 (PIC, Octeon, O64) | `.MIPS.abiflags`, soft-float `.gnu.attributes` | `MIPS:BE:64:octeon-32addr:o64`; `free` and `strlen` through the GOT |
| `octeon2.elf` | 0x808d2001 (Octeon II, O64) | `.MIPS.abiflags` | generic selection, as before, with a note that only the original Octeon has an O64 model |

`noabi.elf` reproduces the header of a toolchain that writes no ABI bits
(clearing them by hand) and no ABI-flags record or GNU attributes (discarded
by `link-noabi.ld`). Rebuild in this directory with GCC 7.3.0 and GNU
binutils 2.30 built for `mips64-octeon-linux-gnu` from
`MarvellEmbeddedProcessors/Octeon-Toolchain`'s `toolchain-src-265.0.tar.bz2`
(archive SHA-256 `1616c4c43a5cbb5ae67f88fc5fbd5b577be697ecc59ed441669af4b01c5967a1`);
`${CROSS}` is the tool prefix:

```sh
cc() { ${CROSS}gcc -O1 -march=octeon -mno-abicalls -fno-pic -G0 -fno-ident "$@"; }
ld32() { ${CROSS}ld -m elf32btsmip --build-id=none -z max-page-size=0x10 -z common-page-size=0x10 "$@"; }
noabi() { printf '\200\213\000\001' | dd of="$1" bs=1 seek=36 conv=notrunc status=none; }  # e_flags = 0x808b0001
cc -mabi=o64 -c calls.c -o calls.o
cc -mabi=32 -c calls.c -o o32.o
${CROSS}as -march=mips64r2 -mabi=o64 -o cop2.o cop2.S
ld32 -T link.ld -o o64.elf calls.o
ld32 -T link-noabi.ld -o noabi.elf calls.o && noabi noabi.elf
ld32 -T link-bare.ld -o bare.elf calls.o && noabi bare.elf
ld32 -T link-noabi.ld -o mdebug32.elf o32.o && noabi mdebug32.elf
ld32 -e load -Ttext=0x10000000 -o cop2.elf cop2.o
pic() { ${CROSS}gcc -O2 -march=octeon -mabi=o64 -mabicalls -fPIC -fno-ident "$@"; }
pic -c provider.c -o provider.o && ld32 -shared -soname libc.so.6 -o libc.so.6 provider.o
pic -c imports.c -o imports.o && ld32 -shared -o pic.so imports.o libc.so.6
${CROSS}gcc -O1 -march=octeon2 -mabi=o64 -mno-abicalls -fno-pic -G0 -fno-ident -c calls.c -o octeon2.o
ld32 -T link.ld -o octeon2.elf octeon2.o
rm -f calls.o o32.o cop2.o provider.o libc.so.6 imports.o octeon2.o
```
