# Rust integer array indices

The public repro changes `return (*p).values[i];` with an `i64` index into `return (*p).values[i as usize];`. Rust requires a target-sized array index. The helper preserves the complete index expression, evaluates it once and gives literal-dependent arithmetic its original integer width before conversion. A bounded context query refuses unknown contexts. Primitive integer/unknown widths 1/2/4/8 are accepted when the varnode size agrees; constants, enums and noninteger types keep their prior path. Cast suppression and plain pointer-addition notation retain their existing behavior. Raw-pointer wrapping is a separate path.

The registered native test uses 45 complete emitted Rust bodies without rewriting them. At O0/O2, 23,112 function/input comparisons agree with C, checking signed/unsigned indices and element widths, valid array addresses, nested arrays, every store and guard byte, callback order and all 64 shift counts over boundary-valued memory. Both already-valid constant-index controls remain identical. The parent bundle has 41 E0277 errors at each optimization level. Rust uses overflow-checks=off and C uses -fwrapv on x86_64. The first successful prototype is retained separately: rebuilding after the stage/corpus counter changed the executable, so native captures were repeated on the final binary before validation.

The candidate was captured fresh across 72 images, 37908 distinct functions and 138972 language/function records. Its exact frozen parent captures come from published pointer PR1463 and are hash-verified against parent binary `07ec71b5f28e84d003428f78738196f1edd7ec5c6220c36f5db018b49724b0d3`. All 2726 changed Rust records are classified, with 10274 index casts and 0 literal-width contexts. The AST audit removes only a new usize cast whose operand matches the original index, optionally with the matching primitive integer identity wrapper. Existing indices/casts, operands, dereferences, calls, assignments, address expressions and the rest of the AST must remain identical. Eighteen positive/negative controls cover wrong operands, narrowed casts, changed bases, lost conversions, nested indices and duplicated calls.

C/Go records remain identical. Variable names, types and storage remain unchanged; all 0 changed provenance records match the exact union of their source-line mappings. There are no new engine or parser errors. The 24 inherited raw Rust parser failures retain their diagnostics, with identifier normalization only for structural review. The upstream probestack binary is inspected statically and never executed. Structural review does not establish whole-runtime native equivalence or complete runtime type correctness.

The address semantics remain a separate gap. A complete emitted array-address function matches C for valid indices 0 and 8, but valid C one-past index 9 returns an address in C and panics in Rust. Its unrevised body and raw outputs are archived in `remaining-array-address/`. This index conversion does not repair that reference/bounds behavior, raw-pointer fallback syntax, global byte-scale recovery, uninitialized local arrays or arbitrary nested-subpiece integer contexts. No multiarchitecture native parity is claimed.

All four repository checks plus tooling/lint/catalog/counters pass: 10372 workspace tests, all original 675 assertions, and 3794 stage assertions. Ten new stage assertions cover loads, stores, valid addresses, nested arrays, callbacks, literal-width contexts, constants and diagnostic cast suppression. No prior assertion was lost, and the original baseline is unchanged.

The fixed speed plan uses 64 balanced pairs across eight cases, three warmups per arm, and a 5% CPU/wall median budget. Every case passes; the largest median delta is 0.667%. The largest 95% upper mean bound is 10.985%, reported as uncertainty rather than an acceptance threshold. Every sample and CPU telemetry record is retained. Source manifests, binaries, fixtures and all 154 compiled specs were verified before and after timing.

| Case | CPU median | Wall median | CPU upper mean bound | Wall upper mean bound |
|---|---:|---:|---:|---:|
| c-array | +0.667% | +0.642% | +1.088% | +10.985% |
| go-array | +0.328% | +0.281% | +0.681% | +10.258% |
| rust-array | +0.403% | +0.419% | +0.844% | +7.448% |
| rust-callback | +0.085% | +0.180% | +1.021% | +7.623% |
| rust-index-context | +0.414% | +0.545% | +1.312% | +0.197% |
| rust-large | +0.080% | +0.492% | +0.531% | +7.778% |
| rust-count | +0.063% | +0.649% | +0.800% | +6.273% |
| rust-inflate | -0.461% | -0.284% | +5.036% | +5.566% |

`record.json` binds this proof to the exact published parent tree and its actual merge. The complete changed records are losslessly compressed and round-trip hash-checked. `manifest.json` hashes the evidence files.
