#!/bin/sh
# Regenerates the fixture classes and golden.txt (JDK 21, Vineflower 1.12.0 jar).
set -e
cd "$(dirname "$0")"
javac --release 21 -d classes src/p/Outer.java
javac --release 21 -d lib src/q/Other.java
java -cp "${VINEFLOWER_JAR:-$HOME/kwt/jvm/oracle/vineflower-1.12.0.jar}" GenClassModelOracle.java . > golden.txt
