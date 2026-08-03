# funcptralign - plan

## Design

Replace the `funcptr_align()` STUB(W4) accessor in
`decompiler/crates/kuna-decomp/src/p3_dataflow/ruleaction_8.rs` with the live
architecture read, `data.get_arch().funcptr_align` - the same accessor
`p2_lift/jumptable.rs:3661` and `p3_dataflow/ruleaction_6.rs:609` already use.

Nothing else in `RuleFuncPtrEncoding` changes: the rule body is already a
line-for-line transcription of upstream `ruleaction.cc:9928`, it is already
registered in the pool (`infra/universalaction.rs:515`), and the architecture
plumbing that populates `funcptr_align` from the cspec's `<funcptr align="N"/>`
already exists and is already proven live at runtime by
`tests/stages/gh8471-thumbfuncptr.xml`.

Scope: **small**. One function body, no new module, no new option, no new
registration point, no phase-order change.

## Why no option

See `analysis.md` (Why no option). Short form: the alignment width is declared
by the *processor cspec*, upstream honours it unconditionally, the pre-fix C
stated an operation the program does not perform, and both parity gates stay
green with the fix live and no re-pin. `CLAUDE.md`'s carve-out - "a strict bug
fix that only corrects wrong output needs no flag" - applies. It gets a DIV row
because the registry records no-flag bug fixes (DIV-43...DIV-53).

## Risks and how each was closed

1. **Broad ARM text churn.** ~4,000 masks disappear corpus-wide and the
   `(code *)` casts turn into `(code **)` loads. Closed by ablation: 675/675
   datatests and the full stages corpus stay PARITY OK with no re-pin and no
   per-test opt-out (the two ARM datatests and the six MIPS ones contain no
   masked indirect call). Whole-binary x86-64 controls are byte-identical.
2. **`RulePtrsubUndo::preserve_thumb_funcptr` (GH-8471 / DIV-2) interaction.**
   The two features touch the same low bit from opposite ends. Closed by the
   existing `tests/stages/gh8471-thumbfuncptr.xml` (still 2/2) and by pass 2 of
   the new stage test, which re-runs the same function with
   `option thumbfuncptr off` and asserts the mask stays stripped either way.
   They cannot collide by construction: this rule only removes an `INT_AND` that
   a `CALLIND` consumes, and it rewrites it to a transparent `COPY`, so any
   other reader of the masked value keeps its value.
3. **MIPS / AARCH64 / Loongarch / 8051 also declare `<funcptr>`.** Measured
   rather than assumed - see `analysis.md`, which records the stock-Ghidra-12.1
   cross-check on AArch64 `align=4` and the MIPS `JXWritePC` lifting shape.

## Artifacts

- `decompiler/crates/kuna-decomp/src/p3_dataflow/ruleaction_8.rs` - the accessor,
  the stale STUB(W4) module-header note, and three rule-level unit tests
  (align=0 inert, align=1 strips `& ~1`, align=2 strips `& ~3` and declines
  `& ~1`).
- `tests/stages/ghdec-funcptralign.xml` - two-pass ARM/Thumb end-to-end test
  (0/2 pre-fix, 2/2 post-fix), plus the corpus-file-count bump in
  `decompiler/crates/kuna-base/src/xml.rs` and the re-recorded
  `docs/baseline-stages.json`.
- `docs/spec/03-ssa-and-simplification.md` section 3.2 - the normative prose.
- `docs/history.md` - the DIV row.
- `docs/features/funcptralign/` - this bundle.

No `phases.toml` row, no `options.rs` registration, no catalog-count bumps: no
option is added.
