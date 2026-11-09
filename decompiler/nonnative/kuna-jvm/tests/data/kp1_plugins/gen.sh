#!/usr/bin/env bash
# Regenerates the kp1_plugins fixture for tests/kp1_plugins.rs:
#  - classes/ (-g) and classes_nodebug/ (-g:none): src/ compiled by javac 8;
#  - golden.txt: GenPluginsOracle on JDK 21 with the Vineflower 1.12.0 jar and the
#    VariableRenaming plugin jar the release jar bundles (META-INF/plugins);
#  - expected/<variant>/<Main>.java: each variants.tsv row decompiled by the release jar with
#    its bundled plugins (vf-decompile --fixture --plugins) from <classes dir>/p/<Main>*.class.
# WORK (default: a new mktemp directory) receives the extracted plugin jars and the runs.
set -euo pipefail
cd "$(dirname "$0")"
VF_JAR=${VINEFLOWER_JAR:-$HOME/kwt/jvm/oracle/vineflower-1.12.0.jar}
JDK8=${JDK8:-$HOME/kwt/jvm/jdks/jdk8}
VF_DECOMPILE=${VF_DECOMPILE:-$HOME/kwt/jvm/corpus/bin/vf-decompile}
WORK=${WORK:-$(mktemp -d)}
mkdir -p "$WORK"

unzip -q -o "$VF_JAR" 'META-INF/plugins/vineflower-variable-renaming.jar' -d "$WORK/jars"
mapfile -t srcs < <(find src -name '*.java' | LC_ALL=C sort)
mkdir -p classes classes_nodebug
"$JDK8/bin/javac" -source 1.8 -target 1.8 -encoding UTF-8 -g -proc:none -d classes "${srcs[@]}"
"$JDK8/bin/javac" -source 1.8 -target 1.8 -encoding UTF-8 -g:none -proc:none -d classes_nodebug "${srcs[@]}"

java -cp "$VF_JAR:$WORK/jars/META-INF/plugins/vineflower-variable-renaming.jar" GenPluginsOracle.java . > golden.txt

while IFS=$'\t' read -r variant dir main opts; do
  case "$variant" in '#'*|'') continue ;; esac
  args=()
  IFS=';' read -r -a pairs <<< "$opts"
  for kv in "${pairs[@]}"; do args+=(--opt "$kv"); done
  mapfile -t siblings < <(find "$dir/p" -maxdepth 1 -name "$main\$*.class" | LC_ALL=C sort)
  classes=("$dir/p/$main.class" "${siblings[@]}")
  out="$WORK/run/$variant"
  "$VF_DECOMPILE" --fixture --plugins "${args[@]}" "${classes[@]}" -o "$out" > "$WORK/$variant.log" 2>&1
  mkdir -p "expected/$variant"
  cp "$out/$main.java" "expected/$variant/$main.java"
done < variants.tsv
