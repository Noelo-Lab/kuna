# [PROPOSAL] Re-roll inlined libc idioms (`cmpsb`/`df` loop → `strcmp(v19,"-")`)

Part of the **kuna → IDA Pro parity** program. Gap #5. **Large (multi-week) — needs human
go/no-go.**

## The problem

In `fmt/main`, gcc `-O2` inlines the `strcmp(file, "-")` used to detect the `-` (stdin)
argument as a 2-byte `repe cmpsb`. kuna emits the raw lowered loop; IDA re-rolls it:

```c
// kuna
v21 = 0;                       // df (direction flag)
...
v11 = 2;
v14 = file; v16 = (uint1 *)0x6f52;    // 0x6f52 = "-"
do {
  if (v11 == 0) break;
  v11 = v11 + -1;
  v15 = &v16[(uint8)v21 * -2 + 1];    // df-driven pointer step
  v13 = &v14[(uint8)v21 * -2 + 1];
  v19 = *v14 < *v16;
  v20 = *v14 == *v16;
  v14 = v13; v16 = v15;
} while (v20);
if ((!v19 && !v20) != v19) { ... }    // the mangled comparison result
// IDA Pro
if ( !strcmp(v19, "-") ) { ... }
```

kuna surfaces the whole `cmpsb` machinery: the `df` direction flag (`v21`), the count-down
(`v11`), the `df`-scaled pointer increments (`* -2 + 1`), and the flag-arithmetic comparison
result (`(!v19 && !v20) != v19`). This is correct but deeply unreadable, and the `df`/flag
plumbing leaks into many string-heavy functions.

## The IDA / Ghidra reference

IDA's decompiler has a library of **inlined-libc-idiom recognizers** (its microcode pattern
matchers / "idioms") that re-roll recognized `rep movs`/`rep stos`/`rep cmps`/inlined
`strcmp`/`memcpy`/`memset`/`strlen` sequences back into the library call. Ghidra recognizes some
via its `rep`-prefix + string-op modeling and structuring; coverage varies.

## Why this is large (needs go/no-go)

1. **A new recognition pass, not a tweak.** Detecting the lowered idiom requires matching a
   multi-op, loop-shaped pattern (the `cmpsb`/`movs` body + the `df`-scaled stride + the count),
   proving it computes `strcmp`/`memcpy` semantics, and rewriting it to a synthesized call —
   an S6/S7-class transform with its own pattern library. This is architecturally comparable to
   the SAILR structuring ports, not a single-op rule.
2. **`df` modeling.** kuna currently exposes the direction flag as a live variable (`v21`).
   Correct re-rolling needs to prove `df == 0` (the standard forward case) and fold the
   `* -2 + 1` stride to `+1`, then eliminate `df` — a prerequisite sub-analysis.
3. **Correctness surface.** Each recognized idiom must be provably equivalent (count, direction,
   NUL/length semantics) or it silently changes behavior. A conservative matcher covers the
   common gcc/clang shapes; a general one is open-ended.
4. **Scope of payoff vs cost.** High readability win on string-heavy code, but the matcher set is
   large (strcmp/strncmp/strlen/memcmp/memcpy/memset/strcpy, each with size/direction variants)
   and compiler-version-sensitive.

## Proposed staging (for an approved worker)

- **Phase 0 (prerequisite):** `df`-forward proof + stride folding (`(uint8)df * -2 + 1` → `1`
  when `df==0` is provable), eliminating the direction-flag variable. Independently valuable
  (removes `v21`-style noise) and smaller.
- **Phase 1:** the single highest-value recognizer — inlined `strcmp`/`memcmp` (the `rep cmpsb`
  shape above) → synthesized `strcmp`/`memcmp` call, behind strict pattern guards.
- **Phase 2+:** `memcpy`/`memset`/`strlen`/`strcpy` recognizers, one per PR.

## Default policy

Given the correctness surface, this is the one area where an `--option` ablation gate is
recommended during bring-up (per-recognizer), even under the program's default "no flag" policy,
until each recognizer is corpus-proven — then flip default-on.

## Speed / risk

- **Speed:** a pattern-scan over loop bodies; bounded, but adds an analysis pass. Measure per
  phase.
- **Risk:** HIGH (correctness). Strict provable-equivalence guards; conservative bail-out on any
  unmatched variant (fall through to today's raw loop).

## Testing

Per recognizer: a compiled fixture (`gcc -O2` inlined `strcmp`/`memcpy`) asserting the re-rolled
call, plus a near-miss fixture that must NOT be re-rolled. Full corpus gates.

## Effort

**Large (multi-week), multi-PR.** Recommend approving **Phase 0** (`df` folding) first as a
standalone readability win, then go/no-go on Phase 1.
