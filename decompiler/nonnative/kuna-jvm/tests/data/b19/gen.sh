#!/bin/sh
# Regenerates the fixture classes and golden.txt (JDK 21, Vineflower 1.12.0 jar: VINEFLOWER_JAR,
# default $KUNA_JVM_ORACLE/oracle/vineflower-1.12.0.jar).
# inconsistent/ pairs b/X.class (Y an inner class) with b/X$Y.class from a build
# where Y is a static nested class, so the two InnerClasses entries disagree.
set -e
cd "$(dirname "$0")"
javac --release 21 -d classes src/b/*.java
tmp=$(mktemp -d)
javac --release 21 -d "$tmp/v1" src_inconsistent/v1/b/X.java
javac --release 21 -d "$tmp/v2" src_inconsistent/v2/b/X.java
mkdir -p inconsistent/b
cp "$tmp/v2/b/X.class" inconsistent/b/X.class
cp "$tmp/v1/b/X\$Y.class" 'inconsistent/b/X$Y.class'
java -cp "${VINEFLOWER_JAR:-${KUNA_JVM_ORACLE:?set it to the JVM oracle workspace (decompiler/nonnative/kuna-jvm/tools/oracle/README.md)}/oracle/vineflower-1.12.0.jar}" GenB19Oracle.java . > golden.txt
