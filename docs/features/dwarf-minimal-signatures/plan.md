# Preserve arguments when DWARF omits signature information

Use the pinned angr Rust 1.93 format fixture and small Rust 1.90 minimal/full
DWARF controls. A unit without any type reference or formal/variadic parameter
record has no evidence for a locked function signature. Keep its names and
ranges, and let ordinary dataflow recover arguments. Apply the same rule to
both DWARF prototype producers. This is a strict correction to the existing
DWARF importer, not an optional rendering preference.

Run the full workspace, original and stage corpora, spec/catalog checks, lint,
and tooling suite. Compare every function of both controls and two angr Rust
binaries; record errors, all changed bodies and speed against unmodified main.
