#!/bin/bash
# For every image in the oracle run: does the STRIPPED image itself still name a
# symbol `main` (the pass's ambiguity refusal), and does it name __libc_start_main?
set -u
IFS=$'\t' read -r s u label <<< "$1"
syms=$(readelf -W --syms --dyn-syms "$s" 2>/dev/null)
has_main=no;  echo "$syms" | awk '{print $8}' | grep -qx 'main' && has_main=yes
has_lsm=no;   echo "$syms" | awk '{print $8}' | sed 's/@.*//' | grep -qx '__libc_start_main' && has_lsm=yes
printf "%s\thas_main=%s\thas_libc_start_main=%s\n" "$label" "$has_main" "$has_lsm"
