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

---

## DIV-2: eight stage-model sub-stage fixes become the default

- **Flip** (user decision 2026-06-07: "all of these on by default if they are not
  overly destructive"): `compareform` → **original**, `arraynotation` → **on**,
  `thumbfuncptr` → **on**, `inferfuncentry` → **on**, `booleanmask` → **on**,
  `addcarrychain` → **on**, `ovlesssimplify` → **on**, `memsetrecover` → **on**
  (with a new minimum-fill guard: ≥2 COPYs and ≥16 bytes, so a lone string NUL
  terminator is never claimed as a memset — found via the Stack string #6 ablation).
  Every option remains settable; `option <name> off` (or `compareform canonical`)
  restores the upstream behavior per option.
- **Deliberately NOT flipped** (destructive, stay opt-in):
  - `returnpair single` — ablation showed 3/675 upstream tests legitimately need the
    multi-register join (real wide returns would be truncated);
  - `v850indirectbranch` — the jmp-shape predicate (CALLIND of a named register)
    matches every register-indirect call on non-V850 arches; the global flip diverges
    into unbounded jump-table sub-queries. Correct only as a per-program assertion.
- **Justification**: these eight all resolve open upstream issues (GH-558, 8471,
  6930, 1282, 7190, 8913, 9230, 1537) where the kuna output is strictly more faithful
  to source or strictly more readable, and the ablation matrix shows their corpus
  effect is text-only (compareform 12, arraynotation 10) or nil (the other six, after
  the memset guard).
- **Changed upstream assertions: 22 of 675** (204/204 unit tests unchanged), exactly
  the two presentation sets, no interaction effects:
  - `compareform original` (12): Bitfields #2/#7/#11, Boolean thru Less-than #1,
    Ccmp test #1, Else-if #14, For-loop with skip #2, If/Switch #2, Inline target #4,
    No for-loop global call #1, Promotion on compare #1, Signed byte #4
  - `arraynotation on` (10): Heap string #3/#4/#7, Pointer Compare #1,
    Pointer to array #1/#3, Relative pointers #6, Stack string #1/#11/#12
  The 22 datatest regexes were updated in place to assert the new rendering
  (old forms remain in git history); `docs/baseline.json` regenerated.
- **Stage-testcase inversion**: the nine option-gated `tests/stages/gh*` testcases
  for flipped options now assert fix-under-default and old-behavior-under-
  `option … off`, so both directions of each decision stay pinned. Two recorded
  default-interactions: GH-8471's symbolic pointer renders `&fn[1]` (arraynotation)
  and GH-7190's opt-out garbage shows `0 <=` (compareform).
- **Date**: 2026-06-07.
