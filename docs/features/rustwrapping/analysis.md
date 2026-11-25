# Rust wrapping integer operations

The same implementation has been replayed onto
`b5d9f8563c97d35c51e6c2c2d1d586b2793f3567`. The four wrapping cases now run in
the existing Rust integer conversion stage. One multiline assertion checks the
four fixed-width operations and the mixed-width conversion. This preserves all
4,591 parent stage assertion keys across 796 XML files. The optimized `a447beee8`
R06 build passes the five affected stage files, 47/47. The exact `b5d9f8563`
build and full gates remain pending.

A lifted 64-bit addition returns zero for `u64::MAX + 1`. The parent emits
`return a + b;`, which panics with Rust overflow checks enabled. It also emits
unary `-value` for unsigned negation, which Rust rejects. The reference is
fixed-width machine arithmetic modulo the operation width.

The owning decision is [P9 Rust operator emission](../../spec/09-emission.md#rust-wrapping-arithmetic).
For primitive integer add, subtract, multiply, and two's-complement negate,
the wrapping method takes the p-code width and the printed result signedness.
Each operand is evaluated once in p-code order. In-place forms expand to an
assignment whose right-hand side wraps. Recovered overflow branches and panic
calls remain separate. This is a correctness fix with no feature flag.

The [stage witness](../../../tests/stages/ghangr-rust-integer-conversions.xml)
requires each of the four original return expressions. The pre-fix release
fails the combined assertion, and the optimized R06 release passes it. The
authored native checks cover 40 complete emitted functions at 8, 16, 32, and 64
bits, with 2,088 arithmetic cases at both O0 and O2 with overflow checks
enabled. A separate loop harness checks 125 results and callback counts at
each optimization level. A synthetic IR
test pins unsigned 128-bit addition with a signed operand. That expression
passes 36 native boundary cases at each optimization level. In a separate
x86-64 `__int128` binary probe, Kuna still recovers 64-bit input limbs and a
split array result. The prototype override rejects `int16` and `uint16`, so the
128-bit check establishes emission from primitive IR.

This change has been replayed onto `3a7385b58ae1562137d3d71bf0f77b9aa667f891`.
The release build and static gates pass: 675/675 original assertions and
4,502/4,502 stage assertions, retaining all 4,498 parent assertions and adding
exactly four. Catalog and spec checks pass. All 426 C/Go comparison records
match that exact parent. The 40 arithmetic bodies match the previously executed
native harness byte for byte; the callback-loop body also matches, and its full
output matches the previous release including the lint attribute omitted from
the authored harness. No original fixture binary was executed. The workspace
runtime suite remains pending.

Current-parent timing used ten alternating ABBA/BAAB rounds per workload, or
20 samples per build, on CPU 17. Rust paired median wall changes were +1.14%
for the authored fixture and +1.21% for the bounded 128-function fixture sample.
The largest paired median wall or CPU change across all four workloads was
+2.09%, within the 5% budget. This is a coarse shared-host measurement: SMT
sibling 57 had median activity of 0.48% but peaked at 76.53%, and individual
C/Go wall pairs ranged from −57.33% to +289.89%. All rounds were retained and
the measurement was not repeated. It does not establish whole-corpus speed.
The figures below describe the earlier validated snapshot.

The exact comparison starts at `a42c0364798cec80c4363fee0519f0f65b86d5e1`.
80 authored C and Go function outputs and 346 C and Go fixture records are
byte-identical. Upstream datatests pass 675/675, and the stage corpus passes
4,427/4,427 with four assertions added. Spec, catalog and tool checks pass. The
full workspace suite is pending.

Both release builds used the same compiled specs for timing. After warmup,
ten alternating ABBA and BAAB rounds on one CPU compared the mean of two runs
per build in each round. The median paired wall change is +2.97% for the 40
authored arithmetic functions in Rust mode, within the 5% budget. A
128-function sample from the Rust formatting fixture changed 45 bodies,
produced zero errors on each arm, and measured −1.95% median wall time. The
[record](record.json) retains the paired deltas and build hashes.
