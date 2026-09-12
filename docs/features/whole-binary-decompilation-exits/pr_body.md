## The problem

The need records one major-severity instance where whole-binary decompilation
returned success even though every selected function failed: the command exited
`0`, its top-level error was `null`, and its only function had no code body.
That makes an unusable aggregate look successful to scripts even though the
per-function record correctly preserves the underlying error.

The original EndOfWorld observation remains important historical evidence, but
DIV-151 `decodehalt` now gives that packed function a truncation body. The durable
witness is therefore the vendored `fauxware`: selecting `main` with a fatal
five-instruction ceiling deterministically produces one selected function, zero
bodies, and one error record.

## The fix

- Add one shared `BatchOutcome` classifier for whole-binary results.
- Let `decompile-all` finish its complete text or JSON output, then return `1`
  with a run-level diagnostic when a non-empty selection produced no bodies.
- Give ordinary and streaming project exports the same aggregate verdict only
  after their `.c`, `.h`, `.asm`, README, and streaming index artifacts are
  complete. A completed streamed all-failed export removes `.streaming` before
  returning `1`.
- Keep partial success successful: one body is enough for a mixed batch to exit
  `0`, while every failed function remains represented by its own error record.
- Keep an empty narrowed selection distinct from a non-empty all-failed run.

This is a driver-tier contract correction, recorded as DIV-167, and adds no
runtime option.

## The acceptance probe

Acceptance `a-ca0acbabc161` now passes 1/1 against the durable `fauxware main`
witness. It requires exit `1`, a top-level aggregate error, the stderr diagnostic,
and preservation of the per-function flow-ceiling error. The focused Cargo tests
also pin the opposite control requested by triage: `main,__libc_csu_fini` contains
one body and one failure, so `decompile-all` and both project modes remain exit
`0`.

## Gates

- `make test`: 675/675 PARITY OK; baseline unchanged.
- `make test-stages`: 797/797 PARITY OK.
- `make test-cli`: 145/145 PASS.
- Focused release `decompile_all_cli` and `decompile_project_cli` aggregate-exit
  tests: PASS.
- Acceptance `a-ca0acbabc161`: 1/1 PASS.
- Strict spec, catalog, counter, and merge guards: clean; DIV-167 is unique.

🤖 Generated with [OpenAI Codex](https://openai.com/codex/).
