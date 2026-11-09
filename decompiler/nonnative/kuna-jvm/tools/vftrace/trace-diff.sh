#!/usr/bin/env bash
# Trace the same classes with Vineflower 1.12.0 and with kuna, then report, per method, the first pass
# whose output differs (compare.py) and a histogram of those passes.
#
#   trace-diff.sh <class files...> [--opt k=v]... [--fixture] [--filter S] [--initial-only]
#                 [--hash index|stock|random:SEED] [--out DIR] [--kuna BIN] [-- compare.py flags]
#
#   --fixture       both engines start from the corpus fixture's base option map
#   --initial-only  kuna runs codeToJava only up to DomHelper.parseGraph ("Initial"); compared with
#                   --prefix, so methods agree when all steps kuna has agree
#   --hash MODE     Vineflower's identity-hash mode (run.sh); default index = kuna's arena order
#   --out DIR       where the two traces go (default: a fresh directory under $TMPDIR)
#   --kuna BIN      a built jvmtrace example (default: cargo build -p kuna-jvm --example jvmtrace)
# Env: as run.sh (BUILD, VF_JAR, VF_JAVA, VF_XMX); CARGO_TARGET_DIR is honoured by the cargo build.
set -euo pipefail
HERE=$(cd "$(dirname "$0")" && pwd)
CRATE=$(cd "$HERE/../.." && pwd)
WORKSPACE=$(cd "$CRATE/../.." && pwd)
HASH=index
OUT=
KUNA=
COMMON=()
KUNA_ONLY=()
COMPARE=()
while [ $# -gt 0 ]; do
  case "$1" in
    --opt|--filter|--lib) COMMON+=("$1" "$2"); shift 2 ;;
    --fixture) COMMON+=("$1"); shift ;;
    --initial-only) KUNA_ONLY+=("$1"); COMPARE+=(--prefix); shift ;;
    --hash) HASH=$2; shift 2 ;;
    --out) OUT=$2; shift 2 ;;
    --kuna) KUNA=$2; shift 2 ;;
    --) shift; COMPARE+=("$@"); break ;;
    *) COMMON+=("$1"); shift ;;
  esac
done
[ -n "$OUT" ] || OUT=$(mktemp -d "${TMPDIR:-/tmp}/trace-diff.XXXXXX")
mkdir -p "$OUT"
if [ -z "$KUNA" ]; then
  (cd "$WORKSPACE" && CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 \
    cargo build -q -p kuna-jvm --example jvmtrace) >&2
  KUNA=${CARGO_TARGET_DIR:-$WORKSPACE/target}/debug/examples/jvmtrace
fi
"$HERE/run.sh" --hash "$HASH" --trace "$OUT/vineflower.vft" "${COMMON[@]}" -o "$OUT/vineflower-out" >"$OUT/vineflower.log" 2>&1 \
  || { echo "vineflower run failed, see $OUT/vineflower.log" >&2; exit 2; }
"$KUNA" --trace "$OUT/kuna.vft" "${KUNA_ONLY[@]}" "${COMMON[@]}" >"$OUT/kuna.log" 2>&1 \
  || echo "kuna run exited non-zero, see $OUT/kuna.log (comparing what was traced)" >&2
echo "traces: $OUT/vineflower.vft $OUT/kuna.vft" >&2
exec python3 -I "$HERE/compare.py" "$OUT/vineflower.vft" "$OUT/kuna.vft" --names vineflower kuna "${COMPARE[@]}"
