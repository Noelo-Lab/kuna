#!/bin/sh
# Regenerates b16_golden.txt from the real Vineflower 1.12.0.
#   sh gen.sh ~/kwt/jvm/oracle/vineflower-1.12.0.jar
set -e
cd "$(dirname "$0")"
JAR=${1:?usage: gen.sh <vineflower-1.12.0.jar>}
OUT=$(mktemp -d)
javac -cp "$JAR" -d "$OUT" GenB16Oracle.java
java -cp "$JAR:$OUT" GenB16Oracle > b16_golden.txt
javac --release 17 -d . B16Rec.java
