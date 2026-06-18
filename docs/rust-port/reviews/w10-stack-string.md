# Stack string StringSequence COPY driver (w10-stack-string) — ACCEPT, +15

Wave (2026-06-18) integrated at rust-port (merge of `rport/w10-stack-string` @ `a38fec6`, base
`36e01a6`/549). Verdict: **ACCEPT**. Measured +12 on its own 549 base; landed **+15** at integration
onto 573 — the keystone (`query_local_properties` OR in `set_varnode_properties`) also unblocked
Bitfields #14/15/16 as a bonus.

## The port

- **`constseq.rs` — `StringSequence` / `RuleStringCopy` full body.** `build` (ctor +
  collectCopyOps/checkInterference/formByteArray), `collect_copy_ops` (address-only
  `beginLoc(addr)..endLoc(addr)` scan via `VarnodeBank::iter_loc_addr_range`),
  `construct_typed_pointer` (spacebase PTRSUB/PTRADD + `getTypePointerStripArray`),
  `build_string_copy`, `remove_forward`/`remove_copy_ops` (COPY teardown + INDIRECT/CONCAT cascade),
  `transform`; `RuleStringCopy::apply_op` wired to the full body (constseq.rs:1585+).
- **`varmap.rs`** — `ScopeLocal::query_container` + `StringContainerEntry` snapshot (additive).
- **`funcdata_varnode.rs`** — `Funcdata::construct_spacebase_input` / `construct_const_spacebase`
  (additive).
- **`funcdata.rs:1510` (keystone)** — `set_varnode_properties` now OR's `query_local_properties` into
  the global query. `RuleSplitCopy` splits wide stack COPYs into per-byte COPYs in the same cleanup
  pool right before `RuleStringCopy`, and those fresh outputs needed the local-scope addr-tied marking
  the global-only query never applied. For global-mapped arrays it falls back to
  `ArchHandle::query_container_global` (the merged tree's `ScopeLocal` db carries only the stack
  scope, splitting the C++ `getScopeLocal()->queryContainer` walk-to-global).

## Effect

573 → **588** (+15). Gained: Stack string #1/2/3/4/6/7/8/10/11/12/13/14 (+12) and Bitfields
#14/15/16 (+3 bonus — the set_varnode_properties OR gave the split per-byte stack COPYs their
addr-tied marking, which the bitfield member render also needed).

## Gate (at integration onto 573)

- datatests: `[675, 588]`; passing-set diff vs base 573: **regressed-set EMPTY** (gained exactly 15).
- `cargo test --workspace --no-fail-fast`: **0 failures** — Heap-string #1-7, for-loop, Chain-B,
  switch clusters all held.
- oracle `run_tests --baseline docs/baseline.json`: **PARITY OK**; C++ tree + docs byte-untouched.

## Residual

Stack string **#5** (overlapping null terminator at a two-string boundary — `v1[0xc]=0` dropped where
"goodbye all" also starts at byte 12) and **#9** (`v1[9]=a0` rendered as
`v1[8]=CONCAT11(a0,v1[8])` — a SubpieceCast/merge render artifact, the Subpiece/merge wave's
territory, not the COPY driver) remain out of scope.

## No special-casing

Faithful StringSequence transcription + the addr-tied local-query OR; no name/address/value
hardcoding. The keystone landed in `funcdata.rs` (`set_varnode_properties`), neither owned nor
forbidden; the existing deferral comment there is updated and the empty regressed-set confirms wiring
the local half is safe.
