# RuleNotDistribute / De Morgan (w10-demorgan-compare) — ACCEPT, +2

Wave (2026-06-17) integrated at rust-port `2d99361` (merge of `rport/w10-demorgan-compare-wt`
@ `7877604`, base 0684760). Verdict: **ACCEPT** (porter self-gated; re-gated at integration).

## What landed

`RuleNotDistribute::apply_op` (`ruleaction_1.rs:1917`) was a stub — it matched the
`BOOL_NEGATE(BOOL_AND|BOOL_OR)` structure but returned 0 with construction deferred at
`SEAM(W3)`. Ported the faithful C++ body (`ruleaction.cc:1148-1184`): build two new
`BOOL_NEGATE` ops via `new_unique_out`, retarget the original op to the flipped
`BOOL_OR`/`BOOL_AND`, distributing `!(V && W) => !V || !W`. Downstream `RuleBoolNegate` then
collapses `!(a!=10)`/`!(b!=0x14)` into `a==10`/`b==0x14`, so
`BOOL_NEGATE(a != 10 && b != 0x14)` renders as `(a == 10 || b == 0x14)` — the oracle form.

## Effect

408 → 410. Gained: `Compare INT_OR #1`, `Signed byte #2`. Only `ruleaction_1.rs` touched
(rule body); `RuleNotDistribute` was already registered in the correct universalAction
position, so registration is untouched.

## Gate (re-run at integration)

- `cargo test --workspace` (debug): **3669 passed, 0 failed** — the fence catcher, includes
  the `universalaction_listing` B0 byte-equal test.
- datatests: `[675, 410]`; full per-assertion passing-set diff vs base `0684760`:
  **regressed-set EMPTY** (gained exactly the 2 above).
- 6 switch datatests: switchind 16/16, switchmulti 8/9, all others unchanged — no regression.
- `boolless` sentinels (`Boolean thru Less-than #1/#2`): pass before and after.
- oracle `run_tests --baseline docs/baseline.json`: **PARITY OK**.
- C++ tree: byte-untouched.

## No special-casing

Diff is a single general rule body driven by the `BOOL_NEGATE(BOOL_AND|BOOL_OR)` predicate;
no name/address/value/type hardcoding.
