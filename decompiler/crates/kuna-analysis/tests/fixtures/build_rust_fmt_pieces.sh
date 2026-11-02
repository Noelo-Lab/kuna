#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"
for version in 1.89.0 1.90.0 nightly-2025-05-22; do
    rustc "+$version" --edition=2021 -C opt-level=2 -C debuginfo=1 -C strip=debuginfo \
        rust_fmt_pieces.rs -o "rust_fmt_pieces_$version"
done
as --64 rust_fmt_pieces_x86_64.S -o /tmp/kuna-rust-fmt-pieces.o
ld -pie --no-dynamic-linker -e literal_shape /tmp/kuna-rust-fmt-pieces.o -o rust_fmt_pieces_x86_64
