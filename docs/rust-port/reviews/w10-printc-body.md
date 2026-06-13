# w10-printc-body
verdict: ACCEPT
verifier: claude-opus-4-8 (independent verifier)
date: 2026-06-13
branch: worktree-wf_fa71d46d-9cf-1 (281c3d8, + verifier test commit f07f495)

## Scope

C++ in scope: `printlanguage.cc` (the RPN driver), `printc.cc` (the op-emitter
data + token table), `prettyprint.cc` (the `Emit` back-end).  This item is a
W10 parity-grind refinement against the already-`verified` `w8-s9-printc`: it
realizes the `PrintLanguage` RPN driver (`pushOp`/`pushAtom`/`pushVn`/`recurse`/
`emitOp`/`emitAtom`/`opBinary`/`opUnary`/`parentheses`, printlanguage.cc:129-580)
as an `impl PrintC` block driving the real `EmitNoMarkup`, and adds a parity
gate that honestly measures the byte-match count against the C++ oracle.

gate: `cd rust && cargo test -p kuna-decomp` -> PASS (90 suites `ok`, 0 failed;
lib unit tests 1594 incl. the 9 `rpn_*` + my 3 `w10_*`; `tests/printc_parity.rs`
green).

## Parity-claim audit (the central question for a W10 grind item)

The commit makes a deliberately *non-inflated* claim: it does NOT assert a
datatest pass-count increase. I confirmed this directly.

- Ran `tests/printc_parity.rs` with the C++ oracle genuinely available
  (`KUNA_DECOMP_TEST`/`KUNA_SPECS`/`KUNA_DATATESTS` pointed at the main
  checkout's `decomp_test_dbg` + built `.sla`). Measured report:
  `Rust decompiled+printed: 8/9`, `byte-matched vs C++ oracle: 0/8`,
  `oracle available: true`. The one non-running fixture (`gp:printf`) errors on
  an unmapped-bytes loader seam, not the printer.
- Spot-checked the per-fixture diff: the Rust side emits the `void NAME(void)`
  signature shell + `/* WARNING: body emission blocked ... */`; the C++ oracle
  emits the real decompiled body (e.g. `boolless` -> `uint1` return, `v1`,
  `if (dat_52 <= 10)`, `return v1`). The divergence is exactly the upstream
  decompilation-pass seam (heritage/simplify/merge/type+proto recovery/block
  structuring are stubs, so the IR reaching the printer is raw lifted p-code),
  NOT a printer defect. The gate's only hard assertion is `ran >= 8` (decompile
  + print, brace-matched, real signature); the byte-match count is
  measured-and-reported, never asserted at a level it does not reach.
- Conclusion: the parity claim is HONEST. No inflation. datatests remain at the
  pre-existing 3/675 (no increase claimed, none required of this item — its
  gate is `cargo test -p kuna-decomp`).

## Re-derivation against the C++ (token/store-write order)

The ported RPN engine was re-derived line-by-line against printlanguage.cc:
- `push_op` (129), `push_atom` (162), `push_vn` (197), `recurse` (521),
  `op_binary` (553), `op_unary` (573), `emit_op` (332), `emit_atom` (379),
  `parentheses` (270) — all transcribed faithfully.
- The `mutable id2` semantics (C++ `emitOp(revpol.back())` mutates the live
  stack top's `id2`, read back by the second `emitOp` on the same entry during
  the `pushAtom` close) are reproduced in Rust by cloning the top, letting
  `emit_op` set `id2` on the clone, and mirroring `id2` back onto the live top
  guarded by `ptr::eq(tok) && id==id` (ids are unique per openGroup/openParen,
  so the guard cannot mis-target). For the wired `EmitNoMarkup` path this id2
  plumbing is inert anyway (its `open_paren` returns the passed id and matches
  C++ prettyprint.hh:601-604 byte-for-byte; `open_group` returns 0).
- Token table values (precedence/spacing/associativity/type) and the negate
  cross-links (`token_negate`: less_than<->greater_equal etc., printc.cc:130-135)
  match C++ exactly.
- `recurse` termination: re-derived for the only reachable (no-growth) case —
  it drains `nodepend` down to `last_pending` exactly as the C++ `while
  (lastPending < pending)` loop, restoring `mods`. On the tested path
  `op_binary`/`op_unary` push resolved leaf atoms (never `push_vn`), so
  `nodepend` stays empty and `recurse` is never triggered — consistent with the
  `// SEAM(decompile-passes)` note.

## The mandatory hunt list

- Signedness: clean. The driver compares `visited` (int4) against `tok.stage`
  (int4) and container lengths via `pending: usize` vs `nodepend.len()` — no
  signed/unsigned mix; `parentheses` precedence compares are int4 vs int4.
- Integer widths: clean. `uint4` modifiers, `int4` visited/stage/id, `usize`
  pending — faithful. No promotion-sensitive arithmetic in the driver.
- Wrapping: clean — no arithmetic on uintb-lineage values in the ported block
  (the only `+= 1` is `visited`, bounded by `stage` <= 2).
- Comparator totality: N/A — no new comparator; `parentheses` is a pure
  match-on-enum function (ported earlier, re-verified faithful).
- Iteration-order provenance: `revpol`/`nodepend` are `Vec` used as LIFO stacks
  (C++ `vector` + `emplace_back`/`pop_back`/`back`) — order preserved. No
  HashMap/HashSet anywhere in the added code.
- Off-by-one / do-while: the C++ `pushAtom` `do {} while(!revpol.empty())` is
  ported as `loop { ...; if revpol.is_empty() break }` with the increment-then-
  test inside — preserves at-least-once + the post-pop empty test. Re-derived
  by hand; my `w10_deep_nonassoc` test exercises multi-pop drain.
- Erase-while-iterating: `recurse`'s `nodepend.pop()` mirrors C++ `pop_back`;
  drain order verified.
- Exception->Result partial-state: ONE divergence found (F1 below).

## Mechanical pass

- grep of the added code: no `todo!`/`unimplemented!`/`HashMap`/`HashSet`/
  `sort_unstable`; no bare `as` casts in the RPN block; the two `unwrap_or(0)`
  are in the parity test's XML-attr parsing (test scaffolding, not ported
  logic).
- clippy: `cargo clippy -p kuna-decomp` (lib) CLEAN. The workspace
  `--tests -D warnings` error is in `heritage.rs:1602` test code
  (`!i.is_heritaged() || true` tautology) — a file UNTOUCHED by this item and
  pre-existing on the `rust-port` base; not attributable to w10-printc-body.

## findings

  - F1 (low, latent): `op_binary` negate-flip on a token with no complement
    diverges from C++. C++ `opBinary` throws
    `LowlevelError("Could not find fliptoken")` when `tok->negate == 0` under
    `negatetoken`; the Rust port uses `token_negate(tok).unwrap_or(tok)`, which
    silently keeps the original (un-negated) token and clears the modifier.
    Where C++ would abort the function (partial RPN state -> restart/recover
    machinery), Rust emits the non-negated operator. NON-BLOCKING: `op_binary`
    is not yet IR-driven (it is only reachable from unit tests with controlled
    tokens that always have complements), so the path is unreachable in
    production today. Pinned by my `w10_negate_flip_no_complement_*` test so a
    future restoration to the C++ throw is a visible change. Should be flagged
    to the eventual body-driver item.
       cpp: decompiler/cpp/printlanguage.cc:556-561
       rust: rust/crates/kuna-decomp/src/printc.rs:1366-1372

## adversarial tests (landed on branch, commit f07f495)

  - `printc::tests::w10_postsurround_call_id2_roundtrips` — `foo(a + b)`;
    exercises the postsurround `id2` set-at-visited==1 / read-at-visited==2
    mirror-back (the most fragile transcription point).
  - `printc::tests::w10_deep_nonassoc_right_nesting_parenthesizes_each_level` —
    `a - (b - (c - d))` and the equal-precedence non-associative `(a - b) - c`
    (caught and corrected my own initial wrong expectation against
    printlanguage.cc:278-287 -> the port is faithful).
  - `printc::tests::w10_negate_flip_no_complement_keeps_token_does_not_panic` —
    pins the F1 divergence + the modifier-clear ordering.
  All three pass; full suite stays green (1594 lib tests).

## losses

No new loss. The deferred IR-leaf edges (`recurse` implied-op dispatch,
`pushVnExplicit` symbol/constant resolution, `emitBlock*`, real proto signature)
are covered by the existing LOSS-130 (the W10 grind target list, which names
"PrintC body emitter (THE keystone)" explicitly). The branch correctly added no
losses.md entry. (Ledger note, pre-existing, not this branch: the main-tree
losses.md has two `## LOSS-130` headings — a duplicate id; max LOSS id is 132.)

## verdict rationale

The RPN engine is a faithful transcription of printlanguage.cc:129-580, the
token table / negate links match printc.cc, and the `EmitNoMarkup` back-end
matches prettyprint.hh. The parity claim is honest and independently confirmed
(8/9 run, 0/8 byte-match, oracle available) — no inflation, no claimed
output-vs-oracle match that does not actually match. The single divergence (F1)
is latent/unreachable and pinned by a test. ACCEPT.
