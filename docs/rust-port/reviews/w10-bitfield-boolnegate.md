# BOOL_NEGATE printc dispatch (w10-bitfield-subsystem) — ACCEPT, +2

Wave (2026-06-17) integrated at rust-port (merge of `rport/w10-bitfield-subsystem` @ `ec4e2f07`,
base 60756b5/430). Verdict: **ACCEPT**. (The wave was scoped at the bitfield cluster but the
fix it found was a general BOOL_NEGATE render gap.)

## Root cause

`CPUI_BOOL_NEGATE` had NO printc dispatch — it fell through `op_emit_kind`'s
`_ => OpEmitKind::Custom` arm (`printc.rs:919`) to `op_func_ir`, rendering `BOOL_NEGATE(x)`
instead of `!x`. C++ `PrintC::opBoolNegate` (`printc.cc:834`) is a hand-written override (not a
token-table entry) with double-negation cancellation (`negatetoken` mod) and the
flip-the-next-operator optimization (`checkPrintNegation`, `printc.cc:2464`, via
`get_booleanflip`).

## Fix

- `printc.rs:2783` — `CPUI_BOOL_NEGATE` dispatch arm.
- `printc.rs` — `op_bool_negate_ir` + `check_print_negation` (after `op_unary_ir`).
- The negate-token flip consumer side (`NEGATETOKEN` mod, `token_negate`, consumed in
  `op_binary_ir`/`op_unary_ir`) was already present; this wired the missing PRODUCER side.

## Effect

430 → 432 on its base; +2 on the integrated 433 tree → **435**. Gained: Bitfields #8,
MIPS Bitfields #10 (the two bitfield assertions whose render needed `!x`).

## Remaining bitfield failures (NOT this root — for future waves)

The bitfield expression machinery (`RuleBitField*`/`RulePullAbsorb`/`RuleInsertAbsorb`) is fully
ported and wired (universalaction.rs:564-569). The remaining ~26 (Bitfields/MIPS #11-21,28,29)
are blocked on **stack-local struct typing** — the LOSS-156/070/153 chain (the stack symbol never
gets the `myfoo` type because RuleLoad/StoreVarnode never feeds a sized stack hint and
callee-proto propagation doesn't type the `&v2` call argument). Plus #2 (INT_LESS→INT_LESSEQUAL
normalization, ruleaction.cc:1317/2249) and #4 (the deliberately-gated SUBPIECE-cast arm in
`op_subpiece_ir`, off to avoid a `condconstsub` regression). These overlap the stack-frame
cluster wave.

## Gate (re-run at integration onto 433)

- `cargo test --workspace` (debug): 0 failures.
- datatests: `[675, 435]`; passing-set diff vs base 433: **regressed-set EMPTY** (gained
  Bitfields #8, MIPS Bitfields #10).
- switch cluster: 8/16/3 — unchanged. oracle **PARITY OK**; C++ tree byte-untouched.

## No special-casing

A general per-opcode render override matching C++ `opBoolNegate`/`checkPrintNegation`; no
name/address/value hardcoding.
