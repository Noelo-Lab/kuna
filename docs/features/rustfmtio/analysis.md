# Stdout formatting and its native result

The modern Rust formatting case in angr expects `write_out` to contain
`writeln!(writer, "v: {}", value)`. Kuna still emits an explicit
`Stdout::write_fmt` call. The formatting arguments are recoverable, but the
receiver and return value require separate ABI evidence.

The full method identity is
`<&std::io::stdio::Stdout as std::io::Write>::write_fmt`. Two Rust 1.93.1
ELF images give this method different machine interfaces. In a native fixture,
the first argument points to a reference cell: two loads reach Stdout's inner
lock. The angr fixture promotes the argument and passes that inner lock directly.
Casting both forms to `*mut Stdout`, or both to `*mut &Stdout`, would be wrong.
The complete ELF function ranges are 502 and 509 bytes respectively, including
cleanup blocks omitted by a decompiler's initial main-body range.

Rust's private Stdout layout in the selected producer contains one reference.
The native result is an eight-byte `Result<(), io::Error>` with null success.
An error carries a tagged pointer payload and owns its custom allocation.
Replacing this result with a boolean would lose information and ownership.
The proposed bridge consumes an Error into a raw pointer and then explicitly
exposes that pointer as an integer, preserving the original result bits.

Native prerequisite probes on Rust 1.90.0, 1.93.1 and 1.99.1 checked Stdout and
Error layout, normal Unicode output, an actual broken pipe, six OS-error
payloads and 64 custom error objects per run, each dropped once. These probes
validate candidate bridges with explicit typed inputs; they do not establish
that arbitrary machine functions use either ABI or that complete decompiled
functions recompile. No recovery feature is implemented in this proposal.

The full-body profiles retain all internal branches and non-relative bytes.
External relocations identify `free`, `syscall` and `_Unwind_Resume`; direct
calls and the formatting adapter have recorded symbol identities. The adapter's
drop symbol names a Cursor instantiation despite its use in this Stdout method,
demonstrating compiler code merging. That name cannot establish a concrete type.
Receiver recognition must therefore bind a full method identity to verified
machine-code evidence and reject unknown or conflicting forms.

References:

- [angr formatting test at the pinned revision](https://github.com/angr/angr/blob/6afa0c79a83d9f677a46b0275cdb63bf16c1979e/tests/analyses/decompiler/test_rust_decompiler.py#L432)
- [Rust 1.93.1 Stdout definition and Write implementations](https://github.com/rust-lang/rust/blob/1.93.1/library/std/src/io/stdio.rs)
- [Rust 1.93.1 pointer-based io::Error representation](https://github.com/rust-lang/rust/blob/1.93.1/library/std/src/io/error/repr_bitpacked.rs)
