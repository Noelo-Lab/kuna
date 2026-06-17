# mixed float/int proto param names (w10-mixed-floatint) — ACCEPT, +3

Wave (2026-06-17) integrated at rust-port (merge of `rport/w10-mixed-floatint` @ `b82f7bb`,
base d9a8376/465). Verdict: **ACCEPT**.

## Root + fix

`emit_prototype_inputs` (`printc.rs:1700`) emitted `param.get_name()` verbatim, which is `""`
for recovered (unlocked) proto params. Upstream resolves an empty-name param through its scope
symbol via `Scope::buildDefaultName` (`database.cc:2511`, kuna `database.rs:156` `kuna_arg_name`
function_parameter branch) to `a<i>` (angr) / `param_<i+1>` (ghidra); kuna's `ProtoStoreInternal`
(fspec.rs:4629, fed by `update_input_types` coreaction_protos.rs:1467 with `""`) kept the literal
empty name. Fix: positional default-name fallback in the print path, gated on empty name only.
Faithful, no special-casing.

## Effect

465 → 468. Gained: Mixed float/int #3 (`float4 fi(float4 a0,int4 a1)`), #4
(`int4 id(int4 a0,float8 a1)`), #5 (`int4 ldld(int4 a0,float8 a1,int4 a2,float8 a3)`).

## Gate (re-run at integration onto 465)

- `cargo test --workspace` (debug): 0 failures. The `verify_w10_input_params` no-regression fence
  was re-pinned from the old unnamed `void nanops(float8,float8)` to the oracle-faithful
  `void nanops(float8 a0,float8 a1)` (confirmed against the cpp oracle).
- datatests: `[675, 468]`; passing-set diff vs base 465: **regressed-set EMPTY** (gained the 3).
- switch cluster unchanged; oracle **PARITY OK**; C++ tree byte-untouched.

## Remaining mixed-float failures (separate root, out of scope)

#1 (`dldlll`) and #7-#10 (`main`) are XMM `float8` register parameter/output recovery failures —
the 8-byte float read at `XMM0_Qa` is lane-divided into two `xunknown4` halves before param
recovery and never re-typed to float8 (`coreaction_render.rs` lanedivide,
`coreaction_cleanup.rs:1585` XMM0 lane/symbol-conflict, + `type.rs` float propagation). Where
recovery succeeds the float↔int casts already render correctly — NOT a conversion-rule gap.

## No special-casing

Positional default-name fallback matching C++ `Scope::buildDefaultName`; gated on empty name only.
