# 00 — Overview & machinery

```yaml
Anchors:
  - decompiler/crates/kuna-decomp/src/substrate
  - decompiler/crates/kuna-decomp/src/p0_knowledge
  - decompiler/crates/kuna-decomp/src/infra
  - decompiler/crates/kuna-cli/src
  - decompiler/crates/kuna-console/src
  - decompiler/crates/kuna-ghidra/src
```

> **STATUS: stub.** Section list is normative; prose lands chapter by chapter
> (draft -> adversarial verify -> commit). See `docs/spec/README.md` for the rules.

## 0.1 The two tiers

*Scope:* program preparation (kuna-analysis) vs the per-function decompiler pipeline (kuna-decomp); the AnalysisOutput stash -> commit_pending_analysis hand-off contract.

## 0.2 Front-ends and the decompile-all walk

*Scope:* kuna-cli decompile-all in-process path; the console (decomp_dbg) and its XML datatest path; the kuna-ghidra process front-end (phase 1, engine-stubbed); load-time env bridges.

## 0.3 The IR substrate

*Scope:* Varnode/PcodeOp/BlockGraph/Funcdata; arena ids; the Funcdata impl map (which phase folder owns which funcdata_* impl block); dtype as shared IR.

## 0.4 The knowledge plane (P0)

*Scope:* symbol Database/Scopes, Overrides, the assertion store (kassert), OptionDatabase + generated OptionValues; effective defaults (single narrative).

## 0.5 The two Architecture types

*Scope:* the engine god object vs the per-function ArchContext snapshot (glb); build_arch_handle and the flag-copy hazard.

## 0.6 The schedule

*Scope:* universal_sched: one declarative pass tree, filtered per root variant; fullloop/mainloop/stackstall; the oppool rule pools; Band B (P3-P6 mutual fixpoint); the restart model.

## 0.7 Feedback edges

*Scope:* the typed feedback-edge inventory re-anchored to Rust (jump-table restart, proto-change restart, dead-code delay, lowered-switch two-halves, ...).

## 0.8 Reading order

*Scope:* folder taxonomy != execution order; per-chapter map; the foo.rs + foo/tests.rs convention.
