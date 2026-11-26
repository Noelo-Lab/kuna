# The JVM benchmark

`scripts/javabench` scores Java decompilers against the original source of five large
Java projects, method by method, the way decbench scores native decompilers
(`docs/decbench-loop.md`). It is how kuna's JVM engine is compared with Vineflower 1.12.0,
the decompiler it ports. The package's `README.md` has every definition and rule; this page
is the summary and the result.

## What it measures

The dataset is the five most-starred Java projects on GitHub that publish both a build and
its sources: Guava 33.8.0-jre, spring-boot 4.1.2, spring-framework 7.0.10 (core, beans,
context), Ghidra 12.2 (SoftwareModeling) and Elasticsearch 9.5.6 (the server). Every method
with code is either scored or excluded for a stated reason (synthetic and bridge methods,
static initializers, members with no source declaration, Kotlin and generated sources), so
110,238 methods are scored and every decompiler is judged on the same ones. A decompiler
gets the class files of the scored top-level classes as one jar, with default options.

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

## Running it

It needs the Python packages and JDK homes `scripts/javabench/README.md` lists, and about
2 GB of disk for the downloads plus 1 GB for full runs of both tools; `fetch` downloads the projects, coursier, Temurin 25
and the Vineflower jar, each checked against a pinned SHA-256. Everything goes to one workspace directory. From the repository root:

```bash
export JAVABENCH_WORKSPACE=/path/to/workspace
scripts/javabench/fetch.sh
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
run has perfect and the other does not. Without `--full`, `run` scores a 3,000-method sample
per project in a few minutes.

## Result

kuna and Vineflower 1.12.0 were scored on all 110,238 methods. A method that is not perfect
departs from the source, or is one of the few the harness cannot score perfectly even from the
source itself: the source pseudo-decompiler (`--decompiler source`) reaches 100% on
structure, 99.98% on bytes and 98.96% of variable types, the misses being source-level
`var` (`scripts/javabench/README.md`). Timings are in `docs/jvm.md`, "Performance and
limits".

### kuna against Vineflower

kuna with default flags against the same Vineflower runs: its fixes, and the three options
that are on by default, `jvm_javac_layout` (ifs, loops, exits and boolean comparisons laid out as
javac laid out the bytecode), `jvm_source_locals` (the locals the class file names kept,
declared in slot order and in the source's scopes) and `jvm_source_patterns` (a test and a cast where the source wrote
them, not an `instanceof` pattern). Each cell is Vineflower's score, then kuna's where it differs; *structure perfect*
is GED 0, *types perfect* is every declared type of the method exact, *bytes exact* is an
identical recompiled listing, *classes recompile* is the share of decompiled classes javac
accepts as written, and *methods lost* counts methods the decompiler failed on or whose
text does not parse.

| Project | Methods | Perfect on all metrics | Structure perfect | Types perfect | Bytes exact | Classes recompile | Methods lost |
|---|---|---|---|---|---|---|---|
| guava | 12,167 | 91.86% → 98.78% | 98.19% → 99.87% | 99.64% → 99.97% | 91.92% → 98.84% | 84.04% → 94.55% | 0 |
| spring-boot | 3,967 | 88.98% → 99.29% | 97.09% → 99.97% | 99.41% → 99.95% | 89.01% → 99.32% | 95.04% → 97.16% | 1 → 0 |
| spring-framework | 13,874 | 90.83% → 99.03% | 97.45% → 99.90% | 99.36% → 99.91% | 90.88% → 99.14% | 95.75% → 98.88% | 2 → 0 |
| ghidra | 18,153 | 80.05% → 98.71% | 90.60% → 99.78% | 98.56% → 99.83% | 80.18% → 98.86% | 98.09% → 99.73% | 0 |
| elasticsearch | 62,077 | 84.20% → 98.89% | 96.80% → 99.82% | 99.62% → 99.92% | 84.26% → 99.00% | 91.82% → 97.54% | 14 → 0 |

Over all five projects, 109,006 methods are perfect on every metric instead of 94,106
(98.88% instead of 85.37%), every method is decompiled, and 380 more classes recompile
(7,988 of 8,155 instead of 7,608). Most of the gain in perfect methods comes from
`jvm_javac_layout`: Vineflower writes two returns as one `return c ? a : b`, negates ifs
and swaps their branches against the bytecode's order, and folds an early `return;` or
`continue;` into a negated `if` around the rest, all of which recompile to equivalent code
laid out differently from the original. Most of the gain in classes that recompile comes
from the fixes, as the javac errors that go away show: raw generic constructors (`new
C(..)` for a library class `C`, whose lambda arguments then lose their parameter types;
`array required, but found` in 106 Elasticsearch classes), imports of non-public nested
library classes (22), unchecked type patterns (20), `assert` statements printed where Java
needs an expression, enums without the `;` before their members, and the casts and type
arguments generic inference needs (`incompatible types`: 129 classes to 76 in
Elasticsearch, 60 to 18 in Guava, 41 to 8 in spring-framework). Every class that
recompiles from Vineflower's output also recompiles from kuna's.

Method by method, 15,365 methods score better on some metric and worse on none, and 32
score worse on at least one. 27 of them come from the three options:

- 8 lose only bytes similarity, and 7 more lose bytes similarity while their GED or types
  improve: the recompiled method follows the source's layout, local order, declaration
  scopes or test-and-cast more closely, but another difference (a reordered `switch`, a
  loop shape, a variable Vineflower merges) shifts how the benchmark aligns its listing with the original's.
- 12 have a GED one to three higher, 9 of them with bytes closer to the original: an
  early exit kept as in the bytecode (`if (..) return;`), code kept after a loop where javac
  placed it, a comparison with `false` kept as the source wrote it, or an `else if` chain of
  `instanceof` tests adds a node to the statement graph of a method whose structure differs
  from the source's elsewhere.

The other 5 come from fixes:

- Four lose one declared type: Elasticsearch's two
  `SimulateBulkRequest.convertRawTemplateTo*`, `InternalEngine.noOp` and
  `DecayFunctionBuilder.doToFunction`. Their try-with-resources is now rebuilt (fix
  `twr_unresolved_resource`) and returns from inside the `try`, where the source assigns a
  local declared before it and returns that local after it.
- Elasticsearch's `TranslogWriter.writeAndReleaseOps`: 10 of 11 types instead of 11, with
  bytes now exact. The rebuilt resource is written `try (toWrite)`, Vineflower's form, where
  the source declares `try (ReleasableBytesReference toClose = toWrite)`.
