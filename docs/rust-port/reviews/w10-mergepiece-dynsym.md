# w10-mergepiece-dynsym
verdict: ACCEPT-WITH-LOSSES
verifier: independent (Opus 4.8, round 2)
date: 2026-06-16
branch: rport/w10-mergepiece-dynsym @ 23d7ced (base rust-port @ 7514d11)

NOTE: this supersedes the round-1 REJECT (branch was @ d2e554e / 326-675). The
round-1 F1 (`retstruct` "Return Structure #3" regressing 327->326) was remediated
by commit 23d7ced (dynamic-mapping idempotency + partialCopyShadow endianness);
the branch is now 327/675 with a base-byte-identical passing set.

## Gate (run fresh)
- C++ oracle (untouched — rust-only diff): `python -m kuna.run_tests --all --baseline docs/baseline.json`
  -> 207/207 unit + 675/675 datatest, **PARITY OK**. C++/specs/baseline byte-identical base vs branch.
- Rust workspace: `cargo test --workspace` -> **all green, 0 failed** (incl. the 4 new adversarial tests).
- Clippy: `cargo clippy -p kuna-decomp --lib` -> **clean**; new test target -> clean.
- Rust-engine datatests: `KUNA_ENGINE=rust run_tests --datatests` -> **327/675** (>= 326 required).
- Passing SET vs base (7514d11): 348-line FAIL set, **diff EMPTY both directions** — 0 gained, 0 regressed.
- boolless / readstruct / condconst_conn: byte-identical (workspace-green + FAIL-set-diff; boolless BE stable).
- B0 `universalaction_listing` -> 1 passed (pass order unchanged; coreaction_render edit is body-only).
- Reserved files (coreaction_cleanup.rs / architecture.rs / fspec.rs / coreaction_protos.rs): **none touched.**

## What the branch does (faithfulness, verified independently against C++)
- **Piece read-back** (funcdata_merge.rs + variable.rs): `high_group_info`/`high_piece`/`piece_offset`/
  `piece_size`/`piece_group`/`piece_high`/`piece_num_intersection`/`piece_intersection`/
  `bank_update_piece_intersections` now read the real `VariableGroup`/`VariablePiece` arena (1:1 with
  variable.hh:61-91). `high_internal_cover` returns the RAW `high->internalCover` (merge.cc:1622) — the
  correct fix vs the prior `getCover()`. `decode_piece` narrows `MergePieceId(u64)`->`VariablePieceId(u32)`;
  the encoding is always `u64::from(u32)`, so `as u32` is exact.
- **partialCopyShadow / findSubpieceShadow / findPieceShadow** (funcdata.rs): statement-by-statement
  transcription of varnode.cc:1014-1147. COPY-chain walk, SUBPIECE/MULTIEQUAL recursion (depth-1 truncation,
  incremented `recurse` passed down), constant `whole` shift `>> leastByte*8` (bounded < 64), PIECE
  backtracking size arithmetic — all verified to match. inflateTest caller (merge.cc:1616-1647) faithfully
  de-shadows the C++ double-`i` inner loop to `i`/`j`.
- **attemptDynamicMapping[Late]** + **ActionDynamicMapping/Symbols::apply**: faithful bodies. Scope iteration
  is `beginDynamic()..endDynamic()` = C++ `list` insertion order; Rust `dynamic_entries` snapshots
  `Vec<Option<SymbolEntry>>` in index order (`push`-appended, `None`-skipped) == C++ order. The dynamic-map
  idempotency guard (`vn_high_has_dynamic_binding`) reproduces `getSymbolEntry()!=0`.
- **Console `map hash` carry-across** (database.rs/decompile_drive.rs/ifacedecomp.rs): dynamic counterpart
  of the existing `map addr` re-seed; `scope_dynamic_symbol_specs` filters `get_offset()!=0` (whole-map start).

## Mandatory hunt list
- Signedness: clean (de-shadowed inflateTest loop preserves iteration; no new signed/size compare).
- Integer widths: clean (`getIn(1)->getOffset() as int4` == C++ `(int4)`; `.size() as int4` == C++ int4 return).
- Wrapping: clean (no uintb-lineage arithmetic beyond faithful shifts/`calc_mask`).
- Comparator totality: n/a.
- Iteration-order: dynamic-entry list == insertion order (Vec index, None-skipped); piece intersections by
  index. NO HashMap/HashSet in any added path.
- Off-by-one / do-while / reverse iterators: `findPieceShadow` size arithmetic + recursion-depth truncation
  verified faithful.
- Erase-while-iterating: collect-then-process snapshot == C++ `++iter`-before-apply (apply adds no entries).
- Exception->Result: C++ `attemptDynamicMapping` throw (`dynamic symbol outside local scope`) -> `Ok(false)`
  when localmap None (invariant-violation path, unreachable on corpus).

## Mechanical pass
- `todo!`/`unimplemented!`/`HashMap`/`HashSet`/`sort_unstable`/`panic!`: NONE in the diff.
- 3 `.unwrap()`: all provably-Some (guarded by prior code-check or `len()==1`).
- 4 bare `as` casts: all faithful/safe; 3 carry explanatory comments, 1 the `decode_piece` doc — none with
  the literal `// cast:` prefix (minor protocol nit, not a correctness issue).

## NO special-casing
Full-diff grep: ZERO hardcoded symbol-name / function-name / address / magic-offset literals in production
code. Only literals are the C++-faithful `warningHeader` text + one doc-comment fragment; numeric compares
(`!=0`,`<0`,`==0`,`>1`,`==1`) all match C++ anchors. `retstruct`/`0x1e`/`partialmerge` tokens appear ONLY in
the verifier test file.

## findings
- F1 (low / latent, accepted as LOSS-210): `varnode_partial_copy_shadow` reads `bigEndian` from parameter
  `b` (C++ `op2`) instead of `a` (C++ `this`, the `getSpace()->isBigEndian()` source — established from all
  four seam callers: trait-`a` == C++-`this`). Diverges only for a big-endian arch with a CONSTANT operand
  (constant space is HOST_ENDIAN per space.cc:356-362). **Proven inert**: patched to read from `a`, rebuilt,
  FAIL set BYTE-IDENTICAL (327/675, incl. all 8 BE datatest targets). Documented in-code.
       cpp: decompiler/cpp/varnode.cc:1140
       rust: rust/crates/kuna-decomp/src/funcdata.rs (`varnode_partial_copy_shadow`, big_endian from `b`)
- F2 (loss, accepted as LOSS-210): early `setSymbolProperties::updateType` and late `localmap->retypeSymbol`
  are no-op stand-ins (W4 Varnode->SymbolEntry retype seam; printer renders via `kuna_symbol_type`). End-state
  (name + `mapped` flag) is corpus-byte-stable.
       cpp: decompiler/cpp/funcdata_varnode.cc:1426-1432
       rust: rust/crates/kuna-decomp/src/funcdata.rs (`attempt_dynamic_mapping_late`, `let _ = sym_type;`)

## adversarial tests (committed: rust/crates/kuna-decomp/tests/verify_w10_mergepiece_dynsym.rs)
- `w10_mp_r2_retstruct_struct_3_not_regressed` — Round-1 F1 regression guard (structure-return body survives).
- `w10_mp_r2_partial_merge_11_payoff_stable` — pieced-inflate payoff witness (`return glob1.a + 7;`).
- `w10_mp_r2_boolless_be_unperturbed` — big-endian shadow/piece path leaves boolless byte-stable.
- `w10_mp_r2_copytrim_dynsym_idempotent` — early+late dynamic scan idempotency (no double-bind corruption).
All 4 pass on the branch; each is derived from the C++ oracle B5 and SKIPs (never false-passes) when the
binary/specs are absent.

## losses
LOSS-210 (endianness-source latent divergence + W4 dynamic-symbol type-retype seam). Faithful PARTIAL,
ZERO corpus regression -> ACCEPT-WITH-LOSSES.
