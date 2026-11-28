# The JVM benchmark

`scripts/javabench` scores Java decompilers against the original source of open-source Java
projects, method by method, the way decbench scores native decompilers
(`docs/decbench-loop.md`). It is how kuna's JVM engine is compared with Vineflower 1.12.0,
the decompiler it ports. The package's `README.md` has every definition and rule; this page
is the summary and the result.

## What it measures

Three sets of projects, each a jar and its sources at a pinned release
(`scripts/javabench/docs/dataset.md` has the rule that chose each set and every pin):

| Set (`--project`) | Projects | Scored methods | What it says |
|---|---|--:|---|
| `insample` (default) | Guava, spring-boot, spring-framework (core, beans, context), Ghidra (SoftwareModeling), Elasticsearch (server) | 110,238 | how the engine does where its fixes were first developed |
| `heldout1` | antlr4, h2, httpcore5, fastjson2, tink, janino, rhino, joda-time, commons-math3, jenetics, nashorn | 63,189 | measured once out of sample, then used to develop fixes |
| `heldout2` | parboiled, hsqldb, undertow, dsl-json, jasypt, ecj, luaj, junit4, ojalgo, langchain4j | 51,458 | measured once on projects nothing had been tuned against, then used to develop fixes |

Every method with code is either scored or excluded for a stated reason (synthetic and bridge
methods, static initializers, members with no source declaration, Kotlin and generated
sources), so every decompiler is judged on the same methods. A decompiler gets the class files
of the scored top-level classes as one jar, with default options.

Each scored method gets three measurements against its source declaration:

- **Structure**: the graph edit distance between the statement-level control-flow graphs of
  the source and the decompiled method (decbench's procedure); perfect is 0.
- **Types**: the declared type of the return value, every parameter and every local; perfect
  is every one exact.
- **Bytes**: the decompiled class is recompiled with the javac that best reproduces the
  shipped class files, and the method's normalized bytecode is compared with the original;
  perfect is an identical listing.

A method is *perfect on all metrics* when it is perfect on every one that applies to it. A
method the decompiler loses or fails stays in every denominator. The scoreboards also report
mean GED, type matches at the erasure level, bytecode similarity, the share of classes that
recompile as written, and the decompiler's wall time, CPU time and peak memory.

The harness has a ceiling of its own: the source pseudo-decompiler (`--decompiler source`,
the original source scored as if it were a decompiler's output) reaches 100% on structure
and 99.98% on bytes on the five in-sample projects and misses 1% of variable types (source-level
`var`), and on the second held-out set it reaches 92.79% of methods perfect, because ecj is
compiled by the Eclipse compiler, whose bytecode no javac reproduces (about 70% of its methods
can be byte-exact), and luaj was compiled for Java 1.3.

## Running it

It needs the Python packages and JDK homes `scripts/javabench/README.md` lists, and about
2 GB of disk for the downloads of the default set plus 1 GB for full runs of both tools;
`fetch` downloads the projects, coursier, Temurin 25 and the Vineflower jar, each checked
against a pinned SHA-256. Everything goes to one workspace directory. From the repository
root:

```bash
export JAVABENCH_WORKSPACE=/path/to/workspace
scripts/javabench/fetch.sh                    # the five projects; --project heldout1 | heldout2 | all for the others
(cd decompiler && CARGO_INCREMENTAL=0 cargo build --release -p kuna-cli)
for p in guava spring-boot spring-framework ghidra elasticsearch; do
  python3 -m scripts.javabench run --decompiler kuna --kuna-bin decompiler/target/release/kuna \
      --project $p --full --jobs 12 --run-name kuna-$p
  python3 -m scripts.javabench run --decompiler vineflower --project $p --full --jobs 12 --run-name vf-$p
  python3 -m scripts.javabench compare kuna-$p vf-$p --project $p
done
```

`run` decompiles and scores (`--jobs` is the scoring parallelism and Vineflower's `-thr`;
kuna runs with its defaults), writing `$JAVABENCH_WORKSPACE/results/<run>/` with
`scoreboard.md`, `scoreboard.json` and one record per method. `compare` lists the methods one
run has perfect and the other does not. `--project` takes project names and the set names
above. Without `--full`, `run` scores a 3,000-method sample per project in a few minutes.

## Results

Each cell is Vineflower's score, then kuna's where it differs. *Structure perfect* is GED 0,
*types perfect* is every declared type of the method exact, *bytes exact* is an identical
recompiled listing, *classes recompile* is the share of decompiled classes javac accepts as
written, and *methods lost* counts methods a decompiler failed on or whose text the harness
cannot find or parse. Timings are in `docs/jvm.md`, "Performance and limits".

### The five projects the fixes were first developed on

| Project | Methods | Perfect on all metrics | Structure perfect | Types perfect | Bytes exact | Classes recompile | Methods lost |
|---|--:|---|---|---|---|---|---|
| guava | 12,167 | 91.86% → 98.79% | 98.19% → 99.88% | 99.64% → 99.97% | 91.92% → 98.85% | 84.04% → 94.55% | 0 |
| spring-boot | 3,967 | 88.98% → 99.34% | 97.09% → 99.97% | 99.41% → 99.95% | 89.01% → 99.37% | 95.04% → 97.16% | 1 → 0 |
| spring-framework | 13,874 | 90.83% → 99.14% | 97.45% → 99.90% | 99.36% → 99.92% | 90.88% → 99.24% | 95.75% → 98.88% | 2 → 0 |
| ghidra | 18,153 | 80.05% → 98.72% | 90.60% → 99.79% | 98.56% → 99.83% | 80.18% → 98.88% | 98.09% → 99.73% | 0 |
| elasticsearch | 62,077 | 84.20% → 98.92% | 96.80% → 99.83% | 99.62% → 99.93% | 84.26% → 99.03% | 91.82% → 97.67% | 14 → 0 |
| **all** | 110,238 | 85.37% → 98.92% | 96.02% → 99.84% | 99.41% → 99.92% | 85.44% → 99.02% | 93.29% → 98.03% | 17 → 0 |

109,044 methods are perfect on every metric instead of 94,106, every method is decompiled, and
386 more classes recompile (7,994 of 8,155 instead of 7,608). Most of the gain in perfect
methods comes from laying code out as javac laid out the bytecode: Vineflower writes two
returns as one `return c ? a : b`, negates ifs and swaps their branches against the bytecode's
order, and folds an early `return;` or `continue;` into a negated `if` around the rest, all of
which recompile to equivalent code laid out differently from the original. Most of the gain in
classes that recompile comes from fixes, as the javac errors that go away show: raw generic
constructors (`array required, but found` in 106 Elasticsearch classes), imports of non-public
nested library classes, unchecked type patterns, `assert` statements printed where Java needs
an expression, enums without the `;` before their members, and the casts and type arguments
generic inference needs. Every class that recompiles from Vineflower's output also recompiles
from kuna's.

Method by method, 15,393 methods score better than Vineflower's on some metric and worse on
none, and 23 score worse on at least one. By the verdicts described under `heldout2` below,
in 15 kuna's text is the source's reading or an equally right one and another difference in
the method shifts a metric: nine follow the code where Vineflower rewrites it (seven of them
print the early `return` or `continue` the code has, with bytes closer to the original and an
approximate GED that is higher), and six are a declaration placed where the source has it, an
empty `case` label, or noise in a score. Eight are kuna's to fix:
three loops printed as `while (true)`, a `break` out of one printed as `return`, code after a
switch drawn into its `default` arm, a try-catch nested in a try-finally where the source has
one `try`, a condition written `!(a && !b)`, and a resource the variable table names printed
as `try (toWrite)`.

### Eleven more projects (`heldout1`)

When this set was first measured, with an engine that had seen none of it, kuna had 92.39% of
its methods perfect against Vineflower's 81.68%. That run showed defects kuna introduced
(a local split into two variables, switch case groups reordered, an `@Override` javac rejects)
and failure shapes it shared with Vineflower (casts that select an overload dropped, a
qualified `super()` call dropped, raw locals captured by lambdas). They were fixed from what
the class files prove, each with a small original reproducer in the corpus, and the set now
reads:

| Project | Methods | Perfect on all metrics | Structure perfect | Types perfect | Bytes exact | Classes recompile | Methods lost |
|---|--:|---|---|---|---|---|---|
| antlr4 | 1,447 | 83.07% → 97.44% | 95.12% → 99.08% | 98.65% → 99.93% | 83.41% → 98.13% | 98.62% → 99.31% | 0 |
| h2 | 12,095 | 78.45% → 89.14% | 94.94% → 99.09% | 91.38% → 93.76% | 81.97% → 93.71% | 86.55% → 97.96% | 76 → 0 |
| httpcore5 | 5,138 | 91.32% → 96.96% | 97.51% → 99.78% | 99.66% → 99.96% | 91.48% → 97.04% | 99.40% → 99.80% | 0 |
| fastjson2 | 6,645 | 67.96% → 93.27% | 88.50% → 98.50% | 98.86% → 99.74% | 68.16% → 93.56% | 91.14% → 98.41% | 0 |
| tink | 4,384 | 90.10% → 99.59% | 98.41% → 99.80% | 99.75% → 99.85% | 90.21% → 99.70% | 98.21% → 100.00% | 3 |
| janino | 4,095 | 51.36% → 95.82% | 97.16% → 98.54% | 98.00% → 98.49% | 51.65% → 96.34% | 95.50% → 98.20% | 40 → 38 |
| rhino | 6,178 | 85.22% → 97.72% | 95.34% → 99.34% | 99.29% → 99.80% | 85.45% → 97.94% | 95.80% → 99.10% | 0 |
| joda-time | 3,910 | 86.85% → 97.77% | 97.87% → 99.90% | 98.00% → 98.57% | 87.24% → 98.24% | 95.74% → 99.29% | 0 |
| commons-math3 | 8,073 | 87.44% → 97.08% | 97.85% → 99.89% | 99.54% → 99.97% | 87.48% → 97.09% | 96.38% → 97.32% | 4 → 1 |
| jenetics | 1,915 | 91.49% → 97.28% | 99.41% → 99.95% | 99.41% → 99.89% | 91.85% → 97.39% | 88.51% → 92.53% | 2 → 0 |
| nashorn | 9,309 | 87.94% → 97.97% | 96.48% → 98.97% | 99.71% → 99.87% | 88.24% → 98.52% | 93.42% → 95.18% | 1 |
| **all** | 63,189 | 81.68% → 95.49% | 95.80% → 99.28% | 97.69% → 98.52% | 82.52% → 96.60% | 94.12% → 98.04% | 126 → 43 |

60,341 methods perfect instead of 51,610, and 4,254 of 4,339 classes recompile instead of
4,084. 9,956 methods score better than Vineflower's and 81 worse. h2 and joda-time ship
without local-variable tables, which is where the types column stays lowest for both tools.

### Ten projects nothing had been tuned against (`heldout2`)

Chosen after the fixes above, by a rule written before any decompiler ran on any candidate,
and measured once with an engine that had seen none of it: **89.28% of methods perfect
against Vineflower's 79.56%**, under a ceiling of 92.79%, and 2,841 of 2,952 classes
recompiling against 2,754. That is the out-of-sample number. The same run had 716 methods
scoring worse than Vineflower's, 601 of them in ecj, which the Eclipse compiler compiled:
kuna laid its code out as javac would have compiled it, and on bytecode javac did not write
that inference is unfounded. It also had 13 methods in classes that recompile from
Vineflower's text and not from kuna's, and one method kuna failed.

What that run showed was then fixed, so this set is no longer out of sample either. A class
now records which compiler wrote it, read from the order of its constant pool, and ECJ's
code is read as ECJ's (spec chapter 11, sections 11.3 and 11.9); the failed method and the
classes that did not recompile were fixed from what their class files prove. The set now
reads:

| Project | Methods | Perfect on all metrics | Structure perfect | Types perfect | Bytes exact | Classes recompile | Methods lost |
|---|--:|---|---|---|---|---|---|
| parboiled | 348 | 83.33% → 90.52% | 97.97% → 100.00% | 93.60% → 95.06% | 86.21% → 92.82% | 87.88% → 90.91% | 0 |
| hsqldb | 9,959 | 75.13% → 88.27% | 92.49% → 98.48% | 89.52% → 92.26% | 78.00% → 92.25% | 94.32% → 97.78% | 2 → 0 |
| undertow | 7,542 | 86.50% → 97.55% | 95.15% → 99.32% | 99.43% → 99.91% | 86.62% → 97.69% | 94.55% → 98.59% | 0 |
| dsl-json | 1,074 | 80.26% → 98.04% | 95.61% → 100.00% | 99.72% → 100.00% | 80.45% → 98.04% | 93.75% → 98.75% | 0 |
| jasypt | 673 | 93.16% → 98.37% | 98.66% → 100.00% | 99.67% → 100.00% | 93.31% → 98.37% | 97.44% → 100.00% | 0 |
| ecj | 10,886 | 59.14% → 69.67% | 84.19% → 92.56% | 95.97% → 98.33% | 59.26% → 70.02% | 88.09% → 94.34% | 136 → 0 |
| luaj | 2,218 | 80.79% → 85.98% | 96.36% → 99.17% | 95.79% → 96.52% | 82.46% → 87.87% | 90.72% → 92.78% | 0 |
| junit4 | 1,473 | 90.77% → 97.69% | 97.67% → 99.86% | 99.03% → 99.93% | 90.77% → 97.69% | 96.97% → 97.58% | 0 |
| ojalgo | 13,686 | 90.68% → 98.61% | 97.65% → 99.76% | 99.82% → 99.96% | 90.81% → 98.73% | 92.53% → 95.23% | 0 |
| langchain4j | 3,599 | 88.25% → 95.58% | 97.24% → 99.94% | 99.86% → 100.00% | 88.25% → 95.61% | 97.14% → 98.18% | 0 |
| **all** | 51,458 | 79.56% → 89.48% | 93.36% → 97.94% | 96.70% → 97.93% | 80.29% → 90.47% | 93.29% → 96.68% | 138 → 0 |

46,044 methods perfect instead of 40,939, and 2,854 of 2,952 classes recompile instead of
2,754. 7,501 methods score better than Vineflower's and 172 worse (ecj 96, hsqldb 49,
undertow 10, ojalgo 9, luaj 3, langchain4j 3, dsl-json 2); every method is decompiled. One of
ecj's is a method whose earlier text, like Vineflower's, left a loop at its first element and
scored better than the text that is right ("What the recompiled projects refer to" has it).

On ecj the structure column rises to 92.56% while bytes exact falls from the first run's
74.41% to 70.02%, with the same 69.67% of methods perfect. ECJ compiles `return a <= b;` as
two returns of constants, the code javac writes for `if (a <= b) return true; return
false;`. Where ECJ's line table shows one statement, kuna now prints the returned condition
the source has, which javac recompiles to one return; the two-statement form recompiled to
ECJ's bytes and had the wrong structure. 527 methods changed this way.

The 276 methods of the three sets where Vineflower's text scores better on some metric (23,
81 and 172) each have a verdict: 96 from reading both texts against the source, 180 from the
shape of the difference and from which text's lines the source has (eight of the largest
group were then read, and held). In 115 both texts are
equally right and a metric prefers one: the order of `case` groups, which the class file does
not record (32); structure that is equal and scored apart by the approximate GED or the byte
alignment (31); a declaration placed where the source or the slots put it (14); empty `case`
labels (10); the slot a constant local such as `final int prime = 31` leaves unused (9). In
111 kuna follows the code where Vineflower rewrites it, and each wins a metric: 75 print the
early `return` or `continue` the code has, 22 keep the locals the source has where Vineflower
merges slots, 6 print the two returns of the code where Vineflower returns a conditional. 50
are kuna's to fix: loops printed as `while (true)` (11), `instanceof` patterns in classes the
Eclipse compiler wrote (9), a generic varargs call written with its empty array (6), blocks
that only a variable's scope shows (5), a local split in two (5), an unneeded cast of `null`
(3), and eleven others in groups of one or two.

## Round trip of small sources

The three sets are large projects, and all but five of the 26 ship local-variable tables.
`python3 -m scripts.javabench.roundtrip` asks a stricter question of small sources, with
debug information and without: compile one source with javac, decompile each top-level
class, recompile the text with the same flags, and compare the code of every method. On
Vineflower's own test sources for Java 8, 16, 17 and 21 (481 sources, of which 462 compile
alone with `--release 21`):

| | Round-trips | Code differs | Text does not compile |
|---|--:|--:|--:|
| kuna, `-g` | 353 | 102 | 6 |
| Vineflower 1.12.0, `-g` | 288 | 145 | 29 |
| kuna, `-parameters -g:none` | 332 | 124 | 6 |
| Vineflower 1.12.0, `-parameters -g:none` | 245 | 158 | 59 |

With debug information kuna round-trips 67 sources Vineflower does not and Vineflower two
that kuna does not (`TestDoWhileTrue`, `TestLocalScopes`); without it, 87 and none. (One
more source, `TestRecordPatterns8`, compiles back to equal code with `-g` under a comment
about a duplicated exception handler and is counted in neither column.) A source
whose code differs is a list to read, not a defect count: javac lays out equivalent text
differently, and without debug information it merges equal lambda bodies. Reading kuna's
lists method by method found twenty-three shapes of wrong code that no benchmark project
had shown, most of them Vineflower's too and several only in code without local-variable
tables (an `int` local printed as `byte`, an increment folded into a condition that does not
always evaluate it, a static initializer moved into its declaration ahead of a call that
runs before it, a constant captured by a lambda with the wrong type, `finally { throw t; }`
moved into its `try`, code after a loop copied under a `catch` that would catch it, a
`catch` body taken for the increment of a for-each); each is fixed with a corpus case, and
no shape of wrong code is known to be left in these lists (the methods whose code differs
only in the layout of jumps were sampled, not all read). The lists are not where it ends.
Sources written by hand against each new rule found seven more shapes that neither the lists
nor the projects' scores had shown, all printed by Vineflower too and all in ordinary code:
`count += 2` moved into its declaration as `= 2`, from a static initializer and from the
constructors; an argument whose generic type selects another overload than the one the
bytecode calls (`show((Object) list.get(0))` beside a `show(String)`, printed without the
cast); a lambda printed with the captured variable of another lambda, because javac
compiles lambdas of equal bodies to one method unless it is given `-g`; a value inlined
into a lambda's capture and lost (`Object o = c.toString(); add(() -> o.hashCode())`
printed as `add(() -> var0.hashCode())`); a blank `static final` assigned a constant in
a static block, printed as a constant variable that javac then inlines; and a comparison or
conditional that mixes `float` with `int` or `long`, printed without the conversion that
decides the type it computes in (`(double) f == i` as `f == i`, which rounds the `int`). With the
typing of lambdas under raw types, these fixes made 889 more methods perfect in the DEX
benchmark below from debug builds and 844 from release builds, where no score had pointed
at them.
Of the text that does not compile, what is left is four sources whose types the
class file states only in part (generic casts javac compiles to no instruction, a raw
`new` that looks like a diamond) and two structures (a switch expression as the argument
of `this(..)`, which Vineflower's own expected text gets wrong the same way, and a local
class written twice). On kuna's own corpus sources (189 compile) the same command gives
153 / 35 / 0 with debug information and 133 / 52 / 3 without; one more source carries a
failure comment in both.

## What the recompiled projects refer to

The benchmark's metrics compare a decompiler's text with the project's source, so a call that
resolves to another overload, or a loop that leaves where the source continues, can score as
well as the source while the program means something else. `python3 -m scripts.javabench.refs`
asks the round trip's question of the three sets, from the class files a `run --full` already
recompiled: for every class of every file whose text compiled as written, it compares each
method of the original with the recompiled one as the sequence of what the code refers to
(invoked members, `invokedynamic` arguments, field reads and writes, the types of `new`, casts
and `instanceof`, constants, operators and conversions, switch keys grouped by target, and the
references each exception range covers). Owners are resolved through the classpath and the
JDK, so `ArrayList.add` for `List.add` is the same call, and the recompiling javac's own
choices (enum switch maps, `$values()`, javac 19's `String.valueOf` in `+`, implicit null
checks, capture field names, try-with-resources desugaring) read as equal;
`scripts/javabench/README.md`, "Reference sequences", has the rules. Kuna before and after the
fixes below:

| Set | Methods compared | Equal | Differing | of which not only layout |
|---|--:|--:|--:|--:|
| in-sample | 116,139 → 116,230 | 115,894 → 115,985 | 245 → 245 | 57 → 55 |
| heldout1 | 61,337 | 60,792 → 60,813 | 545 → 524 | 117 → 95 |
| heldout2, ecj apart | 41,267 | 40,928 → 40,932 | 339 → 335 | 22 → 18 |
| ecj | 9,399 | 8,326 → 8,327 | 1,073 → 1,072 | |

More in-sample methods are compared after because two Elasticsearch files now compile. ecj is
a row of its own: the Eclipse compiler wrote it, and javac's recompilation of the same text
differs in shape throughout (ECJ's enum switch tables, finally copies and branch layout).

Reading every method whose difference is not only layout found eight shapes of wrong code, each
in Vineflower too, each now fixed from what the class file proves and pinned by a corpus case
(spec chapter 11): a cast that selects an overload dropped when the argument's type is outside
the class context (`distance(Vector v) { return distance((Point) v); }` printed as a call of
itself, 12 commons-math3 methods, whose benchmark input lacks the codeless `Point` and `Vector`
interfaces); an upcast that selects an overload (`arr((Named[]) items)`); a widening that
selects an inherited overload beside a generic one (ojalgo's `invoke(float)` printed calling
itself); an unboxing beside an inherited boxed overload (fastjson2's
`validate(value.doubleValue())`, and ojalgo's `invoke(x.doubleValue())` through a generic
interface); a widening that rounds under another conversion (fastjson2's `(long) (float) i`
printed `(long) i`); a loop update reached from one switch case or one catch handler (luaj's
and joda-time's number scanners returned at their first digit, Ghidra's rename retry loop
stopped at the first duplicate, ECJ's `hasOnlyTrivialExceptionBounds` returned false for every
bound); a loop inside a `finally` printed with no exit (janino's close loop; an Elasticsearch
class whose code after such a loop javac rejected now compiles); and a concatenation constant
holding `\u0001` (Elasticsearch's range-query sentinel printed as `""`).

What is left is layout (blocks in another order, a `finally` or `return` copied or merged, a
`StringBuilder` chain written as `+`) or not wrong code: casts and conversions javac adds or
drops because kuna types a local or a generic value more narrowly (`byte` for a local holding
small constants, a `checkcast` for a generic result), method references javac compiles through
a synthetic lambda printed as lambdas, upcasts that only silence a varargs warning, and
exception ranges that cover the same code. Not decided from the class files: 25 commons-math3
calls whose argument is a class of the input implementing the missing `Vector` interface,
where only `Vector`'s relation to `Point` would show whether the cast selects (with the full
jar as context kuna writes it), a `call((Object) null)` beside an `Object...` overload of a
library class outside the input, and a varargs method reference printed as a lambda, which
loses the reference's null check.

## From DEX

The same five projects compiled to DEX with d8 and decompiled from the DEX are scored by the
same harness against jadx 1.5.3: 70.74% of methods perfect from debug builds (jadx 44.31%)
and 57.38% from release builds (jadx 38.57%), kuna ahead on each project in both.
`docs/jvm.md`, "Android: DEX and APK input", has the per-project tables and what a DEX costs relative to
the class files.
