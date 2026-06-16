# w10-doublemove
verdict: ACCEPT-WITH-LOSSES
verifier: claude-opus-4-8 (independent)
date: 2026-06-16
branch: rport/w10-doublemove (fb343db) — round 1
gate: cargo test -p kuna-decomp (lib 1729/0; verify_w6_s5_double 5/0) -> PASS;
      cargo test --workspace --no-fail-fast -> 3575 ok / 0 failed;
      cargo clippy -p kuna-decomp --lib -> clean (0 warnings/errors)

## Scope

Diff touches THREE rust-only files (no decompiler/cpp, specs/, Makefile, or
baseline): `double.rs`, `funcdata_varnode.rs`, `tests/verify_w6_s5_double.rs`.
It restores TWO of LOSS-084's four double-precision seams:

- `op_is_arith_or_float` (double.rs:4744): was a hard-coded `false` (W6 seam);
  now reads `TypeOp::isArithmeticOp()||isFloatingPointOp()` from the ported
  `type_op_info` addlflags table. Faithful to double.cc:3239 (RuleDoubleIn) and
  :3320 (RuleDoubleOut). Flag values match C++ exactly (arithmetic_op=8,
  floatingpoint_op=0x20; typeop.hh:45,47).
- `combine_input_varnodes` (funcdata_varnode.rs:1227, new): a verbatim
  transcription of `Funcdata::combineInputVarnodes` (funcdata_varnode.cc:383)
  — input/contiguity guards, PIECE-reader-vs-other partition, SUBPIECE
  replacement of non-PIECE readers via totalReplace, destroy-then-setInput, and
  the PIECE->COPY collapse. `combine_input_varnodes_seam` now routes to it.

## Faithfulness (verified line-by-line vs C++)

- `combine_input_varnodes` matches funcdata_varnode.cc:383-456 statement order:
  the isInput throw (386), the BE/LE contiguity arithmetic (389-398), the
  not-contiguous throw (399-400), the two descend partition loops keyed on
  `code()==PIECE && in0==vnHi && in1==vnLo` (405-416), `opRemoveInput(.,1)` then
  `opUnsetInput(.,0)` (417-420), the otherOpsHi/Lo SUBPIECE-of-whole builders
  with `newConstant(4,vnLo->getSize())` / `newConstant(4,0)` + totalReplace
  (423-442), `outSize=hi+lo`, destroy both, `setInputVarnode(newVarnode)`
  (443-447), and the PIECE->COPY + subHi/subLo rewire (448-455). The C++ throws
  map to `KunaError::lowlevel` Result returns at the identical pre-mutation
  points; `combine_input_varnodes_seam`'s `.expect()` mirrors the uncaught C++
  throw and is dead given the caller's `isInput`+`isAddrTiedContiguous` guards
  (the internal re-check is provably equivalent to the caller's, double.cc:789).
- `RuleDoubleOut::apply_op` (double.rs:4952) is unchanged in control flow and
  matches double.cc:3332-3355.

## Mandatory hunt list

- Signedness: clean. The only width-relevant casts are `hi_size/lo_size as i64`
  (int4->int8, == C++ getSize() promoting into Address::operator+(int8)) and
  `lo_size as uintb` (== C++ int4->uintb in newConstant). Sizes are small
  non-negative; sign-extension is harmless and matches C++.
- Integer widths: clean. `out_size = hi_size + lo_size` is int4+int4, == C++
  `int4 outSize` (444); no promotion divergence.
- Wrapping: clean. Address `+ i64` uses `wadd`/`wrap_offset`, == C++
  `wrapOffset(offset+off)`. No other arithmetic on uintb-lineage values.
- Comparator totality: N/A (no comparators in scope).
- Iteration-order provenance: clean. `descend_snapshot` collects the
  insertion-ordered `SmallVec` `descend` (== C++ `list<PcodeOp*>` order); both
  partition loops and both later index-loops over `piece_list` preserve that
  order, == the C++ `for(i...)` indexing. No HashMap/HashSet.
- Off-by-one / do-while / reverse iterators: clean. No do-while; the descend
  loops are forward; the contiguity `==` boundary is the accepted edge (proven
  by the collapse adversarial test).
- Erase-while-iterating: clean. Snapshot-then-mutate (Vec) replaces the C++
  in-place descend traversal; the partition loops are read-only, so the same
  ops are visited in the same order before any opRemove/opUnset mutates them.
- Exception->Result partial-state parity: clean. Both early Errs (non-input,
  non-contiguous) precede ALL graph mutation, == the C++ throws at 387/400;
  proven by `..._rejects_non_input_no_mutation` (num_varnodes unchanged).

## Mechanical pass

- grep (production diff): NO function-name/address/value special-casing; NO
  HashMap/HashSet/sort_unstable/todo!/unimplemented!/panic!. The only literals
  are `4`/`0`/`vnLo->getSize()` directly transcribed from the C++ newConstant
  calls. Three bare `as` casts WITHOUT `// cast:` comments (F2, cosmetic).
- clippy -p kuna-decomp --lib: clean.

## No-regression evidence

- Apples-to-apples passing-SET diff base(merge-base 30cf0b5) vs branch:
  the ONLY base-pass-not-on-branch is `verify_w6_s5_double_arith_whole_marking_
  disabled_by_w6_seam` — which the porter INTENTIONALLY renamed to `_restored`
  and flipped (0->1) to reflect the now-faithful marking. ZERO genuine
  regressions. (The 85 base "failures" are environmental: the fresh base
  worktree lacks built `.sla` specs — "Could not find .sla file"; they all PASS
  on the branch worktree, which has artifacts.)
- New PASS is REAL parity: `verify_w6_s5_double_arith_whole_marking_restored`
  asserts attemptMarking returns 1 + hi.isPrecisHi() for a written INT_ADD
  whole — re-run green, matching the C++ oracle (double.cc:3238-3256). INT_ADD
  is arithmetic in the table (independently pinned, typeop.cc:1172).
- boolless / readstruct / condconst_conn: all byte-parity tests
  (`w10_byte_identical_boolless_*`, `w10_dc_condconst_conn_*`,
  `w10_eob_condconst_conn_*`, readstruct symbol-naming) PASS on the branch;
  `corpus_functions_byte_compare_against_cpp_oracle` PASS.
- C++ oracle 675/675 + B0 / universalAction: byte-untouched (rust-only diff —
  no C++/spec/datatest file changed).
- cargo test --workspace --no-fail-fast: 3575 ok / 0 failed.

## findings

  - F1 (LOSS, accepted): `RuleDoubleOut::attemptMarking` still OMITS the
    `Varnode::getSymbolEntry` mismatch reject (double.cc:3306-3313). LOSS-084's
    restoration criterion (1) required this to be co-restored with
    `op_is_arith_or_float`; the W4 symbol surface is genuinely unported
    (`get_symbol_entry` grep-absent), so it could not be. After this branch the
    omission's prior "unreachable because arith/float always declines"
    justification is STALE (arith/float can now fire), but it remains
    non-observable: reachability now rests on the persist gate, and the
    passing-set diff + corpus byte-compare show ZERO divergence. Cited as the
    accepted divergence (new LOSS-206; carried-forward from LOSS-084).
       cpp: decompiler/cpp/double.cc:3306-3313
       rust: rust/crates/kuna-decomp/src/double.rs:4917-4921
  - F2 (nit): three bare `as` casts in the new code lack the protocol-required
    `// cast:` justification comment. Each is a correct, faithful width/sign
    resolution (int4->int8 for Address+, int4->uintb for newConstant), so this
    is cosmetic, not behavioral.
       rust: rust/crates/kuna-decomp/src/funcdata_varnode.rs:1250,1255,1306

## adversarial tests (committed, branch 08000d2)

  - combine_input_varnodes_w10dm_rejects_non_input_no_mutation
        (funcdata_varnode.rs — isInput guard + Exception->Result partial-state)
  - combine_input_varnodes_w10dm_rejects_noncontiguous_le
        (funcdata_varnode.rs — LE contiguity arithmetic, pre-mutation reject)
  - combine_input_varnodes_w10dm_le_piece_collapses_to_copy
        (funcdata_varnode.rs — happy path: destroy originals, size-8 input at
         lo addr, PIECE->COPY, num_input==1)
  - w10_doublemove_arith_or_float_classification
        (typeop/tests.rs — the restored isArithmeticOp||isFloatingPointOp
         disjunction: INT_ADD/MULT/SUB + FLOAT_ADD/MULT true; COPY/PIECE/
         SUBPIECE/INT_AND/INT_OR false)

All four pass; no porter unit test existed for the `combine_input_varnodes`
body, so these are its primary coverage.

## losses

LOSS-206 (appended) — the carried-forward `getSymbolEntry`-mismatch-reject
omission in RuleDoubleOut::attemptMarking; supersedes the now-stale
unreachability justification in LOSS-084's rust-anchor.
