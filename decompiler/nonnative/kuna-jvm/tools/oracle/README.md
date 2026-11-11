# The Vineflower oracle

kuna-jvm's tests compare its output with the text the released Vineflower 1.12.0 jar prints for the
same class files and options. This directory builds that ground truth: it replays Vineflower's own
test registrations (core's `SingleClassesTest`, the Kotlin plugin's `KotlinTests`) against class
files compiled from Vineflower's `testData` sources the way Vineflower's Gradle build compiles them,
and records what the jar prints. The vendored regression corpus in `tests/jvm/` (607 cases, the
`make test-jvm` gate) is cut from the two corpora built here; nothing in this directory runs in CI.

| Path | What |
|---|---|
| `fetch_vineflower.sh` | the release jar and a source checkout at the release commit |
| `java/` | the Java corpus: core's `SingleClassesTest`, 598 runnable tests |
| `kotlin/` | the Kotlin corpus: `KotlinTests` (66) and core's 7 `KOTLIN` tests |
| `*/regen.sh` | rebuild a corpus end to end |
| `java/bin/vf-decompile`, `kotlin/bin/vf-decompile-kt` | decompile anything through the test fixture's API path |
| `*/driver/src/` | the Java drivers behind those front-ends (`VfDriver`, `KtVfDriver`, `JasmCompile`) |
| `*/scripts/` | fetch, compile, manifest, oracle run, validation and cross-check steps |

## The workspace: `KUNA_JVM_ORACLE`

Everything these scripts download or produce goes to one directory outside the repository, named by
the environment variable `KUNA_JVM_ORACLE`. Every script that needs it and is run without it stops
with a one-line message naming the variable; the per-script overrides (`VF_JAR`, `VF_SRC`, `JDKS`,
`CORPUS`, ...) still take precedence where a script has them. The same variable is the default
source for `tests/jvm/regen.py`, `plugin_oracle.py`, `kotlin_corpus.py`, the engine's
`tests/data/*/gen.sh` scripts and `tools/vftrace`.

```
$KUNA_JVM_ORACLE/
  oracle/vineflower-1.12.0.jar     the release jar
  vineflower/                      Vineflower's sources at b8273988 (testData, test registrations)
  jdks/jdk{8,9,11,16,17,21,25}     JDK homes (links); jdks/kotlin/x200/kotlinc: kotlinc 2.0.0
  corpus/                          the Java corpus (below), its built driver and tools/asmtools/
  corpus-kt/                       the Kotlin corpus, its built driver, tools/{m2,plugin,realworld}/
  dataset/spring-framework/        optional: the javabench dataset (scripts/javabench), for the
                                   Kotlin real-world references and tests/data/kt1, kt3
  vftrace-build/                   tools/vftrace's build output
  work/                            per-run work directories (regen, crosscheck)
```

Requirements: Linux x86-64, bash, curl, git, unzip, python3, GNU `time` (`/usr/bin/time`, for the
real-world references), about 3 GB of disk for the JDKs, and enough memory for 16 JVMs at 1 GiB of heap
each (`JOBS` lowers the count). OpenJDK 17 and 21 are not downloaded: `jdks/jdk17` and `jdks/jdk21` link
to existing homes (`JDK17_HOME`, `JDK21_HOME`, default the Debian/Ubuntu packages
`/usr/lib/jvm/java-{17,21}-openjdk-amd64`). The shipped corpora were built with Ubuntu's OpenJDK
17.0.20.1 and 21.0.12.1.

```sh
export KUNA_JVM_ORACLE=/path/to/oracle-workspace
decompiler/nonnative/kuna-jvm/tools/oracle/java/regen.sh       # -> $KUNA_JVM_ORACLE/corpus
decompiler/nonnative/kuna-jvm/tools/oracle/kotlin/regen.sh     # -> $KUNA_JVM_ORACLE/corpus-kt
python3 -I tests/jvm/regen.py                                  # -> tests/jvm (see "Into tests/jvm")
```

`regen.sh [OUT_ROOT]` writes the corpus in place (overwriting, never deleting) or into a fresh
directory; compare two builds by their `SHA256SUMS`. A rebuild into a fresh directory reproduces
`SHA256SUMS` byte for byte (2,209 files for the Java corpus, 583 for the Kotlin one). After the
downloads, one corpus takes a few minutes with 16 JVMs; the Kotlin real-world references add a few
more (`SKIP_REALWORLD=1` skips them). `scripts/crosscheck.sh` (optional) re-runs the oracle under the
variations listed below and writes `reports/crosscheck.json`; that is eight more oracle passes for the
Java corpus, eleven and two recompiles for the Kotlin one.

## What is downloaded, and from where

Every download is checked against the hash pinned in the script that fetches it.

| What | From | Pin |
|---|---|---|
| `vineflower-1.12.0.jar` | Maven Central `org.vineflower:vineflower:1.12.0` | sha256 `1dfcfe974395734fa467ce620661c7623d05ba83670de0529b1fbd63ff548b9d` |
| Vineflower sources | `github.com/Vineflower/vineflower`, tag `1.12.0` | commit `b8273988af850e8cfb234ca08d129058502b032f` (a different HEAD warns) |
| JDK 8 | Temurin 8u512-b09 (`adoptium/temurin8-binaries`) | sha256 `fb3bfa86...43f7e4` |
| JDK 9 | AdoptOpenJDK 9.0.4+11 (foojay's "aoj"; Temurin has no 9) | sha256 `aa4fc8bd...b951f1` |
| JDK 11 | Temurin 11.0.33+10 | sha256 `c4b8eaf5...a6f0ec` |
| JDK 16 | Temurin 16.0.2+7 | sha256 `323d6d74...73c2c` |
| JDK 25 | Temurin 25.0.5+8 | sha256 `68982529...178d7fb` |
| `asmtools-core-7.0.b10-ea.jar` | Maven Central `org.openjdk.asmtools` (what Vineflower's `buildSrc` uses) | sha256 `5c204b9d...cd0388` |
| kotlinc 2.0.0 (build `2.0.0-release-341`) | `kotlin-compiler-2.0.0.zip`, GitHub release v2.0.0 of JetBrains/kotlin | sha256 `ef578730...49da3b` (= the release's `.sha256`); `lib/kotlin-compiler.jar` `06cff2f8...284697b` |
| kotlin-stdlib, -jdk7, -jdk8 2.0.0, kotlin-bom 2.0.0, annotations 13.0 and 24.0.0 | Maven Central (`tools/m2`) | `.sha1` from Maven Central and a sha256 each |
| okio 2.6.0, okhttp 4.7.2, kotlin-stdlib 2.2.20, annotations 13.0, kotlinx-coroutines-core-jvm 1.8.0, moshi-kotlin 1.9.3, kotlin-reflect 1.6.10 | Maven Central (`tools/realworld`) | `.sha1` from Maven Central and a sha256 each |

The full hashes are in `fetch_vineflower.sh`, `java/scripts/fetch_tools.sh` and
`kotlin/scripts/fetch_tools.sh`. The Kotlin plugin jar the release jar bundles
(`META-INF/plugins/vineflower-kotlin-0.1.0.jar`, sha256 `56d988c6...cd61b3b0`) is extracted by
`kotlin/driver/build.sh`. The spring-framework classes and their 48 libraries come from the javabench
dataset (`docs/jvm-benchmark.md`); `SPRING_DIR` points elsewhere.

## The Java corpus (`$KUNA_JVM_ORACLE/corpus`)

| Path | What |
|---|---|
| `classes/<set>/` | compiled test data: `java8 java8nodebug java9 java11 java16 java16nodebug java16preview java17 java17preview java21 java21preview java25 jasm`, and `custom/` (Vineflower's checked-in class files) |
| `manifest.json` | every test `SingleClassesTest` registers (611) |
| `expected/<set slug>/<test class>.java` | the fixture oracle, 598 files, raw bytes |
| `expected-defaults/<source set>/<test class>.java` | `java -jar vineflower-1.12.0.jar <same class files> <dir>`, no options, 598 files |
| `logs/`, `reports/`, `SHA256SUMS` | per-set compiler logs and exact arguments, run and validation reports, hashes of everything above |

**Source sets.** Each set is compiled by its own language version's javac with the arguments Gradle's
`JavaCompilerArgumentsBuilder` emits for `build.gradle`'s `createJavaTestDataSet`: `-source N -target N
-d <dir> -encoding UTF-8 -g -sourcepath "" -proc:none -XDuseUnsharedTable=true`, then the set's own
arguments, all sources in one invocation:

| Set | Compiler | Set arguments | Sources | Classes | Tests |
|---|---|---|---|---|---|
| `java8` | javac 1.8.0_512 (Temurin 8u512-b09) | | 357 | 601 | 347 |
| `java8nodebug` | javac 1.8.0_512 | `-g:none` | 14 | 22 | 14 |
| `java9` | javac 9.0.4 (AdoptOpenJDK 9.0.4+11) | `-sourcepath <set root>` | 9 | 9 | 4 |
| `java11` | javac 11.0.33 (Temurin 11.0.33+10) | | 2 | 3 | 2 |
| `java16` | javac 16.0.2 (Temurin 16.0.2+7) | | 88 | 119 | 85 |
| `java16nodebug` | javac 16.0.2 | `-g:none` | 4 | 4 | 4 |
| `java16preview` | javac 16.0.2 | `--enable-preview` | 2 | 9 | 9 |
| `java17` | javac 17.0.20.1 | | 24 | 38 | 23 |
| `java17preview` | javac 17.0.20.1 | `--enable-preview` | 35 | 54 | 34 |
| `java21` | javac 21.0.12.1 | | 23 | 74 | 21 |
| `java21preview` | javac 21.0.12.1 | `--enable-preview` | 4 | 4 | 4 |
| `java25` | javac 25.0.5 (Temurin 25.0.5+8) | | 12 | 23 | 11 |
| `jasm` | asmtools-core 7.0.b10-ea `jasm`, on JDK 17 | `-g` | 26 | 26 | 25 |
| `custom` | checked in (`testData/classes/custom`) | | | 20 | 15 |

`java9` holds `module-info.java`, so it is a module compile, and javac 9 rejects that with an empty
`-sourcepath`; Gradle then passes the set root, and so does `compile_sets.sh`. The `kt`, `groovy` and
`scala` sets are not compiled here (13 registered tests: the Kotlin ones are covered by the Kotlin
corpus).

**How the test fixture is reproduced** (from `testFixtures/.../DecompilerTestFixture.java` and
`SingleClassesTestBase.java`):

- **API, not CLI.** A test builds a `ConsoleDecompiler` subclass with `(targetDir, options,
  PrintStreamLogger(System.out), SaveType.LEGACY_CONSOLEDECOMPILER)`, calls `addSource(File)` per class
  file, `decompileContext()`, `TextBuffer.checkLeaks()`. `VfDriver` does the same.
- **Options** are the fixture's base map (`log-level=warn decompile-generics=1 remove-synthetic=1
  remove-bridge=1 keep-literals=1 __unit_test_mode__=1 new-line-separator=1 error-message=""
  include-runtime=""`), then the set's pairs; Fernflower overlays them on its defaults. A set option
  `include-runtime=1|current` becomes the JDK home of the test's language version (only the *Java
  Runtime* set; the manifest records `${java.N.home}`).
- **No plugins.** Gradle runs the tests from class directories, so no plugin is registered;
  `vf-decompile` clears `PluginSources` unless `--plugins`.
- **Inputs**, in `addSource` order: the test class, its `Name$*.class` siblings, then each companion the
  same way, duplicates kept. The fixture lists siblings with `Files.list`, whose order is unspecified;
  the manifest sorts them by name, which is the order upstream's `.dec` files match (only
  `renaming/pkg/TestRenameEntities` depends on it).
- **Output and comparison.** The fixture reads `<class file name>.java` (or `setExpectedFileName`) and
  compares it with `testData/results/<test class>.dec` after `\r\n` -> `\n` and `String.trim()`,
  falling back to `<test class>_1.dec`. `expected/` stores the untrimmed bytes.
- The *Javadoc* sets pass an `IFabricJavadocProvider` object; `vf-decompile --javadoc plain|markdown`
  recreates both.

**Validation** (`scripts/validate.py`, `reports/match-fixture.json`): 598 of 598 outputs pass the
fixture's own comparison with upstream's `.dec`; 597 match the primary file and one,
`default/pkg/TestJava11StringConcat`, matches upstream's `_1.dec`, which records the string-concat code
a different javac 11 build emits (`scripts/mismatch-categories.json`).

**Cross-checks** (`scripts/crosscheck.sh`): a fixture re-run and a run with one visible CPU change
nothing; feeding the class files in reverse order changes only `TestRenameEntities`; `vf-decompile
--save folder --plugins --opt include-runtime=current` equals `java -jar` on all 598. Under CLI
defaults, reversed input and a JDK 21 runtime image change nothing, the bundled plugins change only
`TestIdeaNotNull`, and 75 outputs need the JDK's class library (`include-runtime=current`, the oracle
JVM's runtime image).

**`vf-decompile`**, the general front-end:

```sh
java/bin/vf-decompile [--fixture] [--opt k=v]... [--javadoc plain|markdown] [--plugins] \
    [--save legacy|folder|file|console] [--lib PATH]... [--inputs-from FILE] [--print-options] INPUT... -o OUT
# replay manifest entries by id substring, as regen.sh does
python3 -I java/scripts/run_oracle.py --manifest "$KUNA_JVM_ORACLE/corpus/manifest.json" \
    --classes "$KUNA_JVM_ORACLE/corpus/classes" --corpus "$KUNA_JVM_ORACLE/corpus" --mode fixture \
    --out-root NEW_DIR --work NEW_WORK_DIR --only default/pkg/TestEnhancedForLoops
```

`-o` must be new or empty. Without `--fixture`/`--opt`, Vineflower's built-in defaults apply. For jar
inputs use `--save folder` (what `java -jar` does for a directory destination). Env: `VF_JAVA`
(default JDK 17), `VF_JAR`, `VF_XMX` (1g), `VF_JAVA_OPTS`.

## The Kotlin corpus (`$KUNA_JVM_ORACLE/corpus-kt`)

The same layout, for the Kotlin plugin, in four forms:

| Path | What |
|---|---|
| `classes/kt/`, `classes/ktold/` | `plugins/kotlin/testData/src/{kt,ktold}` compiled (96 and 1 classes) |
| `classes/core-kt/` | root `testData/src/kt` compiled (core's `KOTLIN` tests, 11 classes) |
| `manifest.json` | the 73 tests |
| `expected/<set slug>/<test class>.kt\|.java` | the fixture oracle: `KotlinTests` sets `entire-classpath`, `multifile-facade` (`.kt`); core sets `core-default`, `core-entire-classpath` (no plugins, `.java`) |
| `expected-defaults/`, `expected-defaults-stdlib/` | `java -jar` with no options, and with `-e=kotlin-stdlib-2.0.0.jar` |
| `realworld/<name>/{vf/,vf.log,run.json}` | `java -jar` reference trees for okio 2.6.0, okhttp 4.7.2 and spring-framework's Kotlin classes |

**Compiling.** Upstream builds the test data with the Kotlin Gradle plugin 2.0.0, which runs the
compiler of its own version, so `compile_sets.py` runs kotlinc 2.0.0 on JDK 17 with the arguments KGP
derives for a JVM compilation: `-no-stdlib -no-reflect -Xallow-no-source-files -jvm-target 17
-jdk-home <JDK 17> -module-name <archivesName>_<sourceSet> -classpath <kotlin-stdlib-jdk8, kotlin-stdlib,
kotlin-stdlib-jdk7 2.0.0, annotations 13.0> -d <dir> [free args] <sources, sorted>`; `ktold` adds
`-Xlambdas=class -Xsam-conversions=class -Xstring-concat=inline`. The module names are confirmed by the
`d2` strings of upstream's `.dec` files and `mv = {2, 0, 0}` by every `@Metadata`. The compiler
output is deterministic, and JDK 21 as `-jdk-home` gives the same bytes. The pin matters: compiled
with kotlinc 2.2.20 instead, only 59 of the 73 outputs still match upstream.

**The fixture**, as for Java, with these Kotlin points:

- `KotlinTests` registers *Entire Classpath* (63 tests, no `include-classpath` despite the name) and
  *Multifile Facade* (3); options are the base map plus `bytecode-source-mapping=1
  __dump_original_lines__=1 dump-exception-on-error=0 ignore-invalid-bytecode=1 remove-synthetic=0`,
  plus `verify-anonymous-classes=1` or `kt-decompile-facades=1`.
- `KotlinTests.getClassFile` takes `<name>.class`, else `<name>Kt.class` (4 tests); the fixture then
  reads `<class file name>.kt` and compares it with `results/pkg/<test class>.dec`.
- Only the Kotlin plugin is loaded (Gradle's `ServicePluginSource` over the plugin project);
  `vf-decompile-kt --kotlin-only` loads exactly that plugin from the extracted jar.
- Core's `KOTLIN` tests load no plugin and print Java. Its two *Entire Classpath* tests set
  `include-classpath=1`, which scans the test JVM's class path; the manifest's
  `driver_classpath_extra` (kotlin-stdlib 2.0.0 and its jdk7/jdk8 parts, annotations 24.0.0) is
  appended to the driver JVM's class path (`VF_CP_EXTRA`).

**Validation:** 73 of 73 outputs pass the fixture's comparison with upstream's primary `.dec` (58
byte-identical, 15 differing only by the final newline). **Cross-checks**: re-run, reversed input,
one CPU, a JDK 21 driver, `--plugins` instead of Kotlin-only (changes only the 7 core tests, which
become Kotlin) and driver-equals-CLI all hold; under CLI defaults, `-e=kotlin-stdlib` changes 9
outputs, no JDK library 42, a JDK 21 runtime image none.

**Real-world references** (`scripts/realworld.py`): `java -jar` with default options, `-Xmx4g`, `-e=`
once per library jar, on JDK 17, each followed by a rerun with one visible CPU that must give the same
tree. okio and okhttp take the `tools/realworld` jars; spring-kotlin decompiles the spring-framework
dataset's `classes.jar` with `--only=` its 19 top-level Kotlin classes (exactly the 72 classes that
carry `@kotlin.Metadata`) and all 48 dependency jars. Vineflower 1.12.0 drops the package directories
of an `--only` run and then fails to save the first class, so the script creates them first.
`run.json` records the command and the sha256 of every input.

## Into `tests/jvm` and the engine's tests

| Consumer | Reads |
|---|---|
| `tests/jvm/regen.py` | `corpus/manifest.json`, `classes/`, `expected/`, `reports/match-fixture.json`: vendors the 530 cases that need no JDK library and writes `cases.tsv` and `README.md`; `--skip-vendor --full-manifest FILE` writes all 598 runnable cases for `corpus --manifest FILE` |
| `tests/jvm/plugin_oracle.py` | `java/scripts/compile_sets.sh` and `java/bin/vf-decompile --plugins`, the Vineflower checkout's plugin test data, `corpus/expected-defaults/`: the 6 plugin cases |
| `tests/jvm/kotlin_corpus.py` | `corpus-kt/manifest.json`, `classes/`, `expected/` (checked against `SHA256SUMS`): the 71 vendorable Kotlin cases and `kotlin.json`; `--full-manifest` all 73 cases in all three forms; `--overlays` the plugin cases under option overlays, through `kotlin/bin/vf-decompile-kt` |
| `decompiler/nonnative/kuna-jvm/tests/data/*/gen.sh` | the jar, the JDKs, kotlinc, asmtools, `corpus/classes` and `vf-decompile` |
| `decompiler/nonnative/kuna-jvm/tools/vftrace` | the jar, the checkout, JDK 17/21, `corpus/manifest.json`; it compiles `java/driver/src/corpus/VfDriver.java` into its own build |

Order: `kotlin_corpus.py` and `plugin_oracle.py` write their cases into `tests/jvm`, then `regen.py`
appends them to `cases.tsv`. All three take `--out DIR` to write elsewhere.

## Known gaps

- **Gradle was never run.** The javac and kotlinc argument lists are reconstructed from how Gradle and
  the Kotlin Gradle plugin behave. The evidence that they are right is 598/598 and 73/73 upstream
  matches, including the byte-exact `@Metadata` strings.
- **JDK builds differ from upstream CI** (Temurin from foojay in containers): 17 and 21 here are
  OpenJDK distribution builds. The only visible javac effect is `TestJava11StringConcat` above. The
  oracle JVM matters for the two `include-classpath` tests and for `expected-defaults/` (75 Java and
  42 Kotlin outputs read its runtime image; JDK 21's changes none of them).
- **The release jar is not rebuilt from source**; it is assumed to be `b8273988`, which the upstream
  matches support.
- Only primary outputs are stored; a companion class's file can be regenerated with `vf-decompile`.
- Real-world library versions are a choice (kotlin-stdlib 2.2.20 for okio and okhttp, whose POMs name
  1.3.70/1.3.71); parity only needs both decompilers to get the same `-e` list.
