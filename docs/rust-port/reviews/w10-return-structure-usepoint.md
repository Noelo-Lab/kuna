# Return Structure — usepoint-scoped register-Symbol binding (w10-return-structure-usepoint) — ACCEPT, +5

Wave (2026-06-18) integrated at rust-port (merge of `worktree-agent-a2ca7032ee07b58c3` @ `68e9026`,
base `99deca7`; integrated onto 604 after Wave G). Verdict: **ACCEPT**. The residual of the PieceNode
grouping wave — the grouping rendered the retstruct body but needed the register Symbol bound at its
usepoint.

## The fix

- **`query_container_for_link(addr, usepoint)` (varmap.rs:1499)** — now threads the usepoint into
  `find_container` (was `Address::default()`). C++ `Funcdata::linkSymbol` (funcdata_varnode.cc:1190)
  calls `queryProperties(vn->getAddr(), 1, vn->getUsePoint(*this), fl)`; `SymbolEntry::inUse`
  (database.cc:115) makes an addr-tied / empty-`uselimit` Symbol in use at every code address
  (usepoint-independent), but a register-storage local with a non-empty `uselimit` range (the
  `type varnode %EAX(pc)` directive's usepoint-scoped Symbol) only matches when the usepoint falls in
  its range. Threading the real usepoint lets the `tmp` retstruct return Symbol bind at the register
  read it is scoped to.
- **`coreaction_cleanup.rs:~1956`** — linkSymbol call passes `data.vn_use_point(name_rep)` (C++
  `vn->getUsePoint(*this)`); the proto-partial-root (:1631) and spacebase (funcdata_varnode.rs:1316)
  callers keep an invalid usepoint (addr-tied → usepoint-independent, behavior-preserving).
- **`database.rs:2221` `scope_usepoint_symbol_specs`** + `funcdata.rs` `usepoint_symbol_specs` /
  `seed_usepoint_symbols` + `decompile_drive.rs:471` + `ifacedecomp.rs:1487` — carry register/
  usepoint-scoped symbols with their use address across the console IR rebuild.

## Integration note

Wave G (Partial Merge, integrated just before) also calls `query_container_for_link`; its
funcdata_merge.rs:198 global/addr-tied symbol-guard call was updated to pass `&invalid`
(usepoint-independent, per the SymbolEntry::inUse semantics above) — a behavior-preserving
merge-resolution edit by the integrator.

## Effect

604 → **609** (+5). Gained: Return Structure #1/#2/#4/#5/#6 (full cluster; #3 already passed).

## Gate (at integration onto 604)

- datatests `[675, 609]`; regressed-set **EMPTY** (switch / for-loop / Bitfields / Stack-string /
  Piece-Structure / Partial-splitting all byte-identical).
- `cargo test --workspace --no-fail-fast` (debug): **0 failures** (3 new adversarial tests in
  `verify_w10_usepoint_register_symbol.rs`).
- oracle `run_tests --baseline docs/baseline.json`: **PARITY OK**; C++ + docs byte-untouched.

## No special-casing

Faithful database.cc:115 / funcdata_varnode.cc:1190 usepoint transcription; no name/address hardcoding.
