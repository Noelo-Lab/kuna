#!/bin/sh
# Regenerates golden.txt from Vineflower 1.12.0 and its bundled Kotlin plugin:
#   sh gen.sh [vineflower-1.12.0.jar]   (default: $KUNA_JVM_ORACLE/oracle/vineflower-1.12.0.jar)
# Env: JDK (default $KUNA_JVM_ORACLE/jdks/jdk17).
set -e
cd "$(dirname "$0")"
JAR=${1:-${KUNA_JVM_ORACLE:?set it to the JVM oracle workspace (decompiler/nonnative/kuna-jvm/tools/oracle/README.md)}/oracle/vineflower-1.12.0.jar}
JDK=${JDK:-${KUNA_JVM_ORACLE:?set it to the JVM oracle workspace (decompiler/nonnative/kuna-jvm/tools/oracle/README.md)}/jdks/jdk17}
OUT=$(mktemp -d)
"$JDK/bin/javac" --release 8 -d classes src/kt5/KtProps.java
(cd "$OUT" && unzip -o -q "$JAR" META-INF/plugins/vineflower-kotlin-0.1.0.jar)
CP="$JAR:$OUT/META-INF/plugins/vineflower-kotlin-0.1.0.jar"
"$JDK/bin/javac" -nowarn -cp "$CP" -d "$OUT" GenKt5Oracle.java
"$JDK/bin/java" -cp "$CP:$OUT" GenKt5Oracle ../b02/b02/B02Fields.class cases.txt > golden.txt
