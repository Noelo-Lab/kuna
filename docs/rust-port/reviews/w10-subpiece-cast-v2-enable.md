# SUBPIECE-cast render arm ENABLE (w10-subpiece-cast-v2) — ACCEPT, +3

Wave (2026-06-17) integrated at rust-port (merge of `rport/w10-subpiece-cast-v2` @ `d3e2f3e`,
base 1e4a2df/473). Verdict: **ACCEPT**. Closes LOSS-230 Family-1. A cascade from the call-return
recovery wave. (Distinct from the earlier `w10-subpiece-cast.md` which covered the original
disable/seam.)

## Context

The SUBPIECE→cast render arm was DELIBERATELY DISABLED at `printc.rs:3826` because enabling it
regressed `Modified conditional constant #4` via a spurious `SUB(ptr,0)` in `condconstsub.xml`.
The call-return-recovery + ActionDeindirect wave (now landed at 473) fixed that IR bug, so the
arm is unblocked.

## Change

`printc.rs:3826-3843` (`op_subpiece_ir`): replaced the gated `let _ = subpiece_is_cast(...);
op_func_ir(...)` tail with the faithful `if subpiece_is_cast(...) { op_type_cast_ir(...) } else
{ op_func_ir(...) }` (C++ `PrintC::opSubpiece` printc.cc:892-897). `subpiece_is_cast`
(printc.rs:3843) already reads high-facing types. One function changed.

## Effect

473 → 476. Gained: Bitfields #4, Union #8, Union #28. condconstsub #4 stays passing
(byte-identical). Union #14 did NOT flip — deeper union family (LOSS-230 Family-2/3).

## Gate (re-run at integration onto 473)

- `cargo test --workspace` (debug): 0 failures.
- datatests: `[675, 476]`; passing-set diff vs base 473: **regressed-set EMPTY** (gained the 3).
- switch cluster unchanged; oracle **PARITY OK**; C++ tree byte-untouched.

## Remaining (LOSS-230)

Union Family-2 (#19/#22/#25, FLOAT2FLOAT implied-cast in ActionSetCasts) and Family-3 (#4/#27,
inheritUnionField SEAM) remain open.
