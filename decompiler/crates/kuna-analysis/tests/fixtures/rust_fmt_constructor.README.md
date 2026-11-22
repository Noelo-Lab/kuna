# Rust formatting constructor transport

These four non-executable ELF files were independently authored with Rust 1.93.1.
`build_rust_fmt_constructor.py OUTPUT --rustc /path/to/rustc` reproduces the source,
assembly caller, optimized ordinary calls, wrong-body and unsupported-version
controls. The builder compiles but never executes an image. Copy its `route`,
`ordinary`, `wrong-constructor` and `unsupported-version` `.elfdata` files to the
corresponding `rust_fmt_constructor_*_x86_64.elfdata` names, preserving mode 0644.

The isolated assembly caller reserves forty bytes and retains one `r13_tick`
call, one complete `core::fmt::Arguments::new` invocation, and one stderr/stdout
sink. The stage asserts only the assembly entry's u64 argument. The constructor
is the compiler's complete seventeen-byte body; its scratch stores require the
current call-frame proof in P9. The body and version controls are separate copies
with one changed instruction byte and one changed `.comment` version respectively.

Only decompile these files. Native validation compiles fresh emitted source or
an explicitly identified macro/effect projection. The projection cannot establish
that the surrounding nominal `NonNull` and `ArgumentType` expressions compile.
