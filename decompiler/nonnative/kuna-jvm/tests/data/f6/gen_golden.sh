#!/bin/sh
# Records what Vineflower 1.12.0 registers for each fixture (golden/*.txt).
# usage: gen_golden.sh <vineflower-1.12.0.jar>
set -eu
here=$(cd "$(dirname "$0")" && pwd)
vf=$1
work=$(mktemp -d)
javac -cp "$vf" -d "$work" "$here/VfDump.java"
mkdir -p "$here/golden"
dump() {
  out=$1
  shift
  java -cp "$vf:$work" org.jetbrains.java.decompiler.struct.VfDump space "$@" | sed "s#$here#\$DATA#g" > "$here/golden/$out.txt"
}
for f in mixed.jar mr.jar badver.jar dup.zip prefixed.zip trailing.zip comment.zip zip64.zip mrtool.jar inner.jar; do
  dump "${f%.*}" "$here/$f"
done
dump tree "$here/tree"
dump single "$here/tree/pkg/Hello.class" "$here/tree/readme.txt" "$here/tree/missing.class"
