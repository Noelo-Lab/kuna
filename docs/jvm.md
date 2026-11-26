# The JVM engine

`kuna-jvm` (`decompiler/nonnative/kuna-jvm`) decompiles JVM class files, jars and
directories of classes to Java, and the classes the Kotlin compiler emitted to Kotlin. It
began as a Rust port of [Vineflower](https://github.com/Vineflower/vineflower) 1.12.0 with
the three plugins Vineflower bundles (Kotlin, variable renaming, IDEA not-null) and has
since diverged: it decompiles the methods Vineflower fails on, corrects code Vineflower
prints wrong, and lays code out as javac compiled it (*Provenance*, below). It is reached
through the ordinary `kuna` CLI: give a command a `.class`, a jar or a directory where it
would take a binary.

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
| `docs/spec/10-jvm.md` | what the engine computes, stage by stage, and how its code is organised |
| `docs/options-jvm.md` | the generated `jvm_*` option catalog (`kuna catalog --markdown --engine jvm`) |
| `tests/jvm/README.md` | the regression corpus: what a case is, how to add one, how to investigate a red one |
| `docs/jvm-benchmark.md` | the benchmark against Vineflower on five large Java projects: what it scores, how to run it, the result |

## Inputs

`kuna-cli` picks the JVM engine for an input before any engine code runs
(`kuna_input::sniff`: a class-file, zip, jmod, jimage or DEX header, a directory, or a jar
with a launcher stub in front; spec chapter 00, section 0.2). A class file brings its nested
classes; a jar or directory brings every class in it. Below the CLI the JVM engine shares
nothing with the native one: JVM bytecode is typed, its exception ranges are tables and the
class file states the program's structure, so the engine works over a Java-level IR of its
own instead of p-code.

### The JDK runtime library

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

## Options worth knowing

`kuna catalog --engine jvm` lists every `jvm_*` option; each also answers to Vineflower's
long and three-letter names. The ones a user reaches for:

| Option | Default | Effect |
|---|---|---|
| `jvm_include_runtime` | `on` | the JDK read as context (above) |
| `jvm_kt_enable` | `on` | Kotlin classes as Kotlin; `--language java` is `off` (below) |
| `jvm_javac_layout` | `on` | ifs, exits, loops and boolean comparisons laid out as javac laid out the bytecode (spec section 10.9) |
| `jvm_source_locals` | `on` | locals as the class file names them, declared in slot order and in the source's scopes (spec section 10.10) |
| `jvm_source_patterns` | `on` | a test and a cast where the source wrote them, not an `instanceof` pattern (spec section 10.10) |
| `jvm_resugar_idea_notnull` | `on` | removes the null checks IntelliJ's compiler adds for `@NotNull`; `off` shows them |
| `jvm_variable_renaming` | off | `jad` names locals after their type the way JAD did (`i`, `s`, `flag`), `tiny` the way tiny-remapper does; `jvm_rename_parameters on` renames parameters too. It overrides the LocalVariableTable's names, so it is for obfuscated or debug-info-free code |

`tiny` with `jvm_rename_parameters on` fails every method that has a LocalVariableTable
(`NullPointerException: type`). `--lib PATH` adds class files or jars as context without
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
(`b.toLong()`), not Java's `(long)b`. A JDK member or class Kotlin maps to its own is
written by its Kotlin name (`text.length`, `(x as Number).toInt()`); any other Java type
whose simple name Kotlin reuses is written qualified (`java.util.Collection`).

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

**Known limits.** The plugin rewrites syntax, not semantics, so whatever the compiler
lowered stays lowered (above). A `suspend` function the metadata declares is written with
its direct body, but one whose state machine kuna
cannot map completely (its continuation also used by inlined
`suspendCoroutineUninterceptedOrReturn`) is written as the machine the compiler made of it:
its continuation appears as `object : ContinuationImpl($completion) { ... }` where the
function creates it, and the type Kotlin cannot name shows as `<unrepresentable>`
Suspend lambdas, the suspend functions of
objects kotlinc gives no function metadata (a `fun interface` lambda's `emit`) and open
suspend functions' `$suspendImpl` bodies are rebuilt the same way; a function type
with a receiver is still written without it (`suspend (FlowCollector<T>) -> Unit` for
`suspend FlowCollector<T>.() -> Unit`), so a rebuilt lambda with a receiver, written
`flow@{ this@flow.emit(x) }`, does not match the declaration the plugin writes.
The metadata schema is the one the plugin was built with (Kotlin 2.2), applied without a version
check. When a method cannot be decompiled, its Kotlin declaration is written with the failure
comment in place of the body, exactly as for Java, and the CLI reports it on stderr and in
the record's `error`; `--language java` usually still gets the Java view. A whole class
the plugin cannot write is a `/* $VF: Unable to decompile class
*/` comment with the exception, which the CLI reports as a failed class (*How failures are
reported*, below). A class whose metadata cannot be decoded is still written as
Kotlin, from its class file alone, under `// $VF: Class flags could not be determined`.

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
`TimeoutException`, and the
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
if` chains, lambdas, field initializers) fails the method or class it is in with a
`StackOverflowError`, as on Vineflower's JVM, and the run goes on (`src/rt/stack.rs`). The engine threads have
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
| `make test-jvm` (= `kuna test --jvm`) | the gate: every case of the regression corpus prints the text pinned for it (`tests/jvm/README.md`). Needs no SLEIGH specs, no JDK and no Kotlin compiler. |
| `cd decompiler && cargo test -p kuna-jvm --test integration` | the engine's suite (part of `make rust-test`): the driver, deadlines, determinism, failures, spans, inventory, input classification, options, the class-file and archive readers and the runtime kernel against a real JDK |

`kuna test --jvm --name SUBSTR --out DIR` writes the text of the matching cases; `--save-digests`
re-records their digests once a change of text is reviewed. `tests/jvm/README.md` has the
commands for adding a case and for investigating a red one. `kuna decompile <class|jar|dir>
[<class>] [--option NAME VALUE]...` reruns a class under any option set.

## Provenance

- **Upstream:** Vineflower 1.12.0, commit `b8273988` ("Release 1.12.0"),
  https://github.com/Vineflower/vineflower, Apache License 2.0. Vineflower continues
  JetBrains' Fernflower through ForgeFlower, FabricMC's fork and Quiltflower. The
  attribution is in the repo's `NOTICE`.
- **What was ported.** 242 of the 273 Java files under `org/jetbrains/java/decompiler/` in
  Vineflower's `src/`, and the three plugins its release jar bundles (`src/kotlin/`,
  `src/plugins/`), were ported one to one; each names its original in its first line, so
  `grep -r '^//! Port of' decompiler/nonnative/kuna-jvm/src` is the manifest. The rest
  needed a JVM and is replaced by kuna's own code: the jar, directory and `jrt:` sources
  (`context_source`, `jdk_runtime`, `zip_reader`), reflection (an embedded table of
  OpenJDK 21's varargs methods, `src/p4_calls/classpath_helper_jdk.txt`), the thread pool
  (the parallel driver) and the watchdog (the cooperative deadline). A user renamer class,
  `include-classpath` and third-party plugin jars are not supported.
- **Since the port.** kuna's own code is in the `kuna_*` modules, in `src/fixes/` (the
  corrections of Vineflower's defects, one module per family) and in the options of form
  (`jvm_javac_layout`, `jvm_source_locals`, `jvm_source_patterns`). The regression corpus
  (`tests/jvm/`) began as Vineflower's own test suite and pins kuna's text, not Vineflower's.
