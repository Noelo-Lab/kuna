#!/bin/sh
set -eu
: "${MIPS_CC:?}" "${CLANG:?}" "${LLC:?}" "${MIPS_LD:?}" "${OBJCOPY:?}" "${READELF:?}"
cd "$(dirname "$0")"
work=$(mktemp -d "${TMPDIR:?set TMPDIR to a scratch directory}/mips-declarations.XXXXXXXX")
trap 'rm -rf "$work"' EXIT

# Assemble GCC's PIC callers separately from the debug producer.
float=-mhard-float
calls() {
    "$MIPS_CC" $abi "$float" -O1 -ffreestanding -fno-builtin -fno-ident -fPIC -mabicalls \
        -S "$@" -o "$work/calls.s"
    "$CLANG" -target "$target" $abi "$float" -fdebug-compilation-dir=. \
        -c "$work/calls.s" -o "$work/calls.o"
}
declarations() {
    "$CLANG" -target "$target" $abi -mhard-float -ffreestanding -fno-builtin -fdebug-compilation-dir=. \
        -O2 -gdwarf-5 -fno-optimize-sibling-calls -S -emit-llvm "$@" -o "$work/decl.ll"
    "$LLC" -emit-call-site-info -function-sections "$work/decl.ll" -o "$work/static.s"
    awk -f ../link-declarations.awk "$work/static.s" > "$work/pic.s"
    "$CLANG" -target "$target" $abi "$float" -fdebug-compilation-dir=. -c "$work/pic.s" -o "$work/decl.o"
}
link_image() {
    image=$1
    shift
    "$MIPS_LD" -m "$em" -shared --gc-sections --build-id=none -z max-page-size=0x1000 \
        "$work/calls.o" "$@" -o "$work/image.so"
    "$OBJCOPY" -R .comment "$work/image.so" "$image"
}

abi='-march=mips32r2 -mabi=32'
target=mips-linux-gnu
em=elf32btsmip
calls -DSTRIPPED o32.c
link_image "$work/raw.so"
# Keep distinct symbol values but make the one-pointer slot share bzero's stub.
"$OBJCOPY" --dump-section .got="$work/got" "$work/raw.so"
"$READELF" --dyn-syms -W "$work/raw.so" > "$work/dynsyms"
python3 - "$work/got" "$work/dynsyms" <<'PYTHON'
from pathlib import Path
import sys
got, symbols = map(Path, sys.argv[1:])
stubs = {row[-1]: int(row[1], 16).to_bytes(4, "big")
         for line in symbols.read_text().splitlines() if (row := line.split())
         and row[-1] in {"take_one", "bzero"}}
data = got.read_bytes()
words = [data[i:i + 4] for i in range(0, len(data), 4)]
assert all(words.count(stubs[name]) == 1 for name in ("take_one", "bzero"))
words[words.index(stubs["take_one"])] = stubs["bzero"]
got.write_bytes(b"".join(words))
PYTHON
"$OBJCOPY" --strip-debug --update-section .got="$work/got" "$work/raw.so" o32-shared.so

calls scoped.c
declarations scoped.cpp
link_image scoped.so "$work/decl.o" --emit-relocs
