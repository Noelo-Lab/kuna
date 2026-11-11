#!/bin/sh
# Regenerates b04_golden.txt from the real Vineflower 1.12.0 classes.
#   sh gen.sh [vineflower-1.12.0.jar]   (default: $KUNA_JVM_ORACLE/oracle/vineflower-1.12.0.jar)
set -e
cd "$(dirname "$0")"
JAR=${1:-${KUNA_JVM_ORACLE:?set it to the JVM oracle workspace (decompiler/nonnative/kuna-jvm/tools/oracle/README.md)}/oracle/vineflower-1.12.0.jar}
OUT=$(mktemp -d)
javac -nowarn -cp "$JAR" -d "$OUT" B04Oracle.java
java -cp "$JAR:$OUT" org.jetbrains.java.decompiler.modules.decompiler.stats.B04Oracle > b04_golden.txt
