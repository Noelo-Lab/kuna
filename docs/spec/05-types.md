# 05 — Types

```yaml
Anchors:
  - decompiler/crates/kuna-decomp/src/p5_types
  - decompiler/crates/kuna-decomp/src/substrate/dtype.rs
```

> **STATUS: stub.** Section list is normative; prose lands chapter by chapter
> (draft -> adversarial verify -> commit). See `docs/spec/README.md` for the rules.

## 5.1 Type representation

*Scope:* dtype: the factory, type classes, structs/unions (shared IR, lives in substrate).

## 5.2 Inference

*Scope:* coreaction_infertypes: the bounded propagation lattice; constant pointers; typeop transfer functions.

## 5.3 Ranges & consume bits

*Scope:* rangeutil.

## 5.4 Union resolution

*Scope:* unionresolve.

## 5.5 Double precision

*Scope:* double.rs: the *Form rule families.

## 5.6 kuna extensions

*Scope:* memsetsequence/memsetrecover, constseq, inferfuncentry, thumbfuncptr, bitfield, prefersplit.
