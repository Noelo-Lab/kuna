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
| spring-boot | 3,967 | 88.98% → 99.29% | 97.09% → 99.97% | 99.41% → 99.95% | 89.01% → 99.32% | 95.04% → 97.16% | 1 → 0 |
| spring-framework | 13,874 | 90.83% → 99.06% | 97.45% → 99.90% | 99.36% → 99.91% | 90.88% → 99.16% | 95.75% → 98.88% | 2 → 0 |
| ghidra | 18,153 | 80.05% → 98.72% | 90.60% → 99.78% | 98.56% → 99.83% | 80.18% → 98.87% | 98.09% → 99.73% | 0 |
| elasticsearch | 62,077 | 84.20% → 98.90% | 96.80% → 99.83% | 99.62% → 99.92% | 84.26% → 99.01% | 91.82% → 97.56% | 14 → 0 |
| **all** | 110,238 | 85.37% → 98.89% | 96.02% → 99.84% | 99.41% → 99.91% | 85.44% → 99.00% | 93.29% → 97.96% | 17 → 0 |

109,016 methods are perfect on every metric instead of 94,106, every method is decompiled, and
381 more classes recompile (7,989 of 8,155 instead of 7,608). Most of the gain in perfect
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

Method by method, 15,370 methods score better than Vineflower's on some metric and worse on
none, and 32 score worse on at least one: 27 where kuna's layout, local order, declaration
scope or test-and-cast follows the bytecode and another difference in the method shifts a
metric (15 lose bytecode similarity, 7 of them while their GED or types improve; 12 have a
GED one to three higher, 9 of them with bytes closer to the original), and 5 in Elasticsearch whose try-with-resources kuna rebuilds
where the source assigns a local before the `try` and returns it after.

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
| antlr4 | 1,447 | 83.07% → 97.37% | 95.12% → 99.08% | 98.65% → 99.93% | 83.41% → 98.06% | 98.62% → 99.31% | 0 |
| h2 | 12,095 | 78.45% → 89.00% | 94.94% → 99.08% | 91.38% → 93.70% | 81.97% → 93.55% | 86.55% → 97.83% | 76 → 0 |
| httpcore5 | 5,138 | 91.32% → 96.96% | 97.51% → 99.78% | 99.66% → 99.96% | 91.48% → 97.04% | 99.40% → 99.80% | 0 |
| fastjson2 | 6,645 | 67.96% → 93.06% | 88.50% → 98.50% | 98.86% → 99.74% | 68.16% → 93.35% | 91.14% → 97.05% | 0 |
| tink | 4,384 | 90.10% → 99.59% | 98.41% → 99.80% | 99.75% → 99.85% | 90.21% → 99.70% | 98.21% → 100.00% | 3 |
| janino | 4,095 | 51.36% → 95.78% | 97.16% → 98.52% | 98.00% → 98.49% | 51.65% → 96.29% | 95.50% → 98.20% | 40 → 38 |
| rhino | 6,178 | 85.22% → 97.62% | 95.34% → 99.34% | 99.29% → 99.78% | 85.45% → 97.86% | 95.80% → 98.80% | 0 |
| joda-time | 3,910 | 86.85% → 97.70% | 97.87% → 99.90% | 98.00% → 98.52% | 87.24% → 98.18% | 95.74% → 98.58% | 0 |
| commons-math3 | 8,073 | 87.44% → 96.84% | 97.85% → 99.89% | 99.54% → 99.97% | 87.48% → 96.85% | 96.38% → 97.32% | 4 → 1 |
| jenetics | 1,915 | 91.49% → 97.02% | 99.41% → 99.95% | 99.41% → 99.78% | 91.85% → 97.23% | 88.51% → 92.53% | 2 → 0 |
| nashorn | 9,309 | 87.94% → 97.97% | 96.48% → 98.97% | 99.71% → 99.87% | 88.24% → 98.52% | 93.42% → 95.18% | 1 |
| **all** | 63,189 | 81.68% → 95.39% | 95.80% → 99.28% | 97.69% → 98.50% | 82.52% → 96.49% | 94.12% → 97.83% | 126 → 43 |

60,274 methods perfect instead of 51,610, and 4,245 of 4,339 classes recompile instead of
4,084. 9,895 methods score better than Vineflower's and 87 worse. h2 and joda-time ship
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
| hsqldb | 9,959 | 75.13% → 88.22% | 92.49% → 98.48% | 89.52% → 92.25% | 78.00% → 92.14% | 94.32% → 97.04% | 2 → 0 |
| undertow | 7,542 | 86.50% → 97.53% | 95.15% → 99.31% | 99.43% → 99.91% | 86.62% → 97.68% | 94.55% → 98.42% | 0 |
| dsl-json | 1,074 | 80.26% → 98.04% | 95.61% → 100.00% | 99.72% → 100.00% | 80.45% → 98.04% | 93.75% → 98.75% | 0 |
| jasypt | 673 | 93.16% → 98.37% | 98.66% → 100.00% | 99.67% → 100.00% | 93.31% → 98.37% | 97.44% → 100.00% | 0 |
| ecj | 10,886 | 59.14% → 69.67% | 84.19% → 92.53% | 95.97% → 98.24% | 59.26% → 70.02% | 88.09% → 94.34% | 136 → 0 |
| luaj | 2,218 | 80.79% → 85.93% | 96.36% → 99.13% | 95.79% → 96.52% | 82.46% → 87.83% | 90.72% → 92.78% | 0 |
| junit4 | 1,473 | 90.77% → 97.69% | 97.67% → 99.86% | 99.03% → 99.93% | 90.77% → 97.69% | 96.97% → 97.58% | 0 |
| ojalgo | 13,686 | 90.68% → 98.30% | 97.65% → 99.76% | 99.82% → 99.93% | 90.81% → 98.45% | 92.53% → 95.07% | 0 |
| langchain4j | 3,599 | 88.25% → 95.58% | 97.24% → 99.94% | 99.86% → 100.00% | 88.25% → 95.61% | 97.14% → 98.18% | 0 |
| **all** | 51,458 | 79.56% → 89.38% | 93.36% → 97.93% | 96.70% → 97.90% | 80.29% → 90.37% | 93.29% → 96.51% | 138 → 0 |

45,994 methods perfect instead of 40,939, and 2,849 of 2,952 classes recompile instead of
2,754. 7,436 methods score better than Vineflower's and 198 worse (ecj 115, hsqldb 51,
ojalgo 14, undertow 10, luaj 3, langchain4j 3, dsl-json 2); every method is decompiled.

On ecj the structure column rises to 92.53% while bytes exact falls from the first run's
74.41% to 70.02%, with the same 69.67% of methods perfect. ECJ compiles `return a <= b;` as
two returns of constants, the code javac writes for `if (a <= b) return true; return
false;`. Where ECJ's line table shows one statement, kuna now prints the returned condition
the source has, which javac recompiles to one return; the two-statement form recompiled to
ECJ's bytes and had the wrong structure. 527 methods changed this way. What is left of the
198: ECJ's array for-each, which both decompilers print as an indexed loop (21 methods where
that makes Vineflower's nesting score closer), declarations in code without local-variable
tables (hsqldb, luaj), and locals kuna keeps where the bytecode has them and the recompiled
code differs.

## Round trip of small sources

The three sets are large projects, and all but five of the 26 ship local-variable tables.
`python3 -m scripts.javabench.roundtrip` asks a stricter question of small sources, with
debug information and without: compile one source with javac, decompile each top-level
class, recompile the text with the same flags, and compare the code of every method. On
Vineflower's own test sources for Java 8, 16, 17 and 21 (481 sources, of which 462 compile
alone with `--release 21`):

| | Round-trips | Code differs | Text does not compile |
|---|--:|--:|--:|
| kuna, `-g` | 343 | 103 | 16 |
| Vineflower 1.12.0, `-g` | 288 | 145 | 29 |
| kuna, `-parameters -g:none` | 319 | 122 | 21 |
| Vineflower 1.12.0, `-parameters -g:none` | 245 | 158 | 59 |

With debug information kuna round-trips 57 sources Vineflower does not and Vineflower two
that kuna does not (`TestDoWhileTrue`, `TestLocalScopes`); without it, 74 and none. A source
whose code differs is a list to read, not a defect count: javac lays out equivalent text
differently, and without debug information it merges equal lambda bodies. Reading kuna's
lists method by method found fifteen shapes of wrong code that no benchmark project had
shown, most of them Vineflower's too and several only in code without local-variable tables
(an `int` local printed as `byte`, an increment folded into a condition that does not always
evaluate it, static initializers moved into declarations out of order, a constant captured
by a lambda with the wrong type); each is fixed with a corpus case. Five known shapes are
left: four in how `try` regions are structured (`finally { throw t; }` moved into its `try`,
code after a loop copied under a `catch`, a `return` lost from a try-with-resources whose
`finally` returns, a lost `catch` body), and a record pattern's binding typed `Object`
without debug information. Of the text that does not compile without debug information,
what is left is mostly types the class file states only in part: a lambda whose target is
printed raw, and generic casts javac compiles to no instruction. On kuna's own corpus
sources (145 compile) the same command gives 120 / 24 / 0 with debug information and
102 / 37 / 5 without; one more source carries a failure comment in both.

## From DEX

The same five projects compiled to DEX with d8 and decompiled from the DEX are scored by the
same harness against jadx 1.5.3: 69.87% of methods perfect from debug builds (jadx 44.31%)
and 56.56% from release builds (jadx 38.57%), kuna ahead on each project in both.
`docs/jvm.md`, "Android: DEX and APK input", has the per-project tables and what a DEX costs relative to
the class files.
