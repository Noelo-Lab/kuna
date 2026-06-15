# w10-global-persist
verdict: ACCEPT-WITH-LOSSES
verifier: independent (Opus 4.8)
date: 2026-06-15
round: 2

gate: cargo test --workspace -> green (incl. the 5 GlobalQuery adversarial
      tests); clippy -p kuna-decomp --lib -D warnings -> clean.
      Rust XML datatest harness (decomp_test_dbg, full corpus) ->
        base (merge-base 5e63e22): 668 applied / 194 passing / 2 exec-failures
        branch (f4ea4a6):          672 applied / 195 passing / 1 exec-failure
        passing SET diff base->branch: regressions EMPTY; gained exactly
          `Double precision move #6` (STRICT SUPERSET).
      C++ oracle (decomp_test_dbg, decompiler/cpp): 207/207 unit + 675/675
      datatest, PARITY OK (UNTOUCHED — the diff is rust-only; zero C++/specs files).

## Summary

ROUND-2 REPAIR of the round-1 REJECT (F1: passing-SET regression). The repair
DEFERS the `set_varnode_properties` early persist/addrtied marking (returns it to
the W3-base no-op, with the faithful C++ body preserved in the doc and the call
site retained), keeping the global-store SURVIVAL on the genuinely-faithful
heritage path (`Heritage::guard` -> `query_global_properties` -> `guard_returns`
`addrforce` RETURN-COPY). Independently re-run and CONFIRMED:

  - the round-1 regressor `varcross.xml::Global cross #2` is Success on BOTH base
    and branch (regression GONE);
  - the branch datatest passing SET is a STRICT SUPERSET of base (regressions
    EMPTY, +1 `Double precision move #6`) — not a net-zero count masking a
    swap, the exact round-1 failure condition is now cleared;
  - exec-failures 2 -> 1: `displayformat.xml` no longer hits "Requested varnode
    does not exist" / "Did not apply" (the global store now survives so the
    `force varnode #0xaa` constant exists) — it DECODES + APPLIES; its 4
    `Display Format #N` assertions APPLY but still FAIL honestly (the global
    symbol-NAME render is the reserved printc fleet — no fabricated pass).

The transcription is FAITHFUL throughout: `GlobalQuery::query_properties`
(global-scope `Scope::queryProperties` / `findContainer` / `SymbolEntry::inUse`),
the heritage persist RETURN-COPY (heritage.cc:1677-1692 line-for-line), and
`ActionDirectWrite::apply` (coreaction.cc:1366 line-for-line) all match their C++
sources. No special-casing, no reserved-file edits, clippy clean, 5 adversarial
tests committed. Two divergences (the deferred early marking; the `has_store()`
input-param guard) are documented as cited losses (LOSS-190, LOSS-191) — hence
ACCEPT-WITH-LOSSES rather than ACCEPT.

## Mandatory hunt list

- Signedness: clean. `find_container_flags` compares `e.space_index != space_index`
  (both int4 after explicit `get_index`), `e.first > start`/`e.last < end` (u64),
  `e.size < oldsize`/`oldsize == -1`/`e.size == size` (int4) — every operand pair
  is same-typed; no silent C++ signed/unsigned comparison carried over.
- Integer widths: clean. `end = addr.getOffset() + size - 1` reproduced as
  `start.wrapping_add(size as u64).wrapping_sub(1)`, matching the C++ `uintb`
  widening of the non-negative `int4 size`; `getAllFlags = extraflags | getFlags`
  reproduced exactly.
- Wrapping: clean (pinned by av5). The only unsigned arithmetic (`end = start +
  size - 1`) uses `wrapping_*`; no panic near u64::MAX.
- Comparator totality: n/a — no new operator</sort; the rangemap subsort is a
  pre-existing BTreeMap key, untouched.
- Iteration-order provenance: clean. `directwrite_apply` seeds over
  `vbank.iter_loc()` (BTreeMap<LocKey> = C++ VarnodeLocSet order) and walks
  `descend_iter()` (Vec = C++ `list<PcodeOp*>` insertion order); the taint is a
  fixpoint so order-independent anyway. `guard_returns` uses
  `iter_code(CPUI_RETURN)` (returnlist list order = C++ beginOp..endOp).
  `build_global_query` walks `rangemap.records()` (BTreeMap key order). No
  HashMap/HashSet anywhere.
- Off-by-one / do-while / reverse iterators: clean (av1). C++ findContainer's
  rangemap query guarantees `first<=addr` then checks `getLast()>=end`; the port
  makes both bounds explicit (`e.first>start || e.last<end` skip). The C++
  reverse `--res.second` vs the port's forward linear scan differ in order, but
  selection is by smallest-size + inUse (a min, order-independent) so equal-flag
  containers resolve identically; the only theoretical divergence is two
  EQUAL-size containers with DIFFERENT flags at the same addr (not constructible
  for global mapped storage) — noted, not a finding.
- Erase-while-iterating: clean. Every added loop collects to a Vec first
  (`iter_loc().collect()`, `iter_code(...).collect()`, `descend_iter().collect()`)
  then mutates — no iterator invalidation; the RETURN-COPYs are inserted BEFORE
  each return (not new returns), invisible to even a live C++ iterator.
- Exception -> Result partial-state: minor, non-finding. `guard_returns` /
  `new_varnode_out` use `Err(_) => continue` on `new_varnode_out`/`create_def`,
  leaving an orphan `new_op` on a path C++ never errors on; defensive only,
  unreachable in this flow.

## Findings

- F1 (RESOLVED — was round-1 blocker): the passing-SET regression on
  `varcross.xml::Global cross #2` is REPAIRED. Independently re-run: `Global
  cross #2` is `Success` on BOTH base (merge-base 5e63e22) and branch
  (f4ea4a6); the full passing-SET diff (base->branch) has EMPTY regressions and
  gains exactly `Double precision move #6`. Root cause of the round-1 regression
  (the early `set_varnode_properties` persist marking perturbing the un-landed
  HighVariable merge) is removed by deferring that marking. Config matrix
  (porter, confirmed consistent with my superset re-run):
        SVP on,  HFL on, DW on  -> 194, REGRESSES Global cross #2 (round-1 branch)
        SVP off, HFL on, DW on  -> 195, STRICT SUPERSET, +Double precision move #6 (this branch)
  cpp: decompiler/cpp/funcdata_varnode.cc:25, decompiler/cpp/heritage.cc:1192
  rust: rust/crates/kuna-decomp/src/funcdata.rs (set_varnode_properties no-op),
        rust/crates/kuna-decomp/src/heritage.rs (guard fl from query_global_properties)

- F2 (note, now cited LOSS-191): `directwrite_apply`'s input-param seed gates on
  `getFuncProto().has_store() && possible_input_param(...)`, an extra condition
  C++ `possibleInputParam` lacks (C++ always reaches the model). Defensive
  W4-store seam-guard, not a behavioral special-case; unobservable on the
  corpus. Documented as LOSS-191.
  cpp: decompiler/cpp/coreaction.cc:1384
  rust: rust/crates/kuna-decomp/src/coreaction_render.rs (directwrite_apply has_store guard)

- F3 (RESOLVED): the two bare `as` casts now carry `// cast:` justification
  comments (database.rs space_index usize->int4; seams.rs size int4->u64). The
  third bare cast (`(last-first+1) as i32`) is in the verifier's own adversarial
  test file, not production code.
  rust: rust/crates/kuna-decomp/src/database.rs (commented),
        rust/crates/kuna-decomp/src/seams.rs (commented)

- F4 (cited LOSS-190): `set_varnode_properties` is now a documented no-op (the
  C++ `setSymbolProperties`->`updateType`/`mapentry`/`setFlags(getAllFlags()&
  ~typelock)` and the no-entry `setFlags(vflags&~typelock)` are both deferred).
  Faithful for the item's dataflow target (the persist RETURN-COPY comes from the
  heritage query in C++, not from this marking); the early marking + the
  type-lock surface are render-fleet-dependent. Documented as LOSS-190.
  cpp: decompiler/cpp/funcdata_varnode.cc:25, decompiler/cpp/varnode.cc:429
  rust: rust/crates/kuna-decomp/src/funcdata.rs (set_varnode_properties)

## Reserved-file check

PASS. The diff (vs the true merge-base 5e63e22) touches only: architecture.rs,
coreaction_render.rs, database.rs, funcdata.rs, heritage.rs, ruleaction_4.rs,
seams.rs (+ the verifier adversarial test). NONE of merge.rs, funcdata_merge.rs,
variable.rs, varmap.rs, printc.rs, cast.rs, coreaction_infertypes.rs,
coreaction_protos.rs, dtype.rs, fspec.rs is edited.

## Mechanical pass

- grep todo!/unimplemented!/HashMap/HashSet/sort_unstable/partial_cmp in added
  lines: NONE.
- grep symbol-name/address/value hardcoding in added SRC lines (0x301014 /
  globalfree / global_cross / "glob / .name()== / strcmp): NONE — no special-casing.
- bare `as`: 2 in production code, both now `// cast:`-commented (F3 closed).
- cargo clippy -p kuna-decomp --lib -- -D warnings: clean.
- C++ oracle: 207/207 unit + 675/675 datatest, PARITY OK (zero C++/specs files
  touched -> provably untouched, re-run to confirm).
- exec-failures: base 2 (displayformat, indproto) -> branch 1 (indproto); the
  displayformat "Requested varnode does not exist" is gone on the branch (real
  decode parity, confirmed by running displayformat.xml in isolation on both).

## Adversarial tests (committed to the branch)

rust/crates/kuna-decomp/tests/verify_w10_global_persist_adversarial.rs (5, all green):
- av1_containment_boundary_is_inclusive_last_and_first  (getLast()>=end & first<=addr)
- av2_owned_scope_fallback_sets_mapped_addrtied_persist_plus_property
- av3_constant_address_never_matches  (stackContainer isConstant guard)
- av4_smallest_in_use_entry_wins  (findContainer strict-< minimal + inUse(usepoint):
    addr-tied/invalid-usepoint/in-uselimit/out-of-uselimit selection — non-vacuous,
    flags distinguished via readonly/volatil bits)
- av5_end_offset_wrap_near_u64_max_does_not_panic  (uintb wrap)
The GlobalQuery snapshot logic is UNCHANGED between round 1 and round 2, so these
remain valid and pin the transcription as faithful.

## Verdict rationale

A faithful PARTIAL with ZERO parity regression. The dataflow target (global-store
survival, displayformat exec-failure fix) is reached on a genuinely-faithful path
(the heritage `queryProperties` -> RETURN-COPY, which is where C++ delivers the
persist effect too). The round-1 REJECT condition (passing-SET regression) is
independently confirmed CLEARED — the branch passing set is a strict superset of
base. The two carried divergences (deferred early marking; has_store() input-param
guard) are cited losses (LOSS-190, LOSS-191), and the displayformat assertions
FAIL honestly (no fabricated pass). Per the protocol, a faithful PARTIAL with 0
regression and cited losses is ACCEPT-WITH-LOSSES.

losses: LOSS-190 (deferred set_varnode_properties early persist marking),
        LOSS-191 (directwrite has_store() input-param guard).
        (related: LOSS-187 — highvar-naming is_global_data persist proxy.)
