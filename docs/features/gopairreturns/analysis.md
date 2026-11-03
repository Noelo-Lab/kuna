With `godwarfsig on` and `gopairreturns on`, complete amd64 Go ABIInternal
DWARF contracts recover exactly two full-width signed or unsigned integer
results. The option remains off in every preset. Go declarations, calls and
returns use two-result syntax; C and Rust retain ordinary structs.

The final build includes current main's GOT tail-call feature and DWARF member-layout corrections. Its full
workspace passes 10356 tests with zero failures, and all 675 original
assertions remain unchanged. The stage baseline passes 3737
assertions; catalog and corpus counts are 376/683.

Native proofs bind this final binary: sixteen complete Go bodies on Go 1.22.5
and 1.27.3, the same sixteen C bodies at O0/O2, seven callback and projection
bodies, and explicit/inferred opaque-return forwarders. All recovered bodies
execute unchanged. These provide 1370 native input comparisons, including
exact callback counts, signed boundaries, zero/high-word constructors and
Go division panics. C division by zero is excluded; C integer widths and
wrapping arithmetic are explicit. Native Rust equivalence is not claimed.

Fresh whole-image comparisons cover 72 images, including three GOT
tail-call fixtures and two DWARF member-layout fixtures. They retain 37908 distinct functions and
88440 language records per arm. Default-off records are identical
to current main. All 439 enabled changes are reviewed using complete
before/after records, with unfamiliar changes held pending. The exact review
basis is preserved in implementation/breadth-review.json.gz. This is not a
whole-runtime native equivalence claim.

Remaining limitations include untyped bool/slice results and call arguments,
signed Go field assignments, and Rust aggregate carriers. Individual cases
remain listed in record.json. No complete Go or Rust parity is claimed.

The six required default-off speed comparisons pass their predeclared 5%
median criteria: largest median increase 0.967%. Each parent comparison
has 128 balanced paired rounds. Per-case 95% upper bounds on mean ratios are
reported as uncertainty, with a largest default-off bound of 5.705%;
these bounds are not acceptance gates. Three opt-in comparisons report costs
separately. A distinct 32-round
same-binary option flip supplies the standard speed fields. Sources, binaries,
fixtures and compiled specs are hash-bound. Every sample is retained.

Prior evidence is preserved under prior-a76/ and prior-a324/. The a76 build
passed its stronger simultaneous-bound study. The a324 integration passed
correctness and native checks; its timing was cancelled before analysis when
main gained DWARF member-layout changes. Its worker kept samples in memory,
so only its plan and progress log survived cancellation. No pass is claimed.
The final integration study was declared before its builds and samples, using
the repository median budget and explicit uncertainty reporting. These studies
are distinct. Earlier inconclusive measurements and failed probes are retained.
