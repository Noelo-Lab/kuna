# enum4 RuleExpandLoad + HighVariable typeDirty propagation (w10-highvar-typerep) — ACCEPT, +3

Wave (2026-06-17) integrated at rust-port (merge of `rport/w10-highvar-typerep` @ `ffb0d79`,
base 843b0e2; carries the enum4 substrate merge `a9a686a`). Verdict: **ACCEPT**. Resolves
LOSS-228.

## Root cause

C++ `Varnode::updateType` (`varnode.cc:480-481`) always calls `high->typeDirty()` on any type
change; `HighVariable::updateType()` (`variable.cc:400-416`, gated on `typedirty`) then
recomputes `getTypeRepresentative` (`variable.cc:377`). The Rust `Varnode::update_type`
(`varnode.rs:1442-1450`) had a `SEAM(W7)` stub for the `high->typeDirty()` call — the Rust
`Varnode` lives in a different arena than its `HighVariable`, so it cannot reach the high.

Consequence: after `RuleExpandLoad::modifyAndComparison` (`ruleaction.cc:10931/10937`, the enum4
substrate) widened the `(V & C) == D` operands, the AND-output varnode got the size-8 `flags`
type but its high stayed `xunknown1` (size 1). `vn_high_type_read_facing` returned the stale
high type and `TypeOpEqual::getInputCast` (`typeop.cc`) emitted a spurious same-value `(flags)`
cast, breaking the enum.xml anchor `if ((ptrhigh->flagfield & ...`.

## Fix

New `Funcdata::vn_update_type` (`funcdata.rs:2052-2071`) sets the varnode type AND dirties the
high in the high-bank, faithfully reproducing `Varnode::updateType`. The 8 ActionSetCasts cast
sites (`coreaction_casts.rs` castOutput/castInput/insertPtrsubZero — the `coreaction.cc:2656-2794`
`updateType` calls) route through it. The matching `ActionInferTypes` site (coreaction.cc:5310)
already dirtied the high from a prior wave (`coreaction_infertypes.rs:283-294`), so this completes
the contract consistently rather than special-casing.

## Effect

+2 on its 413 base (Enum Reading #4 + Intermediate pointers #4); +3 on the integrated 430 tree
(also Intermediate pointers #3 — a positive cross-wave interaction with the landed RSP/convert
work). The bundled enum4 `RuleExpandLoad` substrate is the prerequisite (LOSS-228 substrate, +0
alone).

## Gate (re-run at integration onto 430)

- `cargo test --workspace` (debug): 0 failures.
- datatests: `[675, 433]`; full per-assertion passing-set diff vs base 430: **regressed-set
  EMPTY** (gained Enum Reading #4, Intermediate pointers #3, Intermediate pointers #4).
- switch cluster: switchind 16/16, switchmulti 8/9, switchhide 3/4 — unchanged.
- oracle **PARITY OK**; C++ tree byte-untouched.

## No special-casing

Completes a documented C++ contract (`high->typeDirty()` on every type change) at the cast sites;
general, no name/address/value/type hardcoding. `HighVariable::get_type_representative` itself was
already faithful — the bug was the missing dirty propagation from the varnode arena.
