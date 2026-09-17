#!/bin/bash
# The 37-binary corpus sweep. $1 = out subdir, $2 = kuna binary, $3 = extra args.
# KUNA_GH657_VARIANT selects one of the instrumented second-span builds used for
# the cause attribution in analysis.md (the instrumentation is not committed; the
# variants it selects are described in cause.py).
# $1 = out subdir name under .scratch/sweep36, $2 = kuna binary, $3 = extra args (may be empty)
set -u
V=$1; K=$2; EXTRA=${3:-}
export SLEIGHHOME=/home/mahaloz/kwt/gh657/specs
export KUNA_SPECS=/home/mahaloz/kwt/gh657/specs
R=/home/mahaloz/github/decbench/results/full_run_address_2026-09-11
T=/home/mahaloz/kwt/gh657/tests/bug-repro
OUT=/home/mahaloz/kwt/gh657/.scratch/sweep36/$V
mkdir -p "$OUT"
list() {
cat <<L
O0-coreutils-fmt $R/O0/coreutils/stripped/fmt
O2-coreutils-fmt $R/O2/coreutils/stripped/fmt
O0-grep $R/O0/grep/stripped/grep
O2-grep $R/O2/grep/stripped/grep
O0-gzip $R/O0/gzip/stripped/gzip
O2-gzip $R/O2/gzip/stripped/gzip
O0-diff $R/O0/diffutils/stripped/diff
O2-diff $R/O2/diffutils/stripped/diff
O0-bzip2 $R/O0/bzip2/stripped/bzip2
O2-bzip2 $R/O2/bzip2/stripped/bzip2
O0-find $R/O0/findutils/stripped/find
O2-find $R/O2/findutils/stripped/find
tree-faillog $T/faillog
tree-sort $T/sort
tree-grep $T/grep
tree-libselinux $T/libselinux.so.1
O0-tar $R/O0/tar/stripped/tar
O2-tar $R/O2/tar/stripped/tar
O0-dash $R/O0/dash/stripped/dash
O2-dash $R/O2/dash/stripped/dash
O0-su $R/O0/shadow/stripped/su
O2-su $R/O2/shadow/stripped/su
O0-e2fsck $R/O0/e2fsprogs/stripped/e2fsck
O2-e2fsck $R/O2/e2fsprogs/stripped/e2fsck
O0-kmod $R/O0/kmod/stripped/kmod
O2-kmod $R/O2/kmod/stripped/kmod
O0-ls $R/O0/coreutils/stripped/ls
O2-ls $R/O2/coreutils/stripped/ls
O0-ip $R/O0/iproute2/stripped/ip
O2-ip $R/O2/iproute2/stripped/ip
O0-libedit $R/O0/libedit/stripped/libedit.so.0.0.70
O2-libedit $R/O2/libedit/stripped/libedit.so.0.0.70
O0-zlib $R/O0/zlib/stripped/libz.so.1.2.13
O2-libselinux $R/O2/libselinux/stripped/libselinux.so.1
O2-libacl $R/O2/libacl/stripped/libacl.so.1.1.2301
O0-ssh $R/O0/openssh-portable/stripped/ssh
O2-ssh $R/O2/openssh-portable/stripped/ssh
L
}
one() {
  n=$1; b=$2
  [ -f "$b" ] || { echo "MISS $n"; return; }
  /usr/bin/timeout 1800 "$K" decompile-all "$b" $EXTRA > "$OUT/$n.c" 2> "$OUT/$n.err"
  echo "$n rc=$? lines=$(wc -l < "$OUT/$n.c")"
}
export -f one; export OUT K EXTRA
list | xargs -P 10 -L 1 bash -c 'one "$0" "$1"'
echo "SWEEP_DONE_$V"
