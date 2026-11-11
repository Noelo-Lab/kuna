#!/usr/bin/env bash
# Regenerates expected.txt for tests/b06_sforms.rs (JDK 21 javac/java on PATH).
#   VF_JAR  Vineflower jar (default $KUNA_JVM_ORACLE/oracle/vineflower-1.12.0.jar)
#   BUILD   scratch dir for the compiled driver (default: a fresh mktemp dir)
set -euo pipefail
HERE=$(cd "$(dirname "$0")" && pwd)
VF_JAR=${VF_JAR:-${KUNA_JVM_ORACLE:?set it to the JVM oracle workspace (decompiler/nonnative/kuna-jvm/tools/oracle/README.md)}/oracle/vineflower-1.12.0.jar}
BUILD=${BUILD:-$(mktemp -d)}
javac -nowarn -cp "$VF_JAR" -d "$BUILD" "$HERE/SFormsOracle.java"
java -cp "$VF_JAR:$BUILD" org.jetbrains.java.decompiler.modules.decompiler.decompose.SFormsOracle > "$HERE/expected.txt"
