#!/bin/sh
# Compiles the kt0 language-plugin fixture classes (tests/kt0_hooks.rs).
set -e
cd "$(dirname "$0")"
javac --release 8 -g -d classes src/p/*.java
