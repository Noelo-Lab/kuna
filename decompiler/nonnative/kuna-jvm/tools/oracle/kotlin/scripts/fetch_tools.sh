#!/usr/bin/env bash
# Fetch + verify the Kotlin toolchain and the jars the Kotlin corpus is built with. Idempotent; never deletes.
#   $JDKS/kotlin/dl/kotlin-compiler-2.0.0.zip   GitHub release v2.0.0, sha256 = the release's published .sha256
#   $JDKS/kotlin/x200/kotlinc/                  extracted zip (build.txt 2.0.0-release-341)
#   $CORPUS/tools/m2/                           Maven Central jars/poms (Gradle's resolved class path),
#                                               checked against Maven Central .sha1 and the sha256 pinned here
#   $CORPUS/tools/realworld/<group>.<artifact>-<version>.jar
#                                               Maven Central jars for scripts/realworld.py and the engine's
#                                               tests/data/kt1 and kt3 gen.sh, same checks
# Also needs $JDKS/jdk17 and $JDKS/jdk21 (../../java/scripts/fetch_tools.sh links them).
set -euo pipefail
NO_ORACLE='set it to the JVM oracle workspace (decompiler/nonnative/kuna-jvm/tools/oracle/README.md)'
CORPUS=${CORPUS:-${KUNA_JVM_ORACLE:?$NO_ORACLE}/corpus-kt}
JDKS=${JDKS:-${KUNA_JVM_ORACLE:?$NO_ORACLE}/jdks}
K=$JDKS/kotlin
ZIP_URL=https://github.com/JetBrains/kotlin/releases/download/v2.0.0/kotlin-compiler-2.0.0.zip
ZIP_SHA256=ef578730976154fd2c5968d75af8c2703b3de84a78dffe913f670326e149da3b
COMPILER_JAR_SHA256=06cff2f853a9250d27ac58bcea35c227c5876436ad87db0a421a30927284697b
mkdir -p "$K/dl"
if [ ! -f "$K/dl/kotlin-compiler-2.0.0.zip" ]; then
  curl -sSfL --retry 3 --max-time 1800 -o "$K/dl/kotlin-compiler-2.0.0.zip.part" "$ZIP_URL"
  echo "$ZIP_SHA256  $K/dl/kotlin-compiler-2.0.0.zip.part" | sha256sum -c --quiet -
  mv -f "$K/dl/kotlin-compiler-2.0.0.zip.part" "$K/dl/kotlin-compiler-2.0.0.zip"
fi
echo "$ZIP_SHA256  $K/dl/kotlin-compiler-2.0.0.zip" | sha256sum -c --quiet -
if [ ! -e "$K/x200/kotlinc" ]; then
  mkdir -p "$K/x200"
  unzip -q "$K/dl/kotlin-compiler-2.0.0.zip" -d "$K/x200"
fi
[ "$(cat "$K/x200/kotlinc/build.txt")" = "2.0.0-release-341" ] || { echo "unexpected kotlinc build" >&2; exit 1; }
echo "$COMPILER_JAR_SHA256  $K/x200/kotlinc/lib/kotlin-compiler.jar" | sha256sum -c --quiet -

MC=https://repo1.maven.org/maven2
fetch() {  # maven-path dest sha256
  local path=$1 f=$2 sum=$3
  if [ ! -f "$f" ]; then
    mkdir -p "$(dirname "$f")"
    curl -sSfL --retry 3 --max-time 600 -o "$f.part" "$MC/$path"
    curl -sSfL --retry 3 --max-time 120 -o "$f.sha1" "$MC/$path.sha1"
    echo "$(head -c 40 "$f.sha1")  $f.part" | sha1sum -c --quiet -
    mv -f "$f.part" "$f"
  fi
  echo "$sum  $f" | sha256sum -c --quiet -
}

M2=$CORPUS/tools/m2
while read -r path sum; do
  fetch "$path" "$M2/$(basename "$path")" "$sum"
done <<'EOF'
org/jetbrains/kotlin/kotlin-stdlib/2.0.0/kotlin-stdlib-2.0.0.jar 240938c4aab8e73e888703e3e7d3f87383ffe5bd536d6d5e3c100d4cd0379fcf
org/jetbrains/kotlin/kotlin-stdlib-jdk7/2.0.0/kotlin-stdlib-jdk7-2.0.0.jar dd958064f946b01f745365c2efa11e4c5812a22e26b6caa4433f207fb5b4a5c5
org/jetbrains/kotlin/kotlin-stdlib-jdk8/2.0.0/kotlin-stdlib-jdk8-2.0.0.jar 92035a0fa8b97a97bb430d30ba03e5ad51f4cfa839cf1559e55245cba735aa94
org/jetbrains/annotations/13.0/annotations-13.0.jar ace2a10dc8e2d5fd34925ecac03e4988b2c0f851650c94b8cef49ba1bd111478
org/jetbrains/annotations/24.0.0/annotations-24.0.0.jar ff112f54ce874b8ae899cfd68f0315d96c9f406a338b8eca80c76d10e2e5a2f7
org/jetbrains/kotlin/kotlin-bom/2.0.0/kotlin-bom-2.0.0.pom b5fa4cc303593390ceba96c2975e4e13fcb919dce6a831ec1e58955551a6ab92
org/jetbrains/kotlin/kotlin-stdlib/2.0.0/kotlin-stdlib-2.0.0.pom 590e626bc93397a5e75cc788f2cec835b37a1ee2e54839af30dbd8d5ba1c2ef2
org/jetbrains/kotlin/kotlin-stdlib-jdk7/2.0.0/kotlin-stdlib-jdk7-2.0.0.pom 1ea14bf50d37b24cee99b3bbeca0e24a07ff361d906bb20067cf41606e143096
org/jetbrains/kotlin/kotlin-stdlib-jdk8/2.0.0/kotlin-stdlib-jdk8-2.0.0.pom afe5006f9d2bb7a26a1f1b92b007a871be26dd82fadbbb2cca9f3757534c5b83
EOF

RW=$CORPUS/tools/realworld
while read -r name path sum; do
  fetch "$path" "$RW/$name" "$sum"
done <<'EOF'
com.squareup.okio.okio-2.6.0.jar com/squareup/okio/okio/2.6.0/okio-2.6.0.jar 4d84ef686277b58eb05691ac19cd3befa3429a27274982ee65ea0f07044bcc00
com.squareup.okhttp3.okhttp-4.7.2.jar com/squareup/okhttp3/okhttp/4.7.2/okhttp-4.7.2.jar 6b64ca1d2069751242984537d5de2cbe627ae000430f5117c47d8cb3d272facd
org.jetbrains.kotlin.kotlin-stdlib-2.2.20.jar org/jetbrains/kotlin/kotlin-stdlib/2.2.20/kotlin-stdlib-2.2.20.jar 8836ccffd3585fadda9901244b20d42901d2f3cd581058d8434e2ffabcf3a3e7
org.jetbrains.annotations-13.0.jar org/jetbrains/annotations/13.0/annotations-13.0.jar ace2a10dc8e2d5fd34925ecac03e4988b2c0f851650c94b8cef49ba1bd111478
org.jetbrains.kotlinx.kotlinx-coroutines-core-jvm-1.8.0.jar org/jetbrains/kotlinx/kotlinx-coroutines-core-jvm/1.8.0/kotlinx-coroutines-core-jvm-1.8.0.jar 9860906a1937490bf5f3b06d2f0e10ef451e65b95b269f22daf68a3d1f5065c5
com.squareup.moshi.moshi-kotlin-1.9.3.jar com/squareup/moshi/moshi-kotlin/1.9.3/moshi-kotlin-1.9.3.jar 9ec83f9a5ae27ec529a88e646372386c8ff1692cf60f816ab97265ab716bec1e
org.jetbrains.kotlin.kotlin-reflect-1.6.10.jar org/jetbrains/kotlin/kotlin-reflect/1.6.10/kotlin-reflect-1.6.10.jar 3277ac102ae17aad10a55abec75ff5696c8d109790396434b496e75087854203
EOF
for v in 17 21; do
  test -x "$JDKS/jdk$v/bin/java" || { echo "missing $JDKS/jdk$v (run ../../java/scripts/fetch_tools.sh)" >&2; exit 1; }
done
