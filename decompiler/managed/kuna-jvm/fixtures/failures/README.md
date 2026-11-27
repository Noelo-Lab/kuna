# Failure fixture for the JVM failure-reporting tests

Classes the JVM engine fails on by construction, for the tests of what is reported about a
failure: `kuna-cli/tests/jvm_cli.rs`, `kuna-jvm/tests/kuna_spans.rs` and
`tests/cli/jvm-nested-body-failure.json`.

`regen.sh` compiles `src/com/ex/fail/*.java` (javac 21.0.13.1, `-g --release 17`) into
`classes/`, then retags the constant of every `@Mark('\u7a7a')` (a `CONSTANT_Integer`
`0x7a7a`) as a `CONSTANT_Float` and overwrites every `sipush 0x7b7b` with `pop; nop; nop`.
Writing such an annotation throws `ClassCastException`, and processing such a body pops an
empty stack, so:

- `Nested.escape` and `Nested.both` write one and two lambdas whose anonymous class carries the
  broken annotation: the lambdas fail, the methods are written around them.
- `Nested.later` returns an anonymous class whose `run` carries it: the method fails.
- The field initializer `Nested.QUOTE` is such a lambda: a failure outside any method.
  `Nested.plain` decompiles.
- `Whole` has an annotated field: the class fails as a whole. `Mark` decompiles.
- `Broken.lambda` holds a lambda, and `Broken.anonymous` an anonymous class, whose body pops an
  empty stack: the bodies fail while they are processed, the methods are written around them. The
  field initializer `Broken.HEADER` is a lambda whose anonymous class's `run` carries the broken
  annotation on its header: a failure outside any method. `Broken.plain` decompiles.
