#!/bin/sh
# Regenerates render_golden.txt from Vineflower 1.12.0 and its bundled Kotlin plugin
# (the plugin jar is META-INF/plugins/vineflower-kotlin-0.1.0.jar inside the release jar).
#   sh gen-render.sh ~/kwt/jvm/oracle/vineflower-1.12.0.jar ~/kwt/jvm/corpus-kt/tools/plugin/vineflower-kotlin-0.1.0.jar
set -e
cd "$(dirname "$0")"
JAR=${1:?usage: gen-render.sh <vineflower-1.12.0.jar> <vineflower-kotlin-0.1.0.jar>}
PLUGIN=${2:?usage: gen-render.sh <vineflower-1.12.0.jar> <vineflower-kotlin-0.1.0.jar>}
OUT=$(mktemp -d)
javac -nowarn -cp "$JAR:$PLUGIN" -d "$OUT" KT6Oracle.java
java -cp "$JAR:$PLUGIN:$OUT" org.jetbrains.java.decompiler.modules.decompiler.stats.KT6Oracle > render_golden.txt
