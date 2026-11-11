The option `gopairreturns on` recovers exactly two 64-bit integer results for
amd64 Go ABIInternal functions with complete, consistent DWARF contracts. It
requires `godwarfsig on` and remains off in every preset. C and Rust use the
ordinary struct representation; Go exposes two source-level results.

Sixteen complete recovered functions execute unchanged against authored Go
source on Go 1.22.5 and 1.27.3. The same sixteen C bodies pass at O0/O2 with
explicit-width types and wrapping arithmetic. The native checks cover 210 Go
and 420 C input comparisons; C division by zero is excluded. Seven complete
callback/projection/identity/mixed-sign bodies add 180 comparisons, including
a narrowed second result and an exact one-call counter. Complete opaque-return
forwarders add 560 comparisons across both languages, both fixture versions,
both Go compilers and both C optimization levels. Half exercise an unresolved
unlocked prototype while retaining the recovered 16-byte result.

The full workspace passes 10343 tests, with zero failures. The original
675 baseline assertions are unchanged. The stages contain 3733 assertions,
including eight new pair assertions. Catalog and corpus counts are 375/682.

The final whole-image comparison covers 67 images. Default-off
function records are identical to the independent parent. Enabled changes,
all corresponding full function records, and review limits are retained in
implementation/. Structural review is not a whole-runtime equivalence proof.
In particular, untyped callers still have incomplete bool/slice return and
argument inference; some signed Go field stores and Rust aggregate-carrier
expressions still need work. These are listed individually in record.json.
No claim of complete Go or Rust parity is made by this feature.

Six default-off timing comparisons pass the predeclared 5% criterion: largest
median change 0.743%, largest simultaneous upper bound 4.122%.
Three parent-to-opt-in comparisons report costs separately. A distinct 32-round
measurement flips the option on the same final binary and supplies the
timeit-compatible speed fields in record.json. Each parent comparison uses
1536 balanced paired rounds; binaries, sources and compiled
specs are hash-bound.

The proposal record and previous failures remain preserved. The previous
candidate's 384-round timing study did not establish the required wall-time
bounds. That full result is retained. The revision removes two heap allocations
per imported scalar result and bypasses disabled pair call dispatch. All
176670 complete enabled/disabled function records equal the prior candidate.
The revised sample size and unchanged acceptance criteria were declared before
the final measurement; no failed sample was removed.
The first complete
capture found invalid raw-to-pair conversions at unresolved runtime calls;
the final implementation handles those through a single-evaluation adapter.
