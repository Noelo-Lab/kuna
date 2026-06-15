# w10-lanedivide
verdict: ACCEPT-WITH-LOSSES
verifier: Opus 4.8 (1M context), independent verifier (round 1)
date: 2026-06-15
branch: rport/w10-lanedivide @ 322503a (adversarial tests + commit at e96563c)

## Scope

Port of the lane-division analysis stack onto the W5 TransformManager:
- coreaction.cc:516-637 `ActionLaneDivide::apply` / `processVarnode` / `collectLaneSizes`
  -> coreaction_render.rs (`ActionLaneDivide::apply` + `lane_divide_process_varnode`
  + `lane_divide_collect_lane_sizes`)
- subflow.cc:3533-4143 `LaneDivide` engine (setReplacement, build{UnaryOp,BinaryOp,
  Piece,Multiequal,Indirect,Store,Load,RightShift,LeftShift,Zext}, trace{Forward,
  Backward}, processNextWork, doTrace) -> subflow.rs `LaneDivide`
- architecture.cc:291/313/933 `getLanedRegister`/`getMinimumLanedRegisterSize`/
  `decodeRegisterData` -> architecture.rs + seams.rs
- funcdata_varnode.cc:300 `checkForLanedRegister` + funcdata.cc lanedMap
  (`beginLaneAccess`/`endLaneAccess`/`clearLanedAccessMap`) -> funcdata_varnode.rs +
  funcdata.rs (`laned_map` BTreeMap, `LanedKey`)

The `LaneDescription`/`LanedRegister`/`restriction`/`extension`/`getBoundary` and the
TransformManager factory surface are W5 (`w5-s3-transform`, already verified); not in
this diff. Spot-checked faithful against transform.cc.

gate (fresh): cd rust && cargo build --workspace -> OK; cargo test --workspace ->
  3418 passed / 0 failed (+ the 1680-test harness + 4 new adversarial tests).
  Rust datatest corpus (decomp_test_dbg over decompiler/datatests): 187 Success /
  481 FAIL -> **passing SET byte-identical to rust-port baseline** (diff -> empty
  both ways; monotonic, zero new passes, zero regressions).
  cargo clippy --lib -p kuna-decomp -- -D warnings -> clean.
  Byte-parity fences (verify_w10_emptyblock_orform, print_b5_boolless) -> green.
  Files touched: ONLY rust/ (C++ oracle byte-identical -> 675/675 PARITY OK intact).

## Fleet discipline

- Files changed: architecture.rs, coreaction_render.rs, funcdata.rs,
  funcdata_varnode.rs, seams.rs, subflow.rs. ALL under rust/.
- printc.rs / printc/ / dtype.rs: **NOT touched** (reserved for the bitfield wave).
  Confirmed via `git diff --stat -- printc.rs dtype.rs` -> empty.
- No C++ files touched -> C++ oracle untouched.

## Mandatory hunt list

- Signedness: clean. `int4 max = lanerecords.size()-1` ported as `len() as int4 - 1`
  (signed) so the empty-table underflow to -1 matches the C++ (a `usize` subtraction
  would panic — see adversarial F-mutation below). All `int4` loop indices vs counts
  match C++; `srcLane - skipLanes < numLanes` style bounds transcribed verbatim.
- Integer widths: clean. `get_offset() as int4` for shiftSize/bytePos mirrors C++
  `(int4)...getOffset()`; `byte_pos as uintb` / `... as int8` mirror the C++ implicit
  int8->uintb / int4->int8 conversions in build{Store,Load}.
- Wrapping: clean. The binary-search `(min+max)/2` over <=16 records cannot overflow
  (same as C++); `1u32 << size` lane-mask shifts use the W5 `wshl` (transform.rs);
  no `uintb`-lineage arithmetic in the new code wraps.
- Comparator totality: clean. `LanedKey` derives `Ord` over
  `(space_index: int4, offset: u64, Reverse(size: u32))` — a faithful, total
  transcription of `VarnodeData::operator<` (pcoderaw.hh:67: space asc, offset asc,
  size DESC). No float compares.
- Iteration-order provenance: clean.
  * `lane_access_snapshot` = `laned_map.values()` over a **BTreeMap** = C++
    `std::map<VarnodeData,...>` key order (beginLaneAccess..endLaneAccess). Verified.
  * `descend_snapshot` = `Varnode::descend` list order (C++ beginDescend..endDescend).
  * the apply location loop uses `iter_loc_size_addr(sz,addr)` = C++ beginLoc(sz,addr)
    /endLoc(sz,addr) (size=s..size=s+1 lower_bound, verified against varnode.cc:1629).
  * NO HashMap/HashSet anywhere in the lane code (grep -> 0).
- Off-by-one / do-while / reverse iters: clean. `process_next_work` pops `work_list`
  back (`Vec::pop`) = C++ `workList.back()`+`pop_back()` (LIFO, matches). The
  build{Right,Left}Shift while-loops transcribed verbatim; `get_boundary` boundary
  branches (pos==wholeSize -> numLanes, miss -> -1) are W5, verified.
- Erase-while-iterating: clean by construction. The lane trace never mutates the real
  Varnode/Op graph (only the TransformManager placeholder graph) until `apply()`, so
  the up-front `descend_snapshot`/collect-Vec snapshots are equivalent to the C++
  `op = *iter++` cursor-before-mutate idiom. The apply location loop recalculates
  `iter_loc_size_addr` after every successful processVarnode (the only mutation point)
  exactly as the C++ recomputes beginLoc/endLoc; between recalcs only failed (no-op)
  processes run, so the stale Vec is provably valid there.
- Exception->Result partial-state: clean. `do_trace` clears varnode marks AFTER the
  loop regardless of success then returns retval (C++ order). `lane_divide_process_
  varnode` degrades a `tm.apply` Err to `return false` (= "no split"), so a partially
  built transform is never half-applied — matching the LOSS-053 seam behavior that
  SplitFlow/SubfloatFlow already exhibit.

## Mechanical pass

- grep todo!/unimplemented!/HashMap/HashSet/sort_unstable in added lines -> 0 hits.
- bare `as` casts: 23, each a faithful transcription of a C++ explicit `(int4)` or
  implicit narrowing/widening conversion (`get_offset() as int4`, `byte_pos as uintb`,
  `... as int8`, index `as usize`/`as int4`). No `// cast:` comments (a style nit
  consistent with the surrounding W5 SplitFlow/SubfloatFlow ported code in the same
  file); none introduces a wrap/signedness divergence. Not a finding.
- string literals added: only expect/panic messages + the pspec schema names
  "register_data"/"register"/"name"/"vector_lane_sizes" — these are the verbatim
  C++ ELEM_REGISTER_DATA/ELEM_REGISTER/ATTRIB_NAME/ATTRIB_VECTOR_LANE_SIZES
  constants (architecture.cc:74, transform.cc:21), NOT special-casing.

## No special-casing

Grepped the diff for function-name / address / value / type-name / field-name
hardcoding -> none. The `decode_register_data` resolves register size generically by
name through the translator (= C++ `storage.decodeFromAttributes` -> `getRegister`);
no "XMM"/"main"/address literals. The lane-size set comes entirely from the pspec
`vector_lane_sizes` attribute and the local SUBPIECE/PIECE op shapes — purely
data-driven. The x86-64 "3 records, min 16" is a consequence of the vendored pspec
(XMM 16B / YMM 32B / ZMM 64B each carry vector_lane_sizes), not a hardcode.

## Adversarial tests (rust/crates/kuna-decomp/tests/verify_w10_lanedivide.rs, 4 tests)

- `w10_lanedivide_empty_table_lookup_is_none_not_panic` — empty `lanerecords`
  binary search returns None for all sizes without a debug panic (the `int4 max =
  size()-1 == -1` underflow path). **Mutation-verified**: rewriting the cast as
  `(len()-1) as int4` (usize subtraction) makes this test FAIL with a debug overflow
  panic, confirming it guards the bug class.
- `w10_lanedivide_binary_search_exact_and_miss` — multi-record (x86-64 shape
  16/32/64) exact-hit at left/middle/right of the search + miss for every
  below/between/above size; confirms the record is keyed on size only (Address
  ignored) per C++ getLanedRegister.
- `w10_lanedivide_lane_size_mask_boundary_16` — addLaneSize(16) sets bit 16 of the
  32-bit mask (the XMM whole size), allowedLane/getSizeBitMask read it back, neighbours
  (7/9/15/17/0) NOT allowed, iter_sizes yields {8,16} ascending (normalize hits the
  lowest set bit, not bit 0).
- `w10_lanedivide_real_arch_empty_min_size_is_minus_one` — the real
  architecture::Architecture getMinimumLanedRegisterSize empty-table -> -1 path
  (distinct from the seam's cached field), the value build_arch_handle copies.

All 4 green on the branch; clippy-clean. Committed to the porter branch (e96563c).

## Findings

- F1 (loss, accepted): `LaneDivide::apply()` materializes through the shared
  `TransformManager::apply` factory seam, which errors on any placeholder op / real
  piece — so doTrace succeeds on the XMM lane scheme but apply() returns the seam Err
  and the action degrades to a no-op. This is the PRE-EXISTING, already-ledgered
  **LOSS-053**, which explicitly names `ActionLaneDivide` as a blocked consumer. Not a
  new gap; inherited from W5. Confirmed by the byte-identical 187/187 passing set.
     cpp: decompiler/cpp/subflow.cc:4127 (doTrace) + transform.cc:225 (apply seam)
     rust: rust/crates/kuna-decomp/src/coreaction_render.rs (apply Err -> false)
- F2 (loss, accepted): `decode_register_data` ports only the `vector_lane_sizes`
  half; the `volatile` attribute branch (symboltab property-range paint) is skipped.
  ZERO impact on the corpus (the x86-64 pspec has 96 vector_lane_sizes, 0 volatile).
  New ledger entry **LOSS-183**.
     cpp: decompiler/cpp/architecture.cc:933 (isVolatile branch)
     rust: rust/crates/kuna-decomp/src/architecture.rs (decode_register_data)

No correctness findings. No regression. No fabricated passes (the honest commit
message claims monotonicity, not new passes; verified the passing SET is identical
to rust-port). No reserved-file edits.

## Verdict rationale

Faithful, complete transcription of the lane scoring/lane/narrowing algorithm with
zero special-casing and zero regression. The only behavioral divergences are two
seam deferrals — LOSS-053 (inherited, the shared apply materialization that already
blocks SplitFlow/SubfloatFlow and is named for ActionLaneDivide) and the new minor
LOSS-183 (volatile half, zero corpus impact). A faithful PARTIAL with 0 regression is
ACCEPT-WITH-LOSSES per the verification protocol.

losses: LOSS-053 (inherited), LOSS-183 (new, appended)
