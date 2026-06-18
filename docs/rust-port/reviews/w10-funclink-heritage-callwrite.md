# func_link_output LOCKED + heritage normalize_write_size CALL (w10-heritage-callwrite) — ACCEPT, +11

Wave chain (2026-06-18) integrated at rust-port (merge of `rport/w10-heritage-callwrite` @ `26d14e7`,
which carries `rport/w10-funclink-output` @ 1512b0c). Verdict: **ACCEPT**. Resolves LOSS-236.

## Chain (two waves)

1. **func_link_output LOCKED-output arm** (substrate 1512b0c, +7/-5 alone): the LOCKED-output build in
   `ActionFuncLink::func_link_output` (coreaction_protos.rs:736) was a SEAM. Ported C++
   `ActionFuncLink::funcLinkOutput` LOCKED arm (coreaction.cc:1582-1613): `new_varnode_out(sz, addr,
   callop)` for a non-spacebase non-bool locked output + `assumed_output_extension` post-call
   SEXT/ZEXT/PIECE + `op_mark_calculated_bool` (funcdata_op.rs:131). Gained Access packed fields #1-4 +
   Conditional Subpiece #4-6, but float-returning callees panicked on an unported heritage seam (-5).
2. **heritage normalize_write_size CALL arms** (this wave): removes the panic + the -5.

## heritage port

- `heritage.rs:2069-2074` (most-sig CALL arm) + `:2100-2104` (least-sig) — `if op_is_call &&
  call_op_indirect_effect(...) { new_indirect_creation(op, pieceaddr, size, ...) }` (C++
  heritage.cc:435-459). Reused the existing `Funcdata::new_indirect_creation` (from the killedbycall
  wave) — no new CALL primitive needed.
- `heritage.rs:1928` `call_op_indirect_effect` (C++ heritage.cc:359) + `fspec.rs:6116`
  `has_effect_translate` (C++ fspec.cc:5941).

## Effect

490 → **501** (+11). Gained: Access packed fields #1-4, Conditional Subpiece #4-6, Mixed float/int
#7-10 (the last 4 because `main` no longer panic-aborts). Mixed float/int #2-6 stay passing.

## Gate (re-run at integration onto 490)

- `cargo test --workspace --no-fail-fast`: 0 failures.
- datatests: `[675, 501]`; passing-set diff vs base 490: **regressed-set EMPTY** (gained exactly the 11).
- switch cluster: switchind 16/16, switchmulti 9/9 — unchanged. oracle **PARITY OK**; C++ byte-untouched.

## No special-casing

Faithful transcription of the LOCKED-output arm + the heritage CALL partial-write arms; no
name/address/value hardcoding. The LOCKED sibling of the killedbycall/init_active_output UNLOCKED arm.
This unblocks the broader committed-prototype-callee output recovery (retstruct cluster candidates).
