# Failure fixture for kuna-cli's JVM tests

Classes the JVM engine fails on the way Vineflower 1.12.0 does, for the tests of what the
CLI reports about failures (`kuna-cli/tests/jvm_cli.rs`, `kuna-jvm/tests/kuna_spans.rs`,
`tests/cli/jvm-nested-body-failure.json`). Every run pins `--option jvm_vineflower_compat on`,
so a kuna fix for one of these defects leaves the fixture failing there.

- `src/com/ex/fail/Nested.java` -> `classes/`: a switch expression yielding a `String`
  and a `char` constant fails in Vineflower (`invalid constant type`), here inside a lambda
  (`escape`), two lambdas (`both`), an anonymous class's method (`later`, `Nested$1.run`)
  and a field initializer's lambda (`QUOTE`); `plain` decompiles. javac 21.0.12:
  `javac -g --release 17 -d classes src/com/ex/fail/Nested.java`.
- `kotlin/src/com/ex/ktfail/*.kt` -> `kotlin/classes/`: `Lens` (a default argument computed
  with a lambda: the Kotlin writer gives up on the whole class, `NullPointerException:
  exprents`), `Inl.kt` (`byLength` fails on its SAM object; `sortNames`'s
  `$$inlined$byLength$1` copy is written as a top-level unit of its own) and `Plain`
  (decompiles). kotlinc-jvm 2.0.0 (running on JDK 21.0.12) with `-jvm-target 17`, against
  kotlin-stdlib 2.0.0 and annotations 13.0, exactly as `../kotlin/README.md`.

`regen.sh` rebuilds both (set `KOTLINC`, `JDK17`, `STDLIB`, `ANNOTATIONS` for the Kotlin part).
