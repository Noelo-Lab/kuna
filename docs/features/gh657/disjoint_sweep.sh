#!/bin/bash
# disjoint 20-binary sweep: $1 = variant name, $2 = kuna binary
set -u
V=$1; K=$2
export SLEIGHHOME=/home/mahaloz/kwt/gh657/specs
export KUNA_SPECS=/home/mahaloz/kwt/gh657/specs
R=/home/mahaloz/github/decbench/results/full_run_address_2026-09-11
OUT=/home/mahaloz/kwt/gh657/.scratch/corpus2/$V
mkdir -p "$OUT"
declare -a LIST=(
  "tar tar/stripped/tar" "dash dash/stripped/dash" "su shadow/stripped/su"
  "e2fsck e2fsprogs/stripped/e2fsck" "kmod kmod/stripped/kmod"
  "ls coreutils/stripped/ls" "ip iproute2/stripped/ip"
  "libedit libedit/stripped/libedit.so.0.0.70" "zlib zlib/stripped/libz.so.1.3.1"
  "ssh openssh-portable/stripped/ssh"
)
for opt in O0 O2; do
  for e in "${LIST[@]}"; do
    n=${e%% *}; rel=${e#* }; p="$R/$opt/$rel"
    [ -f "$p" ] || { echo "MISS $p"; continue; }
    "$K" decompile-all "$p" > "$OUT/$opt-$n.c" 2> "$OUT/$opt-$n.err"
    echo "$V $opt-$n rc=$? $(wc -l < "$OUT/$opt-$n.c")"
  done
done
echo "DONE-$V"
