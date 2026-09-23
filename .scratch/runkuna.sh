#!/bin/bash
set -u
export SLEIGHHOME=/home/mahaloz/kwt/castcensus/specs
export KUNA_SPECS=/home/mahaloz/kwt/castcensus/specs
K=/home/mahaloz/kwt/castcensus/decompiler/target/release/kuna
R=/home/mahaloz/github/decbench/results/full_run_address_2026-09-11
OUT=/home/mahaloz/kwt/castcensus/.scratch/cur
mkdir -p $OUT
while read -r opt proj b; do
  [ -z "$opt" ] && continue
  tag="${opt}__${proj}__${b}"
  [ -s "$OUT/$tag.json" ] && continue
  timeout 1800 $K decompile-all "$R/$opt/$proj/stripped/$b" --json > "$OUT/$tag.json" 2>"$OUT/$tag.err"
  echo "$tag rc=$? size=$(stat -c%s $OUT/$tag.json 2>/dev/null)"
done < /home/mahaloz/kwt/castcensus/.scratch/binlist.txt
echo RUNKUNA_DONE
