#!/bin/bash
# elfmain whole-corpus sweep: decompile-all on/off over every in-tree ELF fixture.
set -u
export SLEIGHHOME=/home/mahaloz/kwt/elfmain/specs; export KUNA_SPECS="$SLEIGHHOME"
K=/home/mahaloz/kwt/elfmain/decompiler/target/release/kuna
OUT=/home/mahaloz/kwt/elfmain/.scratch/sweep
FIX=/home/mahaloz/kwt/elfmain/decompiler/crates/kuna-analysis/tests/fixtures
mkdir -p "$OUT/on" "$OUT/off" "$OUT/diff"
: > "$OUT/summary.txt"
for f in "$FIX"/*; do
  [ -f "$f" ] || continue
  sz=$(stat -c%s "$f"); [ "$sz" -gt 300000 ] && continue
  head -c 4 "$f" | grep -q $'\x7fELF' || continue
  b=$(basename "$f")
  timeout 300 $K decompile-all "$f" > "$OUT/on/$b.c" 2>/dev/null
  timeout 300 $K decompile-all "$f" --option elfmain off > "$OUT/off/$b.c" 2>/dev/null
  if ! diff -q "$OUT/off/$b.c" "$OUT/on/$b.c" > /dev/null 2>&1; then
    diff -u "$OUT/off/$b.c" "$OUT/on/$b.c" > "$OUT/diff/$b.diff"
    n=$(grep -c '^[-+][^-+]' "$OUT/diff/$b.diff")
    echo "CHANGED $b lines=$n" >> "$OUT/summary.txt"
  else
    echo "same    $b" >> "$OUT/summary.txt"
  fi
done
echo "SWEEP_DONE" >> "$OUT/summary.txt"
