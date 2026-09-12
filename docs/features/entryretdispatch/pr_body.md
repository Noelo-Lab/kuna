## The problem

The i386 `bm3.exe` entry point is a 105-byte dispatcher made from repeated
`push <continuation>; push <callee>; ret` links. Kuna classified the first RET
as an ordinary function return and emitted an empty body, losing the complete
dispatcher including `LoadLibraryA` and `GetProcAddress`.

An explicit `flow 0x40176f call` already recovered the full body after #504,
which showed that target resolution and continuation walking were present. The
missing step was a conservative way to distinguish this stack-directed call
from a genuine RET without requiring a user assertion.

## The fix

- The shared RET-call-chain recognizer now proves from raw p-code that RETURN's
  destination was loaded from a slot written during the current straight-line
  run and that the adjacent slot holds this RET's exact fall-through address.
- Provenance is deliberately conservative: signed affine constants use their
  operand width; LOAD/STORE facts retain the p-code memory-space ID; overlapping
  stores and register aliases invalidate facts; conditional flow, opaque stack
  writes, unsupported `CALLOTHER`, and unknown indirect behavior decline.
- P2 flow classification seeds derived CALL facts at the shared decompile
  boundary. Explicit flow facts still own their sites, including an explicit
  `flow ... return` veto.
- New option `entryretdispatch` is default on. Turning it off restores the old
  empty-return behavior exactly.

The mechanism and rejected generalizations are recorded in
[`analysis.md`](analysis.md) and [`plan.md`](plan.md).

## Controls and gates

The vendored i386 fixture covers a three-link positive and ordinary RET,
RET-immediate, copied incoming-return-address, computed/constant target,
discarded continuation, unrelated store, signed negative-displacement,
conditional-bypass, partial-SP, cross-memory-space, and unsupported-userop
negatives. The stage and CLI suites exercise both default-on and option-off
behavior.

- `make test`: **675/675 PARITY OK**; `docs/baseline.json` unchanged.
- `make test-stages`: **803/803 PARITY OK**.
- `make test-cli`: **147/147 passed**.
- `cargo test -p kuna-console`: green, including 7 focused chain tests and the
  architecture-generic provenance controls.
- `make check-spec` and strict spec validation: green.
- `kuna catalog --check`: catalog OK; counters have no drift.
- Need acceptance: **1/1 pass**; mergecheck against exact base
  `b612f48b705c2fe7be08c76f4588b9e08af33320`: zero rejects.
- Exact-base unchanged-function control: +0.48%, inside the 5% budget; the
  target changes only in the intended function.

🤖 Generated with [OpenAI Codex](https://openai.com/codex/)
