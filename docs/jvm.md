# The JVM engine

`kuna-jvm` (`decompiler/nonnative/kuna-jvm`) decompiles JVM class files, jars and
directories of classes to Java. It is a Rust port of
[Vineflower](https://github.com/Vineflower/vineflower) 1.12.0, and on the same class files
and options it prints the same text Vineflower prints, byte for byte. It is reached
through the ordinary `kuna` CLI: give a command a `.class`, a jar or a directory where it
would take a binary.

```bash
kuna decompile app.jar com.ex.Main            # one class, as a compilation unit
kuna decompile app.jar com.ex.Main.check      # one method
kuna decompile-project app.jar -o out         # out/com/ex/Main.java, ...
kuna functions app.jar --json                 # the class and method inventory, no decompiling
kuna catalog --engine jvm                     # the jvm_* options
```

| Read | For |
|---|---|
| `docs/cli.md`, "JVM inputs" | every command, flag, output shape and error on JVM inputs |
| `docs/spec/10-jvm.md` | what the engine computes, stage by stage |
| `docs/options-jvm.md` | the generated `jvm_*` option catalog (`kuna catalog --markdown --engine jvm`) |
| `tests/jvm/README.md` | the regression corpus: what a case is, how it was generated |
| `decompiler/nonnative/kuna-jvm/tools/vftrace/README.md` | the pass-by-pass trace differ |

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
| `src/infra/` | the per-method schedule (`method_processor`), and kuna's parallel driver, spans, tracer and corpus runner |
| `options.toml` | the `jvm_*` option rows; `build.rs` generates the static table from it |
| `examples/` | `corpus` (the regression runner), `jvmdec` (a development CLI), `jvmtrace` (trace one run) |
| `tests/` | the unit and differential suite, one `integration` test binary (`tests/integration.rs` registers every file) |
| `tools/vftrace/` | the trace differ against the real Vineflower |

The folders follow kuna's phase names so a stage can be found by its phase; the order the
stages run in is the engine's own schedule, `src/infra/method_processor.rs` per method and
`src/p0_knowledge/context_unit.rs` per class (spec section 10.1).

### Porting conventions

Every ported file starts with `//! Port of \`org/jetbrains/java/decompiler/.../X.java\`.`,
and the port follows the original line by line, so a Vineflower fix can be found and
applied in the same place. A maintainer needs these rules:

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
  `kuna_options`, `kuna_driver`, `kuna_partition`, `kuna_plugins`, `kuna_spans`,
  `kuna_trace`, `kuna_corpus`) and in the modules that replace JVM facilities (`context_source`,
  `jdk_runtime`, `zip_reader`, `byte_cursor`). Changes inside ported files are kept to the
  seams those modules need (span marks in the writer, a deadline poll in the
  cancellation manager, partial runs in `fernflower.rs`), noted in the file headers.
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
| `exception.rs` | Java exceptions as unwinding panics: `throw`, `try_catch`, `rethrow`; an ordinary Rust panic in the region (an index out of bounds, an `unwrap` on `None`) is caught the same way, as the `IndexOutOfBoundsException` or `NullPointerException` it stands for |
| `bitset.rs` | `java.util.BitSet` |

Hash collections keyed by objects whose Java hash is the identity hash order those
objects by creation in the arena; see the fidelity contract below.

## Provenance

- **Upstream:** Vineflower 1.12.0, commit `b8273988` ("Release 1.12.0"),
  https://github.com/Vineflower/vineflower, Apache License 2.0. Vineflower continues
  JetBrains' Fernflower through ForgeFlower, FabricMC's fork and Quiltflower. The
  attribution is in the repo's `NOTICE`.
- **Test data:** the regression corpus in `tests/jvm/` is built from Vineflower's own
  `testData` sources and registrations and from the output of the released
  `vineflower-1.12.0.jar` (`tests/jvm/README.md`).
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
- **Not ported:** Vineflower's plugins (Kotlin, Scala, IDEA `@NotNull`, variable renaming).
  The plugin and language-plugin API is ported, and `jvm_plugins` (default `on`) loads the
  plugins bundled with the engine, but that bundle is still empty: every class, including
  one compiled from Kotlin or Scala, is written as Java. A user renamer class and
  Vineflower's `include-classpath` need a JVM and are refused (`options.toml`,
  `[[driver_owned]]`).

## The fidelity contract

**The test oracle is Vineflower's text.** On the same class files and options, kuna-jvm
prints exactly what Vineflower 1.12.0 prints. Every case of the vendored corpus (530, all
listed in `tests/jvm/baseline.txt`) matches, and so do the 68 cases that need a particular
JDK on the machine and are run from the external oracle corpus (598 in all). Whole jars (log4j,
pdfbox, fontbox, commons-logging, gettext, the Vineflower jar itself, Vineflower's
obfuscated test jars) and 1,755 classes of JDK 21 come out identical apart from the
differences below, and the parallel driver's output on Guava and spring-boot is identical
at every worker count. A change that makes a case differ is a regression to fix, unless
it is a named kuna option that is off by default.

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

   With `jvm_dump_exception_on_error off` neither decompiler writes the exception. Run
   with the command-line defaults of both decompilers, 11 of the 598 corpus cases differ
   only in this, and one more because of a plugin (item 5).
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
5. **Plugins.** Output that a Vineflower plugin would change differs: Kotlin and Scala
   classes are written as Java, and IDEA's `@NotNull` checks are not folded (one corpus
   test, `TestIdeaNotNull`).
6. **Resources.** Vineflower's savers copy a jar's non-class entries into the output; kuna
   writes Java sources only.

## The JDK runtime library

Vineflower decompiles with the JDK's classes as context, and so does kuna by default. The
JDK is only read, never run: kuna needs a JDK home on disk, not a working `java`.

**Discovery.** `--option jvm_include_runtime` takes `on` (the default; also `current` or
`1`), a JDK home path, or `off`. With `on`, the first of these that holds a readable
runtime is used: `JAVA_HOME`; the `java` on `PATH`, followed through symlinks to its home;
then the usual install directories (`/usr/lib/jvm`, `/usr/lib64/jvm`, `/usr/java`,
`/opt/java`, `/Library/Java/JavaVirtualMachines`, `~/Library/Java/JavaVirtualMachines`,
`~/.sdkman/candidates/java`), newest `JAVA_VERSION` first. A home is readable when it has
`lib/modules`, `jmods/java.base.jmod`, or an `rt.jar` in `lib` or `jre/lib`; kuna reads the
jimage, the jmods or the Java 8 jars respectively. The JSON documents name the runtime
used (`runtime: {path, version}`); when none is found, one note on stderr says so and the
run continues without one.

**What changes without one.** Every class is still decompiled, but whatever needs a JDK
class's members or hierarchy degrades. The same jar, with and without a JDK 21:

```diff
-   private final List<String> names = new ArrayList();
+   private final List<String> names = new ArrayList<>();

+   @Override
    public String toString() {
```

Casts and generic arguments that depend on JDK signatures can also differ. Output
therefore depends on which JDK is found: for reproducible output, pin one with
`--option jvm_include_runtime /path/to/jdk` or use `off`. The vendored corpus runs with no
JDK. `--lib PATH` adds further class files or jars as context the same way, without
decompiling them.

## Performance and limits

**Parallelism.** `--jobs N|auto` (default `auto`) runs N engine instances on N threads
over the same input (`src/infra/kuna_driver.rs`). The output is byte-identical at every N:
each instance decompiles a batch of root classes in the order a single run would, also
processes (without writing) the other roots its classes depend on, and a batch whose run
read state it did not produce is repeated before its result is used (spec section
10.14). `auto` is the number of cores, at most 16, reduced until the memory estimate below
fits in 90% of the available memory. Measured on Guava (6.5 MB of classes): 81 s with
one worker, 28 s with four, 14 s with sixteen.

**Memory.** Each instance holds the whole class universe of the input and its libraries.
The driver estimates an instance at 128 MiB, plus 96 MiB with a JDK, plus 64 bytes per
byte of input and library class files, and the decompiled state of the whole input at 400
bytes per input byte, spread over the instances; the estimate is fitted to Guava (2.7 GB
with one worker, 9.4 GB with sixteen) and spring-boot (3.0 MB of classes: 0.9 and 2.3 GB).
`--max-fn-memory` is process-wide on JVM inputs, since the instances are threads of one
process.

**Time.** `--max-fn-seconds N` (default 120, `0` disables) is a per-method budget. The
engine polls it at the heads of its fixed-point loops (spec section 10.12); a method past
it is written as `// $VF: Couldn't be decompiled` with a `TimeoutException`, exactly as
Vineflower writes a method its watchdog killed, and the rest of the class is written
normally. A method stuck between two polls is not interrupted; as a last resort the CLI
stops waiting after `2N+30` seconds, writes the classes that finished, gives the stuck ones
a stub comment and exits.

**Speed.** One engine instance is slower than Vineflower on one JVM: 3 to 8 times on whole
jars when the port reached parity (for 1,755 JDK classes, 116 s against 18 s). The
parallel driver is how kuna makes up the difference on large inputs.

**Hard failures.** A method or class that throws is contained (spec section 10.12). A
stack overflow or a failed allocation is not, and ends the process; the engine threads get
512 MiB stacks, because the port recurses as deeply as Vineflower does.

## Testing

| Command | Checks |
|---|---|
| `make test-jvm` (= `kuna test --jvm --baseline tests/jvm/baseline.txt`) | the gate: every case in the baseline still prints Vineflower's exact text. Needs no SLEIGH specs and no JDK. |
| `cd decompiler && cargo test -p kuna-jvm --test integration` | the unit and differential suite (part of `make rust-test`): kernel goldens against a real JDK, per-pass oracles, option-table checks, driver determinism |
| `cd decompiler && cargo run --release -p kuna-jvm --example corpus -- --jobs 16 --baseline ../tests/jvm/baseline.txt` | the same corpus gate without the CLI |

Useful corpus-runner flags (`corpus --help` lists all): `--filter SUBSTR` runs matching
cases in-process (quick); `--out DIR` writes each case's actual text, diff and log;
`--save-baseline ../tests/jvm/baseline.txt` re-records the baseline when a change makes
more cases match (never to absorb a regression); `--manifest FILE` runs another manifest,
such as the full 598-case one generated from the external corpus.

**Regenerating the corpus.** `tests/jvm/` is generated, never edited by hand:
`python3 -I tests/jvm/regen.py --corpus <oracle corpus>`. The oracle corpus compiles
Vineflower's `testData` sources with the JDK of each source set and decompiles them with
the released jar; `tests/jvm/README.md` records how and with which JDKs. The 68
JDK-dependent cases run only from that external corpus
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

The build also compiles the oracle corpus's Vineflower driver; the environment variables
that locate each input (`VF_JAR`, `VF_SRC`, `JAVAC`, `CORPUS`, `BUILD`) are at the top of
`build.sh`.

Use it whenever an output difference is not obviously in the writer. It sees only the
statement tree (not the CFG, SSA state or variable names), and a method that throws
mid-pipeline just stops; `tools/vftrace/README.md` has the format and the canonicalization
rules. For quick runs outside the CLI, `cargo run -p kuna-jvm --example jvmdec -- [--opt
key=value]... [--runtime SPEC] [--jobs N] [-o DIR] <class|jar|dir>` takes Vineflower's
option names directly.

## Adding an option or a kuna improvement

**The option table is one to one with Vineflower.** `options.toml` has one `[[option]]`
row per Vineflower key a user may set, named `jvm_<long name>` with `-` turned into `_`
(Vineflower's long and three-letter names are accepted as aliases), and a
`[[driver_owned]]` entry for each key kuna sets itself. `tests/kuna_options.rs` checks the
two together against `IFernflowerPreferences::all_options()`: every key is exactly one row
or one driver-owned entry, every default agrees with Vineflower's, every row's
`inspiration` names the Java constant, and `phase`/`subphase` name a real folder and the
modules that read the key. `build.rs` refuses an incomplete row. There are no hard-coded
option counts to bump, unlike the native catalog. After touching the table, regenerate the
catalog page: `decompiler/target/release/kuna catalog --markdown --engine jvm >
docs/options-jvm.md` (CI checks it is fresh).

**A kuna improvement is a named option.** Anything that changes the emitted Java beyond
what Vineflower prints ships behind a `jvm_*` row of its own: `vineflower = ""`,
`short = ""`, `aliases = []`, `source_decompiler = "kuna"`, a `change_kind` of its own, and
every other field filled in. Its code goes in a `kuna_<slug>.rs` module in the owning phase
folder, with a minimal seam in the ported code; a kuna row resolves to its own name, so it
cannot collide with a Vineflower key. Describe it in `docs/spec/10-jvm.md` (tagged
`(kuna)`, checked by `make check-spec`, whose strict mode requires every row to be
mentioned). A destructive row also updates the pinned destructive list in
`tests/kuna_options.rs`. A strict bug fix that makes kuna match Vineflower needs no
option.

**Default-on needs corpus evidence.** The corpus gate compares against Vineflower's text,
so a kuna improvement that changes any baseline case stays off by default. It may default
on only when `make test-jvm` still matches every baseline case with it on, and when it
passes the speed budget (the same rule as the native engine's: a default-on feature must
not majorly slow kuna down).

## Re-syncing with a newer Vineflower

1. Diff the new release against `b8273988` (`git diff b8273988..<tag> -- src/`) and map
   each changed Java file to its module through the `//! Port of` headers. New files get
   new modules in the folder of their closest neighbours, named by the convention above.
2. Port each change line by line in the same place, keeping the order of operations,
   iteration orders and constructor calls; `cargo test -p kuna-jvm` after each file.
3. Reconcile the options: new keys in `IFernflowerPreferences` get rows (or
   `[[driver_owned]]` entries); removed keys lose theirs; changed defaults change the row.
   `tests/kuna_options.rs` fails until the table matches. Regenerate
   `docs/options-jvm.md`.
4. If the reference JDK moves, regenerate the embedded varargs table
   (`tests/data/jdk_varargs/gen.sh`) and the kernel goldens under `tests/data/`.
5. Rebuild the oracle corpus with the new release jar and regenerate `tests/jvm/` with
   `regen.py`; run the corpus and use the trace differ on every case that no longer
   matches. Re-record `tests/jvm/baseline.txt` only as part of the sync, and say so in the
   commit message.
6. Update this page (version, commit, the differences above), the crate header in
   `src/lib.rs`, `decompiler/nonnative/README.md`, the `inspiration` field of every row,
   and `NOTICE` if the upstream copyright notices changed.
