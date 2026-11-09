#!/usr/bin/env bash
# Regenerate the B08 lift oracle. Needs JDK 21 (javac/java on PATH) and the Vineflower 1.12.0 jar.
#   VF_JAR    Vineflower jar (default ~/kwt/jvm/oracle/vineflower-1.12.0.jar)
#   ASMTOOLS  asmtools jar, for inputs/*.jasm (default ~/kwt/jvm/corpus/tools/asmtools/asmtools-core-7.0.b10-ea.jar)
#   BUILD     scratch dir for the compiled driver (default: a fresh mktemp dir)
set -euo pipefail
HERE=$(cd "$(dirname "$0")" && pwd)
VF_JAR=${VF_JAR:-$HOME/kwt/jvm/oracle/vineflower-1.12.0.jar}
ASMTOOLS=${ASMTOOLS:-$HOME/kwt/jvm/corpus/tools/asmtools/asmtools-core-7.0.b10-ea.jar}
BUILD=${BUILD:-$(mktemp -d)}
mkdir -p "$BUILD/driver" "$HERE/classes"

javac -nowarn -d "$BUILD/driver" -cp "$VF_JAR" \
  "$HERE/../cfg/patch/org/jetbrains/java/decompiler/code/cfg/BasicBlock.java" "$HERE/GenB08Oracle.java"
javac -nowarn --release 8 -g -d "$HERE/classes" "$HERE/inputs/B08Lift.java"
java -cp "$ASMTOOLS" org.openjdk.asmtools.jasm.Main -d "$HERE/classes" "$HERE/inputs/B08Stack.jasm"

java -cp "$BUILD/driver:$VF_JAR" GenB08Oracle "$HERE/lift_golden.txt" \
  "$HERE/classes/B08Lift.class" "$HERE/classes/B08Stack.class"
