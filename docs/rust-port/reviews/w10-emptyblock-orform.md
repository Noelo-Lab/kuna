# w10-emptyblock-orform
verdict: ACCEPT
verifier: claude-opus-4-8[1m] (independent verifier)
date: 2026-06-15
round: 1

gate:
  - `cargo test --workspace` (debug) -> GREEN (lib 1664 passed incl. 3 verifier-added; all integration suites pass; the single release-mode `constseq::w6s5_form_byte_array_huge_bytepos_diverges_debug_panic` "failure" is a pre-existing debug-vs-release `#[should_panic(expected="add with overflow")]` artifact in an UNRELATED W6/S5 item, untouched by this diff, and PASSES in debug — the canonical mode).
  - `cargo test -p kuna-decomp --test verify_w10_emptyblock_orform` -> 3/3 PASS (splice, `&&` byte-identical, boolless-not-perturbed).
  - C++ oracle `decomp_test_dbg` full datatests -> **675/675 PASS** (PARITY OK, untouched).
  - Rust vs C++ engine byte-diff: `condconst_conn` `&&` form, `boolless`, `readstruct` all **BYTE-IDENTICAL**.
  - lift-diff `--self-test` -> 8/15 PASS, identical to the `rust-port` base (NO execution-failure regression; `condconst` fixture PASS; the 7 DIVERGENT are pre-existing W4 ActionFuncLink call-placeholder cases).
  - `cargo clippy -p kuna-decomp -- -D warnings` -> CLEAN (no warning in any of the 4 touched files).

## What the item is

A genuine RESTORATION that CLOSES two previously-documented losses:
  - **LOSS-101** — `blockaction::is_complex` was the `FlowBlock` base default (`true`),
    forcing `ruleBlockOr` to always `continue` and `ruleBlockWhileDo` to always take
    overflow syntax. Now ports `BlockBasic::isComplex` (op-list statement count vs
    `max_implied_ref`) routed through the `BlockCopy.copy` pointer.
  - **LOSS-078 / funcdata_block.cc seams** — `ActionDoNothing`/`ActionLateDoNothing`
    were no-ops past the `clearDelayedDonothing` sweep because `isDoNothing`/
    `unblockedMulti`/`hasNoImmediateCopy` predicates and the `removeDoNothingBlock`/
    `blockRemoveInternal`/`pushMultiequals`/`createReplaceVarnode`/`replaceInHigh`
    removal chain were unported. All now transcribed.

The `&&` form is REAL: produced by genuine `CollapseStructure` over the cleaned CFG;
the Rust engine output for `condconst_conn` byte-matches the C++ oracle (`if ((x == 0)
&& (y != 10)) {`). The empty then/else blocks are spliced out by `removeDoNothingBlock`
only after the merge/dominant-copy emptied them; a non-empty / non-mergeable block
cannot reach the splice (`is_do_nothing` requires `hasOnlyMarkers`, and
`block_remove_internal` hard-errors on a CALL or an assignment with descendants
outside the block — so over-splicing is structurally impossible, verified by the guards
at funcdata_block.rs:551-563).

## Hunt list

- **Signedness**: clean. `bb_is_complex` `statement > 2`, `totalref > maxref` are
  `int4`-vs-`int4` matching C++; `get_in_index`/`get_out_rev_index` return `int4` slot
  indices used as loop bounds, no signed/unsigned cross-compare introduced.
- **Integer widths**: clean. `statement`/`maxref`/`totalref` are `int4`, mirroring
  `BlockBasic::isComplex`. No promotion divergence.
- **Wrapping**: clean. No `uintb`-lineage arithmetic in the ported bodies; the only
  arithmetic is `statement += 1` / `totalref += 1` on small `int4` counters bounded by
  the (short, by precondition) block op count — cannot wrap.
- **Comparator totality**: n/a — no new comparator; `HighVariable::insert` reuses the
  existing `compare_just_loc` (loc-keyed binary search, total order, unchanged).
- **Iteration-order provenance**: clean.
    - `bb_ops`/`bb_is_complex`/`hasOnlyMarkers`/`unblockedMulti`/`pushMultiequals` walk
      the intrusive op list head→next == C++ `op.begin()..op.end()` insertion order.
    - `descend_iter` is a `SmallVec` (push_back == C++ `list<PcodeOp*> descend`); both
      the read-only descend scan and the snapshot preserve order.
    - `complex_blocks` is a `BTreeSet<BlockId>` (ordered; membership test only — order
      irrelevant). NO `HashMap`/`HashSet` in any ported logic.
- **Off-by-one / do-while / reverse iterators**: clean.
    - `ruleBlockOr` `j` clause-match loop keeps the `j==2` not-found sentinel and the
      `getOut(1-j)` no-loop guard.
    - `collapse_conditions` re-reads `self.size()` per step via `while i < self.size()`
      (the C++ `graph.getSize()` re-eval) — correct, since `newBlockCondition` shrinks
      the component list.
    - `bb_is_complex` `statement > 2` boundary verified at the exact threshold by VF1.
- **Erase-while-iterating**: clean and FAITHFUL.
    - `block_remove_internal` op-destroy loop iterates a `bb_ops` Vec SNAPSHOT (immune
      to `op_destroy`'s list splice), reproducing C++ `iter++` before `opDestroy`.
    - `push_multiequals` descenders loop: the C++ `*titer++` (advance before
      `opSetInput` invalidates the node) is reproduced by a `.collect()` snapshot taken
      AFTER `replaceop` is built (so `replaceop` is in the snapshot, exactly as in C++
      where it is in `origvn->descend` when iteration begins); the per-op first-matching
      `break` and the `outblock_ind && MULTIEQUAL && parent==outblock` skip match C++.
- **Exception -> Result partial-state parity**: clean. The two `Err` arms in
  `block_remove_internal` (unreachable-`descend2Undef` SEAM, CALL-`deleteCallSpecs`
  SEAM) are GUARDS on paths the `removeDoNothingBlock` caller (`unreachable=false`,
  marker-only block) provably never reaches — they fail loud rather than silently
  drop, matching the C++ `throw LowlevelError("Deleting op with descendants")` site
  for the descendant case.

## Faithfulness anchors verified

- `is_complex` virtual chain: `FlowBlock::isComplex`(base `true`) /
  `BlockBasic::isComplex`(count) / `BlockCopy::isComplex`(`copy->isComplex()`) /
  `BlockList::isComplex`(`getBlock(0)->isComplex()`) — reproduced by
  `get_front_leaf` (descends `subBlock(0)` to the `t_copy` leaf) → `get_copy()` →
  `complex_blocks` lookup, with the `None`→`true` base fallback. cpp block.cc:340/2403,
  block.hh:254/503/549/649 → rust blockaction.rs:1946-1965, block.rs:1229/485.
- `complex_blocks` keying: `build_copy_from` stamps each BlockCopy with
  `copy: Some(<bblocks id>)`; the precompute in `ActionBlockStructure::apply` keys by
  the same bblocks ids. Precompute-once is valid because `isComplex` reads only
  op-list contents + out-degree, immutable during structuring (which mutates only the
  sblocks mirror). cpp blockaction.cc → rust blockaction.rs:3314-3331.
- `isDoNothing`/`hasOnlyMarkers`/`unblockedMulti`/`hasNoImmediateCopy`: line-for-line.
  cpp block.cc:2561/2605/2626/2644 → rust funcdata_block.rs:31-251.
- `removeDoNothingBlock`/`blockRemoveInternal`/`pushMultiequals`/`opZeroMulti`/
  `createReplaceVarnode`/`replaceInHigh`: faithful. cpp funcdata_block.cc:86-354,
  varnode.cc:353, variable.cc:515-545 → rust funcdata_block.rs:430-586, variable.rs:1381.
- `ruleBlockOr` && precondition checks `orblock->isComplex()` (active C++ line 1342),
  NOT `bl->isComplex()` (commented out at 1333); `ruleBlockWhileDo` overflow checks
  `bl->isComplex()` (1538). Both correct. The && vs || is the duality from the
  `i==1`/`j==0` `negateCondition` arms, faithfully ported.
- `ActionDoNothing`/`ActionLateDoNothing` arms (infinite-loop mark, `unblockedMulti`,
  `isNormalizationOn || hasNoImmediateCopy`, `setDelayedDonothing`,
  `removingCreatesRedundancy`, removeList) all transcribed. cpp coreaction.cc:3569-3648.

findings:
  - F1 (minor / doc-protocol, NOT a blocker): a bare `as` cast without the
    protocol-required `// cast:` justification.
       cpp: decompiler/cpp/funcdata_block.cc:167 (`newOp(branches.size(), ...)`)
       rust: rust/crates/kuna-decomp/src/funcdata_block.rs:573 (`branches.len() as int4`)
       Semantically SAFE: `branches.len()` == outblock in-degree, a small bounded count;
       cannot truncate `int4`. Flagged only for the missing comment per the verifier
       protocol's bare-`as` rule. Does not affect output; not grounds for REJECT.
  - N1 (analyzed, NOT a finding): `ActionDoNothing::apply` is refactored from C++'s
    single interleaved loop into two loops (clear-all-then-detect). Observationally
    EQUIVALENT at the only consumer (`ActionLateDoNothing`): `ActionDoNothing` runs to
    fixpoint under `rule_repeatapply`, and the final no-removal pass fully traverses
    every block in BOTH engines, normalizing the `delayed_donothing` flag identically
    before `ActionLateDoNothing` ever reads it. The predicates do not read the flag, so
    the intra-call ordering is irrelevant. Confirmed by 675/675 parity + byte-identical
    condconst/boolless/readstruct.
  - N2 (analyzed, NOT a finding): `replace_in_high` re-ORs `flagsdirty|namerepdirty|
    coverdirty|typedirty` onto `replaceHigh` where C++ `replaceHigh->inst[0]=this` sets
    no flags. Redundant no-op: `replacevn`'s freshly-created `HighVariable::new` already
    sets exactly those 4 flags, so the OR is idempotent. Equivalent.

adversarial tests (committed to rport/w10-emptyblock-orform @ e169824,
  rust/crates/kuna-decomp/src/funcdata_block.rs):
  - vf1_bb_is_complex_statement_count_boundary  (statement > 2 off-by-one, single-out)
  - vf2_bb_is_complex_branch_seed_and_marker_skip  (sizeOut>=2 seed=1; MULTIEQUAL skip)
  - vf3_bb_unblocked_multi_rejects_inconsistent_redundant_branch  (the vnremove!=vnredund deref)
  All 3 PASS. Porter's 6 unit + 3 integration tests also re-run GREEN.

losses: none introduced. This item CLOSES LOSS-078 (the do-nothing/removal seams) and
  LOSS-101 (the isComplex base-default seam) — their restoration criteria are met
  (BlockBasic::isComplex ported and routed through BlockCopy; removeDoNothingBlock +
  predicates wired into ActionDoNothing/ActionLateDoNothing with positive end-to-end
  byte-parity tests). No new losses.md entry required (F1 is a doc-comment nit, not a
  behavioral divergence).
