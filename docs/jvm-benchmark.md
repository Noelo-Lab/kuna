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

It needs Python 3 with tree-sitter-languages 1.10.2, networkx, numpy, scipy,
diff-match-patch and psutil, JDK 17 and 21 homes, and about 2 GB of disk for the downloads
plus 1 GB for full runs of both tools; `fetch` downloads the projects, coursier, Temurin 25
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
kuna runs with its defaults; to score kuna in compat mode, give `--kuna-bin` a script that
runs kuna with `--option jvm_vineflower_compat on` added to `decompile-project`), writing `$JAVABENCH_WORKSPACE/results/<run>/` with
`scoreboard.md`, `scoreboard.json` and one record per method. `compare` lists the methods one
run has perfect and the other does not. Without `--full`, `run` scores a 3,000-method sample
per project in a few minutes.

## Result

kuna's engine has two modes (`docs/jvm.md`, "The fidelity contract"): compat mode
(`--option jvm_vineflower_compat on`) prints Vineflower's text, and the default mode is the
same text with kuna's fixes of Vineflower's defects applied. Both were scored against
Vineflower 1.12.0 on all 110,238 methods.

### Compat mode: the same scores as Vineflower

Every score field is identical between kuna in compat mode and Vineflower on all five
projects: the scoreboards match, and so does every per-method record (GED, type and
bytecode scores, perfect flags, failures). The benchmark shows that the port's output is
Vineflower's on large real-world inputs; the timing columns show what the port costs.

The timings were measured on one 80-core machine with both tools run back to back, one
project at a time: kuna with default flags (before its fixes were added, when its output
was compat mode's), Vineflower 1.12.0 on JDK 25 (Temurin 25.0.5) with `-thr=12`, each run
once while the machine had other work. Wall and CPU time are the decompiler process's (user
plus system); peak memory is the larger of its own maximum resident set and the sampled
resident set of its process tree.

| Project | Methods | Perfect on all metrics (both tools) | kuna wall / CPU / peak memory | Vineflower wall / CPU / peak memory |
|---|---|---|---|---|
| guava | 12,167 | 91.86% | 2.1 s / 41 s / 1.5 GiB | 4.5 s / 87 s / 3.8 GiB |
| spring-boot | 3,967 | 88.98% | 0.7 s / 10 s / 0.7 GiB | 2.7 s / 49 s / 1.6 GiB |
| spring-framework | 13,874 | 90.83% | 3.2 s / 47 s / 1.5 GiB | 6.0 s / 119 s / 4.7 GiB |
| ghidra | 18,153 | 80.05% | 3.4 s / 70 s / 1.4 GiB | 7.1 s / 138 s / 6.9 GiB |
| elasticsearch | 62,077 | 84.20% | 17.7 s / 357 s / 7.4 GiB | 29.2 s / 395 s / 11.5 GiB |

A method that is not perfect is one where Vineflower's output (and so kuna's) departs from
the source, or one of the few the harness cannot score perfectly even from the source
itself: the source pseudo-decompiler (`--decompiler source`) reaches 100% on structure,
99.98% on bytes and 98.96% of variable types, the misses being source-level `var`
(`scripts/javabench/README.md`).

### Default mode: what kuna's fixes and options change

kuna with default flags against the same Vineflower runs: its fixes, and the three options
that are on by default, `jvm_javac_layout` (ifs, exits and boolean comparisons laid out as
javac laid out the bytecode), `jvm_source_locals` (the locals the class file names kept,
declared in slot order) and `jvm_source_patterns` (a test and a cast where the source wrote
them, not an `instanceof` pattern). Each cell is Vineflower's score, then kuna's where it differs; *structure perfect*
is GED 0, *types perfect* is every declared type of the method exact, *bytes exact* is an
identical recompiled listing, *classes recompile* is the share of decompiled classes javac
accepts as written, and *methods lost* counts methods the decompiler failed on or whose
text does not parse.

| Project | Methods | Perfect on all metrics | Structure perfect | Types perfect | Bytes exact | Classes recompile | Methods lost |
|---|---|---|---|---|---|---|---|
| guava | 12,167 | 91.86% → 98.17% | 98.19% → 99.81% | 99.64% → 99.97% | 91.92% → 98.22% | 84.04% → 92.73% | 0 |
| spring-boot | 3,967 | 88.98% → 98.94% | 97.09% → 99.95% | 99.41% → 99.95% | 89.01% → 98.97% | 95.04% → 96.69% | 1 → 0 |
| spring-framework | 13,874 | 90.83% → 98.57% | 97.45% → 99.89% | 99.36% → 99.90% | 90.88% → 98.66% | 95.75% → 98.73% | 2 → 0 |
| ghidra | 18,153 | 80.05% → 97.47% | 90.60% → 99.71% | 98.56% → 99.81% | 80.18% → 97.61% | 98.09% → 99.73% | 0 |
| elasticsearch | 62,077 | 84.20% → 98.25% | 96.80% → 99.80% | 99.62% → 99.91% | 84.26% → 98.35% | 91.82% → 97.40% | 14 → 0 |

Over all five projects, 108,228 methods are perfect on every metric instead of 94,106
(98.18% instead of 85.37%), every method is decompiled, and 361 more classes recompile
(7,969 of 8,155 instead of 7,608). Most of the gain in perfect methods comes from
`jvm_javac_layout`: Vineflower writes two returns as one `return c ? a : b`, negates ifs
and swaps their branches against the bytecode's order, and folds an early `return;` or
`continue;` into a negated `if` around the rest, all of which recompile to equivalent code
laid out differently from the original. Most of the gain in classes that recompile comes
from the fixes, as the javac errors that go away show: raw generic constructors (`new
C(..)` for a library class `C`, whose lambda arguments then lose their parameter types;
`array required, but found` in 106 Elasticsearch classes), imports of non-public nested
library classes (22), unchecked type patterns (20), `assert` statements printed where Java
needs an expression, enums without the `;` before their members, and the casts and type
arguments generic inference needs (`incompatible types`: 129 classes to 82 in
Elasticsearch, 60 to 27 in Guava, 41 to 10 in spring-framework). Every class that
recompiles from Vineflower's output also recompiles from kuna's.

Method by method, 14,905 methods score better on some metric and worse on none, and 45
score worse on at least one. 37 of them come from the three options:

- 16 lose only bytes similarity, and 8 more lose bytes similarity while their GED or types
  improve: the recompiled method follows the source's layout, local order or test-and-cast
  more closely, but another difference (a reordered `switch`, a loop shape, a variable
  Vineflower merges) shifts how the benchmark aligns its listing with the original's.
- 12 have a GED one to three higher, 9 of them with bytes closer to the original: an
  early exit kept as in the bytecode (`if (..) return;`), a comparison with `false` kept as
  the source wrote it, or an `else if` chain of `instanceof` tests adds a node to the
  statement graph of a method whose structure differs from the source's elsewhere.
- spring-framework's `AbstractClassGenerator.generate` loses one declared type, with bytes
  closer to the original.

The other 8 come from fixes:

- spring-framework's `ClassReader.readTypeAnnotations`: GED 103 to 270 (bytes similarity 0
  to 0.91). Vineflower turns its switch statement, whose cases assign the variable more
  than once or not at all, into a switch expression javac rejects; fix
  `switch_expression_assignments` keeps the statement, written the way Vineflower writes
  any switch statement, with the 60 labels that share `default` one per line, which the
  statement-level graph counts.
- Elasticsearch's `ProjectMetadata.filterFields`: 13 of 15 types right instead of 14 (bytes
  similarity 0 to 0.86). Vineflower writes `value instanceof Map<String, Object> map`, which
  javac rejects as unchecked; fix `unchecked_type_pattern` makes it a test and a cast, and
  the second local the source names `map`, in a later block, is then the same variable,
  so one of the source's declarations has no counterpart.
- Four lose one declared type: Elasticsearch's two
  `SimulateBulkRequest.convertRawTemplateTo*`, `InternalEngine.noOp` and
  `DecayFunctionBuilder.doToFunction`. Their try-with-resources is now rebuilt (fix
  `twr_unresolved_resource`) and returns from inside the `try`, where the source assigns a
  local declared before it and returns that local after it.
- Elasticsearch's `TranslogWriter.writeAndReleaseOps`: 10 of 11 types instead of 11, with
  bytes now exact. The rebuilt resource is written `try (toWrite)`, Vineflower's form, where
  the source declares `try (ReleasableBytesReference toClose = toWrite)`.
- Elasticsearch's `TermVectorsService.getTermVectors` gets its try-with-resources back, with
  GED 64 to 0; its bytes similarity drops from 0.917 to 0.913.

`docs/jvm.md` ("Performance and limits") has separate timings, measured as the best of
three on a quiet machine with `kuna decompile-project` and Vineflower's default thread count.
