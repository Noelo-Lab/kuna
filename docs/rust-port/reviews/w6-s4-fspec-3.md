# w6-s4-fspec-3
verdict: ACCEPT-WITH-LOSSES
verifier: claude-opus-4-8 (independent verifier)
date: 2026-06-13
gate: `cd rust && cargo test -p kuna-decomp` -> PASS (51 `test result: ok` lines, 0 FAILED, incl. the 5 new verify_w6_s4_fspec_3 tests)

## Scope

C++ in scope: `decompiler/cpp/fspec.cc` 4854-5982 (`FuncCallSpecs` + the
`FspecSpace::printRaw`/`encodeAttributes` arms at 2118-2169) and
`fspec.hh:1645-1742`. cpp_blob_sha verified against the checklist
(`fspec.cc` 903b587…, `fspec.hh` 6d1615b…) — match.

Rust diff: the fspec-3 commit (778355b) over the fspec-2 base (1807f47).
`rust/crates/kuna-decomp/src/fspec.rs` (+892), `rust/crates/kuna-base/src/space.rs`
(+126, the FspecSpace arms + the call-spec registry), `fspec/tests.rs` (+546).

## What was ported (correct, re-derived) vs deferred

Ported and faithful: the `FuncCallSpecs` struct (FuncProto by composition),
all simple accessors, `initActiveInput`, `get/setInputBytesConsumed`,
`checkInputJoin`/`doInputJoin`, `lateRestriction` (the unlocked compat gate),
the restart-decision paths of `deindirect`/`forceSet`,
`getSpacebaseRelative`/`resolveSpacebaseRelative`/`abortSpacebaseRelative`,
`transferLockedInput`/`transferLockedInputParam` (register-reuse path), the
FspecSpace `printRaw`/`encodeAttributes` arms + the call-spec side-table
registry, and `ParamActive::getTrialForInputVarnode`. Seamed-to-`Err`
(W4 Funcdata factory) / absent: `createPlaceholder`, `transferLockedOutput`
(non-void), `buildParam`, `transferLockedOutputParam`, `commitNewInputs`,
`commitNewOutputs`, `insertPcode`, `collectOutputTrialVarnodes`,
`finalInputCheck`, `checkInputTrialUse`, `checkOutputTrialUse`,
`buildInputFromTrials`, `buildOutputFromTrials`, and the success-commit/op-rewrite
branches of `deindirect`/`forceSet`. Absent but purely portable at this boundary:
`clone`, `paramshiftModifyStart/Stop`, `hasEffectTranslate`, `findPreexistingWhole`,
`countMatchingCalls`. All of the above are ledgered as **LOSS-093**.

## Mandatory hunt list

- **Signedness.** Clean. `setInputBytesConsumed`'s `inputConsume.size() <= slot`
  and `slot >= size()` are `len() as i32` vs `int4` — matches the C++ size_t/int4
  comparison; the `slot` operands are non-negative call slots. `get_slot()-1`,
  `num_params()` are int4-vs-int4 throughout.
- **Integer widths.** Clean. `effective_extrapop:int4`, `stackoffset:uintb (u64)`,
  `EXTRAPOP_UNKNOWN=0x8000`, `OFFSET_UNKNOWN=0xBADBEEF`, `match_call_count:int4`
  all match C++. `size as i64` in the FspecSpace sized arm is a sign-preserving
  widen of `int4 size` for `writeSignedInteger`.
- **Wrapping.** Clean and pinned. `resolveSpacebaseRelative` `stackoffset -= addr.getOffset()`
  → `self.stackoffset.wsub(addr.get_offset())` + `wrap_offset` reproduces the C++
  `uintb` underflow wrap (adversarial test F-C: refvn 0 − locked param 0x20 wraps
  to 2^64−0x20, no panic/saturate). Address `+`/`-` operators wrap via `wadd`/`wsub`
  + `wrap_offset`, matching C++ `operator+(int4)` sign-extend-then-wrap.
- **Comparator totality.** Clean. The only comparators reached are `Address::cmp`
  (space-rank → index → offset; total, with the null/max sentinel handling) used in
  `transferLockedInputParam`'s `startaddr < trial.addr` / `trialend < lastaddr`.
  `compareByEntryAddress` (also `Address <`) is only used by the unported
  `countMatchingCalls`.
- **Iteration-order provenance.** Clean. Every loop is a `Vec` index walk
  (`activeinput.trial`, `source` params, `inputConsume`) preserving C++
  `vector`/`for(i)` order. The new `FSPEC_REGISTRY` is a `BTreeMap<u64,…>`
  (deterministic) — no HashMap/HashSet in the added code (grep clean).
- **Off-by-one / do-while / reverse iterators.** Clean and pinned.
  `getTrialForInputVarnode` `slot < stackplaceholder ? 1 : 2` boundary verified at
  `slot == stackplaceholder` (subtract 2, NOT 1) — adversarial test F-B; a buggy
  `<=` would OOB. No do-while / reverse-iterator idioms in the ported surface.
- **Erase-while-iterating.** N/A — the methods that erase during traversal
  (`buildInputFromTrials::deleteUnusedTrials`, `buildOutputFromTrials`) are unported
  (LOSS-093). `abortSpacebaseRelative` removes one input then destroys the def op
  using a pre-saved stable `VarnodeId` (not an iterator) — faithful.
- **Exception -> Result partial-state parity.** Clean. `setFuncdata` (multiple-set
  throw), `doInputJoin` (locked throw), `resolveSpacebaseRelative` (the two
  `LowlevelError` throws + the end-of-fn throw) all map to `Err(...)` with the exact
  C++ strings, at the same points, leaving the same partial mutation (e.g. `stackoffset`
  is written before the "Unresolved stack placeholder" error, as in C++). The
  `deindirect`/`forceSet` restart-pending flag is set at exactly the C++ control-flow
  points (early-return on success, set on failure/no_return/inline).

This wave's extra items: `Datatype::compare`/sort-order — N/A (no Datatype compare
in scope). `ParamEntry`/`ProtoModel` sort orders — only `Address`-keyed sorts are
reached here and the `Address` order is total/faithful. `TypeFactory` cache
determinism — N/A.

## Mechanical pass

- grep of the fspec-3 added regions: no `todo!`/`unimplemented!`/`HashMap`/`HashSet`/
  `sort_unstable`/`panic!`. Bare `as` casts: all 9 are width/sign-preserving and
  justified (`size as i64` widen-signed; `len() as i32` for int4 compare; `slot as usize`
  index; `(sz-1) as i64` for the Address offset op). No unjustified `as`.
- `cargo clippy -p kuna-decomp -- -D warnings`: clean. `cargo clippy -p kuna-base
  -- -D warnings`: clean. (`cargo clippy --tests -D warnings` surfaces pre-existing
  lints in OTHER, untouched test files — double.rs, verify_w3/w4/w5_* — and zero in
  the new verify_w6_s4_fspec_3.rs; not in scope.)

## Adversarial tests (added, committed on the branch)

`rust/crates/kuna-decomp/tests/verify_w6_s4_fspec_3.rs` (5 tests, all pass):
- `fspec3_input_bytes_consumed_sentinel_and_monotone` — grow-with-0, smallest-wins,
  equal/larger no-change, the `oldVal==0` sentinel short-circuit, OOB read → 0.
- `fspec3_trial_for_input_varnode_placeholder_equal_boundary` — the `slot ==
  stackplaceholder` index boundary (subtract 2), with `slotbase` starting at 1.
- `fspec3_resolve_spacebase_relative_locked_wrap_underflow` — the locked-stack-param
  `uintb` subtract underflow wraps (not panic), reading the offset off the placeholder
  def first.
- `fspec3_late_restriction_locked_register_reuse` — `transferLockedInput` →
  `transferLockedInputParam` register-reuse (`reuse>0 ⇒ op->getIn(reuse)`), the
  newinput list `[dest, reused]`, proto converted to the restricted (locked) one.
- `fspec3_check_input_join_reaches_funcproto` — `checkInputJoin` positive
  delegation to `FuncProto::checkInputJoin` (complements the porter's negative-gate
  test), deterministic.

## findings

  - F1 (minor): a large set of in-scope `FuncCallSpecs` methods are seamed-to-`Err`
    or entirely absent. The IR-mutating ones are legitimate W4 Funcdata-factory
    seams (matching the fspec-2 convention); however `clone`, `paramshiftModifyStart/Stop`,
    `hasEffectTranslate`, `findPreexistingWhole`, and `countMatchingCalls` are
    PURELY PORTABLE at this boundary (their dependencies — `FuncProto::hasEffect`,
    `wrapOffset`, `loneDescend`/`getOut`, `opRemoveInput`/`removeParam`, the `Address`
    total order — are all already present) yet were omitted with no ledger entry.
    Resolved by appending **LOSS-093** (cited below); not a blocker — no consumer
    exists at this boundary, the omissions produce no wrong output, and the codebase's
    dominant precedent (LOSS-050/078/082/083/084/088/090/092) accepts cross-wave
    seams with a ledgered loss rather than REJECT.
       cpp: decompiler/cpp/fspec.cc:5957 (countMatchingCalls), :5941 (hasEffectTranslate), :5908/:5918 (paramshiftModify*), :5757 (findPreexistingWhole), :4969 (clone)
       rust: rust/crates/kuna-decomp/src/fspec.rs (no counterpart; FuncCallSpecs impl ends at transfer_locked_output)

  - N1 (note, not a finding): the FspecSpace `printRaw` name is resolved at
    *registration* time (`fspec_printed_name` → side-table) rather than at print time
    as in C++ (which casts the offset to `FuncCallSpecs*` and reads live). A name/entry
    change between register and print would print a stale string. This is forced by the
    crate boundary (kuna-base cannot see `Architecture`/`FuncCallSpecs`) and is unobservable
    until `newVarnodeCallSpecs` (W4) wires the registry into real flow. Covered by the
    LOSS-093 cross-wave framing; called out for the de-seam.

## hunt list summary

signedness: clean · widths: clean · wrapping: clean (pinned F-C) ·
comparator totality: clean · iteration order: clean (BTreeMap registry, Vec walks) ·
off-by-one: clean (pinned F-B) · erase-while-iterating: N/A (unported) ·
exception→Result: clean

## losses

LOSS-093 (the seamed IR-mutating half + the purely-portable-but-deferred
`clone`/`paramshiftModify*`/`hasEffectTranslate`/`findPreexistingWhole`/`countMatchingCalls`).
