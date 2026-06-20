# w10-sborrow-compare
verdict: ACCEPT
verifier: independent-verifier (Opus 4.8)
date: 2026-06-15
gate: cargo test --workspace (in /tmp/kuna-sborrow) -> 3496 passed; 0 failed; clippy -p kuna-decomp --lib -D warnings -> clean

## Scope

Un-defers LOSS-056 + LOSS-066 (the `RuleSborrow`/`RuleScarry` signed-compare
rewrites and the `AddExpression` discovery primitive they consume). The
merge-base (effd91a) shipped only the trivial `sborrow(V,0)/scarry(V|0,*)=>false`
cases with a `SEAM(expression)` bail; this branch ports the full descend-walk
rewrite and the `AddExpression` class verbatim. This is a RESTORATION named in
LOSS-056's restoration criterion ("Sborrow/Scarry when expression.rs ports
AddExpression ... and add a positive end-to-end transform test"), not a new loss.

Files changed (3, none reserved):
- rust/crates/kuna-decomp/src/expression.rs (+`AddExpression` class)
- rust/crates/kuna-decomp/src/ruleaction_3.rs (RuleSborrow/RuleScarry rewrites)
- rust/crates/kuna-harness/tests/verify_w10_inline_body.rs (parity assertion bump 6->7)

## Faithfulness (C++ ruleaction.cc:3380-3506 + expression.cc:300-394)

`RuleSborrow::applyOp` / `RuleScarry::applyOp` are line-for-line transcriptions:
the trivial COPY rewrite (SBORROW: bvn==0 only; SCARRY: avn|bvn==0 — both
faithful to the asymmetry in C++), the `svn->beginDescend()..endDescend()` walk,
the `cvn = (in0==svn)?in1:in0` selection, the `signop->code()==INT_SLESS` guard,
the `constantMatch(0)` zside discrimination (zside=0 when in0 matches, zside=1
when in1 matches), `xvn = signop->getIn(1-zside)`, the AddExpression equivalence
gate, and the EQUAL->INT_SLESSEQUAL / NOTEQUAL->INT_SLESS arms with the exact
`(avn,1-zside)/(bvn,zside)` vs `(avn,zside)/(bvn,1-zside)` slot placement.
RuleScarry's pre-loop swap (`avn=bvn; bvn=op->getIn(0)`), the integer-minimum
guard (`val = calc_mask(size); val ^= (val>>1)`), and the `newval = -bvn->off &
calc_mask(size)` materialization are all exact. `AddExpression::gather` /
`gatherTwoTermsSubtract/Add/Root` / `isEquivalent` / `Term::isEquivalent` match
expression.cc including the `depth-=1` only-on-non-const-in1 rule, the
INT_MULT-coeff fold, and the commutative 2-term equivalence (both orderings).

## Hunt list

- Signedness: clean. `zside` int4 ∈ {0,1}; `1-zside` int4 ∈ {1,0}; no underflow.
  No int4-vs-uint comparisons introduced.
- Integer widths: clean. uintb=u64, int4=i32 mappings consistent with C++;
  `calc_mask(int4)` and `constant_match(uintb)` byte-verified against C++.
- Wrapping: clean. `constval = constval + coeff*off` -> `wrapping_add(wrapping_mul)`;
  `coeff *= off` -> `wrapping_mul`; `newval = -off` -> `0u64.wrapping_sub(off)`.
  All uintb-lineage arithmetic carries wrapping; matches C++ silent unsigned wrap.
- Comparator totality: n/a (no operator< ported here).
- Iteration-order provenance: the descend walk collects `descend_iter()`
  (DescendVec/SmallVec, insertion order == C++ `std::list<PcodeOp*>`) into a Vec,
  iterates in order, returns on first match. No HashMap/HashSet. Order parity
  preserved; the rule returns before any second iteration so collect-then-iterate
  is equivalent to the live iterator (no erase-while-iterate hazard).
- Off-by-one / do-while: n/a; `1-zside` boundary verified.
- Erase-while-iterating: clean (single mutation + immediate return).
- Exception->Result: `.expect()` only on invariants C++ also assumes
  (written-vn-has-def, valid slot index); no partial-state divergence.

## Special-casing

None. grep of added production code found no hardcoded constant / function-name /
address / symbol name. Every literal (0,1,2, the `constantMatch(0)`, `numTerms<2`)
comes directly from the C++. All operand values flow from the IR
(`vn_offset`/`get_offset`) and op const inputs.

## No-regression (independent re-run, decomp_test_dbg, full corpus)

merge-base effd91a: 668 applied / **222 passing**
branch ea16648:     668 applied / **225 passing**
passing-SET diff (base -> branch): regressions EMPTY; new = exactly 3
  - Inlining #2            (inline.xml:        `if (x < 10)`              — SBORROW s<)
  - For-loop with skip #2  (forloop_withskip.xml: `if (0xb <= val)`       — s<=)
  - Bitfields #7           (bitfields.xml:     `if (...3... <= ...sfield4)`— s<=)
FAIL-SET diff: zero new FAILs; the 3 above flipped FAIL->Success.
All three are genuine SBORROW/SCARRY-produced signed-compare normalizations.
boolless / readstruct / condconst_conn: BYTE-IDENTICAL base vs branch.

The harness `verify_w10_inline_body` assertion (Total passing tests = 7,
`Success -- Inlining #2`) is REAL parity — re-confirmed by running inline.xml
on both base (6/12, #2 FAIL) and branch (7/12, #2 Success) binaries directly,
not a fabricated/skipped assertion (the x86-64 .sla is built, no SKIP emitted).

## Other gates

- cargo test --workspace (worktree): 3496 passed, 0 failed.
- cargo clippy -p kuna-decomp --lib -- -D warnings: clean (no warning from the
  new code; pre-existing crate warnings only surface under --tests).
- C++ oracle untouched: `git diff -- decompiler/ specs/ docs/baseline.json`
  empty; `python -m kuna.run_tests --all --baseline docs/baseline.json` ->
  207/207 unit, 675/675 datatest, PARITY OK.
- B0 list-action order unchanged: universalaction.rs not touched; sborrow(396)
  precedes scarry(397) precedes sborrowge/lessequal2zero/... matching
  coreaction.cc:5783/5825/5878.

## findings
  - F1 (nit, non-blocking): bare `as` cast without a `// cast:` justification
    comment (the verifier-protocol mechanical convention). Safe in fact —
    `num_terms` is bounded 0..2 by `add`'s `if self.num_terms < 2` guard, so the
    index is always in-range — but the convention comment is absent.
       cpp: decompiler/cpp/expression.hh:165 (`terms[numTerms++]`)
       rust: rust/crates/kuna-decomp/src/expression.rs:75 (`self.num_terms as usize`)

## adversarial tests (committed to branch as b5d77cf)
  - w10_sborrow_equal_zside1_folds_to_slessequal  (EQUAL arm, zside==1 swap -> b s<= a)
  - w10_sborrow_notequal_zside0_folds_to_sless     (NOTEQUAL arm, zside==0 swap -> a s< b)
  - w10_sborrow_nonequivalent_expr_rejected        (AddExpression gate false-fold guard -> rule no-ops, opcode untouched)
  - w10_scarry_constant_notequal_folds_to_sless_negconst (newConstant(-#W)=0xfffffffb materialization, zside==1)
  All 4 pass against the port; together they cover EQUAL/NOTEQUAL x zside{0,1},
  the equivalence rejection, and the scarry constant negation — the branches the
  porter's single NOTEQUAL/zside==1 test did not exercise.

## losses
  none (faithful full port, zero regression — restores LOSS-056/LOSS-066 deferral).
