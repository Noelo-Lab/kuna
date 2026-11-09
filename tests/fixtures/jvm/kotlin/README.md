# Kotlin fixture for kuna-cli's JVM tests

`src/com/ex/kt/*.kt` compiled to `classes/`, which `kuna-cli/tests/jvm_cli.rs` and the
`tests/cli/jvm-kotlin-*.json` probes read as a directory input. It covers what the CLI does
with Kotlin output: a class with a property whose getter and setter share one declaration
(`Counter.count`), a function with a default argument (`bump`, and its hidden
`bump$default`), a companion object member (`Counter$Companion.of`), top-level functions in
the file class `CountersKt`, and a lambda compiled to `invokedynamic`
(`quadruple$lambda$0`). CI has no Kotlin compiler, so the class files are committed.

Compiled with kotlinc-jvm 2.0.0 (`2.0.0-release-341`, from
`https://github.com/JetBrains/kotlin/releases/download/v2.0.0/kotlin-compiler-2.0.0.zip`)
running on OpenJDK 17.0.20.1, against `kotlin-stdlib-2.0.0.jar` (sha256 `240938c4...0379fcf`)
and `annotations-13.0.jar` (sha256 `ace2a10d...bd111478`) from Maven Central:

```sh
JAVA_HOME="$JDK17" "$KOTLINC" -no-stdlib -no-reflect -jvm-target 17 -jdk-home "$JDK17" \
  -module-name kuna-cli-fixture -classpath "$STDLIB:$ANNOTATIONS" -d classes \
  src/com/ex/kt/Counter.kt src/com/ex/kt/Counters.kt
```

`regen.sh` runs exactly this (set `KOTLINC`, `JDK17`, `STDLIB`, `ANNOTATIONS`). The output
of `kuna decompile-project classes --option jvm_include_runtime off` is byte-identical to
`java -jar vineflower-1.12.0.jar --include-runtime=0 classes OUT` (Kotlin), and with
`--language java` to the same command with `--kt-enable=0` (Java).
