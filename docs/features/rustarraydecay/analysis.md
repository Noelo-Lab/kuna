# Rust array-to-pointer conversions

The public repro returns a nine-element array through an integer pointer return type. The parent emits `return *p;`, rejected by Rust with E0308. The candidate emits `return &raw mut *p as *mut i32;`, which compiles and returns the same address as C. Complete parent and candidate bodies and compiler diagnostics are retained without body rewrites.

The conversion applies at typed pointer returns, assignments, STORE values, known call parameters and explicit casts. It shares the raw-address array-place proof: an implied PTRSUB at the exact start of a nonempty struct array field, or offset zero through a pointer to a nonempty array. It requires matching 32/64-bit byte-addressed nonrelative pointer types. A bounded search follows at most seven implied same-width pointer COPY/CAST operations, preserving the address while selecting the actual consumer type. It does not cross loads, calls, integer conversions or explicit variables. Taking the raw address evaluates the place once without creating a reference or loading array values. Unknown parameters and unproved places keep their previous paths; cast suppression remains diagnostic. Array projection requires valid containing storage, and arbitrary null/dangling bases are not covered.

The registered regression uses 97 complete emitted bodies: twelve element types across field returns, whole-array returns, constant one-past addresses, erased pointers, explicit byte-pointer casts, pointer-to-array results, guarded pointer stores and call arguments, plus a callback-order case. The candidate passes 776 C/Rust comparisons at O0/O2 over readable and PROT_NONE mappings and two base locations. Store guard words and callback counts agree exactly. The parent bundle reports 97 E0308/E0605 errors per profile. The initial candidate still failed twelve erased-pointer returns because hidden pointer casts obscured the declared consumer type; the complete failure is retained in `initial-prototype-a547/`. The bounded pointer-chain search fixes those returns. The original 73-case parent fixture is also preserved; it was expanded to 97 before implementation or timing to include explicit casts and pointer-to-array results.

Four previously retained constant-address bodies now compile and pass eight additional C/Rust comparisons across O0/O2. The separate full six-function wide-store bundle still fails at `*p = 0` for a recovered `[u8; 16]` destination. Its prior array-to-pointer errors are repaired, but no native equivalence is claimed for this still-failing six-body bundle. The previous raw-array-address feature's evidence retains its original failures; current full captures and diagnostics are under `extras/`.

All current proof binds to final CLI `a1bc22c5d2a4b10662a9338ed48d1a2f85144ef4a9178b30b19eec7a76305bda`. The native and structural checks were repeated after the stage/corpus rebuild. Fresh candidate captures cover 72 images, 37908 distinct functions and 138972 language/function records against hash-verified frozen parent captures from merged PR1467. All 462 changed Rust records are classified, comprising 1434 array-to-pointer conversions. The paired AST audit allows a raw mutable address plus a pointer cast only at the corresponding original expression. At that location, up to seven pointer casts may be subsumed, with every old and new target recorded. Place expressions, integer casts, calls, evaluation order and surrounding AST remain identical. The 24 positive/negative controls reject changed places, integer cast removal, references, reordered or duplicated calls, unrelated pointer casts and changed memory operations. This establishes structural scope; old syntax alone does not establish the consumer pointer type or whole-runtime correctness.

C/Go records remain identical. Variable identity, type, storage and provenance remain unchanged. No new engine errors occur; neither side has a Rust parser failure among the changed records. Probestack is inspected statically and never executed. Unproved global/local arrays, wide scalar STORE repair and multiarchitecture native equivalence remain outside this fix. Native execution uses x86_64, Rust overflow-checks=off and C -fwrapv.

All four repository checks plus tooling/lint/catalog/counters pass: 10374 workspace tests, 675 original assertions and 3811 stage assertions. Nine stage assertions were added; no existing passing key was lost and the original baseline is unchanged.

The predeclared speed plan uses 64 balanced pairs across eight cases, three warmups per arm and a 5% CPU/wall median budget. Every case passes; the largest median delta is 0.441%. The largest 95% upper mean bound is 2.970%, reported as uncertainty rather than an acceptance threshold. All samples and CPU telemetry are retained. Sources, binaries, fixtures and all 154 compiled specs were verified before and after timing.

| Case | CPU median | Wall median | CPU upper mean bound | Wall upper mean bound |
|---|---:|---:|---:|---:|
| c-array | +0.253% | +0.266% | +0.337% | +0.269% |
| go-array | +0.257% | +0.276% | +0.443% | +0.439% |
| rust-array | -0.196% | -0.216% | +0.156% | +0.128% |
| rust-callback | +0.405% | +0.441% | +0.490% | +0.507% |
| rust-index-context | +0.284% | +0.229% | +0.360% | +0.377% |
| rust-large | -0.355% | -0.355% | -0.172% | -0.175% |
| rust-count | -1.113% | -1.170% | +0.018% | -0.038% |
| rust-inflate | -0.435% | +0.090% | -0.234% | +2.970% |

`record.json` binds the proof to the parent tree and actual merge. Complete changed records are losslessly compressed and round-trip hash-checked; `manifest.json` hashes the evidence files.
