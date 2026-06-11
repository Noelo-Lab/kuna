# w4-fw-action
verdict: ACCEPT-WITH-LOSSES
verifier: independent verifier (round 1)
date: 2026-06-11
gate: `cd rust && cargo test -p kuna-decomp` -> PASS (13 test binaries, 328 tests, 0 failed, including the 6 new verifier tests)

C++ in scope: decompiler/cpp/action.cc, action.hh (cross-checked coreaction.cc:5675
`buildDefaultGroups`, funcdata.hh:485 `opDeadAndGone`, architecture.cc:336 `clearAnalysis`).
Rust: rust/crates/kuna-decomp/src/action.rs (+ src/action/tests.rs).

## Hunt list
- **Signedness**: clean. The container index loops (`ActionGroup::apply` `state`,
  `ActionPool` `rule_index`) are `usize` against `.len()`/`perop_len`, matching the C++
  `int4`-vs-`size()` comparisons whose only values are non-negative. `ActionRestartGroup`
  `curstart`/`maxrestarts` are `int4` (i32) including the `-1` "completed" sentinel and the
  `curstart > maxrestarts` cap — faithful to C++ `int4`. No int4-vs-uint4 comparison ported
  as a silent `-Wno-sign-compare` case.
- **Integer widths**: clean. int4=i32/uint4=u32/uintb=u64 per ADR 0003. The flag words
  (`ruleflags`/`statusflags`/`breakflags`/`typeflags`) are `uint4` bitsets transcribed
  verbatim with the upstream numeric values (action.hh:55-78,197-202). `OpCode::CPUI_MAX as i32`
  (action.rs:1332) is an enum-bound, not a truncation.
- **Wrapping**: clean. `base.count += res` / `count += res` mirror the C++ `int4 +=`; neither
  side guards overflow (astronomically many changes). No `uintb`-lineage arithmetic in this
  module. `create_at`'s `uniqid.wadd(1)` lives in the already-ported op.rs, out of scope.
- **Comparator totality**: clean. The only ordering used is `SeqNum` (`(pc, uniq)` total order,
  address.rs:674-684) for the optree cursor; no `operator<` is defined in this item.
- **Iteration-order provenance** (per loop):
  - `ActionGroup::{apply,clone_children,reset,reset_stats,clear_break_points,with_sub_*}`
    iterate `Vec<Box<dyn Action>> list` in insertion order == C++ `vector<Action*>` order. OK.
  - `ActionPool::{add_rule,clone_filtered,reset,reset_stats,clear_break_points,with_sub_rule}`
    iterate `Vec<RuleEntry> allrules` in insertion order == C++ `vector<Rule*>`. OK.
  - `ActionPool::perop` is `BTreeMap<OpCode, SmallVec<usize>>`; the per-opcode list is built in
    rule-registration order and indexed positionally in `process_op` — the BTreeMap key set is
    never iterated (only `get(&opc)`), so its order is irrelevant. The C++ `perop[CPUI_MAX]` is an
    array; the BTreeMap is a sparse equivalent. Dead slots 0/45 (BLANK/UNUSED1) are correctly
    skipped (`all_opcodes`/`from_i32`) because no PcodeOp carries those opcodes. OK.
  - `ActionGroupList` is `BTreeMap<String,()>` — only `contains` is queried (C++ `set<string>`
    membership), iteration order never used. ADR 0002 (no HashSet) honored.
  - `ActionDatabase::{groupmap,actionmap}` are `BTreeMap` (C++ `map`); only keyed lookup is used.
  - Optree traversal in `ActionPool::apply` uses `iter_all()` (BTreeMap by SeqNum) == C++
    `beginOpAll`..`endOpAll`. OK.
  No `HashMap`/`HashSet` in ported logic (grep clean).
- **Off-by-one / do-while / reverse iterators**: clean. `Action::perform`'s C++ `do {} while`
  is ported as `loop { ... if !cond break; }` preserving the at-least-once semantics and the
  switch fall-through (status_start->breakstarthit->repeat->mid) is reproduced by cumulative
  `if st == ... || ...` conditions (action.rs:476-519). `status_actionbreak`/`status_end`
  branches match (no re-apply). No reverse iteration in scope.
- **Erase-while-iterating**: VERIFIED faithful. `process_op` dead path does
  `advance_op_state(op)` (reads op's live SeqNum, computes next key) BEFORE
  `obank.destroy(op)` — exactly the C++ `op_state++; opDeadAndGone(op)` order (action.cc:830-831).
  The key-based cursor (`After(sq)` -> "first key > sq") survives the erase because the erased
  key equals the recorded sq. Confirmed by new test
  `w4_fw_action_consecutive_dead_ops_at_head_all_destroyed_live_visited_once` (a run of dead ops
  at the head, each destroyed, live op visited once).
- **Exception->Result partial-state parity**: clean for in-scope `?` sites. `get_group`/
  `get_action`/`derive_action` return `KunaResult` mirroring the C++ `throw LowlevelError`;
  these are pure lookups with no preceding mutation, so there is no partial state to match. The
  `process_op` invariant-violation (C++ `glb->printMessage` "Rule changed op without returning
  result of 1!") is routed to the WarningSink (`pending_error`) and continues, matching the C++
  which also continues after printing. The `ActionRestartGroup` restart uses `data.clear()` for
  `clearAnalysis` minus the comment-DB clear (see F2/LOSS-043).

## Mechanical pass
- grep `todo!`/`unimplemented!`/`HashMap`/`HashSet`/`sort_unstable`: NONE in src/action.rs.
- bare `as` casts: only trait-object coercions (`as Box<dyn Action>`) and `OpCode::CPUI_MAX as i32`
  (enum bound) — no numeric-truncation casts.
- `cargo clippy -p kuna-decomp -- -D warnings`: clean.
- `cargo run -p port-audit -- --item w4-fw-action`: N/A — `port-audit` is not a member of the
  workspace at this branch; blob-sha drift check could not be run.

## Findings
- F1 (minor / accepted loss): breakpoint-resume key-cursor diverges from the C++ live map
  iterator when a rule inserts an op BELOW an `Unstarted` cursor before an action breakpoint —
  the broken op's rules are re-applied on resume. Debug-breakpoint-only; never reaches produced
  decompilation. See LOSS-042.
       cpp: decompiler/cpp/action.cc:822-888
       rust: rust/crates/kuna-decomp/src/action.rs:1038-1047,1174-1200,1260-1275
- F2 (minor / accepted seam): `ActionRestartGroup` restart calls `data.clear()` only; the C++
  `clearAnalysis` additionally clears warning/warningheader comments. Comment DB is a W4 seam,
  absent here. See LOSS-043.
       cpp: decompiler/cpp/action.cc:574 (architecture.cc:336)
       rust: rust/crates/kuna-decomp/src/action.rs:836-838
- F3 (minor / accepted divergence): `deriveAction` substitutes an empty no-op `ActionGroup` for
  a fully-filtered (C++ NULL) root; `get_current` is infallible instead of returning NULL. No
  in-scope consumer distinguishes them; `derive_fresh` still exposes the faithful `None`. See
  LOSS-044.
       cpp: decompiler/cpp/action.cc:1145-1160
       rust: rust/crates/kuna-decomp/src/action.rs:1502-1518

No blockers or majors. The load-bearing transcriptions verified line-by-line: `Action::perform`
state machine (action.cc:298-362), `ActionGroup::apply` (:506), `ActionRestartGroup::apply`
(:553), `ActionPool::processOp` (:822) incl. the opcode-change `rule_index`-to-0 re-zero and the
dead-op skip/destroy, `ActionPool::apply` (:877), the `getSubAction`/`getSubRule` `:`-path
ambiguity search (:456,:481,:789), and `buildDefaultGroups` member lists (coreaction.cc:5675,
incl. the kuna `canonicalcompare`/`presentcompare` split).

## Adversarial tests
(rust/crates/kuna-decomp/tests/verify_w4_fw_action.rs, committed on the branch)
- w4_fw_action_consecutive_dead_ops_at_head_all_destroyed_live_visited_once (erase-while-iterating)
- w4_fw_action_rezero_reoffers_disabled_rule_without_applying_it (opcode-change re-zero + disabled rule)
- w4_fw_action_getsubaction_ambiguous_children_report_not_found (matchcount>1 totality)
- w4_fw_action_breakpoint_resumes_same_op_from_preserved_rule_index (rule_index preserved across break)
- w4_fw_action_empty_grouplist_derives_a_noop_root_not_a_crash (empty-group derive, F3)
- w4_fw_action_cursor_jumps_to_op_inserted_below_after_break_documents_divergence (F1, pinned)
All 6 pass.

## Losses
LOSS-042 (breakpoint-resume cursor below-insert), LOSS-043 (restart comment-DB clear seam),
LOSS-044 (empty-group NULL->empty substitution).
