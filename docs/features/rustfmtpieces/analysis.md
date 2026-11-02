# Verified piece-array Rust printing

`rustfmtpieces` recovers `print!`, `println!`, `eprint!` and `eprintln!` from
the older `fmt::Arguments` piece/argument arrays. It is enabled by default for
the verified x86-64 ELF layouts from Rust 1.89.0, 1.90.0 and the exact nightly
`bf64d66bd` dated 2025-05-21. Other compiler identities remain unsupported.

Recovery requires an exact print sink, a complete five-word live stack
descriptor, a null optional-format pointer, bounded immutable UTF-8 pieces,
and exact primitive Display identities with typed stack reads. Optional-format
length is inactive and uninitialized when its pointer is null, so it is never
read. The renderer can prove immutable table lengths from the ELF RELRO extent,
with its upper boundary rounded down to a complete page; volatile annotations
always refuse recovery. This fact does not change global memory properties.
All IR, descriptor setup, types, function prototypes and return statements remain.

## Native and refusal evidence

Three native producers each exercise 265 values through five primitive functions
plus four literal cases. The unchanged emitted literal bodies and unchanged
primitive macro expressions match both native output streams under three
execution compilers (1.90.0, 1.93.1 and 1.99.1): 11,961 call comparisons.
Primitive expressions use explicit typed input bindings. Existing setup can
contain unresolved callbacks or incompatible pointer assignments, so these
results do not establish whole-function recompilation for primitive cases.

Six integration tests cover all three producers, exact sinks, option and
source/output-language boundaries, incomplete descriptors, argument counts,
missing formatter facts, metadata/version boundaries, ELF kind/architecture,
RELRO partial pages and absent/volatile/mutable storage. Repeated indices,
explicit specifications, floats, generic callbacks, forwarded and dynamic
descriptors remain explicit calls. Nine comparisons from unsupported producers
remain unchanged, including a different nightly with the same version number.

## Complete output comparison

The 24-image comparison covers 27,691 distinct
functions and 57,789 language-specific records.
All 46,165 C/Go records are identical; all
18,474 Go records parse. Disabling the option restores all
11,624 Rust function records exactly.
Only 29 Rust functions change: nine in each native producer and two hermetic
shapes, with one call replacement in each of 29 hunks. An independent AST and
comment comparison verifies all surrounding code. Independent ELF segment and
relocation reads verify every piece byte, and full demangled symbol identities
plus call-linked IR operands verify all fifteen primitive call sites.
Fourteen literal calls and fifteen primitive calls compile and execute under
all three execution compilers. A partial whole-body compiler audit reduces
selected diagnostics from eight to six and standalone compilation improves
from zero to fourteen functions, with no new diagnostic multiset.

Source mappings lose 27 native stack-address instructions into RDI and two
synthetic prologue adjustments folded into the descriptor argument. Disassembly
and original variable offsets verify those two exact synthetic cases. All losses
belong to replaced call lines, no global instruction addresses are added, and
variable-use changes are confined to the recovered arguments. Complete raw
diffs and per-function proofs are retained alongside this record.

The original angr fmt fixture is included in this comparison. Its functions
remain unchanged: it is stripped and Kuna has no recovered print sink identities
there. This layout feature alone does not satisfy that integration test; symbol
recovery, older allocating-format consumers and aggregate types remain gaps.

## Checks and speed

The original corpus remains 675/675 without changing its baseline. The stage
corpus passes 3,523/3,523, with the new case changing from 0/7 to 7/7. The parent
capture omits option commands unknown to its catalog. The workspace passes
10,096 tests, with zero failures and 39 ignored across 71 suites. Tooling passes
114 tests with one optional skip; lint, spec, catalog and shared counters pass.

Seven cases compare the parent, recovery disabled, and default behavior using
768 balanced randomized rounds after three warmups per arm, with fixed physical
CPU assignments. They cover literal and four-argument older printing, C output
of the older Rust image, the original angr Rust function, newer Rust formatting,
scalar C and Go.
Maximum parent-to-default CPU delta is +1.065%
and wall delta is +1.254%.
Maximum off-to-default CPU delta is +0.468%
and wall delta is +0.692%.
Every comparison fits the five-percent budget; all raw samples are retained.
Every upper block-bootstrap confidence bound on the mean CPU/wall ratio also
fits the same budget. The 20,000 bootstrap resamples preserve complete six-round
blocks. One final look allocates family alpha 0.05 across 28 comparisons;
the upper quantile is 1 - 0.05/28. The fixed protocol was declared before any
samples of this implementation. Case workers use disjoint physical CPU pairs,
and every arm of each case stays on the same CPU. No owned builds, tests or
corpus captures run during sampling. All observations and telemetry are retained.

The earlier implementation completed a 192-round study and a fixed confirmation
without clearing the budget: the literal case's final parent wall-time upper
bound was 5.027937%. Those results remain under their original attempt IDs.
The loader was then changed to index only candidate formatter addresses while
still invalidating all conflicting or unknown aliases. A new fixed study tests
that changed implementation; observations from different binaries are not mixed.

The final validation includes main's UTF-8 console paths, shared effect annotations, lazy unknown types, JSON field omission and wrapped-memory splitting at main commit `69fad3ce4cbdbb3232680938252bd75c2bec5a68`. Fresh captures
equal every previously reviewed patched record. Full Go/Rust parity is not
established by this feature.
