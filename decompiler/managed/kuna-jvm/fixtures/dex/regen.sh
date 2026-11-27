#!/bin/sh
# Rebuilds the DEX fixtures and expected outputs from src/. Needs a JDK 17+
# (javac, java, jar), kotlinc (KOTLINC, default on PATH) and d8 from Google's
# r8.jar (R8_JAR). TOOLCHAIN.txt records the versions used. The committed
# outputs are what kuna-dex's and kuna-cli's tests read; they need no JDK.
set -eu
cd "$(dirname "$0")"
: "${R8_JAR:?set R8_JAR to r8.jar}"
KOTLINC="${KOTLINC:-kotlinc}"
JAVAC="${JAVA_HOME:+$JAVA_HOME/bin/}javac"
JAVA="${JAVA_HOME:+$JAVA_HOME/bin/}java"
JAR="${JAVA_HOME:+$JAVA_HOME/bin/}jar"
LIB="${JAVA_HOME:-$(dirname "$(dirname "$(readlink -f "$(command -v "$JAVAC")")")")}"
STDLIB="$(dirname "$(readlink -f "$(command -v "$KOTLINC")")")/../lib/kotlin-stdlib.jar"
D8="$JAVA -cp $R8_JAR com.android.tools.r8.D8"
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT
mkdir -p "$tmp/j" "$tmp/k" "$tmp/d" "$tmp/r" "$tmp/kd" "$tmp/s" "$tmp/sd" expected
"$JAVAC" -g --release 8 -d "$tmp/j" src/dx/*.java
"$JAVAC" -g --release 17 -d "$tmp/s" src/sx/*.java
"$KOTLINC" -nowarn src/kx/Kt.kt -d "$tmp/k"
$D8 --debug --min-api 21 --lib "$LIB" --output "$tmp/d" $(find "$tmp/j" -name '*.class')
$D8 --release --min-api 21 --lib "$LIB" --output "$tmp/r" $(find "$tmp/j" -name '*.class')
$D8 --release --min-api 21 --lib "$LIB" --classpath "$STDLIB" --output "$tmp/kd" $(find "$tmp/k" -name '*.class')
$D8 --debug --min-api 26 --lib "$LIB" --output "$tmp/sd" $(find "$tmp/s" -name '*.class')
cp "$tmp/sd/classes.dex" sugar.dex
"$JAVA" -cp "$tmp/s" sx.Sugar > expected/Sugar.txt
cp "$tmp/d/classes.dex" debug.dex
cp "$tmp/r/classes.dex" release.dex
cp "$tmp/kd/classes.dex" kotlin.dex
for m in Arith Arrays Flow Objects Wide; do
  "$JAVA" -cp "$tmp/j" "dx.$m" > "expected/$m.txt"
done
"$JAVA" -cp "$tmp/k:$STDLIB" kx.KtKt > expected/Kt.txt
mkdir -p "$tmp/apk"
cp release.dex "$tmp/apk/classes.dex"
cp kotlin.dex "$tmp/apk/classes2.dex"
printf 'not a real manifest\n' > "$tmp/apk/AndroidManifest.xml"
rm -f app.apk
find "$tmp/apk" -exec touch -d 2000-01-01T00:00:00Z {} +
(cd "$tmp/apk" && "$JAR" --create --file "$OLDPWD/app.apk" --no-manifest --date=2000-01-01T00:00:00Z AndroidManifest.xml classes.dex classes2.dex)
{
  "$JAVAC" -version 2>&1
  $D8 --version 2>&1 | head -1
  "$KOTLINC" -version 2>&1 | grep -i kotlin | head -1
} > TOOLCHAIN.txt
