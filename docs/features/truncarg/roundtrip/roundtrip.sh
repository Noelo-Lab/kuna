#!/bin/bash
# Compile-and-run round trip for narrowed call arguments.
#
# Builds rt.c with $CC -O2 (default gcc), decompiles its narrowed-argument callers
# with each given kuna, compiles that C against a driver that defines the callees
# exactly as rt.c does, and compares what the callees receive with what the binary
# passes, value by value.  r1/r6/r7 forward a narrow parameter untouched when built
# with clang, which relies on the caller's extension; r1 and r6 are then the same
# bytes, so r6's values stay wrong whatever the argument rendering (its parameter
# is typed char).
#
# usage: [CC=clang] roundtrip.sh <kuna> [<base-kuna>]
#   one kuna:  exit 1 on any mismatch.
#   two kunas: also count values the first gets wrong and the base gets right
#              (NEW_WRONG); exit 1 if there are any.
set -eu
CC=${CC:-gcc}
HERE=$(cd "$(dirname "$0")" && pwd)
W=$(mktemp -d)
cp "$HERE/rt.c" "$W/rt.c"
cd "$W"
$CC -O2 -fno-inline -w -o rt rt.c
./rt > bin.out
run() {
  : > "decomp.$2.c"
  for fn in f k a2 neg ci h bl bc r1 r6 r7; do "$1" decompile rt "$fn" >> "decomp.$2.c"; done
  {
    printf '#include <stdio.h>\n'
    printf 'unsigned char acc8; int accs;\n'
    printf 'void sink(unsigned x) { printf("%%x\\n", x); }\n'
    printf 'void sinki(int x) { printf("%%d\\n", x); }\n'
    cat "decomp.$2.c"
    sed -n '/^static const unsigned vals/,$p' rt.c
  } > "drv.$2.c"
  gcc -O0 -w -o "drv.$2" "drv.$2.c"
  "./drv.$2" > "drv.$2.out"
}
run "$(realpath "$1")" fix
bad=$(paste -d' ' bin.out drv.fix.out | awk '$1!=$2' | wc -l)
echo "ROUNDTRIP ($CC): $bad of $(wc -l < bin.out) values mismatch"
if [ $# -lt 2 ]; then
  [ "$bad" -eq 0 ]
  exit
fi
run "$(realpath "$2")" base
paste -d' ' bin.out drv.fix.out drv.base.out | awk '
  { if ($1!=$3) bw++; if ($1!=$2 && $1==$3) nw++; if ($1==$2 && $1!=$3) fx++ }
  END { printf "base mismatches %d; NEW_WRONG %d, FIXED %d\n", bw, nw, fx; exit (nw > 0) }'
