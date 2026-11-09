#!/bin/sh
# Regenerates const_golden.txt and function_golden.txt from the real Vineflower 1.12.0.
#   sh gen.sh ~/kwt/jvm/oracle/vineflower-1.12.0.jar
set -e
cd "$(dirname "$0")"
JAR=${1:?usage: gen.sh <vineflower-1.12.0.jar>}
OUT=$(mktemp -d)
javac -cp "$JAR" -d "$OUT" GenConstOracle.java
java -cp "$JAR:$OUT" GenConstOracle ../f6/tree/pkg/Hello.class > const_golden.txt
javac --release 8 -d . B02Fields.java
javac -cp "$JAR" -d "$OUT" GenFunctionOracle.java
java -cp "$JAR:$OUT" GenFunctionOracle b02/B02Fields.class > function_golden.txt
