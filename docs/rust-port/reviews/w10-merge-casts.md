# w10-merge-casts (checklist item: w10-unseam-merge-types; branch rport/w10-merge-casts)
verdict: ACCEPT-WITH-LOSSES
verifier: Claude Opus 4.8 (1M context) — independent verifier
date: 2026-06-15
round: 2
gate: `KUNA_ENGINE=rust run_tests --datatests` (branch worktree /tmp/kuna-merge-casts,
      HEAD 540bf30 + verifier fence commit c89fe68)
      -> 182/668 assertions passed. Measured the branch AND the merge-base (37c2450)
      with the Rust engine and diffed the passing SETS (not just counts):
      `comm` => 182 common / 0 branch-only / 0 mergebase-only. BYTE-IDENTICAL passing
      set; this wave adds ZERO datatest passes and ZERO regressions (parity-neutral
      infrastructure — the same un-seam convention as the prior merge items; the
      "count rises" M3 signal lags until the S6 merge graph fully feeds the cast loop).
      `cargo test -p kuna-decomp` -> 2567 passed / 0 failed (2562 prior + my 5 new
      round-2 dispatch fences); `cargo test --workspace` -> 3402 passed / 0 failed;
      `cargo clippy --lib -p kuna-decomp` -> clean (the round-2 fence module adds no
      clippy warnings; the `--tests -D warnings` failures are PRE-EXISTING `SeqNum`
      unused-imports + doc-list formatting in unrelated verify_w3/w4/w6 files, not in
      this diff).
      C++ oracle (main tree): 207/207 unit + 675/675 datatest, PARITY OK (UNTOUCHED —
      the branch diff is 100% under rust/; no C++/specs/baseline/docs touched).

## round-1 -> round-2: the REJECT basis is FIXED

Round 1 REJECTED on two unfaithful `getInputCast` cast decisions (B1, B2). Commit
540bf30 "r2: fix SLESS/ZEXT getInputCast tuples (B1/B2)" repairs both, and a
line-by-line re-check against C++ `typeop.cc` plus five non-vacuous in-module dispatch
fences confirm the fix is faithful (not a corpus-shortcut):

- **B1 FIXED (INT_SLESS / INT_SLESSEQUAL).** The dispatch (coreaction_casts.rs:311-313)
  now routes `CPUI_INT_SLESS | CPUI_INT_SLESSEQUAL` to `get_input_cast_less(.., true)`,
  which uses `reqtype = input_type_local(slot)` (a SIGNED int4, opcode metain=TYPE_INT)
  + the `check_int_promotion_for_compare` gate + `cast_standard(reqtype, curtype, true,
  care_ptr_uint=true)`. This MATCHES C++ `TypeOpIntSless::getInputCast` (typeop.cc:1025-
  1033) and `TypeOpIntSlessEqual` (:1051-1059) statement-for-statement, including the
  `(true,true)` tail. LESS/LESSEQUAL share the body via `get_input_cast_less(.., false)`
  -> `(true,false)` (typeop.cc:1077-1085, :1101-1108). The ONLY difference between the
  families is the `care_ptr_uint` flag, exactly as the C++ (the bodies are otherwise
  identical). EQUAL/NOTEQUAL keep the distinct max-read-facing + `(false,false)` body
  (typeop.cc:934-944, :998-1008). Verified.
       cpp: decompiler/cpp/typeop.cc:1025-1033, :1051-1059, :1077-1085, :1101-1108,
            :934-944, :998-1008
       rust: rust/crates/kuna-decomp/src/coreaction_casts.rs:302-316 (dispatch split),
             :505-524 (get_input_cast_less w/ care_ptr_uint), :471-496 (equal body)

- **B2 FIXED (INT_ZEXT / INT_SEXT).** `get_input_cast_extension` (coreaction_casts.rs
  :527-549) now, after the `check_int_promotion_for_extension` gate, computes
  `reqtype = input_type_local(slot)`, `curtype = read-facing`, and returns
  `cast_standard(&reqtype, &curtype, true, false)` — MATCHING C++ `TypeOpIntZext`/
  `TypeOpIntSext::getInputCast` (typeop.cc:1133-1141, :1159-1167) `(true,false)` tail.
  The spurious `is_annotation()` early-return (which only exists in the *default*
  TypeOp::getInputCast body, typeop.cc:296-304 — still correctly present in the `_`
  arm at coreaction_casts.rs:346-349) was removed from the extension body. Verified.
       cpp: decompiler/cpp/typeop.cc:1133-1141, :1159-1167 (vs the default :296-304)
       rust: rust/crates/kuna-decomp/src/coreaction_casts.rs:527-549

The fix is surgical (1 file, +36/-22) and adds NO new bare `as` casts, NO name/address
dispatch, NO `todo!`/`HashMap`. The rest of the getInputCast/getOutputToken sweep was
already CLEAN in round 1 and is unchanged.

## scope (what this branch is)

`git diff rust-port...rport/w10-merge-casts` is **Rust-only**:
  - `0d6a4e8` "S6 HighVariable Merge engine": lands `funcdata_facing.rs` — the four
    `Varnode` facing accessors (varnode.cc:645-691) re-homed onto `Funcdata` (ADR-0001)
    + `find_resolve` per-kind dispatch. (The merge *coalescing* the prompt names —
    union-find / COPY-shadow / datatype+adjacent merge, merge.rs + funcdata_merge.rs —
    was landed and verified in PRIOR items: w7-s6-merge ACCEPT-WITH-LOSSES and
    w10-merge-naming-types ACCEPT-WITH-LOSSES; it is NOT in this diff.)
  - `a503494` "ActionSetCasts cast insertion": `coreaction_casts.rs` (the driver + the
    per-op getInputCast/getOutputToken surface + the FuncdataCastContext bridge), wires
    `action_set_casts` into the live `ActionSetCasts::apply` (coreaction_cleanup.rs),
    `pub(crate)` bumps on input/output_type_local.
  - `03c2982` (r1 verifier fences) + `540bf30` (the B1/B2 fix) + `c89fe68` (my r2 fences).

The FAITHFULNESS surface owned by THIS verdict is `funcdata_facing.rs` (facing graph)
and `coreaction_casts.rs` (the cast loop + getInputCast/getOutputToken). The
merge-engine order/tie-breaks/COPY-shadow were verified in prior rounds.

## faithfulness verdict

- **getInputCast / getOutputToken (the named keystone): FAITHFUL.** Full sweep of every
  C++ `getInputCast` `castStandard(...)` tuple (typeop.cc) vs the Rust dispatch: TypeOp
  default ->(false,true), Copy ->(false,true), Load ->(false,true), Store
  ->(false,true), Equal/NotEqual ->max-read-facing+(false,false), **SLESS/SLESSEQUAL
  ->inputTypeLocal+(true,true) (B1 fix)**, Less/LessEqual ->inputTypeLocal+(true,false),
  **Zext/Sext ->inputTypeLocal+(true,false) (B2 fix)**, Right/SRight/Div/SDiv/Rem/SRem
  ->(true,true) w/ signed_ext bit, FloatInt2Float ->(care_uint_int,true). getOutputToken
  arms (arithmetic->arithmeticOutputStandard, shift->in0 read-facing w/ BOOL->INT,
  ptrsub/subpiece/piece/load) all match.
- **funcdata_facing.rs: FAITHFUL** vs varnode.cc:645-691 (def-facing `findResolve(def,-1)`,
  read-facing `findResolve(op, op->getSlot(this))`, both gated on `needsResolution()`)
  and the type.cc findResolve per-kind overrides (Pointer gates on ptrto==UNION; Struct
  cache-else-field[0]; Array cache-else-elem; Union cache-else-self; PartialUnion
  cache-else-stripped; base self). The needsResolution gate keeps boolless/condconst
  byte-identical (fences green).
- **ActionSetCasts driver + helpers: FAITHFUL** statement-for-statement vs
  coreaction.cc:2407-2872 (castInput/castOutput/resolveUnion/testStructOffset0/
  tryResolutionAdjustment/tryResolutionCopy/insertPtrsubZero/isOpIdentical, the
  if/else-if cast chaining, the PTRADD/PTRSUB fixups, insertBefore/After placement).
  `cast_check_pointer_issues` is faithfully warnings-only (the C++ body only ever calls
  `data.warning(...)`; no IR mutation). Two faithful-partial divergences (F1 iteration,
  F2 count) ledgered as losses — see below.

## no-special-casing verdict
CLEAN. Grep of the production src files (coreaction_casts.rs, funcdata_facing.rs,
coreaction_cleanup.rs, coreaction_infertypes.rs) for hardcoded function/type/field/var
names and 0x40xxxx addresses finds ZERO in engine logic. The only `boolless`/`HashMap`
strings are in comments and in TEST anchors (`render_one("boolless", 0)` — a byte-parity
fence). Cast decisions dispatch on opcode/metatype/size/field-index, never on a name/
address/magic constant. No `todo!`/`unimplemented!`/`HashSet`/`sort_unstable`.

## hunt list
- **Signedness**: the B1/B2 fixes ARE the signedness-correctness repair (care_uint_int /
  care_ptr_uint control the signed/unsigned cast gate). Verified faithful. One masked-only
  nit unchanged: `get_input_cast_int2float` `shift = 8*vn_size-1` guards only `>=64`
  (reachable only for impossible vn_size==0; same `8*getSize()-1` UB in C++).
- **Integer widths**: clean. int4/uintb mappings preserved.
- **Wrapping**: clean. `address_to_byte` uses wrapping; the SUBPIECE/PTRSUB offset
  arithmetic is the documented debug-panic-vs-C-wrap class on impossible inputs (F3).
- **Comparator totality**: clean — no new comparators; the apply loop uses index/block
  order, no sorts.
- **Iteration-order provenance**: blocks (`bblocks_get_block(j)` = C++
  `basicblocks.getBlock(j)`, dominance/structuring index order) and ops (`bb_ops(bl)`
  head->tail) match. The snapshot-vs-live model diverges for ONE insert path -> F1/LOSS-179.
- **Off-by-one / do-while / reverse iterators**: clean. The `while let Some(td)=
  get_typedef()` typedef-strip loops match the C++ `while(getTypedef()!=0)`.
- **Erase-while-iterating**: n/a — the cast loop inserts (never erases).
- **Exception -> Result partial-state parity**: clean. The C++ helpers don't throw; the
  Rust `?`/`unwrap_or` early-outs leave the same partial IR.

## findings (all secondary — now accepted as losses; the round-1 REJECT basis B1/B2 is fixed)
- F1 (minor / LOSS-179): the apply loop SNAPSHOTS the block op list (`bb_ops`,
  coreaction_casts.rs:1051) whereas C++ uses a LIVE `bb->beginOp()..endOp()` iterator.
  Equivalent for before-inserts and CAST-after-inserts; diverges only on castOutput's
  offset-0 PTRSUB-after (C++ `++iter` revisits it; the snapshot does not). Corpus-
  invisible (passing set byte-identical), deterministic, rare shape. Faithful-partial.
       cpp: decompiler/cpp/coreaction.cc:2812-2872, :2680
       rust: rust/crates/kuna-decomp/src/coreaction_casts.rs:1044-1102
- F2 (minor / LOSS-180): `ActionSetCasts::apply` discards the cast count
  (`let _count = data.action_set_casts(); 0`) without `base_mut().count += _count`,
  diverging from C++ `count += ...` and the in-tree convention. Inert on output
  (non-repeating final pass, warnings seamed). Faithful insertion, bookkeeping gap.
       cpp: decompiler/cpp/coreaction.cc (ActionSetCasts::apply `count +=`), action.hh:81
       rust: rust/crates/kuna-decomp/src/coreaction_cleanup.rs (`let _count = ...; 0`)
- F3 (info, not ledgered): a few bare `as` casts in coreaction_casts.rs carry no
  `// cast:` token (`vn_size as usize`, `in1off as i64`, `idx as i32`, `sz as int4`).
  All benign widenings/index conversions; the SUBPIECE/PTRSUB offset casts skip the C++
  `(int4)getOffset()` truncation (unreachable for small real field offsets). The B1/B2
  fix added NO new `as`. Code-convention nit, not a behavior divergence.
- F4 (info, not a divergence): `find_resolve_facing` field-less-struct arm returns the
  type itself rather than panicking; defensive, equivalent on valid input.

## mechanical pass
- `cargo run -p port-audit -- --item w10-unseam-merge-types`: N/A — the checklist item
  has empty modules/cpp_blob_sha (un-seam item), nothing to drift-check.
- grep: no `todo!`/`unimplemented!`/`HashMap`/`HashSet`/`sort_unstable` in the production
  src files. Bare `as` without `// cast:` — F3 (benign, pre-existing, none added by r2).
- `cargo clippy --lib -p kuna-decomp`: clean (incl. the round-2 fence module).

## adversarial tests
Round 1 (commit 03c2982, verify_w10_merge_casts.rs) — three `cast_standard`-proxy fences
proving the tuple divergence (B1/B2). Still green.

Round 2 (commit c89fe68, IN-MODULE `coreaction_casts.rs::verify_w10_merge_casts_r2`) —
FIVE fences that drive the ACTUAL fixed `get_input_cast` dispatch over a real Funcdata
op (the r1 fences could not — `get_input_cast` is `pub(crate)`, invisible to `tests/`).
Each is self-calibrating (expected = the faithful tuple on the same operands) and built
with 4-byte operands so `checkIntPromotion*` returns false and the corrupted castStandard
tail is actually reached:
  - `at1_sless_dispatch_uses_signed_local_and_true_true` — INT_SLESS over a uint4 operand
    routes to inputTypeLocal+(true,true) (cast required), not the EQUAL body. (B1)
  - `at2_sless_vs_less_reqtype_signedness_differs` — SLESS reqtype is SIGNED int4, LESS
    reqtype is UNSIGNED uint4 (the signedness the EQUAL misroute erases).
  - `at3_zext_dispatch_uses_true_false_not_inverted` — INT_ZEXT uses (true,false), not the
    inverted (false,true). (B2)
  - `at4_sext_dispatch_uses_signed_local_true_false` — INT_SEXT shares the extension body
    (signed local + (true,false)).
  - `at5_equal_still_uses_maxreadfacing_false_false` — INT_EQUAL unperturbed by the fix.
PROVEN NON-VACUOUS: re-injecting B1 fails at1; re-injecting B2 fails at3+at4; reverting
the fix returns all 5 to green. Pure test-only addition (313 lines, #[cfg(test)], no
production logic touched).

Regression fences re-run green this round: byte-identity (boolless/condconst via the
facing graph and the cast stage — w10_byte_identical_boolless_unperturbed_by_casts,
w10_facing_graph_threaded_boolless/condconst, w10_dc_*, w10_eob_*), the funcdata-union
cache (verify_w10_funcdata_union_cache 5/5), typed-access (verify_w10_typed_access 8/8).

## losses
- LOSS-179 (F1): snapshot-vs-live cast-loop iteration (castOutput PTRSUB-after not revisited).
- LOSS-180 (F2): ActionSetCasts::apply discards the cast count (no base.count accumulation).
Both are corpus-invisible, deterministic, faithful-partial; the passing SET is byte-
identical with zero regression. The merge facing graph + the full cast loop land; the
B1/B2 cast-decision bugs that drove the round-1 REJECT are fixed faithfully.
