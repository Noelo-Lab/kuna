# w10-global-persist2
verdict: ACCEPT-WITH-LOSSES
verifier: opus-4.8 (independent, round 2)
date: 2026-06-16
branch: rport/w10-global-persist2 @ a4540b4 (base rust-port @ 7a25277, merge-base == base, +4 commits)

gate: `cargo test --workspace` -> green (0 failed, all crates ok; kuna-decomp lib incl. 4 new heritage adversarial + the rewritten implied-vars adversarial); `cargo clippy -p kuna-decomp --lib` -> clean (no errors; pre-existing unused-import warnings only); rust-engine datatests -> 326/675 (base rust-port 319/675; +7 gains, ZERO regressions, passing SET diff = exactly {Revisit SSA #1..#7}); C++ oracle -> 207/207 unit + 675/675 datatest PARITY OK (byte-untouched; rust-only diff); B0 `decompile_tree_dump_is_byte_equal_to_oracle` -> 1 passed (pass order unchanged).

## Summary
This is the GLOBAL-PERSIST SEED that activates the previously-dormant `Heritage::removeRevisitedMarkers` (LOSS-207, landed inert at 0 hits). The branch adds `Architecture::decode_global` (cspec `<global>` -> seed the global scope's `ram` range), which makes a global RAM Varnode with no covering Symbol pick up `mapped|addrtied|persist` via `Scope::queryProperties`'s inScope branch (database.cc:1276-1281) -> `setVarnodeProperties` (funcdata_varnode.cc:25-36), so a global RAM store SURVIVES ActionDeadCode and the heritage multi-pass `collect` finally produces the `removevars` that fire `remove_revisited_markers`. Net: `revisit.xml` 2/9 -> 9/9, datatests 319 -> 326 (+7), zero regression.

## Reserved-file check (REJECT triggers) — ALL CLEAR
- architecture.rs IS touched, but ONLY in NEW code: the `decode_global` method (969-1014) + its call site (1968). The reserved `decode_pentry`/`decode_join_addr` arms (1633-1791) are byte-untouched. PERMITTED.
- coreaction_cleanup.rs / fspec.rs / coreaction_protos.rs / double.rs — NOT in the diff (4 files: architecture.rs, funcdata_printraw.rs, heritage.rs, tests/verify_w10_implied_vars_adversarial.rs).

## Faithfulness
- `decode_global` vs architecture.cc:816-848 (`decodeGlobal`+`addToGlobalScope`): faithful for `<range>`/`<register>` -> RangeProperties -> `symboltab.add_range(scope,spc,first,last)`. Empty `<range space="ram"/>` widens to `spc->getHighest()`. Ordering (after stackpointer/spacebase exist, before build_default_proto) matches the C++ deferred globalRanges apply. LOSSES: overlay duplication (838-846) + inferPtrSpaces push (836) not transcribed (LOSS-208).
- `remove_revisited_markers` vs heritage.cc:245-298: statement-faithful (INDIRECT/MULTIEQUAL/COPY arms; SUBPIECE rewrite tail; `++pos` via basic_neighbours; skip-trailing-phis while-loop; clearAddrForce; setWriteMask). Verified under LOSS-207, unchanged here. W8 warningHeader text suppressed (cosmetic; deadcode-delay bump still runs via `bump_deadcode_delay_seamed`, a real wrapper not a no-op).
- iop printRaw (funcdata_printraw.rs) vs op.cc:41-59 (`IopSpace::printRaw`): faithful non-branch/seqnum arm; branch arm is an `unreachable!` stub (unreached for INDIRECT iop input). Gated on IPTR_IOP space (raw-IR only); does not affect `print C` datatest output; B0 byte-equal confirms no regression.

## No special-casing — CLEAR
Zero symbol-name / address / magic-value literal in ANY production hunk. Space names read from cspec XML (`attr_str(child,"name")`, `RangeProperties::decode`); register names from XML. The `0x301020`/`dat_301020`/`glob2`/`revisit.xml` tokens appear ONLY in test assertions and doc-comments (production greps over architecture.rs/heritage.rs/funcdata_printraw.rs clean). The `"ram"` literals in heritage.rs are all inside `#[cfg(test)] mod tests` (boundary line 3389).

## Real passes (not fabricated)
Re-ran both base and branch fresh. Base rust-port @ 7a25277 = 319/675; branch = 326/675. Passing-SET diff (FAIL-set comm): branch FAIL set is a STRICT SUBSET of base FAIL set; the only delta is {Revisit SSA #1,#2,#3,#4,#5,#6,#7}. Branch full-run prints `Success -- Revisit SSA #1..#9` (revisit.xml 9/9, == C++ oracle 9/9; #8/#9 already passed on base => the LOSS-207 "2/9"). The gains correspond exactly to the new production code (decode_global seed -> persist -> remove_revisited_markers fires). REAL parity.

## No regression
- ZERO regressions: comm(base-FAIL, branch-FAIL) regression side EMPTY.
- boolless / readstruct / condconst_conn: absent from BOTH fail sets (pass on both; byte-identical at the assertion level).
- Else-if / No-for-loop alias (task-flagged): present in BOTH fail sets (unchanged, NOT regressed).
- C++ oracle 207/207 + 675/675 PARITY OK (C++ source + specs byte-identical base vs branch; rust-only diff).
- B0 byte-equal; action pass order unchanged.

## Hunt list
- Signedness: clean. `for &vn in remove` (no signed-index/size compare); register `last=(first-1)+size` via wrapping ops.
- Integer widths: clean. `newConstant(4,offset)`, `getIn(1)`, literals match C++.
- Wrapping: clean. `wrapping_sub(1).wrapping_add(size)` for register last (address.cc:244); the one bare `as` (`offset as uintb`, heritage.rs:294) carries its `// cast:` justification and the adversarial offset-widening test pins it.
- Comparator totality: n/a (none ported).
- Iteration order: clean. `to_add` in cspec child order; `remove` in vector order; MULTIEQUAL skip walks block list forward. NO HashMap/HashSet.
- Off-by-one/do-while/reverse: clean. MULTIEQUAL skip is a faithful while-not-do-while; `++pos` is basic_neighbours().1 (None==endOp).
- Erase-while-iterating: clean. iterates `std::mem::take`'d Vec; mutations touch fd's op graph not `remove`.
- Exception->Result: clean. `op_set_all_input` Result `.expect`ed; stale-handle `.expect`s panic where C++ derefs invalid pointers (UB).

## Mechanical pass
- grep added lines for todo!/unimplemented!/HashMap/HashSet/sort_unstable/partial_cmp.unwrap: NONE.
- bare `as` in added production lines: exactly one (`offset as uintb`), justified.
- clippy -p kuna-decomp --lib: clean.

## Adversarial tests (porter-committed, verified passing; exercise the production code directly since the corpus path was dormant before this seed)
- heritage::tests::w10_revisit_remove_markers_copy_is_unlinked
- heritage::tests::w10_revisit_remove_markers_multiequal_rewritten_after_phis
- heritage::tests::w10_revisit_remove_markers_indirect_inserts_after_live_target
- heritage::tests::w10_revisit_remove_markers_offset_widening_le
Plus the rewritten verify_w10_implied_vars_adversarial::w10_implied_multiwrite_return_stays_explicit_not_overinlined now asserting the faithful persistent-global form `dat_301020 = a0;` (anti-DCE/anti-over-inline guard) — verified rendering matches.

## Findings
- F1 (loss, accepted): overlay-space duplication + inferPtrSpaces push not transcribed in `decode_global`. cpp: architecture.cc:836,838-846. rust: architecture.rs:969-1014. -> LOSS-208. Unobservable on the corpus (no overlay-base global; inferPtrSpaces is a separate seam).
- F2 (loss, accepted): W8 `warningHeader` revisit text suppressed in co-landed `remove_revisited_markers`. cpp: heritage.cc:253-257. rust: heritage.rs:1764-1771. -> LOSS-208/LOSS-207. Cosmetic; deadcode-delay bump still runs.

losses: LOSS-208 (this wave's deferrals), LOSS-207 (the co-landed remove_revisited_markers W8 text + the now-superseded "dormant" note).
