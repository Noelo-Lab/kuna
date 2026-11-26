#!/bin/sh
# Rebuilds classes/ from src/ (javac 17+ on PATH or JAVA_HOME), then breaks every
# @Mark('\u7a7a') annotation by retagging its CONSTANT_Integer 0x7a7a as a
# CONSTANT_Float. The engine throws ClassCastException where it writes such an
# annotation, so the class files fail to decompile by construction.
set -eu
cd "$(dirname "$0")"
JAVAC="${JAVA_HOME:+$JAVA_HOME/bin/}javac"
rm -rf classes
mkdir classes
"$JAVAC" -g --release 17 -d classes src/com/ex/fail/*.java
python3 -I - classes/com/ex/fail/*.class <<'PY'
import sys
for path in sys.argv[1:]:
    data = open(path, 'rb').read()
    count = data.count(b'\x03\x00\x00\x7a\x7a')
    assert count <= 1, (path, count)
    open(path, 'wb').write(data.replace(b'\x03\x00\x00\x7a\x7a', b'\x04\x00\x00\x7a\x7a'))
PY
