# javabench

A DecBench-style benchmark that scores **Java decompilers against original
source**, per method, on the five most-starred real Java projects on GitHub.
It compares kuna's JVM engine with **Vineflower 1.12.0**; the results and how
they were measured are in `docs/jvm-benchmark.md`.

Three per-method metrics, each with a "perfect" flag, plus a union score --
the same philosophy as decbench's metrics (`docs/decbench-loop.md`):

| metric | measures | perfect |
|---|---|---|
| **structure (GED)** | graph edit distance between the statement-level CFG of the decompiled method and of the original source method (decbench's isomorphism -> VJ-GED -> size-lower-bound procedure) | GED = 0 |
| **types** | declared types of the return value, every parameter and every local vs the original source (exact and erasure level) | every variable exact |
| **bytes** | recompile the decompiled top-level class with javac, compare each method's normalized bytecode with the original (decbench's line-diff Jaccard) | normalized listing identical |

plus **% of top-level classes that recompile** as-is (headline), javac error
categories, decompiler failures, wall-clock, CPU time and peak RSS.

## Layout

```
scripts/javabench/     python package (python3 -m scripts.javabench ..., from the repo root)
  projects.py          pinned dataset definition (5 projects, javac per project), workspace paths
  fetch.py             reproducible fetch (coursier + checksums), never deletes
  classfile.py         class-file reader (cp, Code, LNT, LVT/LVTT, InnerClasses, EnclosingMethod)
  javasrc.py           tree-sitter-java indexer (types, methods, anon/local classes, scopes)
  cfg.py               the ONE CFG extractor (source and decompiled text alike)
  ged.py               decbench GED (imported when available; vendored fallback)
  jtypes.py            variable extraction, type normalization/erasure, pairing
  mapping.py           class-file method <-> declaration correspondence
  index.py             ground-truth index + exclusions + stratified sample
  decompilers.py       adapters: vineflower, kuna, source (oracle)
  recompile.py         javac server driver, stub salvage, byte comparison
  bytecode.py          symbolized instruction listings, similarity, class matching
  java/Recompile.java  javax.tools compile server (one JVM per shard)
  score.py run.py cli.py
  pins/<project>.json  every pinned artifact + SHA-256 (the lock the original fetch wrote)
  docs/dataset.md      ranked candidate list, why kept/dropped, pins, calibration
  tests/               hand-made metric tests
  fetch.sh             = python3 -m scripts.javabench fetch
```

## The workspace

Everything the benchmark downloads or writes goes to one directory, given by
`--workspace DIR` (before the subcommand) or `$JAVABENCH_WORKSPACE`; without
either, every command stops with a one-line message.

```
$JAVABENCH_WORKSPACE/
  dataset/<project>/   classes.jar, sources/, deps/, meta.json, index.json.gz, input-*.jar ($JAVABENCH_DATASET overrides)
  dataset/_cache/      downloads (Maven, coursier, the Ghidra release zip)
  results/<run>/       run outputs ($JAVABENCH_RESULTS overrides)
  tools/               coursier 2.1.26, Temurin 25.0.5, vineflower-1.12.0.jar, the compile server
  locks/<project>.json the lock of a project that has no pin
```

`fetch` downloads every input and verifies it against its pinned SHA-256:
the project artifacts against `pins/` (Maven Central, and Ghidra's GitHub
release zip), coursier and Temurin 25.0.5 against `projects.py`, and
Vineflower 1.12.0 (Maven Central, sha256 `1dfcfe97...548b9d`) against
`fetch.py`; `VINEFLOWER_JAR` uses another jar instead. JDK homes are
`$JAVABENCH_JDK17`, `$JAVABENCH_JDK19`, `$JAVABENCH_JDK21` (default: the
Debian/Ubuntu `openjdk-N` packages under `/usr/lib/jvm`) and `$JAVABENCH_JDK25`
(default: the Temurin build `fetch` installs). JDK 17 builds the compile
server, 21 and 25 recompile (per project, `docs/dataset.md`), Vineflower runs
on 25; 19 is only for `--javac-jdk 19` calibration.

Python packages: `tree-sitter-languages` 1.10.2 (with `tree-sitter` 0.21),
`networkx`, `numpy`, `scipy`, `diff-match-patch` (the byte similarity; without
it `difflib` is used, which can score differently), `psutil` (process-tree
RSS) and either decbench (`DECBENCH_SRC` names a checkout whose
`decbench.metrics` is imported) or `cfgutils` for the vendored copies of
decbench's two GED functions.

## Commands

From the repo root:

```bash
export JAVABENCH_WORKSPACE=/path/to/javabench-workspace

# (a) fetch everything (verifies every SHA-256)
scripts/javabench/fetch.sh                      # or: python3 -m scripts.javabench fetch [--project guava]
python3 -m scripts.javabench index              # ground-truth index (built lazily by run, too)

# (b) the default 3,000-method stratified sample per project, or every scored method (--full)
python3 -m scripts.javabench run --decompiler vineflower --jobs 12 --run-name vf-sample
python3 -m scripts.javabench run --decompiler kuna --kuna-bin decompiler/target/release/kuna \
    --jobs 12 --run-name kuna-sample
#     fast inner loop (no recompilation): add --skip-bytes; one project: --project guava

# (c) compare two runs: methods perfect in A but not B (and vice versa)
python3 -m scripts.javabench compare vf-sample kuna-sample [--project guava] \
    [--metric ged --metric types --metric bytes] [--limit 25] [--json cmp.json]

python3 -m scripts.javabench report vf-sample kuna-sample   # scoreboards
python3 -m scripts.javabench score <run> [--project P]       # re-score an existing run
python3 -m scripts.javabench score <run> --reuse-bytes       # re-score GED/types only, keep recompile results
python3 -m scripts.javabench run --decompiler source ...     # harness self-check / ceiling
python3 -m scripts.javabench run --decompiler source --project spring-boot --javac-jdk 21   # javac calibration
```

`--jobs` is the scoring parallelism and Vineflower's `-thr`; kuna runs with
its own defaults (`--jobs auto`).

The kuna adapter contract: `$KUNA_BIN decompile-project <input.jar> -o <outdir>`
(`--kuna-bin` or `KUNA_BIN`) writes one `.java` per top-level class
(`<outdir>/<pkg/path>/<Class>.java`), exit 0. Classes are discovered by
*parsing* the output, so the layout is not load-bearing. A non-zero exit or
zero `.java` files marks that project `skipped` (reason in `run.json` / the
report) instead of scoring it. A method body containing `$VF: Couldn't be
decompiled` (the comment kuna's JVM engine writes too) or `kuna: couldn't
decompile` counts as a decompiler failure (`Kuna.failure_regex`).

## Outputs (`results/<run>/`)

- `run.json` -- decompiler, version, per-project status, the exact command,
  wall-clock, CPU time, peak RSS.
- `<project>/decompiled/` -- the decompiler's output; `<project>/decompile.log`.
- `<project>/methods.jsonl` -- one record per scored method: `key`
  (`binary.Class#name(desc)`), `status` (`ok`, `class-missing`, `type-missing`,
  `method-missing`, `parse-error`, `decompiler-failed`), `ged`
  (`value`, `method`, block/edge counts), `types` (`total`, `exact`, `erasure`,
  `misses`), `bytes` (`similarity`, `exact`, `compiled`, `error`), the three
  perfect flags, `union_perfect`, `all_perfect`, and absolute paths + line spans
  of the source and decompiled declarations.
- `<project>/classes.json` -- per decompiled file: raw compile result, javac error
  categories (javac diagnostic codes), salvage rounds, stubbed methods.
- `<project>/recompile/` -- javac job files, outputs, stubbed sources.
- `<project>/scoreboard.{json,md}`, and run-level `scoreboard.{json,md}` (pooled).

`compare` prints, per project and metric, `both perfect / A-only / B-only`
counts and the A-only / B-only method lists (largest methods first) with both
scores, both statuses, `file:start-end` for both decompiled declarations and
the original source; `--json` writes the complete lists (plus type misses and
byte errors).

## Metric definitions

**Method correspondence.** Keys come from the class file. Named types match by
binary name. Anonymous/local classes: on the source side by
LineNumberTable lines (innermost declaration containing the class's method
lines whose enclosing type maps to the class's outer class); on the decompiled
side (no line info) by aligning, within each mapped type and supertype group,
the anonymous declarations in textual order with `Outer$N` in index order
(local classes per simple name). Methods: same name, erased parameter simple
names vs the descriptor (type variables erased to their bounds, including the
enclosing method's), allowing javac's synthetic leading parameters (enum
name/ordinal, outer instance) and trailing captures; import-qualified names
break simple-name ties (`Optional` vs `java.util.Optional`); line containment
breaks remaining source-side ties.

**Structure (GED).** `cfg.py` builds a statement-level CFG from tree-sitter-java
with one node per statement plus one per short-circuit (`&&`, `||`) and ternary
operand -- as Joern does for C -- then collapses chains into basic blocks with
PyJoern's merge rule; ENTRY/EXIT roles mark the blocks containing the synthetic
entry/exit. `return`/`throw` reach EXIT; `while(true)`, `for(;;)` and
`do{}while(true)` have no exit edge from the head; `break`/`continue` are pure
jumps without a node (Joern's CONTROL_STRUCTURE jumps are invisible in
decbench's projection); `try` has an edge to each
`catch` and to `finally`, whose exits also reach EXIT (rethrow path); abrupt
jumps leave through `finally`; switch groups fall through; labeled
`break`/`continue` are resolved; lambda bodies and anonymous/local class
bodies are opaque (separate methods in bytecode). GED is decbench's own
procedure (`decbench.metrics.ged._is_isomorphic`, `decbench.metrics.vj_ged.vj_ged`,
200-node VJ cap, size lower bound above it, floor 1 when not isomorphic).
Methods whose source CFG has no statements (empty bodies) are excluded from GED
(decbench's degenerate-CFG rule). **Mean GED** charges a method the decompiler
did not produce (missing, unparseable, or marked failed) the cost of building
the source CFG from nothing (|V|+|E|); `mean_scored_only` is also reported.
"GED perfect (>=2 blocks)" restricts to non-straight-line source methods.

**Types.** Variables: return type (methods), parameters (by position), and
every local declared in the body (local declarations, for/enhanced-for
variables, catch parameters, try-with-resources variables, pattern bindings),
excluding lambda and anonymous/local class bodies. Ground truth is the original
source declaration; for `var` it is the class file's LocalVariableTypeTable
signature, else LocalVariableTable descriptor (matched by name + order) -- a
decompiler printing `var` gets no credit. Pairing is type-blind: parameters by
position, locals by name (same-name declarations in order), leftovers by
order. Normalization drops annotations, `final`, whitespace and
package/outer qualifiers, maps varargs to arrays and `? extends Object` to `?`.
Reported: variable-level exact %, variable-level erasure % (generics dropped,
type variables erased to their declared bound on each side), and method-level
perfect % (all variables exact).

**Bytes.** Each decompiled file is compiled alone (`javac --release N` or
`-source N -target N` as the project's build did, `-g`, `-proc:none`,
`-implicit:none`, empty sourcepath) against `classes.jar` + `deps/`, with the
javac whose recompilation of the original sources best reproduces the shipped
bytecode (see `docs/dataset.md`). The raw result is the headline **classes
recompile %**. For byte scoring only, up to three salvage rounds replace the
bodies of methods that contain javac errors with `throw new
RuntimeException()` (line-preserving) so the healthy methods of a broken class
can still be compared (decbench compiles per function); stubbed methods score
0. Errors outside method bodies fail the whole file. Listings
(`bytecode.py`): constant-pool operands rendered symbolically, short forms
canonicalized (`iload_1`=`iload 1`, `wide` folded, `goto_w`=`goto`), branch
targets blanked for similarity, compiler-numbered synthetic names normalized
(`lambda$m$3`, `access$000`, `Outer$7`), one line per exception-table entry.
similarity = shared / (a_only + shared + b_only) over a line diff
(diff_match_patch, as decbench); exact = identical listing with branch targets
and exception ranges as instruction indices. Recompiled anonymous/local
classes are paired with the originals by (outer class, supertype, local name)
in index order.

**Decompiled-side tolerance.** `javasrc.sanitize` makes two offset- and
line-preserving, syntax-only rewrites before parsing (all sides; valid Java
never changes): type annotations before a varargs `...` are blanked (the
grammar cannot parse them), and a constant-less `enum X { static ... }` that
lost its `;` (a Vineflower bug on Elasticsearch's utility-holder enums) gets
one. Recompilation always uses the raw text, so the compile metric still
penalizes such output. A method that cannot be located because its file does
not parse is `parse-error` (unscored, counted); one the output never declares
is `*-missing`. A decompiled type that omits a constructor javac would
generate identically -- the default constructor of a type that declares none,
or a record's canonical constructor -- is treated as that implicit, empty
constructor (Vineflower drops explicit empty ones), not as missing.

**Union / all perfect.** `union_perfect` = perfect on >= 1 measurable metric
(decbench's Union); `all_perfect` = perfect on every measurable metric.

## Fairness rules

- One extractor, one parser, one normalizer, one javac setup for every
  decompiler and for the original source.
- Every denominator is fixed by the ground-truth index (decompiler-independent):
  a method a decompiler loses stays in all denominators and is never perfect.
- Exclusions are decided once, from the class file + original source, and
  apply to all decompilers equally (`index.json.gz` -> `excluded`): synthetic
  and bridge methods (incl. lambda bodies), `<clinit>`, members with no source
  declaration (default ctors, enum `values`/`valueOf`, implicit record members),
  classes with no Java source (Kotlin), methods with no code, and
  machine-generated sources (`$ANTLR`/JavaCC/`DO NOT EDIT`/`@Generated(` in the
  file header: Ghidra's 31 SLEIGH parser/lexer files, 1,469 methods; Guava's
  public-suffix table). Counts per project: `docs/dataset.md`.
- Decompiler input = the complete top-level classes (all nested classes
  included) that contain scored methods -- the sample jar, or `input-full.jar`
  for `--full` (so generated/Kotlin-only classes are never decompiled).
- Every decompiler gets byte-identical input (`input-<tag>.jar`) and the
  default options; for Vineflower only the output layout (`--folder`) and the
  output-neutral thread count (`-thr=<jobs>`; its default is one per core) are set.
- No project code is executed: annotation processing is disabled, nothing from
  the downloaded archives is run.

## Tests

`tests/test_metrics.py` holds hand-made cases with real javac:

- identical source as "decompiled" output -> GED 0, every variable exact, and
  the recompiled method's listing identical to the original (all three perfect);
- a deliberately restructured method (`&&` split into flag + two ifs,
  `continue` turned into if/else) -> VJ-GED >= 1 while untouched methods stay 0;
- `&&` == nested `if`, `while(true)` == `for(;;)`, `break` vs `if/else` are
  structurally equal; a loop vs a branch is not;
- wrong local types: `int`->`long` misses at both levels, `List<String>` ->
  `List<Object>` misses exact but matches at erasure;
- a method javac rejects (`a * "two"`) -> raw class compile fails with category
  `operator.cant.be.applied.1`, salvage stubs exactly that method (bytes 0,
  `compiled=False`) and the other two methods are still byte-exact;
- constant-pool-layout independence of listings; anonymous / local class and
  overload mapping on both sides (with and without line numbers); enum
  `valueOf(String)` not loosely matched to `valueOf(int)`; omitted default and
  record canonical constructors treated as implicit; type normalization rules
  (qualifiers, generic outers, annotations, varargs, wildcards, erasure).

```bash
PYTHONPATH=. python3 -m unittest scripts.javabench.tests.test_metrics    # 13 tests, ~10 s
```

They need the Python packages above and JDK 21 and 17 homes; without a package
every test is skipped, without a JDK the javac-based ones are. CI has neither,
so they are not part of `make test-tools`; what `make test-tools` runs is
`scripts/tests/test_javabench_workspace.py` (the workspace convention and the
pins, standard library only).

## Harness ceiling

The `source` pseudo-decompiler puts the original sources through the identical
pipeline:

| set | GED perfect | type var exact | bytes mean | bytes exact | classes recompile |
|---|---|---|---|---|---|
| sample (15,000) | 100.00% | 99.27% | 0.9999 | 99.98% | 100.00% |
| full (110,238) | 100.00% | 98.96% | 0.9999 | 99.98% | 99.96% |

Every type miss is a source-level `var` (deliberately not credited; mostly
Elasticsearch, 98.18% there). The three classes that do not recompile from
their own source are toolchain artifacts that hit every decompiler equally:
two objenesis classes Spring repackages at the bytecode level need
`sun.misc.Unsafe`, which `--release 8` hides, and one Elasticsearch class
extends a sealed class across packages, which only compiles inside the named
`org.elasticsearch.server` module (javabench compiles on the classpath).
Anything a decompiler loses below these numbers is the decompiler's.

## Known limitations / blind spots

- **GED is topology-only** (as in decbench): statement contents are never
  compared, so wrong expressions with the right shape score 0, and every
  straight-line method is trivially perfect -- read the ">= 2 blocks" column.
- **GED measures source-structure fidelity, not semantics**: an early `return`
  vs an `if`-wrapped tail, `if/else` returns vs a ternary, or Vineflower's
  `if (x) return true; return false;` -> `return x;` folding are all non-zero.
  `try`-with-resources is one block in source; desugared output (Vineflower 1.12
  does not re-sugar many javac 11+ TWR shapes) is heavily penalized.
- **Type pairing is name-first**: decompilers that keep LocalVariableTable names
  (Vineflower does; the datasets all have `-g`) pair exactly; one that renames
  everything falls back to declaration order, which can mis-pair. Locals the
  decompiler eliminated count as misses (Vineflower's dominant type loss).
  Erasure-level matching uses each side's own declared bounds.
- **Lambdas are not scored as methods** (their bodies are synthetic
  `lambda$...` methods): lambda reconstruction only shows up in the enclosing
  method's bytes (`invokedynamic` operands). `<clinit>` and field initializers
  are not scored either (constructor bytes include instance field initializers).
- **Anonymous classes on the decompiled side** are paired by supertype and
  order (no line info in decompiled text); a decompiler that reorders two
  anonymous classes of the same supertype inside one class gets compared
  against the wrong bodies.
- **Bytes depend on javac fidelity**: calibrated with the source oracle
  (99.98% exact), but a method can still differ for toolchain reasons. Branch
  targets are blanked in the similarity listing (exact keeps them); synthetic
  numbering is normalized, so re-numbered lambdas/anonymous classes are not
  penalized. A class that fails outside method bodies scores 0 for every method.
- **Sample inputs remove cross-class context** (other project classes are not
  in the input jar) -- identical for every decompiler; `--full` decompiles every
  scored class together.
- **Timing**: the sample jars are small, so wall-clock is dominated by start-up;
  peak RSS of a JVM mostly reflects heap sizing (`-Xmx16g`). Vineflower's
  deprecated per-method time limit is off by default, so a single pathological
  method can stall a project (the generated SLEIGH parser did, for >10
  CPU-minutes, before generated sources were excluded); the harness timeout
  (`--timeout`, default 7200 s per project) then marks the project skipped.
- **Grammar**: tree-sitter-java from `tree_sitter_languages` 1.10.2 (Java 21
  syntax). No original source in the dataset fails to parse; newer syntax in
  decompiler output would degrade to `parse-error`.
- Union perfect (decbench's definition, perfect on >= 1 metric) is weak
  (~99.8% for both decompilers); `all perfect` is the discriminating headline.
