#!/usr/bin/env bash
# Trace Vineflower 1.12.0 and kuna over the oracle corpus (or a class directory), compare every method
# pass by pass, print the summary and the first-divergence histogram. Exit 1 on any divergent or
# one-sided method (sweep_compare.py), 2 when an engine run could not be started.
#
#   sweep.sh [--corpus DIR] [--jobs N] [--out DIR] [--kuna BIN] [--hash MODE] [--only S]...
#            [--classes DIR [--set NAME] [--opt k=v]... [--lib PATH]... [--sample N]] [--strict-text]
#
#   --corpus DIR   oracle corpus with manifest.json (default ~/kwt/jvm/corpus); every runnable case
#   --classes DIR  instead: every top-level class of DIR with its nested classes is one case, run with
#                  the --opt/--lib given and no fixture (e.g. --opt include-runtime=<JDK home>)
#   --out DIR      new or empty directory: vineflower/ and kuna/ traces, compare.txt, compare.json
#                  (default: a fresh directory under $TMPDIR)
#   --kuna BIN     a built jvmtrace example (default: cargo build --release -p kuna-jvm --example jvmtrace)
#   --hash MODE    Vineflower's identity-hash mode (run.sh), default index
# Env: BUILD (Java tracer build dir, as build.sh; rebuilt when tools/vftrace sources are newer), VF_JAVA (default JDK 21: kuna's Character tables are
# JDK 21's, and Vineflower escapes string characters by its JVM's tables), VF_XMX (default 1500m),
# CARGO_TARGET_DIR (honoured by the cargo build).
set -euo pipefail
HERE=$(cd "$(dirname "$0")" && pwd)
CRATE=$(cd "$HERE/../.." && pwd)
WORKSPACE=$(cd "$CRATE/../.." && pwd)
JOBS=16
OUT=
KUNA=
HASH=index
COMMON=()
COMPARE=()
while [ $# -gt 0 ]; do
  case "$1" in
    --corpus|--classes|--set|--opt|--lib|--sample|--only) COMMON+=("$1" "$2"); shift 2 ;;
    --jobs) JOBS=$2; shift 2 ;;
    --out) OUT=$2; shift 2 ;;
    --kuna) KUNA=$2; shift 2 ;;
    --hash) HASH=$2; shift 2 ;;
    --strict-text) COMPARE+=("$1"); shift ;;
    -h|--help) sed -n '2,19p' "$0"; exit 0 ;;
    *) echo "sweep.sh: unknown argument $1" >&2; exit 64 ;;
  esac
done
[ -n "$OUT" ] || OUT=$(mktemp -d "${TMPDIR:-/tmp}/vftrace-sweep.XXXXXX")
mkdir -p "$OUT"
if [ -n "$(ls -A "$OUT")" ]; then
  echo "sweep.sh: $OUT is not empty" >&2
  exit 64
fi
if [ -z "$KUNA" ]; then
  (cd "$WORKSPACE" && CARGO_INCREMENTAL=0 cargo build -q --release -p kuna-jvm --example jvmtrace) >&2
  KUNA=${CARGO_TARGET_DIR:-$WORKSPACE/target}/release/examples/jvmtrace
fi
export BUILD=${BUILD:-$HOME/kwt/jvm/scratch/s2-trace/vftrace-build}
if [ ! -f "$BUILD/built.stamp" ] || [ -n "$(find "$HERE/src" "$HERE/gen_shadow.py" "$HERE/build.sh" -newer "$BUILD/built.stamp" | head -1)" ]; then
  "$HERE/build.sh" >&2
fi
export VF_JAVA=${VF_JAVA:-$HOME/kwt/jvm/jdks/jdk21/bin/java}
XMX=${VF_XMX:-1500m}
start=$(date +%s)
python3 -I "$HERE/corpus_trace.py" --engine vineflower --hash "$HASH" --jobs "$JOBS" --xmx "$XMX" \
  --out "$OUT/vineflower" "${COMMON[@]}" 2>"$OUT/vineflower.log" \
  || { echo "sweep.sh: vineflower run failed, see $OUT/vineflower.log" >&2; exit 2; }
python3 -I "$HERE/corpus_trace.py" --engine kuna --kuna "$KUNA" --jobs "$JOBS" \
  --out "$OUT/kuna" "${COMMON[@]}" 2>"$OUT/kuna.log" \
  || { echo "sweep.sh: kuna run failed, see $OUT/kuna.log" >&2; exit 2; }
traced=$(date +%s)
rc=0
python3 -I "$HERE/sweep_compare.py" "$OUT/vineflower" "$OUT/kuna" --jobs "$JOBS" --json "$OUT/compare.json" \
  "${COMPARE[@]}" >"$OUT/compare.txt" || rc=$?
grep -E '^(DIFF|MISSING|TEXT|ORACLE|NOTEXT)' "$OUT/compare.txt" | head -60 || true
sed -n '/^[0-9]* cases, [0-9]* methods: /,$p' "$OUT/compare.txt"
echo "traced in $((traced - start)) s, compared in $(($(date +%s) - traced)) s; details: $OUT/compare.txt, $OUT/compare.json"
exit $rc
