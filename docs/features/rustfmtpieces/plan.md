# Verified piece-array Rust printing

Recover default literal and primitive-integer printing from the older
`fmt::Arguments` piece/argument arrays, behind one `rustfmtpieces` option.
The supported producers are x86-64 ELF stable Rust 1.89.0 and 1.90.0 and the
exact 1.89.0 nightly build `bf64d66bd` dated 2025-05-21. The latter is the actual
compiler comment in angr's `nightly-2025-05-22-O3` fmt fixture. Other nightly
builds remain unsupported even when their version numbers match.

At exact `_print`/`_eprint` identities, require the five live 64-bit stack
descriptor words at the call: pieces pointer/count, argument pointer/count,
and a null optional-format pointer. The optional format length is uninitialized
when that pointer is null and is never read. Bound arguments to sixteen and
decoded UTF-8 messages to 4096 bytes. Recover only exact primitive Display
callbacks with the existing stack-pointer/type proofs; retain every setup
statement and all IR. Non-default specifications, generic callbacks, dynamic
descriptors, unknown sinks and incomplete evidence keep the runtime call.

Piece tables and text must be immutable and nonvolatile. Besides the existing
range properties, this renderer may use the image's protected ELF RELRO extent,
restricted to complete 4096-byte pages at its upper end. This metadata does not
paint global properties or enable additional constant propagation. A relocation
slot alone does not prove that its adjacent length word is immutable.

This is one bounded layout reader at the existing print hook. It reuses the
formatter identities, primitive reader and macro emitter, and adds no pass or
ABI inference. Literal and primitive recovery retain the existing
`rustfmtliteral` and `rustfmtargs` controls. Preserve the new default only after
the original corpus and five-percent speed budget pass.

Validate native output from each supported compiler, refusal boundaries, the
original angr image, all changed function bodies, option ablation and C/Go
invariance. Primitive macro execution uses explicit input bindings when retained
descriptor setup has unrelated compilation gaps; do not claim those whole
bodies recompile.

Primary references:

- [Rust 1.89 Arguments](https://github.com/rust-lang/rust/blob/1.89.0/library/core/src/fmt/mod.rs)
- [Rust 1.90 Arguments](https://github.com/rust-lang/rust/blob/1.90.0/library/core/src/fmt/mod.rs)
- [The exact nightly Arguments](https://github.com/rust-lang/rust/blob/bf64d66bd58719fac2585eae5e546e5e1d9947f5/library/core/src/fmt/mod.rs)
- [glibc RELRO protection](https://github.com/bminor/glibc/blob/glibc-2.41/elf/dl-reloc.c)
