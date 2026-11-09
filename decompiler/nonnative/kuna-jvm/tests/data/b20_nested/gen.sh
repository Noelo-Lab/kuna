#!/bin/sh
# Regenerates golden.txt from the class_model fixture classes (JDK 21, Vineflower 1.12.0 jar).
set -e
cd "$(dirname "$0")"
java -cp "${VINEFLOWER_JAR:-$HOME/kwt/jvm/oracle/vineflower-1.12.0.jar}" GenNestedOracle.java ../class_model > golden.txt
