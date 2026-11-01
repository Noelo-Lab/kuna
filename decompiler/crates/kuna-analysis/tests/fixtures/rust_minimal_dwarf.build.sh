#!/bin/sh
set -eu
# rustc 1.90.0, GNU ld 2.38; run from this directory.
for mode in minimal full; do
    level=1
    [ "$mode" != full ] || level=2
    object=$(mktemp)
    rustc --edition=2021 --crate-type lib --emit=obj -C relocation-model=static \
        -C debuginfo="$level" -C opt-level=1 -o "$object" rust_minimal_dwarf.rs
    ld -e call_input -o "rust_${mode}_dwarf_x86_64" "$object"
    rm "$object"
done
object=$(mktemp)
rustc --edition=2021 --crate-type lib --emit=obj -C relocation-model=static \
    -C debuginfo=1 -C opt-level=1 -o "$object" rust_minimal_dwarf_names.rs
cc -no-pie -o rust_minimal_dwarf_names_x86_64 rust_minimal_dwarf_names.c "$object"
rm "$object"
