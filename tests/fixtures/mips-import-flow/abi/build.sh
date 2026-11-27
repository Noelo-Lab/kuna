#!/bin/sh
set -eu
: "${MIPS_CC:?}" "${CLANG:?}" "${LLC:?}" "${MIPS_LD:?}" "${OBJCOPY:?}"
cd "$(dirname "$0")"
work=$(mktemp -d "${TMPDIR:?set TMPDIR to a scratch directory}/mips-declarations.XXXXXXXX")
trap 'rm -rf "$work"' EXIT
printf 'VER_1 {}; VER_2 {} VER_1;\n' > "$work/versions.map"
for abi in hard soft; do
    fp=-m$abi-float
    for unit in calls versions; do
        "$MIPS_CC" -march=mips32r2 -mabi=32 -EB "$fp" -O1 -ffreestanding -fno-builtin \
            -fno-ident -fPIC -mabicalls -S "$unit.c" -o "$work/$unit.s"
        "$CLANG" -fdebug-compilation-dir=. -target mips-linux-gnu -march=mips32r2 -mabi=32 "$fp" \
            -c "$work/$unit.s" -o "$work/$unit.o"
    done
    "$MIPS_LD" -shared -m elf32btsmip --version-script="$work/versions.map" \
        -soname libversions.so "$work/versions.o" -o "$work/libversions.so"
    for unit in declarations aliases; do
        "$CLANG" -target mips-linux-gnu -march=mips32r2 -mabi=32 "$fp" -O2 -gdwarf-5 \
            -fdebug-compilation-dir=. -ffreestanding -fno-optimize-sibling-calls -fno-builtin \
            -S -emit-llvm "$unit.c" -o "$work/$unit.ll"
        "$LLC" -emit-call-site-info -function-sections "$work/$unit.ll" -o "$work/static.s"
        awk -f ../link-declarations.awk "$work/static.s" > "$work/debug.s"
        if [ "$abi" = soft ] && [ "$unit" = declarations ]; then
            sed -E 's/\.byte[[:space:]]+110[[:space:]]+# DW_AT_linkage_name/.uleb128 0x2007/' \
                "$work/debug.s" > "$work/legacy.s"
            grep -q 'uleb128 0x2007' "$work/legacy.s"
            mv "$work/legacy.s" "$work/debug.s"
        fi
        "$CLANG" -target mips-linux-gnu -march=mips32r2 -mabi=32 "$fp" -fdebug-compilation-dir=. \
            -c "$work/debug.s" -o "$work/$unit.o"
    done
    set --
    if [ "$abi" = soft ]; then
        "$CLANG" -target mips-linux-gnu -march=mips32r2 -mabi=32 "$fp" -x assembler -c partial.S -o "$work/partial.o"
        set -- "$work/partial.o"
    fi
    "$MIPS_LD" -shared --gc-sections --emit-relocs --build-id=none -z max-page-size=0x1000 -m elf32btsmip \
        "$work/calls.o" "$work/declarations.o" "$work/aliases.o" "$@" -L"$work" -lversions -o "$work/calls.so"
    "$OBJCOPY" -R .comment "$work/calls.so" "calls-$abi.so"
done
