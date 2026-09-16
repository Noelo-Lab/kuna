#!/bin/bash
set -u
export SLEIGHHOME=/home/mahaloz/kwt/elfmain/specs; export KUNA_SPECS="$SLEIGHHOME"
K=/home/mahaloz/kwt/elfmain/decompiler/target/release/kuna
IFS=$'\t' read -r s u label <<< "$1"
got=$(timeout 180 $K functions "$s" 2>/dev/null | awk -F'\t' '$2=="main"{print $1}' | head -1)
want=$(nm "$u" 2>/dev/null | awk 'tolower($2)=="t" && $3=="main"{print "0x"$1}' | head -1)
gotn=$(python3 -c "import sys;print(int(sys.argv[1],16) if sys.argv[1] else -1)" "${got:-}" 2>/dev/null)
wantn=$(python3 -c "import sys;print(int(sys.argv[1],16) if sys.argv[1] else -2)" "${want:-}" 2>/dev/null)
if [ -z "$got" ]; then v=MISS; elif [ "$gotn" = "$wantn" ]; then v=MATCH; else v=MISMATCH; fi
printf "%s\t%s\tgot=%s\twant=%s\n" "$v" "$label" "${got:-none}" "${want:-none}"
