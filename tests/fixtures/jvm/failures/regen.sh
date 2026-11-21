#!/bin/sh
# Rebuilds classes/ from src/ (javac 17+ on PATH or JAVA_HOME) and, when KOTLINC,
# JDK17, STDLIB and ANNOTATIONS are set (as for ../kotlin/regen.sh), kotlin/classes/
# from kotlin/src/. The committed class files are what kuna-cli's failure tests read.
set -eu
cd "$(dirname "$0")"
JAVAC="${JAVA_HOME:+$JAVA_HOME/bin/}javac"
rm -rf classes
mkdir classes
"$JAVAC" -g --release 17 -d classes src/com/ex/fail/Nested.java
if [ -n "${KOTLINC:-}" ]; then
  : "${JDK17:?a JDK 17 home}" "${STDLIB:?kotlin-stdlib-2.0.0.jar}" "${ANNOTATIONS:?annotations-13.0.jar}"
  rm -rf kotlin/classes
  mkdir kotlin/classes
  JAVA_HOME="$JDK17" "$KOTLINC" -no-stdlib -no-reflect -jvm-target 17 -jdk-home "$JDK17" \
    -module-name kuna-cli-failures -classpath "$STDLIB:$ANNOTATIONS" -d kotlin/classes \
    kotlin/src/com/ex/ktfail/Inl.kt kotlin/src/com/ex/ktfail/Lens.kt kotlin/src/com/ex/ktfail/Plain.kt
fi
