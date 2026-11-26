# Non-native engines

Everything under `decompiler/crates/` decompiles **native** code: bytes are lifted through
SLEIGH to p-code and recovered by `kuna-decomp`. The engines in this folder decompile
**managed bytecode** instead, and share none of that pipeline. Bytecode such as the JVM's is
typed and stack-based, and it ships with the class structure, exception tables and generic
signatures that native recovery has to infer, so each engine here works over an IR of its own
source language rather than over p-code.

| Crate | Input | Output | Origin |
|---|---|---|---|
| `kuna-jvm` | JVM class files, jars, directories of classes | Java; Kotlin for classes compiled from Kotlin | Rust port of Vineflower 1.12.0 and its bundled Kotlin and IDEA not-null plugins (Apache-2.0) |
| `kuna-managed` | - | - | kuna: the contract between the CLI and a managed engine (no dependencies) |

What the engines do share with the native side is the front door: the `kuna` CLI, its option
catalog and its output formats. `kuna-cli` picks the engine from the input file: the dispatch
and the registry of engines are `decompiler/crates/kuna-cli/src/nonnative.rs`, and the command
lines, output shapes and catalog rows every managed engine shares are under
`decompiler/crates/kuna-cli/src/nonnative/`. The JVM engine's side of the contract (its
selection grammar, the JDK a run reads, its messages) is
`decompiler/nonnative/kuna-jvm/src/infra/kuna_engine.rs`; its documentation is `docs/jvm.md`
(`kuna docs jvm`), its algorithm `docs/spec/10-jvm.md`.

## Adding an engine

An engine implements the two traits of `kuna-managed` in its own crate and is added to
`ENGINES` in `kuna-cli/src/nonnative.rs` (one line, behind its cargo feature).

- `Engine`: its name (the `--engine` value, catalog tag, option prefix and `kuna docs` topic)
  and label; a cheap `sniff`, run on every invocation; `classify`, naming an input's kind or
  refusing it; `open`, which turns an input into a `Program`; its option rows with the text of
  its generated catalog, `resolve` for `--option NAME VALUE` and `check` for its own table
  rules; `language` for `--language`, `usage` for `--help`, and `hidden`, why a selected type
  or member has no text of its own.
- `Program`, an opened input: its `Inventory` (types and members read without decompiling,
  and how a type name prints), `resolve` from the engine's selection grammar to a `Target`,
  `prepare` (what a run reads besides the input, such as a runtime library), and `run`, which
  decompiles a selection and delivers each root type's `Unit` (path, language, text, the
  spans of the types and members written into it with their line mappings and failures)
  through a callback, in root order.

The CLI gives every engine the rest: `decompile`, `decompile-all`, `decompile-project` and
`functions` with their flags and triage, capability errors for the other subcommands and
refusals of the native-only flags, the text and JSON documents, the source-tree export, the
reports of failed and hidden code, `kuna catalog --engine <name>` (rows, Markdown, `--check`),
and the stall watchdog and per-member memory limit, which the engine serves by marking each
member on the `Watch` and calling the `MemoryGuard` around it. The engine owns its inputs, its
names and selection grammar, its options, its threads, the decompilation and its text.

## What a second engine can share

The contract above is everything a new engine needs from the CLI. Inside `kuna-jvm`, the
layers are bound to the JVM to different degrees:

| Layer | Folders | Lines | Bound to |
|---|---|--:|---|
| Container and class model | `p0_knowledge`, `p1_classfile` | 14.0K | class files, jars, the JDK |
| Bytecode front end | `p2_lift` | 5.8K | JVM opcodes and exception tables |
| Middle: statements, expressions, SSA, variables, structuring | `substrate`, `p3_dataflow`, `p6_variables`, `p7_regions`, `p8_structure` | 55.9K | its role is language-neutral; its code is not yet (below) |
| Java semantics | `p4_calls`, `p5_types` | 4.3K | lambdas, generics |
| Writers | `p9_emit` (Java), `kotlin` | 5.4K, 17.1K | one per output language |
| Fixes | `fixes` | 29.1K | Vineflower's Java and Kotlin output |

A second **output** language is already a plug-in on the middle: Kotlin is one. A second
**input** bytecode is not. The middle names the JVM directly in about 1,150 places: 740 uses
of `VarType` (types as JVM descriptors) in 43 of its 128 files, 226 of the class model
(`StructClass`, `StructMethod`, the constant pool) in 36, and 183 of opcodes and access
flags in 30. Routing those three through a neutral type and member model is what sharing
the middle with, say, a CIL front end costs. It is mechanical and it is checked by the
compat corpus, and it should be done with that front end in hand rather than before it,
so the model is shaped by two users instead of guessed from one.
