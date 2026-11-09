#!/bin/sh
# Regenerates k1_golden.txt from the real Vineflower 1.12.0 and the K1Rec.class
# fixture (any class works: PatternExprent only holds the StructClass handle).
#   sh gen.sh [vineflower-1.12.0.jar]   (default: $KUNA_JVM_ORACLE/oracle/vineflower-1.12.0.jar)
set -e
cd "$(dirname "$0")"
JAR=${1:-${KUNA_JVM_ORACLE:?set it to the JVM oracle workspace (decompiler/nonnative/kuna-jvm/tools/oracle/README.md)}/oracle/vineflower-1.12.0.jar}
OUT=$(mktemp -d)
javac -cp "$JAR" -d "$OUT" GenK1Oracle.java
java -cp "$JAR:$OUT" GenK1Oracle > k1_golden.txt
javac --release 8 -d . K1Rec.java
