# 07 — Region identification (angr)

```yaml
Anchors:
  - decompiler/crates/kuna-decomp/src/p7_regions
```

> **STATUS: stub.** Section list is normative; prose lands chapter by chapter
> (draft -> adversarial verify -> commit). See `docs/spec/README.md` for the rules.

## 7.1 The region graph

*Scope:* (angr) kuna_regiongraph: deterministic mutable digraph substrate.

## 7.2 The region identifier

*Scope:* (angr) KunaRegionIdentifier: loop regions, acyclic regions, guard caps, panic mode.

## 7.3 kuna passes at the region tier

*Scope:* (kuna) stackguard (REMOVES canary code).

## 7.4 Observability

*Scope:* the region tree/blocks/walk console commands.
