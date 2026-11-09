#!/usr/bin/env bash
# Compile the corpus driver (VfDriver, JasmCompile) against the released 1.12.0 jar + asmtools into
# $CORPUS/driver/classes.
set -euo pipefail
TOOLS=$(cd "$(dirname "$(readlink -f "$0")")/.." && pwd)
NO_ORACLE='set it to the JVM oracle workspace (decompiler/nonnative/kuna-jvm/tools/oracle/README.md)'
CORPUS=${CORPUS:-${KUNA_JVM_ORACLE:?$NO_ORACLE}/corpus}
JDKS=${JDKS:-${KUNA_JVM_ORACLE:?$NO_ORACLE}/jdks}
VF_JAR=${VF_JAR:-${KUNA_JVM_ORACLE:?$NO_ORACLE}/oracle/vineflower-1.12.0.jar}
mkdir -p "$CORPUS/driver/classes"
"$JDKS/jdk17/bin/javac" --release 17 -encoding UTF-8 -d "$CORPUS/driver/classes" \
  -cp "$VF_JAR:$CORPUS/tools/asmtools/asmtools-core-7.0.b10-ea.jar" \
  "$TOOLS/driver/src/corpus/VfDriver.java" "$TOOLS/driver/src/corpus/JasmCompile.java"
