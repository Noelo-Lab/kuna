# Original MIPS relocation fixtures

These sources were written for the relocation loader tests. They contain no
third-party binaries or implementations. CI reads the checked-in objects and
GNU-linked images directly; neither a target toolchain nor QEMU is required.

`expressions.S` covers N32/N64, both byte orders, signed 16-bit fields, absolute
32/64-bit fields, REL32/PC32, jumps and branches, HI16/LO16 carries, and ordered
32/SUB/HI16 and 32/SUB/LO16 expressions. N64 adds HIGHER/HIGHEST carries. GNU as
represents the expressions as separate N32 records and packed N64 operations.
Additional fields cover null-symbol addends, sign-extended N32 symbols,
intermediate overflow, N64 expressions spanning records, and 64-bit carry wrap.
N64 narrow 32/REL32/PC32 operations followed by a 64-bit write retain the
intermediate truncation observed in GNU ld.
The absolute symbols and `.text` at 0x400000 let the loader and GNU ld use the
same addresses. Tests compare the complete relocated `.text` bytes, including
opcode bits and bytes outside relocated fields.

`implicit.S` covers REL HI/LO pairing, two high halves sharing one low half,
signed implicit addends, and separate same-offset REL expressions reading
earlier writes. It is an O32 big-endian input for the shared relocation
arithmetic; the pinned assembler does not support the O32 little-endian output
format. N32/N64 endian coverage comes from the four `expressions` image pairs.

`gp.S` adds genuine `.cpsetup` N32/N64 and `.cpload` O32 prologues, GPREL16,
GPREL32 and the linker-provided `__gnu_local_gp` symbol. The object defines
`_gp` as 0x408000, and `gp.ld` gives GNU ld the same value. LITERAL fields cover
global and local targets and a composed LITERAL/SUB/LO16 expression; literal
sections are not merged. GNU partial links produce four more object/image pairs
whose register metadata records a nonzero input gp and whose local GPREL32
addends have already been adjusted. Tests also damage that metadata and require
only the dependent fields to remain unchanged. All fourteen object/image pairs
compare bytes; the O32 inputs are arithmetic checks rather than ABI qualification.

`got.S` and `got.ld` add five GOT object/image pairs (BE/LE N32/N64 and
O32 BE). They cover GOT_DISP, local GOT16 and page/offset pairs, signed-low
page carries, CALL16/JALR, split GOT/CALL offsets and an unresolved weak slot.
Tests compare the value each GOT load reaches against GNU ld, allowing a
different slot order, and compare all other instruction bits exactly.

`call-composed.S` adds four BE/LE N32/N64 pairs with distinct untyped external
callees for direct jumps, scalar-to-jump compositions, longer scalar chains,
chains crossing packed-record boundaries and composed PC16 calls. The complete
relocated text matches GNU ld.

The assembler manifest contains 23 object/image pairs.

`pc32.S` adds N32/N64 BE/LE objects and GNU-linked images for a masked R32
result carried into PC32 and a standalone PC32 displacement below the signed
32-bit range. N64 also checks an upper-bit symbol and PC32 followed by a
64-bit write, which must retain the zero-extended 32-bit result. PC32 uses
the low 32 bits of `symbol + addend - place`, without a signed overflow
failure. The unit test compares every relocated text byte against GNU ld and
asserts relocation metadata and the exact `0xffbffffc` result at 0x400000.
GNU as materializes the PC32 addend into the initial field as well as the
RELA record; `.word 0` therefore produces the initial `0xfffffffc` field.
The stage assertions check both recovered return values for all four inputs.
`provenance-pc32.json` records source/output hashes, commands and tool versions.

```sh
python3 build-pc32.py --assembler "$OCTEON_AS" --linker "$MULTIARCH_LD" --output "$OUTPUT"
```

`rel-chain.S` adds N32 implicit-addend cases in both byte orders: two
`R_MIPS_32` records at one offset accumulate `7 + 0x12340000 + 0x56780000`
to `0x68ac0007`, and another pair wraps to `0x10`. `build-rel.py` assembles
ordinary MIPS64r2/N32 code, converts its zero-addend RELA tables to REL while
preserving field bits, symbols, offsets and record order, then links the REL
objects with GNU ld. It rejects nonzero explicit addends rather than dropping
them. `provenance-rel.json` records source/output hashes, the intermediate
RELA hashes, conversions, commands and tool versions. The unit tests require
implicit-addend metadata and exact GNU-linked data bytes; damaged copies
check that one failed REL record leaves successful records intact. The stage
test checks both recovered return values on BE and LE inputs.

```sh
python3 build-rel.py --assembler "$OCTEON_AS" --linker "$MULTIARCH_LD" --output "$OUTPUT"
```

`pic.c` and `external.c` provide four independently compiled PIC object/image
pairs (GCC 4.7/7.3, BE N32/N64). They exercise global and table accesses, direct
and local calls through the GOT, an optional weak call and address returns.
`build-pic.py` derives section addresses from the original object and puts the
external helper at a fixed address.

To regenerate into a separate directory:

```sh
python3 build.py --assembler "$OCTEON_AS" --linker "$MULTIARCH_LD" --output "$OUTPUT"
python3 build-pic.py --compiler "$OCTEON_CC" --linker "$MULTIARCH_LD" --tag gcc73 --output "$OUTPUT"
```

`provenance.json` records every command, source/output SHA-256 and tool version.
The assembler comes from the Marvell Octeon toolchain's GCC 7.3 source archive
`toolchain-src-265.0.tar.bz2`, SHA-256
`1616c4c43a5cbb5ae67f88fc5fbd5b577be697ecc59ed441669af4b01c5967a1`.
GNU multiarch ld 2.47.20260726 supplies the independent relocation oracle.
Repeat the PIC build with GCC 4.7 and `--tag gcc47`. Its `provenance-gcc47.json`
and `provenance-gcc73.json` record source/output hashes, commands and tool
versions separately from the assembler fixture manifest.
