#!/usr/bin/env bash
# Regenerate the Vineflower 1.12.0 Kotlin differential-testing oracle corpus end to end (../README.md).
#
#   regen.sh [OUT_ROOT]   OUT_ROOT defaults to $CORPUS (files are overwritten in place; nothing is ever
#                         deleted). Pass a fresh directory to rebuild side by side and compare the two
#                         SHA256SUMS files.
#
# Env: KUNA_JVM_ORACLE (required unless every override below is given); overrides CORPUS
# ($KUNA_JVM_ORACLE/corpus-kt: the built driver and the fetched jars live here), VF_SRC, VF_JAR, JDKS (jdk17,
# jdk21, kotlin/x200/kotlinc), JOBS (16 JVMs, -Xmx1g each), WORK (a new work directory, default
# $KUNA_JVM_ORACLE/work/kotlin-regen-<timestamp>-<pid>), SKIP_REALWORLD=1, SPRING_DIR (realworld.py).
set -euo pipefail
TOOLS=$(cd "$(dirname "$(readlink -f "$0")")" && pwd)
NO_ORACLE='set it to the JVM oracle workspace (decompiler/nonnative/kuna-jvm/tools/oracle/README.md)'
export CORPUS=${CORPUS:-${KUNA_JVM_ORACLE:?$NO_ORACLE}/corpus-kt}
export VF_SRC=${VF_SRC:-${KUNA_JVM_ORACLE:?$NO_ORACLE}/vineflower}
export JDKS=${JDKS:-${KUNA_JVM_ORACLE:?$NO_ORACLE}/jdks}
export VF_JAR=${VF_JAR:-${KUNA_JVM_ORACLE:?$NO_ORACLE}/oracle/vineflower-1.12.0.jar}
JOBS=${JOBS:-16}
WORK=${WORK:-${KUNA_JVM_ORACLE:?$NO_ORACLE}/work/kotlin-regen-$(date +%Y%m%d-%H%M%S)-$$}
OUT=$(mkdir -p "${1:-$CORPUS}" && cd "${1:-$CORPUS}" && pwd)
mkdir -p "$WORK"

echo "== Vineflower"; "$TOOLS/../fetch_vineflower.sh"
echo "== tools";      "$TOOLS/scripts/fetch_tools.sh"
echo "== driver";     "$TOOLS/driver/build.sh"
echo "== compile"
python3 -I "$TOOLS/scripts/compile_sets.py" --vf-src "$VF_SRC" --kotlinc-home "$JDKS/kotlin/x200/kotlinc" \
  --jdk "$JDKS/jdk17" --m2 "$CORPUS/tools/m2" --classes "$OUT/classes" --logs "$OUT/logs" --work "$WORK/compile"
echo "== manifest"
python3 -I "$TOOLS/scripts/build_manifest.py" --vf-src "$VF_SRC" --classes "$OUT/classes" --logs "$OUT/logs" \
  --jar "$VF_JAR" --corpus "$CORPUS" -o "$OUT/manifest.json"
run() { python3 -I "$TOOLS/scripts/run_oracle.py" --manifest "$OUT/manifest.json" --classes "$OUT/classes" \
          --corpus "$CORPUS" --jar "$VF_JAR" --java "$JDKS/jdk17/bin/java" --jobs "$JOBS" --out-root "$OUT" \
          --work "$WORK/oracle" "$@"; }
echo "== oracle: fixture options (expected/)";                 run --mode fixture
echo "== oracle: plain CLI defaults (expected-defaults/)";     run --mode defaults
echo "== oracle: CLI defaults + -e=kotlin-stdlib-2.0.0.jar (expected-defaults-stdlib/)"
run --mode defaults --lib tools/m2/kotlin-stdlib-2.0.0.jar --dest-key defaults_stdlib_oracle --suffix=-stdlib
echo "== validate against upstream .dec"
python3 -I "$TOOLS/scripts/validate.py" --manifest "$OUT/manifest.json" --vf-src "$VF_SRC" --out-root "$OUT" \
  --categories "$TOOLS/scripts/mismatch-categories.json"
if [ "${SKIP_REALWORLD:-0}" != 1 ]; then
  echo "== real-world references (realworld/*/vf)"
  python3 -I "$TOOLS/scripts/realworld.py" --out-root "$OUT" --work "$WORK/realworld" --java "$JDKS/jdk17/bin/java" \
    --jar "$VF_JAR" --libs "$CORPUS/tools/realworld" --rerun-1cpu
fi
(cd "$OUT" && find classes expected expected-defaults expected-defaults-stdlib manifest.json realworld/*/vf \
     -type f ! -name '*.tmp*' -print0 2>/dev/null | LC_ALL=C sort -z | xargs -0 sha256sum) > "$OUT/SHA256SUMS.tmp"
mv -f "$OUT/SHA256SUMS.tmp" "$OUT/SHA256SUMS"
echo "done: $OUT (work dir $WORK)"
