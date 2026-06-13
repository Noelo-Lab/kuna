# w8-fw-universalaction
verdict: ACCEPT-WITH-LOSSES
verifier: claude-opus-4-8 (independent verifier, round 1)
date: 2026-06-13
gate: `cd rust && cargo test -p kuna-decomp --test universalaction_listing` -> ok (1 passed); adversarial `--test verify_w8_fw_universalaction` -> ok (5 passed)

## Scope

C++ in scope (blob-sha verified, no drift): `decompiler/cpp/coreaction.cc`@`28f6157f`
(`ActionDatabase::universalAction` cc:5722-6032 + `buildDefaultGroups` cc:5675-5718)
and `decompiler/cpp/coreaction.hh`@`bc278439`. Supporting C++ re-read for the
listing/clone semantics: `action.cc` `Action::print` (132), `ActionGroup::print`
(428), `ActionPool::print` (753), `ActionGroup/RestartGroup/Pool::clone(grouplist)`
(391/529/899), `action.hh` `ActionGroupList::contains` (39) + `breakflags` (73-78).

Rust diff: `rust/crates/kuna-decomp/src/universalaction.rs` (commit cb00d90), plus the
gate `tests/universalaction_listing.rs` + fixture
`tests/fixtures/list_action_decompile_oracle.txt`.

## What this item is

A declarative (`SchedNode`) transcription of the imperative C++ `universalAction`
constructor, materialized into the engine Action/Rule tree, plus a `list action`-style
listing renderer that ports C++ `Action::print`/`ActionGroup::print`/`ActionPool::print`
filtered through the group set (porting C++ `clone(grouplist)` + print). The keystone
property under review is **output determinism**: the EXACT action/rule registration
order and the listing format.

## Re-derivation (this wave's required hunts)

- **Action/Rule registration order** — re-derived position-by-position. Extracted the
  full 252-entry ordered `(kind, class, group)` sequence from the C++ constructor and the
  flattened sequence from the Rust `SchedNode` tree (oppool1=138 rules, oppool2=5,
  cleanup=16); after removing the 10 allowlisted-unported classes the **242-entry
  group-arg sequence matches the C++ order exactly** (mechanical diff, zero mismatches,
  equal length). The commented-out upstream `ActionParamShiftStart`/`Stop` (cc:5741,5765)
  and `RuleIndirectConcat` (cc:5938) are correctly absent. Container nesting/flags
  (`rule_repeatapply` on fullloop/mainloop/stackstall/oppool1/oppool2/cleanup,
  `rule_onceperfunc` + `max=1` only on the universal restart group) match cc:5734/5747/
  5749/5773/5775/5933/5972.
- **Listing format (`Action::print`)** — `print_action_line` matches cc:135-143 column-for-
  column: `{:>4}` index, `" repeat "`/8-blank, `!`/`S`/`A` flag columns (breakflags
  1|2 -> 'S', 4|8 -> 'A', matching action.hh:74-77), `depth*5+2` indent, name. Pool rule
  line matches cc:765-771 (`D`/`A` + `depth*5+2` indent). The clone-then-print newline
  discipline (the `s<<endl` after EVERY surviving child) is reproduced: a chain of
  last-child containers emits *stacked* blank lines — hand-derived for the `{base}` root
  and pinned byte-exact (see adversarial test 1).
- **clone(grouplist) drop semantics** — `SchedNode::survives` mirrors the engine
  `clone_filtered` family: leaf survives iff `grouplist.contains(group)` (each leaf's own
  `clone_filtered`), Pool iff >=1 rule survives (cc:899-913 / action.rs:1222), Group/
  RestartGroup iff >=1 child survives (cc:391-405/529-543 / action.rs:594-617). Verified
  the listing-path `survives` agrees with the engine-path `clone_filtered` for the
  `decompile`, `firstpass`, `register`, and all-groups filters.
- **PrintC token order / cfmt number formatting / CastStrategy** — N/A to this item (no
  C-emission, number formatting, or cast logic here; those belong to the printc/cast items
  already reviewed). The "exact registration order" hunt is the relevant one and is clean.

## Mandatory hunt list

- Signedness: clean. Only `int4` (i32) arithmetic: `num+1` (per-line index, ~250 max) and
  `depth*5+2` (indent, max depth 4). No int4-vs-uint comparisons; no loop index vs
  container size. Mirrors C++ `int4`.
- Integer widths: clean. `num`/`depth` are `int4`; no promotion/width games.
- Wrapping: clean. No `uintb`-lineage arithmetic; the two i32 ops cannot overflow at this
  scale (and match C++ plain `int4` add/mul).
- Comparator totality: clean. No comparators. The filter is a `BTreeSet<String>`
  (membership only, total string order) = C++ `std::set<string>` `ActionGroupList`.
- Iteration-order provenance: clean. Every traversal is over a `Vec` in registration order
  (= C++ `vector` insertion order). The only `BTreeSet` is the group filter, used for
  membership, order-irrelevant. NO `HashMap`/`HashSet` in ported logic.
- Off-by-one / do-while / reverse iterators: clean. `for` loops match C++ `for(iter...)`;
  indent loop `0..(depth*5+2)` == `for(i=0;i<depth*5+2;++i)`. No do-while, no rbegin.
- Erase-while-iterating: clean. Construction-only; no mutation during traversal.
- Exception -> Result partial-state: clean. `materialize`/`list_action_dump` are
  infallible; no `?`/throw in the hot path.

## Mechanical pass

- grep: NO `HashMap`/`HashSet`/`todo!`/`unimplemented!`/bare `as`/`partial_cmp`/`.unwrap()`
  in `universalaction.rs`. (One `sort_unstable()` in the module's own `tests.rs`, sorting a
  name set for an order-independent comparison — benign.)
- clippy: `cargo clippy -p kuna-decomp --lib` -> 0 warnings (the library, which contains
  this item, is clean). The crate's `--tests` clippy error is in `heritage.rs:1602`
  (a different, untouched W5 item's `#[cfg(test)]` code) — pre-existing, out of scope.
- port-audit: not present in this worktree (tool not yet built); blob-sha drift checked by
  hand instead — both recorded `cpp_blob_sha` values match the worktree blobs exactly.

## Findings

None blocking. One observation (out of scope, belongs to `action.rs`/`ruleaction_*`):
zero-sized marker rules whose `clone_rule` returns `Some` unconditionally (e.g.
`RuleSborrow`, group `analysis`) would not be group-filtered in the *engine* materialize
path; harmless for every root here because all such rules are group `analysis` (present in
every root that has rules) — and the *listing* path (this item) filters correctly on the
`RuleRow` group. Not a defect of `universalaction.rs`.

## Losses

- LOSS-119 (this item): the materialized tree omits 10 not-yet-ported passes
  (`UNPORTED_ALLOWLIST`) — the assembly-time manifestation of pre-recorded dependency-item
  losses (LOSS-088 stringcopy/stringstore, LOSS-110 lowerswitchinstall, and the W5/W6
  subflow/cleanup leaf-rule deferrals for splitflow/subfloat_convert/stackprobeloop/
  dumptyhumplate/splitcopy/splitload/splitstore). Everything present is in the exact C++
  order with the exact group strings and listing format. The B0 gate byte-matches a genuine
  `decomp_dbg` oracle modulo exactly these 10, and fails on a stale/empty allowlist.

## Adversarial tests (added, committed in worktree aaed67b)

`rust/crates/kuna-decomp/tests/verify_w8_fw_universalaction.rs` (5 tests, all green):
1. `..firstpass_drop_and_stacked_blanks_match_cpp` — `{base}` root: byte-exact hand-derived
   dump, pinning both the pool/group whole-drop and the 3-stacked-blank newline tail the
   decompile oracle never produces.
2. `..empty_or_unmatched_filter_drops_whole_tree` — empty + unmatched filter -> "" (root
   `survives`==false / C++ clone -> NULL).
3. `..allgroups_full_order_count_head_tail` — every group enabled: 249 non-blank lines
   (242 leaves + 7 containers), exact head (incl. `normalizesetup`/`funclink_outonly` that
   decompile drops) and tail, all 4 `directwrite` registrations present.
4. `..allowlist_is_unique_and_correctly_grouped` — allowlist names unique and each group ==
   its C++ universalAction group (guards the renumber-gate's silent trust).
5. `..register_root_drops_cleanup_pool_and_keeps_oppool_subset` — `{base,analysis,
   canonicalcompare,subvar}`: cleanup pool + oppool2 + floatprecision/segment/doubleload
   dropped; analysis/canonical/subvar oppool1 rules kept.

## Verdict

ACCEPT-WITH-LOSSES. The keystone schedule is a faithful, order-exact transcription of the
C++ `universalAction` constructor; the listing renderer faithfully ports the C++
clone+print (order, format, and the stacked-blank newline discipline). The only divergence
is the documented, gated, pre-recorded 10-pass allowlist (LOSS-119), which introduces no
new behavior gap of this item's own — each omission is owned and lossed by a dependency
item with a "wire into W8 universalAction" restoration criterion.
