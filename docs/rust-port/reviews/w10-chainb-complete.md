# Chain B COMPLETE — query_local_properties OR wired (w10-chainb-gap1) — ACCEPT, +11

Wave (2026-06-18) integrated at rust-port (merge of `rport/w10-chainb-gap1` @ `ef903b7`, carries the
Gap-2 substrate `4084bf0` + the b120faf plumbing). Verdict: **ACCEPT**. Resolves the Chain B /
LOSS-156 stack-symbol struct-typing root after ~8 substrate waves across the session.

## The full chain (b120faf → 604408a → 04cd2a2 → 4084bf0 → ef903b7)

The `query_local_properties` OR at `heritage.rs:1381` is now ON — mapped stack ranges return
`mapped|addrtied` from the local stack scope, so stack symbols get their struct types. Wiring it gave
+7/-4 for many waves; the two gaps are now both closed:
- **Gap-2 (closed earlier):** `Scope::addMap` (database.rs:1875) wrongly marked every mapped stack
  local `persist` (it tested `globalscope`, but a `ScopeLocal`'s private root IS the stack scope) →
  `guardReturns` built dead `&struct.field[idx]` spills. Fixed by gating on `is_global()`
  (database.cc:1141).
- **Gap-1 (this wave):** `Merge::mergeIndirect` wrongly merged the store value into the addrforced
  `local_array[10]` because `test_untied_call_intersection` found an empty affecting-op set. Two ports:
  - **W6 store-guard discovery** (`heritage.rs`): `discover_indexed_stack_pointers` (heritage.cc:987),
    `generate_store_guard`/`generate_load_guard` (910/927), `protect_free_stores` (945),
    `reprocess_free_stores` (1112) + `op_mark/clear_spacebase_ptr` (funcdata_op.rs) +
    `get_store_guards` (funcdata.rs). Live StackNode iterators replaced by descend-list snapshots.
  - **W7 StackAffectingOps::populate** (`funcdata_merge.rs`, merge.cc:63): CALL-arm + store-guard-STORE
    arm + `affectsTest` closure, via new `cover.rs::PcodeOpSet` mutators + rep-Address threading
    through `Cover::intersect_op_set`/`affects_test` + `merge.rs:919`.

## Effect

537 → **549** (+11 over base, exceeds the predicted +7). Gained: Local cross #1, No-for-loop alias
#3, Partial splitting #15-19, Wayoff array #1, Store cross #3/#4/#5 (varcross now 9/10).

## Gate (re-run at integration onto 538)

- `cargo test --workspace --no-fail-fast`: 0 failures (the big heritage +415 merge).
- datatests: `[675, 549]`; passing-set diff vs base 538: **regressed-set EMPTY** (gained exactly 11).
- **Mixed float/int held 9/9** — the known hazard (a prior StackAffectingOps attempt regressed
  mixfloatint) did NOT recur; the populate is faithful, not over-broad. No teardown SIGSEGV (correct
  store-cross merge leaves no dead addrforce COPY).
- switch cluster 9/16/3 held; for-loop cluster held; oracle **PARITY OK**; C++ byte-untouched.

## Residual

`analyzeNewLoadGuards`/`establishRange` ValueSet-based guard-range refinement remains a documented
seam — the whole-space first-pass guard is faithful + load-bearing for the merge cross-call test.
The broader stack-struct-typing render (Bitfields/Stack string typing) needs further per-family work,
but the foundational stack-symbol typing is now ON.

## No special-casing

Faithful W6/W7 transcriptions + the addMap is_global fix; no name/address/value hardcoding.
