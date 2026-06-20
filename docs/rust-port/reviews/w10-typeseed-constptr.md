# w10-typeseed-constptr
verdict: ACCEPT-WITH-LOSSES
verifier: opus-4.8 (independent verifier subagent)
date: 2026-06-16 (round 1)
gate: cd /tmp/kuna-typeseed/rust && cargo test --workspace -> ok (0 failed); cargo test -p kuna-decomp --lib -> 1744 passed (== base); clippy -p kuna-decomp --lib -> clean

## One-line

A faithful, non-special-cased port of the const->pointer TYPE-SEED
(`ActionConstantPtr::apply`/`selectInferSpace`/`isPointer`, `Funcdata::spacebaseConstant`,
`Architecture::cacheAddrSpaceProperties`/`inferPtrSpaces`) that lands **+2 real parity**
(rust-engine datatests 331 -> 333) with **ZERO regression**; the full render payoff is
gated behind reserved cast/declaration-render files. ACCEPT_WITH_LOSSES (LOSS-211).

## Branch / diff scope (reserved-file check: CLEAN)

- branch 308433b, base (merge-base with rust-port) 47059f6; ONE commit.
- `git diff 47059f6..308433b --name-only` = 6 files, ALL under `rust/`:
  `architecture.rs`, `coreaction_render.rs`, `funcdata.rs`, `funcdata_varnode.rs`,
  `seams.rs`, `tests/verify_w10_typeseed_constptr.rs`.
- NO reserved file touched: none of funcdata_merge.rs / merge.rs / variable.rs /
  dynamic.rs / database.rs / varmap.rs (the concurrent mergepiece-dynsym wave) or
  coreaction_cleanup.rs (the concurrent base_explicit wave) appears in the diff.
- No non-rust file changed (C++ source / specs / baseline byte-untouched).
- `funcdata_varnode.rs` and `seams.rs` are NOT reserved; their edits are required for the
  feature (global-scope name binding for the ram-spacebase PTRSUB; the `glb` config/query
  routing) and are faithful (see below).

## (1) FAITHFULNESS — line-by-line vs the C++ oracle: PASS

- `ActionConstantPtr::apply` (coreaction.cc:1183): `hasTypeRecoveryStarted` gate,
  `localcount>=4` cap + increment, the constant-space loc-set loop with the
  `!isConstant -> break` (enditer) guard, the offset==0 / ptrcheck / hasNoDescend /
  isSpacebase continues, `loneDescend`, `selectInferSpace`, `getSlot`, the INT_ADD
  other-side-spacebase continue, the PTRSUB/PTRADD continue, `isPointer`,
  `setPtrCheck` AFTER searching, `spacebaseConstant`, the INT_ADD slot==1 `opSwapInput`,
  `count+=1` — all transcribed in order. The Vec-snapshot of the constant subset is the
  faithful membership set (inserts go past enditer; never revisited).
- `selectInferSpace` (coreaction.cc:1020): the TYPE_PTR-with-space fast path
  (size match), the `inferPtrSpaces` scan with `minSize==0 ? size==addrSize : size>=minSize`
  filter, the second-match `searchForSpaceAttribute` tiebreak + break, returns first match.
- `isPointer` (coreaction.cc:1085): the read-facing-PTR fast path (`resolveConstant`,
  needexacthit=false); the typelock NOT-a-pointer reject; the per-opcode switch
  (CALL/CALLIND slot-0 reject + input-lock/infer gate, COPY `checkCopy`, PIECE/INT_EQUAL/
  NOTEQUAL/LESS/LESSEQUAL infer gate, INT_ADD def-facing-PTR + other-side-PTR reject +
  needexacthit=false else infer gate, STORE slot!=2 reject, default reject); the
  pointer-lower/upper-bound range check; the `bit_transitions<3` mask filter WITH the kuna
  GH-6930 `!kunaIsFunctionEntry` escape (coreaction.cc:1158-1159) faithfully transcribed;
  `resolveConstant`; the invalid-rampoint reject; `queryContainer`; the TYPE_ARRAY
  char-print needexacthit relaxation; the `needexacthit && entry->getAddr()!=rampoint`
  reject. Matched arm-for-arm.
- `spacebaseConstant` (funcdata.cc:358-460): `getTypeSpacebase`->`getTypePointer` sb_type,
  the LOAD-BEARING `getTypePointerStripArray(sz, sym->getType(), wordsize)` ptrentrytype
  (funcdata.cc:413) forced onto the PTRSUB output via `updateType(ptrentrytype, typelock,
  false)` (funcdata.cc:417) — verified present and in order; the `extra` byte->address
  conversion; the COPY-replacement isCopy/addOp/extraOp/zextOp/subOp bookkeeping; the
  `spacebase_vn updateType(sb_type,true,true)+setFlags(spacebase)`; the newconstoff =
  origval - extra; the truncated->setPtrFlow; the typelock + TYPE_UNKNOWN unlock; the
  extra!=0 INT_ADD insert; the sz<origsize ZEXT / origsize<sz SUBPIECE adjusters; the
  `!isCopy -> opSetInput`. Line-faithful; wrapping_sub used for both uintb subtractions.
- `cacheAddrSpaceProperties` (architecture.cc:671-707): copyList = inferPtrSpaces + default
  code + default data; sort by compareByIndex; dedup; filter (delay==0 register / SPACEBASE
  / OTHER / overlay); the defPos search promoting the default DATA space to index 0. Faithful
  (segment-op markNearPointers loop is a declared LOSS — see below).
- `addToGlobalScope` `inferPtrSpaces.push_back(spc)` (architecture.cc:836): restored in
  `decode_global`'s `to_add` loop (the LOSS-208 deferral this wave closes).
- `postSpecFile()` -> `cacheAddrSpaceProperties()`: wired at the END of `restore_from_spec`,
  after `decode_global` pushed the cspec spaces and all analysis spaces exist (the C++
  ordering invariant — postSpecFile runs after restoreFromSpec/adjustCaches/buildSymbols;
  the collapse into restore_from_spec's tail preserves "all spaces present, globals pushed").
- kuna divergence: `kuna_const_is_function_entry` wraps the pre-existing
  `kuna_inferfuncentry::kuna_is_function_entry`; supplies `function_entry` via
  `query_container_global` gated on `TYPE_CODE` metatype + exact entry — the seam analogue
  of C++ `queryFunction(rampoint)->getAddress()==rampoint`. Faithful; only RELAXES the mask
  filter for exact function entries, so switch tests stay byte-identical (T2/T3 confirm).
- `linkSpacebaseSymbol` global-scope fallback (funcdata_varnode.rs): C++
  `linkSymbolReference` uses `sb->getMap()` = the GLOBAL scope for a ram spacebase
  (funcdata_varnode.cc:1224); the Rust adds a global-scope fallback to the local-first
  lookup so the ram-spacebase PTRSUB binds its symbol name. Faithful and required.

## (2) NO SPECIAL-CASING: PASS

Grep of the production diff for address / symbol-name / type-name / literal-space
hardcoding is CLEAN. The data-type is read GENERICALLY from the global symbol-table
snapshot (`GlobalContainer.symbol_type` <- `query_container_global` <- `GlobalQuery`); the
inferable spaces come from the manager + cspec `<global>` push. The only literal token in
production code is the `TYPE_CODE` metatype comparison in `kuna_const_is_function_entry`
(generic). `myarray`/`globindex`/`0x601060`/`0x101000` appear ONLY in the verifier test
assertions (correct — tests assert on expected output).

## (3) NEW PASSES ARE REAL: PASS (+2, byte-verified)

Re-ran both harnesses over all 83 datatests:
- base (47059f6): `Total passing tests = 331`.
- branch (308433b): `Total passing tests = 333`.
- FULL FAIL-set diff: NEWLY PASSING = {`Two dimension #3`, `Three dimension #3`};
  REGRESSED = EMPTY.
- `Two dimension #3` / `Three dimension #3` assert `min=0 max=0` for the regex ` \* ` — i.e.
  NO bare ` * ` pointer-deref. Before the type-seed the body read `* 4 + 0x601060` (FAIL);
  after, the array indexes by symbol (no ` * `) -> PASS. Independently confirmed by dumping
  the twodim render: `((int4 *[5])myarray)[globindex][valout] = ...` (no ` * `).
- The exact `myarray[globindex][valin]` (#1/#2) does NOT pass — blocked by the spurious
  `(int4 *[5])` cast (declared LOSS-211, reserved-file-gated). This is the honest partial.

## (4) NO REGRESSION: PASS

- Whole-suite passing 333 >= 331; FULL passing-SET diff: regressed EMPTY.
- THE 6 SWITCH DATATESTS (switchind/switchmulti/switchhide/ifswitch/switchloop/switchreturn):
  render BYTE-IDENTICAL base<->branch (verified by dumping `print C` for each; all six match
  to the byte). The const->ptr typing does not perturb jumptable index recovery — the
  `bit_transitions<3` mask filter holds. T2/T3 confirm no spurious `PTRSUB(0,` / `&dat_`
  leaks into any switch render.
- boolless / condconst: BYTE-IDENTICAL to the C++ oracle (T4 asserts this in-tree; also
  re-verified directly).
- Whole-corpus render diff base<->branch: only 5 stems changed — `twodim`/`threedim`
  (intended) and `copytrim`/`injectoverride`/`longdouble`. The latter three are
  ALREADY-FAILING tests whose per-assertion FAIL set is byte-identical base<->branch (render
  text changed, pass/fail status did not). Their changed renders show the same
  cast/declaration-render gap (LOSS-211); no passing assertion regressed.
- C++ oracle 675/675 PARITY OK, byte-untouched (rust-only diff).
- B0: `cargo test -p kuna-decomp --lib` = 1744 passed on BOTH base and branch (no lib test
  removed; the +5 added tests are the verifier's integration tests).
- cargo test --workspace green (0 failed); clippy -p kuna-decomp --lib clean.

## Hunt list (verification.md §"mandatory hunt list")

- Signedness: clean. `1 - slot` is int4 (slot 0/1); `spc.get_addr_size() as int4 == vn_size`
  matches C++ `getAddrSize()==getSize()`; no signed-index-vs-size compare.
- Integer widths: clean. `8*sz` in `bit_transitions` (small sz); space-id `as int4` in
  `space_from_const` truncates a small valid index (matches C++ getSpaceFromConst).
- Wrapping: clean. `rampoint - entry.addr` and `origval - extra` use `wrapping_sub`
  (funcdata.cc:367/403, uintb wraparound); no debug-panic-prone arithmetic.
- Comparator totality: clean. `cache_addr_space_properties` sorts by `compare_by_index`
  (`a.index < b.index`, total on unique indices) mapped to Ordering both-ways; dedup by
  `Rc::ptr_eq`.
- Iteration-order provenance: clean. The apply loop snapshots `iter_loc()` (BTree loc-set
  order == C++ VarnodeLocSet `beginLoc(cspc)..endLoc(cspc)`) into a Vec, re-reads per id; no
  HashMap/HashSet in ported logic. `inferPtrSpaces` iterated in built (sorted) order.
- Off-by-one / do-while / reverse iter: clean. No do-while; the loc-set break-on-non-constant
  matches the C++ enditer semantics.
- Erase-while-iterating: clean. `spacebaseConstant` only INSERTS new constants (past
  enditer, never revisited); the snapshot Vec is not mutated during traversal.
- Exception->Result partial-state: clean. is_pointer/select_infer_space use `?` to map
  None->skip (== C++ continue/return-null); `spacebase_constant`'s fallible signature is
  exercised only on can't-happen factory/entry-type paths, with a defensive `continue`
  matching "C++ never errors". Stale-handle `.expect`s panic where C++ derefs invalid
  pointers (UB) — partial-state parity preserved.

## Mechanical pass

- `git diff --name-only` -> 6 rust files, no reserved file (above).
- grep diff src for `todo!`/`unimplemented!`/`HashMap`/`HashSet`/`sort_unstable`/bare
  `.unwrap()` -> NONE. Bare `as` casts: `i as i32`/`def_pos as usize` (small loop index),
  `addr_size as int4` (small), `offset as int4` (valid space index) — all benign; the
  space-id cast lacks an explicit `// cast:` comment (minor, F1).
- cargo clippy -p kuna-decomp --lib -> clean.
- Both harnesses rebuilt fresh; datatest sweeps run end-to-end; per-stem byte diffs taken.

## Findings

- F1 (cosmetic): the `data.vbank().get(vn)...get_offset() as int4` in `space_from_const`
  (coreaction_render.rs) is a bare `as` cast without the verification.md-recommended
  `// cast:` justification comment (it truncates a u64 space-id offset to a valid small
  i32 index, matching C++ `getSpaceFromConst`). Non-blocking; note for cleanup.

No correctness finding. No special-casing. No reserved-file edit. No regression.

## Adversarial tests (committed on the branch, all PASS against the built harness)

- `t1_twodim_global_array_indexes_by_symbol_not_bare_constant` — type-seed payoff (myarray
  binds, [globindex] subscript, no `+ 0x601060`).
- `t2_switch_cluster_recovers_and_gains_no_spurious_pointer` — all 6 switch renders intact,
  no `PTRSUB(0,` leak.
- `t3_single_bit_const_not_turned_into_pointer` — no `&dat_`/`PTRSUB(0,` on switch index/mask
  constants (bit_transitions<3 guard).
- `t4_protected_parity_files_byte_identical` — boolless/condconst == C++ oracle.
- `t5_threedim_global_array_indexes_three_deep` — 3D index tree off the typed PTRSUB.

Verified RAN (not skipped) with `KUNA_DECOMP_TEST_RUST`/`KUNA_DECOMP_TEST` set: 5 passed.

## losses

LOSS-211 (appended): (1) the exact `myarray[globindex][valin]` / single-declaration render
is gated behind a spurious `(int4 *[5])` cast + per-reference symbol re-declaration that
live in the RESERVED cast/declaration-render plane (`coreaction_casts.rs`/`printc.rs`);
(2) the `cacheAddrSpaceProperties` segment-op near-pointer promotion (architecture.cc:696-700)
is a deferral (no segmented-space datatest). Both zero-regression; the type-seed substrate is
faithful and delivers +2 real parity now.
