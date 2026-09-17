#!/usr/bin/env bash
set -u
WT=/home/mahaloz/kwt/indirectonly
K=$WT/decompiler/target/release/kuna
R=/home/mahaloz/github/decbench/results/full_run_address_2026-09-11
OUT=$WT/.scratch/sweep3
export SLEIGHHOME=$WT/specs
export KUNA_SPECS=$WT/specs
one() {
  local f=$1 tag=$2
  [ -f "$f" ] || { echo "MISSING $f"; return; }
  [ -f "$OUT/$tag.done" ] && { echo "SKIP $tag"; return; }
  "$K" decompile-all "$f" > "$OUT/$tag-off.c" 2> "$OUT/$tag-off.err"
  "$K" decompile-all "$f" --option indirectonly on > "$OUT/$tag-on.c" 2> "$OUT/$tag-on.err"
  n=$(diff "$OUT/$tag-off.c" "$OUT/$tag-on.c" | grep -c '^[<>]')
  touch "$OUT/$tag.done"
  echo "SLICE $tag difflines=$n"
}
declare -A P=( [ls]=coreutils [sort]=coreutils [grep]=grep [gzip]=gzip [diff]=diffutils \
               [bzip2]=bzip2 [find]=findutils [tar]=tar [kmod]=kmod [e2fsck]=e2fsprogs \
               [dpkg]=dpkg [dpkg-query]=dpkg [dash]=dash [bash]=bash )
JOBS=4
run() { while [ "$(jobs -rp | wc -l)" -ge $JOBS ]; do wait -n; done; one "$1" "$2" & }
for opt in O0 O2; do
  for b in ls sort grep gzip diff bzip2 find tar; do run "$R/$opt/${P[$b]}/stripped/$b" "$b-$opt"; done
done
for opt in O0 O2 O2-noinline; do
  for b in kmod e2fsck dpkg dpkg-query dash bash; do run "$R/$opt/${P[$b]}/stripped/$b" "$b-$opt"; done
done
wait
echo "SWEEP3_DONE"
