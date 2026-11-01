#!/bin/sh
set -eu
cd "$(dirname "$0")"
build_dir=$(mktemp -d)
trap 'rm -r "$build_dir"' EXIT
as --64 rust_fmt_literal_scopes_x86_64.S -o "$build_dir/fixture.o"
ld -pie --no-dynamic-linker --no-relax --build-id=none -e scoped_stdout "$build_dir/fixture.o" -o rust_fmt_literal_scopes_x86_64
