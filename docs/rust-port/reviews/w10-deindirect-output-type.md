# Deindirect Output #1 — locked-output type propagation (w10-deindirect-output-type) — ACCEPT, +1

Wave (2026-06-18) integrated at rust-port (merge of `rport/w10-deindirect-output-type` @ `f7b376a`,
base 26ea141/481). Verdict: **ACCEPT**. Cascade follow-up from the call-return/ActionDeindirect wave.

## Root + fix

`output_type_local` (`coreaction_infertypes.rs:59`, the port of `op->outputTypeLocal()`) was missing
the CALL/CALLIND prototype arm. C++ `TypeOpCall::getOutputLocal` (typeop.cc:722-738) and
`TypeOpCallind::getOutputLocal` (typeop.cc:778-791) return `fc->getOutputType()` when the resolved
`FuncCallSpecs` has a locked, non-VOID output prototype; `Varnode::getLocalType` (varnode.cc:930)
consumes it. The port had the symmetric input arm (`call_input_type_local`) but no output
equivalent, so a deindirected CALL to `int4 *obtainPtr(char *)` left its return Varnode at
`undefined8` and ActionSetCasts inserted a spurious `(int4 *)` → `v1 = (int4 *)obtainPtr(nm)` vs the
oracle `v1 = obtainPtr(nm)`.

Fix: added `call_output_type_local` (faithful — CALL requires in(0) to be an IPTR_FSPEC ref; both
resolve the spec via get_call_specs, gate on `is_output_locked()`, drop VOID) wired into
`output_type_local` before the generic size-only fall-through. `FuncProto::is_output_locked`
(fspec.rs:5007) + `get_output_type` (fspec.rs:4952) already existed.

## Effect

481 → 482. Gained: Deindirect Output #1. (The deindirect family is now fully recovered: Output
#1/#2/#3 all pass.)

## Gate (re-run at integration onto 481)

- `cargo test --workspace --no-fail-fast`: 0 failures.
- datatests: `[675, 482]`; passing-set diff vs base 481: **regressed-set EMPTY** (gained Deindirect
  Output #1). Deindirect Output #2/#3, indirect prototype, and all call-output/cast assertions
  unchanged.
- switch cluster 8/16/3 unchanged; oracle **PARITY OK**; C++ tree byte-untouched.

## No special-casing

Extended the shared type-inference path (the symmetric output arm of the existing input arm); no
name/address/value hardcoding.
