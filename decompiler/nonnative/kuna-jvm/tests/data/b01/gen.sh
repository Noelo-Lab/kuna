#!/bin/sh
# Regenerates B01Calls.class and b01_golden.txt from the real Vineflower 1.12.0.
#   sh gen.sh ~/kwt/jvm/oracle/vineflower-1.12.0.jar
set -e
cd "$(dirname "$0")"
JAR=${1:?usage: gen.sh <vineflower-1.12.0.jar>}
OUT=$(mktemp -d)
javac --release 17 -d . B01Calls.java
javac -cp "$JAR" -d "$OUT" GenB01Oracle.java
java -cp "$JAR:$OUT" GenB01Oracle B01Calls.class > b01_golden.txt
