# Rust raw-pointer indexing on the integrated main tree

The public repro changes `return p[i];` for a raw pointer into `return *p.wrapping_byte_offset((i as isize).wrapping_mul(4));`. The emitter preserves PTRADD's byte scale and modular address arithmetic, uses one dereference for a load or store, and leaves an address as a raw pointer. Literal-dependent indices retain their integer width. Base and index expressions are evaluated once; differing output pointee types receive a pointer cast. Matching 32/64-bit raw pointer/index widths and byte-addressed pointers are required. Constant bases, relative pointers, implied PTRSUB/COPY bases and array-valued sources keep their prior paths, as does cast suppression.

Main advanced to `0b7117e70151eccfcb54010011245a2dd27c26b5` with the persistcalls dataflow change before this fix was published. The earlier completed proof is retained in [the parent archive](../analysis.md). This archive validates both freshly built binaries on the new parent. Only the corpus counter and stage baseline conflicted; the count includes both new stage files and the baseline was regenerated. The pointer implementation itself is unchanged.

The registered test executes 57 complete emitted Rust bodies at O0/O2 against C, making 29,544 function/input comparisons. It checks all destination and guard bytes, signed and unsigned element widths 1/2/4/8, valid positive/negative indices, record fields, modular address bits, callback order and all 64 shift counts. Five supplementary complete bodies add 1,660 comparisons for pointer-result casts and scaled byte memory accesses. They are archived separately and are not registered cargo tests. Rust uses `overflow-checks=off`; C uses `-fwrapv`. No emitted body is rewritten. A fresh nine-body compile survey retains two unresolved array-valued index failures: an array field and the inner index of a nested array.

Both arms were captured fresh across 72 images, 37908 distinct functions and 138972 language/function records. All 15488 changed Rust records pass the AST audit; C/Go records are unchanged. The audit restores original indexing/address syntax while preserving the base, index, dereference, existing casts and surrounding AST. Newly explicit pointer-result casts are recorded. Instruction scales are recorded, but the old AST alone cannot independently establish them. Twenty-six controls cover preserved and added casts in the same and different statements, changed operands, lost dereferences, nonwrapping operations and duplicated calls.

Variable names, types and storage remain identical. All 680 changed variable provenance records have addresses independently checked as the exact union of their source-line mappings. There are no new decompiler or parser errors. The 70 inherited parser failures retain their raw diagnostics and use identifier normalization only for structural review. The upstream `probestack` fixture was decompiled statically and never executed. Structural comparison does not establish native equivalence or full type correctness for runtime images.

All four repository checks and tooling/lint/catalog/counter checks pass: 10371 workspace tests, all original 675 assertions, and 3784 stage assertions. Eight stage assertions were added relative to the integrated main baseline without losing prior assertions. The original baseline remains unchanged.

The fresh speed plan uses 64 balanced pairs across eight cases, with three warmups per arm. Every CPU and wall median delta is within 5%; the largest is 0.738%. The largest 95% upper mean bound is 1.054%; bounds describe uncertainty and are not acceptance thresholds. Every raw sample and CPU telemetry record is archived. Sources, binaries and all 154 compiled specs were checked before and after timing.

| Case | CPU median | Wall median | CPU upper mean bound | Wall upper mean bound |
|---|---:|---:|---:|---:|
| c-pointer | -0.312% | -0.300% | -0.065% | -0.069% |
| go-pointer | -0.511% | -0.544% | -0.103% | -0.117% |
| rust-pointer | -0.121% | -0.140% | +0.175% | +0.164% |
| rust-callback | -0.382% | -0.390% | -0.045% | -0.018% |
| rust-index-context | +0.035% | +0.015% | +0.028% | +0.025% |
| rust-large | +0.737% | +0.738% | +1.054% | +1.054% |
| rust-count | +0.028% | +0.010% | +0.348% | +0.373% |
| rust-inflate | -0.286% | -0.611% | -0.094% | +0.415% |

`record.json` binds this proof to the exact current-main parent tree. The complete changed records are compressed losslessly and round-trip hash-checked. `manifest.json` hashes this archive. Array indices, multiarchitecture native proof, Go aggregate ABI and Rust enum recovery remain separate work.
