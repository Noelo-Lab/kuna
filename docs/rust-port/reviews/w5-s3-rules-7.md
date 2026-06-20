# w5-s3-rules-7
verdict: ACCEPT-WITH-LOSSES
verifier: independent-verifier (round 2)
date: 2026-06-11
gate: `cd rust && cargo test -p kuna-decomp` -> PASS (451 lib + verify suites;
      verify_w5_s3_rules_7 = 11 tests, all green incl. 5 new round-2 tests)

## Scope

C++: `decompiler/cpp/ruleaction.cc:8374-9803`, the 15 classes
`RuleSignDiv2` .. `RuleIgnoreNan` + their `ruleaction.hh` declarations.
Rust: `rust/crates/kuna-decomp/src/ruleaction_7.rs` (+ `ruleaction_7/tests.rs`,
`tests/verify_w5_s3_rules_7.rs`). (`dtype.rs` in the diff belongs to the parallel
`w5-dtype-expand` item; not reviewed here.)

## Round-1 -> round-2 disposition

Round 1 REJECTED on F1/F2/F3: three variable-count shifts used raw `<<`/`<<=`
where the C++ build relies on x86 shift-count masking (`count & 63`), so a
shift >= 64 (reachable: a degenerate inner `>> 64`, or an 8-byte sign-extracted
`a` with root shiftAmt 0 giving `n==64`, or a >8-byte `shiftvn`) **panics in
debug** where C++ wraps and proceeds. The round-2 repair commit (`bd40702`)
replaced all four sites with `Wrap::wshl` (= `wrapping_shl`, masks count modulo
64), matching the x86 codegen exactly:

- F1 RuleDivChain  ruleaction_7.rs:350  `(1u64).wshl(sa as u32)`
- F2 RuleSignMod2nOpt :953  `(1u64).wshl(n as u32) - 1`  (masked `1<<k >= 1`, no underflow)
- F3 RuleSignNearMult :698 `mask.wshl(n as u32) & mask`, :729 `(1u64).wshl(n as u32)`

Re-verified: grep finds NO remaining raw variable-count `<<`/`>>` in the file
(the only `<<`/`>>` matches are `clone_rule`'s `Box<dyn Rule>` syntax). The two
round-1 `#[should_panic]` tests were flipped to assert the C++ oracle outcome
(`w5s3_divchain_right_shift64_wraps_like_cpp` fires+collapses;
`w5s3_signmod2nopt_shiftamt0_returns_zero_like_cpp` returns 0). All three
blockers are CLEARED.

`wshl` faithfulness re-derived for negative `n` too: `n as u32` reinterprets the
i32 bit pattern, `wshl` masks modulo 64 -> same low-6-bits the x86 `shl` uses for
a negative count. Matches C++ for every count value.

## Hunt list

- **Signedness** — clean. All `int4` vs `uintb`/size comparisons faithful; the
  `(... as int4)` narrowings mirror the C++ `(int4)getOffset()` casts. The
  `bitcount > sz*8` (DIV) vs `> sz*8-2` (SDIV) split verified
  (`w5s3_divchain_div_at_limit_fires_sdiv_rejects`).
- **Integer widths** — clean. `uintb->u64`, `int4->i32` per ADR-0003; the
  `val1*val2` / `(div2^mask)+1` / `(~off+1)` use `wrapping_*` (366,373,816,1472);
  `npow-1` (1338) cannot underflow (caller guarantees npow>=2 via
  popcount==1 && !=1).
- **Wrapping** — clean (F1/F2/F3 fixed). Every uintb-lineage arithmetic op is
  `wrapping_*`/`wshl` or proven in-range.
- **Comparator totality** — n/a (no comparators ported; the one
  `sort(ops,compareOp)` is inside the seamed `RuleConditionalMove` tail).
- **Iteration-order provenance** — clean. Every descend loop is
  `vn_descend()` = `descend_iter().collect()` (the C++ live
  `beginDescend()..endDescend()` order). No `HashMap`/`HashSet`. Block loops
  index `getIn(i)` directly.
- **Off-by-one / do-while / reverse iterators** — clean. The `for(i=0;i<2;++i)`
  slot searches are `while i<2` with the post-loop `if i==2`/`if slot>1`
  sentinels; `RuleSignForm2`'s deliberate trailing `return 0` (C++:8563, after
  `opSetInput(op,a,0)`) is transcribed faithfully and now PINNED by a verifier
  test that asserts the input IS rewired while the rule reports 0.
- **Erase-while-iterating** — clean. Rules mutate during descend traversal; the
  snapshot-to-Vec is the arena-safe equivalent and matches the single-fire
  `return 1` exit (no rule continues iterating after a mutation here).
  `RuleIgnoreNan`'s nested advance-before-mutate live-iterator dance is NOT
  ported (whole rule is a W4 seam), so no erase hazard.
- **Exception -> Result partial-state parity** — one note (NOT a finding,
  ledgered as LOSS-075): `RuleSignNearMult` builds `newdiv` + sets its opcode
  before the W3-output `op_set_output` seam `Err`, then returns 0 — leaving an
  inert orphan op. The C++ path never fails. This is the LOSS-035/067
  `newUniqueOut`-after-`newOp` partial-state class; orphan is inert/bounded.
  `RuleSegment`'s C++ `throw LowlevelError` is inside a fully-seamed no-op body.
- **AND-rule traps (transposed conditions / swapped slots / inverted returns /
  missing early-outs)** — clean. Re-derived `RuleModOpt`, `RuleSignMod2nOpt`,
  `RuleSignMod2Opt`, `RuleSignMod2nOpt2` (incl. `checkSignExtForm` /
  `checkMultiequalForm` block topology), `RuleSignDiv2`/`RuleSignNearMult` slot
  loops, `RuleSignForm2`'s SEXT search, `RuleConditionalMove::checkBoolean`,
  `RuleFloatCast` arm-by-arm: slot indices (`1-i`, `1-slot`, `1-aSlot`,
  `1-multSlot`, `1-innerSlot`), the `div2==outvn ? getIn(0)` / `lvn==outvn2 ?
  getIn(1)` fixups, the `(div2^mask)+1` 2's-complement check, the
  `1-getSlot(multOut)` baseOp slot, the `base`/`otherBase` SUBPIECE-trunc swap,
  and the `getTrueOut`/`getFalseOut` + `negSlot==slot` decision all match.
  Round-2 ADVERSARIAL tests added positive-firing coverage of the SREM-family
  slot logic (round 1 had none): `RuleSignMod2nOpt` -> `a s% 4`,
  `RuleSignMod2Opt` -> `a s% 2`, `RuleSignForm2` match-but-return-0 — all match
  the C++ oracle. `change_opcode`/`set_opcode` (op.rs:458) correctly transfer the
  `typeop_for` flags (incl. `booloutput`) to the PcodeOp flag word, so
  `is_bool_output()` is faithful where the matching uses it.

## Findings

None standing. F1/F2/F3 from round 1 are resolved (see disposition above);
re-verified clear.

## Seam losses (deferred, ACCEPT-WITH-LOSSES — appended to MAIN losses.md)

These are faithful cross-wave deferrals consistent with the LOSS-053/054/062
precedent: the rule matches as far as the available IR surface allows, then bails
at an unported W3-output / W4 / W6 surface; never a silent mis-port. `RuleFloatCast`
is NOT seamed (all four arms fire). Ledgered:

- **LOSS-074** — `RuleSegment` (W4: userops/segment/getSpaceFromConst),
  `RulePtrFlow` (W4 default-space + W3-output `truncatePointer`; ctor
  `has_truncations=false`), `RuleConditionalMove` (W4 `CloneBlockOps`/
  `opBoolNegate`), `RuleIgnoreNan` (W4 `nan_ignore_all`/`isFloatingPointOp`/
  `functionalEquality`) — read-only no-ops, no partial mutation.
- **LOSS-075** — `RuleSignNearMult` (+ `RulePtrFlow::truncatePointer`)
  `newOp`-then-`newUniqueOut` W3-output seam: full match builds an orphan
  INT_SDIV op then returns 0 (inert partial state, LOSS-035/067 class).

## Adversarial tests added (round 2)

`rust/crates/kuna-decomp/tests/verify_w5_s3_rules_7.rs` (committed `eb89dae`):
- `w5s3_signmod2nopt_positive_nontrunc_fires_to_srem` (1-getSlot/aSlot, mask+1)
- `w5s3_signmod2nopt_rejects_when_mask_constant_wrong` (AND-mask guard)
- `w5s3_signform2_matches_but_returns_zero_and_rewires_in0` (inverted-return)
- `w5s3_signform2_rejects_when_const_too_large` (multiplier-size guard)
- `w5s3_signmod2opt_positive_nontrunc_fires_to_srem2` (multSlot, getSlot(andOut))

(plus the 6 round-1 tests, with the two `#[should_panic]` flipped to oracle
asserts). All 11 pass.

## Mechanical pass

- ruleaction_7.rs: no `todo!`/`unimplemented!`/`HashMap`/`HashSet`/`sort_unstable`.
- bare `as` casts are all `uintb->int4` / `int4->uintb`/`->u32` mirroring C++
  `(int4)` narrowings / x86 shift-count coercion (faithful; self-documented by
  adjacent C++-quoting comments). Minor style nit (no `// cast:` token), not a
  finding.
- `cargo clippy -p kuna-decomp --lib -- -D warnings`: clean.
- `cargo clippy -p kuna-decomp --tests -- -D warnings`: pedantic doc-list-format
  lints fire across MANY pre-existing verify_* test files (w3_ir_op,
  w4_fw_options, w3_ir_jumptable, etc.) incl. the round-1 module docstring in
  verify_w5_s3_rules_7.rs; project convention runs `--lib`. NOT introduced by
  this item's code (my added tests use `//` line comments). Non-blocking.
- gate `cargo test -p kuna-decomp`: green.
