#!/bin/sh
# rust_fmt_args_nocomment_x86_64: rust_fmt_args_x86_64 without `.comment`; the
# compiler is named only by a standard library source path.
set -eu
cd "$(dirname "$0")"
build_dir=$(mktemp -d)
trap 'rm -r "$build_dir"' EXIT
as --64 -mx86-used-note=no rust_fmt_args_x86_64.S -o "$build_dir/fixture.o"
as --64 -mx86-used-note=no rust_fmt_args_nocomment.S -o "$build_dir/path.o"
ld --build-id=none -e decimal "$build_dir/fixture.o" "$build_dir/path.o" -o "$build_dir/fixture"
objcopy -R .comment "$build_dir/fixture" rust_fmt_args_nocomment_x86_64
