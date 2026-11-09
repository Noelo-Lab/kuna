#!/usr/bin/env bash
# Compile the Kotlin-corpus driver (KtVfDriver) against the released 1.12.0 jar into $CORPUS/driver/classes,
# and extract the bundled Kotlin plugin jar (META-INF/plugins/vineflower-kotlin-0.1.0.jar) into
# $CORPUS/tools/plugin/ for --kotlin-only.
set -euo pipefail
TOOLS=$(cd "$(dirname "$(readlink -f "$0")")/.." && pwd)
NO_ORACLE='set it to the JVM oracle workspace (decompiler/nonnative/kuna-jvm/tools/oracle/README.md)'
CORPUS=${CORPUS:-${KUNA_JVM_ORACLE:?$NO_ORACLE}/corpus-kt}
JDKS=${JDKS:-${KUNA_JVM_ORACLE:?$NO_ORACLE}/jdks}
VF_JAR=${VF_JAR:-${KUNA_JVM_ORACLE:?$NO_ORACLE}/oracle/vineflower-1.12.0.jar}
PLUGIN_SHA256=56d988c6352685eb796c6b92533d4deb2bc013fb71286c6bd0a8961ccd61b3b0
mkdir -p "$CORPUS/driver/classes" "$CORPUS/tools/plugin"
"$JDKS/jdk17/bin/javac" --release 17 -encoding UTF-8 -d "$CORPUS/driver/classes" -cp "$VF_JAR" \
  "$TOOLS/driver/src/corpuskt/KtVfDriver.java"
P="$CORPUS/tools/plugin/vineflower-kotlin-0.1.0.jar"
if ! echo "$PLUGIN_SHA256  $P" | sha256sum -c --quiet - 2>/dev/null; then
  T=$(mktemp -d "$CORPUS/tools/plugin/.x-XXXXXX")
  unzip -q "$VF_JAR" META-INF/plugins/vineflower-kotlin-0.1.0.jar -d "$T"
  echo "$PLUGIN_SHA256  $T/META-INF/plugins/vineflower-kotlin-0.1.0.jar" | sha256sum -c --quiet -
  mv -f "$T/META-INF/plugins/vineflower-kotlin-0.1.0.jar" "$P"
fi
