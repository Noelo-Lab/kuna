# 01 — Program preparation (kuna-analysis)

```yaml
Anchors:
  - decompiler/crates/kuna-analysis/src
  - decompiler/crates/kuna-decomp/src/p1_partition
```

> **STATUS: stub.** Section list is normative; prose lands chapter by chapter
> (draft -> adversarial verify -> commit). See `docs/spec/README.md` for the rules.

## 1.1 The tier contract

*Scope:* AnalysisPass -> AnalysisOutput facts -> stash -> gate-filtered commit; why the tier never calls the pipeline live; env-var bridges for load-time gates.

## 1.2 Load image

*Scope:* ObjectLoadImage: ELF, ET_REL relocatables, Mach-O fat slices/arm64e, PE, COFF.

## 1.3 Loader markup

*Scope:* PLT/GOT import naming per arch, i386-PIE stubs, IAT, ARM/MIPS mapping markers.

## 1.4 Metadata analyzers

*Scope:* strings, DWARF (types/lines/globals; (ida) global naming), demangle, RTTI, ObjC, Go pclntab, source-language detection, format strings, FID, PDB.

## 1.5 Entry discovery

*Scope:* entry oracles, funcstart_patterns, address tables.

## 1.6 The Listing tier

*Scope:* deferred whole-image build at commit; decode/classify/xref walk; consumers.

## 1.7 The no-return family

*Scope:* known names; discovered (>=3 evidence); (angr) call-graph propagation fixpoint; error/reach variants; lift-time extern matching.

## 1.8 In-engine image binding

*Scope:* p1_partition: sleigh_arch / raw_arch / xml_arch.
