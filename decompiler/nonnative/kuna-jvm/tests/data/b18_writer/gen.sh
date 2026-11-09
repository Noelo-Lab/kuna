#!/bin/sh
# Regenerates the fixture classes and golden.txt (JDK 21, Vineflower 1.12.0 jar).
set -e
cd "$(dirname "$0")"
javac --release 21 -parameters -d classes src/w/Dump.java
java -cp "${VINEFLOWER_JAR:-$HOME/kwt/jvm/oracle/vineflower-1.12.0.jar}" GenB18WriterOracle.java . > golden.txt
