#!/bin/sh
# Regenerates the fixture classes and golden.txt (JDK 21, Vineflower 1.12.0 jar: VINEFLOWER_JAR,
# default $KUNA_JVM_ORACLE/oracle/vineflower-1.12.0.jar).
set -e
cd "$(dirname "$0")"
javac --release 21 -d classes src/p/Outer.java
javac --release 21 -d lib src/q/Other.java
java -cp "${VINEFLOWER_JAR:-${KUNA_JVM_ORACLE:?set it to the JVM oracle workspace (decompiler/nonnative/kuna-jvm/tools/oracle/README.md)}/oracle/vineflower-1.12.0.jar}" GenClassModelOracle.java . > golden.txt
