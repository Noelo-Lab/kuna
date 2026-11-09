# The JVM engine

`kuna-jvm` (`decompiler/nonnative/kuna-jvm`) decompiles JVM class files, jars and
directories of classes to Java, and the classes the Kotlin compiler emitted to Kotlin. It
is a Rust port of [Vineflower](https://github.com/Vineflower/vineflower) 1.12.0 with the
three plugins Vineflower bundles (Kotlin, variable renaming, IDEA not-null). On the same
class files and options it prints the text Vineflower prints, byte for byte, except inside
the methods Vineflower fails on and kuna fixes (*Kuna-only fixes*, below);
`--option jvm_vineflower_compat on` turns those fixes off. It is reached through the
ordinary `kuna` CLI: give a command a `.class`, a jar or a directory where it would take a
binary.

```bash
kuna decompile app.jar com.ex.Main            # one class, as a compilation unit
kuna decompile app.jar com.ex.Main.check      # one method
kuna decompile-project app.jar -o out         # out/com/ex/Main.java, out/com/ex/UserKt.kt, ...
kuna functions app.jar --json                 # the class and method inventory, no decompiling
kuna decompile app.jar com.ex.UserKt --language java   # a Kotlin class as Java
kuna catalog --engine jvm                     # the jvm_* options
```

| Read | For |
|---|---|
| `docs/cli.md`, "JVM inputs" | every command, flag, output shape and error on JVM inputs |
| `docs/spec/10-jvm.md` | what the engine computes, stage by stage |
| `docs/options-jvm.md` | the generated `jvm_*` option catalog (`kuna catalog --markdown --engine jvm`) |
| `tests/jvm/README.md` | the regression corpus: what a case is, how it was generated |
| `tests/jvm/kuna-fixes/README.md` | the cases of kuna's own fixes, with the diff each fix makes |
| `decompiler/nonnative/kuna-jvm/tools/oracle/README.md` | the Vineflower oracle the corpus is cut from: what it downloads, how to rebuild it |
| `decompiler/nonnative/kuna-jvm/tools/vftrace/README.md` | the pass-by-pass trace differ |
| `docs/jvm-benchmark.md` | the benchmark against Vineflower on five large Java projects: what it scores, how to run it, the result |

## Why a separate engine

Everything under `decompiler/crates/` decompiles machine code: SLEIGH lifts it to p-code
and the Ghidra-derived pipeline recovers C from that. JVM bytecode is a different kind of
input, and lowering it to p-code would throw away most of what makes Java recoverable:

- **It is a typed stack machine.** Every instruction states the type it works on, values
  live on an operand stack and in typed local slots, and there is no memory a pointer
  could alias. Native recovery spends most of its effort inferring exactly this.
- **Exceptions are tables, not control flow.** A method's handlers are ranges of
  instructions mapped to handler offsets and exception types; `try`, `catch`, `finally`
  and `synchronized` are recovered from those ranges and the code javac duplicates around
  them.
- **The class file states the program's structure.** Classes, members and their
  modifiers, generic signatures, nested, local and anonymous classes, enums, records,
  sealed hierarchies, lambdas (bootstrap methods) and debug names are attributes to read,
  not facts to infer.

The native pipeline has no model for classes, objects, exception ranges, generics or
lambdas, and it writes C (or Rust), not Java. The vendored SLEIGH `JVM` processor
(`specs/Ghidra/Processors/JVM`) does decode bytecode to p-code, but it is Ghidra's: its
field accesses and invocations are `CALLOTHER` placeholders that Ghidra's Java side fills
in from the constant pool by dynamic p-code injection, and what comes out is pseudocode
over the operand stack, not Java. So `kuna-jvm` works over a Java-level IR of its own and
shares nothing with the native pipeline below the CLI.

What it does share is everything a user or an agent touches: the `kuna` CLI and its
subcommands, the option catalog (`kuna catalog`, `--option NAME VALUE`), the text and JSON
output shapes of `decompile`, `decompile-all`, `decompile-project` and `functions`, the
phase taxonomy its source folders are named after (`docs/phases.md`), and the live spec
(`docs/spec/10-jvm.md`), held to the same `make check-spec` gate as the native chapters.
`kuna-cli` decides which engine an input goes to before any native code runs (spec
chapter 00, section 0.2).

## Layout

| Path (under `decompiler/nonnative/kuna-jvm/`) | What |
|---|---|
| `src/p0_knowledge/` | options, the decompilation context, the class universe, class sources, the JDK reader; kuna's input front door (`kuna_input`), inventory (`kuna_inventory`) and option table (`kuna_options`) |
| `src/p1_classfile/` | the class-file model, class nesting (`classes_processor`), the per-class method loop (`class_wrapper`) |
| `src/p2_lift/` | bytecode decoding, the control-flow graph and its exception-range repairs, the stack-to-expression lift |
| `src/p3_dataflow/` | SSA and SSAU forms, stack-variable inlining, expression simplification |
| `src/p4_calls/` | lambdas, synthetic accessors, dynamic constants, the embedded JDK varargs table |
| `src/p5_types/` | the type lattice, generics, casts |
| `src/p6_variables/` | variable versions, declarations, names |
| `src/p7_regions/` | statement recovery from the CFG, irreducible flow, `finally` |
| `src/p8_structure/` | loops, ifs, switches, try-with-resources, patterns, and the class-level rewrites |
| `src/p9_emit/` | the Java writer, imports, line wrapping, line mapping |
| `src/substrate/` | the shared IR (statements, expressions, edges, types) and its containers |
| `src/rt/` | the Java-runtime kernel (below) |
| `src/infra/` | the per-method schedule (`method_processor`), the plugin and language-plugin API, and kuna's parallel driver, spans, tracer, plugin bundle (`kuna_plugins`) and corpus runner |
| `src/plugins/` | Vineflower's VariableRenaming and IdeaNotNull plugins |
| `src/kotlin/` | Vineflower's Kotlin plugin: the metadata decoder, its pass schedule, the Kotlin expression and statement layer, the Kotlin writer (below, "Kotlin") |
| `options.toml` | the `jvm_*` option rows; `build.rs` generates the static table from it |
| `examples/` | `corpus` (the regression runner), `jvmdec` (a development CLI), `jvmtrace` (trace one run) |
| `tests/` | the unit and differential suite, one `integration` test binary (`tests/integration.rs` registers every file) |
| `tools/oracle/` | the Vineflower oracle: rebuilds the Java and Kotlin corpora the regression corpus comes from |
| `tools/vftrace/` | the trace differ against the real Vineflower |

The folders follow kuna's phase names so a stage can be found by its phase; the order the
stages run in is the engine's own schedule, `src/infra/method_processor.rs` per method and
`src/p0_knowledge/context_unit.rs` per class (spec section 10.1).

### Porting conventions

Every ported file starts with `//! Port of \`org/jetbrains/java/decompiler/.../X.java\`.`
(`org/vineflower/...` for the plugins), and the port follows the original line by line,
so a Vineflower fix can be found and applied in the same place. A maintainer needs these
rules:

- **One Java top-level class, one module**, named after the class in snake case
  (`IfHelper` is `if_helper`, `SSAUConstructorSparseEx` is `ssau_constructor_sparse_ex`;
  acronyms split oddly: `IFernflowerPreferences` is `i_fernflower_preferences`,
  `PPandMMHelper` is `p_pand_mm_helper`). Every module is re-exported flat from the crate
  root (`kuna_jvm::if_helper`), so code never names a folder. Types keep their Java names,
  nested Java types live in their outer class's module, and methods are the Java names in
  snake case with prefixes kept (`getParent` is `get_parent`). A class of static methods
  is a unit struct, so a call reads like the Java (`IfHelper::merge_all_ifs(root)`).
  Overloads carry a suffix and a `/// Java: name(ParamTypes)` line.
- **Three shapes of class.** Anything with identity or shared mutable state is an *arena
  object*: a `Copy` handle whose `==` is Java's reference `==`, with Java subclasses as
  typed views over one `kind` enum (`x.is_if()`, `x.as_if()`, `x.try_as_if()`) and virtual
  methods as dispatchers on the base type. Immutable values that Java compares both by
  identity and by `equals` (`VarType`, `MethodDescriptor`, constants) are `Rc`-backed,
  with explicit `equals` and `ptr_eq` and no `PartialEq`, so every comparison says which
  one it means. Small records and Java enums are plain Rust values.
- **Java semantics, not Rust defaults.** A nullable reference is an `Option`; integers
  keep Java's widths and use wrapping arithmetic wherever Java can overflow; strings that
  may be non-ASCII are indexed in UTF-16 units; numbers and characters are formatted and
  classified through `rt/`; `HashMap`/`HashSet` are always the kernel's JDK-order
  collections (the workspace's Clippy configuration bans the `std` ones); a Java `throw`
  is `jthrow!` and a `catch` is `rt::try_catch`.
- **Counter discipline.** Ids come from counters and reach the output (`var10000`,
  `label7`), so an object is allocated exactly where the Java constructor runs, never as
  an extra temporary.
- **kuna's own code** lives in `kuna_*` modules (`kuna_input`, `kuna_inventory`,
  `kuna_options`, `kuna_fixes`, `kuna_driver`, `kuna_partition`, `kuna_plugins`, `kuna_spans`,
  `kuna_trace`, `kuna_corpus`, `kuna_reclaim`), in the modules that replace JVM facilities (`context_source`,
  `jdk_runtime`, `zip_reader`, `byte_cursor`), and in the Kotlin plugin's seams
  (`kt_metadata`, `kt_attrs`, `k_dispatch`). Changes inside ported files are kept to the
  seams those modules need (span marks in the writer, a deadline poll in the
  cancellation manager, partial runs in `fernflower.rs`) and to the gated sites of kuna's
  fixes (*Kuna-only fixes*), noted in the file headers.
- **Comments are minimal**, as everywhere in kuna: Java comments and Javadoc are not
  transcribed.

### The kernel

Vineflower is written against Java's object model, and a line-by-line port needs that
model underneath it. `src/rt/` provides it:

| Module | Gives the port |
|---|---|
| `arena.rs` | objects with identity: thread-local arenas of class scope (one decompile) and session scope (one engine instance), addressed by `Copy` handles |
| `jhash.rs` | `HashMap`/`HashSet`/`LinkedHashMap`/`LinkedHashSet` with JDK 21's exact iteration order, because Vineflower iterates them where the order reaches the output |
| `jlist.rs` | collections shared and mutated by several holders, as a Java reference is |
| `jstring.rs` | Java strings over Rust `str`, including unpaired surrogates, with UTF-16 views |
| `jchar.rs`, `jnum.rs` | `Character` classification from JDK 21's tables; `Double.toString`/`Float.toString` (the JDK's Schubfach algorithm) and Java's number parsing |
| `exception.rs` | Java exceptions as unwinding panics: `throw`, `try_catch`, `rethrow`; an ordinary Rust panic in the region (an index out of bounds, an `unwrap` on `None`) is caught the same way, as the `IndexOutOfBoundsException` or `NullPointerException` it stands for; `KUNA_JVM_BACKTRACE` prints the Rust backtrace of caught ones |
| `bitset.rs` | `java.util.BitSet` |

Hash collections keyed by objects whose Java hash is the identity hash order those
objects by creation in the arena; see the fidelity contract below.

## Provenance

- **Upstream:** Vineflower 1.12.0, commit `b8273988` ("Release 1.12.0"),
  https://github.com/Vineflower/vineflower, Apache License 2.0. Vineflower continues
  JetBrains' Fernflower through ForgeFlower, FabricMC's fork and Quiltflower. The
  attribution is in the repo's `NOTICE`.
- **Test data:** the regression corpus in `tests/jvm/` is built from Vineflower's own
  `testData` sources and registrations (core's and its plugins') and from the output of
  the released `vineflower-1.12.0.jar` (`tests/jvm/README.md`).
- **What was ported.** Of the 273 Java files under `org/jetbrains/java/decompiler/` in
  Vineflower's `src/`, 242 are ported one to one; each names its original in its first
  line, so `grep -r '^//! Port of' decompiler/nonnative/kuna-jvm/src` is the manifest. The
  other 31 are replaced or deliberately left out:

| Java package | Files | Ported | Replaced or not ported |
|---|---:|---:|---|
| `modules/decompiler/**` (exprents, statements, SSA forms, variables, decomposition, deobfuscation) | 102 | 102 | - |
| `code/**`, `modules/code` | 17 | 17 | - |
| `modules/renamer` | 4 | 4 | - |
| `struct/**` (class model, attributes, constants, generics, lazy loading, matching) | 53 | 50 | the jar, directory and single-file context sources, by `context_source` |
| `main`, `main/rels`, `main/collectors` | 18 | 17 | `Init` (plugin loading) |
| `main/extern` | 9 | 6 | `IContextSource` and `IBytecodeProvider` by `context_source`; `IResultSaver` by kuna's output |
| `main/decompiler` | 9 | 1 | the console decompiler, its help, option parser and savers, by the `kuna` CLI and its output; `PrintStreamLogger` is folded into `i_fernflower_logger` |
| `main/plugins` | 7 | 1 | the plugin jar loaders and sources, by the plugins bundled with the engine (`kuna_plugins`) |
| `api/**` | 20 | 17 | `Decompiler`, `DecompilerOption` (by kuna's option catalog), `ClassAttributeRegistry` |
| `util/**` | 34 | 27 | `JrtFinder`, `ModuleBasedContextSource` and `ClasspathScanner` by `jdk_runtime`; `ZipFileCache` by `zip_reader`; `DataInputFullStream` by `byte_cursor`; `NullableConcurrentHashMap` by a plain map; `DotExporter` (debug graph dumps) dropped |

- **JVM facilities replaced** (a port without a JVM has to supply them):
  - **zip:** `java.util.zip.ZipFile` by `src/substrate/zip_reader.rs`, which follows JDK
    21's reader (end-record search, central-directory order, prefix stubs, stored and
    deflated entries).
  - **The `jrt:` image:** Vineflower reads the standard library through the JVM's `jrt:`
    file system; `src/p0_knowledge/jdk_runtime.rs` finds a JDK and reads its
    `lib/modules` jimage, its `jmods`, or a Java 8 `rt.jar` directly.
  - **Reflection:** `ClasspathHelper` asks the running JVM, by reflection, whether a
    method of a class outside the decompiled universe is varargs. kuna answers from
    `src/p4_calls/classpath_helper_jdk.txt`, every varargs method of OpenJDK 21 such a
    lookup can find, generated by `tests/data/jdk_varargs/gen.sh`.
  - **The thread pool:** Vineflower decompiles the classes of one engine on a thread
    pool. kuna runs each engine instance on one thread and parallelizes with several
    instances over the same input (`src/infra/kuna_driver.rs`), which keeps every
    instance's state single-threaded.
  - **The watchdog:** Vineflower kills a method's thread at `max-time-per-method`; kuna
    polls a cooperative per-method deadline (below).
- **The bundled plugins.** Vineflower's release jar bundles three plugins, which
  `java -jar vineflower.jar` loads; all three are ported, one module per Java file, and
  kuna carries them in the same registration order (`src/infra/kuna_plugins.rs`):

| Plugin (`plugins/` in Vineflower's repository) | Java files | Ported to | What it does |
|---|---:|---|---|
| `variable-renaming` | 5 | `src/plugins/` (5 modules) | renames locals (and parameters) JAD-style or tiny-style on request |
| `kotlin` | 51 | `src/kotlin/` (50 modules; the marker interface `KExprent` is a flag on the expression) | detects Kotlin classes and decompiles and writes them as Kotlin |
| `idea-not-null` | 3 | `src/plugins/` (3 modules) | removes the null checks IntelliJ's compiler adds for `@NotNull` |

  The Kotlin plugin also ships a prebuilt jar (`plugins/kotlin/libs/metadata.jar`) with a
  shaded copy of the Kotlin compiler's metadata schema and the protobuf-lite runtime it
  runs on; kuna ports the parts it reads as `src/kotlin/kt_proto.rs` and
  `src/kotlin/kt_flags.rs`. Plugins are not discovered on a class path: `jvm_plugins`
  (default `on`, as with `java -jar`) loads the bundle, and `off` runs the bare Java
  pipeline of Vineflower's test fixture.
- **Not ported:** a user renamer class and Vineflower's `include-classpath` need a JVM and
  are refused (`options.toml`, `[[driver_owned]]`); third-party plugin jars cannot be
  loaded. Vineflower has no Scala or Groovy support, so classes compiled from those
  languages are written as Java.

## The fidelity contract

**Two modes.** The engine has one switch for strict compatibility, `jvm_vineflower_compat`:

- **Compat mode** (`--option jvm_vineflower_compat on`): every kuna-only fix is off, and on
  the same class files and options kuna-jvm prints exactly what Vineflower 1.12.0 prints,
  failures included, apart from the deliberate differences listed below. Everything this
  section claims about matching Vineflower is measured in this mode, and the regression
  corpus's Vineflower-oracle cases run in it.
- **Default mode** (the switch off): the same text, except that each `jvm_fix_*` option that
  is on (all are, by default) changes what Vineflower does wrong. Today that means two
  things: a method Vineflower fails on and a fix covers is decompiled instead of written
  as its `// $VF: Couldn't be decompiled` comment (with any imports its body needs), and
  the note under a failure comment that remains asks for reports to kuna's tracker instead
  of Vineflower's. Nothing else may differ; that is the evidence each fix ships with
  (*Fixing a Vineflower bug*, below).

**The test oracle is Vineflower's text.** In compat mode, on the same class files and
options, kuna-jvm prints exactly what Vineflower 1.12.0 prints. Every Vineflower-oracle case
of the vendored corpus matches: 607 cases, all listed in `tests/jvm/baseline.txt`, of which
530 are core's Java tests, 6
the variable-renaming and IDEA not-null plugins' tests, and 71 the Kotlin plugin's tests
and core's Kotlin tests. So do the 68 Java cases that need a particular JDK on the machine
and are run from the external oracle corpus (598 Java cases in all), and the Kotlin
plugin's tests under nine option overlays (594 runs). Whole jars (log4j, pdfbox, fontbox,
commons-logging, gettext, the Vineflower jar itself, Vineflower's obfuscated test jars)
and 1,755 classes of JDK 21 come out identical apart from the differences below, and so
do the Kotlin libraries okio 2.6.0, okhttp 4.7.2, kotlinx-coroutines 1.8.0 and
kotlin-stdlib 2.2.20 and spring-framework's Kotlin classes, printed as Kotlin. The parallel
driver's output is identical to a single instance's: on Guava and spring-boot at every
worker count, on Elasticsearch at 8 and 24 workers, on okio, okhttp and spring-framework
at 16 workers. A change that makes a case differ is a regression to fix, unless it is a
named kuna option that is off by default.

**Known, deliberate differences:**

1. **Stack traces in failure comments.** A method that cannot be decompiled is written
   with the exception that stopped it. Vineflower prints the Java stack; kuna has no Java
   stack and prints the exception text and one frame, the place in the Rust source that
   threw it:

   ```java
   public void testComplexPattern(TestSwitchOnEnumJ21.A param1) {
      // $VF: Couldn't be decompiled
      // Please report this to the Vineflower issue tracker, at https://github.com/Vineflower/vineflower/issues with a copy of the class file (if you have the rights to distribute it!)
      // IllegalStateException: Invalid switch case set: [[const(null)], [const(0)], [var1_1 instanceof others], [var1_1 instanceof x], [null]] for selector of type Lpkg/TestSwitchOnEnumJ21$A;
      //   at nonnative/kuna-jvm/src/substrate/switch_head_exprent.rs:89
      //
      // Bytecode:
      // 00: aload 1
      ...
   ```

   (That is compat mode; by default the second line names kuna's tracker,
   `jvm_fix_report_to_kuna`.) With `jvm_dump_exception_on_error off` neither decompiler
   writes the exception. Run in compat mode with the command-line defaults of both
   decompilers, 11 of the 598 Java corpus cases and
   2 of the 73 Kotlin ones differ only in this, and so do every difference on the Kotlin
   libraries above but one (item 5): Vineflower fails the same method with the same
   exception and message, and only the trace lines differ. On spring-framework's Kotlin
   classes, half the classes differ this way, because Vineflower itself fails 22 of them
   whole.
2. **Directory order.** Vineflower visits a directory input in `File.listFiles()` order,
   which Java leaves unspecified; kuna visits each directory's entries sorted by name, so
   its output does not depend on the file system. The order is visible only where it
   reaches the text, such as the numbering of `jvm_rename_members` or the order of
   top-level classes under `jvm_decompile_inner off`.
3. **The JDK the engine itself runs on.** Vineflower's answers to a few questions depend
   on the JVM it runs on: which methods outside the class universe are varargs, how a
   character is classified (Unicode versions differ between JDKs), and for a few values
   how a `double` is printed (JDK 19 changed `Double.toString`). kuna's answers are JDK
   21's everywhere, whether or not a JDK runtime is loaded.
4. **Identity hashes.** One piece of Vineflower state depends on JVM identity hashes: the
   order of a statement's predecessor edges after loop elimination. Stock Vineflower is
   not stable from run to run there; kuna orders by creation and is deterministic. No
   output difference has been observed from it.
5. **Vineflower's thread pool.** Vineflower decompiles the classes of one run on several
   threads, and one lookup crosses root classes: written as Java, the switch over a
   `$WhenMappings` array that a Kotlin `when` on an enum compiles to reads the processed
   `$WhenMappings` class of another root, and what it finds depends on whether that root
   was processed yet. So Vineflower's own output can change from run to run there (okhttp
   with `--kt-enable=0` sometimes gains `$VF: Unable to simplify switch on enum`); kuna
   prints what Vineflower prints on one thread (`--thread-count=1`). One other okhttp line differs between a
   Vineflower running on JDK 17 and one on JDK 21 (a `HashMap` sizing difference in the
   JDK), and kuna matches the JDK 21 one, as in item 3.
6. **Resources.** Vineflower's savers copy a jar's non-class entries into the output; kuna
   writes Java and Kotlin sources only.

## Kuna-only fixes

Where Vineflower 1.12.0 throws on a method, the port throws too and writes the method as
`// $VF: Couldn't be decompiled`. kuna fixes such defects in its own engine, one named
option per fix, each on by default and each turned off, all at once, by
`--option jvm_vineflower_compat on`:

| Option | Vineflower 1.12.0 | With the fix |
|---|---|---|
| `jvm_fix_report_to_kuna` | the note under every error comment asks readers to report the class to Vineflower's tracker | it asks for reports to kuna's tracker (a `jvm_error_message` you set still wins) |
| `jvm_fix_lambda_wildcard_return` | fails a method that passes `x::m` on a receiver typed `Foo<?>` to a generic call, when `m` returns `Foo`'s type variable (`NullPointerException: returnType`; spring-boot's `StructuredLoggingJsonProperties.customizers`) | the method decompiles; the lambda keeps the type its call's context gives it |
| `jvm_fix_twr_unary_condition` | fails a method of a Java 11+ class whose `catch (Throwable)` handler starts with `if (!flag)` on a boolean local, taking the condition for a try-with-resources null check (`index out of bounds`; spring-framework's `ConfigurationClassEnhancer.createClass`) | the method decompiles; the `try`/`catch` is written as it is |
| `statement_as_expression` | prints an `assert` where Java requires an expression, which javac rejects: as the body of an expression lambda, `(k, v) -> assert v.equals(k) : k`, and as a `for` loop's update, `for (boolean more = true; more; assert s.valid())` (elasticsearch's `Metadata$Builder`, `Injector`, `DeflateTransportDecompressor.decompress`, ...) | the lambda gets a block body, `(k, v) -> { assert v.equals(k) : k; }`, and the loop stays a `while` loop with the `assert` at the end of its body; the default output differs from Vineflower's only there |

Every fix is a `KunaFix` variant in `src/p0_knowledge/kuna_fixes.rs` and a row in
`options.toml`; each site where kuna departs from the Java asks
`kuna_fixes::enabled(KunaFix::..)` and keeps Vineflower's code verbatim in the other
branch. Spec section 10.16 describes each defect and what kuna does instead, and
`tests/jvm/kuna-fixes/README.md` shows each fix's diff on its case.

### Fixing a Vineflower bug

The recipe, the same for every fix (the first, `jvm_fix_lambda_wildcard_return`, is the
worked example):

1. **Reproduce.** Take a failing method: its comment names the exception and the Rust
   line that threw it (`//   at nonnative/kuna-jvm/src/...`). Check that Vineflower
   fails it the same way: `decompiler/nonnative/kuna-jvm/tools/oracle/java/bin/vf-decompile
   --opt include-runtime=current Foo.class 'Foo$Bar.class' -o NEW_DIR` (with
   `KUNA_JVM_ORACLE` set) prints the Java stack. Then write a small class that fails
   both decompilers *without a JDK* (the corpus runs without one): replace JDK types the
   failure needs by nested stand-ins, and compare `vf-decompile --fixture --opt
   dump-exception-on-error=1 ...` with `cargo run --release -p kuna-jvm --example jvmdec --
   --opt include-runtime= --opt jvm_plugins=0 --opt jvm_vineflower_compat=1 ...` (`jvmdec`
   takes Vineflower's option names and kuna's own rows by their names, and runs with kuna's
   defaults otherwise; the trace differ's kuna side runs in compat mode). For the Rust backtrace of the caught
   throw, set `KUNA_JVM_BACKTRACE` to a piece of its message (`1` prints every caught
   throw, and the engine throws and catches routinely); build with
   `CARGO_PROFILE_RELEASE_DEBUG=line-tables-only` for line numbers.
2. **Diagnose in the Java.** Read the method in Vineflower's sources at the release (tag
   `1.12.0`; `tools/oracle/fetch_vineflower.sh` checks it out) and, in a full clone, what
   upstream did since: `git log 1.12.0..origin/develop/1.13.0 -- <file>`, and `git log
   --all -S'<code>'` for the commit that introduced the defect. Port upstream's fix when there is one. The fix goes
   to the root cause, in the terms of Vineflower's algorithm: never a catch-and-continue,
   never keyed on a class or method name.
3. **Add the fix.** A variant in `KunaFix` (and in `KunaFix::ALL` and `option()`), and
   a row in `options.toml`: `option = "jvm_fix_<slug>"`, `vineflower = ""`, `short = ""`,
   `aliases = []`, `values = "on|off"`, `default = "on"`, `source_decompiler = "kuna"`,
   `change_kind = "kuna-fix"`, `phase`/`subphase` the folder and module of the gated
   site, `tier = "transform"`, the Vineflower method and the upstream state in
   `inspiration`, and the exception text among the `symptoms`. At the site: `if
   kuna_fixes::enabled(KunaFix::X) { ... } else { <the Java, unchanged> }`, and a line in
   the ported file's header naming the fix. A fix longer than a few lines goes in a
   `kuna_fix_<slug>.rs` module next to the ported file, called from the gated branch. Code
   that runs before `Fernflower::new` installs the context reads the set it is building
   (`fixes.contains(..)`), as `jvm_fix_report_to_kuna` does.
4. **Add the case.** Compile the reproducer and run `python3 -I
   tests/jvm/kuna_fix_case.py --fix jvm_fix_<slug> --class DIR/pkg/TestFoo.class --source
   pkg/TestFoo.java --compiler "javac 17.0.20.1" --summary "<what decompiles now>"`. It
   vendors the class and its `TestFoo$*.class` siblings into `tests/jvm/kuna-fixes/`,
   stores what the release jar prints (the compat expectation, which must contain the
   failure comment), checks that kuna in compat mode prints exactly that, stores kuna's
   default output, prints the diff for review, rebuilds `cases.tsv` and the READMEs with
   `regen.py`, and adds both runs to `tests/jvm/baseline.txt`.
5. **Describe it.** A paragraph in `docs/spec/10-jvm.md` section 10.16 (the defect in
   Vineflower's algorithm, what kuna does instead), a row in the table above, and
   `docs/options-jvm.md` regenerated.
6. **Evidence**, quoted in the commit: (a) the root cause, as above; (b) the reproducer's
   default output recompiles with javac and every method's `javap -c -p` matches the
   original class's; (c) no collateral change: `kuna decompile-project IN -o COMPAT
   --option jvm_vineflower_compat on`, `kuna decompile-project IN -o DEFAULT`, then
   `python3 -I tests/jvm/kuna_fix_diff.py COMPAT DEFAULT`, on the five benchmark jars
   (`docs/jvm-benchmark.md`) and on okio, okhttp and kotlin-stdlib: every difference must
   be inside a method that failed in compat mode, a tracker note or an import; (d) `make
   test-jvm` (the 607 Vineflower-oracle cases run in compat mode).
7. **Gates**: `cargo test -p kuna-jvm`, `make test-jvm`, `kuna catalog --check`, `make
   check-spec` and `python3 tools/check_spec.py --strict`, `make lint`.

Fixes made in parallel meet in the same files. Keep both sides of `KunaFix`,
`options.toml` and `tests/jvm/kuna-fixes/cases.json`, then rerun `regen.py` (it rewrites
`cases.tsv` and both READMEs) and regenerate `docs/options-jvm.md`; `baseline.txt` is
sorted, so keep both sides' lines.

## The JDK runtime library

Vineflower decompiles with the JDK's classes as context, and so does kuna by default. The
JDK is only read, never run: kuna needs a JDK home on disk, not a working `java`.

**Discovery.** `--option jvm_include_runtime` takes `on` (the default; also `current` or
`1`), a JDK home path (one that holds no JDK is an error), or `off` (also `0`). With `on`,
the first of these that holds a readable runtime is used: `JAVA_HOME`; the `java` on
`PATH`, followed through symlinks to its home;
then the usual install directories (`/usr/lib/jvm`, `/usr/lib64/jvm`, `/usr/java`,
`/opt/java`, `/Library/Java/JavaVirtualMachines`, `~/Library/Java/JavaVirtualMachines`,
`~/.sdkman/candidates/java`), newest `JAVA_VERSION` first. A home is readable when it has
`lib/modules`, `jmods/java.base.jmod`, or an `rt.jar` in `lib` or `jre/lib`; kuna reads the
jimage, the jmods or the Java 8 jars respectively. The JSON documents name the runtime
used (`runtime: {path, version}`); when none is found, one note on stderr says so and the
run continues without one.

**What changes without one.** Every class is still decompiled, but whatever needs a JDK
class's members or hierarchy degrades. `kuna decompile tests/fixtures/jvm/app.jar
com.ex.Main` with `--option jvm_include_runtime off`, then with a JDK 21:

```diff
       Runnable r = new Runnable() {
+         @Override
          public void run() {
```

Casts and generic arguments that depend on JDK signatures can also differ. Output
therefore depends on which JDK is found: for reproducible output, pin one with
`--option jvm_include_runtime /path/to/jdk` or use `off`. The vendored corpus runs with no
JDK. `--lib PATH` adds further class files or jars as context the same way, without
decompiling them.

## Kotlin

The Kotlin compiler emits ordinary class files, and the Java pipeline decompiles them to
correct but noisy Java: property accessors, `Intrinsics` null checks, `$default` stubs,
`Companion` fields and a `@Metadata(d1 = ...)` annotation full of encoded strings.
Vineflower's Kotlin plugin, which kuna bundles and loads by default, writes them as Kotlin
instead.

**What is detected, and how.** A class is Kotlin when it carries the `kotlin.Metadata`
annotation, which the Kotlin compiler puts on every class it emits; nothing else (file
name, `SourceFile` attribute, package) counts, and a Java class is never written as
Kotlin. The annotation holds a protobuf description of the Kotlin declarations behind the
class (its properties, functions, constructors, nullable types, default arguments,
modifiers), and the plugin decodes it to decide what to write. The decision is per
top-level class and covers everything nested in it; such a unit is written as Kotlin
source to `<package path>/<Class>.kt`, every other unit as Java to `.java`. A top-level
function lives in a file class named after its source file (`Counters.kt` compiles to
`CountersKt`), and so does its unit. The spec's section 10.15 has the algorithm.

**What the output looks like.** `tests/fixtures/jvm/kotlin` holds two small Kotlin files
compiled with kotlinc 2.0.0 (`src/` has the source):

```
$ kuna decompile tests/fixtures/jvm/kotlin/classes com.ex.kt.CountersKt
package com.ex.kt

public fun total(counters: List<Counter>): Int {
   var sum: Int = 0

   for (c in counters) {
      sum += c.count
   }

   return sum
}

public fun applyTwice(f: (Int) -> Int, x: Int): Int {
   return (f(f(x)) as java.lang.Number).intValue()
}

public fun quadruple(x: Int): Int {
   return applyTwice({ it: Int ->
      it * 2
   }, x)
}

public fun describe(c: Counter): String {
   return "count=${c.count}"
}
```

Declarations come from the metadata (`fun`, parameter and return types, `(Int) -> Int`,
`List<Counter>`, `val`/`var`, a property's `private set`, default arguments, `companion
object`, `data`/`sealed`/`object` classes). Bodies come from the bytecode through the same
pipeline as Java, then rewritten into Kotlin syntax: string templates, `for (x in xs)`,
`when`, `?:`, `is`/`as`, lambdas. What the compiler lowered stays visible where the
metadata says nothing: calls keep their JVM targets (`System.out.println`,
`text.length()`, `StringsKt.repeat(..)` for an extension function), boxing shows as casts
(`as java.lang.Number`), a captured local that a lambda changes is an `IntRef`, and the
body of an inline function (`let`, `forEach`) appears expanded where it was called. A Java
type whose simple name Kotlin reuses is written qualified (`java.lang.String`). The text
is Vineflower's, quirks included: the public setter of `Counter.label` prints as
`internal set` (below).

**The CLI on Kotlin units.** Classes and methods are still named by their class-file
names, which is what `kuna functions` lists, so a selection works the same for Java and
Kotlin input (`docs/cli.md`, "JVM inputs", has the details):

```
$ kuna decompile tests/fixtures/jvm/kotlin/classes com.ex.kt.Counter.getLabel
public final var label: String
   internal set
$ kuna decompile tests/fixtures/jvm/kotlin/classes com.ex.kt.Counter.label
error: no method "com.ex.kt.Counter.label" in com.ex.kt.Counter; a Kotlin property is selected by its JVM accessors: "com.ex.kt.Counter.getLabel", "com.ex.kt.Counter.setLabel"
$ kuna decompile tests/fixtures/jvm/kotlin/classes Counter.Companion.of
public fun of(text: String): Counter {
   return Counter(text.length())
}
$ kuna decompile-project tests/fixtures/jvm/kotlin/classes -o out
wrote out: 2 .kt files (882 bytes); classes: 2 ok, 0 failed; methods: 10 ok, 0 failed
```

A property's getter and setter both select the property's declaration; a companion
object's member is in the `$Companion` class; a method the Kotlin writer folds into
another declaration (a `$default` stub, the body of a lambda compiled to a method, a
`@JvmStatic` forwarder) has no text of its own, and naming it is an error that says so.
Every JSON record carries the `language` of its unit (`java` or `kotlin`).

**Getting Java instead.** `--language java` writes every class as Java (it sets
`jvm_kt_enable off`), which shows what the Kotlin view hides: the accessor methods, the
`$default` stubs, the `Intrinsics` checks, every JVM method on its own:

```
$ kuna decompile tests/fixtures/jvm/kotlin/classes com.ex.kt.Counter.bump --language java
public final int bump(int by) {
   this.count += by;
   return this.count;
}
```

`--language auto`, the default, writes each class in the language the engine picks for it.
There is no `--language kotlin`: the engine can only write a class as Kotlin when the
class was compiled from Kotlin, which `auto` already does, so the flag is refused with that
explanation (exit `2`). `--option jvm_plugins off` also gives Java, since it loads no plugin
at all.

**Options.** The plugin's options are the `jvm_kt_*` rows of `kuna catalog --engine jvm`;
each also answers to Vineflower's name (`kt-enable`, ...):

| Option | Default | Effect |
|---|---|---|
| `jvm_kt_enable` | `on` | write Kotlin classes as Kotlin; `off` writes them as Java with the `@Metadata` annotation |
| `jvm_kt_show_public` | `on` | write the `public` that Kotlin source leaves implicit |
| `jvm_kt_unknown_defaults` | `...` | the text after `=` for a default argument whose value could not be recovered from the `$default` stub; empty writes none |
| `jvm_kt_collapse_string_concat` | `on` | write string concatenations as string templates (`"count=${c.count}"`); `off` keeps `"count=" + c.count` |
| `jvm_kt_decompile_facades` | `off` | also write the forwarding functions of a multifile facade (`@file:JvmMultifileClass`), not only the comment listing its parts |
| `jvm_kt_export_metadata` | `off` | parse the metadata even with `jvm_kt_enable off`, for other plugins; changes no output in kuna |

**Known limits.** The Kotlin output is Vineflower's, so it has Vineflower's gaps. The
plugin rewrites syntax, not semantics, so whatever the compiler lowered stays lowered
(above). A `suspend` function that suspends is not decompiled: its continuation is an
anonymous class without Kotlin metadata, and the plugin fails on it as Vineflower does (one
that never suspends is written with `suspend`). A function reference compiled to a
singleton class prints as `<unrepresentable>.INSTANCE`. The metadata schema is the one
the plugin was built with (Kotlin 2.2), applied without a version check. When Vineflower
cannot decompile a method (and no kuna fix covers it), its Kotlin declaration is written
with Vineflower's failure comment in place of the body, exactly as for Java, and the CLI
reports it on stderr and in the record's `error`; `--language java` usually still gets
the Java view:

```
$ kuna decompile tests/jvm/classes/kotlin/kt/pkg/TestAnonymousEverything.class testFunctionReference 2>&1 | head -6
warning: pkg.TestAnonymousEverything.testFunctionReference()V: IllegalStateException: Anonymous class does not have Class Kotlin metadata
public fun testFunctionReference() {
   // $VF: Couldn't be decompiled
   // Please report this to the kuna issue tracker, at https://github.com/Noelo-Lab/kuna/issues with a copy of the class file (if you have the rights to distribute it!)
   // IllegalStateException: Anonymous class does not have Class Kotlin metadata
   //   at nonnative/kuna-jvm/src/kotlin/kotlin_writer.rs:1273
$ kuna decompile tests/jvm/classes/kotlin/kt/pkg/TestAnonymousEverything.class testFunctionReference --language java
public final void testFunctionReference() {
   var ref = (KFunction & Function2)(new Function2<Integer, Integer, Integer>(this) {
      public final Integer invoke(int p0, int p1) {
         return ((TestAnonymousEverything)this.receiver).add(p0, p1);
      }
   });
   ((Function2)ref).invoke(1, 2);
}
```

The trace line under the exception is kuna's own, as for Java (item 1 of the fidelity
contract). A whole class the plugin cannot write is a `// $VF: Couldn't be decompiled`
comment with the exception. A class whose metadata cannot be decoded is still written as
Kotlin, from its class file alone, under `// $VF: Class flags could not be determined`.

## The other bundled plugins

**IDEA not-null.** IntelliJ IDEA's compiler instruments `@NotNull` parameters and return
values with checks that call a synthetic `$$$reportNull$$$0`. The plugin removes them
(`jvm_resugar_idea_notnull`, on by default) and keeps the annotations; `off` shows them:

```
$ kuna decompile tests/jvm/classes/custom/TestIdeaNotNull.class pkg.TestIdeaNotNull.test1
public void test1(@NotNull String s) {
   System.out.println(s);
}
$ kuna decompile tests/jvm/classes/custom/TestIdeaNotNull.class pkg.TestIdeaNotNull.test1 --option jvm_resugar_idea_notnull off
public void test1(@NotNull String s) {
   if (s == null) {
      $$$reportNull$$$0(2);
   }

   System.out.println(s);
}
```

**Variable renaming.** Off unless asked for: `--option jvm_variable_renaming jad` names
locals after their type the way JAD did (`i`, `j`, `s`, `flag`, `file1`), `tiny` the way
tiny-remapper does, and `jvm_rename_parameters on` renames parameters too. It overrides
the LocalVariableTable's names, so it is for obfuscated or debug-info-free code:

```
$ kuna decompile tests/fixtures/jvm/app.jar com.ex.util.Helper.loop --option jvm_variable_renaming jad
public static int loop(int n) {
   int i = 0;

   while (n > 0) {
      if (n % 3 == 0) {
         i += n;
      } else {
         i--;
      }

      n--;
   }

   return i;
}
```

`tiny` with `jvm_rename_parameters on` fails every method that has a LocalVariableTable
(`NullPointerException: type`), as Vineflower's provider does. The two deprecated
Vineflower flags, `jvm_jad_style_variable_naming` and `jvm_jad_style_parameter_naming`,
are accepted for old command lines.

## Performance and limits

**Parallelism.** `--jobs N|auto` (default `auto`) runs N engine instances on N threads
over the same input (`src/infra/kuna_driver.rs`). The output is byte-identical at every N:
each instance decompiles a batch of root classes in the order a single run would, also
processes (without writing) the other roots its classes depend on, and a batch whose run
read state it did not produce is repeated before its result is used (spec section
10.14). `auto` is the number of cores, at most 24 and at most one per 16 class files,
reduced until the memory estimate below fits in 90% of the available memory. Measured on
an 80-core machine (`decompile-project`, best of three): Elasticsearch (53 MB of classes)
190 s with one worker, 27 s with eight, 12.5 s with twenty-four; Guava (6.6 MB) 20 s with
one worker, 3.4 s with eight, 1.9 s with twenty-four.

**Memory.** An instance reads a class's constant pool entries and members when it first
uses them (spec section 10.3), so it holds little more than the classes its batches touch.
The driver estimates an instance at 32 MiB plus 4 bytes per byte of input class files and
1 per byte of library class files, and the decompiled state and results at 90 bytes per
input byte, whatever the count; the estimate bounds the peaks of Elasticsearch (2.5 GB
with one worker, 4.2 GB with eight, 7.4 GB with twenty-four), Guava (0.37, 0.74 and
1.5 GB), Ghidra (9.1 MB of classes: 0.82 GB with eight, 1.4 GB with twenty-four) and
spring-boot (3.0 MB: 0.16, 0.31 and 0.67 GB).
`--max-fn-memory` is process-wide on JVM inputs, since the instances are threads of one
process.

**Time.** `--max-fn-seconds N` (default 120, `0` disables) is a per-method budget. The
engine polls it at the heads of its fixed-point loops (spec section 10.12); a method past
it is written as `// $VF: Couldn't be decompiled` with a `TimeoutException`, exactly as
Vineflower writes a method its watchdog killed, and the rest of the class is written
normally. A method stuck between two polls is not interrupted; as a last resort the CLI
stops waiting after `2N+30` seconds, writes the classes that finished, gives the stuck ones
a stub comment and exits.

**Speed.** One engine instance is slower than Vineflower on one thread. Against
Vineflower 1.12.0 with `-thr=1` on JDK 21, `--jobs 1` takes 1.2 to 2.2 times the wall time
on whole jars: spring-boot 7.2 s against 6.1 s, Guava 20 s against 13 s, spring-framework
38 s against 21 s, Elasticsearch 190 s against 85 s. kuna uses one core for that;
Vineflower's JVM also runs compiler and garbage-collector threads, and its CPU time is
five to nine times its wall time. The parallel driver is how kuna makes up the
difference: with both decompilers' default flags
on the same 80-core machine, kuna finishes spring-boot in 0.7 s against 3.7 s, Guava in
1.9 s against 5.8 s, spring-framework in 3.9 s against 7.5 s and Elasticsearch in 12.5 s
against 14.5 s, using a third of Vineflower's CPU time or less. These are the best of three
`decompile-project` runs on the benchmark's full input jars on a quiet machine, Vineflower
with its default thread count (one per core). `docs/jvm-benchmark.md` has a different
measurement: one javabench run of each tool back to back on a busy machine, Vineflower on
12 threads, which costs it less CPU time; it also has kuna ahead on wall time, CPU time and
peak memory on every project.

**Hard failures.** A method or class that throws is contained (spec section 10.12). A
stack overflow or a failed allocation is not, and ends the process; the engine threads get
512 MiB stacks, because the port recurses as deeply as Vineflower does.

## Testing

| Command | Checks |
|---|---|
| `make test-jvm` (= `kuna test --jvm --baseline tests/jvm/baseline.txt`) | the gate: every case in the baseline still prints its expected text: the 607 Vineflower-oracle cases (Java, plugin and Kotlin, run in compat mode) Vineflower's exact text, and each kuna-fixes case Vineflower's text in compat mode and kuna's reviewed text by default. Needs no SLEIGH specs, no JDK and no Kotlin compiler. |
| `cd decompiler && cargo test -p kuna-jvm --test integration` | the unit and differential suite (part of `make rust-test`): kernel goldens against a real JDK, per-pass oracles, option-table checks, driver determinism |
| `cd decompiler && cargo run --release -p kuna-jvm --example corpus -- --jobs 16 --baseline ../tests/jvm/baseline.txt` | the same corpus gate without the CLI |

Useful corpus-runner flags (`corpus --help` lists all): `--filter SUBSTR` runs matching
cases in-process (quick); `--out DIR` writes each case's actual text, diff and log;
`--save-baseline ../tests/jvm/baseline.txt` re-records the baseline when a change makes
more cases match (never to absorb a regression); `--manifest FILE` runs another manifest,
such as the full 598-case Java one or the Kotlin one generated from the external corpora.

**Regenerating the corpus.** `tests/jvm/` is generated, never edited by hand. Its source
is the Vineflower oracle, built outside the repository by
`decompiler/nonnative/kuna-jvm/tools/oracle` into the directory named by
`KUNA_JVM_ORACLE`: Vineflower's `testData` sources compiled with the JDK of each source set
(and the Kotlin plugin's with kotlinc 2.0.0), decompiled by the released jar under each
test's options. Its `README.md` lists what it downloads, with pinned hashes, and how the
test fixture is reproduced. With `KUNA_JVM_ORACLE` set, `python3 -I
tests/jvm/kotlin_corpus.py` and `python3 -I tests/jvm/plugin_oracle.py --work NEW_DIR`
refresh the Kotlin and plugin cases and `python3 -I tests/jvm/regen.py` the rest and
`cases.tsv`; with `--out DIR` they write a fresh copy elsewhere, which matches the
committed files byte for byte. The 68 JDK-dependent cases run only from the oracle itself
(`regen.py --skip-vendor --full-manifest FILE`, then `corpus --manifest FILE`).

**When a case differs.** The diff shows the symptom, rarely the pass. The trace differ
(`decompiler/nonnative/kuna-jvm/tools/vftrace/`) runs both engines on the same classes and
options, dumps every method's statement tree after every pass in one canonical format, and
reports per method the first pass whose trees differ. From `decompiler/nonnative/kuna-jvm`:

```bash
tools/vftrace/build.sh                                       # once: needs a JDK 17 javac, the Vineflower 1.12.0 jar and sources
tools/vftrace/trace-diff.sh Foo.class 'Foo$1.class' --fixture --opt ternary-in-if=1
KUNA_JVM_TRACE=kuna.vft <any run>                            # the kuna side alone
```

The build also compiles the oracle's Vineflower driver (`tools/oracle/java/driver`). Its
inputs come from `KUNA_JVM_ORACLE`; the per-input overrides (`VF_JAR`, `VF_SRC`, `JAVAC`,
`BUILD`) are at the top of `build.sh`.

Use it whenever an output difference is not obviously in the writer. It sees only the
statement tree (not the CFG, SSA state or variable names), and a method that throws
mid-pipeline just stops; `tools/vftrace/README.md` has the format and the canonicalization
rules. For quick runs outside the CLI, `cargo run -p kuna-jvm --example jvmdec -- [--opt
key=value]... [--runtime SPEC] [--jobs N] [-o DIR] <class|jar|dir>` takes Vineflower's
option names directly.

## Adding an option or a kuna improvement

**The option table is one to one with Vineflower.** `options.toml` has one `[[option]]`
row per Vineflower key a user may set, its bundled plugins' keys included, named
`jvm_<long name>` with `-` turned into `_` (Vineflower's long and three-letter names are
accepted as aliases), and a `[[driver_owned]]` entry for each key kuna sets itself.
`tests/kuna_options.rs` checks the two together against
`IFernflowerPreferences::all_options()` and the options the bundled plugins declare: every
key is exactly one row or one driver-owned entry, every default agrees with Vineflower's
(a plugin option's with the plugin's), every row's
`inspiration` names the Java constant, and `phase`/`subphase` name a real folder and the
modules that read the key. `build.rs` refuses an incomplete row. There are no hard-coded
option counts to bump, unlike the native catalog. After touching the table, regenerate the
catalog page: `decompiler/target/release/kuna catalog --markdown --engine jvm >
docs/options-jvm.md` (CI checks it is fresh).

**A kuna improvement is a named option.** Anything that changes the emitted Java or Kotlin beyond
what Vineflower prints ships behind a `jvm_*` row of its own: `vineflower = ""`,
`short = ""`, `aliases = []`, `source_decompiler = "kuna"`, a `change_kind` of its own, and
every other field filled in. Its code goes in a `kuna_<slug>.rs` module in the owning phase
folder, with a minimal seam in the ported code; a kuna row resolves to its own name, so it
cannot collide with a Vineflower key. Describe it in `docs/spec/10-jvm.md` (tagged
`(kuna)`, checked by `make check-spec`, whose strict mode requires every row to be
mentioned). A destructive row also updates the pinned destructive list in
`tests/kuna_options.rs`. A strict bug fix that makes kuna match Vineflower needs no
option. A fix of a defect Vineflower itself has is a `jvm_fix_*` row and follows the
recipe of *Fixing a Vineflower bug*; `tests/kuna_fixes.rs` checks that every `KunaFix` has
its row and every `jvm_fix_*` row its `KunaFix`.

**Default-on needs corpus evidence.** The corpus gate compares against Vineflower's text,
so a kuna improvement that changes any baseline case stays off by default. It may default
on only when `make test-jvm` still matches every baseline case with it on, and when it
passes the speed budget (the same rule as the native engine's: a default-on feature must
not majorly slow kuna down). The `jvm_fix_*` rows are the exception by construction: the
Vineflower-oracle cases run in compat mode, where every fix is off, and a fix's evidence is
its two-pass case and the whole-output diffs of step 6 of the recipe.

## Re-syncing with a newer Vineflower

1. Diff the new release against `b8273988` (`git diff b8273988..<tag> -- src/ plugins/`)
   and map each changed Java file to its module through the `//! Port of` headers; a new
   `plugins/kotlin/libs/metadata.jar` means a new metadata schema for `kt_proto`. New files get
   new modules in the folder of their closest neighbours, named by the convention above.
2. Port each change line by line in the same place, keeping the order of operations,
   iteration orders and constructor calls; `cargo test -p kuna-jvm` after each file.
3. Reconcile the options: new keys in `IFernflowerPreferences` or in a bundled plugin's
   options get rows (or `[[driver_owned]]` entries); removed keys lose theirs; changed
   defaults change the row.
   `tests/kuna_options.rs` fails until the table matches. Regenerate
   `docs/options-jvm.md`.
4. If the reference JDK moves, regenerate the embedded varargs table
   (`tests/data/jdk_varargs/gen.sh`) and the kernel goldens under `tests/data/`.
5. Rebuild the oracle corpora (Java, plugins, Kotlin) with the new release jar (`tools/oracle`
   pins 1.12.0's jar hash and commit; update both) and regenerate `tests/jvm/`; run the corpus and use the trace differ on every case that no longer
   matches. Re-record `tests/jvm/baseline.txt` only as part of the sync, and say so in the
   commit message.
6. Update this page (version, commit, the differences above), the crate header in
   `src/lib.rs`, `decompiler/nonnative/README.md`, the `inspiration` field of every row,
   and `NOTICE` if the upstream copyright notices changed.
7. Revisit every kuna fix: when the new release fixes the same defect, the ported change
   replaces kuna's, so retire the fix (its `KunaFix` variant, row, gated sites, case and
   spec paragraph); when it still has the defect, the kuna-fixes case's compat
   expectation is regenerated with the new jar by rerunning `tests/jvm/kuna_fix_case.py`.
