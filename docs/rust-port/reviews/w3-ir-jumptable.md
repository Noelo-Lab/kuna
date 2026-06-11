# w3-ir-jumptable
verdict: ACCEPT-WITH-LOSSES
verifier: claude-opus (independent verifier, structurally separated from porter)
date: 2026-06-11
gate: `cd rust && cargo test -p kuna-decomp` -> ok (174 in-crate + all integration suites green; new verifier suite verify_w3_ir_jumptable.rs: 9/9 pass). `cargo clippy -p kuna-decomp -- -D warnings` -> clean (exit 0).
cpp_blob_sha: jumptable.cc c8a8ef30… / jumptable.hh 2fe58131… — both match the checklist's recorded blobs on the branch (verified).

## Scope note
This is a 3.7k-LOC single-file port (budget exception by design). The port splits
cleanly into (i) a **structurally-complete, IR-portable** core that is fully
transcribed and tested, and (ii) a large **seam surface** (model recovery,
emulation, value-set ranges, the loader, structuring) that returns precise
`Err`/`// SEAM` shells because the subsystems it needs (W4/W5/W6/W7) do not exist
at this item's boundary. The seam surface is honest — `JumpBasic` is a unit
struct exposing only its pure static helpers, the only concrete `impl JumpModel`
is `JumpModelTrivial` (fully ported), and no ported method consumes a deferred
method's non-trivial result. The whole seam surface is ledgered as **LOSS-038**.

## Hunt list
- **Signedness**: clean. `int4` loop indices vs container sizes resolved
  deliberately (e.g. `internalIntersect` reverse loop uses `(0..len).rev()`,
  empty when len==0, matching the C++ `int4 i = size-1; i>=0` which is false for
  size 0). `IndexPair`/`Address` comparisons go through total `Ord`.
- **Integer widths**: ONE finding — F1 (`truncate`'s `step*nm`), see below.
  Other width-sensitive sites checked faithful: `quasiCopy` mask `1<<1 <<(bits-1)
  -1` (wsub handles the wrap), `getMaxValue` `coveringmask(...)+1 & calc_mask`,
  `getStride`'s `stride<<=1` (capped at 32), `collapseTable`'s
  `size*num` (`wrapping_mul`).
- **Wrapping**: clean. `uintb`-lineage arithmetic uses `wadd`/`wsub`/`wmul`
  (collapseTable nextaddr, sanityCheck diff, quasiCopy mask, truncate). No
  `wrapping_*` that should be `checked_*` spotted.
- **Comparator totality**: clean. `IndexPair::Ord` is total (position then index,
  both ints); `partial_cmp` delegates to `cmp` (no float). `LoadTable` sort uses
  `addr.cmp` (Address total order). No `partial_cmp().unwrap()` anywhere.
- **Iteration-order provenance, per loop**: clean. No `HashMap`/`HashSet` in the
  file. `block2addr` order: built in `switchOver`/`addBlockToSwitch` insertion
  order then `sort()` by `IndexPair` (unique (pos,index) pairs ⇒ stable-vs-
  unstable irrelevant); `numIndicesByBlock`/`getIndexByBlock` linear-scan the
  sorted vec, reproducing the C++ `equal_range`/`lower_bound(compareByPosition)`
  contiguous-run order exactly (pinned by indexpair_total_order test).
  `PathMeld::meldOps` preserves SeqNum execution order (the C++ merge-sort);
  `collapseTable` re-sorts then compacts in place. `default`-block tally walks
  the sorted vec in run order (faithful to the C++ while/while).
- **Off-by-one / do-while / reverse iterators**: clean. `findDeterminingVarnodes`
  `do{}while(path.size()>1)` ported as `loop { … if path.len()<=1 break }`
  (at-least-once preserved; the inner empty-break flows to the bottom break,
  matching the C++ outer-condition exit). `truncatePaths` `while(size>1)` + back-
  pop + `resize(cutPoint)` faithful (cut_point ≥ 0 guaranteed by caller).
  Reverse scans (`markPaths`/`getEarliestOp`) use `(0..len).rev()`.
- **Erase-while-iterating**: clean. Only `pop()` from the back of `op_meld`
  (truncatePaths) and the `path` stack (findDeterminingVarnodes) — both faithful
  to C++ `pop_back`. No map/list erase-during-traversal in scope.
- **Exception -> Result partial-state parity**: clean. `recoverMultistage`
  take()/restore-on-Err mirrors the C++ copy-then-clear + catch-restore (and C++
  catches `JumptableThunkError` and `LowlevelError` identically there).
  `recoverLabels` restores `origmodel`/`jmodel` even on the `build_labels` Err
  path. `sanityCheck` `mem::take`s addresstable/loadpoints and puts them back
  before propagating the model's Err. `JumptableThunkError` subtype is collapsed
  to a generic error (faithful at the only in-file catch site; subtype-drop noted
  in LOSS-038).
- **SeqNum/create_index allocation-order fidelity (ADR 0001)**: clean within
  scope. `meldOps` orders by `getSeqNum().getOrder()`; no new-op/varnode creation
  happens in the ported jumptable surface (op-creation lives in the seam models).
- **Intrusive-list splice-order fidelity (ADR 0001)**: n/a — jumptable does not
  splice op/varnode lists; `PathMeld` operates on its own `Vec<RootedOp>`/
  `Vec<VarnodeId>` (append-prefix + renumber faithful to the C++
  `insert(begin,…)` + rootVn renumber).

## Mechanical pass
- grep jumptable.rs: no `todo!`/`unimplemented!`/`HashMap`/`HashSet`/
  `sort_unstable`. All 38 `as` casts are standard ADR-0003 width casts
  (`as int4`/`as usize`/`as uintb`/…); no unjustified bare casts.
- `cargo clippy -p kuna-decomp -- -D warnings`: clean (exit 0).
- gate `cargo test -p kuna-decomp`: green.

## findings
  - F1 (minor): `JumpValuesRange::truncate` computes `step * nm` as a 64-bit
    `wrapping_mul` (both operands sign-extended to u64 first); C++ computes it as
    a 32-bit `int` multiply then sign-extends the int result into the u64 add.
    Diverges when `step*nm ≥ 2^31` (concrete trace: step=0x10000,nm=0x10000 over
    a 48-bit mask → C++ getSize 0, Rust getSize 0x10000). No end-to-end effect in
    the current port (every consumer — buildAddresses/emulatePath — is a W4/W5
    seam returning Err), and the triggering magnitude is non-physical for real
    switch tables, so it is a divergence, not a blocker. Recorded as LOSS-039
    and pinned by an executable divergence test.
       cpp: decompiler/cpp/jumptable.cc:268
       rust: rust/crates/kuna-decomp/src/jumptable.rs:1106
  - F2 (info, no action): the seam surface (CircleRange value-set ops,
    EmulateFunction, JumpBasic/JumpBasic2/JumpBasicOverride/JumpAssisted model
    bodies, recoverModel/recoverAddresses/matchModel/setOverride/
    checkForMultistage, backup2Switch, the warning sinks, JumptableThunkError
    subtype) is genuinely-absent dependency, declared and ledgered. Honest
    `Err`/`// SEAM` shells; no silently-wrong logic. Not a defect.
       cpp: decompiler/cpp/jumptable.cc (JumpBasic/JumpModel families)
       rust: rust/crates/kuna-decomp/src/jumptable.rs (seam shells)

## adversarial tests (rust/crates/kuna-decomp/tests/verify_w3_ir_jumptable.rs, committed to the branch)
- truncate_small_product_matches_cpp_int32 (faithful common case)
- truncate_step_times_nm_overflows_int32_diverges_from_cpp (F1 divergence trace)
- collapse_table_overlap_entry_is_skipped (the fall-through skip branch)
- collapse_table_size_change_starts_new_table (new-table else-if + in-place write)
- collapse_table_unsorted_input_is_sorted_then_merged (sort-then-merge slow path)
- indexpair_total_order_position_then_index (comparator totality + scan order)
- default_range_iterates_base_then_extra_value_last (extra value last/not-reversible)
- decode_labeled_then_unlabeled_backfills_no_label (NO_LABEL back-fill)
- decode_unlabeled_then_labeled_is_an_error (label-ordering enforcement)

## losses
- LOSS-038 (jumptable model-recovery / emulation / CircleRange / loader seams) — cited for the accepted seam surface (F2).
- LOSS-039 (truncate step*nm 64-bit vs C++ int32 divergence) — cited for F1.

## verdict rationale
ACCEPT-WITH-LOSSES. The structurally-complete core (LoadTable/collapseTable,
PathMeld, GuardRecord static logic, JumpValues family, JumpModelTrivial, JumpBasic
static helpers, the JumpTable bookkeeping driver incl. switchOver/sanityCheck/
block2Position/isReachable/encode/decode) is a faithful, gate-green,
clippy-clean, adversarially-verified transcription with correct iteration-order,
wrapping, signedness, and partial-state parity. The one numeric divergence (F1)
has no current end-to-end effect and a non-physical trigger; it is recorded as a
cited loss with restoration criteria. The large seam surface is genuinely-absent
W4/W5/W6/W7 dependency, declared honestly and ledgered as LOSS-038. No
blocker/major stands.
