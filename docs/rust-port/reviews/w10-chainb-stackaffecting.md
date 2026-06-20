# Chain B substrate consolidation + StackAffectingOps (w10-chainb-finish) — ACCEPT (partial), +1

Wave (2026-06-17) integrated at rust-port (merge of `rport/w10-chainb-finish` @ `677260d`, base
e49f10e/476). Verdict: **ACCEPT (partial)**. Lands +1 and consolidates the entire 6-wave Chain B
stack-typing substrate into mainline (query_local_properties plumbing, is_unmapped_unaliased,
RuleIndirectCollapse, the full restructureVarnode markNotMapped tail, StackAffectingOps) — all
faithful + regressed-set EMPTY, with the heritage.rs:1400 OR still UNWIRED (no behavior change
beyond the +1).

## The +1 root + fix

`Merge::testUntiedCallIntersection`'s `StackAffectingOps` source (merge.cc:63-89) was stubbed
(`populate_affecting_ops` empty + blanket `affectsTest=false`), so an untied stack value never
registered as crossing a call → forced INDIRECT/MULTIEQUAL merges silently fused. Ported faithfully:
- `funcdata_merge.rs:884` `populate_affecting_ops` (StackAffectingOps::populate — CALL ops + valid
  store-guard STOREs into the cache PcodeOpSet).
- `cover.rs` `AffectsTest` enum on `PcodeOpSetEntry` (CALL/unguarded → Always; guarded STORE →
  isGuarded(rep_addr)); `intersect_op_set` takes `&Address`.
- `merge.rs:896` threads `vn_addr(vn)` as the rep address; `funcdata.rs` heritage store-guard accessor.

Gained `Local cross #1` directly (OR off). `local_cross` (interior cross) passes; `store_cross`
(boundary cross) does not — see sub-gates.

## Gate (re-run at integration onto 479)

- `cargo test --workspace` (debug): 0 failures.
- datatests: `[675, 480]`; passing-set diff vs base 479: **regressed-set EMPTY** (gained Local
  cross #1; ModuloAlt's +3 preserved).
- switch cluster 8/16/3 unchanged; oracle **PARITY OK**; C++ tree byte-untouched.

## Two NEW sub-gates (the OR still gives +8/-4, unwired)

- **Gap-1 (Store cross #1/#2):** the addrforced `mergeIndirect` still fuses ECX (phi'd v2) into
  `local_array[10]`. IR-verified: ECX's block-4 cover stop coincides EXACTLY with the call op's
  uindex (boundary==1), so both the block-cover test AND the now-populated
  testUntiedCallIntersection return false — identically to what C++ computes on this geometry. Yet
  the oracle merges (inserts `allocateCopyTrim u=ECX`). The C++ failure mechanism is upstream of
  testUntiedCallIntersection (likely the addrforced INDIRECT / its getIn(1) op-order under the OR,
  or a mergeAddrTied/mergeRangeMust interaction). Needs C++-side dynamic instrumentation.
- **Gap-2 (Intermediate pointers #3/#5):** the `&v1.arr1[a]` spill slots resolve through
  `sync_varnodes_with_symbols`'s in_scope (mapped|addrtied) arm, never the unmapped nolocalalias
  arm, so RuleIndirectCollapse never drops their INDIRECTs. The restructure tail's markNotMapped is
  NOT making in_scope return false for these auto spill slots — the markNotMapped↔in_scope wiring is
  the open sub-gate (varmap.rs/funcdata_spacebase.rs).

## No special-casing

Faithful StackAffectingOps port; no name/address/value hardcoding.
