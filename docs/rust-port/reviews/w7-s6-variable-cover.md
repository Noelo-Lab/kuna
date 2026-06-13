# w7-s6-variable-cover
verdict: ACCEPT-WITH-LOSSES
verifier: claude-opus-4-8 (independent verifier, round 1)
date: 2026-06-13
gate: `cd rust && cargo test -p kuna-decomp` -> lib 1236/1236 pass; verify_w7_s6_variable_cover 5/5 pass; the only failure is `corpus_bootstrap::ldefs_pspec_cspec_parse_matrix` ("Could not find .sla file for x86:LE:64:default") — a missing gitignored build artifact (`make specs`), pre-existing and wholly unrelated to cover/variable (no HighVariable/Cover code on its path).

blob-sha drift: NONE — all four C++ blobs (`cover.cc cfaed4e2`, `cover.hh 7fd46e62`, `variable.cc 18af26c1`, `variable.hh 7b3ad439`) match the checklist `cpp_blob_sha` exactly. (`cargo run -p port-audit` unavailable: the `port-audit` crate is not present in this worktree — environment gap, not a finding.)

## hunt list
- Signedness: clean. `intersect_op_set`'s `setBlock>=blockStart.size()` / `opIndex<opMax` C++ signed/unsigned compares are non-negative; the Rust uses `usize` throughout (cover.rs:628-671). `finalize`'s `int4 blockNum=-1` < first block index ported exactly (cover.rs:792-799). `compare_by_block` orders int4 block index then uintm order (cover.rs:774-779), faithful.
- Integer widths: clean. `getUIndex` sentinels Begin/Input->0, End->`uintm::MAX` (=`~((uintm)0)`), Op caches the `uintm` SeqNum order (cover.rs:65-72). `uintm`=u32 confirmed. No promotion hazards.
- Wrapping: clean. No `uintb`-lineage arithmetic in the geometry; offsets are int4 adds (`groupOffset+size`, `off_diff`) that mirror C++ int4 adds and cannot overflow for physical layouts.
- Comparator totality: F1 (see findings) — `PieceKey` is a *total* order (offset,size,id) but that totality is exactly the divergence: it over-distinguishes vs the C++ `std::set` (offset,size)-equality, killing the duplicate guard. `HighEdge` derived `Ord` over the id pair is a faithful total order for the pointer-keyed map (ADR 0002); both directions always inserted/purged together so cache semantics are order-independent. `compare_just_loc`/`compare_name` ported field-for-field (variable.cc:439-488).
- Iteration-order provenance: clean. `Cover::cover` is `BTreeMap<int4,..>` == `std::map` ascending; the two-cursor walks of `intersect`/`intersect_list`/`merge`/`compareTo` visit in ascending block order (cover.rs:336-419). `pieceSet` is `BTreeSet<PieceKey>` ordered by (offset,size,id). No HashMap/HashSet anywhere (grep clean).
- Off-by-one / do-while / reverse iterators: clean. `intersect_op_set`'s C++ `do{}while(opIndex<opMax)` ported as `loop{ …; op_index+=1; if op_index>=op_max break }` keeping at-least-once (cover.rs:652-667). `lower_bound` -> `BTreeMap::range(k..)` (first key >= k) for both `cover.lower_bound` and the `purgeHigh`/`moveIntersectTests` HighEdge ranges. Verified by `coverblock_two_piece_wraparound_geometry` and `intersect_op_set_skips_boundary_ops`.
- Erase-while-iterating: clean. `purgeHigh` and `moveIntersectTests`' careful C++ `--iterlast`/`erase(iter++)` dances are ported as collect-then-remove over the same lower_bound ranges, removing the identical set of (a,b)+(b,a) edges (variable.rs:958-1044); re-derived edge-for-edge. `mergeInternal`'s `std::merge` (stable, first-range-wins-ties) ported as `merge_inst_lists` pushing `b` only when `b<a` strictly (variable.rs:1444-1456). `combineGroups`/`mergeGroups` snapshot the op2 member list before transfer (variable.rs:1557-1565, 1688-1715).
- Exception -> Result parity: F1 (the one divergence) — `addPiece`'s `LowlevelError("Duplicate VariablePiece")` is now unreachable. The other throws are faithful `?`-propagating `KunaResult`: `mergeInternal` non-speculative-after-speculative (variable.rs:1394), `merge` speculative-cross-group (variable.rs:1803), `hasName`/`getTiedVarnode`/`getInputVarnode` (variable.rs:741-813), `establishGroupSymbolOffset` (not in this sliver). `get_tied_varnode`'s `(addrtied|insert)==(addrtied|insert)` correctly inlines the real C++ `isAddrTied()` (varnode.hh:251) — verified, not a divergence.

## findings
- F1 (minor / loss LOSS-107): the `VariableGroup::pieceSet` duplicate-(offset,size) guard is dead. C++ `pieceSet` is `set<VariablePiece*,PieceCompareByOffset>` keyed only on (offset,size); `addPiece` throws "Duplicate VariablePiece" on a colliding insert (relied on by `combineGroups`). The Rust `PieceKey::cmp` appends a unique `id`, so `BTreeSet::insert` always succeeds and `group_add_piece`'s `Err` branch is unreachable — on a precondition-violating combine the C++ aborts the action where the Rust silently keeps a duplicate. Gate green; only manifests on input C++ also rejects. LOSS-089 defensive-guard precedent -> loss, not blocker.
     cpp: decompiler/cpp/variable.cc:33-39, :43-52, :78-89
     rust: rust/crates/kuna-decomp/src/variable.rs:174-202, :1476-1495
- F2 (minor): `get_type_representative` does `vn_view.type_.type_order_formal(&rep_view.type_).unwrap_or(0)` — swallowing the W6-SEAM `Err` from `type_order_formal` (the unported `hasSameVariableBase`/`hashSize` for variable-length bases) into 0 ("do not swap rep"). C++ `typeOrderFormal` never throws and returns a real comparison; on variable-length-base members the representative choice could differ. Tied to a pre-existing W6 SEAM (hashSize), narrow input domain.
     cpp: decompiler/cpp/variable.cc:392 (`0>vn->getType()->typeOrderFormal(*rep->getType())`)
     rust: rust/crates/kuna-decomp/src/variable.rs:584
- F3 (minor / style): the bare `as` casts in cover.rs/variable.rs (int4<->usize index/len conversions, e.g. cover.rs:629,642,650,796; variable.rs:294,298,419,423,709,1388) carry no `// cast:` justification comment, which the verifier protocol asks for. All are benign non-negative index/len conversions; consistent with existing codebase laxity (heritage.rs has the same unannotated casts). Not a correctness finding.
     cpp: n/a
     rust: rust/crates/kuna-decomp/src/cover.rs:629; rust/crates/kuna-decomp/src/variable.rs:294
- F4 (minor / seam): `op_uindex_code`'s INDIRECT arm decodes `getOpFromConst(getIn(1)->getAddr())` as `OpId::from(KeyData::from_ffi(addr.get_offset()))` (funcdata.rs:932) — but the sibling `newVarnodeIop` factory that would *establish* that iop-offset<->op-key encoding is explicitly NOT yet ported (funcdata_op.rs:968-986 defers `getOpFromConst` as a seam). The INDIRECT `getUIndex` path is therefore speculative/untested; the core cover algorithm uses the cached `uindex` and is unaffected. Funcdata seam wiring, just outside the cover.cc/variable.cc sliver.
     cpp: decompiler/cpp/cover.cc:44-46
     rust: rust/crates/kuna-decomp/src/funcdata.rs:924-940

## adversarial tests
(rust/crates/kuna-decomp/tests/verify_w7_s6_variable_cover.rs — 5 tests, all green)
- coverblock_two_piece_wraparound_geometry — the `ustart>ustop` two-piece (wrap) contain/intersect branches (cover.cc:90-119) the in-module tests omit.
- coverblock_merge_full_block_and_disjoint — `merge` collapsing to `setAll()` (whole block) and the disjoint earliest-start/latest-stop branch, both orderings (cover.cc:147-184).
- cover_two_cursor_walk_orders_and_classifies — ascending two-cursor walk of `intersect` (short-circuit 2) and `intersect_list` (level threshold, ascending order, absent-block skip).
- intersect_op_set_skips_boundary_ops — the `do-while` op scan with boundary ops (start=2/tail=1) excluded from `affectsTest` and only the interior (boundary 0) op tested (cover.cc:368-377).
- group_with_equal_offset_size_should_error_like_cpp_but_does_not — witnesses F1: `group_with` with equal (offset,size) returns `Ok(())` where C++ throws; flagged to flip to `is_err()` when restored.

losses: LOSS-107 (F1). F2/F3/F4 are minor non-blocking observations carried in this verdict, not separate loss ledger entries (F2/F4 reference pre-existing W6/W3-varnode seams; F3 is style).
