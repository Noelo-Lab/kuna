# 03 — SSA & simplification

```yaml
Anchors:
  - decompiler/crates/kuna-decomp/src/p3_dataflow
```

> **STATUS: stub.** Section list is normative; prose lands chapter by chapter
> (draft -> adversarial verify -> commit). See `docs/spec/README.md` for the rules.

## 3.1 Heritage

*Scope:* SSA construction, guard ranges, dead-code delay and the dead-definition gate.

## 3.2 The rule pools

*Scope:* how a rule fires (opcode-indexed worklist to local fixpoint); the ruleaction_1..8 family map by theme.

## 3.3 Sub-variable flow

*Scope:* subflow: shrinking logical values out of larger containers.

## 3.4 Conditional execution

*Scope:* condexe, conditional constants; (kuna) placement variant.

## 3.5 kuna peephole rewrites

*Scope:* addcarrychain, booleanmask, flagcompare, ovlesssimplify, compareform, (kuna) arraystride.

## 3.6 Early passes

*Scope:* coreaction_early: the pre-heritage normalizations.
