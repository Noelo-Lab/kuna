# w5-s3-rules-6
verdict: ACCEPT-WITH-LOSSES
verifier: claude-opus (independent verifier, round 1)
date: 2026-06-11
gate: `cd rust && cargo test -p kuna-decomp` -> ok (all binaries pass; `verify_w5_s3_rules_6` 8/8, lib 462/462)

## scope

C++ in scope: `decompiler/cpp/ruleaction.cc` 6931-8373 (14 classes
RulePtraddUndo, RulePtrsubUndo, RuleMultNegOne, RuleAddUnsigned, Rule2Comp2Sub,
RuleSubRight, RulePtrsubCharConstant, RuleExtensionPush, RulePieceStructure,
RuleSubNormal, RulePositiveDiv, RuleDivTermAdd, RuleDivTermAdd2, RuleDivOpt) +
`ruleaction.hh` declarations. blob-sha verified against checklist (cc
`cb1ce08…`, hh `5b8c80d…`) — **no drift**.

Note: the checklist *title* ("RuleMultNegOne..RuleDivChain, lines 7188-8479")
is stale; the ported file and this prompt both cover the contiguous 14-class
block RulePtraddUndo..RuleDivOpt (ending exactly before RuleSignDiv2). The port
`specs()` lists exactly those 14 in C++ definition order with the verbatim
`name()` strings — confirmed against the .hh ctors. Not a defect.

This is a heavily **cross-wave-seamed** port: 13 of 14 rules pass their real
guards (on available API) and then return 0 at the first missing W3/W4/W6
primitive. The only fully-committing path is `RuleSubNormal`'s `n==0`
unnecessary-shift collapse. The pure read-only helpers are transcribed in full
and are the load-bearing testable surface.

## hunt list

- **Signedness**: clean. Every `uintb getOffset()` -> `int4`/`int8` narrowing
  matches C++ implicit conversion (e.g. `n`/`c` in RuleSubNormal/findForm,
  `multiplier` in getConstOffsetBack). `xsize > 4*size`, `8*out - n != xsize`,
  the `sa*8-1` nz-mask probes are all signed-int comparisons mirroring C++.
- **Integer widths**: clean. `calc_divisor` uses fixed `[u64;2]` words exactly
  like C++ `uint8[2]`; `maxx = (xsize==64)?0:(1<<xsize)` then `-1` reproduced;
  `8*sizeof(uintb)-clz` == 64-clz preserved (operator precedence `8 * (… as
  int4)` checked).
- **Wrapping**: clean. Arithmetic on `int8`-lineage values uses `wadd`/`wmul`/
  `wsub` (Wrap trait); `calc_divisor`'s `q[0]+1`, `q[0]-r[0]`, `maxx-1` use
  `wadd`/`wsub`; the 128-bit ops route through kuna_num. **Adversarial test
  confirmed the fixed-width u64 `diff += q[0]-r[0]` wrap is load-bearing** (my
  first bignum oracle was wrong; the port matches the C++ fixed-width result).
- **Comparator totality**: N/A — no `operator<`/comparator, no float compares,
  no BTree/sort introduced.
- **Iteration-order provenance**: clean. Every descend loop uses
  `descend_snapshot` (collect of the C++ `Varnode::descend` list, order
  preserved); `lone_descend` matches C++ `loneDescend`. **No HashMap/HashSet.**
- **Off-by-one / do-while / reverse iterators**: clean. `get_const_offset_back`
  depth-limit (`maxLevel-=1; <0` early-out) ported; `spanning_range` loop
  faithful; `RuleSubNormal`'s `k+c+outsize > insize` (strict `>`, not `>=`)
  pinned by `w5s3r6_sub_normal_boundary_equal_collapses_not_extension`.
- **Erase-while-iterating**: clean. `move_sign_bit_extraction` grows `testList`
  via an index walk (matches C++ `for(i=0;i<size();++i)` + push_back) and reads
  descendants from a snapshot — same elements, same order. opSetInput redirect
  visits exactly the original descendants.
- **Exception -> Result parity**: the single `?` (`spanning_range` ->
  `get_sub_type`) faithfully propagates a deeper dtype seam; no apply_op uses
  `?`. `op_set_input(...).expect()` matches C++ "cannot fail here" contract and
  is exercised green by the n==0 commit tests.

## mechanical pass

- grep: no `todo!`/`unimplemented!`/`HashMap`/`HashSet`/`sort_unstable` in
  source. No `.unwrap()` in non-test source.
- bare `as` casts: ~20, **none carry `// cast:` comments** (F1, minor — see
  findings). Each was re-derived and is semantically correct (uintb->int4/int8
  narrowing mirrors C++; sign-extending i32->u64 in `c as uintb` matches C++
  `(uintb)int4`; precedence `8 * (size_of as int4)` correct).
- `cargo clippy -p kuna-decomp --test verify_w5_s3_rules_6 -- -D warnings`:
  clean (after fixing my own doc-list-continuation warnings). The item source
  `ruleaction_6.rs` is clippy-clean. (Pre-existing clippy doc warnings in
  `verify_w3_ir_op`/`verify_w4_fw_options` are other items' files, out of scope.)
- `port-audit` crate is not present at this merge base (future-wave tooling);
  blob-sha checked manually instead — matches.

## adversarial tests (added at tests/verify_w5_s3_rules_6.rs, committed eff08de)

1. `w5s3r6_calc_divisor_off_by_one_adjustment_branch` — the `r>=q` adjustment
   branch + the fixed-width u64 `q[0]-r[0]` wrap (0x2aaaaaaac->3, 0x2aaaaaaad->0).
   *Caught my own incorrect bignum oracle; the port is correct.*
2. `w5s3r6_calc_divisor_rejections_return_zero` — y<=1, n>127, xsize>64, non-magic.
3. `w5s3r6_get_extra_offset_sign_extends_to_negative` — 0x80000000 at a 4-byte
   output sign-extends to a negative int8 (signedness/width boundary).
4. `w5s3r6_get_extra_offset_wide_output_stays_positive` — same value, 8-byte
   output stays positive (proves the extend width is outvn-driven).
5. `w5s3r6_move_sign_bit_through_copy_with_and_masked_amount` — COPY-chain
   testList growth + the INT_AND mask-superset guard, redirect committed.
6. `w5s3r6_move_sign_bit_and_wrong_amount_no_redirect` — AND amount 30 != 31, no
   redirect.
7. `w5s3r6_sub_normal_boundary_equal_collapses_not_extension` — the `>` boundary
   + n==0 commit (a distinct committing path from the porter's test).
8. `w5s3r6_sub_normal_shrink_k_then_seam_no_change` — the `k=insize-c-outsize`
   shrink, then the newOp/newUniqueOut seam leaves the SUBPIECE structurally
   untouched (pins the partial-state bail is inert at the op level).

## findings

- F1 (minor): bare `as` casts in `ruleaction_6.rs` lack the `// cast:`
  justification comments the protocol's mechanical pass requires (e.g. :245,
  :931, :933, :979, :1280, :1635). All were re-derived and are **semantically
  correct** — this is a documentation/style gap, not a correctness defect. Does
  not block ACCEPT.
       cpp: decompiler/cpp/ruleaction.cc:7761 (`int4 n = ...getOffset()`)
       rust: rust/crates/kuna-decomp/src/ruleaction_6.rs:931

- F2 (loss, accepted -> LOSS-058): 13 of 14 rules are cross-wave-seamed to
  no-op (W3 newUniqueOut/newExtendedConstant/isConstantExtended, W4
  equate/enum/scope/stringManager, W6 opSetOpcode(OpCode)/type-facing/
  opUndoPtradd/getExactPiece/duplicateNeed). Only RuleSubNormal's n==0 path
  commits. The seam helpers return the C++ guard's *failing* value, so each rule
  no-ops exactly where its subsystem is missing — no invented behavior.
       cpp: decompiler/cpp/ruleaction.cc:6943,7202,7843,8114,...
       rust: rust/crates/kuna-decomp/src/ruleaction_6.rs:76-136 (seam helpers)

- F3 (loss, accepted -> LOSS-058 partial-state note): RuleSubNormal's extension
  sub-path (:949) and final-shift path (:993) call `new_op(2,...)` and then bail
  at a seam, leaving an orphan dead PcodeOp. It is inert (no varnode references
  it, on the dead list, unbounded re-creation prevented by deterministic no-op)
  and is the established LOSS-035 `newUniqueOut`-after-`newOp` class also present
  in `Funcdata::clone_op`. Rule2Comp2Sub correctly gates its seam BEFORE any
  input rewiring, leaving no partial state. Not a net-new divergence.
       cpp: decompiler/cpp/ruleaction.cc:7776,7807 (newOp in C++ proceeds)
       rust: rust/crates/kuna-decomp/src/ruleaction_6.rs:949,993

## losses

LOSS-058 (appended this review) — covers F2 (13-rule seam-to-no-op) and F3
(RuleSubNormal orphan-op partial state). Cited per the ACCEPT-WITH-LOSSES
requirement.

## verdict rationale

No blocker or major stands. The committable logic (RuleSubNormal n==0 collapse,
all the pure helpers — calcDivisor 128-bit reciprocal math, getConstOffsetBack/
getExtraOffset walkers with the sign-extend boundary, findSubshift/findForm/
checkFormOverlap matchers, moveSignBitExtraction) is faithful and survives
adversarial boundary/wrap/order tests. The remaining divergences are
cross-wave seams of the same accepted class as LOSS-030..038/042/056/057,
properly marked and now ledgered as LOSS-058. F1 is a documentation gap on
verified-correct casts. **ACCEPT-WITH-LOSSES.**
