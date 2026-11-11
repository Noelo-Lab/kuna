# Rust integer stores

The emitter converts a STORE value to the rendered pointer's integer type when both primitive widths match but signedness differs. The public example changes `*p = value;` to `*p = value as i64;` for an `i64` pointer and a `u64` value. Supported widths are 1, 2, 4 and 8 bytes. The shared integer conversion helper preserves literal-width context and declines enums, noninteger values and unsupported widths. Cast suppression remains honored. The pointer and complete value expression are each emitted once.

The registered test compiles 33 complete emitted Rust bodies at O0/O2 and compares them with C: 21,384 function/input comparisons across 324 boundary pairs. It checks every destination and guard byte, sibling fields, nested fields, and exact pointer/value callback counts. Rust uses `overflow-checks=off`; C uses `-fwrapv`. No emitted body is rewritten. The inherited `array_store` raw-pointer indexing error remains excluded from native compilation, with both versions and compiler diagnostics retained.

Another 48 complete widening/narrowing bodies cover all distinct 1/2/4/8-byte width pairs and both signedness choices. Their 1,728 supplementary comparisons also match C and preserve guard bytes. These probes are archived separately and are not registered cargo tests. They demonstrate ordinary compiler-generated stores, not arbitrary inconsistent inferred types, unknown 128-bit values or an independent repair of memory-operation width.

The static comparison covers 72 images, 37908 distinct functions and 138972 language/function records. All 3594 changed Rust records pass the strict assignment audit: removing newly added primitive casts or exact integer identity wrappers around complete assignment right-hand sides restores the parent syntax tree, including every existing cast, destination and other expression. C and Go records are unchanged. No new decompiler or parser failures appear. The 22 inherited parser failures use identifier-only normalization for structural review; original bodies and errors remain retained. The upstream `probestack` image was inspected statically and never executed.

Thirteen audit controls cover whole values, retained casts, literal context, keyword fields, changed destinations, partial casts, lost casts and reordered/duplicated calls. The initial report left four `.fn as i64` records pending because the review helper mistook `fn` before `as` for a declaration. The corrected helper and its new control classify those records without a production change. This structural comparison does not establish native equivalence or type correctness for whole runtime images.

All four repository checks pass: 10362 workspace tests, all original 675 assertions and 3759 stage assertions. Five stage assertions were added without losing prior ones; the original baseline and catalog metadata are unchanged. Production sources stayed fixed throughout validation and timing.

The declared speed plan uses 64 balanced pairs per case and three warmups per arm. Every CPU and wall median delta is within the 5% budget. The largest measured median delta is 0.781%; the largest 95% upper mean bound is 1.010%. Bounds describe uncertainty and are not acceptance thresholds. Every raw sample and CPU telemetry record is retained.

| Case | CPU median | Wall median | CPU upper mean bound | Wall upper mean bound |
|---|---:|---:|---:|---:|
| c-store | -0.943% | -0.951% | -0.098% | -0.100% |
| go-store | +0.260% | +0.252% | +0.957% | +0.947% |
| rust-store | -0.164% | -0.143% | +0.468% | +0.478% |
| rust-sideeffect | -0.215% | -0.201% | +0.173% | +0.256% |
| rust-large | -0.753% | -0.751% | +0.245% | +0.244% |
| rust-count | +0.182% | +0.254% | +0.979% | +1.010% |
| rust-inflate | +0.445% | +0.781% | +0.282% | +0.461% |

`record.json` identifies the candidate and the exact parent tree, which matches its actual merge. Both binaries and all 154 compiled processor specs were checked before and after timing. `manifest.json` hashes the evidence archive. Raw-pointer indexing, arbitrary store-width/type inconsistencies, and Rust enum recovery remain separate work.
