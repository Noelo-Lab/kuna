#!/usr/bin/env bash
# Regenerate the Vineflower 1.12.0 differential-testing oracle corpus end to end (../README.md).
#
#   regen.sh [OUT_ROOT]   OUT_ROOT defaults to $CORPUS (files are overwritten in place; nothing is ever
#                         deleted). Pass a fresh directory to rebuild side by side and compare the two
#                         SHA256SUMS files.
#
# Env: KUNA_JVM_ORACLE (required unless every override below is given); overrides CORPUS
# ($KUNA_JVM_ORACLE/corpus: the built driver and asmtools live here), VF_SRC, VF_JAR, JDKS, JOBS (16 parallel
# JVMs, -Xmx1g each), WORK (a new work directory, default $KUNA_JVM_ORACLE/work/java-regen-<timestamp>-<pid>).
set -euo pipefail
TOOLS=$(cd "$(dirname "$(readlink -f "$0")")" && pwd)
NO_ORACLE='set it to the JVM oracle workspace (decompiler/nonnative/kuna-jvm/tools/oracle/README.md)'
export CORPUS=${CORPUS:-${KUNA_JVM_ORACLE:?$NO_ORACLE}/corpus}
export VF_SRC=${VF_SRC:-${KUNA_JVM_ORACLE:?$NO_ORACLE}/vineflower}
export JDKS=${JDKS:-${KUNA_JVM_ORACLE:?$NO_ORACLE}/jdks}
export VF_JAR=${VF_JAR:-${KUNA_JVM_ORACLE:?$NO_ORACLE}/oracle/vineflower-1.12.0.jar}
JOBS=${JOBS:-16}
WORK=${WORK:-${KUNA_JVM_ORACLE:?$NO_ORACLE}/work/java-regen-$(date +%Y%m%d-%H%M%S)-$$}
OUT=$(mkdir -p "${1:-$CORPUS}" && cd "${1:-$CORPUS}" && pwd)
mkdir -p "$WORK"

echo "== Vineflower";    "$TOOLS/../fetch_vineflower.sh"
echo "== JDKs, asmtools"; "$TOOLS/scripts/fetch_tools.sh"
echo "== driver";        "$TOOLS/driver/build.sh"
echo "== compile";       "$TOOLS/scripts/compile_sets.sh" "$OUT/classes" "$OUT/logs"
mkdir -p "$OUT/classes/custom"
cp -a "$VF_SRC/testData/classes/custom/." "$OUT/classes/custom/"
echo "== manifest"
python3 -I "$TOOLS/scripts/build_manifest.py" --vf-src "$VF_SRC" --classes "$OUT/classes" --jdks "$JDKS" \
  --compile-info-dir "$OUT/logs" --jar "$VF_JAR" -o "$OUT/manifest.json"
run() { python3 -I "$TOOLS/scripts/run_oracle.py" --manifest "$OUT/manifest.json" --classes "$OUT/classes" \
          --corpus "$CORPUS" --jar "$VF_JAR" --java "$JDKS/jdk17/bin/java" --jobs "$JOBS" --out-root "$OUT" \
          --work "$WORK" "$@"; }
echo "== oracle: fixture options (expected/)";        run --mode fixture
echo "== oracle: plain CLI defaults (expected-defaults/)"; run --mode defaults
echo "== validate against upstream .dec"
python3 -I "$TOOLS/scripts/validate.py" --manifest "$OUT/manifest.json" --vf-src "$VF_SRC" --out-root "$OUT" \
  --categories "$TOOLS/scripts/mismatch-categories.json"
(cd "$OUT" && find classes expected expected-defaults manifest.json -type f ! -name '*.tmp*' -print0 \
   | LC_ALL=C sort -z | xargs -0 sha256sum) > "$OUT/SHA256SUMS.tmp"
mv -f "$OUT/SHA256SUMS.tmp" "$OUT/SHA256SUMS"
echo "done: $OUT (work dir $WORK)"
