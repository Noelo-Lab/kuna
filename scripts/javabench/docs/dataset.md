# Dataset selection

Query (run 2026-11-09):

    gh api -X GET search/repositories -f q='language:java stars:>20000' -f sort=stars -f order=desc -f per_page=60

Eligibility rule, applied in star order until five projects were kept:

1. **Real software** (library, framework, server, application) -- not a tutorial,
   interview-prep, awesome-list or algorithm collection.
2. **A published build containing JVM class files** (a Maven Central jar or an
   official release archive) -- the benchmark decompiles real `javac` output.
3. **A published source artifact from that same build** (`-sources.jar`, or a
   `-src.zip` shipped in the release) so that every class file has an exact
   source counterpart without rebuilding anything ourselves.

| rank | repo | stars | verdict | reason |
|---:|---|---:|---|---|
| 1 | NationalSecurityAgency/ghidra | 98,836 | **kept** | application; GitHub release `ghidra_12.2_PUBLIC_20261016.zip` ships each module's jar *and* `lib/<Module>-src.zip` |
| 2 | iluwatar/java-design-patterns | 94,855 | dropped | tutorial (design-pattern examples) |
| 3 | macrozheng/mall | 85,020 | dropped | learning/demo e-commerce system; releases are source-only tags, nothing on Maven Central (no published build) |
| 4 | spring-projects/spring-boot | 81,787 | **kept** | framework; Maven Central jar + sources jar |
| 5 | doocs/advanced-java | 79,171 | dropped | interview-prep notes |
| 6 | elastic/elasticsearch | 78,530 | **kept** | server; `org.elasticsearch:elasticsearch` jar + sources jar on Maven Central |
| 7 | MisterBooo/LeetCodeAnimation | 76,715 | dropped | algorithm/LeetCode collection |
| 8 | TheAlgorithms/Java | 66,756 | dropped | algorithm collection |
| 9 | kdn251/interviews | 65,555 | dropped | interview prep |
| 10 | termux/termux-app | 64,437 | dropped | the published build is Android APKs (Dalvik DEX, not JVM class files); no source artifact |
| 11 | spring-projects/spring-framework | 60,307 | **kept** | framework; Maven Central jars + sources jars |
| 12 | dbeaver/dbeaver | 52,363 | dropped | published build (CE installers with OSGi plugin jars) has no source artifact; pairing a git tag with the shipped jars would be an unverified rebuild assumption |
| 13 | google/guava | 51,920 | **kept** | library; Maven Central jar + sources jar |
| -- | skylot/jadx (51,704), jeecgboot/JeecgBoot, ReactiveX/RxJava (48,154), ... | | not reached | |

## Pinned artifacts (SHA-256 of every file in `pins/<project>.json`)

| project | version | scored module(s) | why these modules | class files | javac used for recompilation |
|---|---|---|---|---|---|
| ghidra | 12.2 (release zip, sha256 in `pins/ghidra.json`) | `Ghidra/Framework/SoftwareModeling` | Ghidra's core: program database API, SLEIGH, p-code | 2,245 (major 69 = Java 25) | Temurin 25.0.5 `--release 25`; deps = every other `Ghidra/Framework/*/lib/*.jar` in the zip |
| spring-boot | 4.1.2 | `org.springframework.boot:spring-boot` | the core module (autoconfigure etc. are separate modules) | 841 (major 61) | javac 25 `--release 17` (calibrated, see below) |
| elasticsearch | 9.5.6 | `org.elasticsearch:elasticsearch` (the `server` module) | the server is the product; libs/modules/plugins are satellites | 10,371 (major 65) | javac 21 `--release 21` (manifest: built by Temurin 21.0.7) |
| spring-framework | 7.0.10 | `spring-core`, `spring-beans`, `spring-context` | the IoC core every other module builds on | 2,570 (61; 43 repackaged classes at 52) | javac 25 `--release 17`/`8` per class (manifest Build-Jdk-Spec 25) |
| guava | 33.8.0-jre | `com.google.guava:guava` | single-module library | 1,972 (major 52) | javac 25 `-source 8 -target 8` (guava-parent pom; it uses Java 9+ APIs from Java 8 code) |

Compile-time dependencies: the POM's dependencies plus the module's `optional`
/ `compileOnly` dependencies from its `build.gradle` at the release tag
(Spring), versions from `spring-boot-dependencies:4.1.2` (which pins Framework
7.0.10) except where Framework's own `framework-platform.gradle` pins differ
(mutiny 1.10.0, blockhound 1.0.8, graal-sdk 22.3.1, ...). Resolved with
coursier 2.1.26 into `pins/<project>.json` (the lock the first fetch wrote); refetches are `--intransitive` on
the locked coordinates and verify every SHA-256.

**Toolchain calibration.** The recompiling javac is the one whose recompilation
of the *original sources* reproduces the shipped bytecode best (the `source`
pseudo-decompiler). Spring Boot's manifest says JDK 17, but javac 17 reproduces
only 99.13% of sampled methods byte-exactly (JDK-8272564: javac >= 18 emits
`invokeinterface` for `Object` methods called on interfaces), javac 21 99.97%,
javac 25 100.0% -- so javac 25 is used.

## The first held-out set (`--project heldout1`)

The five projects above are where kuna's fixes were developed, so their score says how the
fixes do where they were tuned. Eleven more projects were chosen by a rule written on
2026-11-26, before any decompiler had run on any candidate, and nothing was dropped, added
or swapped after a decompiler's output on them had been seen. They were measured out of
sample once; fixes were later developed against what they showed, so this set is no longer
held out:

1. A Maven Central artifact with a binary jar and a `-sources.jar`, at the newest stable
   release in `maven-metadata.xml` on that day.
2. Not in-sample: not one of the five projects or their modules, not one of their
   compile-time dependencies or code repackaged inside them, not any other input the fixes
   were triaged on (okio, okhttp, kotlin-stdlib, kotlinx-coroutines, kotlin-reflect, the JDK's
   own modules, Vineflower's test suite), and no other version, fork or module of those.
3. One project per style slot, the first candidate that passes; the lists were fixed with
   the rule:

| slot | style | candidates in order |
|---|---|---|
| 1 | parser-generator runtime | `org.antlr:antlr4-runtime`; `org.parboiled:parboiled-java` |
| 2 | database engine | `com.h2database:h2`; `org.hsqldb:hsqldb` |
| 3 | network stack | `httpcore5` with `httpcore5-h2`; `io.undertow:undertow-core` |
| 4 | JSON library | `com.alibaba.fastjson2:fastjson2`; `com.dslplatform:dsl-json` |
| 5 | cryptography | `com.google.crypto.tink:tink`; `org.jasypt:jasypt` |
| 6 | compiler front end | `org.codehaus.janino:janino` with `commons-compiler`; `org.eclipse.jdt:ecj` |
| 7 | language runtime | `org.mozilla:rhino`; `org.openjdk.nashorn:nashorn-core` |
| 8 | pre-Java-8 code base (class files major 50 or lower) | `joda-time:joda-time`; `org.apache.commons:commons-math3` |
| 9 | numerical code, pre-Java-8 | `org.apache.commons:commons-math3`; `org.ejml:ejml-ddense` |
| 10 | Java 17+ with records, sealed types and pattern switches | `io.jenetics:jenetics`; `io.jenetics:jenetics.ext` |

4. Feasibility, decided without a decompiler: the fetch, the index and the `source`
   pseudo-decompiler run. A candidate fails with fewer than 300 scored methods, or if the
   source oracle recompiles under 90% of its top-level classes once the compile-only
   dependencies its pom declares are added. The recompiling javac is the one whose
   source-oracle run is most byte-exact.
5. Size: 60,000 to 120,000 scored methods in total. The ten winners had 53,880, so the
   second candidate of slot 7 (nashorn) was added, as the rule says.

| project | version | scored methods | class files | javac used for recompilation |
|---|---|---:|---|---|
| antlr4 | `antlr4-runtime` 4.13.2 | 1,447 | 215 (major 52) | 17 `--release` |
| h2 | 2.5.252 | 12,095 | 1,067 (major 55), no local-variable tables | 17 `--release`; its optional deps (Lucene 9.12, servlet APIs, OSGi, JTS, slf4j) |
| httpcore5 | `httpcore5` + `httpcore5-h2` 5.5 | 5,138 | 894 (major 52) | 17 `-source 8 -target 8` (`jdk.net` is hidden by `--release 8`) |
| fastjson2 | 2.0.66 | 6,645 | 773 (major 52) | 17 `-source 8 -target 8` (`sun.misc.Unsafe`) |
| tink | 1.24.0 | 4,384 | 1,431 (major 55) | 21 `--release` |
| janino | `janino` + `commons-compiler` 3.1.12 | 4,095 | 663 (major 51) | 17 `--release` |
| rhino | 1.9.2 | 6,178 | 709 (major 55) | 21 `--release` |
| joda-time | 2.15.1 | 3,910 | 248 (major 49), no local-variable tables | 17 `--release` |
| commons-math3 | 3.6.1 | 8,073 | 1,301 (major 49) | 17 `--release` |
| jenetics | 9.1.0 | 1,915 | 238 (major 69): records, sealed types, type switches | 25 `--release` |
| nashorn | `nashorn-core` 15.7 | 9,309 | 1,019 (major 55; 79 at 49 or 51) | 17 `--release` |

63,189 scored methods in 4,339 top-level classes. The source oracle reaches 98.90% of them
perfect on all metrics and recompiles every class. Every jar and dependency is pinned in
`pins/<project>.json` exactly as for the five.

A held-out set stays held out only until someone fixes what it shows. The result recorded
in `docs/jvm-benchmark.md` says for each number whether it was measured before or after
fixes were developed against these projects.

## The second held-out set (`--project heldout2`)

Fixes were then developed against the first held-out set, so a second set was chosen on
2026-11-27 by a rule written before any decompiler had run on any candidate. It was measured
out of sample once (89.28% of methods perfect against Vineflower's 79.56%); fixes were then
developed against what that run showed, so this set is no longer held out either, and
`docs/jvm-benchmark.md` gives both numbers. A new out-of-sample number needs a third set,
chosen by this rule with this set added to its exclusions.
Nothing was dropped, added or swapped after a decompiler's output on it had been seen. The
rule is the first set's, with the first set added to the exclusions:

1. A Maven Central artifact with a binary jar and a `-sources.jar`, at the newest stable
   release in `maven-metadata.xml` on 2026-11-27 (alpha, beta, RC, CR, milestone, snapshot
   and preview versions skipped). If that release has no `-sources.jar`, the candidate fails;
   there is no fallback to an older release.
2. Not seen before: everything the first rule excluded, the eleven projects of the first set
   with their modules and dependencies (antlr4-runtime, h2, httpcore5, fastjson2, tink, janino,
   commons-compiler, rhino, nashorn, joda-time, commons-math3, jenetics, lucene, conscrypt,
   google-http-client, ant, joda-convert, JTS, OSGi, the servlet APIs), and the four Kotlin
   libraries measured with it. A candidate's own dependencies may be seen code (Guava, ASM,
   Netty); they are only on the classpath.
3. One project per style slot: the first candidate that passes. Only the named artifact is
   scored; its sibling modules are dependencies.

| slot | style | candidates in order |
|---|---|---|
| 1 | parser-generator runtime or parsing library | `org.parboiled:parboiled-java`; `com.github.javaparser:javaparser-core` |
| 2 | database engine | `org.hsqldb:hsqldb`; `org.apache.derby:derby` |
| 3 | network stack | `io.undertow:undertow-core`; `org.asynchttpclient:async-http-client` |
| 4 | JSON or serialization library | `com.dslplatform:dsl-json`; `com.squareup.moshi:moshi` |
| 5 | cryptography | `org.jasypt:jasypt`; `org.bitbucket.b_c:jose4j` |
| 6 | compiler front end | `org.eclipse.jdt:ecj`; `com.google.googlejavaformat:google-java-format` |
| 7 | language runtime | `org.luaj:luaj-jse`; `org.mvel:mvel2` |
| 8 | pre-Java-8 code base (every class file major 50 or lower) | `junit:junit` 4.13.2; `com.jcraft:jsch` |
| 9 | numerical code | `org.ejml:ejml-ddense`; `org.ojalgo:ojalgo` |
| 10 | Java 17+ with records, sealed types or pattern switches (at least 10 class files with a `Record` or `PermittedSubclasses` attribute or a `SwitchBootstraps.typeSwitch` call site) | `dev.langchain4j:langchain4j-core`; `org.junit.jupiter:junit-jupiter-engine`; `io.avaje:avaje-jsonb` |

4. Feasibility, decided without a decompiler: the fetch, the index and the `source`
   pseudo-decompiler run with javac 17, 21 and 25, each with `--release` and with
   `-source`/`-target`. The project uses the most byte-exact combination (ties: more classes
   recompile, then the older javac, then `--release`). A candidate fails with fewer than 300
   scored methods, or if the source oracle recompiles under 90% of its top-level classes once
   the compile-only (`provided` or optional) dependencies its pom declares are added.
5. Size: 40,000 to 120,000 scored methods in total. Over the top, the largest project is
   replaced by its slot's next candidate; under the bottom, the second candidates of slots 2,
   6 and 3 are added in that order.

What happened: Derby 10.17.1.0 has no `-sources.jar` on Maven Central. `ejml-ddense` 0.46.1
failed feasibility: 42 of its 112 classes import `org.jetbrains.annotations`, a Gradle
`compileOnly` dependency its pom does not declare, so its own source recompiles 62.5% of its
classes with any javac. Slot 9 went to ojalgo. The ten winners have 51,458 scored methods,
inside the size target, so the size rule changed nothing. dsl-json and jasypt got their pom's
`provided` dependencies (dsl-json: JSON-B API 1.0, JSON-P API 1.1.2, paranamer 2.8, jsr305
3.0.2; jasypt: icu4j 3.4.4, servlet-api 2.4).

| project | version | scored methods | top-level classes | class files | debug info | javac used for recompilation | source oracle all perfect / classes recompile |
|---|---|---:|---:|---|---|---|---|
| parboiled | `parboiled-java` 1.4.1 | 348 | 33 | 55 (major 55) | lines, no local-variable tables | 17 `--release` | 98.85% / 100% |
| hsqldb | 2.7.4 | 9,959 | 405 | 686 (677 at 55, 9 at 49) | none (no line numbers, no local-variable tables) | 17 `--release` | 98.43% / 99.75% |
| undertow | `undertow-core` 2.4.4.Final | 7,542 | 569 | 1,510 (major 61) | full | 17 `--release` | 99.87% / 100% |
| dsl-json | 2.1.0 | 1,074 | 80 | 254 (major 52) | full | 21 `--release` | 100% / 100% |
| jasypt | 1.9.3 | 673 | 78 | 113 (major 50) | full | 17 `--release` | 98.96% / 100% |
| ecj | 3.46.100 | 10,886 | 512 | 811 (major 61), compiled by ECJ itself | full | 25 `-source 17 -target 17` | 70.19% / 99.80% |
| luaj | `luaj-jse` 3.0.1 | 2,218 | 97 | 350 (336 at 47, 14 at 49) | none | 17 `--release` | 91.30% / 98.97% |
| junit4 | `junit` 4.13.2 | 1,473 | 165 | 350 (major 49) | full | 17 `--release` | 98.64% / 100% |
| ojalgo | 58.0.0 | 13,686 | 629 | 1,715 (major 66) | full | 25 `--release` | 99.68% / 100% |
| langchain4j | `langchain4j-core` 1.24.1 | 3,599 | 384 | 685 (major 61): 8 records, 10 sealed types | full | 25 `--release` | 99.08% / 100% |

51,458 scored methods in 2,952 top-level classes. The source oracle reaches 92.79% of them
perfect on all metrics and recompiles 2,944 of the 2,947 source files it compiles (five ECJ
and undertow files declare two top-level classes each). The three failures are dependencies
no pom declares: hsqldb's `DatabaseManagerSwing` needs hsqldb's
separate transfer tool, ECJ's `JDTCompilerAdapter` needs Ant, and luaj's `JavaBuilder` needs
BCEL. The ceiling is lower than for the other sets for two reasons, and both apply to every
decompiler equally. ECJ is compiled by ECJ, so no javac reproduces its bytecode: about 70% of
its methods can be byte-exact. luaj was compiled for Java 1.3 (class files major 47), and
javac's `--release 8` output differs from it in 9% of its methods. Every jar and dependency is
pinned in `pins/<project>.json`.

## Scored set

Per project, every class-file method with a `Code` attribute is either scored
or excluded with a reason (`index.json.gz`, field `excluded`): synthetic and
bridge methods (lambda bodies, accessors, `$values`), `<clinit>` (merged from
several source constructs), compiler-generated members with no declaration
(default constructors, enum `values`/`valueOf`, implicit record members), and
classes with no Java source (Kotlin files in Spring), and **machine-generated
sources** (`generated-source`: a `$ANTLR` / JavaCC / `DO NOT EDIT` /
`@Generated(` marker in the first 3,000 bytes of the file -- Ghidra's 31
ANTLR-generated SLEIGH lexers/parsers and Guava's generated public-suffix table;
their "structure" is a DFA dump, and Vineflower spends >10 CPU-minutes on a
single 44 KB generated method). Mapped methods with a tree-sitter parse error in
the ORIGINAL source would also be excluded (there are none in these five
projects); `no-compatible-decl` are implicit record canonical constructors and
enum `valueOf(String)` next to a user overload.

| project | scored methods | top-level classes with scored methods | excluded (reason: methods) |
|---|---:|---:|---|
| ghidra | 18,153 | 1,469 | bridge 958, generated-source 1,469, no-compatible-decl 2, no-source-decl 619, static-initializer 500, synthetic 296 |
| spring-boot | 3,967 | 423 | bridge 98, class-not-in-sources 3, no-compatible-decl 2, no-source-decl 469, static-initializer 193, synthetic 460 |
| elasticsearch | 62,077 | 4,426 | bridge 2,847, no-compatible-decl 94, no-source-decl 7,925, static-initializer 3,654, synthetic 8,124 |
| spring-framework | 13,874 | 1,342 | bridge 398, class-not-in-sources 130, no-compatible-decl 4, no-source-decl 1,015, static-initializer 380, synthetic 930 |
| guava | 12,167 | 495 | bridge 1,412, generated-source 1, no-source-decl 810, static-initializer 268, synthetic 899 |

Default scored set: a deterministic stratified sample of 3,000 methods per
project (`index.sample_keys`): bytecode-size strata 1-16 / 17-64 / 65-256 /
257-1024 / >1024 bytes; half the budget split equally across strata (so large
methods are represented), half proportional; within a stratum methods are taken
in `sha256("javabench-v1" + key)` order. The decompiler input is a jar of the
complete top-level classes (with all nested classes) that contain sampled
methods; `--full` uses the whole `classes.jar` and scores every method.
