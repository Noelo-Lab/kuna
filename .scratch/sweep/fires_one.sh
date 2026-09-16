#!/bin/bash
set -u
export SLEIGHHOME=/home/mahaloz/kwt/elfmain/specs; export KUNA_SPECS="$SLEIGHHOME"
K=/home/mahaloz/kwt/elfmain/decompiler/target/release/kuna
IFS=$'\t' read -r s u label <<< "$1"
on=$(timeout 240 $K functions "$s" 2>/dev/null | awk -F'\t' '$2=="main"{print $1}' | head -1)
off=$(timeout 240 $K functions "$s" --option elfmain off 2>/dev/null | awk -F'\t' '$2=="main"{print $1}' | head -1)
if [ -n "$on" ] && [ -z "$off" ]; then v=FIRES
elif [ -n "$on" ] && [ -n "$off" ]; then v=PREEXISTING
elif [ -z "$on" ] && [ -z "$off" ]; then v=NONE
else v=LOST; fi
printf "%s\t%s\ton=%s\toff=%s\n" "$v" "$label" "${on:-none}" "${off:-none}"
