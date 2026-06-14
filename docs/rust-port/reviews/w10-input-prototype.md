# w10-input-prototype
verdict: ACCEPT-WITH-LOSSES
verifier: independent-verifier (Claude Opus 4.8 1M)
date: 2026-06-14
round: 1
branch: rport/w10-input-prototype (f121138; verifier tests committed at 1d5d7df)

gate (re-run fresh by the verifier):
  - `cd rust && cargo test --workspace` -> ALL GREEN (143 "test result: ok",
    0 FAILED). With the verifier's 8 added tests: kuna-decomp lib 1640 pass.
  - `cargo clippy --workspace` -> clean (no warnings/errors).
  - C++ oracle parity (`python -m kuna.run_tests --all --baseline docs/baseline.json`,
    C++ engine, main tree) -> unit 207/207, datatests 675/675, **PARITY OK**.
    The branch touches ONLY `rust/` (`git diff ... -- ':!rust/'` is empty), so the
    C++ oracle is byte-identical — confirmed.
  - Rust-engine datatests (the real gate context), each tree's OWN freshly built
    `decomp_test_dbg` against its OWN corpus:
      baseline (rust-port) 25/456 ; branch (f121138) 26/456 ; **net +1**.
      Execution-failure (ERROR) count: 28 -> 28 (**no regression**).

## Verdict rationale

ACCEPT-WITH-LOSSES. `ActionInputPrototype` + the `ActionPrototypeTypes`
locked-input/output forcing + the `emitPrototypeInputs`/declarator printing are
a **faithful** transcription of the C++ (verified line-by-line against
coreaction.cc:4824-4997 + fspec.cc:4057-4133 + printc.cc:144-347/2298-2331).
The recovered, typed parameters are **REAL** — flowed through the genuine
`ProtoModel::assignParameterStorage` machinery, matching the C++ oracle's
signature byte-for-byte (proven below). **No special-casing** (no
function-name / address / type-name / param-count hardcoding — grep clean).
Two narrow LOSSes (one latent printer bug, one downstream-cleanup deferral that
costs exactly one datatest assertion) keep this from a clean ACCEPT, but neither
is fabrication, unfaithfulness, or a weakening shortcut.

## The typing is REAL (proven against the oracle, not trusted)

`namespace.xml` `a::b::assign_vals` is the clean demonstration:
  - baseline (pre-item) Rust:  `uint8 a::b::assign_vals(void)`  (unrecovered)
  - branch Rust:               `int4 a::b::assign_vals(int4 spam)`
  - C++ oracle:                `int4 a::b::assign_vals(int4 spam)`  (byte-match)
The `int4 spam` parameter is produced by `update_all_types` ->
`model.assign_parameter_storage(proto, ...)` (the real proto-model storage
assignment, fspec.rs:5325), then the unlocked recovery's trial machinery
(`possible_input_param` / `resolve_model` / `derive_input_map` /
`update_input_types` pulling `vn->getHigh()->getType()`). It is NOT a hardcoded
string. The verifier's integration test
(`namespace_recovers_named_typed_param_matching_oracle`) pins the byte-equal
signature line against the oracle, and asserts the pre-item `(void)` form is
gone.

`condconst2.xml` `char zeroprop(int4 *ptrint,int4 val)` likewise matches the
oracle signature exactly — the locked-prototype seed (`parse line extern` ->
`apply_locked_prototype` -> `seed_locked_from_pieces`) is faithful.

## Faithfulness — hunt list (all checked)

- **Signedness / int widths.** The 3 added `as int4` / `as usize` casts
  (`vdata.size as int4`, `triallist.len() as int4`, `(slot-1) as usize`) are
  faithful to the C++ (`vdata.size`, `triallist.size()`, `triallist[getSlot()-1]`).
  `slot` is 1-based and the index is guarded by `is_used()` exactly as C++ —
  no underflow the C++ doesn't also have.
- **Iteration order.** The input-Varnode walk uses `iter_def_flag(input)` =
  `beginDef(Varnode::input)..endDef`, the established sorted `VarnodeDefSet`
  order (DefKey BTree). Faithful provenance; no HashMap/HashSet introduced
  (grep clean).
- **Off-by-one / do-while / erase-while-iterating.** None. The two trial loops
  are straight `for i in 0..numtrials` index walks matching C++; the mark/clear
  bookkeeping (`set_mark`/`clear_mark` over `triallist`) mirrors fspec.cc:4068-4090.
- **Exception->Result partial-state.** `apply_locked_prototype` resets
  `funcp = FuncProto::new()` on a `seed_locked_from_pieces` error (un-ported W4
  storage-assignment seam) and returns Ok — graceful degradation to the
  unrecovered form. This does NOT fabricate; it falls back to "no proto," which
  is faithful-or-nothing (verified the success path goes through the real
  `assign_parameter_storage`).
- **`is_input_locked` no-store guard** (fspec.rs:4902) and `clear_unlocked_input`
  no-store guard: faithful — a store-less proto has 0 params hence is unlocked
  (matches C++ `numParams()==0 -> false`), same convention as the existing
  `is_output_locked`. When a store IS attached (the locked/recovery path) the
  original logic is unchanged.
- **`ActionPrototypeTypes` locked-output/locked-input forcing** (diff 60-204):
  verbatim vs coreaction.cc:4843-4937 — the RETURN halt/dead skip, the
  `newVarnode`/`opInsertInput`/`updateType(...,true,true)` order, `setInputVarnode`
  + `setLockedInput` + `extendInput`. The truncated-stack INT_ZEXT and the
  `ptr_size` pointer-trim are correctly noted as not-firing on a non-truncated
  default code space (SEAM W4), matching the C++ guard.

## No special-casing (grep-verified)

`grep '^+' diff` for function-name/address strings, `== <number>`, and
type-name literals: the only literals are `"void"` (the faithful C++
`KEYWORD_VOID` for empty/void params) and `"undefined{N}"`/`"void"` (the
faithful `genericTypeName` for anonymous types). The doc-comment examples
(`int8`/`twostruct`/`char`) are comments, not code. No
zeroprop/condmove/assign_vals/spam/main constants anywhere.

## LOSSES (the two reasons this is not a clean ACCEPT)

### F1 (LOSS-154): `declarator_parts` pointer/array parenthesisation is INVERTED.
The C-declarator builder's ptr-inside-array vs array-of-ptr parenthesisation is
swapped vs C++ `pushTypeStart` RPN (`ptr_expr` vs `array_expr` precedence):
  - pointer-to-array `int4 (*)[1]`  -> Rust `int4 *a[1]`  (WRONG; that is array-of-ptr)
  - array-of-pointer `int4 *[1]`    -> Rust `int4 (*a)[1]` (WRONG; that is ptr-to-array)
The porter's OWN doc-comment on `declarator_parts` states the CORRECT C++
output (`int4 (*)[1] -> ("int4 (*", ")[1]")`), which the code does not produce —
implementation contradicts its own spec. The `pending_ptr` walk only wraps when
an ARRAY modifier sees a *preceding* pointer (the array-of-ptr ordering), so the
two nestings come out swapped. **LATENT, not blocking:** `ptrtoarray.xml`
declares such params (`int4 (*a)[1]`) but only *calls* those functions —
none is decompiled+`print C`'d as a function HEADER, so no passing assertion
depends on the broken path today; the common cases (`int8`, `T *`, `T[N]`,
`T **`, `undefined4 *`, `void`) are all correct (5 verifier unit tests pass).
Pinned by `pointer_to_array_paren_inverted_divergence` (asserts the actual buggy
output with a documenting comment).

### F2 (LOSS-154 cont.): condconst2 loses `SUB`-absent assertion #8 (the −1).
Newly applying the locked prototype forces a typed output Varnode whose SUBPIECE
truncation `SUB(r0,0)` is left in the body — the un-ported downstream type/cast
cleanup actions (a later W8 surface) do not remove it. The C++ oracle emits NO
`SUB` (clean `v1 = '\0'; ... return v1;`) and passes the datatest's
`Immediate Conditional #8` (`min=0 max=0 SUB`); the Rust branch fails it. This
is the single lost assertion. It is a real divergence from the oracle (not a
shared limitation), but a strictly-better state than the pre-item unrecovered
`void zeroprop(void)` (which had no signature at all). Pinned by
`condconst2_typed_sig_but_leaks_sub_unlike_oracle` (asserts the typed signature
IS produced AND the SUB leak IS present, AND the oracle has neither extra).

Net datatest effect: **+2 real gains** (`Conditional Constant #11` = `v1 = 0x14;`;
`Namespace #1` = the recovered `a::b::assign_vals(int4 spam)` signature) **−1
real loss** (`Immediate Conditional #8`) = **net +1** (25->26), with the
execution-failure count flat at 28 (no regression). Both gains are non-vacuous
`min=1` content matches; the Namespace gain is the typed-signature win this item
exists to deliver.

## Adversarial tests (8, committed at 1d5d7df, ALL GREEN)

In `rust/crates/kuna-decomp/src/printc/tests.rs`
(`mod w10_input_prototype_declarator`):
  1. `base_type_no_modifier` — `int8 -> ("int8","")`.
  2. `pointer_front_star` — `twostruct * -> ("twostruct *","")`.
  3. `array_tail_no_paren` — `int4[4] -> ("int4","[4]")`.
  4. `pointer_to_array_paren_inverted_divergence` — pins the F1 inverted-paren BUG.
  5. `anonymous_base_generic_name` — `undefined4 *` / `void` genericTypeName.
  6. `pointer_to_pointer` — `char ** -> ("char **","")`.
In `rust/crates/kuna-decomp/tests/verify_w10_input_prototype_adversarial.rs`:
  7. `namespace_recovers_named_typed_param_matching_oracle` — the param is REAL
     and the signature byte-matches the C++ oracle.
  8. `condconst2_typed_sig_but_leaks_sub_unlike_oracle` — the F2 divergence trace
     (typed sig applied; SUB leaked; oracle has neither).

## Why not REJECT

The REJECT triggers are all ABSENT: the typing is not fabricated (proven against
the oracle, driven by the real proto model), there is no special-casing, and the
recovery is not a shortcut to type a specific function (the same machinery
produces the namespace param, the condconst2 params, and would produce any
register/stack param a model assigns). The new positive passes are REAL parity
(byte-content matches). The one lost assertion and the latent declarator bug are
recorded LOSSes with restoration criteria, not silent corruption.
