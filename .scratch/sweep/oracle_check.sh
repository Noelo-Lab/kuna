#!/bin/bash
# Is the address elfmain names `main` the address DWARF/symtab calls `main`?
set -u
export SLEIGHHOME=/home/mahaloz/kwt/elfmain/specs; export KUNA_SPECS="$SLEIGHHOME"
K=/home/mahaloz/kwt/elfmain/decompiler/target/release/kuna
R=/home/mahaloz/github/decbench/results/full_run_address_2026-09-11
OUT=/home/mahaloz/kwt/elfmain/.scratch/sweep/oracle.txt
: > "$OUT"
for opt in O0 O2 O2-noinline; do
  for s in "$R/$opt"/*/stripped/*; do
    [ -f "$s" ] || continue
    b=$(basename "$s"); proj=$(basename "$(dirname "$(dirname "$s")")")
    u="$R/$opt/$proj/compiled/$b"
    [ -f "$u" ] || continue
    head -c 4 "$s" | grep -q $'\x7fELF' || continue
    got=$(timeout 120 $K functions "$s" 2>/dev/null | awk -F'\t' '$2=="main"{print $1}' | head -1)
    want=$(nm "$u" 2>/dev/null | awk 'tolower($2)=="t" && $3=="main"{print "0x"$1}' | head -1)
    [ -z "$want" ] && want=$(readelf -W --syms "$u" 2>/dev/null | awk '$8=="main" && $4=="FUNC"{printf "0x%x\n", strtonum("0x"$2)}' | head -1)
    wantn=$(python3 -c "import sys;print(int(sys.argv[1],16) if sys.argv[1] else -1)" "${want:-}" 2>/dev/null)
    gotn=$(python3 -c "import sys;print(int(sys.argv[1],16) if sys.argv[1] else -1)" "${got:-}" 2>/dev/null)
    if [ -z "$got" ]; then v=MISS; elif [ "$gotn" = "$wantn" ]; then v=MATCH; else v=MISMATCH; fi
    echo -e "$v\t$opt/$proj/$b\tgot=${got:-none}\twant=${want:-none}" >> "$OUT"
  done
done
echo DONE >> "$OUT"
