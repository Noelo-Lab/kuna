# RSP &v1-render REPAIR (w10-rsp-v1render-repair) — ACCEPT, +7

Wave (2026-06-17) integrated at rust-port (merge of `rport/w10-rsp-v1render-repair` @ `3fe92f7`,
base 1cc9c47/410). Verdict: **ACCEPT**. This is the repair of the earlier rejected &v1-render
layer (`w10-rsp-keystone-plan.md` CORRECTION-10); see also `w10-rsp-waveA2-extrapop.md` history.

## Why the first attempt was rejected, and the real root

The original &v1-render (06fc69f, base 7f1f4df) was +5 on ITS base but, merged onto the
ActionReturnSplit tree, regressed 6 Switch Multi assertions (#2/#4/#5/#6/#7/#8). The count
alone dropped only 1 (masked); the full passing-set diff exposed −6/+5.

The root was NOT a nodeSplit/input-effect structural interaction (the block duplication fired
identically). It was a **variable-numbering shift**: the &v1-render's
`Funcdata::name_undefined_spacebase_symbols` ran as an ALL-spacebases-first pre-pass. With
`INPUT_EFFECT_MARKING_ENABLED=true` the stack-pointer INPUT became `unaffected`, so its
`PTRSUB(stack,-0x28)` `&`-reference resolved and the pre-pass renamed the `&`-only stack struct
`v1`, consuming `base=1` AHEAD of the switchmulti loop-carried body register (which precedes the
stack-pointer input in location order). That renumbered the loop var `v1`→`v3`, so the scored
arms rendered `v3 + 10` instead of the oracle `v1 + 10`.

## Fix (faithful to C++ `ActionNameVars::linkSymbols`, coreaction.cc:3040-3074)

C++ builds `namerec` in LOCATION ORDER — const-space spacebase refs first (3040), then each
non-const space's spacebase refs INTERLEAVED with that space's body highs (3055,
spacebase-ref-before-body-high per Varnode) — and renames in that combined order; it does NOT
front-load all spacebases. The repair restricted `name_undefined_spacebase_symbols` to the
const-space pass only, factored a per-Varnode arm `name_undefined_spacebase_symbol_for_vn`
(funcdata_varnode.rs), and drives the register/stack-space spacebase rename from the
location-ordered body walk in `name_local_highs_angr` (coreaction_cleanup.rs ~1386), sharing the
`base` counter. switchhide's `&v1` still shares the named Symbol with body `v1.b` (stack member
highs follow the register input in location order). No special-casing.

## Effect

+6 on its base 410; +7 on the integrated 423 tree (the extra `Offset array #1` from the same
location-order correction). Gained: Switch Hide #3/#4, Return Value Input Register #6/#7,
Intermediate pointers #5, Multi-size return #3, Offset array #1.

## Gate (re-run at integration onto 423, after a 3-way reconcile)

The merge auto-resolved `coreaction_cleanup.rs`/`database.rs`/`varmap.rs` (namespace's
`name_local_highs_angr` qualification edit and the RSP location-ordered rename live in different
regions and COMPOSE correctly); the harness pin resolved to the `&v1` form (supersedes convert's
`PTRSUB(v1,-0x18)`).
- `cargo test --workspace` (debug): 3680 passed, 0 failed (adds the
  `rsp_l4l5_namerec_rename_is_location_ordered_not_spacebase_first` fence).
- datatests: `[675, 430]`; full per-assertion passing-set diff vs base 423: **regressed-set
  EMPTY** (gained exactly the 7 above).
- switch cluster: switchind 16/16, switchmulti 8/9, switchhide 3/4 (#1 fail) — no regression.
- Namespace #1/#2/#3/#4 all still pass (the shared-function auto-merge preserved both edits).
- oracle **PARITY OK**; C++ tree byte-untouched.

## No special-casing

The location-ordered rename is a general per-Varnode walk matching C++; no name/address/value
hardcoding. Brings in the L4/L5 substrate (cspec effect blocks, spacebase typing) faithfully.
