#!/bin/bash
# Compile-and-run round trip for narrowed call arguments.
#
# Builds rt.c with gcc -O2, decompiles its eight narrowed-argument callers with the
# given kuna, compiles that C against a driver that defines the callees exactly as
# rt.c does, and compares what the callees receive with what the binary passes.
#
# usage: roundtrip.sh <path-to-kuna> [workdir]
set -eu
K=$(realpath "$1")
W=${2:-$(mktemp -d)}
HERE=$(cd "$(dirname "$0")" && pwd)
cp "$HERE/rt.c" "$W/rt.c"
cd "$W"
gcc -O2 -fno-inline -w -o rt rt.c
./rt > bin.out
: > decomp.c
for fn in f k a2 neg ci h bl bc; do "$K" decompile rt "$fn" >> decomp.c; done
{
  printf '#include <stdio.h>\n'
  printf 'void sink(unsigned x) { printf("%%x\\n", x); }\n'
  printf 'void sinki(int x) { printf("%%d\\n", x); }\n'
  cat decomp.c
  sed -n '/^static const unsigned vals/,$p' rt.c
} > drv.c
gcc -O0 -w -o drv drv.c
./drv > drv.out
if cmp -s bin.out drv.out; then
  echo "ROUNDTRIP: MATCH ($(wc -l < bin.out) values)"
else
  echo "ROUNDTRIP: MISMATCH on $(diff bin.out drv.out | grep -c '^<') of $(wc -l < bin.out) values"
  exit 1
fi
