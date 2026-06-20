# w5-s3-rules-4
verdict: ACCEPT-WITH-LOSSES
verifier: mahaloz (independent verifier subagent)
date: 2026-06-11
round: 2

gate: `cd rust && cargo test -p kuna-decomp` -> PASS (463 lib + all integration
      suites green; the item's `verify_w5_s3_rules_4.rs` now 9/9 — round 1's 5
      tests + 4 new round-2 tests). `cargo clippy -p kuna-decomp --lib -- -D
      warnings` -> clean (exit 0); the item's test file is clippy-clean under
      `-D warnings`. (The only `-D warnings` hits in the crate are in OTHER waves'
      test files — verify_w3_*/verify_w4_* — out of scope for this item.)

scope: decompiler/cpp/ruleaction.cc:4293-5526 (19 classes RuleLoadVarnode..
       RuleCondNegate) + ruleaction.hh:793-1008 declarations. cpp_blob_sha
       verified in-tree (git hash-object): ruleaction.cc = cb1ce08359..,
       ruleaction.hh = 5b8c80d2d0.. — identical in main and worktree, no drift.
       Rust: rust/crates/kuna-decomp/src/ruleaction_4.rs (+ ruleaction_4/tests.rs).
       (dtype.rs / verify_w5_dtype_expand.rs in this branch's diff belong to the
       parallel w5-dtype item and are OUT OF SCOPE here.)

## Round-1 -> round-2 disposition
Round 1 REJECTED on two findings; both are repaired and independently re-verified:

- **F1 (was BLOCKER) — shift-by->=64 / negative-int4 panic — FIXED.** Both fix
  sites now route through the ADR-0003 wrapping shift helpers (`wshl`/`wshr` =
  `wrapping_shl`/`wrapping_shr`, count masked `& 63`, exactly the x86 SHR/SHL the
  C++ compiles to): RuleShiftAnd (ruleaction_4.rs:1212-1216) and RuleSubZext
  INT_RIGHT branch (:1494). The three round-1 repro tests pass; I added a 4th at
  the RuleSubZext site (round 1 only covered RuleShiftAnd) and one on the
  INT_MULT->leastsigbit->INT_LEFT route — all green.
- **F2 (was MAJOR) — wrong registration group — FIXED.** RuleLoadVarnode /
  RuleStoreVarnode now carry `group:"stackvars"` in `specs()` (:2121-2122) and
  gate `clone_rule` on `contains("stackvars")` (:280, :340); the other 17 stay
  "analysis". The module doc (:17-19) and the in-crate `specs_in_definition_order`
  test agree. Matches coreaction.cc:5939-5940 (actprop2) vs 5845-5876 (actprop).
  Round-1's `loadstore_varnode_clone_under_stackvars_group` now passes.

## Hunt list (re-derived rule-by-rule, all 19)
- **signedness**: clean. Every `int4`-vs-`int4` comparison (RuleSubCancel
  `farinsize<=offset` :1604, RuleDumptyHump `offset<vn2.size` :1832, RuleShiftSub
  `c<0||c+outsize>insize` :1683, RuleSubExtComm `outsize+subcut<=invn` :431) is
  signed-on-both-sides, matching the C++ `int4`s; the data-derived offsets use
  `offset_of(..) as int4` mirroring the C++ `(int4)getOffset()` narrowing
  byte-for-byte (incl. the negative-cast path). No `int4`-loop-index-vs-size.
- **integer widths**: clean. RuleZextShiftZext (:1112) does the `int4-int4`
  subtraction BEFORE the `(uintb)` cast (and a ZEXT guarantees the difference is
  >= 0); the C++ `(int4)`/`(uintb)` casts are all mirrored.
- **wrapping**: F1 RESOLVED (above). Residual bare `+`/`*` on `uintb`-lineage
  values where C++ relies on x86 wraparound remain (F3, MINOR): :424
  (`outsize+subcut`), :1446 (`offset*8`), :1496 (`sa + offset*8`), :1758
  (`pos2+size2`). All but :1496 are bounded by a structural `size <= 8` /
  small-offset guard and cannot wrap on a well-formed graph; :1496 adds a small
  term to an arbitrary right-shift constant `sa` (no `>8` guard in the C++
  INT_RIGHT branch either) — a debug panic only on a malformed-but-legal operand.
  Not gating; same family as the accepted LOSS-068. The `val <<= 8*lo.size`
  shifts (:851/:865/:870) are guarded by RuleConcatCommute's `outsz<=8` early-out
  (shift count < 64) — safe.
- **comparator totality**: n/a — no `operator<`/comparator ported in this batch.
- **iteration-order provenance**: clean. Two loops only — RuleConcatCommute
  `for i in 0..2` (slot index, C++ i==0/i==1) and RuleSubCommute
  `for i in 0..numinput` (input-slot index, with the `lastIn`/`newVn` dedup);
  both reproduce the C++ index order. No HashMap/HashSet anywhere.
- **off-by-one / do-while / reverse iterators**: clean — no do-while / rbegin /
  --end idioms in scope. RuleSwitchSingle's `for i in 1..numEntries` label scan is
  unreachable (seam).
- **erase-while-iterating**: clean — no container erase during traversal.
- **exception -> Result parity**: acceptable. `opSetInput`/`opInsertInput` are
  `.expect()`ed ("can't happen" on a well-formed rule match); C++ does not throw on
  these. The seam shims (`new_unique_out`/`op_set_output`) construct fresh uniques
  whose unify branch is dead, so the no-op `replace_reads` callback is exact.
- **AND-for-rules (transposed cond / swapped slots / inverted returns / missing
  early-outs)**: clean. RuleSubCommute dedup `new_vn.filter(|_| last_in==Some(vn))`
  is the exact `lastIn==vn && newVn!=0` reuse condition, with `last_in` updated
  EVERY iteration (matching C++ `lastIn=vn` outside `if(i!=j)`). RuleHumptyOr's
  a/b/c/d four-arm reassignment (incl. the three-slot `b==c` swap `b=a;a=c;c=d`)
  matches C++ 5371-5384 exactly — adversarially re-tested (R2-C). RuleSubCancel /
  RuleDumptyHump / RuleHumptyDumpty branch structure faithful.

## Findings
- **F1 (was BLOCKER) — RESOLVED in round 2.** See disposition above.
- **F2 (was MAJOR) — RESOLVED in round 2.** See disposition above.
- **F3 (MINOR, non-gating): residual bare `+`/`*` on `uintb`-lineage values.**
  Sites :424, :1446, :1496, :1758. ADR 0003 prefers `wadd`/`wmul`; all are either
  structurally bounded (cannot wrap on a well-formed graph) or, for :1496, a debug
  panic only on a malformed-but-legal operand where C++ silently wraps. Mirrors the
  accepted LOSS-068 family; not a blocker.
       cpp: decompiler/cpp/ruleaction.cc:4435, 5079, 5108, 5270
       rust: rust/crates/kuna-decomp/src/ruleaction_4.rs:424, 1446, 1496, 1758
- **F4 (MINOR/style, non-gating): 28 bare `as` casts lack the `// cast:` marker.**
  Each mirrors an explicit adjacent C++ `(int4)`/`(uintb)` cast (shown in a `// C++`
  comment) and is semantically faithful; the two shift sites (:1212-1216, :1494)
  carry inline `cast:` prose. The protocol's literal `// cast:` convention is unmet
  but no cast changes semantics. Not a blocker.
       rust: rust/crates/kuna-decomp/src/ruleaction_4.rs (28 sites; e.g. :426, :595, :1190)

## Losses (cross-wave seams — appended to docs/rust-port/losses.md)
- **LOSS-070**: RuleLoadVarnode / RuleStoreVarnode `check_spacebase` returns `None`
  (W4 space-machinery seam) -> both `applyOp` no-op where C++ folds a constant /
  spacebase LOAD/STORE to a COPY. Conversion bodies fully transcribed; classifier
  stubbed. Pinned by `loadvarnode_seam_noop`; `specs()` unwired today.
- **LOSS-071**: RuleSwitchSingle returns `0` after the `sizeOut()!=1` guard
  (W?-jumptable seam) -> the single-destination BRANCHIND->BRANCH conversion never
  fires. CONSERVATIVE (matches the C++ `jt==0` early-out exactly); latent.
- **LOSS-072**: RuleCondNegate is PARTIALLY stubbed (W3-block seam) — it inserts
  the BOOL_NEGATE but `opFlipCondition` is a no-op and `opNormalizeFlip` reports
  false. A REACHED body would negate the boolean WITHOUT flipping the branch sense
  (inverted-control-flow hazard) and leave `isBooleanFlip()` set (re-trigger).
  Latent only — the rule is unwired and its sole test hits the early-out — but the
  ledger flags that it MUST NOT be wired before the flip primitive lands.

## Mechanical pass
- grep ruleaction_4.rs for `todo!`/`unimplemented!`/`HashMap`/`HashSet`/
  `sort_unstable`/`panic!`/`.unwrap()`: NONE.
- bare `as` casts: 28 (see F4) — all faithful to an adjacent C++ cast.
- `cargo clippy -p kuna-decomp --lib -- -D warnings`: clean (exit 0); the item
  test file `verify_w5_s3_rules_4`: clean under `-D warnings`.
- port-audit: `port-audit` binary absent from this worktree; blob-sha checked by
  hand (`git hash-object`) against main — both files match, no drift.

## Adversarial tests
Round-1 tests (5, branch @ 8fdfa60, now all PASS post-repair):
  shiftand_right_shift_64_matches_cpp_no_panic, shiftand_left_shift_64...,
  shiftand_negative_int4_shift_count_no_panic, loadstore_varnode_clone_under_
  stackvars_group, subextcomm_sub_misses_ext_bits_to_copy.
Round-2 tests added by this verifier (4, branch @ f09a066,
  rust/crates/kuna-decomp/tests/verify_w5_s3_rules_4.rs — all PASS):
  - subzext_right_shift_64_matches_cpp_no_panic  (the 2nd F1 fix site round 1
    never exercised: RuleSubZext INT_RIGHT `val.wshr(sa)`; >>64 folds, no panic)
  - shiftand_mult_power_of_two_large_sa_no_panic  (INT_MULT -> leastsigbit_set ->
    INT_LEFT route with a large wrapping `nzm <<= 40`; (mask&nzm)!=nzm declines)
  - shiftand_mult_non_power_of_two_rejected  (the `testval != offset` MULT gate)
  - humptyor_b_eq_c_swap_picks_shared_operand  (the three-slot b==c reassignment
    arm — transposed-slot hunt; the shared operand survives as the COPY source)

## Verdict rationale
Both round-1 blockers (F1 panic, F2 group) are repaired and independently
re-verified against the C++ oracle with fresh adversarial tests. The remaining
divergences are (a) MINOR non-gating quality items (F3 bare arithmetic on
unreachable/bounded paths, F4 cast-comment convention) and (b) four documented
cross-wave seam losses (LOSS-070/071/072) where the rule bodies no-op or partially
mutate exactly because their W4/W3/jumptable dependencies are absent and nothing in
the merged W5 tree consumes `specs()`. No blocker or major stands. ACCEPT-WITH-LOSSES.
