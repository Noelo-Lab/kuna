# w10-divmod
verdict: ACCEPT-WITH-LOSSES
verifier: independent verifier (Opus 4.8 1M)
date: 2026-06-15
branch: rport/w10-divmod (round 2)

gate:
  - `cargo test --workspace` (rust/) -> 3345 passed / 0 failed (incl. the 4
    verifier-added independent tests). kuna-harness 48/48.
  - boolless byte-parity: `print_b5_boolless` 11/11, `printc_parity`
    `corpus_functions_byte_compare_against_cpp_oracle` ok — boolless/readstruct/
    condconst byte-identical.
  - `verify_w10_struct_corpus` 16/16 (incl. both R2 divopt reciprocal-multiply
    gates), `verify_w10_divmod` 4/4, `verify_w10_divmod_independent` 4/4.
  - C++ oracle: Rust-ONLY diff — no decompiler/ or specs/ or docs/baseline.json
    file touched -> 675/675 PARITY OK untouched by construction.
  - `cargo clippy -p kuna-decomp --lib` clean (0 warnings); new test file clippy-clean.

## Round-2 disposition

Round 1 REJECTed on a single faithfulness defect (F1): `RuleDivOpt::findForm`
dropped C++ GUARD A (`if (!inVn->isWritten()) return 0;` on `curOp->getIn(0)`,
ruleaction.cc:8113), turning a clean C++ `return 0` into a Rust `def_of().expect()`
PANIC on a free MULT-input-0.

**F1 is FIXED** in commit 6f7358b: GUARD A is now ported verbatim at
ruleaction_6.rs:1654 (`if !is_written(data, in_vn_cur) { return None; }`), placed
on getIn(0) BEFORE the isConstantExtended branch — rejecting both a free getIn(0)
(no panic) and a bare-constant getIn(0) (a constant is never isWritten).
Re-verified by: the porter's `w10_divmod_findform_free_mult_input0_declines_not_panics`
(green) AND my independent `w10_indep_findform_const_mult_input0_declines`
(bare-constant leg, green). F2 (bare `as` casts) was a non-blocking style note;
the casts are correct C++ int4->uintb widenings and remain as-is.

No previously-passing assertion regressed; the round-1 ACCEPT-worthy bulk is
unchanged and re-verified below.

## Scope (closed seams on this branch)

The div/mod reciprocal-multiply reconstruction subset of LOSS-058:
`RuleDivOpt` (findForm GUARD A + full applyOp INT_DIV/INT_SDIV+correction),
`RuleDivTermAdd`/`RuleDivTermAdd2` applyOp commits, `RuleSubNormal` extension +
final-shift commits, `RuleMultNegOne`/`RulePositiveDiv`/`Rule2Comp2Sub`
opSetOpcode commits, `RuleSelectCse` full applyOp incl. the `isHeritaged`
predicate, the shared seam helpers (`isConstantExtended`, `newExtendedConstant`,
`typeop_for`/`op_set_opcode`, `new_unique_out`), the `RuleDoubleShift::clone_rule`
group fix, and the `push_constant_ir` integer-format force-mod. The OTHER in-scope
div/mod rules (`RuleSignForm`, `RuleSignDiv2`, `RuleDivChain`, `RuleModOpt`,
`RuleSignMod*`, `RuleShift2Mult`/`Rule2Comp2Mult`) live in ruleaction_7.rs /
ruleaction_3.rs, were ported in a prior wave, and are wired in universalaction.rs
— verified present, NOT touched by this branch.

## Mandatory hunt list

- Signedness: clean. NZ-mask sign-bit tests (`RulePositiveDiv` 7838/7840,
  `findForm` 8130) and `xsize`/`n` int4 widths transcribed exactly.
- Integer widths: clean. 128-bit math via kuna-num multiprecision; the
  `c/n/(n+1)/(out*8-1) as uintb` widenings are the C++ `newConstant(4,<int4>)`.
- Wrapping: clean. `calc_divisor` uses `wadd`/`wsub`/`wshl`/`wshr` exactly where
  C++ unsigned u64 wraps; `1u64<<63` / `xsize==64->0` branch faithful. Fuzzed
  shift-panic-free at xsize {0,7,63,64}.
- Comparator totality: n/a.
- Iteration-order provenance: clean. Every descend loop is a `SmallVec`
  insertion-ordered snapshot (mirrors C++ `list<PcodeOp*>` beginDescend); the
  `move_sign_bit_extraction` grow-while-iterate is an index walk + snapshot. No
  HashMap/HashSet.
- Off-by-one / do-while: clean. `RuleSubNormal` `c+=k; n-=k*8`, shrink-k
  `k=insize-c-outsize`, and the `n>=outsize*8` SRIGHT-`-1` clamp are line-exact.
- Erase-while-iterating: clean (snapshot-then-rewrite; `op_destroy` after rewrite
  in Rule2Comp2Sub).
- Exception->Result partial-state: the `seam_*().is_err() -> return 0` guards are
  dead in practice (op_set_opcode infallible; new_unique_out only errors on a real
  internal fault) — no half-applied transform in the wired paths.

## No-special-casing audit (central REJECT gate) — CLEAN

- ZERO hardcoded divisor/magic in src/ (grep of {81,89,91,125,0x948b…,0xaaaaaaab,
  0xcccccccd} over the 4 changed src files finds none; only rule-registration
  names + line-anchor comments hit "divopt"/"doubleshift").
- `calc_divisor` derives N from the general `2^n/(y-1)` magic-number math.
  Oracle-pinned against the compiled standalone upstream algorithm for NON-corpus
  magics: /3 (0xaaaaaaab), /5 (0xcccccccd), /9 (0x38e38e39), the maxx-failure
  reject (0x24924925 n34 -> 0), and the y<=1 / n>127 / xsize>64 guards (-> 0).
- No address/function-name/type-name hardcoding in the src diff.

## Real-parity verification (new passes genuine, not coincidental)

The divopt corpus renders the EXACT oracle div/mod statement shape
(`*a0=*a0/81;`, `a0[1]=a0[1]/89;`, `a0[2]=a0[2]/91;`, `a0[16]=a0[16]/125;`,
`*a0=*a0%81;`, `a0[16]=a0[16]%125;`) — full recurring-LHS assignments, not a
`/ 81` substring; the raw reciprocal magic 0x948b0fcd6e9e0653 and the wide
`0xffffffffffffffff` `* -1` mask are CONSUMED (absent). Confirmed by re-running
verify_w10_divmod, verify_w10_struct_corpus (incl. the two R2 reciprocal-multiply
gates), and the corpus stringmatch tally.

## Faithfulness confirmations (cpp <-> rust, both anchors)

- findForm GUARD A: cpp ruleaction.cc:8113 <-> rust ruleaction_6.rs:1654 (the F1
  fix; rejects free + bare-const getIn(0) before getDef()).
- applyOp ext/trunc/ZEXT-unsigned/SEXT-signed: cpp 8325-8369 <-> rust 1816-1900
  (op=newop aliasing + SDIV+SRIGHT+ADD correction faithful).
- DivTermAdd / DivTermAdd2: cpp 7863-7930 / 7984-8061 — ZEXT/SEXT vs RIGHT/SRIGHT
  guard, `shiftopc==MAX -> INT_RIGHT`, the `n+1` shift in TermAdd2, all faithful.
- calcDivisor: cpp 8172-8213 — y<=1 boundary, r>=q off-by-one adjustment, maxx
  feasibility check, faithful.
- isConstantExtended: cpp varnode.cc:818-863 — ZEXT/SEXT(sign_extend if <8)/
  PIECE(lo|hi<<, hi>>) verbatim.
- newExtendedConstant: cpp funcdata_varnode.cc:464 — ZEXT-of-const / PIECE(chi=
  val[1],clo=val[0]) MSB/LSB slots match.
- typeop_for opflags: every value matches typeop.cc (unary Z/SEXT; binary|
  commutative ADD/MULT; binary RIGHT/SRIGHT/DIV/SDIV/PIECE/SUBPIECE). The `_ =>`
  bare-binary fallback is a scoped SEAM(W6) unreached by these opcodes.
- RuleSelectCse heritage predicate (cpp 207 / funcdata.hh:274) and RuleDoubleShift
  "analysis" group (cpp ruleaction.hh:412-416 — clone tests contains(getGroup()),
  g=="analysis"; the old Rust tested the rule NAME "doubleshift", dropping it from
  every pool — a genuine pre-existing bug, now fixed): faithful.
- push_constant_ir force-mod: cpp printc.cc:1393-1405 precedence (NONE->forced;
  force_hex; val<=10||force_dec; mostNaturalBase) <-> resolve_integer_format;
  no-force path is byte-identical to before (boolless `10` preserved).

## findings

None blocking. (Round-1 F1 fixed and re-verified; round-1 F2 was a non-blocking
style note, casts are correct and left as-is.)

## adversarial tests (committed on rport/w10-divmod @ 8ae0c74)

rust/crates/kuna-decomp/tests/verify_w10_divmod_independent.rs (verifier-written,
separate from the porter's file; 4/4 green):
  - w10_indep_findform_const_mult_input0_declines        [GUARD A bare-const leg]
  - w10_indep_calc_divisor_guards_return_zero            [y<=1/n>127/xsize>64 oracle]
  - w10_indep_calc_divisor_div9_recovers_and_maxx_fail_rejects [general /9 + maxx]
  - w10_indep_doubleshift_clone_fires_for_analysis_group [the clone-group fix]

## losses

LOSS-171 (appended): the W6/W4-seamed REMAINDER of the LOSS-058 rule batch is NOT
un-seamed here — `RuleAddUnsigned`, `RuleSubRight`, `RulePtraddUndo`,
`RulePtrsubUndo`, `RulePtrsubCharConstant`, `RuleExtensionPush`,
`RulePieceStructure` still decline on `getTypeReadFacing`/`determineDatatype`/
`duplicateNeed` (W6) and the equate/string/scope arms (W4); and the
`typeop_for`-built TypeOp carries only `opflags` (no `OpBehavior`/`addlflags`) —
the documented W6 narrow-seam.
