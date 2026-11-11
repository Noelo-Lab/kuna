# Rust primitive integer conversions

Rust returns, copies and modular binary operations now convert primitive integers when the rendered value and its destination have the same 1-, 2-, 4- or 8-byte width but different signedness. The public reproduction changes `return value;` to `return value as u64;` for an `i64` argument and `u64` result. Noninteger values, enums and unsupported widths retain their existing handling. The internal cast-suppression setting remains honored.

Context-dependent literals retain their source integer width through an explicit `::core::convert::identity::<T>` expression before conversion. This also covers existing same-width casts around shifts. Casts at the end of nested expressions receive parentheses where a following comparison or shift token would otherwise be parsed as generic arguments. Compound assignment expands only when the emitter's existing safe-place rules permit it; call values are evaluated once.

The registered native tests compare 64 complete emitted bodies against C at O0/O2: 70,152 function/input comparisons. Rust uses `overflow-checks=off` and C uses `-fwrapv` to model machine wrapping arithmetic. The bodies are not rewritten. Coverage includes signed/unsigned widths, high bits, constants, operators, nested shifts, comparison and shift grammar boundaries, and exact callback counts in loops. Four supplementary complete right-shift bodies add 512 comparisons over all 64 shift counts; these are archived separately and are not claimed as registered cargo tests.

The static comparison covers 72 images, 37908 distinct functions and 138972 language/function records. All 12742 changed Rust records are classified; C and Go output is unchanged. No new decompiler or Rust parser failures were found. Twenty-eight previously invalid records now parse, while 66 retain existing syntax failures. The corpus includes static-only inspection of the upstream `probestack` image; it was never executed.

The structural audit strips primitive casts and exact integer identity calls, expands safe local compound assignments, and canonicalizes modular arithmetic while preserving operand order. For the 94 records with a raw parser failure in either version, review-only lexical cast erasure and reserved-identifier renaming permit comparison; original output and parse status remain archived. Thirteen audit controls exercise accepted transformations and rejection of changed call order, duplicated calls, changed literals/operators/precedence and repeated pointer evaluation. This comparison does not establish native equivalence, type correctness, or preservation of existing casts for whole runtime images. Those claims remain bounded by the source guards and targeted native tests.

All four repository checks pass, including 10361 workspace tests, all original 675 assertions and 3754 stage assertions. The original baseline is unchanged. The stage total grows by 12: eleven new cases and one exact Rust call-result conversion check. Catalog metadata is unchanged.

The speed plan was declared before the integrated build or sampling: 64 balanced pairs per case, three warmups per arm, CPU and wall median deltas at most 5%. All cases pass. The 95% upper bounds on mean deltas report uncertainty and are not acceptance thresholds. The maximum measured median delta is 0.413%; the largest reported upper bound is 0.667%. Raw samples and CPU telemetry are retained.

| Case | CPU median | Wall median | CPU upper mean bound | Wall upper mean bound |
|---|---:|---:|---:|---:|
| c-conversion | -0.939% | -0.957% | +0.028% | +0.033% |
| go-conversion | -0.388% | -0.405% | -0.362% | -0.363% |
| rust-conversion | -0.433% | -0.436% | -0.340% | -0.339% |
| rust-shift | -1.185% | -1.173% | -0.604% | -0.521% |
| rust-large | +0.065% | +0.066% | +0.667% | +0.657% |
| rust-count | -0.846% | -0.840% | -0.268% | -0.263% |
| rust-inflate | +0.413% | +0.340% | -0.018% | -0.157% |

Earlier prototypes exposed a literal shift-width regression, an inherited explicit-cast context bug, and nested cast parsing errors. Their failed probes and outputs remain in `diagnostics/`; passing evidence applies only to the exact candidate identified in `record.json`. Both executables, production sources and all 154 compiled specs were verified before and after timing. `manifest.json` hashes the archive. STORE conversion/width handling, raw-pointer indexing, callback types and Rust enum recovery remain separate work.
