# Verified Rust panic formatting

`rustfmtpanic` recovers `panic!` from verified `core::panicking::panic_fmt`
calls. Literal messages preserve braces, Unicode and every trailing newline;
integer messages reuse the existing descriptor, type and static-format proofs.
The option is on by default and disabling it restores every compared Rust
function record exactly. C and Go output are unchanged.

The implementation requires detected Rust, the stable 1.93.0/1.93.1 layout,
little-endian 64-bit storage, exact sink identity, three runtime arguments,
no result and existing no-return evidence. Both fully qualified and exact scoped
identities are accepted. Caller-location addresses must be constant, nonnull,
mapped and nonvolatile. Their contents are not interpreted: the recovered macro
uses the recovered source's location. Original file/line, backtrace and panic-hook
location equivalence are outside this feature's contract.

## Execution and refusal evidence

The native Rust 1.93.1 oracle supplies 263 values to each of six integer panic
functions plus two literal cases: 1,580 captured payloads. Unchanged emitted
literal bodies and unchanged integer macro expressions reproduce all payloads
under Rust 1.90.0, 1.93.1 and 1.99.1 (4,740 comparisons). Explicit typed input
bindings isolate the primitive macros; existing descriptor declarations include
unresolved callbacks and incompatible pointer assignments, so this is not a claim
that every complete primitive function recompiles. Signed boundaries, every byte
pattern, repeated indices, width, alternate flags, control-character fills,
Unicode and terminal newlines are exercised.

Five integration tests cover the native witnesses, exact namespaces, option and
source/output-language boundaries, missing/unsupported versions, absent formatter
identity, no-return facts, argument counts, volatile/unmapped/null/dynamic caller
locations, mutable or invalid text and unsupported generic/floating/dynamic
formats. A dynamic-width function's separate constant out-of-range panic is
recovered; its dynamic formatting call remains explicit. Actual 1.90.0 and 1.99.1
producer controls remain byte-identical. Version 1.93.0 is covered by the primary
layout references and a metadata-version test, not a separate native producer.

## Complete output comparison

Twenty images cover 25,955 distinct functions and
52,581 language-specific records. All 42,693
C/Go records are identical, and 16,738 Go records parse.
All 9,888 Rust records with the option off
equal the parent, including metadata. Only 29 Rust functions change, with one
runtime-to-macro replacement in each of 29 hunks. Parsed ASTs and comments verify
every surrounding statement, declaration, existing macro, return and expression.
All 19 literal messages are checked against exact image bytes; symbolic template
spellings are bound to unique IR calls before reading those bytes. The ten
primitive sites retain typed local reads and the existing template decoder.

All 29 new macro formats compile and execute under all three Rust compilers with
explicit test values. Literal payloads additionally match the decoded image
messages. Full changed bodies receive a partial compiler audit: 335 selected
type/arithmetic diagnostics remain in ten functions, no new diagnostic multiset
appears, and standalone compilation improves from zero to fourteen functions.
Unresolved whole-function dependencies and existing type gaps remain.

Function and variable metadata are unchanged apart from source positions and
argument-use mappings. Ten source instruction addresses disappear from the
replaced call lines; disassembly confirms each is a stack descriptor-array `lea`
into RSI. The corresponding argument is now represented by typed value reads.
Variable-use address changes are confined to those reads at the replaced call.
No new global instruction mapping appears; every setup statement and all IR remain.
The raw diffs and per-function records retain the full review evidence.

## Checks and speed

Original corpus: 675/675, unchanged baseline. Stage corpus: 3,508/3,508; new case
0/6 on the parent and 6/6 with recovery. The parent capture omits new option
commands that its catalog cannot recognize. Workspace: 10,084 passed,
zero failed, 39 ignored across 71 suites. Tooling:
114 passed and one optional skip. Lint, spec, catalog, generated documentation
and shared counters pass. The new live-option count is 170 (catalog total 364).

Five timing cases compare the parent, the new binary with recovery off, and the
default. Three cases use 192 balanced randomized rounds per arm; integer panic
and Go controls use 768, including all 192 initial samples. All six arm
permutations occur in each six-round block. Every run has three warmups per arm and fixed CPU affinity. Cases run in independent processes on disjoint physical CPU pairs; all arms of one case use that run's assigned CPU. The execution change was declared before fresh sampling.
Maximum parent-to-default median CPU delta is +0.555%
and wall delta is +0.623%; maximum off-to-default median CPU delta is
+0.650%. Every median comparison fits the five-percent
budget. Every case additionally requires each upper percentile-bootstrap bound
on the ratio of mean CPU/wall times to fit the same budget. Twenty thousand
resamples preserve complete six-round blocks; the upper quantile is 0.99875.
The initial look is re-evaluated at family alpha 0.025 across twenty comparisons;
the one fixed confirmation look receives another 0.025. Its 576 additional
rounds per unresolved case retain every initial sample. The method was declared before
each relevant sample set was collected. Raw samples, orders and binary hashes are retained.

This implements one panic sink from angr's Rust macro-selection coverage. Other
panic entry points, older formatting layouts, generic arguments and full Go/Rust
parity remain separate work.

The final capture includes main commit `f8746b9730b525a065c89c26e358ffd7c2328bc7` (shared effect annotations, lazy unknown types and JSON field omission). Both baseline and patched binaries were rebuilt, all repository checks and measurements were repeated, and every patched function record equals the previously reviewed panic output.

Initial control measurements were affected by external CPU contention. The first large Rust control exceeded the median CPU budget, with coefficients of variation around 33%; a subsequent monitored run passed the median budget but failed its contention limits. Both are retained. Repeated idle checks found no sustained quiet interval, so a new randomized comparison and confidence-bound acceptance rule were declared before collecting fresh samples. No earlier samples contribute to those bounds. After the later main integration, the same randomized acceptance rule was declared for all five cases before fresh sampling; every case in the current record uses the rebuilt f874 baseline and patched binary. `timing-monitor.json`, `timing-randomized-plan.json` and the attempt directories retain the decisions and observations.

The first f874 randomized run passed every median comparison but left three wall-time upper bounds above five percent. Those observations remain in the combined record. A single fixed-size confirmation adds 576 rounds to the two affected cases, using stricter two-look family allocation; it does not discard or replace any initial samples. Other feature measurements share disjoint physical CPU pairs under a plan declared before their first samples.

The shared coordinator exits nonzero because the other two features have unresolved first-look bounds. All five parent cases pass their final criteria; the parent-specific monitor is the acceptance record. `batch-result.json` distinguishes these outcomes.
