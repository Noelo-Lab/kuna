#!/bin/sh
# Regenerates the fixture classes and golden.txt with JDK 21 and Vineflower 1.12.0's own
# Kotlin plugin (the jar nested in the release jar). Sfp needs a pre-17 target to keep
# ACC_STRICT; Rec needs 16+.
set -eu
HERE=$(cd "$(dirname "$0")" && pwd)
VF=${VF:-$HOME/kwt/jvm/oracle/vineflower-1.12.0.jar}
WORK=${WORK:-$HOME/kwt/jvm/scratch/kt7/gen}
mkdir -p "$WORK" "$HERE/classes"
(cd "$WORK" && unzip -o -q "$VF" META-INF/plugins/vineflower-kotlin-0.1.0.jar)
CP="$VF:$WORK/META-INF/plugins/vineflower-kotlin-0.1.0.jar"
cd "$HERE"
javac -nowarn --release 11 -d classes $(find src -name '*.java' ! -name Rec.java)
javac -nowarn --release 17 -cp classes -d classes src/k7/Rec.java
javac -nowarn -cp "$CP" -d "$WORK" GenKt7WriterOracle.java
java -cp "$CP:$WORK" GenKt7WriterOracle . > golden.txt
