#!/usr/bin/env bash
# Regenerates vectors.txt: GenTraceVectors builds statement trees with the real Vineflower 1.12.0
# classes and dumps them with the Java tracer's Dumper (tools/vftrace). JDK 17+ javac/java.
#   VF_JAR  Vineflower jar (default ~/kwt/jvm/oracle/vineflower-1.12.0.jar)
#   BUILD   scratch dir for compiled classes (default: a fresh mktemp dir)
set -euo pipefail
HERE=$(cd "$(dirname "$0")" && pwd)
CRATE=$(cd "$HERE/../../.." && pwd)
VF_JAR=${VF_JAR:-$HOME/kwt/jvm/oracle/vineflower-1.12.0.jar}
BUILD=${BUILD:-$(mktemp -d)}
JAVAC=${JAVAC:-javac}
JAVA=${JAVA:-java}
mkdir -p "$BUILD/annotations-src/org/jetbrains/annotations" "$BUILD/annotations" "$BUILD/classes"
for a in Nullable NotNull; do
  printf 'package org.jetbrains.annotations;\npublic @interface %s { String value() default ""; }\n' "$a" \
    > "$BUILD/annotations-src/org/jetbrains/annotations/$a.java"
done
"$JAVAC" -nowarn --release 17 -d "$BUILD/annotations" "$BUILD"/annotations-src/org/jetbrains/annotations/*.java
"$JAVAC" -nowarn -XDignore.symbol.file --release 17 -encoding UTF-8 -d "$BUILD/classes" -cp "$VF_JAR:$BUILD/annotations" \
  "$CRATE/tools/vftrace/src/kuna/vftrace/Dumper.java" "$HERE/GenTraceVectors.java"
"$JAVA" -cp "$BUILD/classes:$VF_JAR" org.jetbrains.java.decompiler.modules.decompiler.stats.GenTraceVectors \
  "$CRATE/tests/data/k2_graph_oracle/classes/K2Shapes.class" "$CRATE/tests/data/k1/K1Rec.class" "$HERE/vectors.txt"
echo "wrote $HERE/vectors.txt ($(wc -l < "$HERE/vectors.txt") lines)"
