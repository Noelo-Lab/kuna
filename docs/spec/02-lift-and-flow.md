# 02 — Lift & flow recovery

```yaml
Anchors:
  - decompiler/crates/kuna-decomp/src/p2_lift
```

> **STATUS: stub.** Section list is normative; prose lands chapter by chapter
> (draft -> adversarial verify -> commit). See `docs/spec/README.md` for the rules.

## 2.1 Instruction following

*Scope:* FlowInfo, decode-error policy, inlining, p-code injection (injection library, user ops, call fixups).

## 2.2 CFG construction

*Scope:* resolve-flow, lazy block generation; (angr) tail-call jump recovery; (kuna) stack-probe loops; (kuna) V850 indirect branch.

## 2.3 Jump tables & switch recovery

*Scope:* the JumpBasic model (pathMeld, guards, range pullback, address enumeration); failure mode (Too many branches -> CALLIND demotion); (angr) loweredswitch cascade recovery; (angr) guard/modulo bound extensions; shared-case; multipred; unrolledguard.

## 2.4 No-return at lift time

*Scope:* (angr) extern-name matching; fall-through suppression.

## 2.5 Arch quirks

*Scope:* (kuna) SPARC struct-return; emulate-function hooks.
