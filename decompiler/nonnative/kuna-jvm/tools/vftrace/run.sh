#!/usr/bin/env bash
# Trace Vineflower 1.12.0 pass by pass.
#
#   run.sh [--hash stock|index|random:SEED] --trace OUT [--filter S] [--shift K] VFDRIVER-ARGS...
#
#   --hash stock        released classes, JVM identity hashes (default)
#   --hash index        shadow classes hash by kuna arena creation index (what the port does)
#   --hash random:SEED  shadow classes hash pseudo-randomly from SEED (sensitivity experiments)
# The rest goes to TraceDriver / corpus.VfDriver (--fixture, --opt k=v, inputs, -o OUT), e.g.
#   run.sh --hash index --trace /tmp/t.vft --fixture --opt ternary-in-if=1 Foo.class
# Env: BUILD (as build.sh), VF_JAR, VF_JAVA (default JDK 17), VF_XMX (default 2g), VF_JAVA_OPTS.
set -euo pipefail
HERE=$(cd "$(dirname "$0")" && pwd)
BUILD=${BUILD:-$HOME/kwt/jvm/scratch/s2-trace/vftrace-build}
VF_JAR=${VF_JAR:-$HOME/kwt/jvm/oracle/vineflower-1.12.0.jar}
VF_JAVA=${VF_JAVA:-$HOME/kwt/jvm/jdks/jdk17/bin/java}
HASH=stock
ARGS=()
while [ $# -gt 0 ]; do
  case "$1" in
    --hash) HASH=$2; shift 2 ;;
    *) ARGS+=("$1"); shift ;;
  esac
done
[ -d "$BUILD/classes/kuna" ] || "$HERE/build.sh" >&2
CP="$BUILD/classes:$VF_JAR"
[ "$HASH" = stock ] || CP="$BUILD/shadow:$CP"
# shellcheck disable=SC2086
exec "$VF_JAVA" "-Xmx${VF_XMX:-2g}" -Dfile.encoding=UTF-8 "-Dvftrace.hash=$HASH" ${VF_JAVA_OPTS:-} \
  -cp "$CP" kuna.vftrace.TraceDriver "${ARGS[@]}"
