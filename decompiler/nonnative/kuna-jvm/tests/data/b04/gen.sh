#!/bin/sh
# Regenerates b04_golden.txt from the real Vineflower 1.12.0 classes.
#   sh gen.sh ~/kwt/jvm/oracle/vineflower-1.12.0.jar
set -e
cd "$(dirname "$0")"
JAR=${1:?usage: gen.sh <vineflower-1.12.0.jar>}
OUT=$(mktemp -d)
javac -nowarn -cp "$JAR" -d "$OUT" B04Oracle.java
java -cp "$JAR:$OUT" org.jetbrains.java.decompiler.modules.decompiler.stats.B04Oracle > b04_golden.txt
