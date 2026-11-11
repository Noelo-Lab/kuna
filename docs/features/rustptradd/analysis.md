# Rust raw-pointer indices

Raw pointers cannot use Rust array indexing. The public repro changes `return p[i];` to `return *p.wrapping_byte_offset((i as isize).wrapping_mul(4));`. The byte scale comes from PTRADD. Loads and stores dereference once; address expressions retain a raw pointer without borrowing a pointee. Index arithmetic wraps at pointer width, and literal-dependent shifts retain their source integer width. Base and index expressions appear once. Different output pointee types receive an explicit pointer cast.

The implementation requires matching 32- or 64-bit raw-pointer and integer-index widths, byte-addressed pointers and a constant scale. Constant bases, relative pointers, array-valued sources and implied PTRSUB/COPY bases retain their existing paths. Cast suppression retains its prior diagnostic output. Rust's [`wrapping_byte_offset`](https://doc.rust-lang.org/std/primitive.pointer.html#method.wrapping_byte_offset) is stable since 1.75, below the repository's 1.90 minimum.

The registered test compiles 57 complete emitted functions at O0/O2 and compares 29,544 function/input results against C. It covers signed and unsigned elements of 1/2/4/8 bytes, valid positive and negative indices, all destination and guard bytes, record fields, modular address bits, callback order and all 64 shift counts. Rust uses `overflow-checks=off`; C uses `-fwrapv`. No emitted body is rewritten. Five supplementary bodies add 1,660 comparisons for explicit pointer-result casts and byte loads/stores at an eight-byte stride. These supplementary bodies are archived but not registered cargo tests.

The first prototype passed the original 54 bodies, but additional literal-shift probes failed Rust width inference. The implementation now uses the existing integer-context helper before the `isize` cast. The failed prototype source, binary hash, captures and compiler diagnostics remain archived. A separate compile-only survey confirms float, boolean and pointer-valued raw-pointer accesses. Array fields and nested arrays still have a signed array-index problem: `array_field` stays unchanged, while `nested_array` fixes the outer raw pointer but retains the failing inner array index. Those diagnostics remain visible and belong to a separate change.

The static comparison covers 72 images, 37908 distinct functions and 138972 language/function records. All 15488 changed Rust records are classified by reconstructing their original indexing/address AST from the pointer method. Base, index, dereference and surrounding expressions are preserved; newly explicit pointer-result casts are recorded separately. The byte scale is recorded but cannot be independently inferred from the old indexing AST alone. C and Go records are unchanged. The initial comparison rejected variable provenance changes; all 680 changed variable records retain their names, types and storage, with addresses independently verified as the exact union of their source-line mappings. Existing captures were reused after this check. There are no new decompiler or parser failures. 70 inherited parser failures use review-only identifier normalization, with original bodies and diagnostics retained. The upstream `probestack` image was inspected statically and never executed.

The first audit left pointer-result casts and the inherited `unsafe` parameter name pending. The second audit records pointer-result casts and normalizes that invalid identifier solely for review. The third audit handles functions mixing retained and newly explicit casts. The fourth scopes retained casts to their original statements, distinguishing identical pointer expressions used in different contexts. Existing casts are preserved by complete AST comparison. Twenty-six controls check accepted transformations, preserved casts, changed bases/indices, missing dereferences, nonwrapping operations and duplicated calls. This structural audit does not establish native equivalence or type correctness for whole runtime images.

All four repository checks pass: 10363 workspace tests, all original 675 assertions and 3767 stage assertions. Eight stage assertions were added without losing prior ones. Original baseline and catalog metadata are unchanged. Production sources stayed fixed throughout final validation and timing.

The declared speed plan uses 64 balanced pairs per case and three warmups per arm. Every CPU and wall median delta is within the 5% budget. The largest median delta is 2.778%; the largest 95% upper mean bound is 5.361%. Bounds describe uncertainty and are not acceptance thresholds. All raw samples and CPU telemetry are retained.

| Case | CPU median | Wall median | CPU upper mean bound | Wall upper mean bound |
|---|---:|---:|---:|---:|
| c-pointer | +0.136% | +0.194% | +3.037% | +3.053% |
| go-pointer | +0.085% | +0.117% | +5.361% | +5.361% |
| rust-pointer | -0.076% | -0.069% | +0.650% | +0.653% |
| rust-callback | +0.453% | +0.468% | +0.181% | +0.260% |
| rust-index-context | +0.462% | +0.436% | +3.499% | +3.475% |
| rust-large | +0.825% | +2.778% | +1.295% | +1.205% |
| rust-count | +0.894% | +0.887% | +1.577% | +1.588% |
| rust-inflate | -0.012% | +1.349% | +0.275% | +4.296% |

`record.json` binds the candidate to its validated parent tree, which matches the actual parent merge. Both binaries and all 154 compiled processor specs were checked before and after timing. The complete changed bodies are losslessly archived and hash-checked. `manifest.json` hashes the evidence archive. Full array indexing, multiarchitecture native proof and broader Go/Rust ABI and enum recovery remain separate work.
