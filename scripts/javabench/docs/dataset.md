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
