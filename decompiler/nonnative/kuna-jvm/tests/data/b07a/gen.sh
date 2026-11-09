#!/bin/sh
# Regenerates b07a_golden.txt from the real Vineflower 1.12.0 (JDK 21 on PATH).
#   sh gen.sh ~/kwt/jvm/oracle/vineflower-1.12.0.jar
set -e
cd "$(dirname "$0")"
JAR=${1:?usage: gen.sh <vineflower-1.12.0.jar>}
OUT=$(mktemp -d)
javac -cp "$JAR" -d "$OUT" VarDefOracle.java
java -cp "$JAR:$OUT" VarDefOracle ../k2_graph_oracle/classes/K2Shapes.class > b07a_golden.txt
