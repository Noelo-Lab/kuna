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
kuna runs with its defaults), writing `$JAVABENCH_WORKSPACE/results/<run>/` with
`scoreboard.md`, `scoreboard.json` and one record per method. `compare` lists the methods one
run has perfect and the other does not. Without `--full`, `run` scores a 3,000-method sample
per project in a few minutes.

## Result

Measured on one 80-core machine with both tools run back to back, one project at a time:
kuna with default flags, Vineflower 1.12.0 on JDK 25 (Temurin 25.0.5) with `-thr=12`, each
run once while the machine had other work. Wall and CPU time are the decompiler process's
(user plus system); peak memory is the larger of its own maximum resident set and the
sampled resident set of its process tree, written as javabench's MiB figure over 1,000.

| Project | Methods | Perfect on all metrics (kuna = Vineflower) | kuna wall / CPU / peak memory | Vineflower wall / CPU / peak memory |
|---|---|---|---|---|
| guava | 12,167 | 91.86% | 2.1 s / 41 s / 1.5 GB | 4.5 s / 87 s / 3.9 GB |
| spring-boot | 3,967 | 88.98% | 0.7 s / 10 s / 0.7 GB | 2.7 s / 49 s / 1.7 GB |
| spring-framework | 13,874 | 90.83% | 3.2 s / 47 s / 1.5 GB | 6.0 s / 119 s / 4.8 GB |
| ghidra | 18,153 | 80.05% | 3.4 s / 70 s / 1.5 GB | 7.1 s / 138 s / 7.1 GB |
| elasticsearch | 62,077 | 84.20% | 17.7 s / 357 s / 7.6 GB | 29.2 s / 395 s / 11.8 GB |

Every score field is identical between the two tools on all 110,238 methods: the
scoreboards match, and so does every per-method record (GED, type and bytecode scores,
perfect flags, failures). That is the expected result for a port whose output is
Vineflower's byte for byte (`docs/jvm.md`, "The fidelity contract"); the benchmark shows it
holds on large real-world inputs, and the timing columns show what the port costs. A method
that is not perfect is one where Vineflower's output (and so kuna's) departs from the source,
or one of the few the harness cannot score perfectly even from the source itself: the source
pseudo-decompiler (`--decompiler source`) reaches 100% on structure, 99.98% on bytes and
98.96% of variable types, the misses being source-level `var` (`scripts/javabench/README.md`).

`docs/jvm.md` ("Performance and limits") has separate timings, measured as the best of
three on a quiet machine with `kuna decompile-project` and Vineflower's default thread count.
