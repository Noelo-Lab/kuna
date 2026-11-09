#!/usr/bin/env bash
# Regenerates expected.txt for tests/b11_condense.rs (JDK 21 javac/java on PATH).
#   VF_JAR  Vineflower 1.12.0 jar (default ~/kwt/jvm/oracle/vineflower-1.12.0.jar)
#   BUILD   scratch dir for the compiled oracle (default: a fresh mktemp dir)
set -euo pipefail
HERE=$(cd "$(dirname "$0")" && pwd)
VF_JAR=${VF_JAR:-$HOME/kwt/jvm/oracle/vineflower-1.12.0.jar}
BUILD=${BUILD:-$(mktemp -d)}
javac -nowarn -d "$BUILD" -cp "$VF_JAR" "$HERE/CondenseOracle.java"
java -cp "$VF_JAR:$BUILD" org.jetbrains.java.decompiler.modules.decompiler.decompose.CondenseOracle \
  > "$HERE/expected.txt"
