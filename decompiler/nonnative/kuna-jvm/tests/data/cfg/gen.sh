#!/usr/bin/env bash
# Regenerate the CFG oracle vectors. Needs JDK 21 (javac/java on PATH) and the Vineflower 1.12.0 jar.
#   VF_JAR      Vineflower jar (default $KUNA_JVM_ORACLE/oracle/vineflower-1.12.0.jar)
#   VF_TESTDATA Vineflower testData (default $KUNA_JVM_ORACLE/vineflower/testData)
#   VF_CORPUS   Vineflower's test sources compiled per JDK (default $KUNA_JVM_ORACLE/corpus/classes)
#   ASMTOOLS    asmtools jar, for inputs/*.jasm (default $KUNA_JVM_ORACLE/corpus/tools/asmtools/asmtools-core-7.0.b10-ea.jar)
# (KUNA_JVM_ORACLE: decompiler/nonnative/kuna-jvm/tools/oracle/README.md)
#   BUILD       scratch dir for the compiled driver (default: a fresh mktemp dir)
set -euo pipefail
HERE=$(cd "$(dirname "$0")" && pwd)
NO_ORACLE='set it to the JVM oracle workspace (decompiler/nonnative/kuna-jvm/tools/oracle/README.md)'
VF_JAR=${VF_JAR:-${KUNA_JVM_ORACLE:?$NO_ORACLE}/oracle/vineflower-1.12.0.jar}
VF_TESTDATA=${VF_TESTDATA:-${KUNA_JVM_ORACLE:?$NO_ORACLE}/vineflower/testData}
VF_CORPUS=${VF_CORPUS:-${KUNA_JVM_ORACLE:?$NO_ORACLE}/corpus/classes}
ASMTOOLS=${ASMTOOLS:-${KUNA_JVM_ORACLE:?$NO_ORACLE}/corpus/tools/asmtools/asmtools-core-7.0.b10-ea.jar}
BUILD=${BUILD:-$(mktemp -d)}
mkdir -p "$BUILD/driver" "$HERE/classes/r8" "$HERE/classes/r21" "$HERE/classes/jasm" "$HERE/classes/legacy" \
  "$HERE/classes/props8" "$HERE/classes/props21" "$HERE/golden"

javac -nowarn -d "$BUILD/driver" -cp "$VF_JAR" \
  "$HERE/patch/org/jetbrains/java/decompiler/code/cfg/BasicBlock.java" "$HERE/CfgDump.java" \
  "$HERE/IrreducibleDump.java" "$HERE/PropsDump.java" "$HERE/FinallyDump.java"

javac -nowarn --release 8 -g -d "$HERE/classes/r8" "$HERE/inputs/CfgCases.java"
javac -nowarn --release 21 -g -d "$HERE/classes/r21" "$HERE/inputs/CfgCases.java" "$HERE/inputs/CfgCases21.java"
java -cp "$ASMTOOLS" org.openjdk.asmtools.jasm.Main -d "$HERE/classes/jasm" "$HERE/inputs/CfgOddities.jasm"
javac -nowarn --release 8 -g -d "$HERE/classes/props8" "$HERE/inputs/PropCases.java"
javac -nowarn --release 21 -g -d "$HERE/classes/props21" "$HERE/inputs/PropCases.java"
for c in TestJsr TestJsr2 TestJava1Synchronized TestHotjava; do
  cp "$VF_TESTDATA/classes/custom/$c.class" "$HERE/classes/legacy/"
done

CP="$BUILD/driver:$VF_JAR"
java -cp "$CP" CfgDump "$HERE/golden/cases.cfg" "$HERE/classes/r8" "$HERE/classes/r21" "$HERE/classes/jasm" "$HERE/classes/legacy"
java -cp "$CP" PropsDump "$HERE/golden/props.txt" "$HERE/classes/props8" "$HERE/classes/props21" \
  "$HERE/classes/r8" "$HERE/classes/r21" "$HERE/classes/jasm" "$HERE/classes/legacy"
java -cp "$CP" FinallyDump "$HERE/golden/finally.txt" "$HERE/../k2_graph_oracle/classes/K2Shapes.class" \
  "$HERE/classes/r8" "$HERE/classes/r21" "$HERE/classes/jasm" "$HERE/classes/legacy"
java -cp "$CP" IrreducibleDump "$BUILD/irreducible.txt"
gzip -9 -n -c "$BUILD/irreducible.txt" > "$HERE/golden/irreducible.txt.gz"

JDK_CLASSES=(
  java.base/java/util/ArrayList.class
  java.base/java/util/HashMap.class
  java.base/java/util/TreeMap.class
  java.base/java/util/Collections.class
  java.base/java/util/Scanner.class
  java.base/java/util/Formatter.class
  java.base/java/util/concurrent/ConcurrentHashMap.class
  java.base/java/util/concurrent/ThreadPoolExecutor.class
  java.base/java/util/regex/Pattern.class
  java.base/java/lang/Integer.class
  java.base/java/lang/Thread.class
  java.base/java/lang/ClassLoader.class
  java.base/java/io/BufferedReader.class
  java.base/java/util/zip/ZipFile.class
)
ARGS=()
for c in "${JDK_CLASSES[@]}"; do ARGS+=("jrt:$c"); done
java -cp "$CP" CfgDump "$BUILD/jdk.cfg" --skip-trivial --max-insns 300 "${ARGS[@]}"
gzip -9 -n -c "$BUILD/jdk.cfg" > "$HERE/golden/jdk.cfg.gz"

java -cp "$CP" CfgDump "$BUILD/vineflower.cfg" --skip-trivial --max-insns 300 \
  "$VF_CORPUS" "$VF_TESTDATA/obfuscated.jar" "$VF_TESTDATA/kt25937.jar" "$VF_TESTDATA/bulk.jar"
gzip -9 -n -c "$BUILD/vineflower.cfg" > "$HERE/golden/vineflower.cfg.gz"
