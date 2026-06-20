# w10-param-narrow
verdict: ACCEPT-WITH-LOSSES
verifier: Claude Opus 4.8 (1M context)
date: 2026-06-15
gate: cargo test --workspace (debug) -> 3421 passed / 0 failed; cargo clippy -p kuna-decomp --lib -> clean; C++ oracle `python -m kuna.run_tests --all --baseline docs/baseline.json` -> 207/207 unit + 675/675 datatests, PARITY OK

## Scope of the diff

The entire branch delta versus `rust-port` is **one commit adding a 17-line
`///` doc comment** on `update_input_types` (coreaction_protos.rs:1289-1304).
Mechanically verified: every added line is a `///` comment; **zero code lines
changed**, **zero C++/spec files touched**, only `coreaction_protos.rs` touched
(no reserved file — fleet discipline intact: printc.rs / dtype.rs untouched).

The comment *diagnoses* the param-narrowing gap behind `ModuloAlt`
(modulo2.xml) and `Return Value Input Register` (retspecial.xml) as a
**cross-wave seam**, not a defect of `updateInputTypes`. This review's job is to
confirm that diagnosis is faithful and honest (a real cross-wave dependency, not
a punt) and that nothing regresses.

## Faithfulness of `update_input_types` (the annotated function)

`FuncProto::updateInputTypes` (fspec.cc:4057) vs `update_input_types`
(coreaction_protos.rs:1305): faithful. `isInputLocked` early-return,
`clearAllInputs`, `count`, the `for i in 0..numtrials` trial loop, the
`isMark()` skip, `pieces.type = vn->getHigh()->getType()`,
`store.setInput(count,"",pieces)`, `count+=1`, `setMark`, the trailing
`clearMark` loop, and `updateThisPointer` all map 1:1. The `isPersist`/
`findDisjointCover` global-input branch is collapsed (function-input registers/
stack are never persistent) — this is **pre-existing rust-port code**, not
introduced by this wave, and is annotated as a narrow SEAM(W4) that does not
fire for these inputs.

## The diagnosis, verified against the C++ oracle (KUNA_DUMP=1)

- modulo2: **C++ oracle** renders `int4 mod2(int4 a0)` (register input narrowed
  to int4). **Rust branch** renders `uint8 mod2(xunknown8)` with
  `v1 = ZEXT(SUB(a0,0) % 2)` — the 8-byte `RCX` reaches `updateInputTypes` with
  the `SUB`/`ZEXT` wrappers UN-collapsed, so the function faithfully records the
  8-byte type of the 8-byte Varnode. Confirmed: the gap is the missing
  `RCX`->`ECX` narrowing (`SubvariableFlow::do_replacement`, subflow.rs, gated
  SEAM(W3-funcdata)/SEAM(W6)), **not** a typing bug in `updateInputTypes`.
  Pinning `int4` here would mismatch the 8-byte Varnode and hardcode a name/size
  — correctly REJECTED.
- retspecial: the callee `returnbig(mystruct *rethidden,int4 num)` already shows
  both params in BOTH engines, so the gap is NOT in `updateInputTypes`. The real
  divergence is the caller `read_returnbig`: C++ recovers
  `int8 read_returnbig(int4 a0)` + `returnbig(&v1,a0)`; Rust still renders
  `read_returnbig(void)` + a single-arg call. Confirmed callarg-values cross-wave
  gap.
- `SubvariableFlow::do_replacement` exists in subflow.rs and is documented as
  gated behind SEAM(W3-funcdata)/SEAM(W6) — the comment's cross-reference is
  accurate and consistent with the pre-existing seam docs.

## No special-casing

No function-name / address / value / type-name / field-name hardcoding in code.
The diagnostic names in the comment (`mod2`, `RCX`, `ECX`, `xunknown8`) are
documentation, not logic. Code body grep for `"mod[0-9]"|"ECX"|"RCX"|0x18000|
"int4"|"xunknown` -> clean. Proven algorithmic, not per-function, by the
uniform 8-byte shape across all four modN (adversarial T2).

## Mandatory hunt list (applies to the annotated `update_input_types`)

- Signedness: clean. `(slot - 1) as usize` indexes `triallist` from a 1-based
  `getSlot()` (pre-existing); no int4-vs-uint comparison introduced.
- Integer widths: clean. `count: i32` matches C++ `int4`; no promotion path.
- Wrapping: clean. No arithmetic on uintb-lineage values in this function.
- Comparator totality: n/a (no comparator).
- Iteration-order provenance: clean. `ParamActive.trial` is a `Vec<ParamTrial>`
  (C++ `vector<ParamTrial>`); `0..numtrials` is vector-index order, matching
  C++. The trailing `for &vn in triallist` clear-mark matches C++ index loop. No
  HashMap/HashSet.
- Off-by-one / do-while / reverse iterators: clean. Plain forward index loops;
  no do-while, no rbegin/upper_bound.
- Erase-while-iterating: n/a (no erase during traversal).
- Exception -> Result partial-state: n/a (no `?`/throw in this body).

## Mechanical pass

- `cargo build --workspace` (debug): green.
- `cargo test --workspace` (debug): 3421 passed / 0 failed (3418 base + 3 new
  adversarial).
- `cargo clippy -p kuna-decomp --lib`: clean (0 warnings/errors).
- grep `todo!`/`unimplemented!`/`HashMap`/`HashSet`/`sort_unstable` in the
  function: none. One `as usize` (array index, pre-existing).
- C++ oracle untouched: 207/207 unit + 675/675 datatests, PARITY OK.
- No-regression: the diff has zero code lines, so the passing SET, byte-parity,
  B5 parity fence (`print_b5_boolless`, `printc_parity`), and oracle are
  byte-identical to `rust-port`.

## findings

- F1 (loss, deferred-by-design): the register-input narrowing for
  modulo2/retspecial is NOT delivered; it is a real cross-wave dependency
  (SubvariableFlow / callarg-values), correctly deferred rather than faked. See
  LOSS-181.
       cpp: decompiler/cpp/fspec.cc:4079-4082 (and subflow.cc createReplacement)
       rust: rust/crates/kuna-decomp/src/coreaction_protos.rs:1289-1304 (SEAM)

## adversarial tests

Added `rust/crates/kuna-decomp/tests/verify_w10_param_narrow.rs` (committed to
the porter branch, 56b0c86), 3 tests, all green:
- `w10_param_narrow_mod2_oracle_int4_rust_unnarrowed_8byte` — pins oracle int4 vs
  Rust 8-byte `xunknown8` + `SUB`/`ZEXT`; asserts divergence so the future
  SubvariableFlow port must flip it deliberately.
- `w10_param_narrow_all_mod_n_uniformly_unnarrowed_not_special_cased` — all four
  modN render the same un-narrowed shape (no per-function hack; never a
  narrowed subset).
- `w10_param_narrow_retspecial_caller_arg_is_crosswave_gap` — pins the caller
  `read_returnbig` missing-arg divergence (the callee already has both params).

## losses

LOSS-181 (w10-param-narrow): register-input narrowing for modulo2/retspecial
deferred to the SubvariableFlow (W3-funcdata/W6) and callarg-values waves; zero
regression (comment-only diff).
