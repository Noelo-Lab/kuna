#!/usr/bin/env bash
# Build the Vineflower pass tracer against the released 1.12.0 jar.
#   $BUILD/classes  tracer + shadow DecompileRecord + the corpus driver (VfDriver)
#   $BUILD/shadow   hash-pinnable copies of the identity-hashed classes (gen_shadow.py)
# Env: BUILD (default ~/kwt/jvm/scratch/s2-trace/vftrace-build), VF_JAR, VF_SRC (Vineflower 1.12.0
# checkout, read-only), CORPUS (for driver/src/corpus/VfDriver.java), JAVAC (default JDK 17 javac).
set -euo pipefail
HERE=$(cd "$(dirname "$0")" && pwd)
BUILD=${BUILD:-$HOME/kwt/jvm/scratch/s2-trace/vftrace-build}
VF_JAR=${VF_JAR:-$HOME/kwt/jvm/oracle/vineflower-1.12.0.jar}
VF_SRC=${VF_SRC:-$HOME/github/vineflower}
CORPUS=${CORPUS:-$HOME/kwt/jvm/corpus}
JAVAC=${JAVAC:-$HOME/kwt/jvm/jdks/jdk17/bin/javac}
mkdir -p "$BUILD/classes" "$BUILD/shadow" "$BUILD/shadow-src" "$BUILD/annotations-src/org/jetbrains/annotations" "$BUILD/annotations"
# The jar ships without org.jetbrains.annotations (class-retention only); stubs satisfy javac.
for a in Nullable NotNull; do
  printf 'package org.jetbrains.annotations;\n@java.lang.annotation.Retention(java.lang.annotation.RetentionPolicy.CLASS)\npublic @interface %s { String value() default ""; }\n' "$a" \
    > "$BUILD/annotations-src/org/jetbrains/annotations/$a.java"
done
"$JAVAC" -nowarn --release 17 -d "$BUILD/annotations" "$BUILD"/annotations-src/org/jetbrains/annotations/*.java
"$JAVAC" -nowarn --release 17 -encoding UTF-8 -d "$BUILD/classes" -cp "$VF_JAR:$BUILD/annotations" \
  "$CORPUS/driver/src/corpus/VfDriver.java" \
  "$HERE/src/org/jetbrains/java/decompiler/main/rels/DecompileRecord.java" \
  "$HERE"/src/kuna/vftrace/*.java
python3 -I "$HERE/gen_shadow.py" "$VF_SRC" "$BUILD/shadow-src"
"$JAVAC" -nowarn --release 17 -encoding UTF-8 -d "$BUILD/shadow" -cp "$BUILD/classes:$VF_JAR:$BUILD/annotations" \
  $(find "$BUILD/shadow-src" -name '*.java' | sort)
echo "built $BUILD"
