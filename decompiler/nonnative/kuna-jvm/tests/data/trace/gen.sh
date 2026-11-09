#!/usr/bin/env bash
# Regenerates vectors.txt and record.vft: GenTraceVectors builds statement trees with the real
# Vineflower 1.12.0 classes and dumps them with the Java tracer (tools/vftrace: Dumper, and Trace behind
# the shadow DecompileRecord for record.vft). JDK 17+ javac/java.
#   VF_JAR  Vineflower jar (default $KUNA_JVM_ORACLE/oracle/vineflower-1.12.0.jar)
#   BUILD   scratch dir for compiled classes (default: a fresh mktemp dir)
set -euo pipefail
HERE=$(cd "$(dirname "$0")" && pwd)
CRATE=$(cd "$HERE/../../.." && pwd)
VF_JAR=${VF_JAR:-${KUNA_JVM_ORACLE:?set it to the JVM oracle workspace (decompiler/nonnative/kuna-jvm/tools/oracle/README.md)}/oracle/vineflower-1.12.0.jar}
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
  "$CRATE/tools/vftrace/src/kuna/vftrace/Dumper.java" "$CRATE/tools/vftrace/src/kuna/vftrace/Trace.java" \
  "$CRATE/tools/vftrace/src/org/jetbrains/java/decompiler/main/rels/DecompileRecord.java" "$HERE/GenTraceVectors.java"
"$JAVA" -cp "$BUILD/classes:$VF_JAR" org.jetbrains.java.decompiler.modules.decompiler.stats.GenTraceVectors \
  "$CRATE/tests/data/k2_graph_oracle/classes/K2Shapes.class" "$CRATE/tests/data/k1/K1Rec.class" "$HERE/vectors.txt" \
  "$HERE/record.vft"
echo "wrote $HERE/vectors.txt ($(wc -l < "$HERE/vectors.txt") lines) and $HERE/record.vft"
