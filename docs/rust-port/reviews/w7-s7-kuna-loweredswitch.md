# w7-s7-kuna-loweredswitch
verdict: ACCEPT-WITH-LOSSES
verifier: subagent (Opus 4.8 [1m])
date: 2026-06-13
round: 1
gate: `cd rust && cargo test -p kuna-decomp` -> lib 1239 passed (incl. 14 in-crate loweredswitch), verifier test bin 5 passed; the only FAILED is the pre-existing, out-of-scope `corpus_bootstrap::ldefs_pspec_cspec_parse_matrix` (`.sla` artifact gap — the worktree cannot build specs; documented LOSS-093, not this item).

scope: C++ `decompiler/cpp/kuna_loweredswitch.{cc,hh}` at the recorded `cpp_blob_sha`
(66f092dcc26f0afc56d26ba24044bea18f3e4662 / 68af7d6ec1470c1b171e96d6d8d22f84fbae8cf0) —
**confirmed no blob drift** (re-hashed both files, match exactly). Rust diff:
`rust/crates/kuna-decomp/src/kuna_loweredswitch.rs` (+968) + `kuna_loweredswitch/tests.rs`.

## hunt list
- **Signedness.** Clean. The two strict-`>` counter picks (`bestCount`/`best` init to `1`/`-1`, `int4`) and the `cases.len()`/`tgtset.len()` thresholds use Rust `usize`/`int4` consistently with the C++ `int4` and `.size()` operands; no signed/unsigned compare hazard (`< 3`, `> 16`, `< 2` are all unsigned len vs literal). Verified the `int4` vote counters (`+= 1`) cannot realistically overflow (bounded by block count).
- **Integer widths.** Clean. `uintb` case constants, `int4` var size, `Address` offsets map 1:1; `calc_mask(vn_size(...))` reproduces the C++ `calc_mask(cn.var->getSize())` all-ones sentinel exactly. No promotion divergence.
- **Wrapping.** Clean. No arithmetic on `uintb`-lineage values except the equality/order comparisons; `calc_mask` is the existing ported helper. No `wrapping_*`/`checked_*` decision points in this file.
- **Comparator totality.** Clean. The order-bearing keys are `BTreeMap<uintb,_>` (case values), `BTreeMap<Address,int4>` (default votes), `BTreeSet<Address>` (target set) — `Address::Ord` is total (space-rank then index then offset; re-read address.rs:499-532) and `uintb` is total. The store key `KunaLsKey` derives `Ord` over `(space_index: int4, offset: uintb)` — total.
- **Iteration-order provenance, per loop.** One real divergence (F1, below). The output-order loops are faithful: `cases` (std::map<uintb,Address> -> BTreeMap<uintb,_>) builds `case_vals`/`case_targets` in value order — MATCHES; `default_votes` (std::map<Address,int4>) tie-break picks lowest Address on the `> best` strict compare — MATCHES the BTreeMap order; the case-target distinctness/`tgtset` walk is order-insensitive. The `var_count` switch-var pick and the `cmpmap` head pick are keyed on slotmap `VarnodeId`/`BlockId` (creation order) where C++ uses `std::map<Varnode*/BlockBasic*>` (raw pointer order) — see F1.
- **Off-by-one / do-while / reverse iterators.** Clean. No do-while; the `for guard in 0..8` peel/guard loops match the C++ `for(int4 guard=0;guard<8;...)` bound. The fail-safe cap is exact: `cases.len() > 16` reject, `< 3` reject, `tgtset.len() < 2` reject — re-derived by V2 (16 accepted / 17 rejected).
- **Erase-while-iterating.** N/A — the detect side is read-only (no container erase during traversal); the DFS uses a `visited` set + work stack, transcribed faithfully (`stack.pop()` LIFO == C++ `stack.back()/pop_back()`).
- **Exception -> Result partial-state parity.** Clean. The detect body has no `?`-throwing mutation; on every reject it returns 0 leaving the store untouched (the C++ `return 0`/`return false` paths). `OptionLowerSwitch::apply` propagates `on_or_off` errors as `Err` matching the C++ `onOrOff` throw, with no partial state.
- **This wave's adds:**
  - *HighVariable merge / Cover-intersection order* — N/A here; detection runs pre-merge and keys on the canonicalized SSA `VarnodeId` (the C++ pre-merge pointer-identity argument; `canonSwitchVar` peel chain re-derived).
  - *varmap stack-layout order* — N/A.
  - *block-structuring schema precedence* — the structuring is in the INSTALL half (seamed, F2); the detect side only READS `isSwitchOut`/`sizeOut`/`getTrueOut`/`getFalseOut`, all faithful.

## mechanical pass
- greps over the Rust file + tests: NO `todo!`/`unimplemented!`/`HashMap`/`HashSet`/`sort_unstable`; NO bare numeric `as` casts. The two `BTreeMap`/`BTreeSet` `#[allow(clippy::mutable_key_type)]` on `Address` keys carry the established `heritage.rs` justification.
- `cargo clippy -p kuna-decomp` — NO findings in `kuna_loweredswitch.rs` (the `-D warnings` errors are all in the out-of-scope `kuna_regionid.rs`).
- `port-audit` — the `port-audit` crate is not a workspace member in this tree (cannot run); blob-sha re-hashed manually instead (no drift, above).

## findings
- F1 (minor / LOSS): the DETECT pass iterates `var_count: BTreeMap<VarnodeId,int4>` and `cmpmap: BTreeMap<BlockId,CmpNode>` in slotmap CREATION order, where C++ iterates `std::map<Varnode*,int4>` / `std::map<BlockBasic*,CmpNode>` in raw POINTER order. Both the switch-variable pick (strict-`>` first-at-max) and the head pick (first non-child, `break`) are tie-break-sensitive. On a genuine lowered switch (one dispatch var, unique head) neither tie fires and the record is identical; the divergence is only observable on spurious overlapping cascades. C++ pointer order is itself non-deterministic, so the Rust slotmap order is strictly *more* deterministic (LOSS-097 class).
       cpp: decompiler/cpp/kuna_loweredswitch.cc:455-462 (varCount pick) / :454,:481-490 (cmpmap head)
       rust: rust/crates/kuna-decomp/src/kuna_loweredswitch.rs:723,758-762 / :722,798-807
       -> LOSS-111
- F2 (loss / deferral): the entire INSTALL half is unported — `ActionLowerSwitchInstall::apply`, `Funcdata::kunaInstallLoweredSwitch` (the BRANCHIND + labelled JumpTable CFG surgery), and `JumpTable::kunaSetTrivialModel`. Needs the W4 JumpTable registry on `Funcdata`, the W7 Heritage handle (`getHeritagePass`), `removeUnreachableBlocks` (W4/W8), and `jmodel` access in the unowned jumptable.rs — none available at this boundary. Consequence: with `loweredswitch on` a lowered cascade still renders as an if/else chain (no `switch` is manufactured). Clearly documented in the module SEAM(W7/W4) note. The `kunaRecordRestart(...)` diagnostic-log call (:447) is likewise dropped (SEAM(W7)); behaviorally inert — the load-bearing `set_restart_pending(true)` IS realized.
       cpp: decompiler/cpp/kuna_loweredswitch.cc:296-321,:323-340,:350-432,:447
       rust: rust/crates/kuna-decomp/src/kuna_loweredswitch.rs:57-85,:825-829
       -> LOSS-110
- F3 (loss / deferral, folded into LOSS-110): the `OptionLowerSwitch : ArchOption` option-database registration and the insertion of the two Actions into `ActionDatabase::universalAction` are the W9 assembler's job (coreaction.cc/options.cc not owned here); the option parse body + Action gate are ported. Noted in the module SEAM(W9) doc.

## ported-faithfully (re-derived against the C++, all passing)
`canon_switch_var` (peel COPY/CAST/ZEXT/SEXT/zero-SUBPIECE, 8-step bound),
`is_pure_compare_block` (the allow-list incl. MULTIEQUAL/INT_AND/BOOL_NEGATE),
`analyze_cmp` (the EQUAL / NOTEQUAL match-is-not-taken-edge / SLESS|LESS range
branches, the boolean-flip-aware true/false-out selection, const-on-either-side),
`recover_cascade` (DFS, dup-case bail, `<3`/`>16`/`<2`/no-range/default-collision/
storage-type guards, lowest-Address default vote), `advance_past_guards` (the
all-ones `-1` sentinel skip), the head-finding (child set + first non-child), the
sticky `KunaLoweredSwitchStore` + `key_for_func`, and `OptionLowerSwitch::apply`.

## adversarial tests (committed on the branch, rust/crates/kuna-decomp/tests/verify_w7_s7_kuna_loweredswitch.rs)
- v1_var_count_tie_is_deterministic_first_created_wins  (F1 / LOSS-111)
- v2_cap_boundary_16_accepted_17_rejected               (cap off-by-one)
- v3_duplicate_case_value_bails                          (dup-case bail)
- v4_notequal_match_is_the_not_taken_edge                (analyzeCmp NOTEQUAL branch)
- v5_option_and_store_surface                            (option parse + store)
All 5 pass.

## losses
- LOSS-110 (install half + ArchOption/schedule wiring + kunaRecordRestart diagnostic; deferral)
- LOSS-111 (var_count/cmpmap slotmap-creation-order vs C++ pointer order; determinism improvement)

## rationale
The read-only DETECT half — the entire substance of this item's portable surface —
is transcribed faithfully with no faithfulness bug (no signedness/wrapping/width/
comparator/off-by-one/iteration-order defect in the ported logic). The two
divergences are (F2) a documented cross-wave deferral whose blocking primitives are
genuinely absent (same class as the already-accepted LOSS-035/036/078/093 jumptable
seams) and (F1) a determinism improvement over non-deterministic C++ pointer order
(LOSS-097 class). Both are now cited losses (the verifier appended LOSS-110/111 to
the main-tree losses.md). No blocker or major stands.
