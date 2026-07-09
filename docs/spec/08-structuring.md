# 08 — Structuring

```yaml
Anchors:
  - decompiler/crates/kuna-decomp/src/p8_structure
```

> **STATUS: stub.** Section list is normative; prose lands chapter by chapter
> (draft -> adversarial verify -> commit). See `docs/spec/README.md` for the rules.

## 8.1 Collapse structuring

*Scope:* CollapseStructure: the schema cascade (goto/cat/if/ifelse/whiledo/dowhile/infloop/switch) with TraceDAG goto selection; when it still runs (irreducible fallback).

## 8.2 The region structurer

*Scope:* (angr) Phoenix/SAILR: schema matching over the region tree, edge virtualization ordering, loop refinement, last-resort refinement.

## 8.3 The goto-quality pass family

*Scope:* (angr) gotoreduce, taildup, ifelseflatten, crossjumprevert, dedupitetail, iteregion, earlyreturn, switchreturn, branchflip, loopbreak recovery; returndup (shipped then reverted, DIV-18 - history).

## 8.4 The quality signal

*Scope:* goto counting, the quality command, speculate-validate-rollback.
