#!/bin/sh
# Regenerates b16_golden.txt from the real Vineflower 1.12.0.
#   sh gen.sh [vineflower-1.12.0.jar]   (default: $KUNA_JVM_ORACLE/oracle/vineflower-1.12.0.jar)
set -e
cd "$(dirname "$0")"
JAR=${1:-${KUNA_JVM_ORACLE:?set it to the JVM oracle workspace (decompiler/nonnative/kuna-jvm/tools/oracle/README.md)}/oracle/vineflower-1.12.0.jar}
OUT=$(mktemp -d)
javac -cp "$JAR" -d "$OUT" GenB16Oracle.java
java -cp "$JAR:$OUT" GenB16Oracle > b16_golden.txt
javac --release 17 -d . B16Rec.java
