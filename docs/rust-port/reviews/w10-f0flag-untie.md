# f0-flag-untie — no-op-gated (fix relocated) + queued f0-flag-v2 plan

Wave w2brw9pyg (2026-06-16) declined: the over-tie is NOT in the hypothesized
funcdata_spacebase/database/varmap/funcdata flag-derivation sites. Instrumentation
(step-0, all 4 candidate `set_flags_pub(addrtied|mapped)` sites) proved NONE fire for
`register:0x1000:8` (f0). They fire only for legitimate ties:
`funcdata_spacebase.rs:821/865` → stack `s0x28`; `funcdata.rs:1445` + the GlobalQuery
finalscope fallback → global RAM `r0x424308..r0x424324`; `funcdata.rs:3078/3200` +
`database.rs:2563` → never.

## The real set-site (GOLDEN)

`coreaction_cleanup.rs` **`mark_output_storage_addr_tied`** — the `for vn in targets {
v.mark_mapped_addr_tied() }` tie loop (reached from `ActionMergeRequired::apply`). f0
(register 0x1000, size 8, output_locked=false, written={1,2}, marker_writes=0) falls
through to the **unconditional** tie because the marker-write un-tie gate
(`coreaction_cleanup.rs:356-372`) is `if output_locked && !written.is_empty() { if
!has_marker_write { return; } }` — **nested under `output_locked`**, and the doublemove
protos are output-**UNLOCKED**. C++ leaves f0 implied/un-tied → `return glob1 + glob1;`;
Rust over-ties → `float8 v1; v1 = glob1 + glob1; return v1;` (p-code byte-identical).

## The fix + its one blocker (experimentally validated)

Lifting the un-tie out of the `output_locked` gate (`if !written.is_empty()` instead of
`if output_locked && !written.is_empty()`) reaches **341 passing (+8)**: doublemove #1/#4
flip, plus collateral ifswitch #3/#5/#6, lzcount #1, mixfloatint #2/#6, modulo2 #1. BUT it
**regresses partialmerge :: Partial Merge #3** (a min=0 NEGATIVE assertion) → regressed set
non-empty → HARD GATE violated.

The clean fix needs a **ScopeLocal-ownership predicate (LOSS-206)** that distinguishes a
*transient return register* (un-tie) from a *recovered whole-function local* (keep tied), so
partialmerge #3 is not regressed. Legitimate ties confirmed staying tied: stack `s0x28`,
global RAM `r0x424318/r0x424308` (doublemove #2/#3/#5/#6 stay PASS).

## Queued: f0-flag-v2 (after struct-return-v2 frees coreaction_cleanup.rs)

`coreaction_cleanup.rs` is a **contention hotspot** — three levers want it:
struct-return-v2's `name_local_highs_angr` is_proto_partial hook (RUNNING), this f0-flag
un-tie (`mark_output_storage_addr_tied`), and the held base_explicit PIECE/addrtied arm.
All three are **different functions** → additive. Plan: after struct-return-v2 lands, a
combined coreaction_cleanup wave does (a) the f0 un-tie lifted out of `output_locked`, (b)
the LOSS-206 ScopeLocal-ownership gate so partialmerge #3 holds, and optionally (c) folds in
the held base_explicit branch. Expected **+8** (doublemove #1/#4 + collateral), gate:
partialmerge #3 + the full 333+ passing set monotonic, switch byte-identical, boolless-BE.
