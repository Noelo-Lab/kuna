# kuna-jvm regression corpus

The regression corpus of `kuna-jvm`: 903 cases, each a set of class files, the option pairs the
engine runs them with, and the Java or Kotlin text kuna printed for them when the case was last
reviewed. `make test-jvm` (`kuna test --jvm` from the repository root) runs every case and fails
on any case whose text changed. `cases.tsv` is the one manifest; it is edited by hand and by
`kuna test --jvm --save-digests`.

| Path | What |
|---|---|
| `cases.tsv` | the manifest (format below) |
| `classes/<source set>/` | the class files of the 591 cases from Vineflower's test suite (below), one directory per source set |
| `kuna-fixes/src/`, `kuna-fixes/classes/` | kuna's own cases: each class's source, and the class files compiled from it |
| `kuna-fixes/expected/` | kuna's reviewed text of each kuna case, `<id>.java` or `.kt` |
| `stackguard/` | not cases: the deep-nesting inputs of `kuna-jvm/tests/kuna_stackguard.rs` |

A case's text is pinned one of two ways: by a `kuna` digest in `cases.tsv` (the 591 cases from
Vineflower's suite, whose text is not stored), or by an expected file (the 312 kuna cases, set
*Kuna Fixes*).

## Provenance

- Vineflower 1.12.0 (`b8273988`) `testData`, Apache-2.0 (see `LICENSE`/`NOTICE` at the repo
  root): the sources and registrations of core's `SingleClassesTest` (526 Java cases, sets
  *Default* to *Ternary Constant Simplification*), of its bundled plugins' tests (6 cases, sets
  *With Idea Not Null* to *Plugin Defaults*) and of its Kotlin tests (71 cases, sets
  *Kotlin ...*). Each set's option pairs are the real options the Vineflower test fixture that
  registered it sets; its test-only keys are left out, so every case runs as the product does,
  Kotlin and IDEA not-null passes included (*Kotlin Core Default* writes its Kotlin classes as
  Java with `kt-enable` off).
- Class files compiled per source set the way Vineflower's Gradle build does (`-g`,
  `-source N -target N`, set-specific flags such as `-g:none` and `--enable-preview`):
  `java8`/`java8nodebug` javac 1.8.0_512, `java9` 9.0.4, `java11` 11.0.33,
  `java16`/`java16nodebug`/`java16preview` 16.0.2, `java17`/`java17preview` 17.0.20.1,
  `java21`/`java21preview` 21.0.12.1, `java25` 25.0.5; `jasm` assembled with asmtools-core
  7.0.b10-ea; `custom` and `plugins/` are Vineflower's checked-in binaries and the plugins'
  `testData` compiled with javac 1.8; `kotlin/` is kotlinc 2.0.0, JVM target 17, as the Kotlin
  plugin's own build does.
- The kuna cases: the comment line above each `kuna-fixes/` case in `cases.tsv` names its source
  and the compiler that built its class files.

## What a case is

An id `<set-slug>/<name>`, its set, the ordered option pairs (the set's, overlaid by the case's),
the class files in order (the case's stems, each `STEM.class` then its `STEM$*.class` siblings
in name order, then its `class` records), and the output file it compares. The runner
(`kuna_jvm::kuna_corpus`) hands the pairs to `Fernflower` (which overlays them on the defaults),
adds each class file as a source,
decompiles, takes the last delivered unit whose file name is the compared one, normalizes its
text (`\r\n` -> `\n`, then trims), and compares it with the pinned digest or expected text.

## Format of `cases.tsv`

Line oriented, tab separated; fields escape `\\`, `\t`, `\n`, `\r`; `#` starts a comment line.

| Record | Meaning |
|---|---|
| `kuna-jvm-corpus` `1` | format header (first record) |
| `root` `DIR` | base of relative paths (default: the manifest's directory), for a manifest kept elsewhere |
| `set` `NAME` `SLUG` [`DIR`] | starts a test set; the `opt`/`javadoc` lines after it are the set's; its cases' expected texts, if any, are `DIR/<ID without SLUG/>.java` (or `.kt` when only that exists; default `DIR`: `expected/SLUG`) |
| `case` `ID` `SET` [`STEM`...] | starts a case of set `SET`; following `opt`/`javadoc` lines overlay the set's; each `STEM` (a path without `.class`) adds `STEM.class`, then its `STEM$*.class` siblings in name order |
| `kuna` `HEX` | the case's text is pinned by this digest instead of an expected file: the first 16 hex digits of the SHA-256 of the normalized text |
| `opt` `KEY` `VALUE` | an option pair (a repeated key replaces the earlier value in place) |
| `javadoc` `plain\|markdown` | the set passes the test fixture's Javadoc provider as `fabric:javadoc` |
| `class` `PATH` | a class file, after the stems' |
| `compare` `NAME`... | the output file compared; later names are fallbacks (default: the first class file's name with the expected text's extension, `.java` for a case with `kuna`, then `.java`) |

## Running a case and investigating a red one

```sh
decompiler/target/release/kuna test --jvm                      # the gate (make test-jvm)
decompiler/target/release/kuna test --jvm --name TestEnhancedForLoops --out NEW
```

`--name` takes an id or a substring of one; `--out NEW` writes the text each selected case printed
as `NEW/<id>.java` (or `.kt`). The `FAILURES` list names each red case (a digest case with
both digests). An expected-text case is read against its file:

```sh
diff -u decompiler/managed/kuna-jvm/corpus/kuna-fixes/expected/pkg/TestFoo.java NEW/kuna-fixes/pkg/TestFoo.java
```

A digest case's previous text is not stored: write it with a build of the base commit and diff
the two trees (`BASE_KUNA test --jvm --name ID --out OLD`, then `diff -ru OLD NEW`). Once the
change of text is reviewed and intended, re-record it: `kuna test --jvm --name ID --save-digests`
rewrites the `kuna` record of each selected case from the text it printed; for an expected-text
case, copy the new text over its expected file.

## Adding a case

A new case is a kuna case: a small source, its class files and kuna's reviewed text.

```sh
javac -g --release 17 -d decompiler/managed/kuna-jvm/corpus/kuna-fixes/classes decompiler/managed/kuna-jvm/corpus/kuna-fixes/src/pkg/TestFoo.java
```

Append its records to `cases.tsv` after the last `kuna-fixes/` case, with the comment that
records its source and compiler (a Kotlin class decompiled as Java gets `opt	kt-enable	0`; a case that needs
other classes lists their stems after its own):

```
# kuna-fixes/src/pkg/TestFoo.java, javac 21.0.13.1 (-g --release 17)
case	kuna-fixes/pkg/TestFoo	Kuna Fixes	kuna-fixes/classes/pkg/TestFoo
```

Then create its expected file empty, write kuna's text, review it, and pin it:

```sh
touch decompiler/managed/kuna-jvm/corpus/kuna-fixes/expected/pkg/TestFoo.java
decompiler/target/release/kuna test --jvm --name kuna-fixes/pkg/TestFoo --out NEW
cp NEW/kuna-fixes/pkg/TestFoo.java decompiler/managed/kuna-jvm/corpus/kuna-fixes/expected/pkg/TestFoo.java
```

A case pinned by digest instead gets the record `kuna	-` and `--save-digests`. The counts
`kuna-jvm/tests/kuna_corpus.rs` (`vendored_corpus_is_complete`) checks go up with each case.
