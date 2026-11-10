# The JVM engine

`kuna-jvm` (`decompiler/nonnative/kuna-jvm`) decompiles JVM class files, jars and
directories of classes to Java, and the classes the Kotlin compiler emitted to Kotlin. It
is a Rust port of [Vineflower](https://github.com/Vineflower/vineflower) 1.12.0 with the
three plugins Vineflower bundles (Kotlin, variable renaming, IDEA not-null). On the same
class files and options it prints the text Vineflower prints, byte for byte, except where
kuna fixes a Vineflower defect: a method Vineflower fails on, or code it prints wrong
(*Kuna-only fixes*, below). The fixes are always on; `--option jvm_vineflower_compat on`
turns them all off. It is reached through the
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

## How it relates to the native engine and p-code

```
                         kuna <command> <input>
                                  |
          kuna-cli: sniff the input (magic bytes, or a directory)
                   /                                \
      ELF / PE / Mach-O / raw              .class / jar / directory
                 |                                   |
        NATIVE ENGINE                        NON-NATIVE ENGINE
   kuna-sleigh: bytes -> p-code       decompiler/nonnative/kuna-jvm
   kuna-decomp: P1..P9 on p-code      (Vineflower port, own Java-level IR)
                 |                                   |
            C (or Rust)                       Java or Kotlin
                   \                                /
       shared: CLI commands and flags, option catalog (jvm_* rows),
               text/JSON output shapes, phase-folder naming, docs/spec
```

The two engines meet only in `kuna-cli`, which picks one per input before any engine code
runs (`kuna_input::sniff`: a class-file, zip, jmod, jimage or DEX header, a directory, or a
jar with a launcher stub in front). The JVM engine does not lower bytecode to p-code: a
class file already states its types, exception tables, generic signatures, lambdas and
Kotlin metadata, and p-code would discard all of them. It also has an exact correctness
oracle, Vineflower's text, which only a faithful port of Vineflower's own IR can match byte
for byte.

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
- **Default mode** (the switch off): the same text, except where a kuna fix (*Kuna-only
  fixes*, below) corrects what Vineflower does wrong. Every fix is always on in this mode;
  none has an option of its own. That means three things: a method Vineflower fails on
  and a fix covers is decompiled instead of written as its `// $VF: Couldn't be
  decompiled` comment (with any imports its body needs); code Vineflower prints wrong (a
  statement skipped after a denested `if`, a raw `new C(..)` that loses a lambda's
  parameter types, an `assert` where Java needs an expression, an enum body without its
  `;`, a `finally` copy that swallows the exception) is printed as the program has it, so
  more classes recompile; and the note under a failure comment that remains asks for
  reports to kuna's tracker instead of Vineflower's. Nothing else may differ; that is the
  evidence each fix ships with (*Fixing a Vineflower bug*, below). `docs/jvm-benchmark.md`
  measures both modes against Vineflower on five large projects. One kuna option changes
  the default text further: `jvm_javac_layout` (on; *javac's block layout*, below).

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

   (That is compat mode; by default the second line names kuna's tracker, fix
   `report_to_kuna`.) With `jvm_dump_exception_on_error off` neither decompiler
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
`// $VF: Couldn't be decompiled`. kuna fixes such defects, and code Vineflower prints wrong,
in its own engine. A fix is a correction, not a judgement call, so it has no option: every
fix is always on, and the only way to turn them off is all at once, with `--option
jvm_vineflower_compat on`, which gives Vineflower's exact text. There are 140 fixes, 66 of
them for Kotlin output. Each has an id, used in the docs, the spec and
`tests/jvm/kuna-fixes/cases.json`:

| Fix id | Vineflower 1.12.0 defect | What kuna does |
|---|---|---|
| `report_to_kuna` | the note under every error comment asks readers to report the class to Vineflower's tracker | it asks for reports to kuna's tracker (a `jvm_error_message` you set still wins) |
| `lambda_wildcard_return` | fails a method that passes `x::m` on a receiver typed `Foo<?>` to a generic call, when `m` returns `Foo`'s type variable (`NullPointerException: returnType`; spring-boot's `StructuredLoggingJsonProperties.customizers`) | the method decompiles; the lambda keeps the type its call's context gives it |
| `twr_unary_condition` | fails a method of a Java 11+ class whose `catch (Throwable)` handler starts with `if (!flag)` on a boolean local, taking the condition for a try-with-resources null check (`index out of bounds`; spring-framework's `ConfigurationClassEnhancer.createClass`) | the method decompiles; the `try`/`catch` is written as it is |
| `if_denest_fallthrough` | turns `if (a) { ...; try { ... } ... } Z` into `if (!a) { Z } else { ... }` although the `try` falls through to `Z`: prints a `break` past `Z` on that path (a skipped statement or a missing `return`), or fails the method (`IllegalStateException: Could not find destination nodes for stat id`; Elasticsearch's `InternalEngine.closeNoLock`) | the `if` keeps its shape when its body reaches the code after it; the method decompiles to its source |
| `switch_yield_const_retype` | fails a method whose switch expression yields a string or class literal next to an arm of an unrelated reference type, such as a boxed `char` (`RuntimeException: invalid constant type: Ljava/io/Serializable;`; spring-framework's `BasicJsonWriter.escape`) | the method decompiles; the literal keeps its own type, which the switch's context widens |
| `catch_loop_back_successor` | fails a `while (true)` loop after other code whose body is a `try` with two or more `catch` blocks that all go back to the loop (`RuntimeException: computing post reverse post order failed!`; Elasticsearch's `RetryingInputStream.read`) | the method decompiles; the loop is isolated before its `try`/`catch` is built, as for a single `catch` |
| `kt_synthetic_anonymous_object` | in Kotlin output, fails a method that creates an anonymous class kotlinc gave only synthetic-class metadata: a `Comparator { .. }`, `fun interface` or function-type lambda in an inline function with a `crossinline` parameter (`IllegalStateException: Anonymous class does not have Class Kotlin metadata`; 37 methods of kotlin-stdlib: `compareBy`, `thenBy`, `sortedBy`, `sortBy`, `binarySearchBy`, `AutoCloseable { }`) | the method decompiles, the class written as an `object` expression from the class file; a continuation or local function reference (superclass not `Object`) still fails |
| `kt_local_as_extension_receiver` | in Kotlin output, takes a method without parameters and metadata whose first local is named `$this$..` (a captured extension receiver, or an inlined lambda's receiver) for an extension function and fails (`ArrayIndexOutOfBoundsException`; kotlin-stdlib's `Enumeration<T>.iterator()` with the `remove()` kotlinc adds to its `Iterator`, and 22 whole classes `BeanDefinitionDsl$bean$$inlined$bean$N` of spring-framework) | the method or class decompiles; such a method is written as a plain method |
| `kt_default_args_super_call` | fails a whole Kotlin class whose primary constructor calls its superclass through the superclass's default-arguments constructor when the superclass is decompiled in the same run (`RuntimeException: Constructor X.<init>(...I)V not found`; kotlinx-coroutines' `BroadcastChannelImpl`), and otherwise prints the placeholders and bit mask, `: Base(a, null, 2)` | the call is the constructor it stands for, `: Base(a)`, when only trailing arguments are defaulted and no other constructor could take the rest; otherwise the passed arguments are named, `: Base(a, c = 9L)`, `: Overloaded(a = x)`, when the names select that constructor alone; an inner superclass's masks are read past its outer instance |
| `kt_default_arg_compound_branch` | fails a whole Kotlin class with a default argument computed by several statements, such as an inlined `map` (`NullPointerException`; kotlin-reflect's `AnnotationConstructorCaller`) | the class decompiles; that default is printed as unknown, `= ...`, like any default the plugin cannot read back |
| `generic_parent_without_signature` | fails a method that uses a value whose type is nested in a non-generic inner class of a generic class (`Outer<E>.Itrs.Node`), reading the missing `Signature` of `Itrs` (`NullPointerException`; four methods of the JDK's `ArrayBlockingQueue$Itrs`) | the method decompiles; `Outer`'s type arguments are mapped through `Itrs` as through any enclosing class |
| `self_referencing_lambda` | fails a whole class with a lambda whose synthetic implementation method creates the same lambda again (`Lambda X is recursive?!`, or a `StackOverflowError`; Vineflower's test `TestRecursiveLambda`, obfuscated bytecode) | the class decompiles; those lambdas are written as method references to their method (`this::body`), which stays a method |
| `external_class_diamond` | writes `new C(..)` raw for a generic class `C` that is not in the input (a library), even where the field, local, return or parameter is typed `C<A, B>`, so a lambda argument loses its parameter types and the class does not recompile (wrong output, no failure comment; Elasticsearch's `PARSER = new ConstructingObjectParser("task_info", true, args -> … (String)args[i++] …)`) | `new C<>(..)` wherever the expected type is a parameterization of `C` itself |
| `statement_as_expression` | prints an `assert` where Java requires an expression, which javac rejects: as the body of an expression lambda, `(k, v) -> assert v.equals(k) : k`, and as a `for` loop's update, `for (boolean more = true; more; assert s.valid())` (elasticsearch's `Metadata$Builder`, `Injector`, `DeflateTransportDecompressor.decompress`, ...) | the lambda gets a block body, `(k, v) -> { assert v.equals(k) : k; }`, and the loop stays a `while` loop with the `assert` at the end of its body; the default output differs from Vineflower's only there |
| `enum_body_separator` | prints an enum with members but no constants without the `;` before the members (`public enum Util { public static ...`, javac: `enum constant expected`; 15 classes of Ghidra and Elasticsearch) | the body starts with the `;` line |
| `anonymous_static_qualifier` | qualifies a call of an anonymous class's static method from a class nested in it with the class's missing name, `<unrepresentable>.m(...)` (Elasticsearch's `AbstractTSDBDocValuesProducer`) | the call is written unqualified, as Java requires |
| `assert_dead_throw` | leaves the `throw new AssertionError(...)` an assert was built from as dead code when the assert's `if` reached it through a break (javac: `unreachable statement`), which also keeps its message's variables declared outside their loop (`for (key : keys)`) | the consumed throw is removed |
| `handler_copy_offsets` | fails a method whose try/finally is protected by an exception range with several entries, typically a Kotlin suspend function whose `try` spans a suspension point: the copy of the `finally` handler that range splitting makes has no bytecode offsets, and the finally processing removes offsets it does not have (`removal index (is 0)` or `(is 18446744073709551615) should be < len (is 0)`; kotlinx-coroutines' `awaitClose`, `AbstractFlow.collect`) | the method decompiles; the `try` whose exit holds the inlined copy becomes a plain `try`/`finally`, each split-off part a `try`/`finally` guarded by a semaphore variable |
| `nested_handler_copies` | fails a method whose duplicated handlers include a `catch` and the handler around it (`RuntimeException: parsing failure!`): record patterns inside a `try`/`catch`, Kotlin suspend functions with `try`/`finally`, `use` or `consume` (kotlinx-coroutines' `toList`, `consumeEach`) | the method decompiles; a duplication that was not needed is undone, so the source's single `try` comes back |
| `copied_finally` | writes a `try`/`finally` that irreducible-flow splitting copied as `catch (Throwable)` without the rethrow, which swallows the exception | the copy is a `finally` like its original |
| `type_switch_enum_label` | fails a Java 21+ pattern switch that mixes qualified enum constant labels with type patterns (`case E.X -> ...; case C c -> ...` on an interface), leaving the `EnumDesc` label as the raw switch index (`Invalid switch case set`; Vineflower's own `TestSwitchOnEnumJ21.testComplexPattern`) | the label is resolved (upstream `36ac62ec`) and printed qualified when the switch is on another type |
| `type_switch_pattern_type` | types a type pattern that needs no cast (`case Integer i` on an `Integer` selector) as `Class`, so a switch that also has a constant label fails (`Invalid switch case set` with the JDK loaded, Vineflower's `TestSwitchPatternMatching18.test`/`test2`/`test3`; `invalid constant type` without debug info), and without debug info prints `case Class<Integer> var3` | the pattern is typed by its class; the selector keeps its type |
| `type_switch_label_lookup` | finds a pattern switch's label only in the first position of a case and by value: a case with several labels keeps raw indices (`case Integer _, String _`, `case E.X, E.Y`: `invalid constant type`, index out of bounds; Vineflower's `TestUnnamedVar1.patternSwitch3`), and integer labels come out wrong (`case 5, 7` as `case 5: case 1:`, `case 1 ->` as `case Integer var5 ->`) | every label is resolved; a case of several type patterns prints as one label of unnamed patterns (`case Integer _, String _:`) without javac's re-test |
| `kt_return_outside_lambda` | in Kotlin output, drops `return` (wrong output, no failure comment) in every method without metadata (`@JvmOverloads` overloads, `$$forInline` copies, `DefaultImpls`, `-impl` methods) and in an object's functions written inside a lambda: `fun scale(x: Int): Int { scale$default(x, 0, 2, null) }`, an early `return v` as a bare `v`, a `when` branch `-1 -> return X` as `-1 -> X`; about 600 statements of okio, okhttp, kotlin-stdlib, kotlinx-coroutines, kotlin-reflect | only a lambda's body ends in its bare value; the default differs from Vineflower's output where Vineflower prints these wrongly |
| `kt_failed_method_hidden` | in Kotlin output, leaves a method without metadata whose body fails out of its class with no failure comment (11 members of kotlinx-coroutines and kotlin-reflect, such as `$$forInline` copies of suspend functions) | the method is printed with its `// $VF: Couldn't be decompiled` comment, as in Java output |
| `kt_object_super_call` | in Kotlin output, writes an object expression's superclass without its constructor arguments, `object : Base` for `object : Base(k + 1, s)` and `object : Thread` for `object : Thread()` (65 objects of okio, okhttp, kotlin-stdlib, kotlinx-coroutines, kotlin-reflect) | the superclass is written with the arguments the object's `new` passes it, `object : Task(name, cancelable)`; the default differs from Vineflower's output where Vineflower prints these wrongly |
| `kt_vararg_from_metadata` | in Kotlin output, writes every array parameter of a constructor as `vararg x: Any` (okio's `Options`, okhttp's `ConnectionSpec` with two, kotlin-reflect's `CallerImpl`) and drops `vararg` from primitive and unsigned varargs (`fun String.trim(chars: CharArray)`, `fun maxOf(a: Int, other: IntArray)`) (wrong output) | a parameter is `vararg` exactly when its metadata says so, written with its element type |
| `kt_default_arg_assigned_param` | in Kotlin output, takes the first assignment of a default-arguments branch for the default, so a default computed through locals prints the first local's initializer (`t: Int = this.measure(s)` for `run { val u = measure(s); ...; v - 1 }`) (wrong output) | the default is what the branch assigns to the parameter (directly, or through kotlinc's null-checked temporary); otherwise unknown, `= ...` |
| `kt_constructor_diamond` | in Kotlin output, writes Java's diamond on constructor calls, `ArrayList<>(4)`, which is not Kotlin (196 calls in the Kotlin libraries above) | the type arguments are left to Kotlin's inference, `ArrayList(4)` |
| `kt_local_visibility_comment` | in Kotlin output, writes the note on a local class written outside its function as a line comment reading `outside of methodSupplier`, then the header at column 0 | the note is inline on the declaration's line, `/* $VF: local visibility outside of method */ internal class ..` |
| `kt_default_args_ctor_descriptor` | in Kotlin output, never finds an inner class constructor's defaults, printing `= ...` (okhttp's `Http2Stream.FramingSink(finished: Boolean = ...)`) | the default-arguments constructor is found by the constructor's own descriptor, `finished: Boolean = false` |
| `var_merge_narrowing` | merges two variables of one slot under a type narrower than a value one of them is assigned, which prints a cast the bytecode does not have there, or none (wrong output, no failure comment): every Kotlin suspend loop written as Java casts the suspend call's result before its `COROUTINE_SUSPENDED` check, `Integer v = (Integer)src.next(c)`, which throws where the bytecode returns the marker; kotlinx-coroutines' `var25 = (Waiter)ChannelResult.closed(...)` | the merge is refused; the value keeps its own variable and the cast stays where the bytecode has it. Where the LocalVariableTable declares one local at every store to both and its type admits every value stored and every use, the merge is made with that type instead, the source's one variable (spring-framework's `MethodSpec.overriding`, `modifiers = new LinkedHashSet<>(modifiers)`) |
| `switch_expression_poly_cast` | casts a switch expression whose results are numbers of different types to its reference target when their join is unknown, `return (Number)(switch (x) { case 1 -> 1; default -> 2L; })`, which makes it a standalone numeric switch whose `case 1` yields a `Long` (wrong output, no failure comment) | the switch is written without the cast, a poly expression whose results each fit the target |
| `ternary_numeric_arms` | prints a reference conditional whose boxed arms are numbers of different types as a numeric one, `c ? 1 : 2L` or `c ? 1 : a`, which promotes `1` to `Long` and unboxes `a` (`NullPointerException` on `null`; Elasticsearch's `BucketHelpers`, kotlin-stdlib's `toBooleanStrictOrNull`) (wrong output, no failure comment) | an arm is cast to the conditional's type, `c ? (Number)1 : 2L`, `c ? (Integer)1 : a`, so it stays a reference conditional |
| `kt_reference_anonymous_super_call` | in Kotlin output, reads a function-reference class's owner and name from its constructor's `super(..)` call, which `NestedClassProcessor` has already moved out of an anonymous class (the default, `verify-anonymous-classes` off): a bound reference fails its method (`IllegalStateException: Anonymous class does not have Class Kotlin metadata`; kotlinx-coroutines' `JobKt.invokeOnCompletion`, `BufferedChannel.bindCancellationFun`, spring-framework's `BeanDefinitionDsl.environment`) and an unbound one prints `<unrepresentable>.INSTANCE`; past that, it takes what follows the Kotlin name in the signature constant for the descriptor, which for a function with an inline-class parameter starts with the hash of its mangled JVM name (`Invalid descriptor: -5_sEAP8(...)`; kotlinx-coroutines 1.10.2's `BufferedChannel.bindCancellationFunResult`) | the reference is written, `handler::invoke`, `MatchResult::next`, `::createSegment`, from the super call the class node keeps, its signature split at the `(` (`this::onCancellationChannelResultImplDoNotCall`) |
| `kt_property_reference` | in Kotlin output, does not recognise property-reference classes (`PropertyReference0Impl` and the others): a bound one fails its method (same exception; kotlinx-coroutines' `LockFreeLinkedListNode.toString`) and an unbound one prints `<unrepresentable>.INSTANCE` | the reference is written: `this::classSimpleName`, `Owner::name`, `Receiver::name` for a top-level extension property, `::name` |
| `kt_anonymous_class_failure_reason` | fails what has no Kotlin form, suspend functions' continuations above all, with the bare `Anonymous class does not have Class Kotlin metadata` | the method still fails; the message names the class and says why: a suspend function's coroutine state machine, a reference to a local function that captures variables, an object created outside the method that declares it (code inlined from another function) |
| `nonpublic_nested_import` | imports a non-public member class of a library type (`import org.apache.lucene.search.Weight.DefaultScorerSupplier;`), which javac rejects (`has protected access`), so the whole file does not compile (wrong output, no failure comment; 20 Elasticsearch files) | the class is named through its enclosing class, `Weight.DefaultScorerSupplier` |
| `override_inaccessible_method` | writes `@Override` on a method whose only match in a supertype is private, or package-private in another package (javac: `method does not override or implement a method from a supertype`; Elasticsearch's `ElasticsearchNodeCommand` subclasses) | no `@Override` there |
| `primitive_array_wildcard` | drops the wildcard of a type argument bounded by a primitive array, `Publisher<? extends byte[]>` printed `Publisher<byte[]>`, so an overriding method no longer overrides (spring-framework's `ByteArrayEncoder.encode`) | the wildcard is kept |
| `pattern_variable_scope` | makes `if (x instanceof T) { v = (T) x; ...}` a pattern `x instanceof T v` although `v` is assigned on the other path or read after the `if`, outside the pattern's scope (javac: `cannot find symbol`, `already defined`; Elasticsearch's `ES812PostingsReader.postings`, Ghidra's `FileBytesAdapterV0.createBuffers`, spring-framework's `CustomAutowireConfigurer`) | the pattern is made only when every use of the variable is in its scope |
| `switch_case_scope` | forgets a case's declarations when the case's statements end and visits cases out of written order, so two cases declaring the same name are written without braces (javac: `already defined`; Guava's `DoubleMath.roundIntermediate`) | the clash puts the cases in braces, as the source has them |
| `local_class_anonymous_subclass` | declares a local class after an anonymous class that extends it, where javac cannot resolve it (Guava's `Types.ClassOwnership.detectJvmBehavior`) | the local class is declared before that anonymous class |
| `kt_for_loop_exit_condition` | in Kotlin output, writes a Java `for` loop it cannot turn into a range as `while (true) { if (COND) break ...` with `COND` the loop's *continue* condition, so the loop stops exactly when it should go on (wrong output; 63 loops in okio, okhttp and kotlin-stdlib, such as okio's `GzipSink.updateCrc`, `if (remaining > 0L) break`) | `while (COND) { ... UPDATE }` |
| `kt_for_range_counter` | in Kotlin output, rewrites `for (V = A; L < R; W++)` into `for (V in A..B)` or `repeat(N) { V -> }` without checking that `W` is `V`, that the comparison bounds `V` on the side it moves towards, that the body leaves `V` and the bound alone and that nothing after the loop reads `V`: kotlinc's array loops come out as `for (var5 in size..var4)` with the counter as the bound (wrong output; 446 loops in okio, okhttp and kotlin-stdlib, okhttp's `PublicSuffixDatabase.findMatchingRule` `for (var22 in var15.length..var19)`) | the range form only for a loop that counts its initializer's variable to the bound; otherwise the `while` form |
| `kt_for_range_exclusive_bound` | in Kotlin output, writes the exclusive bound of `V < X` as the inclusive `A..X` when `X` is a variable and as `repeat(N - 1)` when `N` is a constant, one iteration too many or too few (wrong output; 61 loops in okio and kotlin-stdlib, `reverse`'s `for (index in fromIndex..midPoint)`) | `A until X`, `repeat(N)`; a decrementing loop with a variable bound, or a constant bound that would overflow, takes the `while` form |
| `kt_for_range_unmatched_bound` | in Kotlin output, prints nothing for a Java `for` loop whose bound is neither a constant nor a variable, `for (i = 0; i < a.length; i++)`: the `break resugar` of `KDoStatement` skips both the range and the fallback, so the loop disappears (wrong output; 77 loops in okio and kotlin-stdlib, `Duration.parse`'s `skipWhile` loops) | the loop in the `while` form |
| `kt_for_loop_definitions` | in Kotlin output, leaves out the declarations attached to a `for` loop it writes as a `while`, so a counter declared by the loop is assigned but never declared (22 loops) | the declarations before the loop, as for the other loop kinds |
| `kt_for_loop_continue_update` | in Kotlin output, puts the update of a `for` loop it writes as a `while` at the end of the body, which a `continue` skips: `continue` loops forever or repeats an element (wrong output; 11 loops in okio, okhttp and kotlinx-coroutines, such as okhttp's `HttpUrl.portColonOffset` and coroutines' `JobSupport.notifyCancelling`) | when the body continues the loop, `if (C) { do { ... } while (run { UPDATE; C }) }`, where `continue` runs the update |
| `var_merge_liveness` | merges two variables of one slot whose lifetimes overlap after the Kotlin schedule's `SimplifyStack` with `inlineRegularVars` replaced a local by the stack variable it copies, so the later one overwrites a value still in use: okhttp's `findMatchingRule` returns `if (var39.size() > var39.size()) var39 else var39`, `StandardAndroidSocketAdapter(var10, var10, var9)` (wrong output; 14 methods of okhttp, kotlin-stdlib and kotlinx-coroutines, such as `File.toComponents`, whose root becomes the rest of the path; none in the five Java benchmark projects) | the merge only when no write of the later variable (other than a copy of the earlier one) can be followed by a read of the earlier one's value |
| `kt_partial_array_dims` | in Kotlin output, fails a method that creates an array of arrays with only its outer length, `arrayOfNulls<ByteArray>(n)` or `Array(n) { IntArray(it) }`, reading a length for every dimension of the type (`index out of bounds`; okio's `Buffer.snapshot(Int)` and `commonSnapshot`, okhttp's `PublicSuffixDatabase.findMatchingRule`) | the method decompiles; the innermost sized level is `arrayOfNulls(n)` |
| `irreducible_with_exception_edges` | fails a method whose loop is entered at more than one place and holds a `try` (`parsing failure!`): node splitting is never tried where an exception edge exists, and in such a loop no statement dominates the `try`, so the search that isolates regions never proposes it (Kotlin suspend functions with a loop in `try`/`finally` or `consume { }` that suspends in its middle; kotlinx-coroutines' `StateFlowImpl.collect`, `SharedFlowImpl.collect`, `StreamFlow.collect` and the deprecated channel operators `indexOf`, `maxWith`, `toChannel`, ...) | the method decompiles: where Vineflower gives up, the smallest single-entry region holding a handler's whole range is built first, or a statement outside every protected range is copied, and Vineflower's own splitting does the rest |
| `kt_semaphore_store_liveness` | in Kotlin output, drops every `false` store to a `finally` semaphore as dead, because the guard that reads it is not an exprent: the guard reads a variable declared after it (`val var8: Boolean`, `val var25: <unknown>`) while the `true` store goes to a variable of its own, so the output no longer says when the `finally` runs (wrong output; inline functions returning across a `try`/`finally`, suspend functions: 18 methods of okio, okhttp and kotlinx-coroutines), or variable definition fails (`Var types not processed!`; `ChannelsKt__DeprecatedKt.toMap`, `StateFlowImpl.collect`) | the stores to a variable a `finally` guard reads are kept, and the Kotlin output is the Java output's: `var var8: Boolean = false /* VF: Semaphore variable */`, `var8 = true`, `var8 = false` on each normal exit |
| `lambda_catch_var_rename` | declares a lambda body's catch variable under its table name after renaming its uses for a clash with the enclosing method: `catch (Exception e) { log(ex); }` (Elasticsearch `TransportNodesReloadSecureSettingsAction`, `BlobStoreRepository`, spring `DefaultListableBeanFactory`) | the clause declares the name the uses read |
| `type_annotation_qualified_name` | drops the qualifier of a type that carries a type annotation: `Map.@Nullable Entry` is written `@Nullable Entry`, another type or none (wrong output; Guava `AbstractMapBasedMultiset`, `Optional.toJavaUtil`, spring `AnnotationJmxAttributeSource`) | the qualifier the import collector gives the type, before the annotation |
| `type_annotation_argument_scope` | writes a type annotation of a later generic argument in an earlier one: `Function<Map<String, @Nullable @Nullable Object>, @Nullable Map<String, Object>>` (wrong output; spring `MergedAnnotationCollectors.toMultiValueMap`) | each annotation in the argument its path names |
| `assert_throw_after_if` | leaves `if ($assertionsDisabled || cond) { rest } throw new AssertionError();` when the code after an assert returns or ends a loop body ("Could not resugar all assert statements"; Elasticsearch `MultiChunkTransfer`, `IndexLongFieldRange`, `Rounding`) | `assert cond;` followed by the rest |
| `sealed_private_permits` | names private member classes in a top-level sealed class's `permits` clause, which javac rejects (Elasticsearch `DiskBBQBulkWriter`) | no `permits` clause, as the source must have had |
| `switch_expression_assignments` | turns a switch statement that assigns a variable more than once in a case, or leaves a case without assigning it, into a switch expression: `yield` before unreachable code, arms without a value (wrong output; Elasticsearch `JavaDateMathParser.parseMath`, `FunctionScoreQuery.computeScore`, spring's ASM `ClassReader`, Ghidra `RGB32ColorDataType`) | the switch statement |
| `nested_param_rename` | declares a parameter of an anonymous class's method under a name its uses do not read: `onFailure(Exception ex) { log(e); }` (Elasticsearch `TransportService`, `SearchService`, `AbstractThreadedActionListener`) | the name the uses read |
| `switch_expression_arm_scope` | declares a variable in a switch expression's arm under a name already declared around the expression (spring's ASM `Frame`) | the clash is scoped or renamed |
| `unchecked_type_pattern` | turns `if (x instanceof C) { C<A> v = (C<A>) x; ...}` into the type pattern `x instanceof C<A> v` (or `x instanceof T v` for a type variable `T`) even when the cast is unchecked, which javac rejects (`instanceof.reifiable.not.safe`; Elasticsearch's `Settings`, `FlattenedFieldMapper`, spring-framework's `CustomAutowireConfigurer`; wrong output, no failure comment) | the `instanceof` and the cast stay as in the bytecode; a pattern whose type the operand's type determines (`Box<String>` to `Leaf<String>`) or a reifiable one (`List<?>`) is still made |
| `inner_method_argument_shift` | matches the arguments of a call to a method of a non-static inner class to the generic signature's parameters one place late, as if the method took the outer instance like the class's constructors, and casts arguments to the neighbouring parameter's type: `this.subMap((boolean)fromKey, true, (boolean)toKey, false)`, `segment.put((V)key, (long)value, now)` (Guava's `AbstractMapBasedMultimap`, `LinkedHashMultimap`, Elasticsearch's `Cache`; wrong output) | the arguments are matched from the first parameter, and only the method's own type variables are inferred from them |
| `raw_receiver_erasure` | infers a call's generic result from its arguments even on a receiver whose type is raw, where Java erases the member (JLS 4.8): drops the cast in `(String[])map.keySet().toArray(new String[0])` on a raw `Map`, and writes a for-each over a raw collection with a typed loop variable (spring-framework's cglib, Elasticsearch's `KeyStoreWrapper`, `MapBinder`; wrong output) | a call on a raw-typed receiver has its erased types, so the bytecode's cast is written; a for-each over a raw iterable casts it to its class parameterized by the loop variable's type, `(List<MethodInfo>)methods` |
| `boxed_type_variable_cast` | drops the boxing call under a cast to a type variable, `return (T)0;` for `(T) Integer.valueOf(0)`, though Java has no cast from a primitive to a type variable (Guava's `Defaults.defaultValue`, spring-framework's `NumberUtils`; wrong output) | the boxing call stays: `(T)Integer.valueOf(0)` |
| `lambda_argument_overload` | writes an implicitly typed lambda or a method reference without the cast that picks its overload where another overload takes a different functional interface of the same shape, which javac rejects as ambiguous: `spliterator.forEachRemaining(this::add)` on a `Spliterator.OfInt`, `MethodIntrospector.selectMethods(type, m -> ...)` (Guava, spring-framework; wrong output) | the argument is cast to the overload's interface, `(IntConsumer)(this::add)` |
| `static_type_variable_scope` | counts a class's type variables in scope in its static methods, so a static method's unresolved type variable `V` is taken for the class's and written in a cast, `getUninterruptibly((Future<V>)future)`, which javac rejects (`non-static type variable V cannot be referenced from a static context`; Guava's `AbstractFuture`, `ElementOrder`, `ClosingFuture`; wrong output) | a class's type variables are not in scope in its static methods |
| `unchecked_generic_cast` | writes no cast where a returned or assigned value's declared generic type does not fit its target, because the unchecked cast the source had (`(ReferenceEntry<K, V>) NullEntry.INSTANCE`, `(CacheBuilder<K1, V1>) this`, `(Comparator<Object>) comparator`) compiles to no instruction; javac rejects the bare value (`incompatible types`; Guava's `LocalCache`, `CacheBuilder`, `Ordering`, `Predicates`, Elasticsearch's `Script`; wrong output) | `this`, a field (static or of `this`) or a declared variable whose type the target's type arguments do not accept gets the cast: to the target, or to its raw class where the two parameterizations are provably distinct |
| `kt_when_case_type` | in Kotlin output, writes a `when` label with its constant's own type instead of the selector's: `58 -> return i` on a `Char` (okhttp's `HttpUrl.portColonOffset`), `'뉯' ->` for a string's hash code `45679` (kotlin-stdlib's `Path.checkFileName`), which do not compile (18 `when`s) | the labels have the selector's type, `':' ->`, `45679 ->` (a form feed `'\u000c'`) |
| `kt_method_object_return_type` | in Kotlin output, writes a method without metadata that returns `Object` without a return type, a `Unit` function returning a value (`fun describe(value: Any?) { return .. }`; `$$forInline` copies, `invokeSuspend`, reflection's `default$invoke`, 133 methods) | the return type is written, `: Any?` |
| `kt_secondary_constructor_line` | in Kotlin output, does not end the line of a secondary constructor without a body, `: super(cause)}` | the line ends (layout only) |
| `kt_range_operand_precedence` | in Kotlin output, writes a range bound that binds less tightly than the range operator bare, `for (index in if (c) 1 else 0 until length)`, whose `else` branch becomes the range (kotlin-stdlib's `toBigIntegerOrNull`) | the bound is parenthesised, `(if (c) 1 else 0) until length` |
| `kt_lambda_singleton_returns` | in Kotlin output, writes every exit of a lambda compiled to a singleton class (`INSTANCE`) as its bare value, so an early `return@label v` in a loop becomes the statement `v` and the lambda runs on (wrong output) | a lambda with an exit not in tail position is labelled and its exits are `return@lambda_1 v` |
| `kt_type_parameter_id` | in Kotlin output, writes `Any` for every type that refers to a class's (or an outer class's) type parameter, `fun get(index: Int): Any` for `E`, `Map<Any, Any>` for `Map<K, V>` (about 920 declarations of kotlin-stdlib, kotlinx-coroutines, kotlin-reflect) | the parameter's name, `fun get(index: Int): E` |
| `kt_java_cast_syntax` | in Kotlin output, writes casts in Java's syntax, `(int)Math.min(a, (long)b)`, `(T)value` (about 2,500 in the Kotlin libraries) | a numeric conversion is Kotlin's conversion call, `Math.min(a, b.toLong()).toInt()`, a cast to a type variable `value as T`; a cast to a class or array type keeps Java's syntax, as Kotlin's `as` would reject `null` |
| `lambda_pertinence` | counts a lambda or an inexact method reference argument as fixing the type variables of its parameter type, where javac resolves them first, so a generic call that is the qualifier of another and whose variables only such arguments fix gets no type arguments, `names.map(n -> () -> n.length()).iterator()`, `Comparator.comparingInt(Constructor::getParameterCount).reversed()`, which javac rejects (spring-boot's `Instantiator`, Elasticsearch's `SniffConnectionStrategy`; wrong output); counts a call that infers its result from its target only (`Predicates.always()`) as fixing a bare type variable, and a varargs array as its type, not its elements', so `Stream.of(lambda, Predicates.always())` gets none either (Elasticsearch's `IndexRoutingTable`, 43 methods lost with the class) | the call gets its explicit type arguments, `names.<Task<Integer>>map(...)`, `Stream.<Predicate<ShardRouting>>of(...)` |
| `lambda_type_variable_target` | writes a lambda passed for a bare type variable, `Iterators.single((builder, p) -> ...)`, without the cast the source had where the call is an argument of another generic call whose other arguments resolve that variable to a type that is not a functional interface (Elasticsearch's `BlobStoreRepository`, `BulkResponse`; wrong output) | the lambda is cast to its interface, `single((ToXContent)((builder, p) -> ...))` |
| `unchecked_call_result_cast` | writes no cast where a returned or assigned call's declared result does not fit its target, because the unchecked cast the source had (`(Constructor<Exception>) exceptionClass.getConstructor(..)`) compiles to no instruction (spring-framework's `ServiceLocatorFactoryBean`; wrong output) | the call gets the cast of `unchecked_generic_cast`, its type being the method's return type with the receiver's declared type arguments |
| `loop_body_exit_explicit` | drops the exit of a loop's body when the loop is the last statement of an outer loop's body and the exit goes where that body's end goes: `continue outer` out of an inner `for` prints as nothing and the inner loop runs on (wrong output, no failure comment; Vineflower's own jar prints the same) | the exit is printed (`break;`) |
| `type_variable_shadowing` | takes an enclosing class's or method's type variable for a same-named one declared closer to the code, so a cast judged by its bounds is dropped: `use((Number)t)` becomes `use(t)`, which calls another overload (wrong output) | the innermost declaration's bounds are used |
| `compound_shift_count_type` | types the `int` count of a compound shift of a `long` (`mask >>>= sz`) as a `long`, so `sz != 0` prints as `sz != false` (Ghidra's `Utils.leastsigbit_set`) | the count is typed as a shift count, `sz != 0` |
| `definition_const_type` | writes the `0`/`1` of a dead store declared `int` from the local variable table as a boolean, `int var4 = false;` (Ghidra's `InstructionDB.getLength`, Guava's `BigIntegerMath`) | the constant is written in the declared type, `int var4 = 0;` |
| `catch_tail_inline` | moves code only a catch clause reaches after the `try` when the `try` body cannot complete normally, out of the scope of the clause's variable (spring-boot's `LambdaSafe.invoke`, `this.logNonMatchingType(callbackInstance, ex)` after the `try`) | code that reads the clause's variable stays in the clause |
| `diamond_argument_conflict` | writes a diamond on a generic `new` whose arguments bind a type parameter to another type than the expected type needs, which javac cannot infer (`Supplier<Set<V>> f = new EnumSetSupplier<>(valueClass)` with a `Class<V0>`) | the `new` is written raw, the unchecked conversion the source made |
| `kt_default_args_call` | in Kotlin output, writes a call that leaves arguments to their defaults as the call of the synthetic bridge it compiles to, with the placeholders, bit mask and marker: `StringsKt.indexOf$default(s, c, 0, false, 6, null)`, `Response.header$default(r, "Expires", null, 2, null)`, `CompletedExceptionally(e, false, 2, null)` (452 calls and 48 constructor calls in okio, okhttp, kotlin-stdlib and kotlinx-coroutines), which Kotlin cannot call | the call the source makes: `StringsKt.indexOf(s, c)`, `r.header("Expires")`, `CompletedExceptionally(e)`; a passed argument after a defaulted one, or one another overload could take, is written by name (`join(a = x, c = "m")`); a call that cannot be resolved to one function with those defaults, or that is the body of an `@JvmOverloads` overload, is left as it is |
| `kt_generated_constructor_hidden` | in Kotlin output, writes the constructors kotlinc generates for default constructor arguments (the no-argument constructor of a class whose parameters all have defaults, and each `@JvmOverloads` overload) as functions named after the class, `fun Builder() { this(null, 1, null) }`, which Kotlin does not accept and the source does not have (11 in okhttp, kotlin-stdlib and kotlinx-coroutines) | they are not written: the primary constructor's defaults (and its `@JvmOverloads`) say they exist |
| `kt_java_reference_cast` | in Kotlin output, writes casts to class and array types in Java's syntax, `(byte[])this.data`, `(java.util.List<T>)listOf(e)`, `((Http2Stream)this).id`, `(java.lang.String)null` (about 600 in okio, okhttp, kotlin-stdlib and kotlinx-coroutines) | Kotlin's `as` with Kotlin type names, `listOf(e) as List<T>`, `(this as Http2Stream).id`; `as T?` where the operand may be `null` (`null as String?`, a nullable parameter, a type-variable value, the upcast `x as Any?`); a cast that only picks a Java overload for a field of the parameter's own type is left out |
| `kt_explicit_type_arguments` | in Kotlin output, writes explicit type arguments before the called name, `CollectionsKt.<File>last(list)` (7 in kotlin-stdlib and kotlinx-coroutines) | after it, `CollectionsKt.last<File>(list)`; left out when one is not known in full |
| `kt_abstract_class_constructor` | in Kotlin output, does not read an abstract class's constructors from its metadata, and writes them as functions named after the class, `open fun AbstractList() {}`, `open fun ForwardingSink(delegate: Sink) { super() ..}` (41 in okio, okhttp, kotlin-stdlib and kotlinx-coroutines) | the primary constructor in the class header with its supertype call, `abstract class ForwardingSink(delegate: Sink) : Sink`, `protected constructor()`, its code in `init`, and secondary constructors as `constructor(..) : this(..)` |
| `kt_constructor_body_after_first_block` | in Kotlin output, drops a constructor's code after its `super(..)`/`this(..)` call when that code starts with an `if`: the checks and field stores of `IntProgression`'s `init`, `UndispatchedCoroutine`'s (6 constructors in kotlin-stdlib and kotlinx-coroutines, 3 more in abstract classes) | writes it, in `init { .. }` or the secondary constructor's body |
| `kt_inline_class_constructor_accessor` | in Kotlin output, takes the synthetic accessor kotlinc adds for a constructor with an inline-class parameter as the constructor: `class UIntRange(..) : UIntRange(start, endInclusive)` and the real constructor as `fun UIntRange(start: Int, endInclusive: Int) { super(start, endInclusive, 1, null) }` (10 in kotlin-stdlib) | the real constructor: `class UIntRange(..) : UIntProgression(start, endInclusive, 1)` |
| `kt_constructor_default_signature` | in Kotlin output, does not find a constructor whose metadata leaves out its JVM descriptor, and writes it as a function, `fun Unit() {}` in `object Unit` | looks it up by the descriptor its parameters give, as Kotlin's reader does |
| `kt_jump_to_non_loop_label` | in Kotlin output, writes a jump out of a labelled `if` as `labelN@ if (..) { .. break@labelN }`, out of a labelled `switch` or unresugared `for` (both in `run labelN@{ .. }`) as `break@labelN`, and the next iteration of a `repeat` loop as `continue@labelN`: Kotlin's `break`/`continue` need a loop (21 in okio, okhttp, kotlin-stdlib and kotlinx-coroutines) | `run labelN@{ if (..) { .. return@labelN } }`, `return@labelN` |
| `kt_synthetic_object_super_call` | in Kotlin output, fails every suspend function that suspends: its continuation is an anonymous `ContinuationImpl` subclass with only synthetic-class metadata, whose constructor passes `$completion` to its superclass (`IllegalStateException: Anonymous class does not have Class Kotlin metadata`; 59 failure comments in kotlinx-coroutines 1.10.2, 64 in 1.8.0: `joinAll`, `first`, `collect`, `emit$$forInline`, `doSelectSuspend`, `withLock`) | the function decompiles to its state machine, as Vineflower prints a suspend function that does not suspend, the continuation an `object : ContinuationImpl($completion) { .. }` expression with its fields and `invokeSuspend`; its type, which Kotlin cannot name, is `<unrepresentable>` |
| `kt_file_class_constructor` | in Kotlin output, writes the constructor kotlinc gives a multifile class part (`-Xmultifile-parts-inherit`, as in kotlin-stdlib) among the file's top-level declarations, `open fun StringsKt__StringsKt() {}` (88 in kotlin-stdlib) | leaves it out: a Kotlin file has no constructor |
| `kt_when_break` | in Kotlin output, writes Java's unlabelled `break` out of a `switch` that ends a statement inside a case as `break` in the `when`: Kotlin rejects it outside a loop and inside one it leaves the loop (24 in okhttp, kotlin-stdlib and kotlinx-coroutines, 4 more through `kt_java_statement_jump`) | leaves it out where the branch ends after it; otherwise `return@labelN` from a `run labelN@{ .. }` written around the `when` |
| `kt_java_statement_jump` | in Kotlin output, writes the jump after a statement in a `try` block, a `finally` block or a `synchronized` block in Java's syntax, `break;`, `break label161;` (6 in kotlinx-coroutines) | Kotlin's: `break`, `return@label161` |
| `kt_catch_block_jump` | in Kotlin output, drops the jump after a `catch` block, so a `catch (e) { ..; continue }` falls through to the code after the `try` (1 in kotlin-stdlib, `DeepRecursiveScopeImpl.runCallLoop`) | writes the jump |
| `kt_supertype_type_arguments` | in Kotlin output, writes the superclass call in a class header without the superclass's type arguments, `: AbstractCoroutine(parentContext, true, active)`, which Kotlin requires (12 in kotlin-stdlib, 43 in kotlinx-coroutines) | `: AbstractCoroutine<T>(..)`, from the class's metadata |
| `anonymous_class_created_elsewhere` | maps an anonymous class's captured variables only from `new`s in the class that declares it; kotlinc's `$$inlined$` copies of an inline function's lambda create its objects in another class, so the object's first captured value is read as the declaring class's `this` and the rest as the object's own fields: wrong Java and Kotlin-lambda bodies (`FlowKt__LimitKt.this.element++`; 17 classes in kotlinx-coroutines 1.8.0, `sequenceOf` in kotlin-stdlib), and in Kotlin output a failure for an object without Class metadata (`Anonymous class does not have Class Kotlin metadata`; 15 in kotlinx-coroutines 1.10.2: `drop`, `take`, `chunked`, `combine`) | an anonymous class that only one other class creates is nested in it, and its captures are mapped from the creating method; an argument every `new` passes as the same final field of the creating instance reads that field (`Creator.this.$count$inlined`) |
| `captured_var_clash_name` | gives the variable that stands for a captured value in an anonymous or local class or a lambda the name the creating method's variable has when the class is processed; the clash renaming later renames that variable (`x` becomes `xx`) where another of its name is in scope, and the captured copy keeps the old name: the nested body reads the other variable, wrong code in Java and Kotlin output (kotlinx-coroutines' `catchImpl`: the object stores into the `Throwable fromDownstream`, not the `ObjectRef fromDownstreamx` it captures; elasticsearch: 3 classes), and the Kotlin writer fails an object without Class metadata whose capture's name differs from the argument's (`catchImpl` in 1.10.2) | a captured variable is printed with the clashing name of the variable it holds |
| `captured_var_through_cast` | takes a constructor argument for a captured variable only when it is a variable; the Kotlin plugin inlines kotlinc's cast copy of an inline lambda's parameter (`val cont = cancellable as CancellableContinuation`) into the `new`, so the object keeps a field and a constructor parameter: Kotlin output fails an object without Class metadata (kotlinx-coroutines 1.10.2's `awaitClose`) and writes a lambda-form one reading `this.$cont` (1.8.0) | a cast of a variable whose class is a subtype of the cast's class is the capture of that variable; the object reads the variable (`cancellable$iv`) |
| `kt_legacy_constructor_parameter_checks` | in Kotlin output, drops Kotlin 1.4's parameter null checks (`Intrinsics.checkNotNullParameter`) but keeps Kotlin 1.3's (`checkParameterIsNotNull`); in a constructor they come before the `super(..)`/`this(..)` call, which is then not found: the class header lacks the call and `init` holds `super(..)` as a statement (94 in okio 2.6.0 and okhttp 4.7.2), a secondary constructor is written `constructor(x: T) : Intrinsics.checkParameterIsNotNull(x, "x") { super() .. }` (18), and property initialisers are taken from the other constructors only (a property initialised twice, a `val` assigned again) | a constructor's checks before its delegation call are dropped too, as kotlinc's non-null parameter types imply them: `class GzipSink(sink: Sink) : Sink`, `: ForwardingSink(delegate)`, `constructor(sink: Sink, deflater: Deflater) : this(Okio.buffer(sink), deflater)` |
| `kt_hidden_empty_super_call` | in Kotlin output, loses the empty superclass call that `hideEmptySuper` removes for Java: a class or object header names the superclass without a call, `class RealResponseBody(..) : ResponseBody`, `object Default : Random` (okio 4, okhttp 22, kotlin-stdlib 46, kotlinx-coroutines 1.8.0 52 and 1.10.2 44), which Kotlin rejects; a secondary constructor takes its first statement for its delegation call (`constructor(s: String) : this.remember(s)`), and one with no other statement is left out (6 in okhttp and kotlin-stdlib) | the header writes `ResponseBody()`; a secondary constructor is written without a delegation call (the same implicit `super()`), its statements in its body |
| `kt_compound_bitwise_assignment` | in Kotlin output, writes a compound assignment with a bitwise or shift operator in Java's syntax, `this.label \|= Integer.MIN_VALUE` (in every suspend function's continuation), `set \|= 1 shl id`, which Kotlin does not have (241 in okio, okhttp, kotlin-stdlib and kotlinx-coroutines 1.8.0 and 1.10.2) | `this.label = this.label or Integer.MIN_VALUE`, `set = set or (1 shl id)`, where the left side reads the same place again with no effect and its type is `Int`, `Long` or `Boolean` |
| `kt_switch_head_indent` | in Kotlin output, writes the statements before a `when` (the switch's first block, where a branchy `when` hoists its variables) at indentation 0 after one indent, so all but the first start in column 0 (okhttp 8, kotlin-stdlib 7, kotlinx-coroutines 1.8.0 240 and 1.10.2 257 lines) | writes them at the `when`'s indentation, as Java's `SwitchStatement` does; whitespace only |
| `kt_operator_precedence` | in Kotlin output, parenthesises operands by Java's precedence although Kotlin's differs: its infix functions `shl shr ushr and or xor` (and every `infix fun`) share one level, below arithmetic and above comparison, and its elvis binds above `is`, comparison, `&&` and `\|\|`, so `x \| 1 << k` comes out `x or 1 shl k` (Kotlin: `(x or 1) shl k`), `a == b ^ true` as `a == b xor true` (kotlinc rejects it), `x ?: (a && b)` as `x ?: a && b` (wrong output; 135 statements in okio, okhttp, kotlin-stdlib and kotlinx-coroutines 1.8.0 and 1.10.2, okhttp's `(this.set and 1 shl id) != 0`) | adds the parentheses Kotlin's precedence needs, `x or (1 shl k)`, `(a == b) xor true`, `x ?: (a && b)`, and keeps Java's |
| `kt_narrow_bitwise_operand` | in Kotlin output, writes a `Byte`, `Short` or `Char` operand of a bitwise or shift operator as Java does, without its promotion to `Int`: `b and 255`, `c xor 32`, `x shl n`, `b.inv()`, `buffer[i] ^= key[j]`, which kotlinc rejects (`Byte` has no `and`; 628 conversions in okio, okhttp and kotlin-stdlib) | `b.toInt() and 255`, `c.code xor 32`, `x shl n.toInt()`, `(data[p].toLong() and 255L)` where Java widens to `long`, `buffer[i] = (buffer[i].toInt() xor key[j].toInt()).toByte()` |
| `kt_reference_inequality` | in Kotlin output, writes Java's `!=` of two references (`if_acmp`, an identity test) as Kotlin's `!=`, which calls `equals`, while its `==` becomes `===`: `a !== b` comes out `a != b` (wrong output; 293 in okio, okhttp, kotlin-stdlib and both kotlinx-coroutines, `state !== CLOSED`) | `a !== b`; a comparison resugared from `Intrinsics.areEqual` keeps `==`/`!=` |
| `handler_copy_shared_body` | fails a method whose `finally` body branches (`if (consume) cancel(cause)`, or a Boolean argument, which kotlinc compiles to a branch) and whose `try` is protected by a range with several entries, typically a Kotlin suspend function whose `try` spans a suspension point: range splitting copies only the handler's first block, the copies share the rest of the `finally`, which then belongs to no copy's `try`, and the finally processing throws on the first copy (`removal index (is 0) should be < len (is 0)`; kotlinx-coroutines' `emitAllImpl` in 1.8.0 and 1.10.2, and in Java output `combineInternal`'s launched lambda) | the method decompiles; each copy gets its own copy of the blocks only the handler runs, and each part split off at a resume label is a `try`/`finally` guarded by a semaphore variable |
| `kt_field_without_property` | in Kotlin output, writes a field that has no Kotlin property (every field of a class kotlinc gives no Class metadata: the spill fields of the continuations it copies into `$$inlined$` classes and of the continuation objects of suspend functions) as a Java declaration, `open int label;`, `open Object L$0;`, which kotlinc rejects (584 in both kotlinx-coroutines) | a property from the class file: `@JvmField internal var label: Int = 0`, `` @JvmField internal var `L$0`: Any? = null ``; `val` for a final field, the field's visibility (package-private `internal`), `@JvmField` unless private, the JVM default as the initializer a `var` needs; static fields keep Vineflower's form |
| `kt_constructor_without_metadata` | in Kotlin output, writes a constructor the Kotlin metadata does not list (those of classes without Class metadata and of anonymous objects, whose metadata lists none, written as named classes: `$$inlined$` copies, objects created in another method) as a function, ``fun `Cls`(x: Int) { super(x); .. }``, and an object expression's constructor statements as Java's bare initializer block `{ .. }`, both rejected by kotlinc (261 constructors and 18 blocks in okhttp, kotlin-stdlib and both kotlinx-coroutines) | `constructor(x: Int) : super(x) { .. }` (the superclass or `this` call that is the body's first statement as the delegation, none for a hidden empty `super()`; the constructor's visibility), and `init { .. }` |
| `kt_delegation_call_arguments` | in Kotlin output, writes a constructor whose delegation call is passed a value kotlinc null-checks after computing it (a Java call's platform-typed result) with that value in a stack variable before the call, `constructor(pattern: String)  { val var10001 = Pattern.compile(pattern); this(var10001) }`, or a primary constructor's call in an `init` block, which kotlinc rejects (kotlin-stdlib's `Regex`, okhttp's `Challenge`) | the value in the call, `constructor(pattern: String) : this(Pattern.compile(pattern))`, when that computes the same values in the same order; the checks are kotlinc's for the expression in the call |
| `kt_copied_equality` | in Kotlin output, writes a copy of an `==` resugared from `Intrinsics.areEqual` (an `equals` test) as `===`, an identity test: `KFunctionExprent.copy` rebuilds the copy through the constructor that makes Java's `==` `===`, so a default value, `fun f(a: String, b: Boolean = a == "x")`, came out `b: Boolean = a === "x"` (wrong output; no such copy in okio, okhttp, kotlin-stdlib or kotlinx-coroutines) | the copy keeps the function type of what it copies, `a == "x"` |
| `kt_binary_numeric_promotion` | in Kotlin output, writes arithmetic, comparisons and `==` on `Char`, `Byte` and `Short` (and `==` of different number types) as Java does, relying on its promotion to `Int`; Kotlin's `Char + Int` is a `Char`, `Char` compares only with `Char`, and `==` needs one type: `c + 4` (`sb.append(c + 1)` appends a character), `c < 128`, `b == 61`, `l != i` (wrong output; 104 hunks in okio, okhttp and kotlin-stdlib) | the conversion Java's promotion makes: `c.code + 4`, `c.code < 128`, `b.toInt() == 61`, `l != i.toLong()`, `-c.code`; `Char - Char` (an `Int` in both) stays |
| `kt_negated_equality` | in Kotlin output, writes `areEqual(a, b) ^ 1`, which kotlinc before 2.0 compiles `a != b` to, as `(a == b) xor true` (38 in okhttp) | `a != b` (`a !== b` for `(a === b) xor true`); the source's form, equivalent |
| `kt_operator_line_break` | in Kotlin output, breaks a long binary operation before its operator, where Kotlin ends the statement: `return a` / `+ b` returns `a` and evaluates `+b` apart, a line starting with `*` does not parse (wrong output; 25 statement-level breaks, 16 before `+` or `-` (okio, okhttp, kotlin-stdlib, kotlinx-coroutines), e.g. kotlin-stdlib's `this.byteBufferEndIndex = this.byteBufferEndIndex` / `+ this.base64.decodeIntoByteArray(..)`; 328 hunks in all) | breaks after the operator (`a +` / `b`); `&&` and `\|\|` keep theirs; whitespace only |
| `kt_unboxing_cast_type` | in Kotlin output, writes the cast whose value an unwritten unboxing call (`booleanValue()`, `charValue()`) unboxes to the Java box class, `predicate(e) as java.lang.Boolean`, `f() as Character`, which Kotlin does not unbox (kotlinc: condition type mismatch); under `!` it came out `!p(x) as java.lang.Boolean`, which Kotlin reads `(!p(x)) as ..` (544 casts in okhttp, kotlin-stdlib and kotlinx-coroutines 1.8.0 and 1.10.2, 177 of them under `!`) | `predicate(e) as Boolean`, `f() as Char`, `!(p(x) as Boolean)` (the parentheses are `kt_operator_precedence`'s: an unwritten unboxing call binds as what it is written as) |
| `twr_desugaring_shape` | re-sugars try-with-resources from any `catch (Throwable)` that closes the resource first, with no check that the code is javac's desugaring: hand-written code that logs a close failure instead of `addSuppressed`, returns without closing, does work between the initializer and the `try`, rethrows something else or closes a non-`new` resource without null checks is printed `try (R r = ..)`, which behaves differently (wrong output); and writes the resource as the field it was copied from, `try (this.f)`, which javac rejects for a non-final field (Elasticsearch's `AbstractFileWatchingService.processSettingsAndNotifyListeners`) | re-sugars only javac's exact desugaring (`TryWithResourcesProcessor` in spec 10.16) and writes `try (this.f)` only for a final field copied to javac's unnamed variable, `try (T v = this.f)` otherwise (also where the source declared `v`: Elasticsearch's `Store`, `InternalClusterInfoService`) |
| `twr_unresolved_resource` | re-sugars try-with-resources only when the resource's class is known to be `AutoCloseable`, so a resource whose class (or the supertype that makes it `AutoCloseable`) is outside the input stays desugared into `try`/`catch (Throwable)`/`addSuppressed` and the closes at every exit (1,210 methods of the Java benchmark: Ghidra's `Lock.Closeable`, Elasticsearch's `Releasable`, `XContentParser`, Lucene's `IndexInput`) | re-sugars it when the bytecode is javac's exact desugaring and kuna cannot resolve the class; a class whose whole hierarchy is known and not `AutoCloseable` never is |
| `twr_class_version` | looks for javac 11's try-with-resources desugaring only in class files of Java 11 and later, though javac 11+ emits it for every `--release` (Guava's Java 8 classes: `ByteSink.write`, 15 methods) | looks for it in older class files too, accepting only javac's exact shape |
| `compound_assignment_shape` | decides between `x op= e` and `x = x op e` for a field or array element by a heuristic (no side effects in `e`, a variable or constant array and index, a cast only to `e`'s own type), against the bytecode, which loads the receiver once and `dup`s it only for the compound form: `b += buffer.remaining()` comes out `this.b = this.b + buffer.remaining()`, `destMask[i] &= ~src[i]` as `destMask[i] = (byte)(destMask[i] & ~src[i])`, and `this.n = this.n + 1` as `this.n += 1`; and writes `x = e op x` as `x op= e`, which reads `x` before `e` (wrong output where `e` writes `x`: `a[0] = bump(a) + a[0]`) | a field or array element assignment is compound exactly when its receiver (array and index) is one load in the bytecode, through the implicit narrowing cast; `x = e op x` keeps its order; locals and static fields, the same either way, and Kotlin classes keep Vineflower's choice |
| `implicit_outer_instance` | writes the creation of an inner class whose outer instance is an enclosing class's instance as `Outer.this.new Inner()`, which javac compiles with an outer-instance null check the bytecode does not have (105 methods of the Java benchmark, Guava's `LocalCache$KeySet.iterator`; in a generic outer the qualified form breaks inference, Guava's `TreeBasedTable$TreeRow.headMap`) | `new Outer.Inner()` (as Vineflower writes it for the current class's `this`) when the outer instance is the one an unqualified creation passes and the null check is absent; a creation whose check Vineflower removed stays qualified |
| `vararg_array_length` | writes the array a varargs call passes last as its elements even when it is created by its length, which has none: `Arrays.asList(new T[n])` (a list of `n` nulls) comes out `Arrays.asList()` (wrong output; Ghidra's `AssemblyConstructStateGenerator.applyOperands`, 2 in Elasticsearch) | the array is spread only when it is an initializer or its length is the constant 0 (javac's code for no varargs); otherwise it is passed: `Arrays.asList(new T[n])` |
| `synthetic_null_check_shape` | deletes a statement `Objects.requireNonNull(x);` (or `x.getClass();`) as javac's null check of a qualified `new`'s outer instance when the next statement creates any nested class taking `x` (or a value of `x`'s type) first, or when a constant, `null` or a store follows it: the source's own check is lost (wrong output; Guava's `ImmutableSet.Builder.forceJdk`, Elasticsearch's `Iterables.flatten`, `TcpTransport.parse`, `MessageDigests.toHexCharArray`, 6 methods) | removes only a call whose instructions are javac's: a check of a `dup` of a value left on the stack (`x.new Inner()`, `x::m`, a pattern switch), or a check followed by a non-`null` constant of a value whose class can have an instance constant field (`x.CONST`); every other call is kept |
| `var_merge_declared_type` | merges two variables of one slot whose types its lattice finds comparable although the LocalVariableTable declares them as two locals of different types: `Box<String> box` of one `static {}` block and `Box<Integer> box` of the next become one `Box<String>`, and javac rejects the second's code (spring-boot's `LoggingApplicationListener` and both `ColorConverter`s, 44 methods lost with their classes); a `char c` and a later `int start` become one `char`, the `int` store narrowed (`c = (char)i;`, wrong output past 65535; Kotlin `start = i.toChar()`, kotlin-stdlib's `InstantKt.parseIso`, and an object local merged with an `Int` in kotlinx-coroutines' `DebugProbesImpl`) | they stay two variables, each with its declared type (`Box<Integer> boxx`, `int start`) |
| `compact_record_constructor` | takes a record's canonical constructor for a compact one when its last statement is `this.c = c`, then removes every field store: `this.node = Objects.requireNonNull(node)`, `this.slices = Map.copyOf(slices)`, `this.content = content.retain()` vanish and the record is printed with no constructor or a compact one (wrong output; 4 records in Elasticsearch, `JoinTask.NodeJoinTask`, `ResumeInfo`), and a compact constructor's static field stores go too | the constructor is compact only when its one exit that does not throw ends with every component's `this.c = c` in order (upstream 1506b2f4), and only stores to fields of `this` are removed |
| `disjoint_generic_cast` | casts a returned or assigned value, or a field's initializer, whose generic type differs from its target's to the target's parameterization even when the two are provably distinct (JLS 4.5), which javac rejects: `(ImmutableSet<Class<? super T>>) builder.build()` of an `ImmutableSet<Class<?>>`, `(Collector<E, ?, ImmutableList<E>>) TO_LIST`, `(Range<C>) ALL` of a `Range<Comparable>` (Guava's `TypeToken`, `CollectCollectors`, `Range`, `MoreCollectors`; Elasticsearch's `InternalOrder`) | the cast is to the raw class, `(ImmutableSet) builder.build()`, as the source wrote it; arguments keep their cast (a raw argument would erase the call's result type) |
| `thrown_type_variable` | writes a method's `throw (T) t` (an unchecked cast to the thrown type variable, no instruction) as `throw t`, and a call of such a method on `new C<Error>()` on a raw `new C()`, whose erased `throws` clause names `Throwable`: javac reports an unreported exception (Guava's four `SneakyThrows`, Elasticsearch's `ScriptCache.rethrow`, `TenaciousRetryBlobContainer.execute`) | `throw (T)t` where a variable of a checked type no declared class covers is thrown and a thrown type variable admits it; the raw receiver is given `RuntimeException` for the type parameters the call throws (what JLS 18.4 infers for a thrown variable); Kotlin output is left alone |
| `kt_java_keyword_variable_name` | in Kotlin output, refuses a debug variable name that is a Java keyword but an ordinary Kotlin name (`new`, `default`, `char`, `int`, `byte`), so the variable keeps its generated name while the header that declares it prints the metadata's: `fun readBomAsCharset(default: Charset)` reads `var1`, `{ old, new -> old == var2 }` (wrong output: the body reads variables nothing declares, in 15 methods of okio, okhttp, kotlin-stdlib and both kotlinx-coroutines) | the debug name, as for any other name (`this` stays refused) |
| `kt_default_accessor_flags` | reads a property accessor's flags from the metadata unconditionally, so a setter whose flags kotlinc leaves out (they equal its property's) is `internal`, `final`: a public `var` is printed `internal set`, a protected one `internal set` (kotlinc rejects it), an `open var` `internal final set` (rejected) (wrong output: 80 setters in the five Kotlin inputs, and every default setter kotlinc 2.0 compiles) | an absent accessor's flags are its property's default accessor flags, as Kotlin's reader takes them; nothing is printed for a default setter |
| `kt_unescaped_name` | in Kotlin output, the writers shared with Java print names bare that Kotlin writes in backticks: a captured variable in an object's method (`$this$asList.length`), a lambda header's parameter (``{ in -> `in` }``, kotlinc's `<anonymous parameter 0>`), a field (`System.in`, `this.L$0`), a called method (`access$getFoo$p()`), which kotlinc rejects (3,857 hunks in the five Kotlin inputs, most of them kotlinx-coroutines' atomicfu and continuation names) | each goes through `toValidKotlinIdentifier` (`this` and `this@x` excepted); an unnamed lambda parameter is `_`. A class's binary name (`Foo$Bar`, a `$$inlined$` class) stays: Kotlin cannot name it |
| `kt_reserved_underscore_name` | in Kotlin output, writes a variable or parameter named `_` or `__` bare, which Kotlin reserves (`for (_ in this.readyQueues)`, `val _: Class`) | the name in backticks, which kotlinc 2.0 accepts there |
| `kt_java_type_syntax` | in Kotlin output, writes types read from Java generic signatures in Java's syntax: type parameters `<T extends Comparable<? super T>>`, supertypes `object : Function0<T[]>`, `KClass<Object>`, nested arguments `Continuation<? super java.lang.Boolean>`, array class literals `int[]::class.java`, and drops the wildcard of a translated collection type (`MutableList<R>` for `List<? super R>`) (164 hunks in the five Kotlin inputs) | Kotlin's syntax: `<T : java.lang.Comparable<in T>>`, a `where` clause for a parameter with two bounds, `out`/`in`/`*` projections, `Array<T>`, `IntArray::class.java`, `Any` for `Object` in an argument |
| `field_read_all_paths` | Kotlin output reads a field again where a local held its value from before a call: the inliner that removes every local (`INLINE_ALL_VARS`) moves a field read to its use while the field is "live" there, and liveness is merged by union, so a read survives a join with a path that called (an `if`, a `c && f()`), the entry of a catch or finally body, and a finally body that calls (`val v = h.count; try { work() } finally { use(v) }` prints `use(h.count)`; kotlin-stdlib's `TestTimeSource.plusAssign` restores `this.reading = this.reading` in its catch; kotlinx-coroutines' `emitAllImpl` reads `$continuation.Z$0` for `consume`) | the local stays: in that pass a field read is live only while it is live on every path, and a finally body that may change fields ends it |
| `kt_inline_evaluation_order` | Kotlin output evaluates a local's value later than the bytecode does: the inliner places a local's value at its use in the next statement wherever that use is, which is right for a stack value but not for a local, so a call moves past other calls and field reads (`val first = src.next(); src.take(src.next(), first)` prints `src.take(src.next(), src.next())`; okhttp's `Cache.Entry` reads the TLS version line before the cipher suite line), into a branch (kotlinx-coroutines' `WorkQueue` decrements only `if (ASSERTIONS_ENABLED && ...decrementAndGet() < 0)`), or, when the use is in a later statement, into a `finally` (`try { block() } finally { restoreThreadContext(ctx, updateThreadContext(ctx, x)) }` in `withCoroutineContext`) | a value that calls, allocates or stores is inlined only into the next statement, where it is evaluated first and on every evaluation of it; one that reads an array element, ahead of any call or store there; values that read only locals and fields move as before |
| `qualified_super_call` | writes a constructor's superclass constructor call `o.super(3)`, where the superclass is an inner member class and `o` an explicit outer instance, as `super(3)`: the call passes the enclosing instance instead of `o` (wrong output) or does not compile, and javac's null check of `o` stays as `Objects.requireNonNull(o);` before it; an anonymous class created as `o.new Inner(3) {..}` is written `new Outer.Inner(o, 3) {..}` with `Objects.requireNonNull(x0);` in an initializer; a superclass outside the input gets its outer instance as the first argument, `super(Outer.this, ctx)`, which does not compile (Elasticsearch's `HalfFloatComparator`, 15 methods lost with the class) | `o.super(3)` when the outer instance is not the instance of the innermost enclosing class having the superclass as a member, or javac checked it; `o.new Inner(3) {..}` when the anonymous class's constructor passes its superclass's constructor a parameter of its own as the outer instance; javac's checks removed; for a superclass outside the input that the InnerClasses attribute names an inner member class, the outer instance leaves the argument list (`super(ctx)`, or the qualifier) |
| `external_inherited_member_type` | learns the member classes a class inherits only from supertypes in the input, so a member class of a supertype outside it neither shadows an import of its simple name nor makes it ambiguous: Elasticsearch's `InternalFilters` (implementing `Filters`, extending a class that implements `MultiBucketsAggregation`, both outside the input and both declaring `Bucket`) writes `implements Bucket` under an import of `Filters.Bucket`, which javac finds ambiguous (28 methods lost with the class), and `NodesHotThreadsResponse` imports `java.io.Reader` where the inherited `Writeable.Reader` is in scope | the member classes of supertypes outside the input are read from the InnerClasses entries of the classes the supertype walk visits; a simple name two inherited member classes share, or an inherited one shadows, is written qualified; inaccessible members (private, package-private elsewhere) do not count; Kotlin output is left alone |
| `lambda_return_type_argument` | writes explicit type arguments on a qualifier call from the value its variable got where the chain's result goes, contradicting a lambda or method reference argument: `ext.map(Ext::names).orElse(Collections.emptyList())`, with `Ext` outside the input, becomes `ext.<List<String>>map(Ext::names)`, and javac rejects `Ext.names`'s `Collection` (Elasticsearch's `TransportVersion`, 29 methods lost with the class) | type arguments that the implementation's erased return type contradicts (no subclass of the argument's class) are not written; javac infers them from the lambda or reference, as the source did |
| `type_annotation_outer_arguments` | writes an annotated class type one class name at a time with only the innermost class's type arguments: an inner class of a generic class, `CombinedFuture<V>.@Nullable CombinedFutureInterruptibleTask<?>`, becomes `CombinedFuture.@Nullable CombinedFutureInterruptibleTask<?>`, type arguments on a member of a raw type, which javac rejects (Guava's `CombinedFuture`, 19 methods lost with the class; `LocalCache`, `MapMakerInternalMap`, `AbstractMapBasedMultimap` get raw types) | each enclosing class keeps the type arguments the generic type gives it, `CombinedFuture<V>.@Nullable ...` |

Every fix is a `KunaFix` variant in `src/p0_knowledge/kuna_fixes.rs`, whose
`KunaFix::id` is the id above; each site where kuna departs from the Java asks
`kuna_fixes::enabled(KunaFix::..)` and keeps Vineflower's code verbatim in the other
branch, which compat mode takes. A unit test that must isolate one fix runs with
`KunaFixes::DEFAULT.without(KunaFix::X)` (through `Fernflower::new_with_fixes`, or a corpus
`Case`'s `fixes`); users cannot. Spec section 10.16 describes each defect and what kuna
does instead, and `tests/jvm/kuna-fixes/README.md` shows each fix's diff on its case.

### Options of form

Some departures from Vineflower's text are not corrections but choices between two
equivalent forms of the same code: Vineflower's, and the one the class file shows the source
had. They are options (`change_kind = "kuna-form"` rows of `options.toml`), not fixes:
`Fernflower::new` takes each off the property list into the decompilation context (its
value, else its row's default; `kuna_options::form_enabled` answers from it) and forces it
off under `jvm_vineflower_compat on`, so compat output stays Vineflower's; a case of one in
`tests/jvm/kuna-fixes/` is labelled with the option's name.

| Option (default) | Vineflower 1.12.0 | With the option |
|---|---|---|
| `jvm_source_locals` (on) | folds a local the LocalVariableTable names into its one use (`result = 31 * result + x; return result;` as `return 31 * result + x;`, `T[] a = new T[2]; a[0] = ..` as `new T[]{..}`, a named iterable or `Iterator` into a for-each) and declares each variable at its first use, so javac renumbers the slots (`int nRead; long total = 0;` as `long total = 0L; int nRead;`) | the locals stay as the source wrote them, and in Java output declarations move up (or split into `T v;` and `v = e;`) until javac gives every local the class file's slot. On the five benchmark projects: 98,306 methods perfect on every metric instead of 97,634, four more classes recompile |

### Fixing a Vineflower bug

The recipe, the same for every fix (the first, `lambda_wildcard_return`, is the worked
example):

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
3. **Add the fix.** A variant in `KunaFix`, added to `KunaFix::ALL`, and its id in
   `KunaFix::id` (`KunaFix::X => "<id>"`, lower snake case, no `jvm_` prefix). There is no
   option row: a fix is not a user option, and `jvm_vineflower_compat` already turns it
   off with the others. At the site: `if kuna_fixes::enabled(KunaFix::X) { ... } else {
   <the Java, unchanged> }`, and a line in the ported file's header naming the fix by its
   id. A fix longer than a few lines goes in a `kuna_fix_<id>.rs` module next to the
   ported file, called from the gated branch. Code that runs before `Fernflower::new`
   installs the context reads the set it is building (`fixes.contains(..)`), as
   `report_to_kuna` does.
4. **Add the case.** Compile the reproducer and run `python3 -I
   tests/jvm/kuna_fix_case.py --fix <id> --class DIR/pkg/TestFoo.class --source
   pkg/TestFoo.java --compiler "javac 17.0.20.1" --summary "<what decompiles now>"`. It
   vendors the class and its `TestFoo$*.class` siblings into `tests/jvm/kuna-fixes/`,
   stores what the release jar prints (the compat expectation, which must contain the
   failure comment), checks that kuna in compat mode prints exactly that, stores kuna's
   default output, prints the diff for review, rebuilds `cases.tsv` and the READMEs with
   `regen.py`, and adds both runs to `tests/jvm/baseline.txt`.
5. **Describe it.** A paragraph in `docs/spec/10-jvm.md` section 10.16 headed by the
   fix's id (the defect in Vineflower's algorithm, what kuna does instead) and a row in
   the table above.
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

Fixes made in parallel meet in the same files. Keep both sides of `KunaFix` (the enum,
`ALL` and `id`) and of `tests/jvm/kuna-fixes/cases.json`, then rerun `regen.py` (it rewrites
`cases.tsv` and both READMEs, and refuses a case whose `fix` is not a `KunaFix` id);
`baseline.txt` is sorted, so keep both sides' lines.

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
body of an inline function (`let`, `forEach`) appears expanded where it was called. A call
that leaves arguments to their defaults is written as the call the source makes, not as the
`$default` bridge it compiles to, and a cast is Kotlin's `as` or conversion call
(`b.toLong()`), not Java's `(long)b` (fixes `kt_default_args_call`, `kt_java_cast_syntax`
and `kt_java_reference_cast`). A Java type whose simple name Kotlin reuses is written
qualified (`java.lang.String`). Apart from kuna's fixes the text is Vineflower's, quirks
included.

**The CLI on Kotlin units.** Classes and methods are still named by their class-file
names, which is what `kuna functions` lists, so a selection works the same for Java and
Kotlin input (`docs/cli.md`, "JVM inputs", has the details):

```
$ kuna decompile tests/fixtures/jvm/kotlin/classes com.ex.kt.Counter.getLabel
public final var label: String
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
`@JvmStatic` forwarder, a constructor the compiler generates for default arguments: the
no-argument one and each `@JvmOverloads` overload) has no text of its own, and naming it is
an error that says so.
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

**Known limits.** The Kotlin output is Vineflower's, so it has Vineflower's gaps, apart
from those kuna's fixes close (*Kuna-only fixes*, above). The plugin rewrites syntax, not
semantics, so whatever the compiler lowered stays lowered (above). A `suspend` function
that suspends is written as the coroutine state machine the compiler made of it, not as a
`suspend` body: its continuation appears as `object : ContinuationImpl($completion) { ... }`
where the function creates it, and the type Kotlin cannot name shows as `<unrepresentable>`
(Vineflower fails on such a function; one that never suspends is written with `suspend`).
The metadata schema is the one the plugin was built with (Kotlin 2.2), applied without a version
check. When a method cannot be decompiled, its Kotlin declaration is written with the failure
comment in place of the body, exactly as for Java, and the CLI reports it on stderr and in
the record's `error`; `--language java` usually still gets the Java view. The trace line
under the exception is kuna's own, as for Java (item 1 of the fidelity contract). A whole class the plugin cannot write is a `/* $VF: Unable to decompile class
*/` comment with the exception, which the CLI reports as a failed class (*How failures are
reported*, below). A class whose metadata cannot be decoded is still written as
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

## How failures are reported

The engine never stops on a method or class it cannot decompile: it writes the failure as
a comment where the code would be. The comment's place decides what failed, and the class
writers record each one as they write it (`src/infra/kuna_spans.rs`), changing no text:

| The engine writes | Recorded as | The CLI reports |
|---|---|---|
| `// $VF: Couldn't be decompiled` as a method's body | the method's `error` | that method failed |
| the same comment as the body of a lambda, of a method of an anonymous or local class, or `// $VF: Couldn't be decompiled` for such a class, written inside method M | a `BodyFailure` on M's span (and the anonymous class's method keeps its own `error`) | M failed too, naming the body (`nested_errors`); one `warning:` per body |
| any of these outside every method (a field initializer's lambda, a member class) | a `BodyFailure` on the class's span | the class's `nested_errors`; `decompile-project` counts them as `outside methods` |
| `/* $VF: Unable to decompile class */` in place of a whole unit (the Java and Kotlin writers' last resort) | the unit's `error` | a failed class (`status: "failed"`) on every command, with that reason |
| a `$VF:` comment above a method that the engine flags as an error (`MethodWrapper::add_error_comment`, the comments that carry the report-this note) | the method span's `warnings` | a `warning:`, not a failure |
| any other `$VF:` comment, or a line only logged | nothing | nothing |

The flagged comments mark text that is complete but may be wrong (`Could not resugar all
assert statements!`, `Unable to simplify switch on enum`, `Could not properly define all
variable types!`, variable-merging and finally-processing failures, ...). The unflagged
ones describe a transformation that worked (`Irreducible bytecode was duplicated to
produce valid code`, `Removed empty exception range`, `Added assignment to ensure
synchronized validity`). The Kotlin writer does not print method comments at all, so a
Kotlin method's flagged comments show only as CLI warnings. A lambda's own flagged comments
are printed by neither writer and are not reported.

The engine's log (the logger the CLI installs is silent) has lines with no comment in the
output: `Statement cannot be decomposed although reducible!` is a dead end of DomHelper's
search that its caller recovers from, unless the method then fails with `parsing
failure!`, which is reported as above; `Inconsistent inner class entries` says which of
two conflicting `InnerClasses` records the engine kept (`jvm_warn_inconsistent_inner_attributes`);
`Inconsistent anonymous class definition` and `Inconsistent references to the class`
mean the engine writes that class as a top-level unit of its own instead of inside the
method that creates it, which the CLI follows (it asks the engine for every class of a
target's outer chain and takes the nearest one written as a unit). None of them is
reported, and the JVM path has no `--verbose` to show them.
`docs/cli.md` (*Failures and warnings*) has the record shapes and the summary line.

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
engine polls it at the heads of its fixed-point loops (spec section 10.12) and counts the
polls: a method may make 200,000 of them per second of the budget, the rate the slowest
methods measured (Ghidra's ANTLR-generated parsers poll 180,000 to 250,000 times a
second), so which methods run out does not depend on the machine or its load. A
wall-clock deadline at `2N` seconds backs the count up for a method that polls rarely. A
method that runs out is written as `// $VF: Couldn't be decompiled` with a
`TimeoutException`, exactly as Vineflower writes a method its watchdog killed, and the
rest of the class is written normally. With the default flags Ghidra's full
`classes.jar` finishes in under five minutes on 24 workers with three methods of its
ANTLR-generated parsers stopped (`SleighParser.tokendef`, `SleighParser_SemanticParser`'s
`sizedstar` and `expr_func`), the same three at any worker count; under the wall-clock
limit it was two on an idle machine and four on a busy one (spec section 10.12). A
method stuck between two polls is not interrupted; as a last resort the CLI stops waiting
after `2N+30` seconds, writes the classes that finished, gives the stuck ones a stub
comment and exits. `jvmdec --opt mpm=N` is `--opt max-time-per-method=N`, as for any
three-letter name.

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

**Hard failures.** A method or class that throws is contained (spec section 10.12), and
so is recursion too deep for the stack: input nested thousands deep (expressions, `else
if` chains, lambdas) or nesting without end (lambdas whose bodies create each other under
`jvm_vineflower_compat`) fails the method or class it is in with a `StackOverflowError`,
as on Vineflower's JVM, and the run goes on (`src/rt/stack.rs`). The engine threads have
512 MiB stacks of which the guard lets the engine use 32 MiB; the deepest real input
measured (Elasticsearch, Guava, Ghidra, spring, okhttp) uses 150 KiB, and the limit
nests a sum about four times deeper than Vineflower manages on its 1 MiB thread stack.
`KUNA_JVM_STACK_MIB=<n>` gives the engine threads an `n` MiB stack instead (a developer
knob for exercising the guard). Running out of memory still ends the process: deeply
nested expressions cost memory quadratic in their depth (a 2,400-term sum takes 3 GB),
and the per-method memory limit (`--max-fn-memory`, `docs/cli.md`) is the bound on that.

**Memory guard.** Under a capped cgroup (or with `--max-fn-memory`) each method is armed
against the per-function memory limit (`docs/cli.md`). On JVM inputs a method starts
without first handing free heap pages back (`kuna_fnmemory::set_release_before_drives`):
the engine threads share one heap, and trimming it at method starts made 674,000
`madvise` calls on Elasticsearch under a 15 GB cap (49 without; median system time 11.6 s
against 9.1 s over five runs). A method that trips the limit still releases on the way
out.

## Testing

| Command | Checks |
|---|---|
| `make test-jvm` (= `kuna test --jvm --baseline tests/jvm/baseline.txt`) | the gate: every case in the baseline still prints its expected text: the 607 Vineflower-oracle cases (Java, plugin and Kotlin, run in compat mode) Vineflower's exact text, and each of the 152 kuna-fixes cases Vineflower's text in compat mode and kuna's reviewed text by default (911 runs in all). Needs no SLEIGH specs, no JDK and no Kotlin compiler. |
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

## javac's block layout

`jvm_javac_layout` (on by default, off under `jvm_vineflower_compat`) is a judgement call,
so an option: on classes javac compiled (no `kotlin.Metadata`) it structures ifs and exits
the way javac laid the bytecode out, where Vineflower prints an equivalent but different
form. Two return instructions stay two returns instead of one return of a ternary (`if (c)
return true; return false;` is not folded to `return c;`); an `if` keeps javac's
then-branch, the code right after the condition, so `if (o == null || getClass() !=
o.getClass()) return false;` keeps its polarity instead of becoming `} else if (o != null
&& ..) { .. } else { return false; }`; and an early `return;`, `throw`, `break` or
`continue` the bytecode has stays, instead of the rest of the method or loop body being
nested in a negated `if`. Spec section 10.9 has the rules. On the Java benchmark
(`docs/jvm-benchmark.md`) it makes the decompiled code recompile to the original bytecode
far more often: methods perfect on every metric go from 97,634 to 105,078 of 110,238
(88.57% to 95.32%; bytes exact 88.65% to 95.41%, structure perfect 96.06% to 98.53%), with 86
methods worse on some metric, 53 of them try-with-resources that stays desugared (lever 4
of the triage) where the restored early exit costs graph-edit distance while the bytecode
gets closer. Where the bytecode is the same for both forms (an if-else of two returns, or
a `continue` that ends a loop body), Vineflower's form mostly stays. The two-pass case `tests/jvm/kuna-fixes/` `pkg/TestJavacLayout` shows each
shape, and `tests/jvm/kuna_fix_diff.py` recognises its hunks as `jvm_javac_layout`.

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

**A kuna improvement is a named option.** A judgement call that changes the emitted Java or
Kotlin beyond what Vineflower prints ships behind a `jvm_*` row of its own: `vineflower = ""`,
`short = ""`, `aliases = []`, `source_decompiler = "kuna"`, a `change_kind` of its own, and
every other field filled in. Its code goes in a `kuna_<slug>.rs` module in the owning phase
folder, with a minimal seam in the ported code; a kuna row resolves to its own name, so it
cannot collide with a Vineflower key. Describe it in `docs/spec/10-jvm.md` (tagged
`(kuna)`, checked by `make check-spec`, whose strict mode requires every row to be
mentioned). A destructive row also updates the pinned destructive list in
`tests/kuna_options.rs`. A strict bug fix that makes kuna match Vineflower needs no
option. Neither does a fix of a defect Vineflower itself has: it is a `KunaFix` with an id,
always on and off only in compat mode, and follows the recipe of *Fixing a Vineflower bug*;
`tests/kuna_fixes.rs` checks that no `jvm_fix_*` row exists and that such a name is an
unknown option. An option is for a judgement call (a readability trade, a heuristic), the
same rule as the native engine's (`docs/agents.md`).

**Default-on needs corpus evidence.** The corpus gate compares against Vineflower's text,
so a kuna improvement that changes any baseline case stays off by default. It may default
on only when `make test-jvm` still matches every baseline case with it on, and when it
passes the speed budget (the same rule as the native engine's: a default-on feature must
not majorly slow kuna down). Fixes are the exception by construction: the
Vineflower-oracle cases run in compat mode, where every fix is off, and a fix's evidence is
its two-pass case and the whole-output diffs of step 6 of the recipe. So is an option that
compat mode turns off with the fixes, such as `jvm_javac_layout`: it leaves every
Vineflower-oracle case alone, changes only kuna-fixes cases' default text (re-recorded after
reading their diffs), and defaults on on the evidence of a full benchmark run (every
method that got worse read and explained) and the same whole-output diffs, recognised hunk
by hunk.
its two-pass case and the whole-output diffs of step 6 of the recipe. So are the options of
form (*Options of form*, above): compat mode turns them off, and one defaults on only when a
full benchmark run of the five projects shows it better (methods perfect on every metric,
classes that recompile, each worse method read) at the same speed; the default-mode texts of
the kuna-fixes cases it changes are re-recorded after review.

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
