#!/bin/sh
set -eu
cd "$(dirname "$0")"
build_dir=$(mktemp -d)
trap 'rm -r "$build_dir"' EXIT
as --64 rust_fmt_args_x86_64.S -o "$build_dir/fixture.o"
ld --build-id=none -e decimal "$build_dir/fixture.o" -o rust_fmt_args_x86_64
