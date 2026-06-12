# w5x-helpers-completion
verdict: ACCEPT-WITH-LOSSES
verifier: claude-opus (independent, round 1)
date: 2026-06-12
gate: `cd rust && cargo test -p kuna-decomp` -> PASS (41 test binaries OK, 1253+ tests, 0 failed; my 6 new adversarial tests included). `cargo clippy -p kuna-decomp -- -D warnings` -> clean (lib). `cargo test --workspace` -> 1823 passed, 0 failed.

## Scope

Completion of the `funcdata_op.cc` helpers the W5 wave left seamed, and the
rules whose only blocker was one of them:

- Helpers ported: `replaceLessequal` (:1029), `distributeIntMultAdd` (:1079),
  `cseElimination` (:1397) + `cseEliminateList` (:1459) + `compareCseHash`
  (:1445), and the `opSetInput` (:104) constant re-duplication guard.
- W6 typeop-flag seam (`w6_type_op`) installs the verbatim `typeop.cc` opflags.
- `Varnode::set_annotation` `pub(crate)` sliver (resolves LOSS-077); `newCodeRef`
  now sets the annotation flag.
- Rules un-deferred: `RuleIntLessEqual` (replaceLessequal), `RuleCollectTerms`
  (distributeIntMultAdd + the already-landed TermOrder), and `RuleOrMask` /
  kuna `RuleAddCarryChain` (now complete because `opSetInput` re-duplicates a
  re-shared constant instead of aliasing it — the prior `add_descend` error path
  is no longer reached). `RuleSelectCse` stays deferred (W7 `isHeritaged`).

## Statement-order fidelity (the FOCUS)

- **replaceLessequal** — re-derived line-by-line against funcdata_op.cc:1037-1070.
  Slot/diff selection (in0->diff=-1/i=0, in1->diff=+1/i=1, else false), the four
  signed/unsigned overflow guards (`calc_int_min`/`calc_int_max` for SLESSEQUAL,
  `val==0`/`calc_uint_max` for LESSEQUAL), `opSetOpcode` BEFORE `newConstant`,
  `res=(val+diff)&calc_mask`, `opSetInput(op,newvn,i)`, then the kuna
  `setCanonicalLessequal()` provenance recorder LAST — all in C++ order. Match.
- **distributeIntMultAdd** — re-derived against funcdata_op.cc:1083-1125. addop
  resolution, the two `(isFree && !isConstant)` bails, `coeff`/`sz`/`mask` reads,
  the first-input then second-input distribution (constant arm = `newConstant`,
  non-constant arm = newOp/opSetOpcode/newUniqueOut/opSetInput x2/opInsertBefore),
  then `opSetInput(op,newvn0,0); opSetInput(op,newvn1,1); opSetOpcode(op,INT_ADD)`
  LAST — order preserved exactly. Match.
- **cseEliminateList** — re-derived against funcdata_op.cc:1465-1487. `if empty
  return`, `stable_sort` (Rust `sort_by` IS stable; comparator on `.0` hash only,
  a total order on uintm), the adjacent-pair walk `(idx-1,idx)` reproducing the
  C++ `liter1/liter2=begin+1` lockstep, the dead/CSE-match gate, the
  null-or-heritaged double-guard, `cseElimination` then `outlist.push_back(out)`.
  Off-by-one / at-least-once boundary verified by my test (empty + single = no-op).
  Match.

## The constant re-duplication guard in opSetInput

C++ funcdata_op.cc:108-115: a non-spacebase constant WITH a descendant is
re-duplicated via `newConstant` BEFORE `addDescend`. This is load-bearing:
`Varnode::addDescend` (varnode.cc:330-337) THROWS "Free varnode has multiple
descendants" for ANY free-and-not-spacebase varnode (a constant IS `isFree()`)
that already has a descendant — so the dedup is precisely what prevents the throw
on a re-shared constant. The Rust guard (`if v.is_constant() && !v.has_no_descend()
&& !v.is_spacebase()`) re-duplicates via `new_constant`, runs BEFORE
`op_unset_input`/`add_descend`/`set_input` (C++ order), and `add_descend`
(varnode.rs) mirrors the C++ throw condition exactly. The prior Rust aliased the
shared constant (a real correctness divergence: a second read on a single-descend
constant); now faithful. Confirmed the un-defer of RuleOrMask/RuleAddCarryChain
follows from this: the const-folded carry path in `kuna_addcarrychain` re-wires a
constant into a new op while the original still reads it, and the dedup is what
lets it complete instead of erroring (verify_w5_kuna_rule_pack pin flipped
correctly from "panics" to "dedups and completes").

## Un-deferred rules re-checked against C++ bodies

- `RuleIntLessEqual::applyOp` (ruleaction.cc:612-618) = `if(replaceLessequal) return
  1; return 0`. Rust matches. No residual blocker.
- `RuleCollectTerms::applyOp` (ruleaction.cc:108-177) — full body re-derived:
  root-of-ADD early-out, the combine loop (getMultCoeff, distributeIntMultAdd on
  either multiplier, the coef-merge with newConstant/zerocoeff and the INT_MULT
  insertion), and the lump-constants tail. `getMultiplier() != 0` -> `Some(m)`
  faithful; the post-loop `i` reuse and the `for j=size-1;j>=i` reverse loop have
  no off-by-one (verified usize-bounds). No residual blocker (TermOrder landed in
  w5-s3-condexe-expression).
- `RuleOrMask` / `RuleAddCarryChain` — bodies already complete; un-deferred only by
  virtue of the opSetInput dedup. Re-checked: no W6/W7 blocker remains.

## No rule un-deferred that still has a W6/W7 blocker

`RuleSelectCse` is correctly LEFT deferred (returns 0): its only path,
`cseEliminateList`, needs `isHeritaged` = `heritage.heritagePass(addr)>=0` on a
`Heritage` the Rust Funcdata does not yet own (W7). `cse_eliminate_list` takes the
heritaged predicate as a parameter so W7 wires it; the rule cannot supply it and
declines, matching pre-heritage behavior (nothing heritaged). Confirmed no other
un-deferred rule reaches an unported W6/W7 primitive.

## Hunt list

- Signedness: clean. `int4 i`/`diff:i64`/`nonzerocount:i32`/`lastconst:usize` all
  faithful; the `for j=size-1;j>=i` reverse loop ported as `while j>i { j-=1 }`
  with no boundary slip.
- Integer widths: clean. uintb/intb/uintm mappings correct; `diff:i64` for ±1.
- Wrapping: clean. `val.wrapping_add(diff as u64)` (diff=-1 -> 0xFFFF..F -> val-1),
  `coeff.wrapping_mul(off)`, `coef1.wrapping_add(coef2)` — all match C++ silent
  unsigned wrap. The one bare `diff as u64` cast is correct two's-complement.
- Comparator totality: clean. `compareCseHash` -> `a.0.cmp(&b.0)` on uintm (total).
- Iteration-order provenance: clean. `cse_eliminate_list` operates on a `&mut [..]`
  the (deferred) caller fills from `beginDescend()` order; `sort_by` is stable;
  RuleCollectTerms walks `TermOrder::get_sort()` (a `Vec<usize>` index list). No
  HashMap/HashSet in any ported logic (grep-confirmed in the changed files).
- Off-by-one / do-while: clean. cseEliminateList adjacent-pair walk verified
  (empty + single = no-op test). No do-while.
- Erase-while-iterating: N/A. cseEliminateList destroys ops in the graph, never
  erases from the sorted list; the next-iteration `is_dead()` guard catches a
  destroyed neighbor exactly as C++.
- Exception->Result partial-state: clean. The only error sources
  (`add_descend`/`total_replace`/`new_*_out`) are `?`-propagated; in practice the
  dedup guard makes `add_descend` unreachable for the constants these helpers
  re-wire, and non-constant inputs are non-free, so no throw path fires. The
  copySymbol no-op happens after the duplicate is fully linked (no partial state).

## Mechanical pass

- grep (changed src): no `todo!`/`unimplemented!`/`HashMap`/`HashSet`/
  `sort_unstable`. One bare `as` cast (`diff as u64`) — correct, documented.
- clippy `-p kuna-decomp -- -D warnings` (lib): clean.
- Note (out of scope): `cargo clippy --workspace --tests` surfaces a pre-existing
  hard error at `heritage.rs:1602` (`overly_complex_bool_expr` in a unit test) and
  many pre-existing per-test warnings — all present on the `rust-port` base
  (eb3bac0), none in this item's diff. Not a blocker for this item; flagged for
  the heritage item owner.

## Findings

- F1 (loss, accepted -> LOSS-079): `replaceLessequal`:1062 and `opSetInput`:112
  drop `Varnode::copySymbol(vn)` onto the re-duplicated constant — the duplicate
  carries `newConstant`'s unknown base, not the source's data-type/`SymbolEntry`/
  `typelock|namelock`.
     cpp: decompiler/cpp/funcdata_op.cc:112, :1062 (copySymbol at varnode.cc:512)
     rust: rust/crates/kuna-decomp/src/funcdata_op.rs op_set_input(~:258),
           replace_lessequal(~:455)
  Structural no-op at W3 (constants carry no symbol yet — identical bits); becomes
  cosmetic-only (never a wrong value/op-graph) once W4/W6 symbol propagation lands.
  The porter's doc comment names it "(latent) loss" but did not ledger it; the
  verifier appended LOSS-079.

## w6_type_op seam — opflags verified vs typeop.cc

INT_SLESS = binary|booloutput; INT_LESS = binary|booloutput; INT_ADD =
binary|commutative; INT_MULT = binary|commutative; COPY = unary|nocollapse;
SUBPIECE = binary; INT_SRIGHT = binary. All match typeop.cc constructors exactly.
The `_ => binary` fallback is a conservative default (worst case a missing
special-semantics flag, never a wrong rewrite). Mirrors the existing
`ruleaction_5::type_op_seam` pattern.

## Adversarial tests (committed to branch worktree-wf_ac003af6-b71-1 @ 90b937a)

rust/crates/kuna-decomp/tests/verify_w5x_helpers_completion.rs (6 tests, all pass):
- w5x_replace_lessequal_signed_min_in_slot0_declines (SMIN<=x decline guard)
- w5x_replace_lessequal_signed_const_slot0_rewrites_diff_minus1 (c<=x rewrite, i=0)
- w5x_replace_lessequal_unsigned_zero_and_uintmax_decline (0<=x and x<=UMAX guards)
- w5x_cse_eliminate_list_empty_and_single_are_noops (off-by-one boundary)
- w5x_distribute_int_mult_add_both_const_folds_no_new_mult (all-constant fold path)
- w5x_op_set_input_third_reshare_dups_independently (re-dup guard re-evaluation)

## Pinned-test flips (verified correct)

- verify_w3_ir_funcdata_op: const-reshare now dedups (was: errs). Correct — the
  dedup matches C++; the fresh constant has zero descendants so add_descend
  doesn't throw.
- verify_w4x_flow_linkage: newCodeRef annotation flag now set (LOSS-077 closed).
- verify_w5_kuna_rule_pack: const-carry now fuses (was: pinned panic). Correct —
  the dedup is exactly what lets the const-folded carry path complete; the
  "multiple descendants" throw is for free non-constant varnodes only.

losses: LOSS-079 (appended by verifier); LOSS-077 closed by this item.
