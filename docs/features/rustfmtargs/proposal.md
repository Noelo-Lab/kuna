# Recover primitive Rust print arguments

Add default-off rustfmtargs in P9, with a bounded template decoder and a loader fact identifying primitive integer Display formatters from full Rust symbols. Limit the first implementation to stable Rust 1.93.0/1.93.1 little-endian 64-bit images and exact stdout/stderr runtime sinks with two arguments and no return value.

Decode default placeholders and explicit argument indices. Validate every template byte as readonly/nonvolatile and require UTF-8 literal pieces. Recover each argument-array cell only from an unambiguous stack INDIRECT associated with this call. Resolve pure stack pointers with a bounded walk. Emit a typed dereference of the original value pointer, preserving the runtime read and signedness. Keep all descriptor setup statements and other IR unchanged. Preserve C output.

Refuse unknown or ambiguous formatter symbols, non-stack arrays, missing/conflicting cells, unsupported flags, unknown versions and malformed templates. Other traits, indirect width/precision, generic references, format!/write!, specialized sinks and older runtime layouts remain separate features.

Validate deterministic accepted/refused assembly shapes, real Rust print_one/reorder, executable emitted macros against original stdout/stderr, all changed functions in a broad sweep, and all repository gates. Enable by default only after unchanged original assertions and a measured slowdown within five percent. Retain raw timing and classified diffs.
