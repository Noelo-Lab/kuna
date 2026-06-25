# plan — foldcallret (call-return variable folding)

## Option
- name: `foldcallret` (slug `call-return-variable-folding-dcde82`)
- arch flag: `fold_call_returns: bool`, default **OFF**
- stage S6 / `explicit-marking` (ActionMarkExplicit, `merge-aggressiveness` group)

## New module
`decompiler/crates/kuna-decomp/src/s6_variables/kuna_callretfold.rs`
- `pub fn call_output_foldable(data: &Funcdata, vn: VarnodeId) -> bool` — the
  order-safety predicate (decider-refined):
  fold iff the call output has exactly one **non-marker** descendant, that use op
  is in the **same basic block** as the call op and is **not** a
  MULTIEQUAL/INDIRECT marker, and **no** op strictly between the call and its use
  (in block op order) is CALL/CALLIND/CALLOTHER/STORE/LOAD (or any `is_call`).
- `pub struct OptionFoldCallRet` with `apply(p1) -> KunaResult<(bool, String)>`
  (the on/off parse + confirmation message), mirroring `OptionLowerSwitch`.

## Anchor edits (minimal, `// (kuna)` marked)
1. `s6_variables/coreaction_cleanup.rs::base_explicit` — in the `is_call()` arm,
   gate: when `data.get_arch().fold_call_returns && call_output_foldable(data,vn)`
   fall through to the descendant-count path instead of `return -1`.
2. `substrate/seams.rs` — `fold_call_returns: bool` field (read by base_explicit),
   default false in the seam ctor.
3. `infra/architecture.rs` — field decl, `false` in ctor, reset default (off),
   `"foldcallret"` parse arm, copy into seam in `build_arch_handle`.
4. `p0_knowledge/options.rs` — add `"foldcallret"` to `KUNA_OPTION_NAMES`.
5. `decompiler/crates/kuna-decomp/stages.toml` — settable row (full provenance).
6. `s6_variables/mod.rs` — `pub mod kuna_callretfold;`

## Test
`tests/stages/ghangr-foldcallret.xml` (slug file
`ghangr-call-return-variable-folding-dcde82.xml`): two-pass — pass 1 `option
foldcallret off` asserts the spilled `vN = call(); if (vN ...)`; pass 2 default-on
in the test asserts the folded `if (call(...) ...)`.

## Default decision
Default-OFF opt-in (ablation will almost certainly change >0 datatest assertions
since this broadly changes call-output rendering). Speed measured regardless.
