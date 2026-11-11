# Verified Rust panic macros

Extend the verified Rust 1.93.0/1.93.1 formatting renderer to exact
`core::panicking::panic_fmt` calls through one P9 `rustfmtpanic` option and module.
This addresses angr's `test_select_macro_chooses_panic_for_panic_fmt` using Kuna's
existing template and primitive formatter proofs. Keep the default on only after
the original 675 assertions and the five-percent speed budget pass.

Require a Rust source-language fact, Rust output, little-endian 64-bit data space,
an exact direct-call identity, three runtime arguments, no result and existing
non-returning flow evidence. The hidden caller location must have a constant address and mapped,
nonvolatile storage. The original location fields are not interpreted or reproduced;
they may reside in relocation-backed data in a PIE image. Preserve literal message bytes, trailing newlines,
primitive width and signedness, argument indices, format flags, setup statements
and all surrounding IR. Dynamic caller locations and unsupported formatting remain
runtime calls. Recompilation changes the panic's source location to the recovered
source, as for an ordinary source-level macro.

Share the existing literal byte reader and add a panic style to the argument
renderer. Literal panic recovery is independent of the print options; integer
arguments require `rustfmtargs`, and static specifications require `rustfmtflags`.
Do not infer generic pointee types or change signatures, argument storage,
unwinding, callbacks or the control-flow graph.

This is one bounded sink extension at the existing P9 direct-call hook. It adds
no pass infrastructure, ABI model or phase. The production changes outside the
new module are registration, configuration and small shared-renderer extensions.

Validate entire emitted literal functions and primitive macro expressions against
native captured panic payloads. The expression harness supplies explicit primitive
input bindings: existing descriptor declarations are not all recompilable, and
this work does not claim otherwise. Add native and hermetic refusal cases, an
off/default stage, complete changed-body review, option ablation and timing, and
all required repository checks before committing.

Primary layout references:

- [Rust 1.93.0 panic entry points](https://github.com/rust-lang/rust/blob/1.93.0/library/core/src/panicking.rs)
- [Rust 1.93.1 panic entry points](https://github.com/rust-lang/rust/blob/1.93.1/library/core/src/panicking.rs)
- [Rust 1.93.0 caller location](https://github.com/rust-lang/rust/blob/1.93.0/library/core/src/panic/location.rs)
- [Rust 1.93.1 caller location](https://github.com/rust-lang/rust/blob/1.93.1/library/core/src/panic/location.rs)
- [Rust 1.93.1 formatting representation](https://github.com/rust-lang/rust/blob/1.93.1/library/core/src/fmt/mod.rs)
