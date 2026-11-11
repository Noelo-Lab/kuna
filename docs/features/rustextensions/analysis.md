# Rust integer extensions

Rust output for `a + (b as i8 as i64) * 8 + 64` spells its byte interpretation
as `u8`, losing the sign before widening. With `a = 1, b = 255`, native code
returns 57 and the emitted expression returns 2105. C's hidden-extension
shortcut also omits widening needed after narrow arithmetic.

Rust now keeps explicit extensions and reinterprets signed core text storage
through its signed width before sign extension. Text declarations and pointer
pointees retain their storage types. This corrects existing Rust output; no new
option is added. C and Go retain their previous behavior. Baseline: `c28134ce02f52cc26a70e0ba71be210f3dd60d8d`.

Six native functions cover signed 8-/16-/32-bit extension, narrow addition and
multiplication, and unsigned control. Each runs all 256 byte values and seven
wider boundaries against six wide values: 1,578 rows and 9,468 results. Three
fresh images from Rust 1.90, 1.93.1 and 1.99.1 each produce unedited bodies that
compile and execute under all three compilers, matching native execution exactly.
Arithmetic overflow checks are explicitly disabled for this comparison; the
original source uses wrapping operations. The old signed-byte body compiles but
gets 798 rows wrong per producer. The old combined bodies also fail type checking.

The eighteen-image comparison covers 25,339 distinct
functions and 50,733 records. All 41,461
C/Go records are identical. 481 Rust bodies add
2,003 integer casts across 1,192 hunks. Independent
Rust AST comparison preserves operators, declaration types, literals, comments,
and every existing cast at the same expression node. Only additional integer
casts and redundant parentheses are permitted. Mutation checks reject changes
to arithmetic, precedence, existing casts, declarations and comments.

Some original runtime bodies have invalid `in(...)` calls or `_::` namespace
prefixes. The verifier makes identical, counted parser-only substitutions on
both sides; emitted code is untouched. Those existing syntax gaps remain open.
The comparison records each occurrence. New casts also expose 71
previously hidden source-address entries. Every new address
matches an original machine instruction, no address is lost, and variable and
function metadata outside source positions is unchanged. Partial standalone
Rust type checking and its limitations are recorded in `type-review.json`.

All 10,061 workspace tests pass, with 39 ignored across 71 suites.
Original assertions remain 675/675; stages pass 3,492/3,492. The new regression
improves from 0/2 to 2/2. Tooling passes 114 tests with one optional skip; lint,
spec, catalog and counter checks pass. The original baseline is unchanged.

Timing uses 31 alternating pairs after three warmups per arm, after this task's
builds and scans finish. Cases cover default C, native Rust byte extension, a
Rust function with 34 added conversions, and Go output. Maximum measured changes
are -0.33% CPU and -0.23% wall, within 5%.
Raw paired samples are retained.
