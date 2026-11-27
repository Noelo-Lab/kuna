#!/usr/bin/env bash
# Rebuilds the K2 smoke-test fixture (JDK 21 javac on PATH).
set -euo pipefail
HERE=$(cd "$(dirname "$0")" && pwd)
mkdir -p "$HERE/classes"
javac -nowarn --release 8 -g -d "$HERE/classes" "$HERE/K2Shapes.java"
