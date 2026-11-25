#!/usr/bin/env bash
# Regenerates the kp1_plugins fixture for tests/naming_plugins.rs:
#  - classes/ (-g) and classes_nodebug/ (-g:none): src/ compiled by javac 8;
#  - the last column of variants.tsv: for each row, the digest (tests/jvm/regen.py's text_digest) of what
#    the release jar with its bundled plugins prints (vf-decompile --fixture --plugins) from
#    <classes dir>/p/<Main>*.class.
# WORK (default: a new mktemp directory) receives the runs; WORK/run/<variant>/<Main>.java is the text.
# Env: KUNA_JVM_ORACLE (decompiler/nonnative/kuna-jvm/tools/oracle/README.md), or the overrides
# JDK8 (default $KUNA_JVM_ORACLE/jdks/jdk8) and VF_DECOMPILE (default the oracle tooling's java/bin/vf-decompile).
set -euo pipefail
cd "$(dirname "$0")"
NO_ORACLE='set it to the JVM oracle workspace (decompiler/nonnative/kuna-jvm/tools/oracle/README.md)'
JDK8=${JDK8:-${KUNA_JVM_ORACLE:?$NO_ORACLE}/jdks/jdk8}
VF_DECOMPILE=${VF_DECOMPILE:-$(cd ../../../tools/oracle/java/bin && pwd)/vf-decompile}
WORK=${WORK:-$(mktemp -d)}
TESTS_JVM=$(cd ../../../../../../tests/jvm && pwd)
mkdir -p "$WORK"

mapfile -t srcs < <(find src -name '*.java' | LC_ALL=C sort)
mkdir -p classes classes_nodebug
"$JDK8/bin/javac" -source 1.8 -target 1.8 -encoding UTF-8 -g -proc:none -d classes "${srcs[@]}"
"$JDK8/bin/javac" -source 1.8 -target 1.8 -encoding UTF-8 -g:none -proc:none -d classes_nodebug "${srcs[@]}"

tsv=$WORK/variants.tsv
: > "$tsv"
while IFS= read -r row; do
  IFS=$'\t' read -r variant dir main opts _ <<< "$row"
  case "$variant" in '#'*|'') printf '%s\n' "$row" >> "$tsv"; continue ;; esac
  args=()
  IFS=';' read -r -a pairs <<< "$opts"
  for kv in "${pairs[@]}"; do args+=(--opt "$kv"); done
  mapfile -t siblings < <(find "$dir/p" -maxdepth 1 -name "$main\$*.class" | LC_ALL=C sort)
  classes=("$dir/p/$main.class" "${siblings[@]}")
  out="$WORK/run/$variant"
  "$VF_DECOMPILE" --fixture --plugins "${args[@]}" "${classes[@]}" -o "$out" > "$WORK/$variant.log" 2>&1
  digest=$(python3 -I -c 'import sys; sys.path.append(sys.argv[1]); from regen import file_digest; print(file_digest(sys.argv[2]))' \
    "$TESTS_JVM" "$out/$main.java")
  printf '%s\t%s\t%s\t%s\t%s\n' "$variant" "$dir" "$main" "$opts" "$digest" >> "$tsv"
done < variants.tsv
cp "$tsv" variants.tsv
