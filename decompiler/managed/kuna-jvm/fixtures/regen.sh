#!/bin/sh
# Rebuilds classes/ and app.jar from src/ (needs a JDK 11+: javac and jar on PATH,
# or JAVA_HOME). The committed outputs are what kuna-cli's JVM tests read.
set -eu
cd "$(dirname "$0")"
JAVAC="${JAVA_HOME:+$JAVA_HOME/bin/}javac"
JAR="${JAVA_HOME:+$JAVA_HOME/bin/}jar"
rm -rf classes app.jar
mkdir classes
"$JAVAC" -g --release 11 -d classes src/com/ex/Main.java src/com/ex/util/Helper.java
find classes -exec touch -d 2000-01-01T00:00:00Z {} +
(cd classes && "$JAR" --create --file ../app.jar --no-manifest --date=2000-01-01T00:00:00Z com)
