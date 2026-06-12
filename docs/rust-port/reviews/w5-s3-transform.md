# w5-s3-transform
verdict: ACCEPT-WITH-LOSSES
verifier: claude-opus (independent verifier, round 1)
date: 2026-06-11
gate: `cd rust && cargo test -p kuna-decomp` -> ok (647 passed, 0 failed, incl. the 34 in-crate transform tests + 12 new adversarial tests)

scope: decompiler/cpp/transform.cc (blob 538d47bc..., matches checklist), decompiler/cpp/transform.hh (blob 58bdecb4..., matches checklist). Rust: rust/crates/kuna-decomp/src/transform.rs (+ transform/tests.rs). The dtype.rs hunk in the branch diff belongs to the parent w5-dtype-expand item (already reviewed) and is out of scope here.

## hunt list
- **Signedness**: clean. `getBoundary` uses `int4` min/max with `len() as int4 - 1` underflowing to -1 on an empty `lanePosition` exactly as the C++ `size()-1` (the `min<=max` guard then fails — faithful, verified by `w5s3_get_boundary_single_lane_and_whole_size`). `getPiece` `res.val != lsb_offset as uintb` reproduces the C++ uintb-vs-int4 implicit conversion (i32→u64 sign-extend == C++). pieceMap key `getCreateIndex() as int4` (u32→i32) reproduces the C++ `map<int4>` implicit-conversion key, INCLUDING the >2^31 negative-wraparound ordering (BTreeMap<i32> orders the same) — clean.
- **Integer widths**: clean. int4/uint4/uintb mappings consistent (i32/u32/u64). `bitpos as uintb` / `lsb_offset as uintb` are i32→u64 sign-extend == C++ int4→uintb; values are non-negative positions in practice. `LaneDescription::uniform`'s `origSize/sz` is plain i32 division == C++ `int4/int4`.
- **Wrapping**: clean. The only arithmetic on uintb-lineage values is `(val >> lsbOffset) & calc_mask` (newConstant) and `(offset >> bitpos) & calc_mask` (newSplit const-lane), both via `wshr`/`wshl` (Wrap trait, x86-style count mask) guarded by `bitpos < 64` exactly as C++ `bitpos < sizeof(uintb)*8`. `addLaneSize`/`allowedLane` use `wshl`/`wshr` with sizes gated to [0,16] — no wrap reachable; the degenerate >=32 case where C++ `1<<size` is UB matches the port's x86-style mask (both agree). Verified by `w5s3_new_constant_masks_and_strips_beyond_precision`.
- **Comparator totality**: n/a — no `operator<`/comparator in this item (pieceMap is keyed by a plain `int4`, BTreeMap's derived Ord on i32 is total).
- **Iteration-order provenance**: clean and load-bearing. `pieceMap` = `BTreeMap<int4,Vec<TransformVar>>` reproduces `map<int4,TransformVar*>` ascending-create-index order; `newOps`/`newVarnodes` = `Vec` (push == `list::emplace_back`); within a piece array the lanes are a `Vec` walked LSB-first to the `split_terminator`. NO HashMap/HashSet anywhere. The visit order drives downstream create-index allocation; pinned by `apply_visits_piece_map_before_newvarnodes` (in-crate) and the new `w5s3_apply_visits_pieces_in_create_index_order_not_insertion_order` (inserts the higher-create-index split first, asserts the lower one still materializes first).
- **Off-by-one / do-while / reverse iterators**: clean. The C++ `createOps` `do { followCount=0; ... } while(followCount!=0)` at-least-once loop is ported as `loop { ...; if follow_count==0 break; }` (runs the body once minimum) — faithful. `createVarnodes`' unbounded `for(int4 i=0;;++i) { ...; if split_terminator break; }` is ported as `for idx in 0..len { ...; if split_terminator break; }` — the `0..len` bound is a safety net that coincides with the terminator (always the last array element; pinned by `w5s3_split_terminator_marks_only_last_lane`). `getBoundary`'s `(min+max)/2` binary search ports exactly. The `LanedIterator` deref-then-increment semantics (`*iter` = current, `++iter` = advance+normalize) are reproduced in `Iterator::next` (return current, then advance) — verified by `w5s3_iter_normalize_skips_to_lowest_set_bit`.
- **Erase-while-iterating**: clean. `LaneDescription::subset` clears `lanePosition` BEFORE the rebuild loop while still reading the OLD `laneSize[i]` and building into a fresh `newLaneSize` assigned after the loop — the Rust mirrors the exact sequencing (clear `lane_position`, loop reading old `lane_size`, assign `lane_size = new_lane_size` after). Both `getBoundary` calls happen before the clear. Verified by `w5s3_subset_trims_suffix_and_rebases_positions` / `w5s3_subset_prefix_single_lane`. `removeOld` iterates `newOps` by index and only destroys the *original* op (not a list element) — no iterator invalidation.
- **Exception -> Result parity**: clean / documented. The C++ `throw LowlevelError` sites (getPiece mismatch, piece-not-byte-aligned, bad TransformVar type, bad lane size) map to `Err(KunaError::lowlevel(...))` with `?`-propagation. The seam `Err`s (LOSS-053) leave the same partial state the C++ would at that point (op created/create-index bumped before the opSetOpcode seam; first input's vn deleted before the setInputVarnode seam); apply() aborts via `?` and no W5 caller relies on partial state.

## mechanical pass
- port-audit: tool `port-audit` is not present in this workspace (`cargo run -p port-audit` → "package not found"); blob-sha drift checked manually instead — both transform.{cc,hh} blobs match the checklist's recorded `cpp_blob_sha` exactly (no drift).
- grep `todo!`/`unimplemented!`/`HashMap`/`HashSet`/`sort_unstable` in transform.rs + transform/tests.rs: NONE.
- bare `as` casts: all are `as usize` (container indexing), `as int4` (create-index u32→i32, intentional and faithful — see hunt list), or `as u32`/`as uintb` (shift counts / value widths matching C++ implicit conversions). None carry a `// cast:` comment (a style nit against the protocol letter), but each is a faithful width/index conversion and several are explained in adjacent line comments; not a correctness finding.
- `cargo clippy -p kuna-decomp -- -D warnings`: clean. (`--tests` surfaces lint debt in *other* items' pre-existing W3/W4 verifier test files — not in transform.rs/tests nor in the new verify file, which is clippy-clean in isolation.)

## findings
  - F1 (loss, not a blocker): `apply()` materialization stops at the Funcdata factory seams (opSetOpcode/W6, newVarnodeOut/newUniqueOut/setInputVarnode/transferVarnodeProperties/W3-varnode, markIndirectCreation/W4, getOpFromConst). A transform graph with any placeholder op, real (non-constant) piece, INDIRECT mark, constant_iop, or input piece cannot be fully applied; the preexisting input-arity reshape loops are present only as comments. The construction phase (all factories, dedup/guard, visit order, create-index bumps) is ported in full. Recorded as LOSS-053.
       cpp: decompiler/cpp/transform.cc:225,175,729,654
       rust: rust/crates/kuna-decomp/src/transform.rs:1172,1289,1402,1126
  - F2 (observation, unreachable — NOT a loss): `LanedRegister::parseSizes` token parser (`parse_int_autobase`) diverges from C++ `istringstream`+`unsetf(dec|hex|oct)` on MALFORMED tokens — C++ `>> sz` reads the longest valid integer PREFIX (e.g. "1abc"→1, "08"→0), the Rust full-string parse yields -1 (→ "Bad lane size" error). Real `vector_lane_sizes` spec values are clean small decimals ("1,2,4" / "1,2,4,8", confirmed by grepping specs/), for which both agree exactly (including the trailing-comma empty-token→error case, verified by `w5s3_parse_sizes_range_gate_and_trailing_comma`). Output-affecting only on a hand-corrupted spec attribute; not output-affecting at any real boundary, so no losses.md entry (cf. LOSS-051 which IS a loss only because its surface — adjustvma — is plausibly user-supplied).
       cpp: decompiler/cpp/transform.cc:319-325
       rust: rust/crates/kuna-decomp/src/transform.rs:1445

## adversarial tests
Added rust/crates/kuna-decomp/tests/verify_w5_s3_transform.rs (12 tests, all pass — confirming faithfulness, no divergence):
  - w5s3_get_boundary_single_lane_and_whole_size
  - w5s3_get_boundary_non_uniform_two_lane
  - w5s3_subset_trims_suffix_and_rebases_positions
  - w5s3_subset_prefix_single_lane
  - w5s3_subset_partial_intersection_fails_unmodified_check
  - w5s3_restriction_and_extension_differ_only_in_sign
  - w5s3_parse_sizes_range_gate_and_trailing_comma
  - w5s3_iter_normalize_skips_to_lowest_set_bit
  - w5s3_apply_visits_pieces_in_create_index_order_not_insertion_order
  - w5s3_new_constant_masks_and_strips_beyond_precision
  - w5s3_preexisting_guard_full_truth_table
  - w5s3_split_terminator_marks_only_last_lane

## losses
LOSS-053 (apply() materialization seams). F2 is an unreachable observation, not a cited loss.
