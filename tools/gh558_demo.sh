#!/usr/bin/env bash
# Show the ACTUAL decompiled C before/after the GH-558 stage-model assertions
# (docs/prototypes/gh558.md). Two demos:
#   1. compareform  -- hand-assembled `int gh558(int x){ if (x<=8) return 1; return 0; }`
#   2. arraynotation -- the `foo` function from the issue's own attached binary
#                       (needs the PHADE issue dataset; skipped if absent)
#
# Usage: tools/gh558_demo.sh
#   PHADE_DATASET=/path/to/issue_dataset   override the dataset location
set -u
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
DBG="$ROOT/decompiler/cpp/decomp_dbg"
export SLEIGHHOME="$ROOT/specs"
PHADE="${PHADE_DATASET:-$HOME/github/PHADE/issue_dataset}"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

[ -x "$DBG" ] || { echo "error: $DBG not built -- run 'make binaries' first" >&2; exit 1; }

# Run decomp_dbg with a command script on stdin; print only the `print C` dumps.
run_c() {
  "$DBG" 2>/dev/null <<EOF | awk '/^\[decomp\]> print C$/{grab=1;next} /^\[decomp\]>/{grab=0} grab'
$1
EOF
}

# ---------- Demo 1: compareform (x <= 8 vs x < 9) ----------
printf '.text\n.globl gh558\ngh558:\n cmp $8, %%edi\n jg 1f\n mov $1, %%eax\n ret\n1: xor %%eax, %%eax\n ret\n' > "$TMP/gh558.s"
as "$TMP/gh558.s" -o "$TMP/gh558.o" || { echo "error: 'as' (binutils) required" >&2; exit 1; }

COMMON1="load file $TMP/gh558.o
map function 0x0 gh558
parse line extern int4 gh558(int4 x);
load function gh558"

echo "==============================================================="
echo "Demo 1: source was 'if (x <= 8)' (cmp \$8,%edi; jg ...)"
echo "==============================================================="
echo "--- BEFORE (default pipeline: canonical form) ---"
run_c "$COMMON1
decompile
print C
quit"
echo "--- AFTER (option compareform original) ---"
run_c "$COMMON1
option compareform original
decompile
print C
quit"

# ---------- Demo 2: arraynotation (p->r + i vs &p->r[i]) ----------
FOOZIP="$PHADE/ghidra/binaries/558/foo.zip"
if [ ! -f "$FOOZIP" ]; then
  echo "(skipping demo 2: $FOOZIP not found -- set PHADE_DATASET)" >&2
  exit 0
fi
unzip -oq "$FOOZIP" -d "$TMP"

COMMON2="load file $TMP/foo
read symbols
parse line struct S { uint4 n; uint4 **r; };
parse line extern void init(uint4 *x);
parse line extern void foo(S *p);
load function foo"

echo
echo "==============================================================="
echo "Demo 2: source was 'init(p->r[i])' (issue's own binary, gcc -O3)"
echo "==============================================================="
echo "--- BEFORE (default pipeline: pointer arithmetic) ---"
run_c "$COMMON2
decompile
print C
quit"
echo "--- AFTER (option arraynotation on) ---"
run_c "$COMMON2
option arraynotation on
decompile
print C
quit"
