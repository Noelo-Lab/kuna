# Non-native engines

Everything under `decompiler/crates/` decompiles **native** code: bytes are lifted through
SLEIGH to p-code and recovered by `kuna-decomp`. The engines in this folder decompile
**managed bytecode** instead, and share none of that pipeline. Bytecode such as the JVM's is
typed and stack-based, and it ships with the class structure, exception tables and generic
signatures that native recovery has to infer, so each engine here works over an IR of its own
source language rather than over p-code.

| Crate | Input | Output | Origin |
|---|---|---|---|
| `kuna-jvm` | JVM class files, jars | Java | Rust port of Vineflower 1.12.0 (Apache-2.0) |

What the engines do share with the native side is the front door: the `kuna` CLI, its option
catalog and its output formats. `kuna-cli` picks the engine from the input file: the dispatch
is `decompiler/crates/kuna-cli/src/nonnative.rs`, and the JVM command lines, selection grammar,
output shapes and catalog rows are under `decompiler/crates/kuna-cli/src/nonnative/`.
