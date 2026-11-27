#!/bin/sh
# Rebuild the GCC PIC images. MIPS_CC is GCC 7.3.0 (the Octeon toolchain's
# mips64-octeon-linux-gnu-gcc), CLANG is Clang 21.1.8 (used only to assemble
# GCC's output) and MIPS_LD is GNU ld 2.47.
set -eu
: "${MIPS_CC:?}" "${CLANG:?}" "${MIPS_LD:?}"
cd "$(dirname "$0")"
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT

image() {
    out=$1 src=$2 target=$3 em=$4
    shift 4
    "$MIPS_CC" "$@" -mhard-float -O1 -ffreestanding -fno-builtin -fno-ident -fPIC -mabicalls \
        -S "$src" -o "$work/calls.s"
    "$CLANG" -target "$target" "$@" -mhard-float -fdebug-compilation-dir=. -c "$work/calls.s" -o "$work/calls.o"
    "$MIPS_LD" -m "$em" -shared --build-id=none -z max-page-size=0x1000 "$work/calls.o" -o "$out"
}

image n32.so n32.c mips64-linux-gnuabin32 elf32btsmipn32 -march=mips64r2 -mabi=n32
image n32-le.so n32.c mips64el-linux-gnuabin32 elf32ltsmipn32 -EL -march=mips64r2 -mabi=n32
image n64.so n32.c mips64-linux-gnuabi64 elf64btsmip -march=mips64r2 -mabi=64
image r6.so r6.c mips-linux-gnu elf32btsmip -march=mips32r6 -mabi=32

# Clang cannot assemble O64; GCC's own assembler writes its ELF32 flags.
"$MIPS_CC" -march=mips64r2 -mabi=o64 -mhard-float -O1 -ffreestanding -fno-builtin -fno-ident \
    -fPIC -mabicalls -c o64.c -o "$work/calls.o"
"$MIPS_LD" -m elf32btsmip -shared --build-id=none -z max-page-size=0x1000 "$work/calls.o" -o o64.so
