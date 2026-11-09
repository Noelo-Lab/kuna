#!/bin/sh
# Regenerates golden.txt from the class_model fixture classes (JDK 21, Vineflower 1.12.0 jar: VINEFLOWER_JAR,
# default $KUNA_JVM_ORACLE/oracle/vineflower-1.12.0.jar).
set -e
cd "$(dirname "$0")"
java -cp "${VINEFLOWER_JAR:-${KUNA_JVM_ORACLE:?set it to the JVM oracle workspace (decompiler/nonnative/kuna-jvm/tools/oracle/README.md)}/oracle/vineflower-1.12.0.jar}" GenNestedOracle.java ../class_model > golden.txt
