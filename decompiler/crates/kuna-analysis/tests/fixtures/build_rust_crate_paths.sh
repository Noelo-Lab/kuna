#!/bin/sh
# rust_crate_paths_x86_64: a Rust 1.95.0 program over one registry crate, with
# `.comment` removed so only its source paths name the compiler and the crate.
set -eu
cd "$(dirname "$0")"
registry=$(echo "${CARGO_HOME:-$HOME/.cargo}"/registry/src/index.crates.io-*)
cargo_home=${registry%/registry/src/*}
build_dir=$(mktemp -d)
trap 'rm -r "$build_dir"' EXIT
remap="--remap-path-prefix $cargo_home=/usr/local/cargo"
rustc +1.95.0 --edition=2018 -O --crate-type rlib --crate-name arrayvec --cap-lints allow \
    $remap "$registry/arrayvec-0.7.6/src/lib.rs" --out-dir "$build_dir"
rustc +1.95.0 --edition=2021 -O -C panic=abort -C strip=debuginfo $remap --extern "arrayvec=$build_dir/libarrayvec.rlib" \
    rust_crate_paths.rs -o "$build_dir/fixture"
objcopy -R .comment "$build_dir/fixture" rust_crate_paths_x86_64
