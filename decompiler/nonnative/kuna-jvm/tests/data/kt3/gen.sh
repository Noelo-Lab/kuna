#!/bin/sh
# Regenerates the KT3 vectors with Vineflower 1.12.0's own Kotlin plugin, via
# KtElementDump.java (KType, KTypeParameter, KParameter, KContract, KTypes,
# KUtils, KotlinImportCollector and KotlinChooser on real and crafted input).
#
#   gen.sh              committed vectors: meta.golden (KT1's fixtures, the
#                       Kotlin corpus, subset.txt of the real-world jars),
#                       chooser.golden, kotlintypes.golden, misc.golden; also
#                       recompiles classes/ from src/
#   gen.sh sweep DIR    full dumps of every input into DIR; then
#                       KT3_SWEEP_DIR=DIR cargo test -p kuna-jvm --test integration kt3_sweep -- --ignored
set -eu
HERE=$(cd "$(dirname "$0")" && pwd)
JDK=${JDK:-$HOME/kwt/jvm/jdks/jdk17}
VF=${VF:-$HOME/kwt/jvm/oracle/vineflower-1.12.0.jar}
CORPUS=${CORPUS:-$HOME/kwt/jvm/corpus-kt/classes}
WORK=${WORK:-$HOME/kwt/jvm/scratch/kt3/gen}
LIB=/opt/joern/joern-cli/frontends/kotlin2cpg/lib
DEPS=$HOME/kwt/jvm/dataset/spring-framework

jars() {
  echo "spring=$DEPS/classes.jar"
  echo "stdlib=$DEPS/deps/kotlin-stdlib-2.3.21.jar"
  echo "reflect=$DEPS/deps/kotlin-reflect-2.3.21.jar"
  echo "coroutines110=$DEPS/deps/kotlinx-coroutines-core-jvm-1.10.2.jar"
  echo "okio=$LIB/com.squareup.okio.okio-2.6.0.jar"
  echo "okhttp=$LIB/com.squareup.okhttp3.okhttp-4.7.2.jar"
  echo "coroutines=$LIB/org.jetbrains.kotlinx.kotlinx-coroutines-core-jvm-1.8.0.jar"
  echo "moshi=$LIB/com.squareup.moshi.moshi-kotlin-1.9.3.jar"
  echo "reflect16=$LIB/org.jetbrains.kotlin.kotlin-reflect-1.6.10.jar"
}

dirs() {
  echo "kt1fx=$HERE/../kt1/classes"
  echo "kt=$CORPUS/kt"
  echo "ktold=$CORPUS/ktold"
  echo "corekt=$CORPUS/core-kt"
}

mkdir -p "$WORK"
(cd "$WORK" && unzip -o -q "$VF" META-INF/plugins/vineflower-kotlin-0.1.0.jar)
CP="$VF:$WORK/META-INF/plugins/vineflower-kotlin-0.1.0.jar"
"$JDK/bin/javac" -nowarn -cp "$CP" -d "$WORK" "$HERE/KtElementDump.java"
run() { "$JDK/bin/java" -Xmx2g -cp "$CP:$WORK" KtElementDump "$@"; }

if [ "${1:-}" = sweep ]; then
  mkdir -p "$2"
  { dirs; jars; } | while IFS== read -r label path; do
    run meta "$label" "$path" > "$2/meta-$label.txt"
    run kotlintypes "$label" "$path" > "$2/kotlintypes-$label.txt"
  done
  exit 0
fi

for s in a b1 b2 b3 c; do
  "$JDK/bin/javac" -nowarn -g:none --release 11 -d "$WORK/stub-$s" "$HERE"/src/stub-$s/kotlin/*.java \
    $(find "$HERE/src/stub-$s" -path '*/jvm/*' -name '*.java')
  "$JDK/bin/javac" -nowarn -g:none --release 11 -cp "$WORK/stub-$s" -d "$HERE/classes" "$HERE"/src/$s/*.java
done

{
  dirs | while IFS== read -r label path; do run meta "$label" "$path"; done
  jars | while IFS== read -r label path; do run meta "$label" "$path" "$HERE/subset.txt"; done
} > "$HERE/meta.golden"
{
  run chooser kt3 "$HERE/classes"
  run chooser kt1fx "$HERE/../kt1/classes"
} > "$HERE/chooser.golden"
{
  run kotlintypes crafted -
  dirs | while IFS== read -r label path; do run kotlintypes "$label" "$path"; done
  jars | while IFS== read -r label path; do run kotlintypes "$label" "$path" 31; done
} > "$HERE/kotlintypes.golden"
run misc > "$HERE/misc.golden"
