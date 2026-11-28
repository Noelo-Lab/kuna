#!/bin/sh
# Rebuilds classes/ from src/ with kotlinc 2.0.0 (KOTLINC: its bin/kotlinc),
# JDK 17 (JDK17: its home) and kotlin-stdlib 2.0.0 + annotations 13.0 from
# Maven Central (STDLIB, ANNOTATIONS: the jar paths). The committed class files
# are what kuna-cli's Kotlin tests read; no Kotlin compiler is needed to run them.
set -eu
cd "$(dirname "$0")"
: "${KOTLINC:?kotlinc 2.0.0: bin/kotlinc}" "${JDK17:?a JDK 17 home}"
: "${STDLIB:?kotlin-stdlib-2.0.0.jar}" "${ANNOTATIONS:?annotations-13.0.jar}"
rm -rf classes
mkdir classes
JAVA_HOME="$JDK17" "$KOTLINC" -no-stdlib -no-reflect -jvm-target 17 -jdk-home "$JDK17" \
  -module-name kuna-cli-fixture -classpath "$STDLIB:$ANNOTATIONS" -d classes \
  src/com/ex/kt/Counter.kt src/com/ex/kt/Counters.kt
