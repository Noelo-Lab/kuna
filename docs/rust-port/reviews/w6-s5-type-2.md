# w6-s5-type-2
verdict: ACCEPT-WITH-LOSSES
verifier: claude-opus-4-8 (independent verifier subagent)
date: 2026-06-12
round: 1
gate: `cd rust && cargo test -p kuna-decomp` -> PASS for every in-scope suite
      (lib 938/938; the four dtype verifier targets green: verify_w5_dtype_expand
      9/9, verify_w6_s5_type_1 7/7, verify_w6_s5_type_2 21/21, and the new
      verify_w6_s5_type_2_adversarial 8/8). The ONLY failing test in the whole
      `-p kuna-decomp` run is `corpus_bootstrap::ldefs_pspec_cspec_parse_matrix`,
      which is environmental and out of scope: it calls `build_spec_file(x86:...)`
      and fails with "Could not find .sla file" because `.sla` are gitignored
      build artifacts and the worktree has *zero* of them (`find . -name '*.sla'`
      = 0; the MAIN tree, where this branch's gate would run with built specs,
      has them). The failing test does not touch dtype / TypeStruct / TypeUnion /
      TypeEnum and is unrelated to this port.
blob-sha: type.cc 9014a307b1c222be9414705e2151d5f50b29a23b (matches recorded),
          type.hh 0bafaef240058a69328f71a2ea632c661efef7ef (matches recorded) —
          no drift.

## Scope

type.cc lines ~1723-3564 (plus the TypeEnum bodies at 1518-1721 that the porter
pulled into this item): TypeStruct (setFields, getFieldIter, findMatchingBitField,
getLowerBoundField, findTruncation, getSubType, getHoleSize,
nearestArrayedComponent{Forward,Backward}, compare/compareDependency,
findCompatibleResolve, assignFieldOffsets + assignContiguousBitfields),
TypeUnion (setFields, compare/compareDependency, findCompatibleResolve,
assignFieldOffsets), TypeEnum (hasNamedValue, getMatches, compare/compareDependency,
assignValues), TypeCode (compareBasic surface + compare/compareDependency surface),
TypeSpacebase (compare/compareDependency spaceid+localframe tie-break), and the
TypePartialStruct/Union/Enum compare/compareDependency/getSubType/getHoleSize
shapes. The Funcdata/FuncProto/TypeFactory/Scope-backed paths are correctly left
as SEAM(W6) and ledgered (LOSS-082).

## Hunt list

- **Signedness** — clean. `getFieldIter`/`getLowerBoundField` keep the C++
  `int4 max = size()-1` (so an empty list gives max=-1 and the loop never runs);
  the `i < field.size()` `int4`-vs-`size_t` compares (with `-Wno-sign-compare`)
  are reproduced as `(i as usize) < len()` *only after* i is proven >= 0 (i+=1
  from a >= -1 lower bound). The int8-`off` narrowing to int4 for the binary
  searches (`off as int4`) faithfully mirrors the C++ implicit narrowing in
  `getSubType`/`getHoleSize`/`findTruncation`/`nearestArrayedComponent*`, and is
  documented at each site.
- **Integer widths** — clean. uintb=u64, int4=i32, int8=i64 mapped consistently;
  `namemap` is `BTreeMap<u64,String>` (matches `std::map<uintb,string>`).
- **Wrapping** — clean for the output-determining paths. The `op.size - size`
  field/namemap-count finishers and base-compare size deltas all use
  `wrapping_sub` (faithful to C++ `int4`/`size_t` wrap-then-truncate; for the
  realistic <2^31 counts the low-32-bit result is identical either way). The
  enum shifts `val <<= 8*offset` use `wrapping_shl` (C++ `<<` is UB for >=64;
  real partial-enum offsets keep 8*offset < 64, so they agree on every reachable
  input). `maxval += 1` in assignValues uses `wrapping_add` (faithful to the C++
  uintb wrap in the unassigned-value search). Observation (non-blocking): the
  layout adds `offset += cursize`, `total_size += num_bits`, `lsb += num_bits`
  are plain `+=` (debug-panic on i32 overflow where C++ silently wraps); these
  are bounded by real struct layout and cannot overflow for any well-formed
  type — noted, not a finding.
- **Comparator totality** — clean. Every ported comparator is a total order:
  the struct/union/enum/partial/spacebase compares chain submeta -> size ->
  field/namemap layout -> offset/localframe -> id, all on `int4`/`u64`/`String`
  (`Ord`). The raw-pointer comparators (`compare_dependency_ptr`,
  `compare_spacebase_space`) reproduce the C++ `ptr < ptr` order via
  `Rc::as_ptr as usize` — total within a run, with the SAME cross-run
  nondeterminism the C++ has (documented at the site; this is the faithful port,
  not a defect). No floats, no partial_cmp().unwrap().
- **Iteration-order provenance** — clean. The only map iterated is `namemap`,
  a `BTreeMap` (key order == `std::map` order) in `compare_dependency_enum`,
  `enum_get_matches` (the `upper_bound`/predecessor walk is reproduced with
  `range((Excluded(target),Unbounded)).next()` for upper_bound and
  `range((Unbounded,Excluded(k))).next_back()` / `iter().next_back()` for the
  `--iter` predecessor, including the `iter==begin()` and `--end()` boundaries —
  re-derived by hand and pinned by three new adversarial tests). Field/bitfield
  loops are `Vec` index/insertion order, matching the C++ vectors. No
  HashMap/HashSet anywhere in dtype.rs.
- **Off-by-one / do-while / reverse iterators** — clean. `getLowerBoundField`'s
  asymmetric `(min+max+1)/2` upper-rounding is transcribed verbatim and stressed
  by a new test; the `TypePartialStruct::getSubType` `do {} while` is rendered as
  `loop { ...; if cond { break } }` keeping the at-least-once semantics
  (the C++ enters the loop once before testing `ct->getSize()-off > sizeLeft`);
  `findTruncation`'s `noff+sz > field->getSize()` span check is exact;
  `assignContiguousBitfields`'s big-endian `std::reverse(begin+startInd, begin+pos)`
  is `bitlist[start..pos].reverse()` (pinned by a new test).
- **Erase-while-iterating** — N/A. No element erasure during traversal in this
  range (decodeField/decodeBitField's `pop_back` of an overlapping field is in
  the decode path, which is SEAM(W6) here, not ported).
- **Exception -> Result partial-state parity** — clean. The ported `throw`s
  (void-field rejection in assignFieldOffsets/assignUnionFieldOffsets, malformed
  bitfield, duplicate enum value) are `Err` returns at the same points; each
  fires before the offending mutation is committed, matching the C++ throw sites.
  The SEAM(W6) `Err`s are pure deferrals returned before any mutation.
- **Datatype::compare/compareDependency total-order fidelity** (wave add-on) —
  clean. Verified the partial-type dispatch cannot mismatch kinds: the base
  compare/compareDependency submeta step runs first, and PARTIALSTRUCT/UNION/ENUM
  carry distinct submetas, so `as_partial_whole()` is only reached when both
  operands are the same partial kind. TypeCode/TypeSpacebase/Struct/Union/Enum
  invariant accessors (`as_*`) all guard against a kind mismatch with an `Err`
  rather than a wrong-cast UB (an improvement over the C++ blind `(T*)&op` cast,
  unreachable when the submeta invariant holds).
- **ParamEntry/ParamTrial sort orders** (wave add-on) — N/A (no fspec types in
  this item's scope).
- **TypeFactory cache iteration determinism** (wave add-on) — N/A here: the
  TypeFactory construction caches (`tree`/`nametree`/`typecache`) and decode are
  correctly SEAM(W6) (LOSS-082's neighborhood); only the comparators that *feed*
  those caches are in scope, and they are total + deterministic per the above.

## Mechanical pass

- port-audit: no `port-audit` crate exists in the tree (the protocol's
  `cargo run -p port-audit` is not yet wired); blob-sha drift checked manually
  against the recorded checklist shas (no drift, see above).
- greps over dtype.rs: `HashMap`/`HashSet` = none; `todo!`/`unimplemented!`/
  `unreachable!`/`panic!` (non-test) = none; `sort_unstable` = none;
  `partial_cmp` = none; all `.unwrap()` are in `#[cfg(test)]` blocks. Bare `as`
  casts in the type-2 source range are width-narrowing/index conversions
  (`len() as int4`, `i as usize`, `off as int4`) that transcribe C++ implicit
  conversions and are documented inline at each site (the project uses inline
  prose rather than a literal `// cast:` token, consistent with type-1).
- clippy: `cargo clippy -p kuna-decomp -- -D warnings` -> clean (lib). The new
  adversarial test target is independently clippy-clean
  (`--test verify_w6_s5_type_2_adversarial -- -D warnings`). NOTE: a tree-wide
  `cargo clippy --tests` surfaces lint debt in *other, pre-existing* test files
  (heritage.rs, verify_w4_fw_architecture_r2.rs, verify_w3_ir_funcdata_varnode.rs,
  verify_w5_s3_rules_7.rs) — none touched by this branch.

## Adversarial pass

Added `rust/crates/kuna-decomp/tests/verify_w6_s5_type_2_adversarial.rs`
(committed to the branch, 8 tests, all green):

- `struct_binary_search_with_holes` — getSubType/getHoleSize/findTruncation over
  a struct with a padding hole between fields (getFieldIter must return -1 in the
  hole; getLowerBoundField must return the last at-or-before field even when it
  does not contain the offset; findTruncation span/None boundaries).
- `struct_lower_bound_before_first_and_empty` — leading hole (lower-bound -1) and
  the empty-struct degenerate search (max=-1, loop body skipped).
- `struct_lower_bound_rounding` — six packed 1-byte fields, every offset probed,
  forcing the `(min+max+1)/2` upper-rounding to converge correctly.
- `enum_getmatches_restrict_branch` — the `else` "restrict" path
  (`target = curval & ~diff`) plus the subsequent `diff >= bitsleft` break;
  oracle hand-derived (and the test caught MY first wrong oracle: 0x07 is NOT
  greedily representable as FIVE|TWO, and the port correctly returns empty —
  confirming the else-branch/break interplay is faithful).
- `enum_getmatches_begin_boundary` — target smaller than every named value
  (upper_bound==begin(), predecessor walk yields nothing -> break) + the
  zero-special-case fallthrough to complement.
- `enum_getmatches_end_boundary` — target greater than every named value
  (upper_bound==end(), predecessor is the last entry) + the complement path.
- `assign_field_offsets_interleaved_bitfields_bigendian` — a contiguous bitfield
  group landing *between* two regular fields (the `nextBitPos` machinery) with the
  big-endian post-assignment `reverse`.
- `assign_field_offsets_bitfields_littleendian_no_reversal` — the same layout
  little-endian (no reversal; LSB order + ident reset preserved).

## Findings

None blocking. One non-blocking observation (layout `+=` debug-overflow vs C++
silent wrap, unreachable for well-formed types) is recorded in the hunt list and
is not a faithfulness defect.

## Losses

LOSS-082 (new; appended): the type-2 structured-override *resolution* surface is
W6-deferred — `TypeStruct`/`TypeUnion`/`TypePartialUnion`
`resolveInFlow`/`findResolve`, the `TypeUnion`/`TypePartialUnion`
`findTruncation`/`resolveTruncation`, `TypeCode::compare`/`compareDependency`
parameter recursion + `getSubType`, and `TypeSpacebase::getSubType`/
`nearestArrayedComponent{Forward,Backward}` (and `printRaw`) SEAM to
`Err(SEAM(W6))` where C++ does the Funcdata/FuncProto/TypeFactory/Scope-backed
work. Distinct from LOSS-050 (the base "return this", restored by type-1). The
compare/compareDependency/getSubType/getHoleSize/nearestArrayedComponent/
findCompatibleResolve/setFields/assignFieldOffsets/getMatches/assignValues
layout-and-ordering bodies ARE ported and tested.
