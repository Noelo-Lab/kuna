#!/bin/sh
set -eu
cd "$(dirname "$0")"
rustc +1.93.1 --edition=2021 -C opt-level=2 -C debuginfo=1 -C strip=debuginfo rust_fmt_result.rs -o rust_fmt_result_real_x86_64
build_dir=$(mktemp -d)
trap 'rm -r "$build_dir"' EXIT
as --64 rust_fmt_result_x86_64.S -o "$build_dir/fixture.o"
ld -pie --no-dynamic-linker --no-relax --build-id=none -e result_decimal "$build_dir/fixture.o" -o rust_fmt_result_x86_64
