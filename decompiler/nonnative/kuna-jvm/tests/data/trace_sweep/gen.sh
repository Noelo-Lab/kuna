#!/usr/bin/env bash
# Regenerates digests.tsv, the digests (tests/jvm/regen.py's text_digest) of the Vineflower 1.12.0
# outputs the trace-sweep regression tests compare against (tests/trace_sweep.rs): each case is a
# vendored corpus class run with the corpus Default set's options plus the --opt pairs listed here,
# through the oracle tooling's vf-decompile (VF; it needs KUNA_JVM_ORACLE,
# decompiler/nonnative/kuna-jvm/tools/oracle/README.md). It prints where each text is.
set -euo pipefail
HERE=$(cd "$(dirname "$0")" && pwd)
TESTS_JVM=$(cd "$HERE/../../../../../../tests/jvm" && pwd)
CLASSES=$TESTS_JVM/classes
VF=${VF:-$HERE/../../../tools/oracle/java/bin/vf-decompile}
DEFAULT=(--fixture --opt bytecode-source-mapping=1 --opt __dump_original_lines__=1 --opt dump-exception-on-error=0
  --opt ignore-invalid-bytecode=1 --opt verify-anonymous-classes=1 --opt include-classpath=0 --opt ternary-in-if=1
  --opt force-jsr-inline=1 --opt verify-pre-post-merges=1)
: > "$HERE/digests.tsv"
gen() {
  local name=$1 file=$2
  shift 2
  local tmp
  tmp=$(mktemp -d)
  "$VF" "${DEFAULT[@]}" "$@" -o "$tmp" >/dev/null
  printf '%s\t%s\n' "$name" "$(python3 -I -c 'import sys; sys.path.append(sys.argv[1]); from regen import file_digest; print(file_digest(sys.argv[2]))' \
    "$TESTS_JVM" "$tmp/$file")" >> "$HERE/digests.tsv"
  echo "$name: $tmp/$file"
}
gen TestEnum2-denum0 TestEnum2.java --opt decompile-enums=0 "$CLASSES/java21/ext/TestEnum2.class"
