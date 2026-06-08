#!/usr/bin/env bash
#
# loweredswitch_demo.sh — decompile a function with the kuna lowered-switch
# recovery OFF (upstream behaviour) and ON, and show the difference.
#
# Usage:
#   tools/loweredswitch_demo.sh [BINARY] [FUNC] [extra kuna.decompile args...]
#
# Defaults to coreutils fmt / main (the angr LoweredSwitchSimplifier testcase).
# FUNC may be a symbol name, or an address with --addr appended as an extra arg:
#   tools/loweredswitch_demo.sh ./a.out 0x401040 --addr
#
# Output: the BEFORE listing, the AFTER listing, and a unified diff of the two.
set -euo pipefail

REPO="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

# Pick a python that has the kuna package (the project venv, else PATH python).
PY="${KUNA_PY:-}"
if [[ -z "$PY" ]]; then
  for cand in "$HOME/.virtualenvs/kuna/bin/python" "$REPO/.venv/bin/python" python3 python; do
    if command -v "$cand" >/dev/null 2>&1 && "$cand" -c 'import kuna' >/dev/null 2>&1; then
      PY="$cand"; break
    fi
  done
fi
[[ -n "$PY" ]] || { echo "error: no python with the 'kuna' package found (set KUNA_PY=/path/to/python)" >&2; exit 1; }

BIN="${1:-/home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/fmt}"
FUNC="${2:-main}"
shift "$(( $# > 2 ? 2 : $# ))" || true   # remaining $@ are passed through (e.g. --addr)

[[ -f "$BIN" ]] || { echo "error: binary not found: $BIN" >&2; exit 1; }

BEFORE="$(mktemp)"; AFTER="$(mktemp)"
trap 'rm -f "$BEFORE" "$AFTER"' EXIT

echo "# binary : $BIN"
echo "# func   : $FUNC"
echo "# python : $PY"
echo

"$PY" -m kuna.decompile "$BIN" "$FUNC" "$@"                              > "$BEFORE"
"$PY" -m kuna.decompile "$BIN" "$FUNC" --option loweredswitch on "$@"    > "$AFTER"

echo "===================== BEFORE (loweredswitch off, upstream) ====================="
cat "$BEFORE"
echo
echo "===================== AFTER  (option loweredswitch on) ========================="
cat "$AFTER"
echo
echo "===================== DIFF (before -> after) ==================================="
# diff returns 1 when files differ; don't let that abort the script.
diff -u --label "before" --label "after" "$BEFORE" "$AFTER" || true

# Quick summary: did a switch appear?
b=$(grep -c 'switch(' "$BEFORE" || true)
a=$(grep -c 'switch(' "$AFTER" || true)
echo
echo "# switch() count:  before=$b  after=$a"
