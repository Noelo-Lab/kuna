#!/bin/sh
# Regenerates the KT1 vectors with Vineflower 1.12.0's own Kotlin plugin
# (BitEncoding + shaded protobuf + MetadataNameResolver), via KtMetadataDump.java.
#
#   gen.sh             committed goldens: real.golden (subset.txt of the input
#                      jars), cases.golden (cases.in), flags.golden
#   gen.sh sweep DIR   full dumps of every input jar into DIR/<label>.txt; then
#                      KT1_SWEEP_DIR=DIR KT1_SWEEP_JARS=$(gen.sh jars) cargo test ... kt1_sweep
#   gen.sh jars        the label=jar list for KT1_SWEEP_JARS
#
# The fixture jars (labels fxutf8, fx8to7) are compiled from src/ with kotlinc
# 2.0.0, the second with -Dkotlin.jvm.serialization.use8to7=true (none of the
# real-world jars uses the 8-to-7 encoding).
# Env: KUNA_JVM_ORACLE (decompiler/nonnative/kuna-jvm/tools/oracle/README.md), or the overrides JDK
# (default $KUNA_JVM_ORACLE/jdks/jdk17), VF (the release jar), KOTLINC, WORK (default $KUNA_JVM_ORACLE/work/kt1-gen),
# KT_LIBS (the real-world jars the Kotlin oracle fetches, default $KUNA_JVM_ORACLE/corpus-kt/tools/realworld) and
# SPRING_DIR (the javabench spring-framework dataset, default $KUNA_JVM_ORACLE/dataset/spring-framework).
set -eu
HERE=$(cd "$(dirname "$0")" && pwd)
NO_ORACLE='set it to the JVM oracle workspace (decompiler/nonnative/kuna-jvm/tools/oracle/README.md)'
JDK=${JDK:-${KUNA_JVM_ORACLE:?$NO_ORACLE}/jdks/jdk17}
VF=${VF:-${KUNA_JVM_ORACLE:?$NO_ORACLE}/oracle/vineflower-1.12.0.jar}
KOTLINC=${KOTLINC:-${KUNA_JVM_ORACLE:?$NO_ORACLE}/jdks/kotlin/x200/kotlinc}
WORK=${WORK:-${KUNA_JVM_ORACLE:?$NO_ORACLE}/work/kt1-gen}
LIB=${KT_LIBS:-${KUNA_JVM_ORACLE:?$NO_ORACLE}/corpus-kt/tools/realworld}
DEPS=${SPRING_DIR:-${KUNA_JVM_ORACLE:?$NO_ORACLE}/dataset/spring-framework}

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
  echo "fxutf8=$WORK/fx/utf8.jar"
  echo "fx8to7=$WORK/fx/8to7.jar"
}

if [ "${1:-}" = jars ]; then
  jars | paste -sd, -
  exit 0
fi

mkdir -p "$WORK/fx"
(cd "$WORK" && unzip -o -q "$VF" META-INF/plugins/vineflower-kotlin-0.1.0.jar)
CP="$WORK/META-INF/plugins/vineflower-kotlin-0.1.0.jar"
"$JDK/bin/javac" -nowarn -cp "$CP" -d "$WORK" "$HERE/KtMetadataDump.java"
run() { "$JDK/bin/java" -cp "$CP:$WORK" KtMetadataDump "$@"; }

for mode in utf8 8to7; do
  if [ ! -f "$WORK/fx/$mode.jar" ]; then
    opts=""
    [ $mode = 8to7 ] && opts="-Dkotlin.jvm.serialization.use8to7=true"
    JAVA_HOME=$JDK JAVA_OPTS=$opts "$KOTLINC/bin/kotlinc" -jvm-target 17 -Xcontext-receivers \
      -Xlambdas=class -Xsam-conversions=class -module-name kt1features \
      -d "$WORK/fx/$mode" "$HERE"/src/*.kt 2>/dev/null
    (cd "$WORK/fx/$mode" && zip -q -r -X "../$mode.jar" pkg)
  fi
done

if [ "${1:-}" = sweep ]; then
  mkdir -p "$2"
  jars | while IFS== read -r label jar; do run jar "$label" "$jar" > "$2/$label.txt"; done
  exit 0
fi

jars | while IFS== read -r label jar; do run jar "$label" "$jar" "$HERE/subset.txt"; done > "$HERE/real.golden"
python3 "$HERE/gen_cases.py" > "$WORK/cases.in"
run cases "$WORK/cases.in" > "$HERE/cases.golden"
run flags > "$HERE/flags.golden"
