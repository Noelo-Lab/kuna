#!/usr/bin/env bash
#
# ite-region-converter-missing-5db28e_angr_demo.sh — strip the glibc
# -fstack-protector canary epilogue (angr StackCanarySimplifier port,
# `option stackguard`) and show the angr-vs-kuna gap it closes.
#
# Opportunity: angr's test_ite_region_converter_missing_break_statement::
# authenticate (x86-64 GCC, -fstack-protector).  kuna's default emits a
# `goto label_<addr>` + `label_<addr>:` because the shared canary epilogue
# (`if (canary != *(fs:0x28)) __stack_chk_fail(); return v;`) is a single
# return point every path must funnel into.  `option stackguard on` strips
# that check, so ActionReturnSplit duplicates the bare-return tail into each
# predecessor and the goto disappears — exactly what angr does.
#
# Usage:
#   tools/ite-region-converter-missing-5db28e_angr_demo.sh            # the matrix
#   tools/ite-region-converter-missing-5db28e_angr_demo.sh --show     # full before/after C
#   BIN_ROOT=/path/to/angr/binaries/tests/x86_64 tools/ite-region-converter-missing-5db28e_angr_demo.sh
#
set -uo pipefail

REPO="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BIN_ROOT="${BIN_ROOT:-/home/mahaloz/github/angr-dev/binaries/tests/x86_64}"

SHOW=""
[[ "${1:-}" == "--show" ]] && SHOW="yes"

# Locate a python with the kuna package.
PY="${KUNA_PY:-}"
if [[ -z "$PY" ]]; then
  for c in "$HOME/.virtualenvs/kuna/bin/python" "$REPO/.venv/bin/python" python3 python; do
    if command -v "$c" >/dev/null 2>&1 && "$c" -c 'import kuna' >/dev/null 2>&1; then PY="$c"; break; fi
  done
fi
[[ -n "$PY" ]] || { echo "error: no python with the 'kuna' package (set KUNA_PY=...)" >&2; exit 1; }

# testcase rows:  label | relative-path | target | note
#   ite_region_converter_missing_breaks::authenticate is THE opportunity.
#   The other rows are nearby -fstack-protector functions in the same binary
#   (MD5/MD5Update) that also carry a canary epilogue.
ROWS=(
  "authenticate | ite_region_converter_missing_breaks | authenticate | THE opportunity — goto from the shared canary epilogue"
  "MD5Final     | ite_region_converter_missing_breaks | MD5Final      | neighbour fn — canary stripped, but it had no goto to begin with"
)

decompile() {  # <binpath> <target> <stackguard on|off>
  local bin="$1" tgt="$2" sg="$3"
  "$PY" -m kuna.decompile "$bin" "$tgt" --option stackguard "$sg" 2>&1 \
    | grep -vE "WARNING: Function|overlaps object"
}

printf '%-13s | %-7s | %-7s | %-7s | %-7s | %s\n' "case" "goto/off" "goto/on" "chk/off" "chk/on" "verdict (note)"
printf -- '--------------+---------+---------+---------+---------+--------------------------------------------\n'

for row in "${ROWS[@]}"; do
  IFS='|' read -r label rel tgt note <<<"$row"
  label="${label// /}"; rel="${rel// /}"; tgt="${tgt// /}"; note="${note# }"
  bin="$BIN_ROOT/$rel"
  if [[ ! -f "$bin" ]]; then
    printf '%-13s | %-7s | %-7s | %-7s | %-7s | %s\n' "$label" "-" "-" "-" "-" "MISSING: $bin"
    continue
  fi

  off="$(decompile "$bin" "$tgt" off)"
  on="$(decompile "$bin" "$tgt" on)"

  goto_off=$(grep -cE '\bgoto ' <<<"$off")
  goto_on=$(grep -cE '\bgoto ' <<<"$on")
  chk_off=$(grep -cE 'sub_10e0\(\)|__stack_chk_fail' <<<"$off")
  chk_on=$(grep -cE 'sub_10e0\(\)|__stack_chk_fail' <<<"$on")

  verdict="$note"
  if [[ "$chk_on" -eq 0 && "$chk_off" -ge 1 ]]; then
    verdict="canary STRIPPED; goto $goto_off->$goto_on — $note"
  elif [[ "$chk_off" -eq 0 ]]; then
    verdict="no canary epilogue here (ON==OFF) — $note"
  fi

  printf '%-13s | %-7s | %-7s | %-7s | %-7s | %s\n' "$label" "$goto_off" "$goto_on" "$chk_off" "$chk_on" "$verdict"

  if [[ -n "$SHOW" ]]; then
    echo
    echo "===== $label : DEFAULT (stackguard off) ====="
    echo "$off"
    echo "===== $label : stackguard ON (canary epilogue stripped) ====="
    echo "$on"
    echo
  fi
done

cat <<'NOTE'

Legend:
  goto/off, goto/on  = count of `goto ` with stackguard off (default) vs on
  chk/off,  chk/on   = count of the __stack_chk_fail call (rendered sub_10e0()) off vs on

Key point: with `option stackguard on` the -fstack-protector canary check + its
__stack_chk_fail call are removed, the shared single-return epilogue collapses, and
ActionReturnSplit duplicates the bare return into each path — so the goto from a deep
return (the `Authenticated!` path inside the loop) disappears, matching angr's
StackCanarySimplifier.  Default is OFF (opt-in): stripping deletes real canary-check
instructions, and the ablation shows 3 upstream datatest assertions would change if it
were on by default.
NOTE
