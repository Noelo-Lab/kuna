# w4x-flow-linkage
verdict: ACCEPT-WITH-LOSSES
verifier: opus-4.8 (independent verifier, structural separation honored — porter notes not read)
date: 2026-06-12
gate: `cd rust && cargo test -p kuna-decomp` -> OK (25 test binaries, 0 failures; 625
      lib+integration tests + 9 new verifier tests = green). `tests/flow_linkage.rs`:
      1 passed (real `process_instruction` over 5 arch lift fixtures, output/input
      `(space,offset,size)` triples byte-for-byte). `tests/verify_w4x_flow_linkage.rs`:
      9 passed. `cargo clippy -p kuna-decomp --lib` and `--test verify_w4x_flow_linkage`:
      clean (`-D warnings`).

## Scope

The branch turns the LOSS-035/036/037 op-output / code-ref *stubs* into real
implementations:

- `Funcdata::banks_mut(&mut self) -> (&mut VarnodeBank, &mut PcodeOpBank)` — the
  single split-borrow accessor every `vbank.setDef`/`createDef`/`setInput` caller
  needs so the `xref` read-repointing callback (`replace_reads_thunk`) can reach
  `obank` mid-`vbank` mutation (funcdata.rs).
- `Funcdata::opSetOutput` (funcdata_op.cc:70) — full body: two-`opUnsetOutput`
  ladder, `vbank.setDef` via `banks_mut`, `setVarnodeProperties`, `op->setOutput`.
- `Funcdata::opDestroyRaw` (funcdata_op.cc:253) — inputs, then output, then op.
- `Funcdata::newVarnodeOut`/`newUniqueOut`/`newCodeRef`/`setInputVarnode`
  (funcdata_varnode.cc:106/131/224/342).
- `PcodeEmitFd::dump` (funcdata.cc:875) rewritten in flow.rs `FlowEmit::dump`:
  real output link + code-ref input + ordinary inputs; the infallible
  `PcodeEmit::dump` captures the first factory error in `FlowEmit::error` and
  `process_instruction` re-raises it at the C++ exception-unwind point.

## Re-derivation (C++ vs Rust, statement-for-statement)

- `opSetOutput`: C++ funcdata_op.cc:73-86 — early-out, `opUnsetOutput(op)`,
  `opUnsetOutput(vn->getDef())`, `vn=vbank.setDef(vn,op)`, `setVarnodeProperties`,
  `op->setOutput(vn)`. Rust funcdata_op.rs:160-187 reproduces the order exactly;
  the `set_def` return value (post-xref unification) is assigned back to `vn`
  before `set_output`. ✔
- `PcodeEmitFd::dump` order: C++ creates the op + `newVarnodeOut` BEFORE
  `opSetOpcode`; `op->isCodeRef()` is read AFTER the opcode is set (the coderef
  flag is derived from the opcode). Rust matches: output → `op_set_opcode` →
  `is_code_ref()` check → coderef input (slot 0) → loop inputs. ✔ The two `as int4`
  casts (`out.size`, `i`) reproduce the C++ `uint4→int4` / `int4 i` conversions
  faithfully (varnode sizes / input counts are tiny; no truncation). ✔
- descend/xref order (prompt focus): output is built via `createDef`→`xref`
  BEFORE any input is linked (C++ `newVarnodeOut` then the input loop); each input
  `op_set_input` does `add_descend` BEFORE `op->setInput` ("op must be up to date
  AFTER descend_add"). The `replace_reads_thunk` (pre-existing) snapshots the
  descend list before mutating and skips the self-def entry — verified against
  `VarnodeBank::replace` (varnode.cc:1351). ✔
- `setInputVarnode`: overlap pre-check (returns the existing input for an
  identical candidate) before `vbank.setInput`→xref via `banks_mut`; the
  `funcp.hasEffect` unaffected/return-address marks are a documented W4 no-op. ✔
- error→Result partial-state parity: a `dump`-factory error is captured and
  re-raised in `process_instruction` immediately after the `Ok(s)` arm — exactly
  where the C++ generic `LowlevelError` (NOT caught by the Unimpl/BadData catch
  blocks) unwinds out of `oneInstruction`/`processInstruction`, before the
  `visited.insert`/`xref_control_flow` bookkeeping. The partially-built ops remain
  on the dead list (the C++ leaves them too). `dump`'s `if self.error.is_some()
  { return; }` guard means no further ops are built after the first error, so the
  net op set matches. ✔

## The mandatory hunt list

- Signedness: clean. `s: int4` vs `get_min_laned_size(): int4`; size/slot casts
  mirror the C++ conversions. No int4-vs-uintb comparison introduced.
- Integer widths: clean. `out.size` (u32→int4) == C++ `uint4→int4`; no promotion
  surprises (the new factories delegate width math to the pre-existing bank).
- Wrapping: clean. No new arithmetic in the diff; `uniqid += s` (`wadd`) is the
  pre-existing bank code.
- Comparator totality: clean. No new comparators.
- Iteration-order provenance: clean. `op_destroy_raw` loops `0..num_input` by
  index (C++ `for(i=0;...)`); `replace_reads_thunk`/`destroy_varnode` snapshot
  descend (push_back / std::list order). No HashMap/HashSet introduced
  (grep = 0 in the changed files).
- Off-by-one / do-while / reverse iterators: clean. No do-while; `0..n` ranges
  match the C++ `for` bounds.
- Erase-while-iterating: clean. `op_destroy_raw` re-reads `get_in(i)` fresh each
  iteration (no held iterator); the xref/destroy paths collect-then-mutate.
- Exception→Result partial-state parity: clean (verified above — the re-raise
  point and the surviving partial ops match the C++ unwind).

## Mechanical pass

- `port-audit`: not present in this workspace (`w4x-flow-linkage` is a fix branch,
  not a checklist item with a recorded `cpp_blob_sha`); blob-sha drift check N/A.
- greps over the changed source: `todo!`/`unimplemented!`/`HashMap`/`HashSet`/
  `sort_unstable` = 0; the only `panic!` (funcdata.rs:677) is pre-existing and
  out of diff scope. Bare `as` casts in added lines: two `as int4` (size / slot),
  both faithful C++ conversions (noted above).
- clippy: `--lib` and `--test verify_w4x_flow_linkage` clean under `-D warnings`.
  (`--all-targets` trips a pre-existing `doc_lazy_continuation` lint in
  `tests/verify_w4_fw_options.rs`, which this branch does NOT touch and which is
  present on `rust-port` — out of scope; the item gate is `cargo test`, not clippy.)

findings: none (no correctness defect).

losses:
  - LOSS-077 (NEW, appended): `Funcdata::newCodeRef` cannot set
    `Varnode::annotation` (`Varnode::set_flags` private to varnode.rs); the
    code-ref destination Varnode comes back free, `is_annotation()` == false where
    C++ returns true. The `(space,offset,size=1)` triple — all the branch input /
    flow-following reads — is faithful; no W4-boundary consumer branches on
    `isAnnotation()` for a code-ref. This is the single residual sliver of
    LOSS-036/037 after this branch restored the rest of the op-output/coderef
    family.
       cpp: decompiler/cpp/funcdata_varnode.cc:232 (`vn->setFlags(Varnode::annotation)`)
       rust: rust/crates/kuna-decomp/src/funcdata_varnode.rs new_code_ref (~:440 no-op comment)

adversarial tests (rust/crates/kuna-decomp/tests/verify_w4x_flow_linkage.rs, 9):
  - w4x_flow_linkage_op_set_output_replaces_existing_output_and_steals_from_other_op
  - w4x_flow_linkage_op_set_output_idempotent_when_already_set
  - w4x_flow_linkage_new_varnode_out_does_not_clobber_free_at_same_address
        (cross-flag-class non-unification — surfaced and corrected a verifier
        misconception about loc-tree unification; pins the CORRECT behavior)
  - w4x_flow_linkage_new_unique_out_allocates_unique_and_links_def
  - w4x_flow_linkage_new_code_ref_is_size1_annotation_at_target_no_annotation_flag
        (PINS LOSS-077: annotation flag absent)
  - w4x_flow_linkage_dump_branch_links_coderef_input_at_target
  - w4x_flow_linkage_dump_constant_space_input_is_plain_constant_varnode (LOSS-015)
  - w4x_flow_linkage_op_destroy_raw_reaps_inputs_output_and_op
  - w4x_flow_linkage_set_input_varnode_formalizes_and_dedups

## Notes

The adversarial pass paid off: an initial "xref unifies the new output with a free
Varnode at the same address and repoints its readers" test FAILED — re-deriving
the loc-tree comparator (`VarnodeCompareLocDef`, varnode.cc:34-53) showed a *free*
and a *written* Varnode at the same `(addr,size)` are DISTINCT keys (different
flagclass), so no cross-flag-class unification occurs. The port is correct; the
test was rewritten to PIN that correct non-clobber behavior. No defect in the diff.
