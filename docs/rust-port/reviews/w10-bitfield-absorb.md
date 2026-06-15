# w10-bitfield-absorb
verdict: ACCEPT
verifier: independent verifier (Opus 4.8)
date: 2026-06-15
gate: `cargo test --workspace` -> green (0 failed); `cargo test -p kuna-decomp` -> green; `cargo clippy -p kuna-decomp --lib` -> clean (0 warnings); C++ oracle `python -m kuna.run_tests --all --baseline docs/baseline.json` -> 207/207 unit + 675/675 datatest, PARITY OK (untouched — branch is Rust-only)

## Scope

Branch `rport/w10-bitfield-absorb` ports the two POST-materialization bitfield
simplification rules out of their `return 0` stub state:

- `RulePullAbsorb` (bitfield.cc:1767-2208) — `absorbRight`/`absorbRightAndCompZero`/
  `absorbLeft`/`absorbLeftRight`/`absorbLeftAnd`/`absorbAnd`/`absorbCompare`/
  `absorbExt`/`absorbSubpiece`/`absorbCompZero`/`applyOp`.
- `RuleInsertAbsorb` (bitfield.cc:2214-2400) — `leftShiftVarnode`/`absorbAnd`/
  `absorbRightLeft`/`absorbShiftAdd`/`absorbNestedAnd`/`applyOp`.

Diff touches exactly two files: `rust/crates/kuna-decomp/src/bitfield.rs` (the two
`apply_op` bodies now delegate to the submodule) and the new
`rust/crates/kuna-decomp/src/bitfield/absorb.rs` (1106 lines). Both rules are
already registered in `universalaction.rs` (rrows `pull_absorb`/`insert_absorb`,
present in the base) — so this branch turns two registered-but-inert rules into
live transforms. NO reserved file touched (merge.rs / funcdata.rs / database.rs /
coreaction_cleanup.rs / printc.rs / varmap.rs all unmodified — confirmed by
`git diff --name-only rust-port...branch`).

## Faithfulness (line-for-line against the cited C++)

Every helper is a faithful transcription of the C++ named in its doc comment.
Cross-checked the load-bearing details:

- `absorbExt` grows a 1-input EXT to 3 inputs with `op_set_input(slot0)` then
  `op_insert_input(slot1)`+`op_insert_input(slot2)`; `absorbSubpiece`/`absorbCompZero`
  (NOTEQUAL) grow a 2-input op with `set(slot0)`+`set(slot1)`+`insert(slot2)`. The
  set-vs-insert asymmetry between these is preserved exactly (cpp:2078-2083 vs
  2102-2107 vs 2150-2155).
- `absorbCompZero` EQUAL path: the endian-adjusted 1-byte output rebuild
  (`opUnsetOutput`→`newVarnodeOut(1,smalladdr)`→`getBase(1,BOOL)`/updateType→
  `opSetInput(comp,newVn,0)`→`deleteVarnode`), then unconditional
  `opSetOpcode(BOOL_NEGATE)`+`opRemoveInput(1)`, in the C++ order.
- `absorbCompare` BOOL_NEGATE path: `opRemoveInput(0)`→`opSetOpcode(BOOL_NEGATE)`→
  `opSetInput(pullOut,0)`→`destroy(inVn)`, order-exact. The LESSEQUAL boundary
  arithmetic (slot-0 `lowBits∈{0,1}` vs slot-1 `lowBits∈{0,mask}`) preserves the
  genuine C++ asymmetry; the `+1>>sa; (-1)&mask` / `-1>>sa; (+1)&mask` conversions
  match.
- `leftShiftVarnode`: INT_MULT matches `1<<sa`, INT_LEFT matches `sa` directly —
  the distinction is preserved (cpp:2222-2229).
- `absorbNestedAnd`: `bitsize` hoisted out of the C++ in-loop read is equivalent
  (`insertOp->getIn(3)` is loop-invariant); coveringmask/popcount gate intact.

## Mandatory hunt list

- Signedness: CLEAN. Every `uintb getOffset()` -> `int4` narrowing matches a C++
  implicit/explicit `(int4)` cast (e.g. `sa = cvn->getOffset()`). `sa < 8*sizeof(uintb)`
  C++ -> `sa < UINTB_BITS(=64)`. No signed/unsigned comparison hazard.
- Integer widths: CLEAN. `UINTB_BITS=64` = `sizeof(uintb)*8`; `newConstant` sizes
  taken from the correct varnode (`vn0->getSize()` -> `vn0_size`, `rightcvn->getSize()`
  -> `rightcvn_size`).
- Wrapping: CLEAN. `absorbCompare` mask `(1<<sa)-1` and the LESSEQUAL +/-1 use
  `wrapping_sub`/`wrapping_add` matching C++ unsigned wrap. The `match_val <<= bitsize-1`
  (absorbAnd) and `mask << sa` (absorbCompare) rely on the PULL-width>=1 / `numbits+sa==sz`
  invariant exactly as the C++ does (C++ has the same UB exposure if width==0); both
  shift amounts are <= 63 in every reachable path. Not a port-introduced defect.
- Comparator totality: N/A — no comparator ported here.
- Iteration-order provenance: CLEAN. C++ `list<PcodeOp*> descend` -> Rust
  `SmallVec<OpId>` in insertion order; `descend_of` snapshots into a Vec, justified
  because every absorb* helper returns on first success WITHOUT a prior graph
  mutation (a `0` return leaves the descend list unchanged), so the snapshot visits
  the same ops in the same order. No HashMap/HashSet (grep clean).
- Off-by-one / do-while / reverse iterators: CLEAN. No do-while; `numbits-1`/`bitsize-1`
  are i32 (no underflow panic for width>=1); no rbegin/lower_bound idioms.
- Erase-while-iterating: CLEAN. Helpers mutate-then-return-1; the outer snapshot is
  not re-traversed after a mutation.
- Exception -> Result partial-state: CLEAN. The `.ok()` on
  `op_set_input`/`op_insert_input`/`destroy_varnode_recursive`/`total_replace`
  discards a `KunaResult<()>` the C++ void functions never had; the only error path
  (`add_descend`) cannot fail in these reachable cases. Consistent with every sibling
  ported rule. The early-return guards (`None => return 0`) leave the graph
  un-mutated, matching the C++ short-circuit returns.

## Special-casing audit

NONE. `grep -niE 'field[0-9]|field5|field7|0xff|0x7f|"main"|->name|strcmp|== "'`
finds only doc-comment text transcribed from the C++ (`0xff` in the absorbNestedAnd
doc example). Every offset/width/mask is read live from op const inputs
(`in(1)`/`in(2)` of the pulls, `in(2)`/`in(3)` of INSERT) or computed by the shared
`coveringmask`/`popcount`/`calc_mask` primitives. No field name, mask literal,
offset, or function name is hardcoded.

## Mechanical pass

- `grep` todo!/unimplemented!/HashMap/HashSet/sort_unstable -> none.
- bare `as` casts: all 21 are `as int4`/`as uintb`/`as i64` width casts matching the
  C++ integer casts; no unjustified bare `as`.
- `cargo clippy -p kuna-decomp --lib` -> clean. (A pre-existing clippy nit at
  heritage.rs:3122, surfacing only under `--tests`, is in a file this branch does NOT
  touch and is unrelated.)

## No-regression

- Passing-SET diff base(`rust-port`) -> branch for `kuna-decomp` (`--no-fail-fast`):
  base-only (= regressed/disabled passes) is EMPTY. Branch adds exactly the 11
  `w10bfa_*` adversarial tests (other branch-only entries are `.sla`-dependent corpus
  tests the freshly-checked-out base worktree lacked built specs for — a build-artifact
  difference, not a code change).
- Byte-parity: `printc_parity` -> 1 passed.
- C++ oracle: `git diff -- decompiler/cpp/` empty; 675/675 datatest + 207/207 unit,
  PARITY OK. The `bitfields2` B5 datatest (`Bitfields #1..#31`, incl.
  `ip->field5 = ip->field5 + 1` #31, `field5 = val + 0x14` #4, `field7 = 100` #5,
  `field5 + field2` #7) all pass in the C++ reference — the parity target these
  Rust transforms move toward. (The full end-to-end `bitfields2` render is NOT yet
  demonstrable in the Rust port because the upstream decompilation passes remain seam
  stubs per LOSS-130/LOSS-193; this item ports one necessary piece faithfully, proven
  live by the graph-driven tests below. This is NOT a new loss for THIS item — it is
  the already-recorded LOSS-130 pipeline gap, and no doc comment in this diff
  overstates coverage.)

findings: none (no correctness findings).

## Adversarial tests (committed to the branch, effd91a)

`rust/crates/kuna-decomp/tests/verify_w10_bitfield_absorb_r1.rs` — 11 tests driving
the REAL `RuleInsertAbsorb`/`RulePullAbsorb` end-to-end on hand-built p-code graphs,
each asserting both the positive rewrite and the negative guard:

- `w10bfa_insert_and_full_mask_absorbs` / `..._partial_mask_does_not_absorb` —
  absorbAnd LSB-mask gate `(mask&val)==mask`.
- `w10bfa_shiftadd_poweroftwo_absorbs` / `..._nonpoweroftwo_does_not_absorb` —
  leftShiftVarnode `1<<sa` match (via absorbShiftAdd).
- `w10bfa_rightleft_narrow_field_absorbs` / `..._wide_field_does_not_absorb` —
  absorbRightLeft `bitsize <= preserved bits` width guard.
- `w10bfa_nestedand_contiguous_lsb_mask_absorbs` /
  `..._noncontiguous_mask_does_not_absorb` / `..._mask_narrower_than_field_does_not_absorb`
  — absorbNestedAnd coveringmask + popcount gate.
- `w10bfa_ext_matching_sign_absorbs` / `..._mismatched_sign_does_not_absorb` —
  absorbExt SPULL/SEXT signedness match.

All 11 pass; they confirm the rules genuinely fire (not stubs) AND that the
guards reject the unsound cases (no over-eager absorption that would corrupt bits).

losses: none.
