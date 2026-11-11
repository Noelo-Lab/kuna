# Go byte-array result storage

A recovered two-word Go return uses a `[16]byte` local. Its artificial members
`v1._0_8_`, `v1._8_8_` and `v1._0_16_` are not Go fields. The correction emits
bounded integer views for each word and returns the entire array directly.
It preserves the existing register result and memory offsets; it does not
claim source tuple recovery.

The comparison baseline is `72757bccf5e212f0a3238bd22c47cd2800665a47`.

The module `p9_emit/kuna_gobyteview.rs` selects only exact-width bounded views
of byte-array storage, under the existing Go output selection. Refusal cases
retain their previous representation. Native and hermetic emitted bodies are
executed without edits, checking both words for positive, negative and extreme
inputs under Go 1.22.5 and Go 1.27.3. C and Rust output must remain unchanged.

The sixteen-image comparison covers 17,026 distinct
functions and 34,107 C/Rust/Go records. All
17,985 C/Rust records are identical. The
2,841 changed Go bodies contain 13,399
new typed views and 9,241 whole-array covers across
4,793 textual hunks. The token verifier reverses each typed view to
its exact prior path, offset and width, and permits only whole-array cover
removal besides those views. Every other token, literal and comment is identical.
Direct local declarations independently bound 5,882
views and 8,501 whole covers; global and nested
bounds are checked by the production planner. All 16,122
Go bodies parse and no decompilation errors occur. Raw diffs and classifications
are retained in this directory.

An initial implementation selected facing IR signedness, which could disagree
with the Go assignment conversion. A manual scan found 315 such assignments.
The corrected planner requires both types to qualify and selects the emitted
Go value type. The signed nested-array regression compiles and executes without
editing its body. The final scan checks 1,769
explicit-conversion assignments and finds no type mismatches. Parsing alone
would not have caught the original error.

Native witnesses cover Go 1.22.5 and 1.27.3, each with compressed and plain DWARF.
Every unchanged emitted body compiles and executes under both compilers for six
signed boundary and wraparound inputs, checking both result words. The nested
array witness runs 25 input pairs with unsigned parameters and another 25 with
signed parameters, checking reads, writes and ten neighboring bytes.

Validation passes 10,024 workspace tests in 71 suites, with no failures and
39 ignored. Original datatests remain 675/675; stages are 3,483/3,483. Tooling
passes 114 tests with one optional skip. Lint, spec, catalog and counter checks
pass. The original baseline is unchanged.

Speed uses 31 alternating baseline/patched pairs after three warmups per arm,
on CPU 79 after this task's builds and sweeps complete. Across default C,
the native Go pair, the Go function with the most whole-array covers, and Rust, the
maximum change is -0.33% CPU and -0.13% wall.
Raw samples and paired changes are retained; both remain within the 5% budget.

This repairs the recovered byte-array representation. It does not recover
source tuples, remove opaque SUB168 extraction helpers in callers, or change
Go aggregate calling conventions. Those remain separate parity gaps.
