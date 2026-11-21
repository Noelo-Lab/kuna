#!/usr/bin/env bash
# Fetch + verify Vineflower 1.12.0 into the oracle workspace. Idempotent; never deletes.
#   $KUNA_JVM_ORACLE/oracle/vineflower-1.12.0.jar   the release jar (Maven Central org.vineflower:vineflower:1.12.0)
#   $KUNA_JVM_ORACLE/vineflower/                    a clone of github.com/Vineflower/vineflower at tag 1.12.0
# Env overrides: VF_JAR, VF_SRC (an existing jar / checkout is only verified).
set -euo pipefail
NO_ORACLE='set it to the JVM oracle workspace (decompiler/nonnative/kuna-jvm/tools/oracle/README.md)'
VF_JAR=${VF_JAR:-${KUNA_JVM_ORACLE:?$NO_ORACLE}/oracle/vineflower-1.12.0.jar}
VF_SRC=${VF_SRC:-${KUNA_JVM_ORACLE:?$NO_ORACLE}/vineflower}
JAR_URL=https://repo1.maven.org/maven2/org/vineflower/vineflower/1.12.0/vineflower-1.12.0.jar
JAR_SHA256=1dfcfe974395734fa467ce620661c7623d05ba83670de0529b1fbd63ff548b9d
SRC_URL=https://github.com/Vineflower/vineflower.git
SRC_COMMIT=b8273988af850e8cfb234ca08d129058502b032f

if [ ! -f "$VF_JAR" ]; then
  mkdir -p "$(dirname "$VF_JAR")"
  curl -sSfL --retry 3 --max-time 600 -o "$VF_JAR.part" "$JAR_URL"
  echo "$JAR_SHA256  $VF_JAR.part" | sha256sum -c --quiet -
  mv -f "$VF_JAR.part" "$VF_JAR"
fi
echo "$JAR_SHA256  $VF_JAR" | sha256sum -c --quiet - || { echo "unexpected Vineflower jar: $VF_JAR" >&2; exit 1; }

if [ ! -e "$VF_SRC" ]; then
  git clone -q --branch 1.12.0 "$SRC_URL" "$VF_SRC"
fi
head=$(git -C "$VF_SRC" rev-parse HEAD)
[ "$head" = "$SRC_COMMIT" ] || echo "warning: $VF_SRC is at $head, the corpora were built from $SRC_COMMIT" >&2
