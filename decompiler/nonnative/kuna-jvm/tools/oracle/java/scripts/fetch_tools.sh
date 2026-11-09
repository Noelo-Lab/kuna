#!/usr/bin/env bash
# Fetch + verify the JDKs and the jasm assembler the corpus is built with, and create the jdk<N> links
# compile_sets.sh / build_manifest.py / vf-decompile use. Idempotent: existing links and files are kept.
#   $JDKS/jdk8   Temurin 8u512-b09        $JDKS/jdk9  AdoptOpenJDK 9.0.4+11 (foojay "aoj"; Temurin has no 9)
#   $JDKS/jdk11  Temurin 11.0.33+10       $JDKS/jdk16 Temurin 16.0.2+7       $JDKS/jdk25 Temurin 25.0.5+8
#   $JDKS/jdk17, $JDKS/jdk21  links to existing OpenJDK 17 / 21 homes: JDK17_HOME / JDK21_HOME, default the
#                Debian/Ubuntu packages (/usr/lib/jvm/java-{17,21}-openjdk-amd64)
#   $CORPUS/tools/asmtools/asmtools-core-7.0.b10-ea.jar   Maven Central (what Vineflower's buildSrc uses)
set -euo pipefail
NO_ORACLE='set it to the JVM oracle workspace (decompiler/nonnative/kuna-jvm/tools/oracle/README.md)'
JDKS=${JDKS:-${KUNA_JVM_ORACLE:?$NO_ORACLE}/jdks}
CORPUS=${CORPUS:-${KUNA_JVM_ORACLE:?$NO_ORACLE}/corpus}
JDK17_HOME=${JDK17_HOME:-/usr/lib/jvm/java-17-openjdk-amd64}
JDK21_HOME=${JDK21_HOME:-/usr/lib/jvm/java-21-openjdk-amd64}
mkdir -p "$JDKS/dl"

fetch() {  # ver url file sha256
  local ver=$1 url=$2 file=$3 sum=$4
  if [ -e "$JDKS/jdk$ver" ]; then return; fi
  if [ ! -f "$JDKS/dl/$file" ]; then
    curl -sSfL --max-time 900 -o "$JDKS/dl/$file.part" "$url"
    echo "$sum  $JDKS/dl/$file.part" | sha256sum -c --quiet -
    mv -f "$JDKS/dl/$file.part" "$JDKS/dl/$file"
  fi
  echo "$sum  $JDKS/dl/$file" | sha256sum -c --quiet -
  mkdir -p "$JDKS/x$ver"
  tar -xzf "$JDKS/dl/$file" -C "$JDKS/x$ver"
  ln -sfn "$(ls -d "$JDKS/x$ver"/*/ | head -1)" "$JDKS/jdk$ver"
}

fetch 8  https://github.com/adoptium/temurin8-binaries/releases/download/jdk8u512-b09/OpenJDK8U-jdk_x64_linux_hotspot_8u512b09.tar.gz \
  OpenJDK8U-jdk_x64_linux_hotspot_8u512b09.tar.gz fb3bfa869012419b562bb4235d0d83ec4f8c1dda62ae432ba20c6c3a7e43f7e4
fetch 9  https://github.com/AdoptOpenJDK/openjdk9-binaries/releases/download/jdk-9.0.4%2B11/OpenJDK9U-jdk_x64_linux_hotspot_9.0.4_11.tar.gz \
  OpenJDK9U-jdk_x64_linux_hotspot_9.0.4_11.tar.gz aa4fc8bda11741facaf3b8537258a4497c6e0046b9f4931e31f713d183b951f1
fetch 11 https://github.com/adoptium/temurin11-binaries/releases/download/jdk-11.0.33%2B10/OpenJDK11U-jdk_x64_linux_hotspot_11.0.33_10.tar.gz \
  OpenJDK11U-jdk_x64_linux_hotspot_11.0.33_10.tar.gz c4b8eaf56c93b95b689ffc08d155df1bcdbbffb666f8f71d21d97aedaea6f0ec
fetch 16 https://github.com/adoptium/temurin16-binaries/releases/download/jdk-16.0.2%2B7/OpenJDK16U-jdk_x64_linux_hotspot_16.0.2_7.tar.gz \
  OpenJDK16U-jdk_x64_linux_hotspot_16.0.2_7.tar.gz 323d6d7474a359a28eff7ddd0df8e65bd61554a8ed12ef42fd9365349e573c2c
fetch 25 https://github.com/adoptium/temurin25-binaries/releases/download/jdk-25.0.5%2B8/OpenJDK25U-jdk_x64_linux_hotspot_25.0.5_8.tar.gz \
  OpenJDK25U-jdk_x64_linux_hotspot_25.0.5_8.tar.gz 68982529869257d0ee6b824ec5aa270ff8b118bb438670da0b97ebdab178d7fb
[ -e "$JDKS/jdk17" ] || ln -sfn "$JDK17_HOME" "$JDKS/jdk17"
[ -e "$JDKS/jdk21" ] || ln -sfn "$JDK21_HOME" "$JDKS/jdk21"
for v in 8 9 11 16 17 21 25; do
  test -x "$JDKS/jdk$v/bin/javac" || { echo "missing javac in $JDKS/jdk$v" >&2; exit 1; }
done

ASM=$CORPUS/tools/asmtools/asmtools-core-7.0.b10-ea.jar
ASM_URL=https://repo1.maven.org/maven2/org/openjdk/asmtools/asmtools-core/7.0.b10-ea/asmtools-core-7.0.b10-ea.jar
ASM_SHA256=5c204b9d64ac9bb57e6d2bec40d2becda9d05213d42ff8792e5e5d3e69cd0388
if [ ! -f "$ASM" ]; then
  mkdir -p "$(dirname "$ASM")"
  curl -sSfL --retry 3 --max-time 600 -o "$ASM.part" "$ASM_URL"
  echo "$ASM_SHA256  $ASM.part" | sha256sum -c --quiet -
  mv -f "$ASM.part" "$ASM"
fi
echo "$ASM_SHA256  $ASM" | sha256sum -c --quiet -
