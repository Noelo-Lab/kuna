#!/bin/sh
# Records the bodies d8 writes for the JDK methods it backports, into
# kuna-dex's src/backports.tsv. Needs a JDK 25 (JAVA_HOME or PATH) and d8 from
# Google's r8.jar (R8_JAR); run from anywhere. Gen.java turns d8's own list of
# backported methods (at any of the levels) into one probe method per JDK
# method; the probe class is
# compiled by d8 at min-api 1 to 35, debug and release, and every synthetic
# method a probe calls instead is hashed by its body (kuna-dex's
# `examples/backports.rs`). probe.dex and index.tsv (min-api 1, release) are
# kept for kuna-dex's tests.
set -eu
cd "$(dirname "$0")"
: "${R8_JAR:?set R8_JAR to r8.jar}"
JAVA="${JAVA_HOME:+$JAVA_HOME/bin/}java"
JAVAC="${JAVA_HOME:+$JAVA_HOME/bin/}javac"
LIB="${JAVA_HOME:-$(dirname "$(dirname "$(readlink -f "$(command -v "$JAVAC")")")")}"
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT
APIS="1 19 21 24 26 28 30 31 33 34 35"
for api in $APIS; do
  "$JAVA" -cp "$R8_JAR" com.android.tools.r8.BackportedMethodList --min-api $api --lib "$LIB"
done | sort -u > "$tmp/list.txt"
"$JAVA" Gen.java "$tmp/list.txt" "$tmp"
"$JAVAC" -nowarn --release 25 -d "$tmp/c" "$tmp/bp/Probe.java" 2>/dev/null
dexes=""
for api in $APIS; do
  for mode in debug release; do
    mkdir -p "$tmp/$api-$mode"
    "$JAVA" -cp "$R8_JAR" com.android.tools.r8.D8 --$mode --min-api $api --lib "$LIB" --output "$tmp/$api-$mode" "$tmp/c/bp/Probe.class"
    dexes="$dexes $tmp/$api-$mode/classes.dex"
  done
done
cp "$tmp/1-release/classes.dex" probe.dex
cp "$tmp/index.tsv" index.tsv
(cd ../../../../kuna-dex && cargo run -q --release --example backports -- "$OLDPWD/index.tsv" $dexes > src/backports.tsv)
"$JAVA" -cp "$R8_JAR" com.android.tools.r8.D8 --version 2>&1 | head -1 > TOOLCHAIN.txt
"$JAVAC" -version >> TOOLCHAIN.txt 2>&1
