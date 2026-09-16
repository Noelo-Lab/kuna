#!/bin/bash
# elfmain: whole-binary decompile-all diff, on vs off, over real stripped decbench ELFs.
# Also a cross-build control: this branch with the option OFF vs the base-commit binary.
set -u
export SLEIGHHOME=/home/mahaloz/kwt/elfmain/specs; export KUNA_SPECS="$SLEIGHHOME"
K=/home/mahaloz/kwt/elfmain/decompiler/target/release/kuna
B=/home/mahaloz/kwt/_baseline/kuna
R=/home/mahaloz/github/decbench/results/full_run_address_2026-09-11
OUT=/home/mahaloz/kwt/elfmain/.scratch/realsweep
mkdir -p "$OUT/on" "$OUT/off" "$OUT/base" "$OUT/diff"
: > "$OUT/summary.txt"
TARGETS="O0/grep/grep O2/tar/tar O2-noinline/diffutils/cmp O2/gzip/gzip O0/coreutils/ls O2/findutils/find O0/dash/dash O2/coreutils/fmt"
for t in $TARGETS; do
  opt=${t%%/*}; rest=${t#*/}; proj=${rest%%/*}; bin=${rest#*/}
  s="$R/$opt/$proj/stripped/$bin"
  if [ ! -f "$s" ]; then echo "MISSING $t" >> "$OUT/summary.txt"; continue; fi
  tag="${opt}_${proj}_${bin}"
  timeout 3000 $K decompile-all "$s" > "$OUT/on/$tag.c" 2>/dev/null
  timeout 3000 $K decompile-all "$s" --option elfmain off > "$OUT/off/$tag.c" 2>/dev/null
  timeout 3000 $B decompile-all "$s" > "$OUT/base/$tag.c" 2>/dev/null
  if diff -q "$OUT/off/$tag.c" "$OUT/base/$tag.c" >/dev/null 2>&1; then ctrl=IDENTICAL; else ctrl=DIFFERS; fi
  if diff -q "$OUT/off/$tag.c" "$OUT/on/$tag.c" >/dev/null 2>&1; then
    echo -e "same\t$tag\tcontrol=$ctrl" >> "$OUT/summary.txt"
  else
    diff -u "$OUT/off/$tag.c" "$OUT/on/$tag.c" > "$OUT/diff/$tag.diff"
    n=$(grep -c '^[-+][^-+]' "$OUT/diff/$tag.diff")
    echo -e "CHANGED\t$tag\tlines=$n\tcontrol=$ctrl" >> "$OUT/summary.txt"
  fi
done
echo REALSWEEP_DONE >> "$OUT/summary.txt"
