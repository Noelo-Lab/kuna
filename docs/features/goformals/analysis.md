# Go result formals stay out of C prototypes

Uncompressed Go DWARF describes results as formal-parameter children with
`DW_AT_variable_parameter`. The generic C/C++ signature importers treated those
children as inputs and read the absent function-level return type as void.
A native `pair(x int64) (int64, int64)` consequently acquired three inputs and
an empty body. Removing that invalid contract allows inference to retain the
input, addition and both returned words. High-level Go tuple emission remains
separate work: the inferred two-word carrier still uses pseudo-fields.

The comparison baseline is
`e3feca89a14fe379e06fa8b062685c996c4de7c0`, including the Go string-pointer fix.
`repro.go` builds the example with `-ldflags=-compressdwarf=false` and does not
depend on a prebuilt fixture or a fixed function address.

The correction checks each compilation unit's `DW_AT_language` before either
generic prototype importer runs. Go units retain their names, globals and
other metadata, while supported source contracts come only from `godwarfsig`.
The check does not depend on the producer string, target architecture or the
setting of `cppproto`. Invalid or unsupported Go contracts fall back to
inference with either debug-section encoding. This corrects an invalid import
under the existing options and does not introduce another option.

The hermetic DWARF 5 fixture now includes its uncompressed twin. The integration
check covers the identity and seven rejected contracts with both `godwarfsig`
and `cppproto` on and off. Four of the fifteen stage assertions fail before the
correction. Native execution also checks fourteen untouched emitted bodies
from each of Go 1.22.5 and 1.27.3's uncompressed images, under both toolchains.

Existing fallback inference remains imperfect for assembly runtime helpers. In
whole-image output, removing invalid callers can expose an inferred return for
`runtime.rtsigprocmask` or `runtime.memclrNoHeapPointers`, or remove an inferred
return from `runtime.nanotime1` while retaining its stack-result store. These
signatures are not claimed as recovered source contracts. The isolated runs
and native execution witnesses bound what this correction verifies.

The sixteen-image comparison covers 17,026 distinct
functions and 68,214 C/Rust/Go records across both settings of
`godwarfsig`. Changes are confined to five uncompressed Go images: the authored
cross-unit fixture, the entry-range fixture, and native Go 1.22.5, native Go
1.27.3 and the pinned angr Go 1.27.1 basics image. Compressed Go, stripped Go,
C and Rust controls remain identical. There are 17,320 changed records
across 19,384 textual hunks, with no decompilation errors. All
32,244 emitted Go bodies parse.

The independent DWARF audit verifies every removed installed contract belongs
to a Go compilation unit. With `godwarfsig` off, 4,705
generic contracts disappear; with it on, 3,861 disappear
and 854 supported contracts remain exactly unchanged.
No contract is added or altered by the exclusion itself.

The hunk classification distinguishes directly removed contracts, their callers,
anonymous type numbering, metadata and inferred callee changes. Symbol call
paths explain propagation; they do not prove arbitrary recovered bodies correct.
For the 34 callee-inference surfaces, separate single-function runs
produce identical bodies before and after except for anonymous type numbering.
Their whole-image differences therefore arise during shared inference. The
paths and raw diffs are retained in `inferred-carrier-review.*`; arbitrary
runtime bodies are outside the standalone compilation claim.

Validation: 10,007 workspace tests passed, none failed, with 39 ignored
across 71 suites. Original datatests remain 675/675; stages are
3,471/3,471. Tooling tests, lint, spec links, the catalog and shared
counters pass. The workspace suite used Go 1.27.3; the native witnesses execute
under both supported toolchains. The original baseline is unchanged.

Speed uses 31 alternating pairs after three warmups per arm on CPU 79, after
this task's builds and sweeps completed. Across six controls, the maximum
median change is +1.51% CPU and
+1.43% wall. Raw samples, ratios of medians
and median paired changes are retained; the final run remains within the 5% budget.
The initial run is also retained in `timing-initial/`: an external sweep caused
substantial scheduling delay in two Go rows. Its plain-Go wall ratio was +12.94%
while the median paired wall change was +0.13% and CPU change +1.74%. All six
rows were repeated to distinguish scheduling delay from execution cost. The
repeat passed for the Go rows; its Rust row then encountered 119–122 ms of
median wall-minus-CPU delay, with a +7.48% ratio of wall medians but +0.85%
paired wall and +0.68% CPU. That run is retained in `timing-repeat/`, and the
Rust control was repeated separately.
