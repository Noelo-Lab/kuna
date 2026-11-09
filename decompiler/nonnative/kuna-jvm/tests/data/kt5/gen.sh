#!/bin/sh
# Regenerates golden.txt from Vineflower 1.12.0 and its bundled Kotlin plugin:
#   sh gen.sh [vineflower-1.12.0.jar]
set -e
cd "$(dirname "$0")"
JAR=${1:-$HOME/kwt/jvm/oracle/vineflower-1.12.0.jar}
JDK=${JDK:-$HOME/kwt/jvm/jdks/jdk17}
OUT=$(mktemp -d)
"$JDK/bin/javac" --release 8 -d classes src/kt5/KtProps.java
(cd "$OUT" && unzip -o -q "$JAR" META-INF/plugins/vineflower-kotlin-0.1.0.jar)
CP="$JAR:$OUT/META-INF/plugins/vineflower-kotlin-0.1.0.jar"
"$JDK/bin/javac" -nowarn -cp "$CP" -d "$OUT" GenKt5Oracle.java
"$JDK/bin/java" -cp "$CP:$OUT" GenKt5Oracle ../b02/b02/B02Fields.class cases.txt > golden.txt
