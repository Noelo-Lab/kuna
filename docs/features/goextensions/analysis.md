# Go integer widening

The body for `a+int64(int8(b))*8+64` loses its widening conversion, leaving
`a + int8(b) * 8 + 64`. Go rejects the mixed integer widths; converting the
final product would also retain incorrect narrow arithmetic. The recovered
IR still has `SEXT18` before the 64-bit multiply. Go was using C's implicit
promotion shortcut. Keeping its widening explicit preserves the inner signed
interpretation and performs arithmetic at the recovered wider width.

Boolean widening has the same cause: `a - (a < b)` needs an explicit 0/1
value. The existing Go conversion form evaluates the condition once and
returns integer one or zero. The correction affects Go only, under the existing
`outlang` selection. Baseline: `34bfd388764c6291089183ef7d9c305c18c68826`.

Ten native functions cover signed 8-/16-/32-bit widening, unsigned control,
signed-to-unsigned conversion, overflow before widening, and boolean widening
at 32 and 64 bits. All 256 byte inputs plus seven wider boundaries are paired
with six wide values: 1,578 rows and 15,780 results per execution. Four fresh
images cover Go 1.22.5 and 1.27.3 with compressed and plain DWARF. Every emitted
body compiles unchanged under both compilers and matches native execution
byte for byte. Baseline bodies fail type checking. These native checks enable
`godwarfsig` explicitly; general caller/callee prototype agreement remains a
separate limitation.

The sixteen-image comparison covers 17,026 distinct
functions and 34,107 records. All 17,985
C/Rust records are identical. The 1,125 changed Go
bodies add 2,239 conversions across 1,706
hunks. An independent Go AST comparison permits only integer conversions and
the exact boolean 0/1 template; every other AST field and comment is unchanged.
Operators and expression structure retain their meaning when redundant
parentheses are ignored. All 16,122 Go bodies parse,
with no decompilation errors. Mutation checks reject altered arithmetic,
precedence, literal mappings, declarations, comments and existing casts.
Partial type checking reduces function records with diagnostics from 2,201 to
1,841 and messages from 9,193 to 8,167, with no newly diagnosed function records.
Five after-only message records expose an unsigned-value/signed-store mismatch
in one runtime function, which already has two unchanged stores rejected for
the same mismatch. Details are in `type-review.json`. Unresolved external
symbols prevent whole-program type checking.

Validation passes 10,045 workspace tests in 71 suites,
with no failures and 39 ignored. Original datatests remain
675/675; stages are 3,490/3,490, and the new regression improves from 0/2 to 2/2.
Tooling passes 114 tests with one optional skip. Lint, spec, catalog and counter
checks pass. The original baseline is unchanged.

Speed uses 31 alternating baseline/patched pairs after three warmups per arm,
on CPU 77 after this task's builds and comparisons finish. Across default C,
the native Go widening example, a Go function with 27 added conversions, and
Rust, maximum changes are +0.07% CPU and +0.07% wall,
within the 5% budget. Raw samples and paired changes are retained.
An initial CPU-79 Rust control was unexpectedly faster; its repeat showed a
25.88% wall increase with only a 0.43% CPU increase. A scheduling sample found
CPU 79 only 10.3% idle while CPU 77 was 100% idle. All four final comparisons
were repeated on CPU 77, after this task's builds and sweeps ended. Both earlier
CPU-79 samples are retained in `timing-cpu79`; their scheduling delay is not
attributed to the code change.

This fixes Go widening emission. Rust conversion typing, source multi-result
signatures, aggregate calling conventions and runtime idiom recovery still
require separate work.
