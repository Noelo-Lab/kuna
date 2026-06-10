# ADR 0004: Error model — Result + thiserror, catch frames mirror C++

## Context

The C++ engine throws `LowlevelError` and subclasses (`RecovError`,
`ParseError`, `DecoderError`, SLEIGH's errors) and catches them at a small
number of well-known frames: per-function decompilation (`perform`), console
command dispatch, datatest execution. Recovery behavior — which errors abort a
function vs. abort the process vs. print and continue — is observable in test
output, so the Rust port must reproduce the *placement* of handling, not just
the messages.

## Decision

Fallible engine code returns `Result<T, KunaError>`, with `KunaError` a
`thiserror` enum mirroring the C++ hierarchy: `Lowlevel`, `Recov`, `Parse`,
`Decoder`, `Sleigh` variants carrying the same explain strings the C++
constructors format. The engine propagates with `?` and **catches at the same
frames C++ catches** — per-function `perform`, console command dispatch — with
the same downgrade/continue decisions. `panic!` is reserved exclusively for
arena integrity violations and violated internal invariants (states the C++
code treats as undefined behavior or `assert`s on); panics are never used for
recoverable engine errors and never caught.

## Consequences

- Error text and "which command keeps the session alive" behavior match the
  C++ oracle, so datatests exercising error paths pass unmodified.
- `?`-propagation makes every fallible call explicit; C++ call sites that
  silently assumed "never throws" surface as type errors during the port and
  get a deliberate decision.
- Variant granularity is fixed by the C++ catch sites: where C++ catches
  `LowlevelError` broadly, Rust matches the enum broadly — no finer-grained
  recovery is invented.
- A panic in production is by definition a port bug or memory-model violation,
  which keeps crash triage unambiguous.
