# The JVM engine

`kuna-jvm` (`decompiler/nonnative/kuna-jvm`) decompiles JVM class files, jars and
directories of classes to Java, and the classes the Kotlin compiler emitted to Kotlin. It
is a Rust port of [Vineflower](https://github.com/Vineflower/vineflower) 1.12.0 with the
Kotlin and IDEA not-null plugins Vineflower bundles. On the same
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
| `tests/jvm/kuna-fixes/README.md` | the two-pass cases of kuna's own fixes |
| `decompiler/nonnative/kuna-jvm/tools/oracle/README.md` | the Vineflower oracle the corpus is cut from: what it downloads, how to rebuild it |
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
jar with a launcher stub in front; spec chapter 00, section 0.2). They share everything a
user or an agent touches: the CLI and its subcommands, the option catalog (`kuna catalog`,
`--option NAME VALUE`), the text and JSON output shapes, the phase taxonomy the source
folders are named after (`docs/phases.md`) and the live spec (`docs/spec/10-jvm.md`, held
to the same `make check-spec` gate as the native chapters). Below the CLI they share
nothing.

**Why a separate engine.** Lowering JVM bytecode to p-code would throw away most of what
makes Java recoverable:

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
over the operand stack, not Java. And the engine has an exact correctness oracle,
Vineflower's text, which only a faithful port of Vineflower's own IR can match byte for
byte.

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
| `src/infra/` | the per-method schedule (`method_processor`), the plugin and language-plugin API, and kuna's parallel driver, spans, plugin bundle (`kuna_plugins`) and corpus runner |
| `src/plugins/` | Vineflower's VariableRenaming and IdeaNotNull plugins |
| `src/kotlin/` | Vineflower's Kotlin plugin: the metadata decoder, its pass schedule, the Kotlin expression and statement layer, the Kotlin writer (below, "Kotlin") |
| `src/fixes/` | kuna's fixes of Vineflower's defects, one module per family of `fixes.toml` (a folder for a large family), called from the gated sites of the ported files (below, "Kuna-only fixes") |
| `options.toml` | the `jvm_*` option rows; `build.rs` generates the static table from it |
| `tests/` | the unit and differential suite, one `integration` test binary (`tests/integration.rs` registers every file) |
| `tools/oracle/` | the Vineflower oracle: rebuilds the Java and Kotlin corpora the regression corpus comes from |

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
  `PPandMMHelper` is `p_pand_mm_helper`). Every ported module is re-exported flat from the
  crate root (`kuna_jvm::if_helper`), so code never names a phase folder; a fix is named by
  its family (`crate::fixes::switches`, `crate::fixes::generic_casts::receiver_type_arguments`). Types keep their Java names,
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
  `kuna_options`, `kuna_fixes`, `kuna_driver`, `kuna_engine`, `kuna_partition`, `kuna_plugins`, `kuna_spans`,
  `kuna_corpus`, `kuna_reclaim`), in `src/fixes/`, in the modules that replace JVM
  facilities (`context_source`, `jdk_runtime`, `zip_reader`, `byte_cursor`), and in the
  Kotlin plugin's seams (`kt_metadata`, `kt_attrs`, `k_dispatch`). Changes inside ported
  files are kept to the seams those modules need (span marks in the writer, a deadline
  poll in the cancellation manager, partial runs in `fernflower.rs`) and to the gated
  sites of kuna's fixes (*Kuna-only fixes*), noted in the file headers.
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
  `testData` sources and registrations (core's and its plugins') and pins, by digest, the
  output of the released `vineflower-1.12.0.jar` (`tests/jvm/README.md`).
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
| `main/decompiler` | 9 | 1 | the console decompiler, its help, option parser and savers, by the `kuna` CLI and its output; `PrintStreamLogger` is not ported |
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
  `java -jar vineflower.jar` loads; Kotlin and IDEA not-null are ported, one module per
  Java file, and kuna carries them in the same registration order
  (`src/infra/kuna_plugins.rs`); variable renaming is not (below):

| Plugin (`plugins/` in Vineflower's repository) | Java files | Ported to | What it does |
|---|---:|---|---|
| `variable-renaming` | 5 | not ported | renames locals (and parameters) JAD-style or tiny-style on request |
| `kotlin` | 51 | `src/kotlin/` (50 modules; the marker interface `KExprent` is a flag on the expression) | detects Kotlin classes and decompiles and writes them as Kotlin |
| `idea-not-null` | 3 | `src/plugins/` (3 modules) | removes the null checks IntelliJ's compiler adds for `@NotNull` |

  The Kotlin plugin also ships a prebuilt jar (`plugins/kotlin/libs/metadata.jar`) with a
  shaded copy of the Kotlin compiler's metadata schema and the protobuf-lite runtime it
  runs on; kuna ports the parts it reads as `src/kotlin/kt_proto.rs` and
  `src/kotlin/kt_flags.rs`. Plugins are not discovered on a class path: the bundle always
  loads, as with `java -jar`, and each plugin acts only through its options
  (`jvm_kt_enable`, `jvm_resugar_idea_notnull`); the corpus runner
  alone runs the bare Java pipeline of Vineflower's test fixture (`jvm_plugins=0`).
- **Not ported:** a user renamer class and Vineflower's `include-classpath` need a JVM and
  are refused (`options.toml`, `[[driver_owned]]`); third-party plugin jars cannot be
  loaded. Vineflower has no Scala or Groovy support, so classes compiled from those
  languages are written as Java.

## The fidelity contract

**Two modes.** One switch, `jvm_vineflower_compat`, selects them:

- **Compat mode** (`--option jvm_vineflower_compat on`): every kuna-only fix is off, and on
  the same class files and options kuna-jvm prints exactly what Vineflower 1.12.0 prints,
  failures included, apart from the deliberate differences listed below. Everything this
  section claims about matching Vineflower is measured in this mode, and the regression
  corpus's Vineflower-oracle cases run in it.
- **Default mode** (the switch off): the same text, except where a kuna fix (*Kuna-only
  fixes*, below) corrects what Vineflower does wrong: a method Vineflower fails on is
  decompiled instead of written as its `// $VF: Couldn't be decompiled` comment (with any
  imports its body needs), code Vineflower prints wrong is printed as the program has it,
  and the note under a failure comment that remains asks for reports to kuna's tracker.
  Nothing else may differ; that is the evidence each fix ships with (*Fixing a Vineflower
  bug*, below). kuna's source forms and javac's block layout (*Source forms* and *javac's block
  layout*, below) change the default text further. `docs/jvm-benchmark.md` measures both modes against
  Vineflower on five large projects.

**The test oracle is Vineflower's text.** Every Vineflower-oracle case of the vendored
corpus matches in compat mode (core's Java tests, the variable-renaming and IDEA not-null
plugins' tests, and the Kotlin plugin's and core's Kotlin tests: `tests/jvm/README.md` has
the counts). So do the Java cases that need a particular
JDK on the machine, run from the external oracle corpus, and the Kotlin
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
   `report_to_kuna`.) With `dump-exception-on-error=0`, as the corpus fixture sets it,
   neither decompiler writes the exception. Run in compat mode with the command-line defaults of both
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

A fix corrects a method Vineflower 1.12.0 fails on (the port fails it too, writing it as
`// $VF: Couldn't be decompiled`) or code Vineflower prints wrong. A fix is a correction, not
a judgement call, so it has no option of its own: every fix is always on. There are 250
fixes, 143 of them for Kotlin output. This table is the description of each; its id names
the fix in the spec:

| Fix id | Vineflower 1.12.0 defect | What kuna does |
|---|---|---|
| `report_to_kuna` | the note under every error comment asks readers to report the class to Vineflower's tracker | it asks for reports to kuna's tracker (the corpus fixture's empty `error-message` still wins) |
| `lambda_wildcard_return` | fails a method that passes `x::m` on a receiver typed `Foo<?>` to a generic call, when `m` returns `Foo`'s type variable (`NullPointerException: returnType`; spring-boot's `StructuredLoggingJsonProperties.customizers`) | the method decompiles; the lambda keeps the type its call's context gives it |
| `twr_unary_condition` | fails a method of a Java 11+ class whose `catch (Throwable)` handler starts with `if (!flag)` on a boolean local, taking the condition for a try-with-resources null check (`index out of bounds`; spring-framework's `ConfigurationClassEnhancer.createClass`) | the method decompiles; the `try`/`catch` is written as it is |
| `if_denest_fallthrough` | turns `if (a) { ...; try { ... } ... } Z` into `if (!a) { Z } else { ... }` although the `try` falls through to `Z`, printing a `break` past `Z` or failing the method (`IllegalStateException: Could not find destination nodes for stat id`; Elasticsearch's `InternalEngine.closeNoLock`) | the `if` keeps its shape when its body reaches the code after it, and the method decompiles |
| `switch_yield_const_retype` | fails a method whose switch expression yields a string or class literal next to an arm of an unrelated reference type, such as a boxed `char` (`RuntimeException: invalid constant type: Ljava/io/Serializable;`; spring-framework's `BasicJsonWriter.escape`) | the method decompiles; the literal keeps its own type, which the switch's context widens |
| `catch_loop_back_successor` | fails a `while (true)` loop after other code whose body is a `try` with two or more `catch` blocks that all go back to the loop (`RuntimeException: computing post reverse post order failed!`; Elasticsearch's `RetryingInputStream.read`) | the method decompiles; the loop is isolated before its `try`/`catch` is built, as for a single `catch` |
| `kt_synthetic_anonymous_object` | in Kotlin output, fails a method that creates an anonymous class with only synthetic-class metadata, such as a `Comparator { .. }` in an inline function with a `crossinline` parameter (`IllegalStateException: Anonymous class does not have Class Kotlin metadata`; kotlin-stdlib's `compareBy`) | the method decompiles, the class written as an `object` expression from the class file; a continuation or local function reference (superclass not `Object`) still fails |
| `kt_local_as_extension_receiver` | in Kotlin output, takes a method without parameters and metadata whose first local is named `$this$..` (a captured extension receiver, or an inlined lambda's receiver) for an extension function and fails (`ArrayIndexOutOfBoundsException`; the `remove()` kotlinc adds to the `Iterator` of kotlin-stdlib's `Enumeration<T>.iterator()`) | the method or class decompiles; such a method is written as a plain method |
| `kt_default_args_super_call` | fails a whole Kotlin class whose primary constructor calls the superclass's default-arguments constructor when the superclass is decompiled in the same run (`RuntimeException: Constructor X.<init>(...I)V not found`), and otherwise prints the placeholders and bit mask, `: Base(a, null, 2)` | the call is the constructor it stands for, `: Base(a)`, when only trailing arguments are defaulted and no other constructor could take the rest, otherwise with the passed arguments named (`: Base(a, c = 9L)`) when the names select that constructor alone |
| `kt_default_arg_compound_branch` | fails a whole Kotlin class with a default argument computed by several statements, such as an inlined `map` (`NullPointerException`; kotlin-reflect's `AnnotationConstructorCaller`) | the class decompiles; that default is printed as unknown, `= ...`, like any default the plugin cannot read back |
| `generic_parent_without_signature` | fails a method that uses a value whose type is nested in a non-generic inner class of a generic class (`Outer<E>.Itrs.Node`), reading the missing `Signature` of `Itrs` (`NullPointerException`; the JDK's `ArrayBlockingQueue$Itrs`) | the method decompiles; `Outer`'s type arguments are mapped through `Itrs` as through any enclosing class |
| `self_referencing_lambda` | fails a whole class with a lambda whose synthetic implementation method creates the same lambda again (`Lambda X is recursive?!`, or a `StackOverflowError`; Vineflower's test `TestRecursiveLambda`, obfuscated bytecode) | the class decompiles; those lambdas are written as method references to their method (`this::body`), which stays a method |
| `external_class_diamond` | writes `new C(..)` raw for a generic class `C` that is not in the input (a library), even where the field, local, return or parameter is typed `C<A, B>`, so a lambda argument loses its parameter types and the class does not recompile (wrong output; Elasticsearch's `PARSER = new ConstructingObjectParser(..)`) | `new C<>(..)` wherever the expected type is a parameterization of `C` itself |
| `statement_as_expression` | prints an `assert` where Java requires an expression, which javac rejects: as the body of an expression lambda, `(k, v) -> assert v.equals(k) : k`, or as a `for` loop's update (elasticsearch's `Metadata$Builder`) | the lambda gets a block body, `(k, v) -> { assert v.equals(k) : k; }`, and the loop stays a `while` loop with the `assert` at the end of its body |
| `enum_body_separator` | prints an enum with members but no constants without the `;` before the members (`public enum Util { public static ...`, javac: `enum constant expected`; 15 classes of Ghidra and Elasticsearch) | the body starts with the `;` line |
| `anonymous_static_qualifier` | qualifies a call of an anonymous class's static method from a class nested in it with the class's missing name, `<unrepresentable>.m(...)` (Elasticsearch's `AbstractTSDBDocValuesProducer`) | the call is written unqualified, as Java requires |
| `assert_dead_throw` | leaves the `throw new AssertionError(...)` an assert was built from as dead code when the assert's `if` reached it through a break (javac: `unreachable statement`), which also keeps its message's variables declared outside their loop (`for (key : keys)`) | the consumed throw is removed |
| `handler_copy_offsets` | fails a method whose try/finally is protected by an exception range with several entries, typically a Kotlin suspend function whose `try` spans a suspension point, because the `finally` copy that range splitting makes has no bytecode offsets for the finally processing to remove (`removal index (is 0) should be < len (is 0)`) | the method decompiles; the `try` whose exit holds the inlined copy becomes a plain `try`/`finally`, each split-off part a `try`/`finally` guarded by a semaphore variable |
| `nested_handler_copies` | fails a method whose duplicated handlers include a `catch` and the handler around it, as with record patterns inside a `try`/`catch` or Kotlin suspend functions with `try`/`finally`, `use` or `consume` (`RuntimeException: parsing failure!`; kotlinx-coroutines' `toList`) | the method decompiles; a duplication that was not needed is undone, so the source's single `try` comes back |
| `copied_finally` | writes a `try`/`finally` that irreducible-flow splitting copied as `catch (Throwable)` without the rethrow, which swallows the exception | the copy is a `finally` like its original |
| `type_switch_enum_label` | fails a Java 21+ pattern switch that mixes qualified enum constant labels with type patterns (`case E.X -> ...; case C c -> ...` on an interface), leaving the `EnumDesc` label as the raw switch index (`Invalid switch case set`; Vineflower's own `TestSwitchOnEnumJ21.testComplexPattern`) | the label is resolved (upstream `36ac62ec`) and printed qualified when the switch is on another type |
| `type_switch_pattern_type` | types a type pattern that needs no cast (`case Integer i` on an `Integer` selector) as `Class`, so a switch that also has a constant label fails (`Invalid switch case set` with the JDK loaded, `invalid constant type` without debug info), and without debug info prints `case Class<Integer> var3` | the pattern is typed by its class; the selector keeps its type |
| `type_switch_label_lookup` | finds a pattern switch's label only in the first position of a case and by value, so a case with several labels keeps raw indices (`invalid constant type`, index out of bounds; Vineflower's `TestUnnamedVar1.patternSwitch3`) and integer labels come out wrong (`case 5, 7` as `case 5: case 1:`) | every label is resolved; a case of several type patterns prints as one label of unnamed patterns (`case Integer _, String _:`) without javac's re-test |
| `kt_return_outside_lambda` | in Kotlin output, drops `return` in every method without metadata (`@JvmOverloads` overloads, `$$forInline` copies, `DefaultImpls`, `-impl` methods) and in an object's functions written inside a lambda, printing an early `return v` as a bare `v` (wrong output) | only a lambda's body ends in its bare value |
| `kt_failed_method_hidden` | in Kotlin output, leaves a method without metadata whose body fails out of its class with no failure comment (11 members of kotlinx-coroutines and kotlin-reflect, such as `$$forInline` copies of suspend functions) | the method is printed with its `// $VF: Couldn't be decompiled` comment, as in Java output |
| `kt_object_super_call` | in Kotlin output, writes an object expression's superclass without its constructor arguments, `object : Base` for `object : Base(k + 1, s)` | the superclass is written with the arguments the object's `new` passes it, `object : Task(name, cancelable)` |
| `kt_vararg_from_metadata` | in Kotlin output, writes every array parameter of a constructor as `vararg x: Any` (okio's `Options`) and drops `vararg` from primitive and unsigned varargs, `fun String.trim(chars: CharArray)` (wrong output) | a parameter is `vararg` exactly when its metadata says so, written with its element type |
| `kt_default_arg_assigned_param` | in Kotlin output, takes the first assignment of a default-arguments branch for the default, so a default computed through locals prints the first local's initializer (`t: Int = this.measure(s)` for `run { val u = measure(s); ...; v - 1 }`) (wrong output) | the default is what the branch assigns to the parameter (directly, or through kotlinc's null-checked temporary); otherwise unknown, `= ...` |
| `kt_constructor_diamond` | in Kotlin output, writes Java's diamond on constructor calls, `ArrayList<>(4)`, which is not Kotlin (196 calls in the Kotlin libraries above) | the type arguments are left to Kotlin's inference, `ArrayList(4)` |
| `kt_local_visibility_comment` | in Kotlin output, writes the note on a local class written outside its function as a line comment reading `outside of methodSupplier`, then the header at column 0 | the note is inline on the declaration's line, `/* $VF: local visibility outside of method */ internal class ..` |
| `kt_default_args_ctor_descriptor` | in Kotlin output, never finds an inner class constructor's defaults, printing `= ...` (okhttp's `Http2Stream.FramingSink(finished: Boolean = ...)`) | the default-arguments constructor is found by the constructor's own descriptor, `finished: Boolean = false` |
| `var_merge_narrowing` | merges two variables of one slot under a type narrower than (or unrelated to) a value one of them is assigned, printing a cast the bytecode does not have there, or none, such as a suspend call's result cast before its `COROUTINE_SUSPENDED` check, `Integer v = (Integer)src.next(c)` (wrong output) | the merge is refused and the cast stays where the bytecode has it, unless the LocalVariableTable declares one local at every store to both whose type admits every value stored and every use, in which case the merge is made with that type |
| `switch_expression_poly_cast` | casts a switch expression whose results are numbers of different types to its reference target when their join is unknown, `return (Number)(switch (x) { case 1 -> 1; default -> 2L; })`, which makes it a standalone numeric switch whose `case 1` yields a `Long` (wrong output) | the switch is written without the cast, a poly expression whose results each fit the target |
| `ternary_numeric_arms` | prints a reference conditional whose boxed arms are numbers of different types as a numeric one, `c ? 1 : 2L` or `c ? 1 : a`, which promotes `1` to `Long` and unboxes `a` (`NullPointerException` on `null`; Elasticsearch's `BucketHelpers`) (wrong output) | an arm is cast to the conditional's type, `c ? (Number)1 : 2L`, `c ? (Integer)1 : a`, so it stays a reference conditional |
| `kt_reference_anonymous_super_call` | in Kotlin output, reads a function-reference class's owner and name from its constructor's `super(..)` call, which `NestedClassProcessor` has by default already moved out of an anonymous class, so a bound reference fails its method (`IllegalStateException: Anonymous class does not have Class Kotlin metadata`) and an unbound one prints `<unrepresentable>.INSTANCE` | the reference is written, `handler::invoke`, from the super call the class node keeps |
| `kt_property_reference` | in Kotlin output, does not recognise property-reference classes (`PropertyReference0Impl` and the others): a bound one fails its method (same exception; kotlinx-coroutines' `LockFreeLinkedListNode.toString`) and an unbound one prints `<unrepresentable>.INSTANCE` | the reference is written: `this::classSimpleName`, `Owner::name`, `::name` |
| `kt_anonymous_class_failure_reason` | fails what has no Kotlin form, suspend functions' continuations above all, with the bare `Anonymous class does not have Class Kotlin metadata` | the method still fails, but the message names the class and says why: a suspend function's coroutine state machine, a reference to a local function that captures variables, or an object created outside the method that declares it |
| `nonpublic_nested_import` | imports a non-public member class of a library type (`import org.apache.lucene.search.Weight.DefaultScorerSupplier;`), which javac rejects (`has protected access`), so the whole file does not compile (wrong output) | the class is named through its enclosing class, `Weight.DefaultScorerSupplier` |
| `override_inaccessible_method` | writes `@Override` on a method whose only match in a supertype is private, or package-private in another package (javac: `method does not override or implement a method from a supertype`; Elasticsearch's `ElasticsearchNodeCommand` subclasses) | no `@Override` there |
| `primitive_array_wildcard` | drops the wildcard of a type argument bounded by a primitive array, `Publisher<? extends byte[]>` printed `Publisher<byte[]>`, so an overriding method no longer overrides (spring-framework's `ByteArrayEncoder.encode`) | the wildcard is kept |
| `pattern_variable_scope` | makes `if (x instanceof T) { v = (T) x; ...}` a pattern `x instanceof T v` although `v` is assigned on the other path or read after the `if`, outside the pattern's scope (javac: `cannot find symbol`, `already defined`; Elasticsearch's `ES812PostingsReader.postings`) | the pattern is made only when every use of the variable is in its scope |
| `switch_case_scope` | forgets a case's declarations when the case's statements end and visits cases out of written order, so two cases declaring the same name are written without braces (javac: `already defined`; Guava's `DoubleMath.roundIntermediate`) | the clash puts the cases in braces, as the source has them |
| `local_class_anonymous_subclass` | declares a local class after an anonymous class that extends it, where javac cannot resolve it (Guava's `Types.ClassOwnership.detectJvmBehavior`) | the local class is declared before that anonymous class |
| `kt_for_loop_exit_condition` | in Kotlin output, writes a Java `for` loop it cannot turn into a range as `while (true) { if (COND) break ...` with `COND` the loop's *continue* condition, so the loop stops exactly when it should go on (wrong output; okio's `GzipSink.updateCrc`) | `while (COND) { ... UPDATE }` |
| `kt_for_range_counter` | in Kotlin output, rewrites `for (V = A; L < R; W++)` into `for (V in A..B)` or `repeat(N) { V -> }` without checking that `W` is `V`, that the comparison bounds `V` on the side it moves towards, that the body leaves `V` and the bound alone and that nothing after the loop reads `V` (wrong output) | the range form only for a loop that counts its initializer's variable to the bound; otherwise the `while` form |
| `kt_for_range_exclusive_bound` | in Kotlin output, writes the exclusive bound of `V < X` as the inclusive `A..X` when `X` is a variable and as `repeat(N - 1)` when `N` is a constant, one iteration too many or too few (wrong output; `reverse`'s `for (index in fromIndex..midPoint)`) | `A until X`, `repeat(N)`; a decrementing loop with a variable bound, or a constant bound that would overflow, takes the `while` form |
| `kt_for_range_unmatched_bound` | in Kotlin output, prints nothing for a Java `for` loop whose bound is neither a constant nor a variable, `for (i = 0; i < a.length; i++)`, so the loop disappears (wrong output; `Duration.parse`'s `skipWhile` loops) | the loop in the `while` form |
| `kt_for_loop_definitions` | in Kotlin output, leaves out the declarations attached to a `for` loop it writes as a `while`, so a counter declared by the loop is assigned but never declared (22 loops) | the declarations before the loop, as for the other loop kinds |
| `kt_for_loop_continue_update` | in Kotlin output, puts the update of a `for` loop it writes as a `while` at the end of the body, which a `continue` skips, so `continue` loops forever or repeats an element (wrong output; okhttp's `HttpUrl.portColonOffset`) | when the body continues the loop, `if (C) { do { ... } while (run { UPDATE; C }) }`, where `continue` runs the update |
| `var_merge_liveness` | merges two variables of one slot whose lifetimes overlap after the Kotlin schedule's `SimplifyStack` with `inlineRegularVars` replaced a local by the stack variable it copies, so the later one overwrites a value still in use (wrong output) | the merge only when no write of the later variable (other than a copy of the earlier one) can be followed by a read of the earlier one's value |
| `kt_partial_array_dims` | in Kotlin output, fails a method that creates an array of arrays with only its outer length, `arrayOfNulls<ByteArray>(n)`, reading a length for every dimension of the type (`index out of bounds`; okio's `Buffer.snapshot(Int)`) | the method decompiles; the innermost sized level is `arrayOfNulls(n)` |
| `irreducible_with_exception_edges` | fails a method whose loop is entered at more than one place and holds a `try`, as node splitting is never tried where an exception edge exists and in such a loop no statement dominates the `try` (`parsing failure!`) | the method decompiles: where Vineflower gives up, the smallest single-entry region holding a handler's whole range is built first, or a statement outside every protected range is copied, and Vineflower's own splitting does the rest |
| `kt_semaphore_store_liveness` | in Kotlin output, drops every `false` store to a `finally` semaphore as dead because the guard that reads it is not an exprent, so the output no longer says when the `finally` runs (wrong output), or variable definition fails (`Var types not processed!`) | the stores to a variable a `finally` guard reads are kept, as in the Java output: `var8 = true`, `var8 = false` on each normal exit |
| `lambda_catch_var_rename` | declares a lambda body's catch variable under its table name after renaming its uses for a clash with the enclosing method: `catch (Exception e) { log(ex); }` (Elasticsearch `BlobStoreRepository`) | the clause declares the name the uses read |
| `type_annotation_qualified_name` | drops the qualifier of a type that carries a type annotation: `Map.@Nullable Entry` is written `@Nullable Entry`, another type or none (wrong output; Guava `AbstractMapBasedMultiset`) | the qualifier the import collector gives the type, before the annotation |
| `type_annotation_argument_scope` | writes a type annotation of a later generic argument in an earlier one: `Function<Map<String, @Nullable @Nullable Object>, @Nullable Map<String, Object>>` (wrong output; spring `MergedAnnotationCollectors.toMultiValueMap`) | each annotation in the argument its path names |
| `assert_throw_after_if` | leaves `if ($assertionsDisabled \|\| cond) { rest } throw new AssertionError();` when the code after an assert returns or ends a loop body ("Could not resugar all assert statements"; Elasticsearch `MultiChunkTransfer`) | `assert cond;` followed by the rest |
| `sealed_private_permits` | names private member classes in a top-level sealed class's `permits` clause, which javac rejects (Elasticsearch `DiskBBQBulkWriter`) | no `permits` clause, as the source must have had |
| `switch_expression_assignments` | turns a switch statement that assigns a variable more than once in a case, or leaves a case without assigning it, into a switch expression: `yield` before unreachable code, arms without a value (wrong output; Elasticsearch `JavaDateMathParser.parseMath`) | the switch statement |
| `nested_param_rename` | declares a parameter of an anonymous class's method under a name its uses do not read: `onFailure(Exception ex) { log(e); }` (Elasticsearch `TransportService`, `SearchService`, `AbstractThreadedActionListener`) | the name the uses read, which is not the table name when that is no Java identifier (kotlinc's setter parameter `<set-?>`) |
| `switch_expression_arm_scope` | declares a variable in a switch expression's arm under a name already declared around the expression (spring's ASM `Frame`) | the clash is scoped or renamed |
| `unchecked_type_pattern` | turns `if (x instanceof C) { C<A> v = (C<A>) x; ...}` into the type pattern `x instanceof C<A> v` (or `x instanceof T v` for a type variable `T`) even when the cast is unchecked, which javac rejects (`instanceof.reifiable.not.safe`; wrong output) | the `instanceof` and the cast stay as in the bytecode; a pattern whose type the operand's type determines, or a reifiable one (`List<?>`), is still made |
| `inner_method_argument_shift` | matches the arguments of a call to a method of a non-static inner class to the generic signature's parameters one place late, as if the method took the outer instance like the class's constructors, casting arguments to the neighbouring parameter's type, `segment.put((V)key, (long)value, now)` (wrong output) | the arguments are matched from the first parameter, and only the method's own type variables are inferred from them |
| `raw_receiver_erasure` | infers a call's generic result from its arguments even on a raw-typed receiver, where Java erases the member (JLS 4.8), dropping the cast in `(String[])map.keySet().toArray(new String[0])` on a raw `Map` and writing a for-each over a raw collection with a typed loop variable (Elasticsearch's `KeyStoreWrapper`; wrong output) | a call on a raw-typed receiver has its erased types, so the bytecode's cast is written, and a for-each over a raw iterable casts it to its class parameterized by the loop variable's type |
| `boxed_type_variable_cast` | drops the boxing call under a cast to a type variable, `return (T)0;` for `(T) Integer.valueOf(0)`, though Java has no cast from a primitive to a type variable (Guava's `Defaults.defaultValue`; wrong output) | the boxing call stays: `(T)Integer.valueOf(0)` |
| `implicit_widening_arithmetic` | leaves a widening conversion unwritten where the other operand, as written, is narrower too, so the operation runs in the narrower type: `-(long) pageSize * n` printed `-pageSize * n`, an overflowing `int` product (wrong output) | the conversion nearest the left operand is written where the operands as written promote to a narrower type than the operation's: `-((long)pageSize) * n` |
| `mixed_type_reassociation` | drops the parentheses of `a * (b * c)` across a type change: `(long) ord * (size * 2L)` printed `ord * size * 2L`, an overflowing `int` product (Elasticsearch's `ES920`/`ES940DiskBBQVectorsWriter.readQuantizedVector`; wrong output) | the parentheses stay unless every step of the regrouped expression computes in the operator's type |
| `protected_back_edge` | builds a `try`/`finally` inside a loop whose back edge lies in its range when the loop's exit is the `finally` copy before a `return`: `try { while (true) { synchronized .. } } finally` printed `while (true) { try { .. } finally { .. } }`, running the `finally` every iteration (Guava's `SequentialExecutor$QueueWorker.workOnQueue`; wrong output) | the loop is built first, inside the `try` |
| `loop_first_if_exit` | turns an infinite loop's first `if` into the loop's exit although the path past it continues the loop, after an empty inner loop sharing the head was removed: `while (true) { while ((l = v[++i]) < p) {} .. }` printed `while ((l = v[++i]) >= p) { .. } break;` (Elasticsearch's `BulkNeighborQueue$ReservoirTopK.quickSelectIntro`; wrong output) | the rewrite only when the `if`'s successor is the exit: `while (true) { if ((l = v[++i]) >= p) { .. } }` |
| `reference_unboxing` | writes an unboxing call as its bare operand where the context takes a reference, `Long.valueOf(x.longValue())` printed `x`, `x.intValue() == y.intValue()` printed `x == y`, so `null` passes where the bytecode throws and boxes are compared by identity (Elasticsearch's `RangeType$2.nextDown`; wrong output) | the unboxing call is written there: `((Long)o).longValue()`, `x.doubleValue() == y` |
| `lambda_argument_overload` | writes an implicitly typed lambda or a method reference without the cast that picks its overload where another overload takes a different functional interface of the same shape, which javac rejects as ambiguous: `spliterator.forEachRemaining(this::add)` on a `Spliterator.OfInt` (wrong output) | the argument is cast to the overload's interface, `(IntConsumer)(this::add)` |
| `static_type_variable_scope` | counts a class's type variables in scope in its static methods, so a static method's unresolved type variable `V` is taken for the class's and written in a cast, `getUninterruptibly((Future<V>)future)`, which javac rejects (Guava's `AbstractFuture`; wrong output) | a class's type variables are not in scope in its static methods |
| `unchecked_generic_cast` | writes no cast where a returned or assigned value's declared generic type does not fit its target, because the unchecked cast the source had, `(CacheBuilder<K1, V1>) this`, compiles to no instruction, so javac rejects the bare value (`incompatible types`; wrong output) | `this`, a field (static or of `this`) or a declared variable whose type the target's type arguments do not accept gets the cast: to the target, or to its raw class where the two parameterizations are provably distinct |
| `kt_when_case_type` | in Kotlin output, writes a `when` label with its constant's own type instead of the selector's, `58 -> return i` on a `Char` (okhttp's `HttpUrl.portColonOffset`), which does not compile | the labels have the selector's type, `':' ->` |
| `kt_method_object_return_type` | in Kotlin output, writes a method without metadata that returns `Object` without a return type, a `Unit` function returning a value (`fun describe(value: Any?) { return .. }`) | the return type is written, `: Any?` |
| `kt_secondary_constructor_line` | in Kotlin output, does not end the line of a secondary constructor without a body, `: super(cause)}` | the line ends (layout only) |
| `kt_range_operand_precedence` | in Kotlin output, writes a range bound that binds less tightly than the range operator bare, `for (index in if (c) 1 else 0 until length)`, whose `else` branch becomes the range (kotlin-stdlib's `toBigIntegerOrNull`) | the bound is parenthesised, `(if (c) 1 else 0) until length` |
| `kt_lambda_singleton_returns` | in Kotlin output, writes every exit of a lambda compiled to a singleton class (`INSTANCE`) as its bare value, so an early `return@label v` in a loop becomes the statement `v` and the lambda runs on (wrong output) | a lambda with an exit not in tail position is labelled and its exits are `return@lambda_1 v` |
| `kt_type_parameter_id` | in Kotlin output, writes `Any` for every type that refers to a class's (or an outer class's) type parameter, `fun get(index: Int): Any` for `E` | the parameter's name, `fun get(index: Int): E` |
| `kt_java_cast_syntax` | in Kotlin output, writes casts in Java's syntax, `(int)Math.min(a, (long)b)`, `(T)value` (about 2,500 in the Kotlin libraries) | a numeric conversion is Kotlin's conversion call, `Math.min(a, b.toLong()).toInt()`, a cast to a type variable `value as T`; a cast to a class or array type keeps Java's syntax, as Kotlin's `as` would reject `null` |
| `lambda_pertinence` | counts a lambda or an inexact method reference argument as fixing the type variables of its parameter type, where javac resolves them first, so a generic call that is the qualifier of another and whose variables only such arguments fix gets no type arguments, `names.map(n -> () -> n.length()).iterator()`, which javac rejects (wrong output) | the call gets its explicit type arguments, `names.<Task<Integer>>map(...)` |
| `lambda_type_variable_target` | writes a lambda passed for a bare type variable, `Iterators.single((builder, p) -> ...)`, without the cast the source had where the call is an argument of another generic call whose other arguments resolve that variable to a type that is not a functional interface (Elasticsearch's `BlobStoreRepository`; wrong output) | the lambda is cast to its interface, `single((ToXContent)((builder, p) -> ...))` |
| `unchecked_call_result_cast` | writes no cast where a returned or assigned call's declared result does not fit its target, because the unchecked cast the source had (`(Constructor<Exception>) exceptionClass.getConstructor(..)`) compiles to no instruction (spring-framework's `ServiceLocatorFactoryBean`; wrong output) | the call gets the cast of `unchecked_generic_cast`, its type being the method's return type with the receiver's declared type arguments |
| `loop_body_exit_explicit` | drops the exit of a loop's body when the loop is the last statement of an outer loop's body and the exit goes where that body's end goes: `continue outer` out of an inner `for` prints as nothing and the inner loop runs on (wrong output) | the exit is printed (`break;`) |
| `type_variable_shadowing` | takes an enclosing class's or method's type variable for a same-named one declared closer to the code, so a cast judged by its bounds is dropped: `use((Number)t)` becomes `use(t)`, which calls another overload (wrong output) | the innermost declaration's bounds are used |
| `compound_shift_count_type` | types the `int` count of a compound shift of a `long` (`mask >>>= sz`) as a `long`, so `sz != 0` prints as `sz != false` (Ghidra's `Utils.leastsigbit_set`) | the count is typed as a shift count, `sz != 0` |
| `definition_const_type` | writes the `0`/`1` of a dead store declared `int` from the local variable table as a boolean, `int var4 = false;` (Ghidra's `InstructionDB.getLength`, Guava's `BigIntegerMath`) | the constant is written in the declared type, `int var4 = 0;` |
| `catch_tail_inline` | moves code only a catch clause reaches after the `try` when the `try` body cannot complete normally, out of the scope of the clause's variable (spring-boot's `LambdaSafe.invoke`, `this.logNonMatchingType(callbackInstance, ex)` after the `try`) | code that reads the clause's variable stays in the clause |
| `diamond_argument_conflict` | writes a diamond on a generic `new` whose arguments bind a type parameter to another type than the expected type needs, which javac cannot infer (`Supplier<Set<V>> f = new EnumSetSupplier<>(valueClass)` with a `Class<V0>`) | the `new` is written raw, the unchecked conversion the source made |
| `kt_default_args_call` | in Kotlin output, writes a call that leaves arguments to their defaults as a call of the synthetic bridge it compiles to, with placeholders, bit mask and marker, `StringsKt.indexOf$default(s, c, 0, false, 6, null)`, which Kotlin cannot call | the call the source makes, `StringsKt.indexOf(s, c)`, naming a passed argument after a defaulted one or one another overload could take; a call that cannot be resolved to one function with those defaults, or that is the body of an `@JvmOverloads` overload, is left as it is |
| `kt_generated_constructor_hidden` | in Kotlin output, writes the constructors kotlinc generates for default constructor arguments (the no-argument constructor of a class whose parameters all have defaults, and each `@JvmOverloads` overload) as functions named after the class, `fun Builder() { this(null, 1, null) }`, which Kotlin does not accept | they are not written: the primary constructor's defaults (and its `@JvmOverloads`) say they exist |
| `kt_java_reference_cast` | in Kotlin output, writes casts to class and array types in Java's syntax, `(java.util.List<T>)listOf(e)` | Kotlin's `as` with Kotlin type names, `listOf(e) as List<T>`, and `as T?` where the operand may be `null`; a cast that only picks a Java overload for a field of the parameter's own type is left out |
| `kt_explicit_type_arguments` | in Kotlin output, writes explicit type arguments before the called name, `CollectionsKt.<File>last(list)` (7 in kotlin-stdlib and kotlinx-coroutines) | after it, `CollectionsKt.last<File>(list)`; left out when one is not known in full |
| `kt_abstract_class_constructor` | in Kotlin output, does not read an abstract class's constructors from its metadata, and writes them as functions named after the class, `open fun ForwardingSink(delegate: Sink) { super() ..}` | the primary constructor in the class header with its supertype call, `abstract class ForwardingSink(delegate: Sink) : Sink`, its code in `init`, and secondary constructors as `constructor(..) : this(..)` |
| `kt_constructor_body_after_first_block` | in Kotlin output, drops a constructor's code after its `super(..)`/`this(..)` call when that code starts with an `if`, such as the checks and field stores of `IntProgression`'s `init` | writes it, in `init { .. }` or the secondary constructor's body |
| `kt_inline_class_constructor_accessor` | in Kotlin output, takes the synthetic accessor kotlinc adds for a constructor with an inline-class parameter as the constructor, `class UIntRange(..) : UIntRange(start, endInclusive)`, and writes the real constructor as `fun UIntRange(..)` | the real constructor: `class UIntRange(..) : UIntProgression(start, endInclusive, 1)` |
| `kt_constructor_default_signature` | in Kotlin output, does not find a constructor whose metadata leaves out its JVM descriptor, and writes it as a function, `fun Unit() {}` in `object Unit` | looks it up by the descriptor its parameters give, as Kotlin's reader does |
| `kt_jump_to_non_loop_label` | in Kotlin output, writes a jump out of a labelled `if`, `switch`, unresugared `for` or `synchronized`, or to a `repeat` loop's next iteration, as `break@labelN` or `continue@labelN`, `labelN@ if (..) { .. break@labelN }`, though Kotlin's `break`/`continue` need a loop | `return@labelN`, wrapping a labelled `if` or `synchronized` in `run labelN@{ .. }`: `run labelN@{ if (..) { .. return@labelN } }` |
| `kt_synthetic_object_super_call` | in Kotlin output, fails every suspend function that suspends, as its continuation is an anonymous `ContinuationImpl` subclass with only synthetic-class metadata whose constructor passes `$completion` to its superclass (`IllegalStateException: Anonymous class does not have Class Kotlin metadata`) | the function decompiles to its state machine, the continuation an `object : ContinuationImpl($completion) { .. }` expression whose type, which Kotlin cannot name, is `<unrepresentable>` |
| `kt_file_class_constructor` | in Kotlin output, writes the constructor kotlinc gives a multifile class part (`-Xmultifile-parts-inherit`, as in kotlin-stdlib) among the file's top-level declarations, `open fun StringsKt__StringsKt() {}` | leaves it out: a Kotlin file has no constructor |
| `kt_when_break` | in Kotlin output, writes Java's unlabelled `break` out of a `switch` that ends a statement inside a case as `break` in the `when`: Kotlin rejects it outside a loop and inside one it leaves the loop | leaves it out where the branch ends after it; otherwise `return@labelN` from a `run labelN@{ .. }` written around the `when` |
| `kt_java_statement_jump` | in Kotlin output, writes the jump after a statement in a `try` block, a `finally` block or a `synchronized` block in Java's syntax, `break;`, `break label161;` (6 in kotlinx-coroutines) | Kotlin's: `break`, `return@label161` |
| `kt_catch_block_jump` | in Kotlin output, drops the jump after a `catch` block, so a `catch (e) { ..; continue }` falls through to the code after the `try` (1 in kotlin-stdlib, `DeepRecursiveScopeImpl.runCallLoop`) | writes the jump |
| `kt_supertype_type_arguments` | in Kotlin output, writes the superclass call in a class header without the superclass's type arguments, `: AbstractCoroutine(parentContext, true, active)`, which Kotlin requires | `: AbstractCoroutine<T>(..)`, from the class's metadata |
| `anonymous_class_created_elsewhere` | maps an anonymous class's captured variables only from `new`s in its declaring class, so for an object that kotlinc's `$$inlined$` lambda copies create in another class its first captured value is read as the declaring class's `this` and the rest as its own fields (wrong output), and Kotlin output fails one without Class metadata (`Anonymous class does not have Class Kotlin metadata`) | an anonymous class that only one other class creates is nested in it and its captures are mapped from the creating method |
| `captured_var_clash_name` | gives a captured value's variable in an anonymous or local class or a lambda the name the creating method's variable has before the clash renaming renames that variable (`x` becomes `xx`), so the nested body reads the other variable of that name (wrong output), and the Kotlin writer fails such an object without Class metadata | a captured variable is printed with the clashing name of the variable it holds |
| `captured_var_through_cast` | takes a constructor argument for a captured variable only when it is a variable, so where the Kotlin plugin inlines kotlinc's cast copy of an inline lambda's parameter into the `new` the object keeps a field and a constructor parameter, and Kotlin output fails an object without Class metadata | a cast of a variable whose class is a subtype of the cast's class is the capture of that variable; the object reads the variable (`cancellable$iv`) |
| `kt_legacy_constructor_parameter_checks` | in Kotlin output, drops Kotlin 1.4's parameter null checks but keeps Kotlin 1.3's (`checkParameterIsNotNull`), which in a constructor come before the `super(..)`/`this(..)` call, so the call is not found: the class header lacks it and `init` holds `super(..)` as a statement | a constructor's checks before its delegation call are dropped too, as kotlinc's non-null parameter types imply them, `class GzipSink(sink: Sink) : Sink`; where the call comes later, in its arguments' branches, the opening checks are dropped and the call stays in the body |
| `kt_hidden_empty_super_call` | in Kotlin output, loses the empty superclass call that `hideEmptySuper` removes for Java, so a header names the superclass without a call, `class RealResponseBody(..) : ResponseBody`, which Kotlin rejects, and a secondary constructor takes its first statement for its delegation call or, with no other statement, is left out | the header writes `ResponseBody()`; a secondary constructor is written without a delegation call (the same implicit `super()`), its statements in its body |
| `kt_compound_bitwise_assignment` | in Kotlin output, writes a compound assignment with a bitwise or shift operator in Java's syntax, `set \|= 1 shl id`, which Kotlin does not have | `set = set or (1 shl id)`, where the left side reads the same place again with no effect and its type is `Int`, `Long` or `Boolean` |
| `kt_switch_head_indent` | in Kotlin output, writes the statements before a `when` (the switch's first block, where a branchy `when` hoists its variables) at indentation 0 after one indent, so all but the first start in column 0 | writes them at the `when`'s indentation, as Java's `SwitchStatement` does; whitespace only |
| `kt_operator_precedence` | in Kotlin output, parenthesises operands by Java's precedence although Kotlin's differs (its infix functions such as `shl` and `or` share one level, below arithmetic and above comparison, and its elvis binds above `is`, comparison, `&&` and `\|\|`), so `x \| 1 << k` comes out `x or 1 shl k` (Kotlin: `(x or 1) shl k`; wrong output) | adds the parentheses Kotlin's precedence needs, `x or (1 shl k)`, and keeps Java's |
| `kt_narrow_bitwise_operand` | in Kotlin output, writes a `Byte`, `Short` or `Char` operand of a bitwise or shift operator as Java does, without its promotion to `Int`, `b and 255`, which kotlinc rejects (`Byte` has no `and`) | `b.toInt() and 255`, `c.code xor 32`, `(data[p].toLong() and 255L)` where Java widens to `long` |
| `kt_reference_inequality` | in Kotlin output, writes Java's `!=` of two references (`if_acmp`, an identity test) as Kotlin's `!=`, which calls `equals`, while its `==` becomes `===` (wrong output) | `a !== b`; a comparison resugared from `Intrinsics.areEqual` keeps `==`/`!=` |
| `handler_copy_shared_body` | fails a method whose `finally` body branches (`if (consume) cancel(cause)`) and whose `try` is protected by a range with several entries, typically a Kotlin suspend function whose `try` spans a suspension point, because range splitting copies only the handler's first block (`removal index (is 0) should be < len (is 0)`) | the method decompiles; each copy gets its own copy of the blocks only the handler runs, and each part split off at a resume label is a `try`/`finally` guarded by a semaphore variable |
| `kt_accessor_backing_field` | in Kotlin output, writes a property accessor's reads and writes of its own backing field `this.x` (`Owner.x`), which in Kotlin calls the accessor again, so okhttp's `Dispatcher.maxRequests` getter recurses without end (wrong output) | `field` inside the accessor's own body and the object expressions written in it |
| `kt_string_literal_escape` | in Kotlin output, writes string constants with Java's escapes: `"$this$commonWrite"` and `"${name}"` are templates in Kotlin, `\f` no escape; a template splices `Char` pieces raw and writes `$key` before a letter (`"$key258EAFA5-.."`) (wrong output) | `\$` where a `$` would start a template, `\u000c`, escaped `Char` pieces, `${name}` before a letter or digit |
| `kt_abstract_function_body` | in Kotlin output, writes an abstract or `external` function with an empty body (`abstract fun dispose() { }`), which Kotlin rejects | no body |
| `kt_misplaced_modality` | in Kotlin output, writes a modality on property accessors (`public final get()`) and `final` on top-level properties, which Kotlin rejects | neither |
| `kt_companion_property_initializer` | in Kotlin output, drops the initializer of a companion object's property, which the outer class's `<clinit>` stores (every `CipherSuite.TLS_..` constant has no value) | the initializer, read from the outer class's static field initializers |
| `kt_default_property_initializer` | in Kotlin output, writes a property whose initializer is its type's default (kotlinc stores none) without one, `var count: Int`, which Kotlin rejects | `= 0`, `= false`, `= null`, ... when no constructor stores the field |
| `kt_missed_override` | in Kotlin output, writes an override with a narrower return type or a type-variable parameter (`Buffer.buffer(): Buffer`, `compareTo(other: ByteString)`) and every overriding property `open`, which Kotlin rejects | `override` where a bridge method of the class calls the function, or a property's getter overrides |
| `kt_outer_this` | in Kotlin output, writes an enclosing class's instance in Java's form, `Pipe.this.buffer`, also in templates (`"$Buffer.this.x()"`) | `this@Pipe.buffer`, `"${this@Buffer}.x()"` |
| `kt_extension_receiver` | in Kotlin output, writes an extension function's receiver by its parameter's name (`` `$this$commonWrite` ``, `var0`), which names nothing in Kotlin | `this` (and `this@Class` for the class's own instance in an extension declared in a class) |
| `kt_nested_annotation_argument` | in Kotlin output, writes an annotation argument of an annotation with `@` (`replaceWith = @ReplaceWith(..)`), Java's form | `replaceWith = ReplaceWith(..)` |
| `kt_throws_class_name` | in Kotlin output, writes the `@Throws` of a `throws` clause with internal names, `@Throws(java/io/IOException::class)` | `@Throws(IOException::class)` |
| `kt_throwable_cast` | in Kotlin output, keeps kotlinc's no-op `checkcast java/lang/Throwable` before `athrow`: `throw IOException(..) as java.lang.Throwable`, which Kotlin rejects | `throw IOException(..)` |
| `kt_mapped_java_member` | in Kotlin output, calls JDK methods by names Kotlin's mapped types do not have: `s.length()`, `s.charAt(i)`, `list.size()`, `map.entrySet()`, `e.getMessage()`, `x.getClass()`, an array's `length` | `s.length`, `s[i]`, `list.size`, `map.entries`, `e.message`, `x.javaClass`, `size` |
| `kt_mapped_java_type` | in Kotlin output, writes `java.lang.String`, `CharSequence`, `Throwable`, `Number` and the boxes other than `Integer` by their Java names, types Kotlin does not unify with its own (`val tag: java.lang.String = loggerTag(name)` is rejected) | `String`, `CharSequence`, `Throwable`, `Number`, `Long`, `Char`, ..., also in type-parameter bounds (`where T : Throwable`) |
| `kt_object_instance` | in Kotlin output, reaches an `object` through its `INSTANCE` field, `SegmentPool.INSTANCE.recycle(s)` | `SegmentPool.recycle(s)` |
| `kt_super_property_access` | in Kotlin output, writes a property read through the superclass's accessor (`invokespecial`) as `this.isActive`, which in an override of `isActive` recurses without end (wrong output; kotlinx-coroutines' `AbstractCoroutine.isActive`) | `super.isActive` |
| `kt_setter_parameter_name` | in Kotlin output, names a setter's parameter in its header by the metadata but in its body by the local variable table, `<set-?>` for a setter kotlinc generates: `set(value) { this.total = var1 }` names a variable that does not exist | the body uses the header's name; `value` for `<set-?>` |
| `kt_java_class_literal` | in Kotlin output, writes the class constants Java's expression code prints (extracted initializers, `init` blocks) as `C.class` | `C::class.java` |
| `kt_field_without_property` | in Kotlin output, writes a field with no Kotlin property (every field of a class without Class metadata, such as continuation spill fields) as a Java declaration, `open int label;`, which kotlinc rejects | a property from the class file, `@JvmField internal var label: Int = 0` (`val` if final, the field's visibility, `@JvmField` unless private, the JVM default as a `var`'s initializer); static fields keep Vineflower's form |
| `kt_constructor_without_metadata` | in Kotlin output, writes a constructor the metadata does not list (in a class without Class metadata or an anonymous object written as a named class) as a function, ``fun `Cls`(x: Int) { super(x); .. }``, and an object expression's constructor statements as a bare block, both rejected by kotlinc | `constructor(x: Int) : super(x) { .. }`, the body's first superclass or `this` call as the delegation, with the constructor's visibility, and `init { .. }` |
| `kt_delegation_call_arguments` | in Kotlin output, puts a delegation call's argument that kotlinc null-checks (a Java call's platform-typed result) in a stack variable before the call, `val var10001 = Pattern.compile(pattern); this(var10001)`, or a primary constructor's call in an `init` block, which kotlinc rejects | the value in the call, `: this(Pattern.compile(pattern))`, when that computes the same values in the same order; the checks are kotlinc's for that expression |
| `kt_copied_equality` | in Kotlin output, writes a copy of an `==` resugared from `Intrinsics.areEqual` as the identity test `===`, so a default value `b: Boolean = a == "x"` comes out `b: Boolean = a === "x"` (wrong output) | the copy keeps the function type of what it copies, `a == "x"` |
| `kt_binary_numeric_promotion` | in Kotlin output, writes `Char`, `Byte` and `Short` arithmetic, comparisons and `==` (and `==` of different number types) relying on Java's promotion to `Int`, though Kotlin's `Char + Int` is a `Char`, `Char` compares only with `Char` and `==` needs one type (wrong output: `sb.append(c + 1)` appends a character) | the conversion Java's promotion makes (`c.code + 4`, `b.toInt() == 61`, `l != i.toLong()`); `Char - Char`, an `Int` in both, stays |
| `kt_negated_equality` | in Kotlin output, writes `areEqual(a, b) ^ 1`, which kotlinc before 2.0 compiles `a != b` to, as `(a == b) xor true` (38 in okhttp) | `a != b` (`a !== b` for `(a === b) xor true`); the source's form, equivalent |
| `kt_operator_line_break` | in Kotlin output, breaks a long binary operation before its operator, where Kotlin ends the statement: `return a` / `+ b` returns `a` and evaluates `+b` apart, and a line starting with `*` does not parse (wrong output) | breaks after the operator (`a +` / `b`); `&&` and `\|\|` keep theirs; whitespace only |
| `kt_unboxing_cast_type` | in Kotlin output, casts a value an unwritten unboxing call (`booleanValue()`) unboxes to the Java box class, `predicate(e) as java.lang.Boolean`, which Kotlin does not unbox, and under `!` writes `!p(x) as java.lang.Boolean`, read as `(!p(x)) as ..` | `predicate(e) as Boolean`, `!(p(x) as Boolean)` (the parentheses are `kt_operator_precedence`'s: an unwritten unboxing call binds as what it is written as) |
| `twr_desugaring_shape` | re-sugars try-with-resources from any `catch (Throwable)` that closes the resource first, so hand-written code (logging a close failure instead of `addSuppressed`) prints as `try (R r = ..)` (wrong output), and a resource copied from a non-final field as `try (this.f)`, which javac rejects | re-sugars only javac's exact desugaring and writes `try (this.f)` only for a final field copied to javac's unnamed variable, `try (T v = this.f)` otherwise |
| `twr_unresolved_resource` | re-sugars try-with-resources only when the resource's class is known to be `AutoCloseable`, so a resource whose class (or the supertype making it `AutoCloseable`) is outside the input stays desugared (Elasticsearch's `Releasable`) | re-sugars it when the bytecode is javac's exact desugaring and kuna cannot resolve the class; a class whose whole hierarchy is known and not `AutoCloseable` never is |
| `twr_class_version` | looks for javac 11's try-with-resources desugaring only in class files of Java 11 and later, though javac 11+ emits it for every `--release` (Guava's Java 8 classes: `ByteSink.write`, 15 methods) | looks for it in older class files too, accepting only javac's exact shape |
| `compound_assignment_shape` | chooses `x op= e` or `x = x op e` for a field or array element by a heuristic, not by whether the bytecode loads the receiver once (`b += buffer.remaining()` comes out `this.b = this.b + buffer.remaining()`), and writes `x = e op x` as `x op= e`, reading `x` before `e` (wrong output where `e` writes `x`) | compound exactly when the receiver (array and index) is one load in the bytecode; `x = e op x` keeps its order, whatever `x` is; for `x = x op e`, locals, static fields and Kotlin classes keep Vineflower's choice |
| `implicit_outer_instance` | writes the creation of an inner class whose outer instance is an enclosing class's instance as `Outer.this.new Inner()`, which javac compiles with an outer-instance null check the bytecode does not have | `new Outer.Inner()` when the outer instance is the one an unqualified creation passes and the null check is absent; a creation whose check Vineflower removed stays qualified |
| `vararg_array_length` | writes the array a varargs call passes last as its elements even when it is created by its length: `Arrays.asList(new T[n])` (a list of `n` nulls) comes out `Arrays.asList()` (wrong output) | the array is spread only when it is an initializer or its length is the constant 0 (javac's code for no varargs); otherwise it is passed: `Arrays.asList(new T[n])` |
| `synthetic_null_check_shape` | deletes `Objects.requireNonNull(x);` (or `x.getClass();`) as javac's outer-instance check whenever a nested class creation taking `x` first, a constant, `null` or a store follows, losing the source's own check (wrong output; Guava's `ImmutableSet.Builder.forceJdk`) | removes only javac's own checks: of a `dup`ed value left on the stack (`x.new Inner()`, `x::m`), or followed by a constant of a value whose class (loaded) declares an instance field with that constant value (`x.CONST`) |
| `var_merge_declared_type` | merges two variables of one slot that its lattice finds comparable although the LocalVariableTable declares them with different types: a `char c` and a later `int start` become one `char`, narrowing the `int` store (wrong output past 65535), and `Box<String>`/`Box<Integer>` one `Box<String>`, which javac rejects | they stay two variables, each with its declared type (`Box<Integer> boxx`, `int start`) |
| `compact_record_constructor` | takes a record's canonical constructor for a compact one when its last statement is `this.c = c` and removes every field store, so `this.node = Objects.requireNonNull(node)` vanishes (wrong output), along with a compact constructor's static field stores | compact only when the one non-throwing exit ends with every component's `this.c = c` in order (upstream 1506b2f4) on the constructor's own line, and only stores to fields of `this` are removed |
| `disjoint_generic_cast` | casts a returned or assigned value, or a field's initializer, to its target's parameterization even when the two generic types are provably distinct (JLS 4.5), which javac rejects: `(ImmutableSet<Class<? super T>>) builder.build()` of an `ImmutableSet<Class<?>>` | the cast is to the raw class, `(ImmutableSet) builder.build()`, as the source wrote it; arguments keep their cast (a raw argument would erase the call's result type) |
| `thrown_type_variable` | writes `throw (T) t` (an unchecked cast to the thrown type variable) as `throw t`, and a call of such a method on `new C<Error>()` on a raw `new C()` whose erased `throws` clause names `Throwable`, so javac reports an unreported exception | `throw (T)t` where a variable of a checked type no declared class covers is thrown and a thrown type variable admits it, and `RuntimeException` (what JLS 18.4 infers) for a raw receiver's thrown type parameters; Kotlin output is left alone |
| `kt_java_keyword_variable_name` | in Kotlin output, refuses a debug variable name that is a Java keyword but an ordinary Kotlin name (`new`, `default`), so the body reads a generated name the header does not declare: `fun readBomAsCharset(default: Charset)` reads `var1` (wrong output) | the debug name, as for any other name (`this` stays refused) |
| `kt_default_accessor_flags` | reads a property accessor's flags from the metadata unconditionally, so a setter whose flags kotlinc leaves out (equal to its property's) is `internal`, `final`: a public `var` gets `internal set` (wrong output), an `open var` `internal final set`, which kotlinc rejects | an absent accessor's flags are its property's default accessor flags, as Kotlin's reader takes them; nothing is printed for a default setter |
| `kt_unescaped_name` | in Kotlin output, the writers shared with Java print names bare that Kotlin writes in backticks (`$this$asList.length`, a lambda's ``{ in -> `in` }``, `this.L$0`, `access$getFoo$p()`), which kotlinc rejects | each goes through `toValidKotlinIdentifier` (`this` and `this@x` excepted) and an unnamed lambda parameter is `_`; a class's binary name (`Foo$Bar`), which Kotlin cannot name, stays |
| `kt_reserved_underscore_name` | in Kotlin output, writes a variable or parameter named `_` or `__` bare, which Kotlin reserves (`for (_ in this.readyQueues)`, `val _: Class`) | the name in backticks, which kotlinc 2.0 accepts there |
| `kt_java_type_syntax` | in Kotlin output, writes types from Java generic signatures in Java's syntax (`<T extends Comparable<? super T>>`, `int[]::class.java`) and drops the wildcard of a translated collection type (`MutableList<R>` for `List<? super R>`) | Kotlin's syntax: `<T : java.lang.Comparable<in T>>`, `where` for two bounds, `out`/`in`/`*` projections, `Array<T>`, `IntArray::class.java`, `Any` for `Object` in an argument |
| `field_read_all_paths` | Kotlin output reads a field again where a local held its value from before a call, since the inliner (`INLINE_ALL_VARS`) merges a field read's liveness by union across calling paths, catch and finally entries and calling finally bodies (`val v = h.count; try { work() } finally { use(v) }` prints `use(h.count)`) | the local stays: in that pass a field read is live only while it is live on every path, and a finally body that may change fields ends it |
| `kt_inline_evaluation_order` | Kotlin output evaluates a local's value later than the bytecode: the inliner moves it to its use in the next statement wherever that is, past other calls, into a branch or a `finally` (`val first = src.next(); src.take(src.next(), first)` prints `src.take(src.next(), src.next())`) | a value that calls, allocates or stores is inlined only into the next statement, where it is evaluated first and every time; an array element read, only ahead of any call or store there |
| `qualified_super_call` | writes `o.super(3)` (`o` an explicit outer instance of an inner member superclass) as `super(3)`, passing the enclosing instance (wrong output) or not compiling, keeping javac's check of `o`, `o.new Inner(3) {..}` as `new Outer.Inner(o, 3) {..}`, and an outside superclass's call as `super(Outer.this, ctx)` | `o.super(3)` when `o` is not the instance of the innermost enclosing class having the superclass as a member, or javac checked it; `o.new Inner(3) {..}` when the anonymous class passes its own parameter as the outer instance; javac's checks removed; for an outside superclass InnerClasses names an inner member class, the outer instance leaves the argument list (`super(ctx)`, or the qualifier) |
| `external_inherited_member_type` | learns inherited member classes only from supertypes in the input, so those of outside supertypes neither shadow nor make ambiguous an import of their simple name (Elasticsearch's `InternalFilters`: an ambiguous `implements Bucket` under an import of `Filters.Bucket`) | member classes of outside supertypes are read from the InnerClasses entries the supertype walk visits; a simple name two accessible inherited member classes share, or one shadows, is written qualified; Kotlin output is left alone |
| `lambda_return_type_argument` | writes explicit type arguments on a qualifier call from the value its variable got where the chain's result goes, contradicting a lambda or method reference argument: `ext.map(Ext::names).orElse(Collections.emptyList())` becomes `ext.<List<String>>map(Ext::names)`, which javac rejects for `Ext.names`'s `Collection` | type arguments that the implementation's erased return type contradicts (no subclass of the argument's class) are not written; javac infers them from the lambda or reference, as the source did |
| `type_annotation_outer_arguments` | writes an annotated class type with only the innermost class's type arguments: `CombinedFuture<V>.@Nullable CombinedFutureInterruptibleTask<?>` becomes `CombinedFuture.@Nullable CombinedFutureInterruptibleTask<?>`, type arguments on a member of a raw type, which javac rejects | each enclosing class keeps the type arguments the generic type gives it, `CombinedFuture<V>.@Nullable ...` |
| `pattern_binding_shape` | makes an `instanceof` and the cast stored at the head of its branch a type pattern whatever the bytecode, including a second field read (`if (this.f instanceof T) { T v = (T) this.f;`) and a cast after a negated test's exit, which javac compiles differently (wrong-form output) | a pattern only where the bytecode is javac's compiled pattern (the subject read once, `aload; checkcast; astore` right after the test's `ifeq`); otherwise the test and the cast stay as written |
| `finally_exit_versions` | the code after a `try` reads a local's value from before the `finally` body that assigned it, as the replay of the body's assignments on the normal exits looks the body up under a key no node has: Kotlin `var x = 1; try { .. } finally { x = 2 }; use(x)` prints `use(1)` | a normal exit of a verified `finally` takes the versions of every local the body assigns, except a slot the removed copies of the body keep elsewhere; the inliner does not move a value out of a `finally` body past its end |
| `var_merge_lost_definition` | prints a variable undeclared: `mergeVars` takes away a declaration hoisted to a statement before merging the variable into an earlier one of its slot, and when `remapVar` refuses (no common type) the declaration is gone (okio's `ByteStringKt`) | the declaration is taken away only for a merge `remapVar` can make; otherwise the variable keeps it |
| `head_definition_scope` | declares two variables of one name in one scope, which javac rejects: a declaration in a switch's or `synchronized`'s head block is written before the statement, but the clash renaming forgets it when the statement ends (okhttp's `Dispatcher.promoteAndExecute`) | the head block's declarations stay in scope after the statement, as an `if`'s do; the later declaration is renamed (`asyncCallx`, `ex`) |
| `switch_case_order` | sorts a switch's cases by key (the default last), where javac keeps the case bodies in the source's order and sorts only the keys: `case 'M' .. 'L' .. 'F' .. '-'` comes out `'-', 'F', 'L', 'M'` (equivalent code, but the recompiled bytecode differs) | in a class javac compiled, cases are ordered by the first instruction their keys jump to (the source's order), javac's synthetic default last; a class kotlinc compiled keeps Vineflower's order |
| `switch_table_fillers` | writes the keys javac fills a `tableswitch`'s gaps with as labels: `case 3: default:` for a source `default:`, and an empty last case `case 3: case 5: case 6: case 7:` (ghidra's `EnumDB.getRepresentation`) | a key inside the table's range that jumps where the default jumps is left out when javac would still emit the same table for the other labels; the table's lowest and highest keys stay, and so do the fewest of the lowest fillers javac needs to keep the table |
| `switch_synthetic_default` | writes javac's synthetic default of an exhaustive switch (`throw new MatchException(null, null)`, `IncompatibleClassChangeError` before Java 21) in a pattern switch and in an enum switch expression whose enum it cannot load, and leaves an enum switch expression whose cases return as a statement switch with that default | javac's default, recognised by its code and its place before every case, is left out of a switch expression and of a pattern switch statement; an enum switch with it whose cases return or throw is written `return switch` |
| `vararg_spread_resolution` | writes the array a varargs call passes as its elements although javac resolves the written call to another method or packs it differently: `build(new String[0])` inside `build()` comes out `build()`, a call to itself (wrong output) | a lone element converting to the array type is cast to the element type (`invoke(target, (Object)callbacks)`); the array is spread only when no other method of the name in the class or its known supertypes takes the written arguments without varargs or, with varargs, beats it by arity |
| `unseen_class_overload` | writes every implicit argument conversion in a call to a class outside the input implicitly, blind to its overloads: `value((double) f)` comes out `value(f)`, which calls `value(float)` (wrong output) | the methods of that name and class the input calls stand in for its overloads, so an argument whose parameter type differs from one of them that javac would choose instead keeps the bytecode's type (a cast, boxing or unboxing call) |
| `double_constant_literal` | writes a `double` constant that a `float` holds exactly as the shorter `float` literal or `Float` constant, whose type the code then takes: `((double) bytes) / 1073741824.0` comes out `bytes / 1.0737418E9F`, a `float` division (wrong output) | the constant is written as a `double` literal (`1.073741824E9`) |
| `concat_operand_type` | rebuilds a string concatenation without the type its `append`/`makeConcatWithConstants` descriptor converted each argument as: `(closed ? '[' : '(')` comes out `(closed ? 91 : 40)`, `String.valueOf(cs)` of a `char[]` comes out `+ cs` (wrong output) | an operand of another kind than its argument (`char`, `int`, `long`, `float`, `double`) is cast to it; integer constants of a `char` argument are written as characters; a `char[]` is written `String.valueOf(cs)` |
| `field_read_reuse` | copies a field read the bytecode `dup`s into each of its uses, so the field is read twice: `while ((i = lock.count) > 0)` comes out `int i = lock.count; if (lock.count <= 0)`, two reads of a volatile field another thread writes (wrong output) | the value stays in its stack variable as the source's assignment expression (`while ((i = lock.count) > 0)`); in Java output no non-final field read is copied into its uses, kotlinc's `x?.f` and `x ?: y` included, but Kotlin's `x++` keeps Vineflower's folding |
| `overlapping_ranges` | fails a kotlinc 1.3 method with a `catch` inside a `try`/`finally` or `use` block whose `try` returns, as the `catch` range also covers the `finally` copy inlined before the `return` and the ranges overlap without nesting (`parsing failure!`) | the method decompiles: the `return` path gets its own `try`/`catch` with copies of the handler, the outer ranges are split into nesting `try`s, and a `finally` whose inlined copies cannot all be removed gets a semaphore variable |
| `constructor_signature_alignment` | written as Java, fails a method creating an anonymous or local object of a kotlinc 1.3 class whose constructor's `Signature` leaves out only the outer instance, aligning the parameters as if it also left out the captured variables (`index out of bounds`) | the method decompiles; the signature is matched to the parameters it describes |
| `split_constructor_call` | writes `X v = new X;` and `v./* $VF: Unable to resugar constructor */<init>(args);`, neither Java nor Kotlin, when the argument evaluation branches (Kotlin's `a!!` from kotlinc 1.3, `?: throw`), joining an allocation and its constructor call only within one statement list | `v = new X(args)` after the branches (the allocation moves past the argument evaluation, as Vineflower's own join moves it past the statements between, so `X` is initialised after the arguments, and not on a path that leaves them) |
| `local_final_writes` | misses the writes in statement heads (`if (++count > limit)`) and counts the one write in a loop body of a local declared before the loop as its only one, so Kotlin output declares the local `val` and kotlinc rejects the reassignment (okhttp's `RetryAndFollowUpInterceptor`) | heads are read where they run and a loop body has its own set of locals written once: such a local is a `var` |
| `kt_not_null_assertion` | deletes kotlinc's `Intrinsics.checkNotNull` of a `!!` and keeps Kotlin 1.3's `if (x == null) Intrinsics.throwNpe()`, so the value stays nullable where the source made it non-null and kotlinc rejects its use (`val var10000: ResponseBody = response.body`) | the check is folded back into `x!!` on the expression it guards (`response.body!!.source()`, `head = head.next!!`, `a!!.b!!.c`) |
| `kt_expression_value_check` | deletes kotlinc's `checkNotNullExpressionValue` of a platform-typed value stored into a property only after simplifying stack variables, and keeps Kotlin 1.3's `checkExpressionValueIsNotNull`, so the stack variable stays and a static property loses its initializer | the check of a value stored into a field is deleted first, so the value goes into the store (`val logger: Logger = Logger.getLogger(..)`); a check of a value used elsewhere is kept |
| `kt_object_instance_local` | reads Kotlin 1.3's object instance local in the object's static initializer as is: printed, it creates a second instance (`val var0 = Hpack(); INSTANCE = var0`), and it keeps the object's properties from their initializers (wrong output) | the local is read as `INSTANCE`, as kotlinc 2.0 compiles it: `NAME_TO_FIRST_INDEX = Hpack.nameToFirstIndex()` is a property initializer again |
| `kt_static_initializer` | prints what is left of a Kotlin static initializer as `@JvmStatic fun {`, which is not Kotlin (okhttp's `ConscryptPlatform`, `Huffman`) | an `init` block of the object, or of the companion object of a class; a file class's remainder is left as it was |
| `kt_property_initializer_key` | looks a property's initializer up under a descriptor made from its Kotlin type (`Lkotlin/Array;`, `Lkotlin/UInt;`) instead of its field's, and loses it: `private val APPROVED_CIPHER_SUITES: Array<CipherSuite>` has no value (wrong output; okhttp's `ConnectionSpec`) | the initializer is found under the field's own name and descriptor |
| `kt_synchronized_accessor` | drops the `synchronized` flag of property accessors (`@get:Synchronized var maxRequests`, okhttp's `Dispatcher`): recompiled, the accessor takes no lock (wrong output) | the property is annotated `@get:Synchronized` / `@set:Synchronized` |
| `inline_marker_finally_copy` | in a Kotlin inline function's body, compares kotlinc's `InlineMarker` depths around `finally` copies as constants, rejecting a non-local `return`'s copy and falling back to a semaphore that for `synchronized` writes `synchronized (x){} // $VF: monitorenter` with the body outside the lock (wrong output, Java and Kotlin) | the depths are not compared: the copies are removed as a `finally`'s, and the lock is a `synchronized` block around the body (`synchronized (this) { .. return null .. }`) |
| `synchronized_break_exit` | keeps the lock release kotlinc places after a `synchronized`'s range for a `return` inside it when a `break` from more than one level inside targets that block: `// $VF: monitorexit` after `varN = x; break`, then `return varN` (equivalent: the `synchronized` releases on the `break`) | the release is the `synchronized` statement's own and is dropped: `return x` inside the block, as in the source |
| `kt_unrepresentable_declared_type` | in Kotlin output, writes the type of a local `val`/`var` or a private property whose type is an anonymous class (an `object` expression, kotlinc's class for a callable reference or lambda) as `<unrepresentable>`, which is not Kotlin | when an initializer follows, no type: Kotlin infers it (`val var10003 = JobSupport::registerSelectForOnJoin`) |
| `kt_inline_lock_local` | the Kotlin plugin's inliner copies a `synchronized` lock local's value into its head and keeps the local for its other uses (`val l = this.lock; synchronized (this.lock) { l.f() }`), so the body can hold another monitor than the bytecode takes (wrong output, Kotlin) | a lock local whose value may be another object when evaluated again (not a constant, a local or a final field) keeps every use: `synchronized (l)`; one with no other use still moves into the head |
| `synchronized_lock_assignment` | takes any assignment in a `synchronized`'s head block for the compiler's lock variable: `Object l = this.lock; this.lock = new Object(); synchronized (l)` becomes `this.lock = new Object(); synchronized (this.lock)` (wrong output, Java) | only an assignment followed by stores of constants or locals to other locals is the lock variable's, and a value the head repeats is dropped only if it has no effects |
| `kt_facade_call` | in Kotlin output, calls top-level functions and reads top-level `@JvmField`/`const` values through their facade class (`Util.closeQuietly(x)`), which Kotlin cannot name, ignores multifile facade parts' metadata, and writes a top-level property's accessor call without its import, reading a same-named local in scope (wrong output) | the callee is found in the parts (on the class path for a library) and written by its Kotlin name with its import, or qualified with the package (`okio.buffer(..)`) where another import, a member or a local of that name would be taken |
| `kt_extension_call` | in Kotlin output, writes an extension function call with its receiver as an argument (`Util.closeQuietly(x)`) and an extension property's accessor call without its receiver (`tag = s` for `s.tag = 4`, assigning the receiver: wrong output) | the call goes on its receiver with its import (`x.closeQuietly()`), the receiver cast where its type would pick another extension or a member, and a boxed primitive receiver given a type argument where an extension taking the primitive would be called (`this.coerceIn<Int>(range)`); it keeps its form where a member could still be taken, or for value classes |
| `kt_jvm_name_call` | in Kotlin output, writes member calls by JVM name (`` `bump$main`() `` for an `internal` member, `url.url()` for a `@JvmName`) and reads only the named class's metadata, so inherited members keep JVM names and inherited properties accessor calls (`this.getClosed()`) | the function's Kotlin name (`bump()`, `url.toUrl()`, `bytes.get(i)`), the metadata of the declaring class read for an inherited member; a name a value class mangled keeps its JVM name |
| `kt_synthetic_accessor_call` | in Kotlin output, writes the calls of kotlinc's synthetic accessors of private members made outside the accessor's class (`Http2Connection.access$getPushObserver$p(c)`, `TaskLoggerKt.access$log(..)`), which Kotlin has no name for | the call is the access the accessor makes (`c.pushObserver`, `log(..)`), when its code is one field access or call of a member of its class with Kotlin metadata; a field access only of a property that is not delegated and whose accessor for it is the default one |
| `kt_nullable_local` | in Kotlin output, declares every local with the non-null form of its JVM type, also one that holds `null`, `var s: Segment = this.head` for a `Segment?` `head`, which kotlinc rejects | a local is `T?` when a store may put `null` in it (the `null` literal, a nullable field, call, parameter or local, `Map.get`/`put`/`remove`), but `T` for a value stored after the method's own null check (`?: return`) |
| `kt_nullable_cast` | in Kotlin output, writes the `checkcast` of a value whose Kotlin type is nullable as `as T`, which throws `NullPointerException` on `null` where the bytecode lets it through: `names.get(key) as String` before the elvis of `names[key] ?: return -1` throws on a missing key (wrong output) | `as T?`, unless the cast is dereferenced at once (a call, a field, an element on it), which throws on `null` either way |
| `external_supertype_arguments` | reads the type arguments a class gives a supertype through the supertype's own class file, so one outside the input gives none and the unchecked-cast fixes write no cast: `return IdentityFunction.INSTANCE` from a `Function<E, E>` method, which javac rejects | the arguments are read from the generic supertypes the known classes' signatures declare, `(Function<E, E>) IdentityFunction.INSTANCE` |
| `bounded_inference_cast` | writes bare a returned or assigned generic call whose result's type variables only the target fixes, where the target puts a variable outside its bound, which javac rejects: `Ordering<E> o = Ordering.natural()` (`<C extends Comparable>`) for an unbounded `E` | the call is cast to the target, `(Ordering<E>) Ordering.natural()`, or to its raw class where the type argument is a class |
| `receiver_type_arguments` | writes a generic call or diamond creation that starts a returned or assigned chain without type arguments, which javac infers from its arguments alone: `return List.of(MODE).iterator()` for an `Iterator<Setting<?>>` (Elasticsearch's `settings()` iterators) | the type arguments the target needs, `List.<Setting<?>>of(MODE)`, `new Builder<E>(comparator)`, unless the arguments give javac exactly those |
| `argument_generic_cast` | writes an argument bare whose parameter's declared type does not accept it, which javac rejects: `out.writeGenericMap(innerMap)` of a `Map<String, String>` (Elasticsearch's `DiffableStringMap`) | the unchecked cast the returned-value fixes write, through the raw class where the two types are provably distinct, `(Map<String, Object>) (Map) innerMap` |
| `kt_field_read_reuse` | in Kotlin output, re-reads a field the bytecode reads once for an elvis or safe call (`if (this.head == null) throw ..; return this.head.pos`) and copies a named local's field value into its uses, though another thread may write it in between and kotlinc rejects the re-read nullable property's use (wrong output) | a non-final field's value stays in its variable, which the check and the uses read; a final field and a read followed in its block by a store into the field (Kotlin 1.3's `x++`) keep the copies |
| `kt_partial_inline` | in Kotlin output, inlines a variable's value into the uses it can and keeps the variable for the others, so a check reads another variable than the one it guards, a cast or an element is evaluated again, and kotlinc cannot smart-cast the variable | a variable that stays is read at every use, as the bytecode reads it; one whose uses all take its value is inlined as before |
| `kt_file_static_initializer` | in Kotlin output, writes what `InitializerProcessor` leaves of a file class's static initializer as `fun { .. }`, not Kotlin, and the properties it sets have no value (okhttp's `okHttpName`) | the remainder is cut at each store into a file property, the code before it and the value becoming that property's initializer (`val x: T = run { ..; value }`) in `<clinit>`'s order, unless a property is set inside a nested statement |
| `kt_property_field_flags` | Kotlin output drops a property's `@Volatile`, `@Transient` and `@JvmSynthetic`, source-retained annotations whose only trace is the backing field's flag, so a recompiled `@Volatile var state` is a plain field (wrong output, Kotlin; okhttp's `Http2ExchangeCodec.stream`) | the property carries the annotation its field's flag stands for (`@delegate:Transient` on a delegated one) |
| `kt_jvm_field_setter` | writes `private set` under every non-private `var` without a setter method, so a `@JvmField var`, which has no accessors, comes out `@JvmField var x: T .. private set`, which kotlinc rejects (okio's `Buffer.head`) | a `var` whose backing field is not private (kotlinc's `@JvmField`) has no setter line |
| `kt_inline_heap_read` | the Kotlin plugin's inliner (`INLINE_ALL_VARS`) copies a local that reads an array element, or computes from field reads, into each use: `val e = slots[i]; if (e != null) e.value` comes out `if (this.slots[i] != null) this.slots[i].value`, a second read another thread may change (wrong output, Kotlin) | such a value with more than one use stays in its local; a final field of an unchanging value, kotlinc's `x as? T` and a single use still move |
| `enum_constant_in_values_array` | drops the constructor arguments of enum constants that Kotlin 1.3's JVM backend builds inside the `$VALUES` array (`dup; putstatic F; aastore`), printing `NO_ERROR,` for `NO_ERROR(0)` (lost code, Java and Kotlin output; okhttp's `ErrorCode`) | each constant's `new E(..)` becomes its initializer when the array's elements build the enum's constants in declaration order with their own names and ordinals: `NO_ERROR(0)` |
| `kt_extension_property` | in Kotlin output, declares an extension property without its receiver and type parameters, `val lastIndex: Int` for `val CharSequence.lastIndex: Int`, its accessors reading the receiver by its debug name `` `$this$lastIndex` `` | the header is `val <T : B> Receiver.name: Type` from the metadata, the accessors read the receiver `this` (the class instance `this@C` in a member extension) |
| `kt_interface_companion_field` | in Kotlin output, declares an interface's companion field in Java form, `Interceptor.Companion Companion = Interceptor.Companion.$$INSTANCE;`: kotlinc initializes it from the companion's `$$INSTANCE`, not with `new`, which the plugin hides | the field is hidden as a class's companion field is; the `companion object` declares it |
| `kt_generated_overload_hidden` | in Kotlin output, writes each overload kotlinc generates for an `@JvmOverloads` function as a function, `@JvmOverloads fun substring(beginIndex: Int): ByteString { return substring$default(this, beginIndex, 0, 2, null) }` | not written: the annotated function declares them (the methods kotlinc generates for constructors are `kt_generated_constructor_hidden`'s) |
| `kt_default_args_bridge_lookup` | in Kotlin output, looks a function's default-arguments method up by Kotlin name and Kotlin-type descriptor, never in `DefaultImpls`, where kotlinc uses the JVM ones, so the defaults of `@JvmName`, `internal`, `suspend` and interface functions, among others, print as `= ...`, which kotlinc rejects, and a function whose Kotlin name is another's JVM name gets its defaults | the method kotlinc emits, `<JVM name>$default` with the function's JVM parameters (after a member's receiver object), the masks and the marker, in the class or its `DefaultImpls`; all those defaults are printed |
| `kt_default_value_names` | in Kotlin output, names a default value's variables from the local variable table: an extension receiver by its debug name (`` `$this$toByteString`.size ``), the instance `this` also where `this` is the extension receiver, an abstract function's parameters `var1` | a parameter by its name in the metadata, the extension receiver `this`, the instance `this`, or `this@Class` in an extension (`width: Int = this@Box.length`) |
| `kt_default_value_coercion_cast` | in Kotlin output, prints the cast to the parameter's type that kotlinc compiles into every default value (`= null as MediaType?`, `= ", " as CharSequence`, `= ArrayList() as java.util.List`, which kotlinc rejects) | the default without the cast (`= null`, `= ", "`, `= ArrayList()`), which compiles to the same bytecode; a cast of a lambda is kept |
| `kt_delegated_member_default` | in Kotlin output, declares a member kotlinc generates for interface delegation (`: Shape by shape`) with `= ...` for each parameter K1's metadata marks as declaring a default (`cancel(cause: Throwable? = ...)`), which an override cannot declare | the parameters without defaults: an override takes them from the function it overrides |
| `ambiguous_type_variable_argument` | casts an argument it must cast to pick an overload to its type variable's inferred type, which makes javac call a more specific overload taking that type: `appendTo((StringBuilder)builder, parts)` makes Guava's `Joiner.appendTo(StringBuilder, Iterator)` call itself until the stack overflows (wrong output) | where the call as written would resolve to another method, the cast names the variable's erasure, and the call's result gets the cast the bytecode makes: `appendTo((Appendable)builder, parts)` |
| `unboxed_argument_type` | casts an argument written without its `Byte.valueOf`/`Short.valueOf` call to the generic parameter's inferred type, so a `byte` or `short` constant loses its narrowing cast or gets the other's: `Map.of(.., byte.class, (byte) 0, ..)` prints `byte.class, 0`, boxed as `Integer` (wrong output) | the argument keeps its primitive type: `byte.class, (byte)0, short.class, (short)0` |
| `kt_enum_when_mappings` | in Kotlin output, writes a `when` over an enum as kotlinc compiled it, over its synthetic `$WhenMappings` array with numeric labels (`` when (LazyKt__LazyJVMKt.WhenMappings.`$EnumSwitchMapping$0`[mode.ordinal]) { 1 -> .. } ``), which kotlinc rejects | `when (mode) { LazyThreadSafetyMode.SYNCHRONIZED -> .. }`, the constants read from the mapping class's static initializer, `null` for a nullable subject's `-1`, the default's filler labels left to `else` |
| `kt_constructor_accessor_call` | in Kotlin output, writes a call of the synthetic constructor kotlinc generates to reach a private constructor from the companion object or a nested class (or one with an inline-class parameter), `CipherSuite(javaName, null)`, a constructor the class does not declare | the call of the constructor the accessor forwards to, `CipherSuite(javaName)` |
| `kt_empty_primary_constructor` | in Kotlin output, writes no `()` for a primary constructor without parameters, so a class with secondary constructors reads as having no primary one and `constructor(e: ExecutorService) : this()` names none (okhttp's `Dispatcher`) | `class Dispatcher() {` when the class has secondary constructors |
| `unlisted_anonymous_class` | nests a class only through its enclosing class's `InnerClasses`, which omits kotlinc's `$$inlined$` copies of inline functions' objects and lambdas, so each is written as a top-level class reading captured values from fields and created by its `$` name, which kotlinc rejects | such a class is nested in the class its `EnclosingMethod` names and written where created, its captured values read as the creating method's variables (`object : Interceptor { .. block(chain) .. }`); a suspend lambda (in Kotlin output) or a copy created in an unwritten method stays a class of its own |
| `kt_companion_nameless_class` | in Kotlin output, finds a class's companion object among its nested classes by name and throws on an anonymous class that sorts before it (`Foo$1`, an `init` block's object), failing the whole class | anonymous classes are skipped |
| `refresh_printed_var_names` | renames a nested class's method variables apart from its captured values by their own names after the clash pass gave a local sharing a parameter's name a clashing name, so the parameter takes the local's printed name: `var countx: Long = countx` for `var count = count` (okio's `Pipe`) | variables with a clashing name keep it and the others are renamed around it |
| `kt_implicit_widening` | in Kotlin output, writes a `Byte`, `Short` or `Char` value (or an `Int` behind an `i2l` it leaves implicit) stored, passed or returned where a wider type is expected as it is, Java's implicit widening: `val n: Long = source.readShortLe()`, which kotlinc rejects | the value takes the conversion the source wrote, `this[i++].toInt()`, `c.code`, `readShortLe().toLong()`: the same instructions |
| `kt_object_monitor_call` | in Kotlin output, calls `java.lang.Object`'s `wait`, `notify` and `notifyAll` on a Kotlin-typed receiver, `lock.notifyAll()`, which Kotlin's `Any` does not have (in okhttp's `inline fun Any.wait()` the call resolves to the extension itself) | `(lock as java.lang.Object).notifyAll()`, as the source writes it |
| `kt_raw_generic_type` | in Kotlin output, writes generic classes without type arguments, which Kotlin has no form for: a local's erased JVM type (`val pins: java.util.List = findMatchingPins(host)`), casts and type tests, so kotlinc rejects every member read through such a local | a local gets the arguments the class file proves (`List<Pin>`, `MutableIterator<String>` where the method mutates through it, `Iterator<*>` where nothing proves them and nothing writes through it), casts take those of the local or return type they feed, type tests star projections; casts made redundant are left out |
| `kt_escaped_class_name` | in Kotlin output, writes a class whose Kotlin name is no identifier bare wherever the class is not declared, ``public val Okio: -DeprecatedOkio = -DeprecatedOkio`` (okio), which kotlinc rejects | each such name is written in backticks, as its declaration is: ``public val Okio: `-DeprecatedOkio` = `-DeprecatedOkio` `` |
| `kt_inner_class_constructor` | in Kotlin output, writes the construction of an `inner` class on the current instance as Java does, through the outer class's name, `return Cache.RealCacheRequest(editor)`, which Kotlin reads as a nested class's constructor and rejects | `return RealCacheRequest(editor)`: the current instance is the implicit receiver, as the source writes it |
| `kt_data_class_property_parameters` | writes a data class's primary constructor with plain parameters, its properties in the body with their stores in `init`, and the members kotlinc synthesizes (`componentN`, `copy`, `equals`, ..), which kotlinc rejects (a constructor parameter that is no property, conflicting declarations) | the constructor declares each property whose field it stores first, `data class Header(@JvmField public final val name: ByteString, ...)`, without the store, and the synthesized members are left to Kotlin |
| `kt_function_parameter_name` | writes a function's or constructor's parameters by their metadata names but reads them in the body by the local variable table's, which a method without one (kotlinc 1.3's data class `equals`) does not have: `equals(other: Any?)` whose body reads `var1` | the body reads such a parameter by the header's name, `return other is Header && ...` |
| `kt_contract_import` | writes a function's contract (`contract { callsInPlace(block, InvocationKind.EXACTLY_ONCE) }`) without importing `kotlin.contracts.contract` and without the opt-in to `ExperimentalContracts` the source gives (`@OptIn` has source retention), both rejected by kotlinc | `contract` is imported (or qualified where the name is taken) and the function is annotated `@OptIn(ExperimentalContracts::class)` |
| `kt_unlabeled_sequence_break` | in Kotlin output, writes an unlabelled `break` whose closure is a sequence as `return@run`, though only a labelled sequence is written `run labelN@{ .. }`, so there is no `run` to return from (okio's `Pipe.sink.write`) | `break`, as Java writes it: the innermost loop ends where the jump goes |
| `kt_legacy_parameter_checks` | in Kotlin output, keeps the Kotlin 1.3 parameter null checks opening a function's body (`Intrinsics.checkParameterIsNotNull(sink, "sink")`), which kotlinc emits again, and in an inline function passes its lambda parameter as a value ("illegal usage of inline parameter") | the checks that open the function's first block, each of a parameter written with a non-null type, are dropped as Kotlin 1.4's are; a check after other code stays |
| `kt_inlined_to_typed_array` | in Kotlin output, writes kotlinc's inlined `toTypedArray()` as its body, `c.toArray(arrayOfNulls(0))` (unresolved on a Kotlin collection) into an `Array<Any>` temporary, with Kotlin 1.3's `TypeCastException` null check and a cast at each use | the call is written `c.toTypedArray()` (`toTypedArray<Any>()` where the array's class is not the receiver's element type), the null check removed and the casts left out: the same instructions |
| `kt_narrow_constant_local` | in Kotlin output, declares a local that only integer constants are stored into by the smallest type that holds them, `var v: Byte` for `20`/`10`/`30`, where the source's local or kotlinc's temporary was an `Int`: `repeat(v)` and `arrayOfNulls(v)` are rejected | such a local is typed `Int` (its constants too) when every read of it takes an `Int`; a local read as a `Byte` keeps its type |
| `kt_suspend_state_machine` | in Kotlin output, writes a `suspend` function that suspends as kotlinc's state machine (continuation object, spill fields, `label` dispatch, `COROUTINE_SUSPENDED` checks), which kotlinc rejects, and a machine-less one's tail suspend call as `return if (var10000 === COROUTINE_SUSPENDED) var10000 else Unit` (Vineflower fails the machines; kuna wrote them since `kt_synthetic_object_super_call`) | the function's direct body as its source has it (spilled locals as the locals they saved, the resumed `result` as the call's value); a function whose code also uses the continuation otherwise (an inlined `suspendCoroutineUninterceptedOrReturn`) keeps the machine |
| `kt_suspend_call_continuation` | in Kotlin output, passes the continuation kotlinc adds to a suspend call, `src.next(`$completion`)`, reads `coroutineContext` as `` `$completion`.context ``, which Kotlin cannot name, and keeps the cast kotlinc puts on the call's `Object` result, nullable (`src.text(n) as String?`) | the call Kotlin writes, `src.next()`, a qualified `kotlin.coroutines.coroutineContext` (in a `CoroutineScope` the simple name is the scope's property), and no cast to the callee's declared return type, the result as nullable as that type |
| `kt_coroutine_boxing` | in Kotlin output, writes the boxing kotlinc emits in coroutine code as a call of `kotlin.coroutines.jvm.internal.Boxing`, `return boxInt(count)`, which is internal to the standard library | the value, boxed implicitly as Kotlin writes it (kotlinc boxes it with `Boxing` again) |
| `kt_suspend_function_type` | in Kotlin output, writes a suspend function type as the function type it compiles to, `(T, Continuation<R>) -> Any?`, ignoring the metadata's `suspend` flag | `suspend (T) -> R`, `(suspend () -> Unit)?` |
| `kt_inlined_stdlib_call` | in Kotlin output, writes kotlinc's inlined copy of a standard-library inline function as the copied body, which calls the library's `internal` helpers kotlinc rejects (`cannot access`): `toString(radix)` as `Integer.toString(x, checkRadix(r))` | a copy that is exactly the function's body, outside the `kotlin` package, is written as the call (`b.toString(16)`, `r.use { it.text() }`), which kotlinc compiles to the same instructions; other copies stay as they are |
| `kt_companion_static_copy` | in Kotlin output, writes a class without its own members that its companion object overrides or shadows: writing the companion marks every method of the class with a companion member's JVM signature as the companion's `@JvmStatic` copy (okhttp's `Android10SocketAdapter` lost `isSupported()`) | only a static method is taken for the companion's copy; the class declares its own members |
| `kt_reused_inline_object` | in Kotlin output, fails a method that creates an inline function's anonymous object kotlinc did not copy (the caller passes on its own value): the `new` is written as the object's body, which belongs to the inline function | the creation is written as the inline function's call when the function's code is only that creation from its parameters: `sortedWith(compareBy(selector))` |
| `kt_nothing_value_throw` | in Kotlin output, writes the `throw KotlinNothingValueException()` kotlinc emits after a call of a function returning `Nothing` (dead code: the call never returns), which kotlinc rejects (the class is `@PublishedApi internal`) | the call alone, which ends the branch as in the source; kept where the callee is not declared `Nothing` (a type parameter or function type instantiated with it) |
| `kt_suspend_without_metadata` | in Kotlin output, writes a suspend function of a class without function metadata (a SAM-converted suspend lambda's `emit`) from its JVM signature with its state machine, `fun emit(value: T, $completion: Continuation<in Unit>): Any` | `override suspend fun emit(value: T)`: a suspend-convention method overriding a function a supertype's metadata declares `suspend` (or whose synthetic bridge does) is one, with that function's result type, rebuilt by `kt_suspend_state_machine` and `kt_suspend_call_continuation` |
| `kt_for_inline_copy` | in Kotlin output, writes the `$$forInline` copy kotlinc adds to an object's suspend function in an inline function (its inliner's template, called by nothing), with its continuation parameter and `InlineMarker` calls (once kuna's fixes decompile it; Vineflower fails these and leaves them out) | the copy is not written, as the plugin leaves out kotlinc's other generated members |
| `kt_suspend_lambda` | in Kotlin output, writes a suspend lambda (a `SuspendLambda` class, `flow { }`, `launch { }`) as `{ // $VF: Could not decompile lambda - root function was not found. Is this a suspend lambda? }` cast to its `FunctionN` interface | the lambda with its body rebuilt from `invokeSuspend`'s state machine, its parameters as the metadata names them, a receiver as `this@label`, its exits as the lambda's value or `return@label`, without the cast |
| `kt_suspend_impl` | in Kotlin output, writes an open suspend function as a call of the static synthetic `foo$suspendImpl(this, ..)` kotlinc moved its body into, which the plugin hides, so the body is lost | the function with the body of `foo$suspendImpl`, its first parameter as `this` (`this@Class` where a lambda captures it), rebuilt as a suspend function's |
| `kt_class_header_metadata` | in Kotlin output, takes a named class's type parameters and supertypes from the JVM signature, without variance (`interface Flow<T>` for `Flow<out T>`), nullability or `Nothing`, and with Java's collection names and kotlinc's marker interfaces | the header declares what the class's metadata does (`interface Flow<out T>`), and kotlinc's stubs for Java's mutators of a read-only collection are left out where it names Kotlin's interface |
| `kt_enclosing_type_parameter` | in Kotlin output, writes an enclosing function's or class's type parameter in an object expression's metadata as `Any`, the metadata naming it by an id it does not declare: `object : Flow<Any>` inside `fun <T> Iterable<T>.asFlow(): Flow<T>` | the id is named after the type variable at the same place in the class's JVM signature when that is the enclosing method's or class's type parameter (`object : Flow<T>`), also for the object's locals; a copy inlined with a class for `T` keeps `Any` |
| `kt_default_impls_delegate` | in Kotlin output, writes the methods kotlinc generates (without JVM default methods) to call an interface member's default body, `override fun plus(other: Job): Job { return Job.DefaultImpls.plus(this, other) }`, though Kotlin cannot name `DefaultImpls` | a method the metadata does not declare that only passes `this` and its parameters to the same-named `DefaultImpls` method of an implemented interface is left out, as is a `DefaultImpls` class left with nothing else: the class inherits the member and kotlinc generates the method again |
| `kt_inline_class_value` | in Kotlin output, writes a value class's compiled plumbing (`Meters.box_impl(m)`, `Meters.plus_HWDY4ac(a, b)` for `a + b`) and a value of a value class type as itself where the code reads its underlying value: `println(m)` for `println(m.v)`, which kotlinc compiles to the value class's `toString` | each expression is typed by what the class files declare and written as the value class's code (`Meters(x)`, `a == b`, `m.v` where the code reads the underlying value); a call whose callee has no metadata keeps Vineflower's text |

Every fix is a row of `decompiler/nonnative/kuna-jvm/fixes.toml` (its id and family), from
which `build.rs` generates its `KunaFix` variant; each site where kuna departs from the Java asks
`kuna_fixes::enabled(KunaFix::..)` and keeps Vineflower's code verbatim in the other
branch, which compat mode takes. A unit test that must isolate one fix runs with
`KunaFixes::DEFAULT.without(KunaFix::X)` (through `Fernflower::new_with_fixes`, or a corpus
`Case`'s `fixes`); users cannot. Spec section 10.16 has the gate and the mechanisms groups
of fixes share; `tests/jvm/kuna-fixes/README.md` lists each fix's case.

### Source forms

Some departures from Vineflower's text are not corrections but choices between two
equivalent forms of the same code: Vineflower's, and the one the class file shows the source
had. kuna always prints the source's; a case of one in `tests/jvm/kuna-fixes/` is labelled
with the module that decides it.

| Module | Vineflower 1.12.0 | kuna |
|---|---|---|
| `kuna_source_locals` | folds a local the LocalVariableTable names into its one use (`result = 31 * result + x; return result;` as `return 31 * result + x;`), declares each variable at its first use, so javac renumbers the slots, splits a reassigned local into `x` and `var3`, and folds an increment into the next expression (`a[i] = i; i++;` as `a[i] = i++;`) | the locals stay as the source wrote them, declared (in Java output) where javac gives every local the class file's slot and in the scope the slots prove the source declared them in (spec section 10.10) |
| `kuna_source_patterns` | makes `if (x instanceof T) { T v = (T) x; ..}` the pattern `x instanceof T v` (javac compiles both alike for a local `x`) | in a class javac compiled, the test and the cast stay where the LineNumberTable gives the cast a line of its own after the test's (spec section 10.10) |

Their effect on the benchmark is in `docs/jvm-benchmark.md`.

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
   dump-exception-on-error=1 ...` with `kuna decompile-project DIR --option include-runtime ''
   --option jvm_kt_enable 0 --option jvm_resugar_idea_notnull 0 --option jvm_vineflower_compat 1
   -o NEW_DIR` (`--option` takes Vineflower's option names and kuna's own rows by their
   names). For the Rust backtrace of the caught
   throw, set `KUNA_JVM_BACKTRACE` to a piece of its message (`1` prints every caught
   throw, and the engine throws and catches routinely); build with
   `CARGO_PROFILE_RELEASE_DEBUG=line-tables-only` for line numbers.
2. **Diagnose in the Java.** Read the method in Vineflower's sources at the release (tag
   `1.12.0`; `tools/oracle/fetch_vineflower.sh` checks it out) and, in a full clone, what
   upstream did since: `git log 1.12.0..origin/develop/1.13.0 -- <file>`, and `git log
   --all -S'<code>'` for the commit that introduced the defect. Port upstream's fix when there is one. The fix goes
   to the root cause, in the terms of Vineflower's algorithm: never a catch-and-continue,
   never keyed on a class or method name.
3. **Add the fix.** A row `{ id = "<id>", family = "<family>" }` in `fixes.toml`, among its
   family's (lower snake case, no `jvm_` prefix; `kotlin = true` for a fix of Kotlin
   output), which makes `KunaFix::X` (the id in CamelCase), and the new count in the sentence
   above the table (`tests/kuna_fixes.rs` checks it). There is no
   option row: a fix is not a user option, and `jvm_vineflower_compat` already turns it
   off with the others. At the site: `if kuna_fixes::enabled(KunaFix::X) { ... } else {
   <the Java, unchanged> }`. A fix longer than a few lines goes in its family's module,
   `src/fixes/<family>.rs` (or `src/fixes/<family>/<id>.rs` in a family with a folder),
   called from the gated branch. Code that runs
   before `Fernflower::new` installs the context reads the set it is building
   (`fixes.contains(..)`), as `report_to_kuna` does.
4. **Add the case.** Compile the reproducer and run `python3 -I
   tests/jvm/kuna_fix_case.py --fix <id> --class DIR/pkg/TestFoo.class --source
   pkg/TestFoo.java --compiler "javac 17.0.20.1" --summary "<what decompiles now>"`. It
   vendors the class and its `TestFoo$*.class` siblings into `tests/jvm/kuna-fixes/`,
   records the digest of what the release jar prints (the compat expectation, which must
   contain the failure comment) in `cases.json`, checks that kuna in compat mode prints
   exactly that, stores kuna's default output, prints the diff for review, and rebuilds
   `cases.tsv` and the READMEs with `regen.py`. The case must also pass `tests/fix_isolation.rs`, which turns its switch off
   alone: a Java case must then print its compat text, and a Kotlin or form-option case
   something other than its default text. A Java case that a second fix also changes names
   that fix in the test's `PARTNERS` table; a Kotlin case whose fix another one masks names
   that one in `MASKED_BY`.
5. **Describe it.** A row in the table above: the defect, what kuna does instead, one
   sentence each. Spec section 10.16 changes only when the fix adds a mechanism that other
   fixes share or changes an invariant of the gate.
6. **Evidence**, quoted in the commit: (a) the root cause, as above; (b) the reproducer's
   default output recompiles with javac and every method's `javap -c -p` matches the
   original class's; (c) no collateral change: `kuna decompile-project IN -o COMPAT
   --option jvm_vineflower_compat on`, `kuna decompile-project IN -o DEFAULT`, then
   `diff -ru COMPAT DEFAULT`, on the five benchmark jars (`docs/jvm-benchmark.md`) and on
   okio, okhttp and kotlin-stdlib: every hunk must be inside a method that failed in compat
   mode, a tracker note or an import, or be the fix's own correction; (d) `make
   test-jvm`. Its compat-mode runs must still match; a fix that changes kuna's default
   text on Vineflower's own inputs moves the digests of their default-mode runs
   (`kuna/<id>`, printed with both digests). Read each moved text (`kuna test --jvm --name
   kuna/<id> --out DIR`), re-record them with `kuna test --jvm --save-digests`, and name the
   moved ids in the commit.
7. **Gates**: `cargo test -p kuna-jvm`, `make test-jvm`, `kuna catalog --check`, `make
   check-spec` and `python3 tools/check_spec.py --strict`, `make lint`.

Fixes made in parallel meet in the same files. Keep both sides of `fixes.toml` and of
`tests/jvm/kuna-fixes/cases.json`, then rerun `regen.py` (it rewrites `cases.tsv` and both
READMEs, keeping each case's `kuna` digest, and refuses a case whose `fix` is not an id of
`fixes.toml`); where both sides moved a digest, rerun `kuna test --jvm --save-digests --out
DIR` and read the texts of the ids it reports.

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
   return (f(f(x)) as Number).toInt()
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
`StringsKt.repeat(..)` for an extension function), boxing shows as casts
(`as Number`), a captured local that a lambda changes is an `IntRef`, and the
body of an inline function (`let`, `forEach`) appears expanded where it was called. A call
that leaves arguments to their defaults is written as the call the source makes, not as the
`$default` bridge it compiles to, and a cast is Kotlin's `as` or conversion call
(`b.toLong()`), not Java's `(long)b` (fixes `kt_default_args_call`, `kt_java_cast_syntax`
and `kt_java_reference_cast`). A JDK member or class Kotlin maps to its own is written by
its Kotlin name (`text.length`, `(x as Number).toInt()`; fixes `kt_mapped_java_member`
and `kt_mapped_java_type`); any other Java type whose simple name Kotlin reuses is written
qualified (`java.util.Collection`). Apart from kuna's fixes the text is Vineflower's, quirks
included.

**The CLI on Kotlin units.** Classes and methods are still named by their class-file
names, which is what `kuna functions` lists: a property is selected by its accessors, a
companion object's member is in the `$Companion` class, and a method the Kotlin writer
folds into another declaration (a `$default` stub, a lambda's method, a `@JvmStatic`
forwarder) has no text of its own. `--language java` (`jvm_kt_enable off`) writes every
class as Java, which shows what the Kotlin view hides: the accessors, the `$default` stubs,
the `Intrinsics` checks. Every JSON record carries the `language` of its unit. `docs/cli.md`, "Kotlin
classes", has the naming table and examples.

**Options.** The plugin's options are the `jvm_kt_*` rows of `kuna catalog --engine jvm`;
each also answers to Vineflower's name (`kt-enable`, ...):

| Option | Default | Effect |
|---|---|---|
| `jvm_kt_enable` | `on` | write Kotlin classes as Kotlin; `off` writes them as Java with the `@Metadata` annotation |
| `jvm_kt_collapse_string_concat` | `on` | write string concatenations as string templates (`"count=${c.count}"`); `off` keeps `"count=" + c.count` |
| `jvm_kt_decompile_facades` | `off` | also write the forwarding functions of a multifile facade (`@file:JvmMultifileClass`), not only the comment listing its parts |

**Known limits.** The Kotlin output is Vineflower's, so it has Vineflower's gaps, apart
from those kuna's fixes close (*Kuna-only fixes*, above). The plugin rewrites syntax, not
semantics, so whatever the compiler lowered stays lowered (above). A `suspend` function
the metadata declares is written with its direct body (fixes `kt_suspend_state_machine`,
`kt_suspend_call_continuation`, `kt_coroutine_boxing`), but one whose state machine kuna
cannot map completely (its continuation also used by inlined
`suspendCoroutineUninterceptedOrReturn`) is written as the machine the compiler made of it:
its continuation appears as `object : ContinuationImpl($completion) { ... }` where the
function creates it, and the type Kotlin cannot name shows as `<unrepresentable>`
(Vineflower fails on every such function). Suspend lambdas, the suspend functions of
objects kotlinc gives no function metadata (a `fun interface` lambda's `emit`) and open
suspend functions' `$suspendImpl` bodies are rebuilt the same way (fixes
`kt_suspend_lambda`, `kt_suspend_without_metadata`, `kt_suspend_impl`); a function type
with a receiver is still written without it (`suspend (FlowCollector<T>) -> Unit` for
`suspend FlowCollector<T>.() -> Unit`), so a rebuilt lambda with a receiver, written
`flow@{ this@flow.emit(x) }`, does not match the declaration the plugin writes.
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

**Variable renaming.** Vineflower's VariableRenaming plugin (`variable-renaming jad|tiny`,
`rename-parameters`) is not ported: it replaced the LocalVariableTable's names with names
made up from the types. Names come from the table, or, with `jvm_use_lvt_names off` or
without one, are Vineflower's numbered `var0`, `var1`.

## How failures are reported

The engine never stops on a method or class it cannot decompile: it writes the failure as
a comment where the code would be. The comment's place decides what failed, and the class
writers record each one as they write it (`src/infra/kuna_spans.rs`), changing no text:

| The engine writes | Recorded as | The CLI reports |
|---|---|---|
| `// $VF: Couldn't be decompiled` as a method's body | the method's `error` | that method failed |
| the same comment as the body of a lambda, of a method of an anonymous or local class, or `// $VF: Couldn't be decompiled` for such a class, written inside method M | a `Failure` on M's span (and the anonymous class's method keeps its own `error`) | M failed too, naming the body (`nested_errors`); one `warning:` per body |
| any of these outside every method (a field initializer's lambda, a member class) | a `Failure` on the class's span | the class's `nested_errors`; `decompile-project` counts them as `outside methods` |
| `/* $VF: Unable to decompile class */` in place of a whole unit (the Java and Kotlin writers' last resort) | the unit's `error` | a failed class (`status: "failed"`) on every command, with that reason |
| a `$VF:` comment above a method that the engine flags as an error (`MethodWrapper::add_error_comment`, the comments that carry the report-this note) | the method span's `warnings` | a `warning:`, not a failure |
| any other `$VF:` comment, or a line only logged | nothing | nothing |

The Kotlin writer does not print method comments at all, so a Kotlin method's flagged
comments show only as CLI warnings; a lambda's own flagged comments are printed by neither
writer and are not reported. `docs/cli.md` (*Failures and warnings*) lists the flagged and
the informational comments, the record shapes and the summary line.

The engine's log (the logger the CLI installs is silent) has lines with no comment in the
output: `Statement cannot be decomposed although reducible!` is a dead end of DomHelper's
search that its caller recovers from, unless the method then fails with `parsing
failure!`, which is reported as above; `Inconsistent inner class entries` says which of
two conflicting `InnerClasses` records the engine kept;
`Inconsistent anonymous class definition` and `Inconsistent references to the class`
mean the engine writes that class as a top-level unit of its own instead of inside the
method that creates it, which the CLI follows (it asks the engine for every class of a
target's outer chain and takes the nearest one written as a unit). None of them is
reported, and the JVM path has no `--verbose` to show them.

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
comment and exits.

**Speed.** One engine instance is slower than Vineflower on one thread. Against
Vineflower 1.12.0 with `-thr=1` on JDK 21, `--jobs 1` takes 1.2 to 2.2 times the wall time
on whole jars: spring-boot 7.2 s against 6.1 s, Guava 20 s against 13 s, spring-framework
38 s against 21 s, Elasticsearch 190 s against 85 s. kuna uses one core for that;
Vineflower's JVM also runs compiler and garbage-collector threads, and its CPU time is
five to nine times its wall time. The parallel driver is how kuna makes up the
difference: with both decompilers' default flags on the same 80-core machine, kuna finishes spring-boot in 0.7 s against 3.7 s, Guava in
1.9 s against 5.8 s, spring-framework in 3.9 s against 7.5 s and Elasticsearch in 12.5 s
against 14.5 s, using a third of Vineflower's CPU time or less. These are the best of three
`decompile-project` runs on the benchmark's full input jars (`docs/jvm-benchmark.md`) on a
quiet machine, Vineflower with its default thread count (one per core).

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
| `make test-jvm` (= `kuna test --jvm`) | the gate: every case prints its expected text: each Vineflower-oracle case (Java, plugin and Kotlin) Vineflower's exact text in compat mode and, by default, the text its `kuna` digest pins, and each kuna-fixes case Vineflower's text in compat mode and kuna's reviewed text by default; Vineflower's text is pinned by its digest (`tests/jvm/README.md` has the format and the counts). Needs no SLEIGH specs, no JDK and no Kotlin compiler. |
| `cd decompiler && cargo test -p kuna-jvm --test integration` | the unit and differential suite (part of `make rust-test`): kernel goldens against a real JDK, per-pass oracles, option-table checks, driver determinism |

Useful flags of `kuna test --jvm` (`docs/cli.md`): `--name SUBSTR` runs the matching cases;
`--out DIR` writes each run's text as `DIR/<id>.java` or `.kt`; `--save-digests` re-records
the `kuna` digests of the default-mode runs it made; `--manifest FILE` runs another
manifest, such as the full Java or Kotlin one generated from the external corpora.
`tests/jvm/regen.py --expected-text DIR CASE...` writes Vineflower's text of named cases
from the oracle workspace, for a diff against kuna's.

**Regenerating the corpus.** `tests/jvm/` is generated, never edited by hand, from the
Vineflower oracle that `decompiler/nonnative/kuna-jvm/tools/oracle` builds outside the
repository into `KUNA_JVM_ORACLE` (its `README.md` lists what it downloads, with pinned
hashes); `tests/jvm/README.md` has the commands, and `regen.py --out DIR` writes a fresh
copy that matches the committed files byte for byte.

**When a case differs.** The diff shows the symptom, rarely the pass. `kuna decompile
<class|jar|dir> [<class>] [--option NAME VALUE]...` reruns a class under any option set;
`--option` also takes Vineflower's option names.

## javac's block layout

`kuna_javac_layout`: on classes javac compiled (no `kotlin.Metadata`, no `.kt` source file) it structures ifs, exits and
loops the way javac laid the bytecode out, where Vineflower prints an equivalent but
different form:

- two return instructions stay two returns instead of one return of a ternary (`if (c)
  return true; return false;` is not folded to `return c;`);
- an `if` keeps javac's then-branch, the code right after the condition, so `if (o == null
  || getClass() != o.getClass()) return false;` keeps its polarity;
- an early `return;`, `throw`, `break` or `continue` the bytecode has stays, instead of the
  rest of the method or loop body being nested in a negated `if`, and a negated pattern
  javac compiled as a guard, `if (!(o instanceof T v)) { return ..; } rest`, keeps its guard;
- a comparison with a boolean literal that has bytecode of its own (`false == x`, `x ==
  true`, `(a || b) == false`) stays a comparison instead of becoming `!x`, `x` or a negated
  condition;
- in loop bodies, an if-else or else-if chain, an `if (c) { ..; continue; }`, consecutive
  `if (..) continue;` statements, a `for` loop's `break` and a `while (true)` loop (not
  made a `do`-`while`) keep their form, and the statement javac placed after a try-catch
  stays after it;
- around loops and protected blocks, the code after a `for (;;)` loop or a switch stays
  after it with the loop's `break`, a value returned after a try or `synchronized` block
  stays after it, a `while` loop or `for` header keeps the form the LocalVariableTable's
  scopes and the LineNumberTable show, and an early exit inside a try-with-resources,
  `finally` or `synchronized` block stays a guard.

Where the bytecode is the same for both forms, Vineflower's form mostly stays. Two
Vineflower steps that would print wrong code on the shapes these rules leave are guarded.
Spec section 10.9 has the rules; the two-pass cases `pkg/TestJavacLayout`,
`pkg/TestJavacBooleanCompare`, `pkg/TestJavacLoops` and `pkg/TestJavacTails`
(`tests/jvm/kuna-fixes/`) show each shape, and `docs/jvm-benchmark.md` its effect.

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
so a kuna improvement that changes any compat-mode text stays off by default; it may
default on only when `make test-jvm` still matches every compat-mode run with it on and it
passes the speed budget (the native engine's rule). Fixes, kuna's source forms and javac's
block layout leave every compat-mode text alone
and change only default-mode ones: the kuna-fixes cases' expected text and the
Vineflower-oracle cases' digests, both re-recorded after review. A fix's evidence is its
two-pass case and the whole-output diffs of step 6 of the recipe; such an option defaults on
only when a full benchmark run of the five projects shows it better (methods perfect on
every metric, classes that recompile, each worse method read and explained) at the same
speed, and the same whole-output diffs are read hunk by hunk.

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
   pins 1.12.0's jar hash and commit; update both) and regenerate `tests/jvm/`; run the corpus and fix every case that no longer
   matches.
6. Update this page (version, commit, the differences above), the crate header in
   `src/lib.rs`, `decompiler/nonnative/README.md`, the `inspiration` field of every row,
   and `NOTICE` if the upstream copyright notices changed.
7. Revisit every kuna fix: when the new release fixes the same defect, the ported change
   replaces kuna's, so retire the fix (its `KunaFix` variant, table row, gated sites and
   case); when it still has the defect, the kuna-fixes case's compat
   expectation is regenerated with the new jar by rerunning `tests/jvm/kuna_fix_case.py`.
