# Intentional default-behavior divergences from upstream

Option-gated sub-stage exposures (the GH-558 pattern) keep default output
byte-identical to upstream and are NOT listed here — they live in `UPSTREAM.md`
*Divergence* (file-level) and `tests/stages/` (behavior). This file records the cases
where kuna **intentionally changes default behavior**: unambiguous bugs where keeping
the upstream default would mean shipping wrong output. Each entry records the flip,
the justification, and the measured effect on the upstream test suite (the
gh558-experiment protocol: run the 204+675 upstream assertions, list every change).

---

## DIV-1: GH-2786 — nested sign tokens merge into the `--`/`++` operator

- **Flip**: `PrintLanguage::parentheses` (printlanguage.cc, unary_prefix case) now
  parenthesizes when two identical `-` (or `+`) unary-prefix tokens would abut.
  Upstream printed `-(-x)` as `--x`.
- **Justification**: `--x` re-parses as pre-decrement — a semantically different,
  assignable expression. Emitting source that means something else is an unambiguous
  bug, not a representation policy; there is no legitimate "other choice" to expose
  as a sub-stage. (Float double-negation is deliberately not folded by analysis for
  NaN/sign correctness — maintainer-confirmed — so the renderer must handle it.)
- **Changed upstream assertions**: **0 of 675** (and 204/204 unit tests unchanged) —
  `docs/baseline.json` still passes as PARITY OK without regeneration; no upstream
  datatest exercises adjacent sign tokens.
- **Verification**: `tests/stages/gh2786-unaryspacing.xml` asserts the fixed form
  `-(-in_f0)` is emitted and the mis-parsing `--in_f0` token is gone (MIPS:BE:32
  bytes from the issue attachment).
- **Date**: 2026-06-06.
