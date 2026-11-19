#!/bin/sh
# Rebuild the WebAssembly fixtures from their sources.
# Needs clang with the wasm32 target, wasm-ld, and rustc with wasm32-unknown-unknown.
set -eu
cd "$(dirname "$0")"
CFLAGS="--target=wasm32 -O2 -nostdlib -Wl,--no-entry"
clang $CFLAGS -Wl,--export-all -o mul_add.wasm mul_add.c
clang $CFLAGS -fvisibility=default -Wl,--export-dynamic -Wl,--allow-undefined -o calls.wasm calls.c
rustc --target wasm32-unknown-unknown --crate-type cdylib -C opt-level=1 -C panic=abort \
  -C debuginfo=0 -C strip=debuginfo -o indirect.wasm indirect.rs
