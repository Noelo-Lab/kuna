# Opt-in Rust zero values for byte-array stores

A wide zero store can recover a byte-array destination and emit `*p = 0;`, which Rust rejects with E0308. The candidate emits `*p = [0; 16];` for the public repro. Complete parent and candidate bodies, compiler diagnostics and native output are retained without body rewrites. The pinned angr STORE-width tests motivated the audit. A fresh reference capture also retains all 13 functions from this authored C-source fixture using its Rust flavor; that output contains C-style member syntax and unresolved types, so it is not claimed compilable or native-equivalent. The source C is the semantic oracle, and `angr-vs-kuna.txt` retains the exact three-way text.

The STORE value helper accepts only a constant zero whose varnode width exactly matches a nonempty array destination. Elements must be one-byte signed, unsigned or unknown integers with alignment one, excluding enums, and the element count must equal the destination size. It emits a contextually typed `[0; N]`. Nonzero values, nonconstants, other element types and inconsistent widths retain their previous handling. The destination expression, evaluation count and alignment remain unchanged. This avoids inventing a more strongly aligned scalar pointer. Existing diagnostic cast suppression remains effective. The correction requires `--option rustarrayzero on`. The option is off in every preset after the initial default-on build failed the predeclared speed budget.

The registered regression contains 13 complete functions: seven zero stores and six unchanged scalar/copy controls. All 416 C/Rust comparisons pass at O0/O2 over eight source/input patterns and two storage locations. Each comparison checks all 128 bytes, including guards, and callback counts. Scalar 128 storage is aligned; the byte-array stores deliberately use unaligned offsets 17 and 33. The C reference uses malloc storage and memcpy from a genuine 128-bit source. Its initial declared-byte-buffer harness is preserved under `initial-declared-buffer-reference/` as superseded evidence, not the accepted aliasing proof. Rust record padding, size and alignment are asserted to match C. The parent full bundle fails with seven E0308 errors per profile. Six control functions and all C/Go records remain identical.

The previously retained six-function wide-store bundle now also compiles in full and passes 2304 C/Rust comparisons at O0/O2. It covers 64 low/high input combinations, two disjoint destinations and exact source/destination aliasing, comparing all memory/guard bytes. Carry boundaries include complete 128-bit wrap. Only `wide_zero` changes from the parent; the other five bodies remain identical. Prior failures remain in the parent feature's evidence. This establishes these complete bodies, not arbitrary scalar STORE-width or type-recovery correctness.

The tracked native fixture was rebuilt byte for byte from the retained source and exact compiler recipe; `fixture-rebuild.json` records the command, compiler and hashes. Repository-named source files are hash-identical, while their different paths affect DWARF path strings.

Current proof binds to final CLI `96cd669278bc3164b30d3944db0e02d1f2a03722c0c5b75dbae127beb6a41dce`. Native and structural checks were repeated after the stage/corpus rebuild. The successful earlier prototype is preserved separately. Fresh candidate captures cover 72 images, 37908 distinct functions and 138972 language/function records against fresh parent captures from main d5a1042b after its independent P6 performance and watchdog changes. The parent, default-off and enabled arms were all recaptured; older outputs were not reused. Every default-off function record in all three languages is identical to the parent. The following changes refer to explicit `rustarrayzero on`. All 14 changed Rust records are classified, comprising 28 zero-array assignments. C/Go records and variable identity, type, storage and provenance are unchanged. No new engine errors occur; neither side has parser failures among the changed Rust records.

The recursive paired AST audit permits only assignment RHS literal zero to become a positive-length repeat of literal zero. Assignment destinations, casts, calls, ordering and surrounding AST stay identical. Twenty positive/negative controls reject changed targets or casts, nonzero/nonconstant expressions, reordered or duplicated effects, boolean/float/suffixed literals, dynamic/zero lengths and changed return expressions. The same audit classifies all eight changed native records across both bundles with no pending cases. Counts are recorded, but old scalar syntax alone cannot prove the storage width; the production IR predicates and native memory comparison provide that separate evidence.

Probestack is inspected statically and never executed. Arbitrary invalid pointers, other array element types, nonzero values, inconsistent scalar widths and multiarchitecture native equivalence remain outside this fix. Native execution uses x86_64, Rust overflow-checks=off and C -fwrapv; the small public zero-store repro also passes with Rust's default overflow checks.

All four repository checks plus tooling/lint/catalog/counters pass: 10379 workspace tests, 675 original assertions and 3819 stage assertions. Eight stage assertions cover off/on behavior and diagnostic cast suppression. The catalog total assertion was renamed for 378 entries; no existing passing behavior was lost and the original 675 baseline is unchanged.

The initial default-on build failed the fixed 64-pair plan: the small C control measured +5.075% CPU median against the 5% budget, with -0.119% wall median. That path does not call the new Rust helper, so no causal explanation or noise dismissal is asserted. Every raw sample, source snapshot, native proof and broad comparison is retained under `failed-default-on/`. The source was then changed to a named default-off option; the unchanged default-on implementation was not retimed.

The first gated build retained identical default-off corpus output but failed two bookkeeping tests: the live-value count and the explicit preset exclusion. Those failures and complete proof are preserved under `pre-main-validation/`. Both are corrected. Main incorporated independent P6 performance changes from PR1466 and PR1468, then the watchdog correction from PR1465, so the parent was rebuilt and native, corpus and repository checks were repeated from that new base. The intermediate native proof before PR1465 is retained under `pre-budget-main/`; its declared timing plan was never sampled. No old speed number is carried into the new comparison.

The final rebased gated build has a separately predeclared plan: 64 balanced pairs for each of 16 comparisons, three warmups per arm. Eight compare default-off against the frozen parent and must meet the 5% CPU/wall median budget; their maximum delta is 1.492%. Eight measure explicit on versus off on the same final binary; their maximum median delta is 1.259%. The option remains off regardless of those later enabled measurements. The largest 95% upper mean bound across all final comparisons is 3.412%, reported as uncertainty rather than an acceptance threshold. All samples and CPU telemetry are retained. Sources, binaries, fixtures and all 154 compiled specs were verified before and after timing.

| Case | CPU median | Wall median | CPU upper mean bound | Wall upper mean bound |
|---|---:|---:|---:|---:|
| c-zero | -0.965% | -0.205% | +2.183% | +2.150% |
| go-zero | -0.351% | -0.454% | +3.412% | +3.277% |
| rust-zero | +1.492% | +1.419% | +0.678% | +0.728% |
| rust-field | -0.501% | -0.490% | +0.398% | +0.240% |
| rust-callback | -0.433% | -0.782% | +2.357% | +2.375% |
| rust-large | +0.125% | +0.118% | +1.195% | +1.195% |
| rust-count | -0.829% | -0.827% | +0.257% | +0.258% |
| rust-inflate | +0.307% | +0.344% | +0.785% | +0.577% |
| enabled-c-zero | +0.068% | +0.384% | -0.171% | -0.096% |
| enabled-go-zero | +0.340% | +0.491% | +0.281% | +0.339% |
| enabled-rust-zero | +1.259% | +1.196% | +0.447% | +0.417% |
| enabled-rust-field | +0.403% | +0.257% | +0.745% | +0.640% |
| enabled-rust-callback | -0.141% | -0.163% | +0.770% | +0.785% |
| enabled-rust-large | -0.310% | -0.139% | +0.926% | +0.931% |
| enabled-rust-count | -0.024% | -0.022% | +0.313% | +0.321% |
| enabled-rust-inflate | -0.627% | -0.627% | -0.308% | -0.240% |

`record.json` binds the proof to the parent tree and actual merge. Complete changed records are losslessly compressed and round-trip hash-checked; `manifest.json` hashes the evidence files.
