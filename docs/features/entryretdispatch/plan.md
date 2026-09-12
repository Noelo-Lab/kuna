# entryretdispatch design

1. Reuse `kuna_retcallchain` as the single recognizer for explicit and automatic
   RET-call chains. Strengthen its provenance rather than add a second heuristic.
2. Track affine addresses with signed constants, overlapping register aliases,
   exact stores/loads, and conservative invalidation. Stop at conditional flow.
3. At the shared decompile boundary, run entry recognition only when the new P2
   option is enabled. Merge derived CALLs with explicit flow facts so explicit
   caller intent wins; treat explicit RETURN as a veto, not an engine override.
4. Add the default-on catalog option, stable ElementId, two-pass stage test,
   focused CLI/direct-recognizer controls, fixture documentation, spec text,
   acceptance recapture, DIV record, catalog regeneration, and speed record.
5. Verify 0/675 datatest movement, the complete stage/Rust/CLI/spec/catalog gates,
   option-off restoration, the bm3 witness, merge safety, and the 5% speed budget.
