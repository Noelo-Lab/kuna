#!/usr/bin/env bash
# Build the Vineflower pass tracer against the released 1.12.0 jar.
#   $BUILD/classes  tracer + shadow DecompileRecord and VarProcessor + the oracle's corpus driver (VfDriver)
#   $BUILD/shadow   hash-pinnable copies of the identity-hashed classes (gen_shadow.py)
# Env: KUNA_JVM_ORACLE (../oracle/README.md), or the overrides BUILD (default $KUNA_JVM_ORACLE/vftrace-build),
# VF_JAR, VF_SRC (Vineflower 1.12.0 checkout, read-only), JAVAC (default JDK 17 javac).
set -euo pipefail
HERE=$(cd "$(dirname "$0")" && pwd)
NO_ORACLE='set it to the JVM oracle workspace (decompiler/nonnative/kuna-jvm/tools/oracle/README.md)'
BUILD=${BUILD:-${KUNA_JVM_ORACLE:?$NO_ORACLE}/vftrace-build}
VF_JAR=${VF_JAR:-${KUNA_JVM_ORACLE:?$NO_ORACLE}/oracle/vineflower-1.12.0.jar}
VF_SRC=${VF_SRC:-${KUNA_JVM_ORACLE:?$NO_ORACLE}/vineflower}
JAVAC=${JAVAC:-${KUNA_JVM_ORACLE:?$NO_ORACLE}/jdks/jdk17/bin/javac}
mkdir -p "$BUILD/classes" "$BUILD/shadow" "$BUILD/shadow-src" "$BUILD/annotations-src/org/jetbrains/annotations" "$BUILD/annotations"
# The jar ships without org.jetbrains.annotations (class-retention only); stubs satisfy javac.
for a in Nullable NotNull; do
  printf 'package org.jetbrains.annotations;\n@java.lang.annotation.Retention(java.lang.annotation.RetentionPolicy.CLASS)\npublic @interface %s { String value() default ""; }\n' "$a" \
    > "$BUILD/annotations-src/org/jetbrains/annotations/$a.java"
done
"$JAVAC" -nowarn --release 17 -d "$BUILD/annotations" "$BUILD"/annotations-src/org/jetbrains/annotations/*.java
"$JAVAC" -nowarn --release 17 -encoding UTF-8 -d "$BUILD/classes" -cp "$VF_JAR:$BUILD/annotations" \
  "$HERE/../oracle/java/driver/src/corpus/VfDriver.java" \
  "$HERE/src/org/jetbrains/java/decompiler/main/rels/DecompileRecord.java" \
  "$HERE/src/org/jetbrains/java/decompiler/modules/decompiler/vars/VarProcessor.java" \
  "$HERE"/src/kuna/vftrace/*.java
python3 -I "$HERE/gen_shadow.py" "$VF_SRC" "$BUILD/shadow-src"
"$JAVAC" -nowarn --release 17 -encoding UTF-8 -d "$BUILD/shadow" -cp "$BUILD/classes:$VF_JAR:$BUILD/annotations" \
  $(find "$BUILD/shadow-src" -name '*.java' | sort)
touch "$BUILD/built.stamp"
echo "built $BUILD"
