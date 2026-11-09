#!/bin/sh
# Vineflower 1.12.0's pass traces of Kotlin classes with its own Kotlin plugin, for
# tests/kt6_passes.rs: one trace per top-level class (with its nested classes), cut at
# ReplaceExprs.
#   sh gen.sh CLASS_DIR OUT_DIR [CLASS...]    (default: every top-level class of CLASS_DIR)
# LIBS="a.jar b.jar" adds libraries (--lib); FILTER=S traces only methods whose key contains S. Needs tools/vftrace built (tools/vftrace/build.sh). The vectors in this directory are
#   sh gen.sh classes .  TestForRange ...      (the classes listed in kt6_passes.rs)
set -e
HERE=$(cd "$(dirname "$0")" && pwd)
RUN="$HERE/../../../tools/vftrace/run.sh"
DIR=${1:?usage: gen.sh CLASS_DIR OUT_DIR [CLASS...]}
OUT=${2:?usage: gen.sh CLASS_DIR OUT_DIR [CLASS...]}
shift 2
DIR=$(cd "$DIR" && pwd)
mkdir -p "$OUT"
OUT=$(cd "$OUT" && pwd)
if [ $# -eq 0 ]; then
  set -- $(cd "$DIR" && ls -- *.class | grep -v '\$' | sed 's/\.class$//')
fi
LIBARGS=
[ -n "${FILTER:-}" ] && LIBARGS="--filter $FILTER"
for L in ${LIBS:-}; do LIBARGS="$LIBARGS --lib $L"; done
WORK=$(mktemp -d)
for C in "$@"; do
  set -- "$DIR/$C.class"
  for N in "$DIR/$C"\$*.class; do
    [ -e "$N" ] && set -- "$@" "$N"
  done
  "$RUN" --hash index --trace "$WORK/$C.vft" --fixture --plugins \
    --opt bytecode-source-mapping=1 --opt __dump_original_lines__=1 --opt dump-exception-on-error=0 \
    --opt ignore-invalid-bytecode=1 --opt remove-synthetic=0 --opt verify-anonymous-classes=1 \
    $LIBARGS "$@" -o "$WORK/out-$C" >/dev/null 2>&1
  awk '
    /^=M / { m = $0; printed = 0; skip = (m in cut); next }
    /^@ / { split($0, f, " "); name = f[3]
            if (name ~ /^(ReplaceExprs|ResugarMethods|CollapseStringConcat|ReplaceContinue)/) { skip = 1; cut[m] = 1 }
            if (!skip && !printed) { print m; printed = 1 } }
    !skip && m != "" { print }
  ' "$WORK/$C.vft" > "$OUT/$C.vft"
done
