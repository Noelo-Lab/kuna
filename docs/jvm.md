# The JVM engine

`kuna-jvm` (`decompiler/managed/kuna-jvm`) decompiles JVM class files, jars and
directories of classes to Java, and the classes the Kotlin compiler emitted to Kotlin. It
began as a Rust port of [Vineflower](https://github.com/Vineflower/vineflower) 1.12.0 with
two of the plugins Vineflower bundles (Kotlin, IDEA not-null) and has
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
| `docs/spec/11-jvm.md` | what the engine computes, stage by stage, and how its code is organised |
| `docs/options-jvm.md` | the generated `jvm_*` option catalog (`kuna catalog --markdown --engine jvm`) |
| `decompiler/managed/kuna-jvm/corpus/README.md` | the regression corpus: what a case is, how to add one, how to investigate a red one |
| `docs/jvm-benchmark.md` | the benchmark against Vineflower on three sets of Java projects (26 in all): what it scores, how to run it, the results |

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
class's members or hierarchy degrades. `kuna decompile decompiler/managed/kuna-jvm/fixtures/app.jar
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

`kuna catalog --engine jvm` lists every `jvm_*` option; an option has that one name
(Vineflower's long and three-letter names are not accepted). The ones a user reaches for:

| Option | Default | Effect |
|---|---|---|
| `jvm_include_runtime` | `on` | the JDK read as context (above) |
| `jvm_kt_enable` | `on` | Kotlin classes as Kotlin; `--language java` is `off` (below) |

`--lib PATH` adds class files or jars as context without
decompiling them.

## Kotlin

The Kotlin compiler emits ordinary class files, and the Java pipeline decompiles them to
correct but noisy Java: property accessors, `Intrinsics` null checks, `$default` stubs,
`Companion` fields and a `@Metadata(d1 = ...)` annotation full of encoded strings.
Vineflower's Kotlin plugin, which is part of kuna's pipeline, writes them as Kotlin
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
`CountersKt`), and so does its unit. The spec's section 11.15 has the algorithm.

**What the output looks like.** `decompiler/managed/kuna-jvm/fixtures/kotlin` holds two small Kotlin files
compiled with kotlinc 2.0.0 (`src/` has the source):

```
$ kuna decompile decompiler/managed/kuna-jvm/fixtures/kotlin/classes com.ex.kt.CountersKt
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

## IDEA not-null

IntelliJ IDEA's compiler instruments `@NotNull` parameters and return values with checks
that call a synthetic `$$$reportNull$$$0`. The engine removes them and keeps the
annotations:

```
$ kuna decompile decompiler/managed/kuna-jvm/corpus/classes/custom/TestIdeaNotNull.class pkg.TestIdeaNotNull.test1
public void test1(@NotNull String s) {
   System.out.println(s);
}
```

## Android: DEX and APK input

A DEX file (`classes.dex`) or an APK (any zip whose root holds `classes.dex`,
`classes2.dex`, ... and no class files) is an input like a jar: `kuna decompile app.apk
com.ex.Main`, `functions`, `decompile-all`, `decompile-project`, selection, spans, JSON and
Kotlin output work unchanged. The engine never sees Dalvik bytecode. When the input is
loaded, `decompiler/managed/kuna-dex` translates each DEX class into a JVM class file in
memory and `src/p0_knowledge/kuna_input.rs` (`dex_unit`) registers the bytes exactly as it
registers a jar's entries: every `classes*.dex` of an APK in order, the first definition of
a class winning, on up to eight threads. A zip with both class files and DEX is a jar (the
DEX is ignored). DEX versions 035 to 039 are read; a container (041) or any other version,
and a malformed file, is `error: PATH: unreadable DEX file: REASON`.

**What is translated.** Every class definition with its fields, methods and code, and the
attributes the engine reads, rebuilt from the system annotations: `Signature`,
`Exceptions` (`Throws`), `InnerClasses` and `EnclosingMethod` (`InnerClass`,
`EnclosingClass`, `EnclosingMethod`, `MemberClasses`; a class lists the classes nested in it,
anonymous and local ones included, as javac does), `AnnotationDefault`,
`MethodParameters`, `ConstantValue` from static values, `SourceFile`, `LineNumberTable`,
`LocalVariableTable` and `LocalVariableTypeTable` from debug info, and `BootstrapMethods`
for `invoke-custom` and method-handle constants. Every other annotation becomes a runtime
visible or invisible (parameter) annotation, so `kotlin.Metadata` reaches the Kotlin
plugin and Kotlin classes print as Kotlin. Code is translated so that the engine sees what
javac would have written: registers are typed by dataflow, each (register, type category)
becomes one local, a value defined once and read once in a block stays on the operand stack
as an expression, constants are written at each use, `new-instance` and its constructor call
become `new T(...)` (through d8's register copies too), the copies d8 makes to line up a
call are forwarded so arguments print inside the call (`super(...)` first, as javac wrote
it), a value that only joins constants (`boolean b = x > 3`) is carried on the stack across
the join, and `finally` and `synchronized` regions are laid out and protected as javac lays
them out. d8's desugaring is undone: lambda classes become the `invokedynamic` javac wrote
(lambdas and method references print as such), calls into API backports
(`$$ExternalSyntheticBackport`, recognised by their code against a table recorded from d8)
become the JDK calls, records on `com.android.tools.r8.RecordTag` print as records, nest
accessors (`-$$Nest$...`, `-IA` constructors) become the private accesses, and a catch-all
that does not rethrow is `catch (Throwable t)`; these work across the `classes*.dex` of an
APK (`docs/spec/11-jvm.md` 11.17 has the algorithm). A method whose code cannot be
translated fails alone, with the engine's usual failure comment naming the reason
(`DEX code not translated: ...`).

**What is lost relative to class files.** The rest of d8's work is not undone: register
reuse, comparisons and constants shared between uses, code moved or merged across branches
(shared returns, a `throw` placed after the return), default and static interface methods
moved into `$-CC` classes below min-api 24, string concatenation (a `StringBuilder` chain),
pattern `switch` (`$$ExternalSyntheticTypeSwitch`), `AutoCloseable` dispatch, the inlined
backports (`List.of()` as `Collections.EMPTY_LIST`, `Boolean.logicalAnd` as `&`), the lambda
classes themselves (still listed and printed), and a `synchronized` method's lock written as
an explicit `synchronized (this)` block. Backports are recognised for d8 8.13 (another
version's bodies may differ). Local names and types exist only in debug builds (a release build keeps lines and
maybe parameter names). A value that crosses blocks is a variable unless only constants
join (a `?:` with computed arms prints as an `if`). DEX does not say
whether a library class is an interface, so a static or `super` call into one is an
interface call only for a short list of JDK interfaces. Class files are written at version
50 (52 for interfaces with code, lambdas and `invoke-custom`, 51 for method-handle constants), so
version-gated sugar the engine reconstructs from newer javac output does not apply.

**How it is checked.** `decompiler/managed/kuna-jvm/fixtures/dex/src` holds small Java and Kotlin programs
that together execute every Dalvik instruction family; `regen.sh` compiles them with javac,
kotlinc and d8 (versions in `TOOLCHAIN.txt`) and records their output. Translated to
class files at version 50 (the `verifiable` option), all six run under `java -Xverify:all`
(the JVM's type-inferring verifier checks every method) and print exactly what the
javac-compiled programs print, from debug and release d8 at min-api 21; in the default mode
(lambdas as `invokedynamic`, version 52, which does not verify without stack maps) and at
min-api 26 and 30 they match unverified.
d8-compiled Guava 33.8 (min-api 21, release and debug) verifies in full: 16,417 of 16,417
methods. The crate's own tests (`cargo test -p kuna-dex`) need no JDK or d8: parser,
structural and garbled-file tests over the committed fixtures.

**Measured.** The JVM benchmark (`docs/jvm-benchmark.md`, all 110,238 scored methods),
with each project's jar compiled by d8 8.13.25 (`--debug --min-api 26`) and scored against
the same sources, methods perfect on all metrics:

| project | from the jar | from DEX, kuna | from DEX, jadx 1.5.3 |
|---|---|---|---|
| Guava | 98.78% | 80.70% | 42.65% |
| spring-boot | 99.29% | 76.38% | 57.40% |
| spring-framework | 99.03% | 74.03% | 62.92% |
| Ghidra | 98.71% | 73.28% | 56.59% |
| Elasticsearch | 98.89% | 66.95% | 36.05% |
| all | 98.88% | 70.74% | 44.31% |

Release builds (what APKs ship) carry no local-variable debug info, and d8 optimises more
(shared values, split returns, rewritten library calls). Built with `--release` instead of
`--debug` (same d8, min-api 26, classpath), methods perfect on all metrics, and the release
metrics for kuna / jadx:

| project | from the jar | debug DEX, kuna | release DEX, kuna | release DEX, jadx | structure | types | bytes | classes recompiling |
|---|---|---|---|---|---|---|---|---|
| Guava | 98.78% | 80.70% | 65.21% | 38.23% | 92.2 / 88.5% | 83.2 / 82.6% | 67.6 / 38.6% | 69.9 / 48.7% |
| spring-boot | 99.29% | 76.38% | 62.21% | 47.90% | 90.0 / 90.5% | 79.2 / 80.9% | 64.4 / 48.9% | 77.5 / 59.8% |
| spring-framework | 99.03% | 74.03% | 61.56% | 52.52% | 88.1 / 88.5% | 79.4 / 81.7% | 63.0 / 52.8% | 77.1 / 68.7% |
| Ghidra | 98.71% | 73.28% | 60.57% | 49.57% | 84.7 / 84.6% | 77.3 / 78.0% | 61.4 / 50.0% | 80.7 / 73.0% |
| Elasticsearch | 98.89% | 66.95% | 53.67% | 31.71% | 85.5 / 83.0% | 84.1 / 81.4% | 54.3 / 31.8% | 69.4 / 44.7% |
| all | 98.88% | 70.74% | 57.38% | 38.57% | 86.6 / 84.8% | 82.1 / 81.0% | 58.4 / 38.8% | 73.2 / 54.7% |

What a release build costs both tools: the types metric most (local types come from debug
info: kuna 96.7% to 82.1%, jadx 93.9% to 81.0%, so neither has much of an edge there), then
bytes exact and classes recompiling (d8's release optimisations reach the recompiled code;
kuna 70.8% to 58.4% and 81.7% to 73.2%, jadx 44.4% to 38.8% and 56.1% to 54.7%), and
structure least (89.6% to 86.6%, jadx 87.8% to 84.8%). Parameter names survive where the code was
compiled with `-parameters`, and line numbers survive.

Most of the remaining loss is structure that differs from javac's (d8's shared comparisons
and merged returns, string concatenation), `byte`/`int` typed where `boolean` is needed
(a constant shared between a boolean and an int use), and 518 methods the engine fails
(mostly "parsing failure" on d8 layouts it cannot structure, and `FinallyProcessor` on the
rest of the `finally` shapes). A `synchronized` block the engine cannot rebuild from d8's
monitor ranges is among those: its text marks the monitor instructions in comments and
rethrows a `Throwable` local, which javac rejects in a method that does not declare it.
Loading is
cheap: on Elasticsearch's APK (21,939 classes in three DEX files, 19.8 MB) `functions` takes
1.6 s (0.5 s for the jar) and `decompile-all --jobs 8` 43 s at 6.1 GB peak (d8 adds a class
per lambda). Nine release APKs (R8-optimised, 3,689 classes) decompile with no panic or
hang and 136 failure comments, none from names that are not Java identifiers.

Not read yet: the binary manifest and resources, ODEX/VDEX/OAT/CDEX, native libraries
inside an APK, and protected (packed or encrypted) APKs.

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
11.14). `auto` is the number of cores, at most 24 and at most one per 16 class files,
reduced until the memory estimate below fits in 90% of the available memory. Measured on
an 80-core machine (`decompile-project`, best of three): Elasticsearch (53 MB of classes)
190 s with one worker, 27 s with eight, 12.5 s with twenty-four; Guava (6.6 MB) 20 s with
one worker, 3.4 s with eight, 1.9 s with twenty-four.

**Memory.** An instance reads a class's constant pool entries and members when it first
uses them (spec section 11.3), so it holds little more than the classes its batches touch.
The driver estimates an instance at 32 MiB plus 4 bytes per byte of input class files and
1 per byte of library class files, and the decompiled state and results at 90 bytes per
input byte, whatever the count; the estimate bounds the peaks of Elasticsearch (2.5 GB
with one worker, 4.2 GB with eight, 7.4 GB with twenty-four), Guava (0.37, 0.74 and
1.5 GB), Ghidra (9.1 MB of classes: 0.82 GB with eight, 1.4 GB with twenty-four) and
spring-boot (3.0 MB: 0.16, 0.31 and 0.67 GB).
`--max-fn-memory` is process-wide on JVM inputs, since the instances are threads of one
process.

**Time.** `--max-fn-seconds N` (default 120, `0` disables) is a per-method budget. The
engine polls it at the heads of its fixed-point loops (spec section 11.12) and counts the
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
limit it was two on an idle machine and four on a busy one (spec section 11.12). A
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

**Hard failures.** A method or class that throws is contained (spec section 11.12), and
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
| `make test-jvm` (= `kuna test --jvm`) | the gate: every case of the regression corpus prints the text pinned for it (`decompiler/managed/kuna-jvm/corpus/README.md`). Needs no SLEIGH specs, no JDK and no Kotlin compiler. |
| `cd decompiler && cargo test -p kuna-jvm --test integration` | the engine's suite (part of `make rust-test`): the driver, deadlines, determinism, failures, spans, inventory, input classification, options, the class-file and archive readers and the runtime kernel against a real JDK |

`kuna test --jvm --name SUBSTR --out DIR` writes the text of the matching cases; `--save-digests`
re-records their digests once a change of text is reviewed. `decompiler/managed/kuna-jvm/corpus/README.md` has the
commands for adding a case and for investigating a red one. `kuna decompile <class|jar|dir>
[<class>] [--option NAME VALUE]...` reruns a class under any option set.

## Provenance

- **Upstream:** Vineflower 1.12.0, commit `b8273988` ("Release 1.12.0"),
  https://github.com/Vineflower/vineflower, Apache License 2.0. Vineflower continues
  JetBrains' Fernflower through ForgeFlower, FabricMC's fork and Quiltflower. The
  attribution is in the repo's `NOTICE`.
- **What was ported.** 242 of the 273 Java files under `org/jetbrains/java/decompiler/` in
  Vineflower's `src/`, and its bundled Kotlin and IDEA not-null plugins (`src/kotlin/`,
  `src/plugins/`), were ported one to one; each names its original in its first line, so
  `grep -r '^//! Port of' decompiler/managed/kuna-jvm/src` is the manifest. The rest
  needed a JVM and is replaced by kuna's own code: the jar, directory and `jrt:` sources
  (`context_source`, `jdk_runtime`, `zip_reader`), reflection (an embedded table of
  OpenJDK 21's varargs methods, `src/p4_calls/classpath_helper_jdk.txt`), the thread pool
  (the parallel driver) and the watchdog (the cooperative deadline). A user renamer class,
  `include-classpath` and third-party plugin jars are not supported.
- **Since the port.** kuna's own code is in the `kuna_*` modules, in `src/fixes/` (the
  corrections of Vineflower's defects, one module per family). The regression corpus
  (`decompiler/managed/kuna-jvm/corpus/`) began as Vineflower's own test suite and pins kuna's text, not Vineflower's.
