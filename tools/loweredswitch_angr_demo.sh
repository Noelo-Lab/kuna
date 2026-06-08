#!/usr/bin/env bash
#
# loweredswitch_angr_demo.sh — run every angr LoweredSwitchSimplifier testcase
# through the kuna decompiler and show, per testcase, whether `loweredswitch`
# reconstructs a switch (ON, the default) vs the upstream if/else chain (OFF).
#
# This reproduces the matrix recorded in PROGRESS.md.
#
# Usage:
#   tools/loweredswitch_angr_demo.sh                 # summary table for all cases
#   tools/loweredswitch_angr_demo.sh --show fmt      # also print the recovered switch block
#   BIN_ROOT=/path/to/angr/binaries tools/loweredswitch_angr_demo.sh
#
set -uo pipefail

REPO="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BIN_ROOT="${BIN_ROOT:-/home/mahaloz/github/angr-dev/binaries/tests/x86_64}"

SHOW=""
[[ "${1:-}" == "--show" ]] && SHOW="${2:-}"

# Locate a python with the kuna package.
PY="${KUNA_PY:-}"
if [[ -z "$PY" ]]; then
  for c in "$HOME/.virtualenvs/kuna/bin/python" "$REPO/.venv/bin/python" python3 python; do
    if command -v "$c" >/dev/null 2>&1 && "$c" -c 'import kuna' >/dev/null 2>&1; then PY="$c"; break; fi
  done
fi
[[ -n "$PY" ]] || { echo "error: no python with the 'kuna' package (set KUNA_PY=...)" >&2; exit 1; }

# testcase rows:  label | relative-path | target | mode(name|addr) | note
ROWS=(
  "fmt        | decompiler/fmt            | main      | name | PIE  — full: switch + 9 cases"
  "mv_-O2     | mv_-O2                    | main      | name | PIE  — partial: switch + 9 cases + default"
  "stat       | decompiler/stat.o         | 0x1a70    | addr | .o   — 68-case tree; capped -> if-chain (no crash)"
  "cksum      | decompiler/cksum-digest.o | print_filename | name | .o   — linear equality chain; guard skips"
  "b2sum      | decompiler/b2sum-digest.o | filename_unescape | name | .o   — not a >=3-case cascade (ON==OFF)"
  "cat        | decompiler/cat.o          | main      | name | .o   — section collision -> wrong fn"
)

# run kuna.decompile, echo the C to stdout; never abort the script on failure.
decompile() {  # <binpath> <target> <mode> <loweredswitch on|off>
  local bin="$1" tgt="$2" mode="$3" sw="$4" args=()
  [[ "$mode" == "addr" ]] && args+=(--addr)
  "$PY" -m kuna.decompile "$bin" "$tgt" "${args[@]}" --option loweredswitch "$sw" 2>&1 \
    | grep -vE "WARNING: Function|overlaps object"
}

printf '%-9s | %-5s | %-6s | %-6s | %s\n' "case" "sw ON" "sw OFF" "crash?" "verdict (note)"
printf -- '----------+-------+--------+--------+--------------------------------------------------\n'

for row in "${ROWS[@]}"; do
  IFS='|' read -r label rel tgt mode note <<<"$row"
  label="${label// /}"; rel="${rel// /}"; tgt="${tgt// /}"; mode="${mode// /}"
  note="${note# }"
  bin="$BIN_ROOT/$rel"
  if [[ ! -f "$bin" ]]; then
    printf '%-9s | %-5s | %-6s | %-6s | %s\n' "$label" "-" "-" "-" "MISSING: $bin"
    continue
  fi

  on="$(decompile "$bin" "$tgt" "$mode" on)"
  off="$(decompile "$bin" "$tgt" "$mode" off)"

  sw_on=$(grep -cE 'switch ?\(' <<<"$on")
  sw_off=$(grep -cE 'switch ?\(' <<<"$off")
  case_on=$(grep -cE '^\s*case ' <<<"$on")
  crash="no"
  grep -qiE "Unable to proceed|Case block has become detached|Low-level ERROR" <<<"$on" && crash="YES"

  verdict="$note"
  [[ "$sw_on" -ge 1 ]] && verdict="switch RECOVERED ($case_on case labels) — $note"
  [[ "$crash" == "YES" ]] && verdict="!! REGRESSED/ERROR — $note"

  printf '%-9s | %-5s | %-6s | %-6s | %s\n' "$label" "$sw_on" "$sw_off" "$crash" "$verdict"

  # optional: print the recovered switch block for one case
  if [[ -n "$SHOW" && "$label" == "$SHOW" ]]; then
    echo
    echo "===== $label : recovered switch (loweredswitch ON) ====="
    awk '/switch ?\(/{p=1} p{print} /^\s*}\s*$/&&p{n++; if(n>=1 && /^}/) {}}' <<<"$on" \
      | sed -n '1,60p'
    echo "===== $label : upstream rendering (loweredswitch OFF, first 25 lines of the cascade) ====="
    grep -nE 'if \(|switch ?\(|case ' <<<"$off" | sed -n '1,25p'
    echo
  fi
done

cat <<'NOTE'

Legend:
  sw ON   = count of `switch (` with loweredswitch on (the DIV-4 default)
  sw OFF  = count with `--option loweredswitch off` (upstream rendering)
  crash?  = did the function fail to decompile ("Unable to proceed") with the feature on?

Key points this demonstrates:
  * fmt / mv_-O2 (linked PIEs): a real getopt switch is reconstructed by default.
  * stat (.o): a 68-case tree that the structurer can't form is CAPPED, so it
    safely falls back to the if/else chain instead of crashing the function.
  * cksum / b2sum / cat (.o): not reconstructed — linear chain / not-a-cascade /
    relocatable-.o section overlap (a harness limit, not a feature limit).

Tip: `tools/loweredswitch_angr_demo.sh --show fmt` prints fmt's recovered switch.
     `tools/loweredswitch_demo.sh <bin> <func>` shows a full before/after diff.
NOTE
