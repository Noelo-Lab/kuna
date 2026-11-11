#!/usr/bin/env bash
# Regenerates the Vineflower 1.12.0 outputs the trace-sweep regression tests compare against
# (tests/trace_sweep.rs): each case is a vendored corpus class run with the corpus Default set's
# options plus the --opt pairs listed here, through the oracle tooling's vf-decompile (VF; it needs
# KUNA_JVM_ORACLE, decompiler/nonnative/kuna-jvm/tools/oracle/README.md).
set -euo pipefail
HERE=$(cd "$(dirname "$0")" && pwd)
CLASSES=$(cd "$HERE/../../../../../../tests/jvm/classes" && pwd)
VF=${VF:-$HERE/../../../tools/oracle/java/bin/vf-decompile}
DEFAULT=(--fixture --opt bytecode-source-mapping=1 --opt __dump_original_lines__=1 --opt dump-exception-on-error=0
  --opt ignore-invalid-bytecode=1 --opt verify-anonymous-classes=1 --opt include-classpath=0 --opt ternary-in-if=1
  --opt force-jsr-inline=1 --opt verify-pre-post-merges=1)
gen() {
  local out=$1 file=$2
  shift 2
  local tmp
  tmp=$(mktemp -d)
  "$VF" "${DEFAULT[@]}" "$@" -o "$tmp" >/dev/null
  cp "$tmp/$file" "$HERE/$out"
  echo "wrote $HERE/$out"
}
gen TestEnum2-denum0.java TestEnum2.java --opt decompile-enums=0 "$CLASSES/java21/ext/TestEnum2.class"
