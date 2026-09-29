# Repository cleanup

One cumulative PR on `refactor/deslop`, starting at `632437155`.
Preserve CLI output, option semantics, decompiler defaults, and regression
expectations. Commit verified milestones; do not re-pin baselines to hide failures.

## Current changes

- Option dispatch and its 245-name frontend allowlist now come from the same
  declaration in `p0_knowledge/kuna_option_dispatch.rs`. The original name order
  and all handler bodies are preserved. Catalog metadata remains independent;
  a test checks names and uniqueness against the implemented handlers.
- CLI subprocess and in-process loading share environment bindings and value
  conversions in `loadtime.rs`. Tests cover legacy token behavior, final-option
  precedence, explicit removal, and restoration after unwinding.
- Removed obsolete porting instructions and historical count-change logs from
  the touched option code. Count assertions remain intact.
- CLI integration tests exercise the executable instead of compiling private
  module copies. They parse output with an independent JSON implementation.
  Removed missing-command and missing-spec success paths throughout the CLI
  suite, including wrappers that returned `Option` only to support those skips.
  Missing processor specs now fail tests; optional C-compiler checks are retained.
- Decode fallback variants, enumeration, count, and diagnostic spellings come
  from one declaration. A compatibility test pins the public order and strings.
- Removed obsolete engine shims and unreachable error branches without changing
  opcode mutation order. Test-only helpers live with tests. Ignored setup errors
  now fail tests; two fixtures were corrected to use valid function inputs.
- Constant-sequence assembly checks range addition before indexing. Its existing
  overflow-panic test now verifies that valid writes survive an invalid range.
- The phase registry uses a standard TOML parser and typed build-time schema.
  Missing, duplicate, unknown, or wrongly typed fields fail the build; live flag
  mappings must be complete. No catalog row, default, or option name changed.
- CLI JSON parsing uses the standard parser while retaining number tokens,
  duplicate-key order, and all existing rendering. Baseline records require a
  string-valued passing set; malformed records cannot report parity.
- Pipeline inventory reads fail without writing when data is corrupt or
  unreadable. Legacy missing sections and extra metadata remain supported.
  Isolated Python unit tests run through `make test-tools` and CI.
- Test-profile optimization is enabled with debug assertions and
  overflow checks still enabled; ordinary development builds are unchanged.
- CLI flag-value and option-pair consumption is shared. Parse errors reach the
  dispatcher as `Result`; missing values stop before command execution instead
  of printing an error and continuing. Repeated options retain their argv order.
- Floating-point constant evaluation handles NaN input signs explicitly instead
  of depending on the optimizer's choice of host arithmetic instruction. The
  first NaN operand supplies the sign; invalid non-NaN arithmetic retains the
  negative quiet encoding pinned by the existing vectors.
- WASM tests use a standard JSON decoder with exact 64-bit integer checks and
  no longer treat missing specs or failed architecture initialization as a pass.
- Pipeline/repipe writers share unique sibling-file publication and failure
  cleanup. JSON layout, encoding, explicit modes, and existing locks are retained.
- Worker record encoding and decoding are isolated from process scheduling.
  Literal-byte tests pin both existing wire versions; framing, flush order and
  truncated-tail recovery are unchanged. The repeated pool design narrative
  is replaced by a short ownership summary pointing to the existing spec.
- SLEIGH compiler modules no longer suppress dead-code warnings. Removed three
  unused private items and corrected stale porting documentation. Its fixture
  checks fail when vendored sources are missing; independent compiler evidence
  is recorded in `notes/deslop-slacomp.md`.
- Removed unused private IR, emulator, split-value and loop helpers found by a
  forced-warning build. String/fill modules no longer suppress dead-code
  warnings, and fill-model fixture helpers live with tests. Input registration
  uses the canonical prototype-effect constants without an always-true switch.
- Console, SLEIGH and Ghidra regression tests fail when required fixtures cannot
  be loaded. Setup helpers return their actual values without skip-only `Option`
  wrappers. An 8051 context test that normally skipped now exercises `TMode` on
  the existing ARM fixture. CI no longer needs log-grep canaries to detect
  passing tests that did no work; processor builds and completeness checks stay.
- Compiler parity compares the same 44 specs with pinned C++ output hashes,
  independent of locally built `.sla` files. Per-run temporary output replaces
  shared scratch filenames. Compiler CLI filename normalization is shared and
  accepts extensionless files beneath dotted directories; excess filenames fail.
- Compiler XML output honors `-y`. The symbol/constructor boundary preserves
  the opcode encoder's format-specific behavior instead of forcing binary
  integer attributes through XML. Default binary output is unchanged.
- SLEIGH symbol decoding also preserves its format-specific opcode reader.
  One canonical ID table replaces duplicate definitions and supplies complete
  XML registration; existing import paths remain available. XML registration is
  lazy, and the packed binary path keeps its numeric IDs.
- CLI call-graph queries, callee-first scheduling and object-file views live in
  their own modules. Command drivers share these boundaries without importing
  unrelated helpers from `decompile_all`. Inventory storage stays private to
  the graph; its caller-completeness query retains the conservative read-failure
  policy. The command module also loses its obsolete introductory narrative.
- Remaining CLI test JSON scanners use the standard parser and explicit field
  paths. Function records and project index fields retain their raw tokens for
  byte-equality checks; semantic name checks decode escapes. Numeric fields must
  belong to the requested object, not a nested record with the same key.
- SLEIGH encoding borrows constructor templates without cloning their arena.
  The mutable template callback now serves decoding only. Sparse section IDs,
  section ordering and invalid-handle errors have direct regression checks.
- Twelve boolean loader gates share their environment-token policy. The existing
  variable names, public wrappers, defaults, whitespace/case handling and unknown
  token behavior are unchanged; other loader vocabularies remain separate.
  Corrected nearby obsolete option-dispatch and stack-guard comments without
  changing the standalone option object's disabled initial state.
- SLEIGH `with` blocks retain their context assignments and pass inherited
  changes directly to each constructor, eliminating a temporary arena round trip.
  A new pinned C++ oracle covers nesting, siblings, local overrides, `globalset`
  and leaving a block. None of the existing 44 oracle digests changed.
- Declaration naming keeps reserved and assigned spellings in one lookup table
  instead of copying them among two sets and a suffix-counter map. Assignment
  follows caller order; hash iteration does not choose names. Existing names,
  suffix selection and the public API are preserved.
- Compiler diagnostics and section validation read the constructor's existing
  source metadata instead of maintaining a separate location map. Both public
  constructor builders now share the complete finalizer; the parser entry point
  takes ownership of its section vector before delegating.
- Ranked backlog parsing distinguishes missing files from corrupt or unusable
  data. Selection validates the fields it consumes while retaining optional
  defaults, extra metadata, ranking, filters and shell/JSON rendering.
- Pipeline dispatch distinguishes empty work, retryable claim/disk pauses and
  fatal selector/state errors. Errors retain their diagnostics and stop dispatch;
  already-running workers finish before the driver returns failure.
- Finalized compiler macro definitions are shared immutably by symbols and the
  expansion table. Each invocation still owns its rewritten operations; the
  existing owned setter and borrowed getter retain their signatures.
- The reserved branch-normalization action no longer walks the CFG and builds
  flip lists that it discards. Registration, group filtering and the initialized
  graph check are retained; no branch-rewriting feature is enabled. The module
  introduction no longer claims its implemented supporting APIs are absent.
- Declaration rendering borrows names while counting duplicates and passes them
  directly to the allocator. The counting borrow ends before names can change;
  the intermediate vector of cloned strings is gone. Rendered-signature dedup
  uses its existing default constructor and a reviewed membership-only hash set.
- Compiler consistency errors now propagate through optimization and compilation.
  A single read preceding or occurring in its sole write is rejected before
  encoding, rather than silently skipped; the obsolete checker trampoline is gone.
- Compiler consistency inspection borrows template fields and traverses existing
  ordered-map values. Five overlapping read helpers become one, temporary key
  vectors are gone, and rule selection copies only the chosen record.
- CLI fault-injection tests share pipe draining, timeout handling and child
  reaping. Each caller retains its environment setup, polling interval and cap.
  Read and launch failures cannot silently produce partial successful results.
- Optional CLI test tools skip only on a spawn `NotFound`; other launch errors
  and unsuccessful exits fail explicitly. The limits test no longer hides an
  architecture-loading failure, and the dead-writer check loses its always-true
  skip wrapper. The CLI manifest no longer describes obsolete subprocess-only
  behavior or promises unverified byte equality.
- Dashboard artifact and asset lookups match request identifiers against directory
  entries, extending the boundary already used by agent logs. Identifier checks
  match the whole string. Asset containment, recorded-report precedence and the
  documented need-record symlink policy remain in place. The module introduction
  no longer describes finished modules as under construction or promises every
  route returns 200.
- The compiler's existing local-collision check now reports shared temporary
  exports. The existing `-c` flag adds operand names and source locations;
  summary warnings remain nonfatal and do not change compiled images. A private
  detector borrows the templates and preserves constructor/operand order.

## Evidence

- Untouched baseline: 675/675 upstream assertions. The stage suite initially
  failed six Cortus APS3 assertions because its compiled spec predated the
  source. Rebuilding that one artifact made the baseline pass 1467/1467.
- Refactor: upstream and stage parity pass; catalog JSON is byte-identical;
  `catalog --check` and `make check-spec` pass; 268/268 CLI probes pass.
  Text/JSON decompilation and JSON function listings match the baseline's stdout
  and stderr with repeated and mixed loader options. The full workspace suite
  and generated-options freshness test pass.
- Eight interleaved warm runs of `decompile fauxware main` with loader overrides:
  median 133 ms before, 120 ms after. Other tests were running concurrently, so
  this is a check for a noticeable slowdown, not evidence of a speedup.
- Baseline executables retained outside the repository at
  `/tmp/kuna-deslop-baseline.5KlEYM` for comparison.
- CLI test cleanup: all four gates and 268/268 CLI probes pass again. Running
  the modulo CLI regression with an empty specs directory fails as intended;
  missing specs can no longer turn that test green.
- Engine/registry cleanup: 3060 core unit tests, eight arithmetic-rule integration
  tests, five code-generation tests, and 15 parallel-decode unit tests pass in
  release builds. The parser replacement initially generated byte-identical
  Rust; later edits only corrected its generated documentation. Upstream parity
  (675/675), stage parity (1467/1467), 268/268 CLI probes, catalog consistency,
  and spec checks pass. The full workspace run failed only the three embedded
  manual checks: the manual was edited after compilation. A later frozen,
  combined snapshot passed the complete workspace gate.
- Catalog JSON, text/JSON decompilation, JSON function listings, and whole-binary
  JSON remain byte-identical to the original baseline, including stderr.
  An initial eight-run timing sample overlapped compilation and was unstable.
  Twenty further interleaved runs measured 135 ms versus 140 ms median CPU time
  at 10 ms resolution; median wall time was 150 ms for both. Other workspace
  tests were running throughout, so no speedup claim is made.
- JSON/baseline changes: 215 CLI unit tests, 14 isolated parser/baseline tests,
  and the standalone docs example check pass. Catalog and both baseline files
  round-trip identically through all three renderers. The new CLI baseline
  regression fails against the previous binary, which accepts `{}` as a passing
  parity record. Six Python tooling tests and 59 level-zero repipe smoke checks
  pass. The subsequent combined end-to-end and workspace gates also pass.
- The first fixed-snapshot optimized workspace run took 417 seconds including
  an 80-second build. Only the existing `golden_opbehavior` vector test failed:
  row 14113 selected the wrong NaN sign. The numeric fix and companion CLI/WASM
  cleanup passed all four gates independently, with 7,697 workspace tests;
  their integration here subsequently passed the combined gates.
- The integrated workspace run passed 7,367 tests with 38 existing ignores and
  no warnings, in 414 seconds including a 66-second cold build. Upstream and
  stage parity remain 675/675 and 1467/1467; all 268 CLI probes, catalog/spec
  checks, and the independent baseline CLI harness pass. Focused numeric and
  WASM release suites passed 116 tests with one existing ignore. Representative
  CLI stdout and stderr remain byte-identical to the original baseline.
- Sixteen isolated Python tests cover inventory validation, publication timing,
  interleaved writers, failed serialization/replacement, collision handling,
  permissions, long destination names, and the cache's existing best-effort
  persistence policy. The long-name regression fails with destination-derived
  scratch names and passes with fixed-length unique sibling names. The
  59-case level-zero smoke suite passes after the writer consolidation.
- Worker wire fixtures pass before and after the module split. All 29 worker
  unit tests pass; the moved codec bodies are byte-identical apart from the
  three visibility qualifiers needed by the parent module. The combined compiler,
  worker and private-helper snapshot passed all four gates: 7,369 workspace tests
  with 38 existing ignores and no warnings, 675/675 upstream assertions,
  1467/1467 stage assertions, and spec checks. All 268 CLI probes, 16 Python
  tests, and catalog checks pass. The full workspace run took 428 seconds.
- Required-fixture negative controls: with only this worktree's x86-64 spec link
  withheld, the prior console no-return, SLEIGH alignment-NOP and Ghidra faillog
  tests all passed without their assertions. The changed executables each exit
  101 with the spec missing and pass after restoring it. The ARM context test
  and context-commit checks across 149 languages pass. The pinned compiler hash
  test passes all 44 source cases; the previous four wrapper tests are replaced
  by one manifest-driven test without reducing spec coverage.
- The first strict-fixture workspace run exposed an invalid existing test image:
  `exact_address_selection_preserves_nondefault_space` placed a symbol at 0x100
  in the 8051's one-byte INTMEM address space. Previously it reported a setup
  skip. The fixture now uses 0x40 in both spaces, retaining both name and exact
  address-space assertions. All other tests in that run passed.
- The corrected, combined snapshot passed all four gates: 7,372 workspace tests
  with 38 existing ignores and no warnings in 434 seconds; 675/675 upstream and
  1467/1467 stage assertions; and spec checks. All 268 CLI probes, 16 Python
  tests, catalog checks, and 56 Ghidra release tests (including the normally
  ignored breadth case) pass. The focused compiler/SLEIGH run passed 357 tests.
  This build also reproduced all 44 pinned C++ XML hashes. Fourteen CLI cases
  covering graph exports, summaries, reachability, Mach-O slices, image readers,
  and callee ordering match the original baseline's stdout, stderr and status.
- Before moving the graph scheduler, all seven planning tests passed, including
  two new checks: exhaustive directed graphs up to four nodes against independent
  reachability, and a 100,000-function chain that exercises the iterative walk.
- After the module split, all 299 focused CLI tests and 358 compiler/SLEIGH
  tests pass. The 14 saved CLI cases remain byte-identical, including status and
  stderr. A syntax-tree comparison confirms 44 moved function bodies retain
  their tokens after normalizing formatting and the new graph method receiver.
  Twenty interleaved summary runs measured medians of 146.396 ms before and
  148.728 ms after (+1.6%); this is a local warm-run check, not a speedup claim.
- The combined graph/decoder snapshot passed all four gates: 7,375 workspace
  tests with 38 existing ignores and no warnings in 411 seconds, 675/675
  upstream and 1467/1467 stage assertions, and spec checks. Catalog checks,
  16 Python tests and 56 Ghidra tests pass. The first concurrent CLI run missed
  the disassembly timing bound (673 ms minimum versus 600 ms required), with
  correct output. The unchanged probe passed on rerun; an interleaved comparison
  measured minima of 313 ms before and 315 ms after, with identical output.
  Both the targeted rerun and the later complete 268/268 CLI run passed without
  changing the bound. The first failure log is retained outside the repository.
  Twenty further interleaved pairs after those suites finished measured
  406/435 ms wall medians and a 1.012 median paired ratio, again with identical
  output; the variation does not establish a small speedup or slowdown.

- Two JSON-helper regressions fail with the former scanners: nested metadata
  supplied the wrong count, and malformed JSON was accepted. They pass with the
  standard parser, including escaped names and unchanged raw-record bytes.
  The initial affected CLI run found four incorrect summary paths in the new
  helpers; those tests were corrected to read `summary.reachable_from_entry`.
  An earlier graph assertion similarly needed `binary.label`.
- The integrated template change passes 360 compiler/SLEIGH release tests,
  including the unchanged 44-spec binary oracle and new sparse-section/error
  tests. Independent allocation and speed measurements are recorded in
  `notes/deslop-slacomp.md`.
- The frozen JSON-test/template snapshot passes all four gates: 7,379 workspace
  tests with 38 existing ignores and no warnings, 675/675 upstream assertions,
  1467/1467 stage assertions, and spec checks. All 268 CLI probes, 16 Python
  tests, 56 Ghidra tests, and catalog checks pass. All 44 pinned C++ XML hashes
  also remain unchanged. A focused project run overlapped replacing its own
  executable and failed worker spawning with `No such file or directory` in
  every retained error record; the unchanged project and triage suites then
  passed 34/34 and 16/16. The complete workspace run passed without that failure.
- The shared environment reader passes 3,061 core unit tests, including all
  existing per-gate environment checks and an explicit token/default matrix.
  The context fix passes 360 compiler/SLEIGH release tests and all 45 binary
  oracle cases. Before applying it, the new context fixture produced SHA256
  `a8e6d023...` instead of the pinned C++ stream's `7c2c42f2...`.
- The first environment/context workspace run passed 7,379 tests and failed
  only the relocated-manual test with `ETXTBSY`. That failure reproduced twice
  in 40 unchanged parallel runs; 40 serial runs passed. A process trace showed
  other test children being spawned while the executable copy was open for
  writing. A test-only mutex now excludes copy and process spawning; it is
  released before waiting for output, and no assertion or timeout was relaxed.
- All 45 pinned C++ XML outputs match after the context fix. Fourteen saved CLI
  cases remain byte-identical to the original baseline. Twenty alternating
  summary runs measured 141.136 → 139.949 ms wall medians (140.008 → 139.007 ms
  child CPU), with a 0.9993 median paired wall ratio. Other tests were running;
  this local check does not establish a small speedup.
- The corrected frozen snapshot passes all four gates: 7,380 workspace tests
  with 38 existing ignores and no warnings, 675/675 upstream and 1467/1467
  stage assertions, and spec checks. All 268 CLI probes, 16 Python tests,
  56 Ghidra tests and catalog checks pass again. The repaired nine-test docs
  suite also passed 100 debug and 100 release repetitions (1,800 assertions),
  including the profile and parallel-test setup that reproduced the failure.

- All 13 declaration-helper tests pass before and after the name allocator
  refactor. A first-free-name reference checks 64,000 assignments independently;
  a standalone comparison of the extracted implementations agrees on another
  640,000 calls, including reserved suffixes, occupied and unlisted names,
  generated names reused as bases, and empty/Unicode strings.
- The extracted production allocator was measured in 30 alternating rounds of
  50 runs after warmup, pinned to CPU 40. Median per-run times were 140.340 →
  77.003 µs for 512 unique names, 251.041 → 142.415 µs for 512 repeated names,
  751.061 → 421.346 µs for 2,048 mixed names, and 284.572 → 229.863 µs with
  1,024 reserved suffixes. Separate allocation instrumentation measured requests
  of 1552 → 1028, 3087 → 1540, 14104 → 8068, and 4621 → 3075, respectively;
  cumulative requested bytes fell 53%, 53%, 51%, and 31%. These are local helper
  measurements, not whole-decompilation speedups or peak-memory measurements.
  A single-tree prototype was rejected because two workloads slowed 19–31%.
- The name table's narrow lookup-only Clippy expectation is fulfilled. Existing
  collection-policy failures elsewhere remain: the release engine-library check
  reports 216 errors, down from the prior 222; this is not a passing Clippy gate.
- Both included-constructor diagnostic regressions reproduce with the saved
  old compiler and pass afterward. All three direct constructor-builder tests
  fail on the former owned-vector path and pass after unification. The combined
  compiler/SLEIGH release suite passes 365 tests, including all 45 binary oracles.
- The frozen naming/constructor snapshot passes all four gates: 7,387 workspace
  tests with 38 existing ignores and no warnings, 675/675 upstream and 1467/1467
  stage assertions, and spec checks. All 268 CLI probes, 16 Python tests,
  56 Ghidra checks and catalog validation pass. All 45 pinned C++ XML outputs
  and the 14 saved CLI cases are unchanged. Twenty alternating summary pairs
  measured 137.650 → 132.882 ms wall medians and 136.781 → 131.968 ms child CPU;
  the median paired wall ratio was 1.002, with identical output. Concurrent
  workspace activity means this does not establish a small speed difference.

- Corrupt JSON, invalid text, unusable row fields and a corrupt secondary
  backlog fail selection explicitly. Six driver regressions fail on the old
  script: selector, claim and reaper errors; claim races; disk pauses; and error
  shutdown with a live worker. A held-worker test now verifies that shutdown
  really waits, and an end-to-end case runs the real selector against corrupt
  temporary data. Git, GitHub and actual agent workers are never invoked.
- The 34 Python tests pass. Before the final end-to-end case was added, the
  33-test suite passed ten consecutive repetitions. Across 200 valid backlogs
  and six CLI modes, 1,200 output/status comparisons match the old selector.
  Sixty alternating reads of 1,000 rows after warmup measured 2.228 → 3.856 ms
  medians: validation adds roughly 1.6 ms to this local file-read workload.
- Macro sharing passes the existing 365 compiler/SLEIGH release tests and two
  new API checks for cloned-symbol replacement, shared borrowing and independent
  template copies. Compiler allocation and timing evidence is recorded in
  `notes/deslop-slacomp.md`.
- The frozen dispatch/macro snapshot passes all four gates: 7,389 workspace
  tests with 38 existing ignores and no warnings, 675/675 upstream and 1467/1467
  stage assertions, and spec checks. All 268 CLI probes, 34 Python tests,
  56 Ghidra checks, 367 compiler/SLEIGH tests and catalog validation pass.
  All 45 pinned C++ XML outputs and the 14 saved CLI cases remain unchanged.

- Three branch-action compatibility tests pass on the old implementation,
  including eligible flip candidates, unchanged IR and edge order, action/group
  identity, empty initialized graphs and the existing missing-root panic.
- A focused caller prototype retains the same name allocator on both sides.
  Removing the owned name counter and intermediate string vector reduced
  allocation/reallocation requests from 2062 → 1037 (512 unique names),
  2566 → 1541 (512 repeated), 12171 → 8074 (2,048 mixed), and 5134 → 3085
  (1,024 reserved suffixes). Cumulative requested bytes fell from 131694 → 94058,
  54336 → 40992, 310006 → 226710, and 257824 → 190444, respectively.
  Thirty alternating rounds of 50 runs after warmup, pinned to CPU 40, measured
  196.352 → 140.904, 138.261 → 116.353, 593.492 → 503.879, and
  382.601 → 307.036 µs median per run. These are local helper measurements,
  not whole-decompilation speedups or peak-memory measurements.
- The temporary-order regression fails against the previous compiler, which
  exits successfully for `local tmp:4; r0=tmp; tmp=1;`. The fixed test checks
  both a read before its write and a read in the writing operation, plus a valid
  ordered control, against the pinned compiler's error text and exit behavior.
- After the changes, all 3,066 core unit tests and 368 compiler/SLEIGH release
  tests pass. The rendered-signature set's narrow Clippy expectation is fulfilled;
  the remaining engine collection-policy errors decrease from 216 to 214.
- The frozen branch/declaration/temporary-order snapshot passes all four gates:
  7,393 workspace tests with 38 existing ignores and no warnings, 675/675
  upstream and 1467/1467 stage assertions, and spec checks. All 268 CLI probes,
  34 Python tests, 56 Ghidra checks and catalog validation pass. All 45 pinned
  C++ XML outputs remain unchanged. The 14 saved CLI cases and three additional
  declaration-rendering cases retain identical stdout, stderr and status.
- Twenty alternating whole-binary declaration-rendering pairs measured
  157.691 → 153.834 ms wall medians and 157.026 → 153.131 ms child CPU, with
  identical output. The median paired wall ratio was 0.9997. Concurrent
  workspace activity means this does not establish a small speed difference.

- The limits test reported success before, and fails afterward, when only this
  worktree's required x86-64 spec link is temporarily withheld. The link is
  restored; all seven limits tests pass with the required input present.
- An earlier header test silently passed when `cc` was replaced by a command
  exiting 1. The shared optional-tool check now fails that case; a missing `cc`
  still takes the documented optional skip. New process-helper tests cover
  non-executable tools, nonzero exits, a 512 KiB stream on each pipe, and timeout
  termination. All four passed 100 repetitions, and the focused helper/limits
  run passed 11 tests. An initial after-check accidentally used a stale test
  executable; that log is excluded from the comparison.
- The borrowed checker passes all 368 compiler/SLEIGH release tests, including
  its diagnostic cases and 45 independent binary oracles. Independent compiler
  allocation and timing measurements are recorded in `notes/deslop-slacomp.md`.
- The affected CLI release suites pass all 94 whole-binary and 34 project-export
  tests, including worker failures, watchdogs, streaming errors, and optional
  compiler round trips.
- The frozen process-helper/checker snapshot passes all four gates: 7,397
  workspace tests with 38 existing ignores and no warnings, 675/675 upstream
  and 1467/1467 stage assertions, and spec checks. All 268 CLI probes, 34 Python
  tests, 56 Ghidra checks and catalog validation pass. All 45 pinned C++ XML
  outputs and 17 saved CLI comparisons remain unchanged.

- Eight offline dashboard tests cover complete identifiers, valid asset and
  artifact lookup, source precedence, missing/unreadable directories and the
  existing link policies. The complete-identifier check fails before the change
  because a trailing newline was accepted. No live server or external service
  was used. The first test setup used a Python 3.11-only unittest helper; it was
  corrected to `ExitStack` for the installed Python 3.10 before the comparison.
- All 42 Python tooling tests pass. Across 960 before/after probe reads, result
  dictionaries are identical. Forty alternating rounds measured 74.7 → 92.2 µs
  per lookup with 32 files, 74.5 → 323.2 µs with 512, and 76.2 → 1773.9 µs with
  4,096. These local warm-directory measurements include JSON reading and cover
  early, middle, late and missing names. The explicit listing adds work; this is
  not a performance improvement or a whole-dashboard throughput measurement.
- The collision-warning regression fails against the saved old compiler; its
  no-collision controls already pass. All 370 compiler/SLEIGH release tests pass
  after restoration, including 45 pinned binary oracles. The compiler library
  Clippy check passes; the new visited set has a narrow, fulfilled membership-only
  expectation. Independent diagnostic, XML and timing evidence is recorded in
  `notes/deslop-slacomp.md`.

Final thirteenth checkpoint validation: all four required gates pass. The full
workspace reports 7,399 passed, zero failed, 38 existing ignores across 436
groups, and no warnings. Upstream and stage parity remain 675/675 and 1467/1467;
268 CLI probes, 42 Python tests, 56 Ghidra tests, spec and catalog checks pass.
All 45 pinned C++ XML outputs and the 17 saved CLI comparison cases remain
unchanged. Source and new-file hashes were checked again after the final test
process exited. On published commit `8d8ace707`, all six CodeQL analyzer jobs
and the aggregate CodeQL check pass; the PR-ref open-alert query returns no
findings. The four dashboard findings are no longer reported. No alert was
dismissed or suppressed.

## Fourteenth checkpoint: profiler ownership and compiler finalization

The action profiler manually paired `enter` and `leave` around `apply`. A caught
action panic left both the child and enclosing group on the timing stack: their
rows disappeared, and subsequent successful actions never published the file.
A new isolated integration test fails before the fix on the missing panic row.
Afterward it checks both unwound rows, publication after the panic, accumulating
later actions, and ignored write errors. It passed 100 repeated release runs.
A separate nested-action executable independently reproduces the old failure
and confirms the corrected row/file behavior.

One private profiler state now owns the root label, open frames and totals.
`Action::perform` holds a thread-bound drop guard around each timed application.
The public profiling functions, cached enablement and text format are unchanged.
Six deterministic accounting tests replace sleep-based assertions and cover
exclusive child costs, captured root labels, repeated rows, saturating time,
empty-stack handling, exact formatting and cost ties. All 3,068 core release
unit tests pass. Rendering borrows keys rather than cloning them. The collection
policy expectation is narrow and fulfilled; the engine's remaining Clippy errors
fall from 214 to 211, not to zero. The profiler and orchestration headers no
longer claim process-wide aggregation or describe implemented flow/printing as
stubs.

The extracted production renderer matches the prior implementation for 3,072
deterministic tables. Thirty alternating rounds of 25 renders, after warmup and
pinned to CPU 41, measured 18,062 → 15,715 ns for 32 rows, 136,276 → 121,580 ns
for 256, and 804,597 → 706,251 ns for 2,048. Separate allocation-counter runs
report 104 → 72, 780 → 523, and 6,159 → 4,111 requests, respectively. Requested
bytes fall from 10,856 → 9,234, 100,706 → 75,216, and 815,992 → 643,022. These
are local rendering costs and cumulative allocation requests, not whole-engine
speedups or peak memory measurements.

Twenty alternating whole-CLI pairs per profiling mode, with two warmup pairs
and CPU 41 affinity, preserve stdout, stderr and status exactly, both between
versions and with profiling on/off. Disabled median wall time is 102.770 →
102.948 ms (paired ratio 1.0014); enabled is 108.459 → 107.534 ms (paired ratio
1.0008). CPU medians are 102.289 → 102.464 ms and 107.133 → 107.082 ms. Both
versions publish nonempty profiles when enabled. The shared-host measurements
do not establish a whole-decompilation speed change.

The compiler now uses canonical constructor references and shared address-space
lookup policy. Final passes borrow arenas/table lists and update referenced
templates in place, retaining traversal order and missing-section behavior;
the empty cross-reference pass is removed. All 370 compiler/SLEIGH release tests
pass. Independent byte-oracle, diagnostic, allocation and timing evidence is in
`notes/deslop-slacomp.md`. No baseline or existing oracle was changed.

Final fourteenth checkpoint validation: all four required gates pass, with
7,402 workspace tests passed, zero failed, 38 existing ignores across 437
groups, and no warnings. The 675 upstream and 1,467 stage assertions, 268 CLI
probes, 42 Python tests, 56 Ghidra tests, spec and catalog checks pass. All 45
pinned C++ XML references and 17 saved CLI cases remain identical. Frozen source
and new-file hashes match after the final process exits; no baseline moved.

## Fifteenth checkpoint: one JSON writer and completed compiler paths

Compact and indented CLI JSON shared their value model but duplicated recursive
rendering. The unsorted indented path also allocated an index vector for every
object, even though the stored field order was already correct. One private
writer now owns all three layouts. Only sorted objects with multiple fields
allocate an ordering vector; it contains borrowed references and uses stable
sorting. Numeric tokens, duplicate keys, ASCII escaping, indentation, field order
and the public rendering functions remain unchanged. A stale abandoned-plan
comment in transcript extraction was removed; extraction behavior is unchanged.

Five new conformance tests pass both before and after the refactor. They pin
nested layout, raw numeric spelling, stable duplicate-key sorting, empty values,
control/surrogate escapes and every Unicode scalar's ASCII-safe round trip.
All 224 CLI unit tests pass. The production writer also matches 30,003 saved-code
comparisons and 3,000 independent CPython formatting comparisons. A CLI-only
Clippy run reports no JSON-module diagnostics, but still finds 21 existing
collection-policy errors and 22 warnings elsewhere; it is not a clean gate.

For arrays containing 32, 256 and 2,048 representative records, unsorted indented
rendering allocation requests fall from 75 → 11, 526 → 14 and 4,113 → 17;
cumulative requested bytes fall from 17,912 → 16,376, 143,352 → 131,064 and
1,146,872 → 1,048,568. Compact and sorted allocation counts are unchanged on
these inputs. Thirty alternating timing rounds of ten renders after warmup,
pinned to CPU 41, measured indented output at 13,719 → 12,646 ns, 102,638 →
99,004 ns and 572,100 → 552,285 ns. Compact timings were 1.2–1.6% higher;
sorted timings ranged from unchanged to 2.1% higher. A const-generic prototype
did not establish a useful advantage over the simpler layout enum. These are
local helper measurements, not an across-the-board speedup or peak-memory claim.

Twenty alternating whole-command pairs after two warmup pairs retain identical
stdout, stderr and status. The 828,889-byte catalog JSON median wall time is
13.936 → 13.757 ms (paired ratio 0.9921); a whole-binary JSON decompilation is
103.115 → 102.791 ms (paired ratio 0.9957). CPU medians are 13.326 → 13.187 ms
and 102.634 → 102.298 ms. Both were pinned to CPU 41 on the shared host; the
small differences do not establish a whole-command performance improvement.

The compiler's stale sub-byte register-alias rejection is replaced by registration
of the already implemented bitrange symbol. The root's old-code oracle test
fails on the added fixture. Two small fixtures extend the independent oracle to
47 cases, retaining the original 45 hashes. Attachment directives now share
their duplicate-warning and replacement logic. Seven direct root before/after
attachment comparisons preserve complete diagnostics, status and output bytes,
including existing error behavior. All 370 compiler/SLEIGH release tests pass.
Independent C++ diagnostics, images and compiler timing evidence are recorded in
`notes/deslop-slacomp.md`; no parity baseline changed.

Final fifteenth checkpoint validation: all four required gates pass. The full
workspace reports 7,407 passed, zero failed, 38 existing ignores across 437
groups, and no warnings. Upstream/stage parity is unchanged at 675/675 and
1467/1467; 268 CLI probes, 42 Python tests, 56 Ghidra tests, spec and catalog
checks pass. All 47 pinned C++ XML cases and 17 saved CLI comparisons match.
Frozen source and all four new-file hashes match after the final process exits.
CodeQL and parity CI also passed on the preceding commit `76f3769d4`.

## Sixteenth checkpoint: worker header ownership and symbol compaction

The worker header merger now has a private module instead of sharing the pool's
lifecycle implementation. It parses each block once and borrows canonical
definition spans for membership checks. Previously it allocated every definition,
parsed union inputs twice and cloned each candidate again before insertion.
The first containing block still wins verbatim; otherwise the original first
block and later unseen definitions retain input order. Existing CRLF, missing
newline, incomplete definition and spacing behavior is deliberately unchanged.

Seven new conformance tests and the existing merge test pass before the change.
An additional test verifies borrowing versus normalization ownership. All 232
CLI unit tests pass afterward. A saved-code comparison checks 4,096 inputs:
953,269 stdout bytes and 603,172 stderr bytes match exactly, including warnings.
A separate production-source helper comparison checks 20,000 outputs and
warning decisions. Four ELF/PE fixtures at two and four workers produce eight
matching exports: all 32 C, header, assembly and README artifacts are unchanged.

Thirty alternating rounds of five helper calls after warmup, pinned to CPU 41,
measured the following medians. The timing and allocation harness replaces the
warning write with the same counter in both implementations; the separate
saved-code comparison above exercises real stderr. Requests include allocation
and reallocation calls; bytes are cumulative requested storage, not peak memory.

| Definitions per shard / case | Time, ns before → after | Requests before → after | Bytes before → after |
|---|---:|---:|---:|
| 16, containing block | 13,144 → 8,761 | 113 → 15 | 8,535 → 4,150 |
| 16, union | 44,309 → 15,648 | 414 → 22 | 34,518 → 13,642 |
| 128, containing block | 114,560 → 65,976 | 824 → 27 | 71,850 → 34,418 |
| 128, union | 331,455 → 113,646 | 3,123 → 34 | 287,344 → 110,830 |
| 512, containing block | 460,959 → 263,976 | 3,232 → 35 | 293,468 → 138,482 |
| 512, union | 1,279,047 → 469,703 | 12,353 → 42 | 1,162,588 → 446,062 |

Twenty alternating whole-project export pairs after two warmup pairs, with two
workers pinned to CPUs 41–43, preserve all four artifacts on every run. Median
wall time is 541.356 → 537.260 ms (paired ratio 0.9871); child CPU time is
683.411 → 675.419 ms. These shared-host measurements do not establish a
whole-command speedup. The CLI-only Clippy check has no new module diagnostics
and its narrow membership-only expectation is fulfilled, but 20 existing errors
and 22 warnings remain elsewhere.

Compiler symbol compaction now remaps scope name bindings alongside numeric
references. Its new public-API regression fails before the fix: looking up the
first retained name returns the second symbol. Repeated cleanup now preserves
name, numeric and scope-iteration agreement. A second regression pins removal
of macro/unused-table operands and empty scopes. Cleanup takes ownership of
removed symbols rather than cloning their names and child lists. All 372
compiler/SLEIGH release tests pass. Independent image, allocation and timing
evidence is recorded in `notes/deslop-slacomp.md`; no baseline changed.

Final sixteenth checkpoint validation: all four required gates pass, with
7,417 workspace tests passed, zero failed, 38 existing ignores across 437
groups, and no warnings. Upstream/stage parity remains 675/675 and 1467/1467;
268 CLI probes, 42 Python tests, 56 Ghidra tests, spec and catalog checks pass.
All 47 pinned C++ XML outputs and 17 saved CLI comparison cases remain
identical. Frozen tracked and new-file hashes match after the final process
exits. CodeQL and parity CI passed on the preceding commit `8474451f6`.

## Seventeenth checkpoint: replay cache and compiler pattern ownership

The replay cache no longer tracks renamed results in a separate set. Each
retained first-pass result carries its provenance; replacing or invalidating
that result updates the eventual rename count automatically. Results remain
sparse by target index, and first-pass and sweep results remain distinct.
Staleness checks compare borrowed answer names and full definitions, allocating
owned snapshots only when a result must be retained. A sweep that is not
predicted no longer constructs an unused key.

Four new conformance tests pass with the old comparison and with the new one.
They pin answer count, missing answers, held versus minted names, definition
changes under an unchanged name, and 15,360 mutated key comparisons. All 236
CLI unit tests pass. A separate extracted-production comparison checks 100,000
cases using actual `SynthRequest` values. Forty old/new whole-binary CLI cases
preserve stdout and successful status across four ELF/PE fixtures, text/JSON,
two/four workers, forced recompilation, serial fallback and failed installation.
The relevant diagnostic markers are required on each path; interleaved worker
stderr is not claimed byte-identical.

Thirty alternating rounds of 1,000 helper comparisons after three warmups,
pinned to CPU 41, measured matching keys of 1, 8 and 32 answers at 461 → 84,
4,894 → 628 and 12,700 → 1,953 ns. An initial mismatch measured 423 → 36,
3,283 → 30 and 11,492 → 24 ns, respectively. Each generated definition has
eight named fields. Separate allocator runs report 11 → 0, 81 → 0 and 321 → 0
requests, with 792 → 0, 6,336 → 0 and 25,542 → 0 cumulative requested bytes,
for either outcome. These are isolated comparison costs, not whole-engine
speed or peak-memory measurements.

Twenty alternating two-worker project-export pairs after two warmup pairs,
pinned to CPUs 41–43, preserve all four artifacts on every run. Median wall
time is 457.777 → 453.712 ms, but the paired median ratio is 1.0004; child CPU
time is 489.556 → 490.308 ms. The shared-host run does not establish a
whole-command performance change. The CLI-only Clippy count falls from 20 to
17 existing errors, with 22 warnings; it is still not a passing gate.

Compiler insertion moves one owned name into its scope map and reads duplicate
diagnostics from the stored symbol. Replacement updates the binding directly,
retaining rejected-insertion slot behavior. Pattern folding, context validation
and decision-tree setup borrow their constructor-owned inputs. Decision nodes
still own their simplified patterns, without copying the simplifier's result
again. All 372 compiler/SLEIGH release tests pass. Twenty direct root compiler
comparisons preserve complete stdout, stderr, status and encoded bytes across
default and strict pattern handling. Independent allocation, timing and public
API evidence is in `notes/deslop-slacomp.md`. No baseline or oracle changed.

Final seventeenth checkpoint validation: all four required gates pass. The full
workspace reports 7,421 passed, zero failed, 38 existing ignores across 437
groups, and no warnings. Upstream/stage parity remains 675/675 and 1467/1467;
268 CLI probes, 42 Python tests, 56 Ghidra tests, spec and catalog checks pass.
All 47 pinned C++ XML outputs and 17 saved CLI cases remain identical. Frozen
source and new-test hashes match after the final test process exits. CodeQL
and parity CI passed on the preceding commit `589d4c4ba`.

## Eighteenth checkpoint: failures that tests and the compiler must report

Ten round-trip test paths treated an unsuccessful `cc`, `gcc` or `clang`
version probe as an absent compiler. They now use the existing shared process
helper: only a spawn `NotFound` is optional, while a nonzero exit or other
spawn error fails the check. Successful probes and the spelling checks retained
without a compiler are unchanged.

Three Unix subprocess regressions exercise the real round-trip tests under a
child-only executable search path: missing compilers, executables returning 7,
and non-executable compiler files. They cover both a single compiler probe and
compiler-list selection, require that the named child test actually ran, and
check the failure diagnostic. Unique scratch directories are removed on drop;
the parent environment and repository fixtures are untouched. Before the fix,
the broken-tool regressions fail while the missing-tool control passes. All
three pass afterward, as do all 97 whole-binary CLI tests. Separate saved-test
runs also demonstrate both false greens before the fix.

The compiler now reports previously discarded pattern-building reasons, names
unused tables and reports both source locations for conflicting constructors.
Errors retain table-qualified constructor references, so distinct conflicts in
different tables no longer collapse merely because their local indices match.
The root's three new compiler tests fail before the fix and pass afterward;
all 375 compiler/SLEIGH release tests pass. Identical patterns remain errors,
and ordinary overlap errors remain controlled by the existing `-l` flag.

Direct root comparisons match all 27 focused cases against the saved pinned
C++ diagnostics, status and decoded-image references. Another 256 generated
cases retain pre-fix Rust status, stdout and accepted encoded bytes. Their
three acceptance/image differences and 96 diagnostic differences from C++
already existed; this fix does not adopt C++'s different suppression of later
errors involving an already-marked constructor. Independent compiler timing
and the detailed comparison evidence are in `notes/deslop-slacomp.md`.
No production CLI/decompiler path changed in the test cleanup, and no baseline
or existing oracle expectation moved.

Final frozen-tree validation passed all four required gates: 675 upstream and
1467 stage assertions, 7427 workspace tests (38 ignored, no warnings), and spec
checks. All 268 CLI probes, 42 Python tests, 56 Ghidra tests, 47 pinned compiler
XML comparisons and 17 saved CLI comparisons also passed. Twenty repeated runs
of the new compiler-probe subprocess tests passed all 60 checks. The tracked
diff and both new test files retained their frozen hashes through the last
suite exit. The preceding commit's six CodeQL analyses and parity CI passed.

## Nineteenth checkpoint: schedule provenance and decision-tree ownership

The schedule fixture called a raw C++ oracle had accumulated kuna-only passes
such as `structsynth`, `constspaceload` and `callpush`. It is now named
`list_action_decompile_snapshot.txt`, with its origin and update policy stated
accurately. The rename preserves all 285 lines and SHA-256
`33d651474dbf5a12c7bd3aedda5bad87afa75668ec037156b621e94d2af72aa3`.
The two byte comparisons, empty-allowlist assertions, pass-presence checks,
adjacency checks and other-root tests are unchanged. All 15 focused integration
tests pass. Obsolete claims about stripping passes, an unchanged upstream tree
and current C++ recapture commands have been removed from active documentation;
archived feature histories are left alone.

SLEIGH's crate, expression and p-code builder headers now describe the existing
compiler/runtime boundary instead of saying the compiler is unported. They
retain the actual context-free operand-evaluation limitation and its existing
error string. Stripping comment-only lines confirms the edited production
files and schedule unit tests otherwise match their previous contents.
Rustdoc succeeds; correcting two existing schedule links leaves 435 warnings
elsewhere in the engine documentation, so this is not a clean documentation
lint result. No schedule, assertion expectation or production behavior changes
in this part of the cleanup.

The compiler's decision-tree contribution sorts original pattern indices,
checks conflicts without losing identity, then moves the owned patterns into
the final order. Branch values are streamed instead of first collected. The
production path loses 41 lines and two deep copies of each terminal list;
public APIs and existing diagnostic suppression semantics are preserved.
An 11-line fixture adds independently captured binary/XML coverage for overlap
resolution, disjunction and specialization; all 47 previous entries are intact.

Root verification confirms the model's old/new functions exactly match the
before/after production source. Its 50,000 pattern-list comparisons pass, as do
all 375 compiler/SLEIGH release tests and 286 direct compiler comparisons of
status, stdout, stderr and raw/decoded bytes. The new fixture is compiled from
this worktree's path as well. Independent full-compilation allocation and timing
measurements are recorded in `notes/deslop-slacomp.md`; output is unchanged and
the measured wall-time deltas remain within -0.83% to +0.61%.

Final frozen-tree checks passed: 675 upstream and 1467 stage assertions,
7427 workspace tests (38 ignored, no warnings), spec/catalog checks, 268 CLI
probes, 42 Python tests, 56 Ghidra tests, 48 pinned XML comparisons and 17 saved
CLI comparisons. The tracked diff and both new paths retained their frozen
hashes through the final suite exit. The preceding commit's six CodeQL analyses
and parity CI also passed.

## Twentieth checkpoint: required native runs and compiler scratch ownership

Nineteen generated round-trip execution sites and five native-fixture sites
checked stdout without requiring a successful exit. They now use one required
command helper layered on the existing checked optional-tool helper. A missing
required command, launch error or nonzero exit fails with a diagnostic; an
absent optional C compiler remains optional. The process module is reached
through the existing shared test module instead of repeated path declarations.

The required-command regression fails on the extracted unchecked behavior and
passes with status checking; all five process-helper tests pass. Independent
child-only compiler wrappers make the real `globalref` and `elemptr` round-trip
programs print their usual output but return 7. Both actual tests pass before
the fix and fail afterward with the exit-7 diagnostic. No repository fixture
was changed or hidden for these checks.

Those two tests also returned before their spelling assertions on any fixture
launch error, despite reporting that spellings were checked. Native execution
now uses the same explicit Linux/x86-64 policy as adjacent tests, while all
spelling and export checks run on other hosts. Two additional tests exercise
the non-native path on this host; they pass even with failed compilers on the
child search path. Unique scratch names prevent the paired tests from sharing
directories. All 99 whole-binary CLI tests pass with unchanged output assertions.

The compiler contribution reuses a bounded 256-counter array across candidate
fields instead of allocating per score. Root verification confirms both old
and new model functions match the production source exactly; all 800,000
floating-point score bits and 5,000 selected fields agree. All 375 compiler and
SLEIGH release tests and 286 status/diagnostic/image comparisons pass. The
independent measurements in `notes/deslop-slacomp.md` report 605,069 fewer x86
allocation requests and 72,810,920 fewer requested bytes per compile, with
Toy/x86/Hexagon wall-time deltas of -2.21%/-5.31%/-0.80%. These are cumulative
allocation requests, not resident-memory savings. Its five recent evidence
sections were reordered intact to match implementation order.

Final frozen-tree checks passed: 675 upstream and 1467 stage assertions,
7430 workspace tests (38 ignored, no warnings), spec/catalog checks, 268 CLI
probes, 42 Python tests, 56 Ghidra tests, 48 pinned XML comparisons and 17 saved
CLI comparisons. The tracked diff retained its frozen hash through the final
suite exit. The preceding commit's six CodeQL analyses and parity CI passed.

## Twenty-first checkpoint: replay table transfer and shared symbol resolution

The CLI planner borrowed a temporary replay, copied every minted name and
request into its lookup map, then discarded the originals. It now owns that
temporary and consumes its table after deriving the answers, superseded names
and ordered worker message. The new consuming accessor preserves the existing
borrowed API. The ledger regression checks the original allocation is transferred,
the encoded table is identical, mint order is retained, and an empty replay
produces an empty table. All 15 shard tests and 236 CLI unit tests pass.

An external comparison using the real request types checks 10,000 table maps,
including duplicate names and nested field recipes. Every result agrees. For
32 eight-field requests, the isolated map handoff drops from 1,089 allocation
requests / 51,734 requested bytes to one request / 8,784 bytes. Both versions
receive prebuilt input; their preparation is excluded. The same isolated
handoff is 58.864 versus 18.545 microseconds on CPU 41. These numbers describe
table collection, not whole-program decompilation or resident memory.

All 40 before/after CLI cases preserve status and stdout across four ELF/PE
fixtures, text/JSON output and normal, forced, serial and failed-install paths.
Their expected diagnostic markers remain present; concurrent stderr order is
not claimed byte-identical. Twenty alternating project-export pairs preserve
all four artifacts, with a paired median wall-time ratio of 0.9837 on CPUs
41–44. Two warmup pairs also agree. This is a shared-host measurement, not a
claim of a general end-to-end speedup.

The resolution contribution replaces duplicate constructor-id and matched-leaf
walks with one borrowed decision walk. Non-subtable validation is also shared;
public APIs, errors and selected constructors are unchanged. Root verification
passes all 375 compiler/SLEIGH release tests, including the independent lift
fixtures, and all 286 compiler status/diagnostic/image comparisons. The separate
30-pair lift timing in `notes/deslop-slacomp.md` measures +0.18% wall time with
identical expected output in every run.

Final frozen-tree checks passed: 675 upstream and 1467 stage assertions,
7430 workspace tests (38 ignored, no warnings), spec/catalog checks, 268 CLI
probes, 42 Python tests, 56 Ghidra tests, 48 pinned XML comparisons and 17 saved
CLI comparisons. The tracked diff retained its frozen hash through the final
suite exit. The preceding commit's six CodeQL analyses and parity CI passed.

## Twenty-second checkpoint: synthesized-structure protocol ownership

The worker pool no longer owns the replay algorithm in the middle of its
scheduling code. `jobs/synth.rs` owns the first-pass and sweep caches, replay
plans, compatible renames, forced runs and serial fallback. Its parent-facing
interface is one controller and its result; process lifetime, scheduling and
wire records retain their existing owners. The six focused tests move with
the implementation and reuse the existing test fixtures.

A Rust syntax-tree comparison verifies that all 138 function bodies across
the original pool and replay tests retain every non-comment token. No assertion
or algorithm changed. The function header now points to the complete contract
in the spec instead of repeating it. All 236 CLI unit tests pass. The release
build and CLI documentation build are warning-free after removing the obsolete
parent import and correcting two preexisting documentation links/markup issues.

All 40 replay CLI cases match the prior committed binary's captured status and
stdout, including the expected fallback markers. Two independent 20-pair
project-export timings, plus warmups, preserve all four artifacts in every run.
The first paired median wall-time change is +2.02%; a confirmation without other
root probes running is +0.12%. Both are within the 5% budget. These shared-host
measurements do not establish a general speedup from moving the module.

The compiler contribution borrows normalized instruction blocks for zero-shift
algebra, uses the default decision node directly, and iterates context then
instruction fields explicitly. Public APIs and search order are unchanged.
Root verification confirms the complete before/after pattern modules and four
field-selection model functions match production. All 280,000 algebra results,
50,000 selected fields, 375 compiler/SLEIGH release tests and 286 compiler CLI
comparisons agree. Independent full-compilation allocation and timing evidence
is in `notes/deslop-slacomp.md`; the measured wall-time range is -1.22% to +0.34%.

Final frozen-tree checks passed: 675 upstream and 1467 stage assertions,
7430 workspace tests (38 ignored, no warnings), spec/catalog checks, 268 CLI
probes, 42 Python tests, 56 Ghidra tests, 48 pinned XML comparisons and 17 saved
CLI comparisons. The tracked diff and both moved/new source paths retained their
frozen hashes through the final suite exit. The preceding commit's six CodeQL
analyses and parity CI passed.

## Twenty-third checkpoint: truthful compiler guidance and pattern access

`kuna specs --diff` said there was no in-tree compiler oracle, claimed datatest
success replaced compiler-output comparison, and linked a deleted document.
It remains an informational command with exit 0 and no compiler invocation,
but now names the pinned Ghidra compiler test separately from the decompiler's
behavioral gate and links the current provenance documents. Help and the
embedded CLI manual agree. Six specification anchors and one field comment
now identify the production script builder rather than its test-only wrapper.

The real-CLI regression fails against the old message and passes afterward.
It points the child-only compiler override at a non-executable manifest, so
accidentally invoking a compiler cannot pass. All six help tests pass in cargo
and in the independently compiled test executable. The advertised compiler
command was run from the repository root and passes its pinned-oracle test.
The final release and CLI documentation builds are warning-free. The help-test
header no longer claims that its manually copied command list automatically
covers future additions; that duplication remains a separate design issue.

The compiler contribution shares mask/value extraction and instruction/context
comparison paths without changing public signatures or short-circuit order.
Root checks confirm the actual before/after modules match the compared source.
With overflow checks both enabled and disabled, all 1,844,850 word reads and
boundary outcomes, 280,000 algebra results and 60,000 predicates agree. All 375
compiler/SLEIGH release tests and 286 compiler status/diagnostic/image cases
pass. Independent allocation totals are unchanged; the full-compilation timing
range in `notes/deslop-slacomp.md` is -0.77% to +0.61%.

Final frozen-tree checks passed: 675 upstream and 1467 stage assertions,
7431 workspace tests (38 ignored, no warnings), spec/catalog checks, 268 CLI
probes, 42 Python tests, 56 Ghidra tests, 48 pinned XML comparisons and 17 saved
CLI comparisons. All eight gate processes exited zero, and the tracked diff
retained its frozen hash through the last exit. The preceding commit's six
CodeQL analyses, aggregate and parity CI passed.

## Twenty-fourth checkpoint: one command registry and conservative pattern queries

The CLI repeated its public command names in dispatch, top-level help and a
test-only list. A small ordered table now owns names and handlers; dispatch
and the help list both use it. The help tests discover names from the actual
binary and reject empty or duplicate entries before exercising every command.
All 18 handlers, result reporting, runtime-hint ordering, special aliases and
command order remain unchanged. No command or public flag was added.

The seven help/alias tests pass before and after the refactor, as do all 236
CLI unit tests. A controlled wrapper advertising an absent command demonstrates
the coverage improvement: the old copied-list test falsely passes, while the
new discovery test fails on that advertised command. The wrapper is external
verification only, not another repository harness. All 65 saved command cases
retain exact statuses, stdout and stderr, including no arguments, aliases,
unknown commands and each subcommand's help and usage errors.

With 130 alternating pairs after warmups on CPU 41, paired median wall time
changes are +0.94% for top-level help, +0.39% for decompile help and +0.40% for
fid help. All captured outputs are identical. These short-process shared-host
measurements stay within the 5% budget; they are not a general speedup claim.

The compiler contribution shares conservative OR truth queries and simplifies
maximum extents and positive word bounds. Empty alternatives and the stricter
all-alternatives instruction query keep their existing behavior. The actual
before/after modules match the compared candidates. Root runs with overflow
checks on and off agree on 1,844,850 reads and boundary outcomes, 40,000 OR
queries/simplifications, 280,000 algebra results and 60,000 predicates. All 375
compiler/SLEIGH release tests and 286 compiler CLI cases pass. Independent
allocation totals are unchanged; all timing runs, including contradictory
short Toy medians and their stable paired results, are retained in
`notes/deslop-slacomp.md`.

Final frozen-tree checks passed: 675 upstream and 1467 stage assertions,
7432 workspace tests (38 ignored, no warnings), spec/catalog checks, 268 CLI
probes, 42 Python tests, 56 Ghidra tests, 48 pinned XML comparisons and 17 saved
CLI comparisons. All eight gate processes exited zero and the tracked diff
retained its frozen hash. The preceding commit passed all six CodeQL analyses.
Its CI job passed the actual parity, CLI, catalog, Ghidra and browser-worker
checks, then failed before opening a browser page because Chrome did not create
its DevTools port within ten seconds. That job is not reported as green; the
discarded browser diagnostics prevent identifying the startup cause from its
log. No browser code changed in that commit.

## Twenty-fifth checkpoint: shared IR factories and token alignment

Carry-chain and array-stride rules copied the unique-output factory and
omitted its high-variable bookkeeping. Their helper outputs therefore lacked
HighVariables when the public rules ran after high-level assignment. The
ordinary schedule applies these rules earlier; this is an API-state invariant
failure, not a reproduced CLI crash. Both rules now use `Funcdata::new_unique_out`.
Matching, gates, opcode metadata and graph-edit order are unchanged. Stale
comments claiming the shared factory and architecture flags were unavailable
are removed. The two remaining ruleaction factory copies are still an audit
target, not silently folded into this change.

Two existing tests now run with high-level variables off and on, retaining all
their graph assertions and checking helper metadata. Compiled separately
against the old library, these fixtures give 8 passes and 2 intended failures;
against the rebuilt library all 10 pass. A second valid-IR probe against the
actual public rule types changes from 0/2 to 2/2. All 3068 engine-library tests
pass without adding a test harness or new test functions to the repository.
Forty ELF/PE whole-binary cases preserve exact output across worker counts and
forced replay/fallback modes. Twenty alternating project-export pairs after
warmups preserve all four artifacts; paired median wall time changes +0.27%,
within the 5% budget.

Token alignment shares directional slice comparison, retains reverse size
accumulation and copies common prefixes/suffixes once instead of repeatedly
inserting at the front. Root AST checks show all non-comment production tokens
match the model sources; the older model headers differ only in comments.
Both overflow modes agree on 72,000 alignment/algebra cases and 100,000 boundary
outcomes, including partial states and error order. All 375 compiler/SLEIGH
release tests and 286 compiler CLI cases pass. Independent allocation and
timing evidence is retained in `notes/deslop-slacomp.md`. The native release
build is warning-free. The preceding commit passed CodeQL and the complete CI
job, including the browser test; the earlier Chrome startup failure did not recur.

Final frozen-tree checks passed: 675 upstream and 1467 stage assertions,
7432 workspace tests (38 ignored, no warnings), spec/catalog checks, 268 CLI
probes, 42 Python tests, 56 Ghidra tests, 48 pinned XML comparisons and 17 saved
CLI comparisons. All eight gate processes exited zero and the tracked diff
retained its frozen hash. No baseline or existing output expectation changed.

## Twenty-sixth checkpoint: browser startup and expression evaluation

The browser smoke-test launcher discarded stderr, did not handle spawn errors,
and waited ten seconds after an exited process. It now reports spawn, exit,
invalid-port and deadline failures with bounded diagnostics, and cleans up its
owned profile on failure or close. The public return shape, Chrome flags and
default ten-second deadline are unchanged. A complete valid port line is
required before returning. Eleven build-free process-fixture cases cover these
paths, partial port publication and cleanup; CI runs them before the web build.

A deterministic exit-7 fixture with sentinel stderr fails against the old
launcher after 10,082 ms with only a generic message. The actual edited launcher
rejects in 57.5 ms with the status and sentinel intact. The repository's eleven
tests pass in ten repetitions. Three real Chrome launches and DevTools
evaluations pass, as do the six build-free scripts, a fresh warning-free WASM
build, worker/error checks, generated visualization checks and both complete
study-view worker and real-browser suites. This improves failure handling and
diagnosis; it does not establish the cause of the earlier remote Chrome failure.
The preceding published commit passed the complete CI job and all CodeQL checks.

Runtime expression evaluation and compiler leaf substitution now share their
arithmetic traversal, removing 52 production lines without changing public
entrypoints. Root AST checks match both actual modules to the model sources.
With overflow checks on and off, 357,744 outcomes across 12,336 trees agree,
including callback order, failures, panics and partial substitution cursors.
All 375 compiler/SLEIGH release tests, 286 compiler CLI cases and 65 CLI
command/status/output snapshots pass. The native build is warning-free.
Independent allocation totals are unchanged; compiler and runtime timings stay
within the 5% budget, with full evidence in `notes/deslop-slacomp.md`.

Final frozen-tree checks passed: 675 upstream and 1467 stage assertions,
7432 workspace tests (38 ignored, no warnings), spec/catalog checks, 268 CLI
probes, 42 Python tests, 56 Ghidra tests, 48 pinned XML comparisons and 17 saved
CLI comparisons. All eight gate processes exited zero; the tracked diff and
new startup test retained their frozen hashes. No baseline or existing output
expectation changed.

## Twenty-seventh checkpoint: rule factories and operand remapping

The remaining unique-output factory copies in `ruleaction_3.rs` and
`ruleaction_4.rs` now use `Funcdata::new_unique_out` at all nineteen production
call sites. Their two copied implementations and stale module descriptions are
removed. The addressed-output and output-reassignment helpers are unchanged;
their aliasing and register-lane behavior needs a separate review.

Two existing rule tests now exercise high-level state both off and on while
retaining their graph assertions. Before the production change, both fail only
at the new high-variable check. The edited engine passes all 3068 release
library tests, including both regressions, without warnings. Root AST checks
confirm that all 206 remaining production function bodies retain every
non-comment token except the nineteen redirected allocation calls. Rule guards,
registration, opcode metadata and graph-edit order are unchanged.

Two independent public-rule probes also change from 0/2 to 2/2; a follow-up
checks that each new output is recorded at the correct lane-storage address in
both high-level states. Forty ELF/PE whole-binary cases keep identical output
across worker counts and forced replay/fallback modes. All 286 compiler CLI
comparisons are exact, and the native release build is warning-free.

Twenty alternating project-export pairs after warmups, pinned to CPU 41,
preserve all four artifacts. Separate median wall time is 458.18 → 458.31 ms;
the paired median changes -0.02%, within the 5% budget. The preceding published
commit passed the complete CI job and all CodeQL checks.

Compiler operand remappers share a private left-to-right visitor, removing
24 production lines. Root comparisons in both overflow modes agree on 196,864
outcomes across 12,304 trees, including callback order, interrupted updates and
repeated index remaps. Both actual modules match the compared sources' AST
tokens. All 375 compiler/SLEIGH release tests pass without warnings. Allocation
counts are unchanged and compiler timing remains within the 5% budget. The
broader compiler comparison in `notes/deslop-slacomp.md` records lower allocation
counts and faster compilation against the saved pre-macro-sharing build; that
baseline follows the initial fixes and is not the original PR base.

Final frozen-tree checks passed: 675 upstream and 1467 stage assertions,
7432 workspace tests (38 ignored, no warnings), spec/catalog checks, 268 CLI
probes, 42 Python tests, 56 Ghidra tests, 48 pinned XML comparisons and 17 saved
CLI comparisons. All eight processes exited zero. A fresh warning-free WASM
build and all eleven web scripts, including the real-browser suite, also pass.
The tracked diff retained its frozen hash through the last check. No baseline
or existing output expectation changed.

## Twenty-eighth checkpoint: addressed outputs and runtime xrefs

Store promotion and extension shortening now use `Funcdata::new_varnode_out`
instead of the copied factory in `ruleaction_4.rs`. The copy and its unknown-type
helper are removed. Root AST checks confirm all 116 remaining production
function bodies preserve every non-comment token except these two calls.
Output reassignment remains a separate audit target. Nearby comments now
describe the active scope-property and cover handling instead of claiming it is
deferred; those central implementations are unchanged.

The new in-tree test fails before the fix and passes afterward. It exercises
high-level state and active/finished lane collection independently, preserving
the output address, definition, input and old-output unlinking assertions.
Two external public-API probes aggregate all little/big-endian and high-level
combinations before checking metadata; both fail before and pass after. All
3069 engine and 375 compiler/SLEIGH release tests pass without warnings, as does
the native release build. The 286 compiler CLI comparisons are exact.

Runtime register/context xrefs iterate the global scope directly, borrow names
until storage and use one register-map entry lookup. Root models in both
overflow modes match 48,000 outcomes across 2,000 symbol tables, including
duplicates, callback failures/panics and partial state. Both extracted models
retain the three exact production methods, and the applied module matches the
compared candidate byte-for-byte.

Root allocation probes reproduce 49/444/740 fewer initialization requests and
711/18154/14900 fewer requested bytes on Toy/x86-64/Hexagon. The three runs per
spec retain identical register/user-operation metadata and context sizes.
These are allocation traffic, not resident-memory measurements; instrumented
elapsed times are not used for speed claims.

Forty ELF/PE whole-binary cases retain exact output across worker counts and
forced replay/fallback paths. CPU-41-pinned runtime checks pass all sixteen
independent lift fixtures in 66 runs; thirty measured samples per version give
413.92 → 414.12 ms median wall time (+0.047%). Twenty alternating project-export
pairs after warmups preserve all four artifacts, with paired median wall time
changing +0.12%. Both measured deltas fit the 5% budget.

The frozen checkpoint passes every gate: 675/675 upstream and 1,467/1,467
stage assertions retain parity; the workspace finishes with 7,433 passed,
38 existing ignores and no failures or warnings across 438 groups. All 268
CLI probes, 42 Python tests, 56 Ghidra tests, 48 compiler XML comparisons and
17 saved CLI comparisons pass. A fresh WASM build and all eleven browser
scripts pass, including the real Chrome UI flow. Spec and catalog checks are
green, and no baseline expectation moved. The tracked diff remained
`29b0c54979d64b396a0b48c37aade0c5af53db93acd28cff1081a0c5b94bef91`
through the last gate; logs use `/tmp/kuna-deslop-addressed-final-`.

## Twenty-ninth checkpoint: shared output reassignment and borrowed register names

The last copied output-reassignment helper in `ruleaction_4.rs` linked the banks
but omitted the shared scope-property update. Public `RuleSubZext` calls in
high-level state therefore produced defined outputs without cover storage. The
middle-truncation and shifted-truncation probes both failed before the change;
their graph, sizes, masks and shifts were otherwise correct.

All three callers now use `Funcdata::op_set_output`, retaining old-definition
unlinking, the bank's reader-replacement callback and cover bookkeeping. The
obsolete claim that the central implementation was deferred is removed. This
corrects an IR invariant, not a new rewrite decision or default. One persistent
test checks both truncation shapes with high-level state off and on; it fails
before the helper replacement. Both independent public probes pass afterward.
An AST comparison checks all 115 retained production bodies: only the three
calls change non-comment tokens. All 3,070 core release tests pass.

Register-name helpers now borrow the selected bytes until their caller builds
the return value. Public byte-vector APIs remain owned; snapshot string APIs
avoid the intermediate vector. Root differential models retain all eight exact
production methods and match 3,488,000 outcomes across 2,000 maps in each
overflow mode, including aliasing spaces, boundaries, misses and invalid UTF-8.
The applied source matches the compared module byte-for-byte. All 375 focused
compiler/SLEIGH tests, 286 compiler CLI comparisons and 40 whole-binary ELF/PE
comparisons pass without output changes.

Root allocation probes reproduce 467 → 234, 13,987 → 6,994 and 5,007 → 2,504
requests for Toy/x86-64/Hexagon snapshot queries. Requested bytes decrease by
672/48,664/11,665; three repetitions preserve every result byte. These counts
include result collection but exclude initialization, query setup and teardown;
they are not peak-memory or whole-decompiler speed measurements.

CPU-41-pinned, uninstrumented lookup checks use 30 samples per version and
image after warmups. Median lookup times improve 13.80%/10.42%/8.66%, with
identical checksums. Full runtime checks pass all 16 independent lift fixtures
in 66 runs: median wall time changes 413.48 → 413.61 ms (+0.031%). Twenty
alternating project-export pairs preserve all four artifacts; paired median
wall time changes +0.078%. The full-runtime deltas fit the 5% budget. Evidence
is under `/tmp/kuna-deslop-output-link.DAmpMyRB`; the fresh native build has
no warnings.

All gates pass on the frozen checkpoint: 675/675 upstream and 1,467/1,467
stage assertions retain parity, and the workspace finishes with 7,434 passed,
38 existing ignores and zero warnings across 438 groups. All 268 CLI probes,
42 Python tests, 56 Ghidra tests, 48 compiler XML comparisons and 17 saved CLI
comparisons pass. The fresh WASM build and all eleven browser scripts pass,
including the real Chrome UI flow. Spec and catalog checks are green; no
baseline moved. The diff hash remained
`8b6a28289eae00bb554cc4ce33f2bc9eb5ca54572345891b4da6a4a9de2104ad`
through the final gate. Logs use `/tmp/kuna-deslop-output-final-`.

## Thirtieth checkpoint: a borrowed console request and shared display pieces

The single-function script builder took fourteen positional arguments, most
already stored in the parsed request. A private `decompile/script.rs` now owns
command construction and path quoting. It borrows `DecompileArgs`, while the
subprocess driver supplies the resolved input, effective address selection,
per-attempt defaults and output paths. Retry and transcript handling stay in
the driver. The test adapter likewise takes a request rather than repeating
twelve positional fields. An orphaned comment describing a removed boolean
parser is deleted.

The new raw-image regression pins the complete script, including spaced paths,
symbol installation, raw output and region output. It and all 42 existing
single-function unit tests pass before the move. Afterward all 237 CLI unit
tests pass with no warnings. Source checks preserve all 25 retained production
bodies except the one checked forwarding call, all four installed helper
bodies match the model, and all 44 existing test/fixture bodies preserve their
statements outside request construction. Each overflow-mode differential run
matches 11,324 complete scripts and 676 invalid-input panics across 12,000 cases.

Thirty-five real text-mode comparisons preserve stdout, exit status and engine
diagnostics: 32 successful requests plus missing-function, invalid-option and
strict-assertion failures. They include two targets, ordinary/spaced input
paths, raw/region output, declarations and directives. Both versions deliberately
use the same current engine; only its three-line build-identity warning is
excluded from diagnostic comparison. Forty whole-binary ELF/PE cases and all
286 compiler CLI cases remain exact. The refreshed CLI-only Clippy check has
17 collection-policy errors and 21 warnings, with no single-function module
diagnostics; it remains an open gate, not a passing one.

Constructor syntax construction examines the last piece once, and its three
printers share literal/operand emission. Public signatures, whitespace and
operand boundaries, flow-through dispatch, failure state and partial text stay
unchanged. The complete-module models retain the production tokens and match
245,760 outcomes across 4,096 cases in each overflow mode. Root-only pre-existing
unit tests are preserved. All 375 compiler/SLEIGH tests pass. Both public
assembly APIs agree at all 1,171 instruction locations from 16 lift fixtures,
with zero errors and strings identical to the saved Rust output. Those strings
are before/after evidence, not independent C++ assembly oracles.

CPU-41-pinned compiler measurements retain identical SLA images. Wall medians
change +3.21%/+3.06%/+1.12% for Toy/x86-64/Hexagon; paired medians change
-1.83%/+2.57%/+1.56%, and CPU medians change +0.59%/+1.45%/+0.57%.
The short Toy wall samples are noisy; no speedup is claimed. Sixty-six assembly
runs preserve every output byte (+0.55% median wall time), and 66 lift runs pass
all 16 independent fixtures (+0.45%). Twenty alternating text-command pairs
preserve output and diagnostics (+0.31% paired wall time); twenty project-export
pairs preserve all four artifacts (+0.41%). All deltas fit the 5% budget.
Root artifacts are under `/tmp/kuna-deslop-console-script.NABIGGZz`.

The frozen tree passes all nine final gates: 7,435 workspace tests, 38 existing
ignores across 438 groups and no warnings; 675/675 upstream and 1,467/1,467
stage assertions retain parity. All 268 CLI, 42 Python and 56 Ghidra tests,
48 binary/XML comparisons, 17 saved CLI comparisons, spec/catalog checks,
and the fresh WASM build with all 11 browser scripts pass. Tracked and new-file
hashes are unchanged through the final gate. No baseline expectation moved.

## Thirty-first checkpoint: string-filter ownership and native adapters

The string inventory delegates its existing pattern grammar and bounded matcher
to a private `strings/filter.rs` module. Repetition state travels as one value,
ordinary and counted quantifiers share suffix handling, and counted numbers no
longer allocate a temporary string. The obsolete claim that the workspace avoids
a regex dependency is removed; this is not a switch to the dependency's different
grammar. Unicode folding, empty repeats, overflow fallback, diagnostics and
budget warnings retain their existing behavior.

Two added regressions pass on the original implementation: numeric values and
cursor advancement, and the grammar's overflow/empty-repeat cases. All ten
string tests pass before the move; all 239 CLI and 375 compiler/runtime tests
pass afterward without warnings. Source checks retain all 14 inventory-command
functions, the complete modeled filter, and all 11 test/helper bodies. Each
overflow-mode model agrees on 82,092 compile/match outcomes across 4,116 patterns
and 60,033 numeric value/cursor outcomes. Eighty-three real CLI cases preserve
stdout, stderr and status exactly, including Unicode/wide text, attribution,
invalid patterns and budget exhaustion. Forty whole-binary ELF/PE cases and all
286 compiler CLI cases are unchanged. CLI-only Clippy remains open at 17
collection-policy errors and 20 warnings; no filter-module diagnostic remains.

The native engine's string register-name adapter now uses the borrowed lookup
already shared by the base API and snapshots. Emission drops an unused manager
argument, the build result is matched directly, and unimplemented-template
reporting no longer constructs an unused read-only walker. The eight modeled
methods match production; both overflow modes agree on 3,488,000 lookup outcomes
across 2,000 maps. Seventy-two real runtime outcomes match, including 32
unimplemented-template errors, partial emissions, retries, delay slots and
emitter panics. Assembly strings at all 1,171 locations from 16 lift fixtures
match the saved Rust baseline through both public APIs, with zero errors.

Three stable allocation samples reduce native lookup requests from 467 to 234
for Toy, 13,987 to 6,994 for x86-64 and 5,007 to 2,504 for Hexagon, retaining
identical results. Requested bytes fall by 672, 48,664 and 11,665 respectively.
These count lookup and result collection, not initialization or peak memory.
Each thousand counted-filter compile/match operations eliminates one or two
temporary allocations per operation across five representative patterns, with
the same result counts in all three repetitions.

CPU-41-pinned isolated filter timings change -17.54% for compilation and -2.81%
for matching (paired medians -17.65%/-5.46%). Native lookup medians change
-19.83%/-9.28%/-8.03% for Toy/x86-64/Hexagon. These are local component timings,
not whole-decompiler speed claims. The 66-run full assembly and lift workloads
change -0.27%/+0.21%; twenty project-export pairs retain all four artifacts
(+0.18% paired wall time). Single-command string timings were noisy at 1–2 ms,
so a second run batches 16 commands per sample: thirty measured samples per
version for each of four patterns change +0.50%/-2.84%/+0.45%/-0.25% in median
wall time, preserving every output and diagnostic. All complete workloads stay
within the 5% budget. Artifacts are under
`/tmp/kuna-deslop-string-filter.wte6INQZ`.

All nine final gates pass on the frozen tree: 7,437 workspace tests, 38 existing
ignores across 438 groups and zero warnings; 675/675 upstream and 1,467/1,467
stage assertions retain parity. Also green are 268 CLI, 42 Python and 56 Ghidra
tests, 48 binary/XML comparisons, 17 saved CLI comparisons, spec/catalog checks,
and a fresh WASM build with all 11 browser scripts. Tracked and new-file hashes
remain exact through the last gate. No baseline expectation moved.

## Thirty-second checkpoint: callee-first execution and p-code construction

The whole-program execution path moves from `decompile_all.rs` to its private
`callee_first.rs` owner. Planning remains in `callgraph`, loading and selection
remain in the command module, and project export keeps the same entry point.
Five repeated decompile calls share target cloning and the driver's copied
output options, changing only the planned prototype-parking bit. Caller-vote,
structure, callback and element-global rounds retain their order and failure
handling. Stale comments now acknowledge that parking changes recovered types,
element-global convergence follows callback parking, and budget ties use the
space/address key.

A new regression pins equal-charge admission across address spaces and input
orders; it and the three existing budget tests pass before the move. All 240 CLI
unit tests and 375 compiler/runtime tests pass afterward with zero warnings.
Source checks account for all 119 function/test bodies and 13 constants, with
only the five checked substitutions; the installed child matches the candidate.
The clean native build is warning-free. CLI-only Clippy still reports 17
collection-policy errors and 20 warnings, not a passing lint gate.

The saved before/after comparisons cover 125 callee-first requests, including
15 fixtures, text and JSON, default/off/cycles/lock policies, caller-vote and
callback settings, narrowed selections and decision traces. Twelve project
exports retain all four artifacts, stdout, stderr and status. These are not
vacuous option comparisons: the prototype-order fixture's caller signature
changes when the option is off, caller-vote traces record actual redos, and
callback traces record parked prototypes. Forty additional whole-binary
ELF/PE cases and 286 compiler CLI cases remain exact.

P-code construction shares input-location generation, clones the existing queued
operation before pointer adjustment, extends the varnode pool from a sized
iterator, and resolves each label through one varnode borrow. The cache uses its
derived default; indices, masks, wrapping, reuse and failure ordering remain.
The entire installed runtime module matches the checked candidate. Both
overflow-mode cache models agree on 589,824 stepwise outcomes across 4,096
sequences, including partial failures and reuse. All 144 dynamic read/write,
delay-slot, unimplemented-template, retry and emitter-panic outcomes match
across both byte orders. The tiny Rust-compiled fixtures match the decoded
contents of their pinned C++ images; only compression bytes differ.

The full allocation probe preserves 7,321 operations and 19,581 varnodes over
1,171 instruction locations, with identical results in three independent
repetitions per fixture. Requests remain 8,090 and cumulative requested bytes
remain 1,073,016; this workload shows no allocation reduction. Both public
assembly APIs retain the saved strings at all 1,171 locations, with zero errors.
CPU-41-pinned full lift and assembly medians change -0.05%/-0.24% over 66 runs
each. Twenty measured callee-first pairs per fixture change -0.22%/-1.24%/-0.98%
for budget, callback and cyclic-structure cases (paired -0.27%/-1.39%/-0.98%).
All stay within the 5% budget. Root artifacts are under
`/tmp/kuna-deslop-callee-owner.8wTNk6yj`; the existing corrected crate header is
preserved rather than overwritten by the independent documentation variant.

The frozen final tree passes all four required gates: 675/675 upstream and
1,467/1,467 stage assertions, 7,438 workspace tests (38 ignored, 438 groups,
zero warnings), and spec validation. The 268 CLI, 42 Python, 56 Ghidra and 48
compiler XML checks also pass, as do 17 saved CLI comparisons and all eleven
browser scripts against freshly built WASM. Both the tracked diff and new
callee-first module hashes remained unchanged through the final gate. The
CLI Clippy audit remains open at 17 errors and 20 warnings; it is not a passing
gate. Final gate logs use `/tmp/kuna-deslop-callee-final-*.log`.

## FID deduplication and runtime documentation

Cross-input FID dedup now borrows each name while building a membership mask,
then compacts the input in order after releasing those borrows. It keeps the
first record's metadata and distinguishes full hash, specific hash and name;
the database API, format and generation policy are unchanged. The lookup-only
set has a narrowly justified collection-policy expectation, since the mask
rather than hash iteration determines output order.

Two persistent tests pass against the extracted original two-statement logic
before replacement. They cover first metadata, order, hash collisions, aliases,
empty input and all-duplicate input. Both overflow modes match 8,192 sequences,
including 524,022 input records and their serialized databases per mode. Source
checks pin the installed helper to that model and all ten other production
functions, with only the checked forwarding substitution in the command.

For 4,096 unique records, dedup allocation requests fall from 4,108 to 13 and
cumulative requested bytes from 741,404 to 544,828. The duplicate-heavy case
falls from 4,101 to 6 requests and 74,796 to 8,268 bytes. Empty names instead
add one request (12 to 13), while bytes fall from 671,772 to 544,828. These are
dedup-only allocation traffic, not peak memory, verified in three repetitions.
Thirty measured CPU-41-pinned component pairs reduce paired time by 28.1%,
33.1%, 27.6%, 12.8% and 14.1% for unique, duplicate, full-hash-collision,
long-name and empty-name inputs respectively.

All 15 real FID CLI comparisons retain exact status, diagnostics and database
bytes, including repeated objects, duplicate archive members, header variants
and errors. Successful default-header cases also match the vendored database.
Thirty measured end-to-end pairs change median wall time by -0.03%/+3.59%/-0.90%
for single, eight-repeated-object and archive inputs; paired changes are
-0.003%/-1.44%/-0.14%. Every timed database and diagnostic also matches. The
component savings are not a claim of a measurable end-to-end speedup; all
three cases remain within the 5% budget.

Seven runtime module headers now describe implemented code instead of claiming
the decoder, IR emulation, snippet language or template mutators are absent.
They remove 170 lines, preserving the already corrected compiler/crate headers.
Every non-header byte is unchanged. Strict rustdoc passes before and after;
242 CLI and 375 compiler/runtime focused tests pass without warnings. The
286 compiler CLI cases match status, diagnostics and raw/decoded hashes. The
CLI Clippy audit is still failing, now at 16 errors and 20 warnings.

Root artifacts are under `/tmp/kuna-deslop-fid-dedup.Dk64MKp8`; the final native
build log is `/tmp/kuna-deslop-fid-final-build.log` (17.37 seconds, no warnings).

The final frozen tree passes 675/675 upstream and 1,467/1,467 stage assertions,
7,440 workspace tests (38 ignored, 438 groups, zero warnings), and spec checks.
Also green are 268 CLI probes, 42 Python tests, 56 Ghidra tests, 48 compiler XML
oracles, 17 saved CLI comparisons, and all eleven browser scripts against fresh
WASM. The tracked diff hash is unchanged through the final gate. Logs use
`/tmp/kuna-deslop-fid-final-*.log`; no baseline expectation moved.

## Shared query function records and XML-image buffers

`function_info` owns function attribution, inventory display names, the engine's
generated-name fallback and the ordered function JSON record. `crypto` no longer
depends on the `strings` command's implementation; strings, constants and
cross-references share the identical record shape. Cross-reference target and
global-name precedence remains separate. Source checks pin 39 retained function
and test bodies, all three old JSON helpers, the ownership helper and the
expanded inventory-name helper. Four parent functions contain only the checked
forwarding substitutions. Each installed query file matches its candidate.

The new exact-byte JSON regression passes before the move and after it. It
checks field order, Unicode/control-character escaping, zero, an address above
2^53, and `u64::MAX`. All 50 real query comparisons preserve stdout, stderr and
status, including ARM string ownership, crypto-constant ownership, PE import
aliases, empty queries and errors in text/JSON modes. The 243 CLI and 375
compiler/runtime focused tests pass without warnings. All 286 compiler CLI
cases retain exact status, diagnostics and raw/decoded hashes. Clippy remains
an open audit at 16 errors and 20 warnings, not a passing gate.

XML-image encoding writes hex directly into the content buffer; decoding shares
the existing permissive signed-byte conversion. The first padding pass retains
chunks in address order without collecting their keys or repeating map lookups.
The second pass still snapshots keys because it inserts pads that must not be
visited again. Padding is appended in one batch. The installed complete module
matches the reviewed candidate. Both root overflow-mode models match 90,984
outcomes, including all byte pairs, aliased spaces, wrapping endpoints, repeated
padding, partial reads, XML output and partial state after panics. The supplied
probe had only an older padding comment; the root copy was corrected before
verifying complete-module prefixes and identical probe suffixes.

The root public-API probe opens and encodes all 16 lift-fixture XML images three
times. Each repetition gives the same counts. Opening allocation requests fall
388 to 258 and cumulative requested bytes 46,001 to 35,089; encoding requests
fall 10,920 to 520 and bytes 238,388 to 155,188. All 26,106 encoded bytes match.
These isolated phase counts exclude initialization and XML parsing, and do not
measure peak memory. Both assembly APIs retain the saved output at all 1,171
instruction locations with zero errors.

CPU-41-pinned isolated open/encode timing changes -28.69%/-42.64%, over 66
balanced runs with 200 repetitions per phase and fixture. Full lift and assembly
wall medians change -1.06%/-0.38%, with all fixture outputs retained on every
run. Twenty measured real query pairs change -0.61%/-0.20%/-0.04% for strings,
crypto and cross-references (paired -0.55%/-0.29%/-0.04%). Every timing remains
within the 5% budget; small query differences are not claimed as speedups.

Root artifacts are under `/tmp/kuna-deslop-query-functions.L47PEIBK`. The final
native build is warning-free and takes 7.48 seconds
(`/tmp/kuna-deslop-query-final-build-clean.log`).

The frozen final tree passes 675/675 upstream and 1,467/1,467 stage assertions,
7,441 workspace tests (38 ignored, 438 groups, zero warnings), and spec checks.
All 268 CLI probes, 42 Python tests, 56 Ghidra tests, 48 compiler XML oracles,
17 saved CLI comparisons and eleven browser scripts against freshly built
WASM also pass. The tracked diff and new module hashes remain unchanged through
the last gate. Final logs use `/tmp/kuna-deslop-query-final-*.log`; no baseline
expectation moved.

## Owned archive staging and context buffers

FID archive ingestion now has a private owner. Each object member is written
through the already locked `tempfile` 3.27 library, then its writing handle is
closed before the path-based loader opens it. A path guard cleans up after
loading and on unwind; concurrent ingests have independently owned member
files. Archive parsing, classification, warning labels, record order and dedup
remain unchanged. Cargo adds the existing package as a CLI dependency without
changing package versions. Source checks preserve all nine parent production
bodies and existing tests, pin object classification, and restrict the moved
archive body to the checked staging/cleanup substitutions.

Three persistent staging tests cover independent lifetimes, private Unix
permissions, unwind cleanup and normal concurrent use. The exact helper and
tests pass in an external crate, including 50 repetitions (150 tests) with an
empty temporary directory afterward. A hermetic real-CLI test writes an archive
without an external tool, containing two duplicate objects and a non-object
member. It passes against original ingestion before the change and after it,
retaining the vendored FID bytes and leaving no staging files. This is normal
operation and ownership coverage.

All 15 saved FID CLI cases preserve status, diagnostics and database bytes,
including object/archive duplication, custom headers and errors. All 286
compiler cases preserve status, diagnostics and raw/decoded hashes. Focused
validation passes 246 CLI unit tests, the new CLI integration test, and 375
compiler/runtime tests without warnings. The native build is also warning-free
(18.86 seconds). The CLI Clippy audit remains failing at 16 errors/20 warnings.

Context words and masks resize their own buffers instead of allocating and
copying replacements. Cache hits/misses select their existing database slice
before one shared copy. The entire installed module matches the reviewed
candidate. Both root overflow-mode models match 376,832 outcomes across 2,048
sequences, including masks, clones, registration/defaults, range paints, cache
hits/misses, write masks and partial state after panics.

Three native initialization samples per spec retain exact metadata and stable
counts. Toy stays at 2,308 allocation requests, x86-64 drops 203,957 to 203,955,
and Hexagon stays at 78,230. Requested bytes increase 24/8/8 from
662,877/60,051,728/22,338,449 respectively. This is a small capacity tradeoff,
not a memory-saving claim; the measurements exclude input reads, metadata
formatting and teardown and are not peak memory. Both assembly APIs retain
the saved text at all 1,171 locations with zero errors.

CPU-41-pinned full lift and assembly wall medians change +0.80%/+1.30% over 66
balanced runs each, within the 5% budget. Thirty measured FID pairs per case
change +3.04%/-0.26%/-0.49% for single, eight-repeated-object and archive inputs
(paired +1.36%/-0.62%/-0.25%). Every timed database and diagnostic matches.
Small command timing changes are not claimed as speedups. Root artifacts are
under `/tmp/kuna-deslop-archive-owner.Ykj97osF`; the native build log is
`/tmp/kuna-deslop-archive-final-build.log`.

All nine final gates pass on the frozen source: 7,445 workspace tests with
38 existing ignores across 439 groups and no warnings; 675/675 upstream and
1,467/1,467 stage assertions retain parity. The 268 CLI probes, 42 Python tests,
56 Ghidra tests, 48 XML compiler comparisons, 17 saved CLI comparisons and all
11 browser probes pass, including a fresh WASM build. Spec/catalog checks pass.
The tracked diff and both new-file hashes remain unchanged through the last
terminal success. Logs use `/tmp/kuna-deslop-archive-final-`. No baseline moved.

## Console output ownership and runtime API cleanup

Text decompilation now owns its C and optional region-output files with
`TempPath` guards instead of generating unowned names and manually removing
them at the end. Creation is private; the writing handle closes before the
console opens the file. Both guards remain alive across discovery retries and
clean up on success, error or unwind. An unavailable output directory now
returns a direct driver error before spawning the console. No option, script
ordering, retry decision or output format changes.

A six-case real-CLI regression covers success, empty output and pipeline
failure with and without regions. It passes against the original driver and
after the change. Two unit tests cover independent lifetimes, readable/writable
files, private Unix permissions and unwind cleanup; another CLI test checks
the early creation failure. Source checks preserve 69 other function/test
bodies and all 33 existing integration helpers/tests. The decompile body differs
only in its two fallible path acquisitions and removal of manual cleanup.

All 35 saved text-command cases retain stdout, status and diagnostics, excluding
only the three-line build-identity warning from pairing the saved CLI with the
current console. Their temporary directory contains spaces and is empty after
every command. All 286 compiler cases retain status, diagnostics and raw/decoded
hashes. The final native build completes in 19.03 seconds without warnings.
Focused validation passes 248 CLI units, 25 text integration tests and 375
compiler/runtime tests, also warning-free. An earlier focused run used a stale
console and failed two build-identity assertions; rebuilding all native binaries
together resolves them. The scoped CLI Clippy audit remains 16 errors and 20
warnings; an unscoped attempt also encountered the known dependency lint debt.

Memory-state setters borrow address-space handles instead of cloning them, and
bank lookup uses one checked vector access. Complete-module models match 8,794
rows / 854,281 bytes in both overflow modes and against both native libraries.
Coverage includes both byte orders, bank aliases and missing indices, named and
varnode writes, borrow errors and bounded snippet programs. The snippet module
itself is unchanged; the broader snippet candidate remains excluded.

Five runtime modules lose 77 stale comment lines. The documented symbol
variants, snippet-language implementation and ownership boundaries now match
the code. Checks apply only the reviewed comment substitutions to the root
sources, preserving earlier symbol tests and documentation corrections; every
non-comment source line is unchanged by this documentation portion. Rustdoc
passes with broken intra-doc links denied.

Across 66 CPU-41-pinned runs, median memory-workload times change -4.30%/-4.14%
for little/big endian. Unchanged snippet workloads vary -4.14%/-4.40%; those
figures are collateral measurements, not a snippet optimization claim. Every
checksum matches. Full lifting changes +0.17% wall / +0.15% CPU. Twenty measured
text-command pairs change 97.133 to 97.204 ms (+0.07%, paired +0.05%), with exact
outputs and diagnostics. All are within the 5% budget. Root comparisons,
source proofs and timings are under
`/tmp/kuna-deslop-console-temp.0KbxK2Hh`; the native build log is
`/tmp/kuna-deslop-console-output-final-build.log`.

All nine final gates pass with tracked diff hash `6717a4fb` unchanged through
the last terminal success: 7,449 workspace tests, 38 existing ignores, 439
groups and no warnings. Upstream/stage parity remains 675/675 and 1,467/1,467.
The 268 CLI probes, 42 Python tests, 56 Ghidra tests, 48 XML comparisons,
17 saved CLI comparisons and 11 browser probes pass, including a fresh WASM
build. Spec/catalog checks pass. Logs use
`/tmp/kuna-deslop-console-output-final-`; no baseline moved.

## FID serialization lookup and format documentation

Name interning now uses one entry lookup instead of a contains-then-insert
sequence. The full-hash buckets, sorted traversal, insertion order inside each
bucket, first-encounter string offsets and reader are unchanged. The module
header describes the implemented database/interface and removes obsolete
future-PR claims. Four narrow collection expectations cover six lint sites:
the private full-hash index is sorted before serialization, and the name-offset
map is used only for lookup while the ordered records determine blob order.

A persistent format test pins the header, hash order, collision-bucket order,
shared/empty-name offsets and exact string blob. It passes on both original and
changed complete modules, alongside all nine existing database tests. Installed
source checks retain every other executable line and existing test. Actual
complete-module comparisons in both overflow modes match 8,192 databases,
519,571 records and 14,260,990 serialized bytes, including repeated serialization,
indexed queries and reserialization. Names include empty, shared, distinct,
Unicode and embedded-NUL strings; metadata retains the original byte behavior.

Three samples per five 4,096-record workloads retain identical serialization
allocation traffic and all 15 full output files. Unique/collision/long-name
workloads each make 44 allocation requests, repeated-name workloads 29, and
empty-name workloads 19. This change removes redundant lookup work, not memory
allocation; the counter measures cumulative requests/bytes, not peak memory.
Across 66 balanced CPU-41 runs, each measuring 128 serializations per workload,
median times change -9.04%/+2.86%/-2.25%/-15.18%/+0.61% for unique, repeated,
collision, long and empty names. Paired changes are -9.02%/+2.82%/-13.86%/-15.16%/
+0.53%; all remain within the 5% slowdown budget.

All 15 saved FID CLI cases retain status, diagnostics and database bytes; the
default success cases also retain the vendored database exactly. All 286
compiler cases match. Thirty measured command pairs per workload change
-1.17%/-1.28%/-1.33% for single, eight-repeated-object and archive inputs, with
every timed output and diagnostic checked. These small command differences are
not claimed as general decompiler speedups.

Focused checks pass 905 analysis tests, 248 CLI units and the FID integration
test without warnings. The native build takes 18.11 seconds and is warning-free.
Scoped analysis Clippy falls from 221 to 215 errors, with 64 warnings unchanged;
none remain in this module, but the crate is not lint-clean. Direct rustdoc of
the original and installed database modules passes with broken links denied and
no warnings. The broader analysis documentation check fails on 18 links in
unchanged files and reports 110 warnings; that separate cleanup remains open.
Artifacts are under `/tmp/kuna-deslop-fid-serialize.7t7ktkH9`; the native build
log is `/tmp/kuna-deslop-fid-serialize-final-build.log`.

All nine final gates pass: 7,450 workspace tests with 38 existing ignores
across 439 groups and no warnings; 675/675 upstream and 1,467/1,467 stage
assertions retain parity. The 268 CLI probes, 42 Python tests, 56 Ghidra tests,
48 XML comparisons, 17 saved CLI comparisons and all 11 browser probes pass,
including a fresh WASM build. Spec/catalog checks pass. Tracked diff hash
`a52fd8ea` remains unchanged through the final terminal success. Logs use
`/tmp/kuna-deslop-fid-serialize-final-`; no baseline moved.

### Analysis documentation ownership and borrowed temporary records

Eighteen analysis source files lose 187 comment lines while retaining every
non-comment, nonblank source line exactly. The format boundary now describes
all four implemented formats; the format-string and PDB headers identify their
current consumers. Obsolete field/type references are corrected. Mach-O entry
documentation no longer claims to decode unsupported `LC_UNIXTHREAD` commands.
The DWARF recursion guard is described in terms of its current `Guard` variants.

Module files own their documentation instead of also carrying duplicate parent
summaries. A small rustdoc fixture reproduces the scope problem: combining an
outer module summary with inner documentation resolves the inner links in the
parent scope; removing the duplicate restores the child scope. The original
analysis documentation has 18 broken public links and 57 with private items
included. Both checks now pass with `-D rustdoc::broken_intra_doc_links`.
There are still 111 other documentation warnings, so this is not a warning-clean
crate. No lint suppression, visibility change or executable edit hides an error.

The integrated compiler change lends overlapping temporary records to the
existing two-pass coalescer instead of collecting cloned records. Complete
before/after source matches the reviewed candidate; the extracted implementations
are verbatim installed code. The differential model reproduces 32,768 transitions
in each overflow mode, including panic text and partial state. Fresh allocation
probes reproduce all twelve full state files and stable costs: 4,867 to 2,819
allocation/reallocation requests and 454,216 to 203,800 cumulative requested bytes
across four workloads. This does not measure peak memory.

Across 66 balanced CPU-41 runs, median aggregate workload time falls 20.35%
(paired 20.40%); all four workloads improve. Full compiler wall-time changes are
+2.53%/-0.06%/-0.61% for Toy/x86/Hexagon, with paired changes
+0.05%/-0.28%/-0.82%; generated images match throughout. All 286 saved compiler
cases retain output bytes, diagnostics and status. The 1,280 focused tests pass
without warnings, as does the native build (19.53 seconds).

Artifacts are under `/tmp/kuna-deslop-analysis-docs.tmqCSugl`, including source
identity proofs, public/private rustdoc logs, the tiny scope fixture, fresh
record models and allocation files, and raw timing samples. The previous
checkpoint's CI job failed when Chrome did not publish a DevTools port in ten
seconds; its unchanged retry passed. The failure is retained in the artifacts.

The first local workspace run failed one top-level compiler-probe test: its
freshly written fake compiler produced `Text file busy` rather than the intended
exit 7. Forty isolated repetitions of the unchanged three-test probe group did
not reproduce the full-suite failure. The fixture now uses symlinks to a
checked-in script with the same bytes, avoiding a write/execute window while
other test threads spawn children. Missing-tool and non-executable-tool setup,
every assertion and all timeouts are unchanged. This is a test-fixture change,
not a retry or a relaxation in the production command runner. The corrected
three-test group passes once through cargo and in forty further parallel-harness
repetitions (120 test results). The old executable and all before/after outputs
remain in the artifact directory.

All nine gates pass after the fixture change: 7,450 workspace tests with 38
existing ignores across 439 groups and no warnings; 675/675 upstream and
1,467/1,467 stage assertions retain parity. The 268 CLI probes, 42 Python tests,
56 Ghidra tests, 48 XML comparisons, 17 saved CLI comparisons and all eleven
browser probes pass, including another fresh WASM build. Spec/catalog checks
pass. Tracked diff hash `14d65c1d` and fixture hash `4d8fe247` (mode 755) remain
unchanged through the final terminal success. Final logs use
`/tmp/kuna-deslop-analysis-docs-verified-`; the earlier failing run remains
under the `analysis-docs-final-` prefix. No baseline moved.

### Worker scratch ownership and compiler API documentation

Worker scratch storage now lives in `jobs/scratch.rs`. A `TempDir` guard owns
exclusive directory creation and cleanup, replacing the timestamp-derived path
and manual destructor. The owner-pid prefix, worker recognition, orphan sweep
and parent-liveness rules remain unchanged. Missing temporary parents are still
created. Unix creation requests mode 0700 before any content is written, and the
existing final chmod preserves owner access under restrictive umasks.

All 113 other function/test bodies are byte-identical, including the three
moved scratch tests. Three new cases cover independent lifetimes, missing parent
directories and failed creation without damaging an existing parent file.
Original and changed lifetime/unwind tests pass under umasks 000, 077 and 777.
The 251 CLI unit tests and 99 pool integration tests pass without warnings,
including worker death, retries, panic recovery and parent termination.

Eighteen pooled CLI comparisons cover three binaries, JSON/C output, explicit
worker counts, one-function chunks and full-load workers. Every run starts with
a missing temporary parent whose path contains spaces, and leaves it empty.
Stdout is byte-identical; stderr is compared after removing only the live
elapsed-time and optional ETA fields. Plans, counts, percentages and all other
messages remain exact. The initial comparison caught an optional ETA difference;
raw streams and that failed attempt remain available. All 286 compiler cases
retain output bytes, diagnostics and status.

Six compiler/pattern modules lose 73 stale comment lines, with every executable
line unchanged. The root parser's earlier header cleanup is preserved. Strict
SLEIGH/compiler rustdoc passes without warnings. The native build passes in
44.07 seconds without warnings. CLI Clippy still reports 16 errors and 20
warnings; this is not a lint-clean claim. Source proofs, scoped permission tests,
CLI streams and timing samples are under `/tmp/kuna-deslop-pool-owner.9HcYnQfa`.

Thirty balanced measured command pairs per fixture, after six warmups, change
median wall time +0.11%/+1.89%/+0.87% for fauxware, protoorder and C++ inputs.
Paired changes are -0.17%/+1.28%/+0.55%; CPU changes are +0.25%/+1.17%/+0.28%.
Every timed run checks output, diagnostics and scratch cleanup. These are small
whole-command differences, all within the 5% budget, not speedup claims.

All nine gates pass: 7,453 workspace tests with 38 existing ignores across
439 groups and no warnings; 675/675 upstream and 1,467/1,467 stage assertions
retain parity. The 268 CLI probes, 42 Python tests, 56 Ghidra tests, 48 XML
comparisons, 17 saved CLI comparisons and all eleven browser probes pass,
including a fresh WASM build. Spec/catalog checks pass. Tracked diff hash
`77863c11` and the new scratch-module hash `09b566ba` remain unchanged through
the last terminal success. Logs use `/tmp/kuna-deslop-pool-owner-final-`;
no baseline moved.

### Pattern normalization and context/image contracts

Pattern blocks use fixed-width leading/trailing bit counts and a reverse search
for their final nonzero mask word, removing 24 implementation lines. Leading
word removal, offset updates, mask/value shifting and sentinel handling remain
unchanged. The actual source matches the reviewed complete-module candidate;
the three context/image documentation files lose 94 comment lines without any
executable change. Their current cache, read-error, symbol-cursor and read-only
marker contracts are preserved.

Root differential checks match 32,768 outcomes in each overflow mode, including
malformed lengths, extreme offsets, panic text, partial state and repeat calls.
All 128 full block-state files match. A new persistent unit checks 2,080 bit-range
combinations against byte-based alignment expectations and verifies idempotence;
it passes with all 21 existing module tests on both original and changed sources.
The 376 focused tests pass without warnings. All 286 compiler cases, 48 XML
outputs and nine complete initialization metadata files match. Strict rustdoc
passes without warnings; the native build takes 57.27 seconds without warnings.

The fresh 66-run CPU-41 microbenchmark improves aggregate median time by 19.70%
(paired 19.67%). Its three ordinary cases improve 6.54%, 27.52% and 17.26%, but
the unchanged sentinel case is 7.08% slower (paired 6.79%). This reproduces the
independent measurement's adverse control result; it is not claimed as an
all-cases speedup. Recompiling both probes against the final library reproduces
their measured binary hashes exactly. Sources, complete outputs and raw timing
samples are under `/tmp/kuna-deslop-pattern-normalization.TQbIBC9M`.

Full compiler wall-time changes are -0.92%/-1.81%/+0.25% for Toy/x86/Hexagon;
paired changes are -0.68%/-1.67%/-0.45%, with every compiled image identical.
Fresh-engine initialization changes -0.82%/+0.03%/+0.17% (paired
-0.79%/-0.22%/-0.30%). These real-workload measurements are within the 5%
budget; the adverse isolated sentinel result remains separately reported.

Full instruction lifting changes -0.46% wall time (paired -0.35%) and -0.42%
CPU time; all 16 independent instruction fixtures pass in each of 66 runs.
Together the full compiler, initialization and lifting measurements justify
retaining the simpler normalization without claiming that every isolated path
improves. No allocation reduction or change in peak memory is claimed.

The first full browser gate reached its final test, then Chrome failed to
publish a DevTools port within ten seconds, before any application page loaded.
The unchanged full build/browser retry passes; the timeout and retry are saved
separately. No timeout or assertion was relaxed. The previous checkpoint's
remote CI and CodeQL checks also pass.

All nine gates now pass: 7,454 workspace tests with 38 existing ignores across
439 groups and no warnings; 675/675 upstream and 1,467/1,467 stage assertions
retain parity. The 268 CLI probes, 42 Python tests, 56 Ghidra tests, 48 XML
comparisons, 17 saved CLI comparisons and all eleven browser probes pass.
Spec/catalog checks pass. Tracked diff hash `7d5e051a` remains unchanged through
the last terminal success. Logs use `/tmp/kuna-deslop-pattern-normalization-final-`,
with the unchanged browser retry at `pattern-normalization-verified-web.log`.
No baseline moved.

### CLI lint contracts and shared image relocation

The CLI now passes Clippy with warnings denied. `make lint-cli` checks this
crate in release mode without linting dependencies; CI installs the Clippy
component and runs the target after the native build. This replaces the CLI's
16 collection-policy errors and 20 warnings with a checked gate, not a claim
that the other crates are lint-clean.

Narrow collection expectations record reviewed ordering contracts. Membership
sets preserve ordered target traversals; lookup maps retain ordered pattern
buckets or are consumed by requested address. The synthesized-structure cache
writes distinct result slots, and its count is order-independent. Serialized
tables come from the ordered replay, not the name lookup map. No collection
implementation changes, and there is no crate-level lint suppression. A new
persistent test pins target order, last-produced duplicate selection, one-time
consumption and missing-record errors; it passes on the original implementation.

Redundant dereferences and a forwarding closure are removed. Command handlers
and budget admission have local type names; resolved mode names and their
ordered options have named fields instead of positional tuple access. The
catalog and workflow headers describe current behavior without migration
narratives. Source checks account for all 425 CLI function bodies using only
the reviewed substitutions. A parsed workflow comparison permits only Clippy
installation and the new lint step.

The gate also passes on CI's Rust 1.98.1, installed separately from the local
1.90 default with an isolated build directory. That check exposed three more
mechanical cleanups: a stable key sort, a while-let loop and an unused final
wire-reader cursor advance. The record decoder's checked slice reads bound
that advance by the remaining frame; accepted frames and trailing bytes are
unchanged. A fourth lint incorrectly treats reordered short-circuit callbacks
as identical. Its narrow allowance preserves greedy/lazy matching and budget
accounting. A persistent six-case test pins callback order, early success and
remaining budget, and passes before the annotation. The newer compiler also
reports an existing unused SLEIGH assignment; it is not a CLI warning and is
left intact because removing its subtraction changes checked-overflow behavior
on malformed bit ranges. No dependency-wide warning suppression was added.

XML chunks and symbols share ordered relocation, removing 14 implementation
lines and two temporary reference-count increments/decrements. Root models
match 92,160 outcomes in each overflow mode, including collisions, aliased
space indices, stale cursors, read-only markers, reads, XML and panic state.
All sixteen complete public-API XML images match. Two compiler ownership
documentation files lose 33 comment lines with executable bytes unchanged;
the root's earlier tests remain intact. The mutable-symbol lookup description
says "empty slot", since removal as well as incomplete decoding can leave one.

All 628 focused tests pass without warnings, including 252 CLI units. The 36
CLI comparisons preserve stdout, stderr and status across queries, modes and
catalog formats; saved before binaries include the matching console executable.
All 286 compiler cases and 48 XML comparisons match. Strict compiler/SLEIGH
rustdoc passes without warnings. The native build passes in 100 seconds,
including time waiting for concurrent release checks. Artifacts are under
`/tmp/kuna-deslop-cli-lint.0Hs47O3b`.

Across 66 balanced CPU-41 public-API runs, aggregate relocation time falls
4.08% (paired 4.39%). Empty/small/medium/large cases change
+3.19%/-4.57%/-2.25%/-7.56%, all within the 5% slowdown budget. No allocation
or peak-memory improvement is claimed. Thirty measured CLI pairs per fixture,
each running function inventory, graph export and crypto scanning, change
median wall time -0.66%/-0.88%/-0.70% for fauxware/protoorder/C++ inputs
on the final CI-compatible source.
Every timed command checks output, diagnostics and status against the saved
baseline. These small command differences are not general speedup claims.

After the additional CI-toolchain fixes, all 705 release CLI tests pass across
47 groups with no warnings. The native rebuild takes 6.92 seconds. All 36 CLI
comparisons still match exactly, and eighteen pooled runs match saved stdout
and stderr apart from elapsed time and optional ETA. Missing/spaced temporary
parents are created and left empty. The original and updated timing samples
are retained separately.

All nine gates pass on the frozen source: 7,456 workspace tests, 38 existing
ignores, 439 groups and no warnings; 675/675 upstream and 1,467/1,467 stage
assertions retain parity. The 268 CLI probes, 42 Python tests, 56 Ghidra tests,
48 XML comparisons, 17 saved CLI comparisons and eleven browser probes pass,
as do spec/catalog checks and `make lint-cli`. Diff hash `1fbad77d` remained
unchanged through the last terminal success. Logs use
`/tmp/kuna-deslop-cli-lint-final-`. Prior remote CI and CodeQL also pass. No
baseline moved.

### Browser exit ownership and borrowed operand expressions

The browser launcher now observes process exit independently of stderr closure
and releases the stderr stream when closed. A subprocess retaining that pipe
previously made an exited browser report a timeout instead of its exit code.
The new regression fails on the original helper after four seconds with the
wrong timeout, and all twelve startup cases pass across ten candidate runs.
The installed helper passes too. Live-browser readiness still has its existing
ten-second deadline. This synthetic failure is not established as the cause
of the earlier real-Chrome startup timeouts.

Constructor pattern building borrows the operand's defining expression instead
of cloning its tree before selecting a definition. Defining symbols still take
precedence, including malformed operands with both definitions. Actual root
modules match all 12,288 modeled outcomes in each overflow mode across 4,096
symbol tables: errors, panic payloads, partial state and retries are preserved.
The model includes 4,623 clean and 980 panic outcomes per mode. Emulator callback
documentation loses 109 comment lines while every executable line stays exact;
default hooks, missing registrations, replacement and unsupported operations
are described from their implementations.

All 376 focused tests and strict compiler/SLEIGH rustdoc pass without warnings.
The native build takes 45.02 seconds without warnings. CI's Rust 1.98.1 CLI
lint gate passes, retaining the separately recorded pre-existing SLEIGH
dependency warning. Seven candidate files match the reviewed source exactly;
the operand model includes the complete installed module. Artifacts are under
`/tmp/kuna-deslop-browser-exit.y9Hr4jNf`.

All 286 compiler cases, 48 XML comparisons and 36 CLI comparisons remain exact.
Public compiler allocation probes retain all nine complete images and diagnostic
streams. Three fresh-process samples per specification agree: Toy/x86/Hexagon
requests change 16,816/2,671,156/681,381 to 16,798/2,671,102/680,770; requested
bytes change 2,328,425/233,236,618/75,827,030 to
2,327,849/233,234,890/75,807,478. Across one compilation of each, that removes
683 allocation/reallocation requests and 21,856 requested bytes. These are
cumulative requests, not peak-memory measurements. Compiler construction is
outside the counter, and both versions use identical input/output paths.

Balanced CPU-pinned compiler timings remain within the 5% budget. Median
wall-time changes are -1.74%/+1.27%/+0.96% for Toy/x86/Hexagon; paired medians
are +0.51%/+1.35%/+0.81%. All compiled images match across 110 measured Toy
runs and twelve runs each for x86 and Hexagon per version, after warmups.
The measured allocation reduction and simpler ownership do not imply a
general compilation speedup.

All nine gates pass: 7,456 workspace tests, 38 existing ignores across 439
groups and no warnings; 675/675 upstream and 1,467/1,467 stage assertions retain
parity. All 268 CLI probes, 42 Python tests, 56 Ghidra tests, 48 XML comparisons,
17 saved CLI comparisons and eleven browser probes pass, including the twelve
startup lifecycle cases. Spec/catalog checks and CLI linting pass. Frozen diff
hash `4186b98f` stayed unchanged through the last terminal success. Logs use
`/tmp/kuna-deslop-browser-exit-final-`. The prior published checkpoint's CI and
CodeQL checks also pass. No baseline or browser deadline changed.

### Borrowed dataflow walks and runtime context commands

`check_indirect_use` borrows each visited Varnode's descendant sequence instead
of allocating a temporary copy. The function only reads the IR; its ordered
worklist and local membership set retain traversal and cycle handling. Marking
still collects accepted inputs before setting flags. The shorter module header
describes the current behavior and preserves the default-OFF warning about
unsound source-side merges. Neither the option nor its default changes.

A new persistent test checks that repeated rejected walks preserve every flag,
including existing traversal marks. All ten module tests pass on both the
original and updated implementation. Actual-IR models agree on 91,161 query
and marking outcomes per overflow mode across 4,096 graphs, including cycles,
absent nodes, read-only flags and repeated marking. Two narrow membership-only
expectations remove three collection-policy errors; every other Clippy
diagnostic is byte-identical. The engine still has 208 such errors and 117
warnings, not a clean gate.

The public-API allocation probe builds its IR outside the counter. Across
empty/single-reader/eight-link/64-link/rejected-64-link cases, requests change
2/3/16/78/78 to 2/2/7/13/13, and requested bytes change
60/68/604/4,908/4,908 to 60/60/532/4,388/4,388. Three fresh-process samples
agree per case: 140 fewer requests and 1,120 fewer cumulatively requested bytes
across one of each case, not peak memory. Both complete stage transcripts,
including option-off/on runs and the unsafe-merge counterexample, remain exact.

Runtime context application borrows stored commands and expression trees in
order instead of cloning them. The actual helper bodies agree on 36,864 modeled
outcomes per overflow mode with a probe walker; this is not a whole-runtime
model. Real-engine probes separately match 192 complete context/p-code records
and 144 error/retry outcomes. Native context decoding confirms `inst_next2`
refusal while uninitialized; nested application in the helper model is a
synthetic stress case, not evidence that real decoding allows that operation.

For 1,171 actual instruction calls, three samples per fixture retain the same
statuses and hashes while requests fall from 8,090 to 2,110 and requested bytes
from 1,073,016 to 824,424. Initialization is outside the counter. These are
cumulative allocations; complete p-code is separately checked by the sixteen
golden fixtures. All 376 focused compiler/SLEIGH tests pass without warnings,
as do strict rustdoc and CI's CLI lint gate (with the existing dependency warning).
The combined native build takes 45.86 seconds without warnings; all 286 compiler
cases and 48 XML outputs match. Source proofs cover all six candidate files and
preserve every other engine lint diagnostic. Artifacts are under
`/tmp/kuna-deslop-indirect-walk.hN59Nwlm`.

Balanced CPU-pinned traversal timings use thirty measured samples per version
after three warmups. Empty/single/eight-link/64-link/rejected-64-link median
changes are +3.82%/-14.27%/-2.50%/-3.08%/-3.51%; paired medians are
+2.71%/-14.42%/-2.16%/-2.66%/-3.56%. The empty case is slower despite its
unchanged allocation count. Both full stage workloads retain exact transcripts
on every run and change +1.04%/+0.74% in median wall time (paired +0.78%/+0.75%).

The first sixty-six instruction-probe runs improve aggregate median time by
4.88% (paired 4.72%). Individual medians range from -38.38% to +4.87%; the
small `gp` fixture is the slowest, with a +5.55% paired median. Whole lift-oracle
runs change -0.94% wall and -0.98% CPU (paired -0.80%/-0.72%), passing all
sixteen fixtures on every run. These measurements do not imply that every
individual instruction workload improves.

The retained follow-up runs ninety measured pairs after three warmup pairs,
checking all sixteen result records every time. Aggregate median time changes
-5.15% (paired -5.01%); `gp` changes +2.94% (paired +3.08%), and the slowest
individual median is +3.37%. The original near-boundary sample remains above
instead of being replaced by this larger follow-up.

All nine gates pass on the frozen source: 7,457 workspace tests, 38 existing
ignores across 439 groups and no warnings; upstream 675/675 and stage
1,467/1,467 assertions retain parity. The 268 CLI probes, 42 Python tests,
56 Ghidra tests, 48 XML comparisons, 17 saved CLI comparisons and eleven
browser probes pass, as do spec/catalog checks and strict CLI linting.
Diff hash `0dcbf51e` remained unchanged through the last terminal success.
Logs use `/tmp/kuna-deslop-indirect-walk-final-`. The prior published commit's
CI and CodeQL checks pass. No baseline moved.

### Index-key cleanup and borrowed runtime handles

Varnode index construction drops four redundant clones of derived-`Copy` keys
and sixteen references that were immediately dereferenced. Cached keys, tree
ordering, range bounds and insertion order are unchanged. The flag-class
description is moved off the address key onto its own type; bank ownership
headers now describe the current op-arena interface instead of unfinished port
work. These changes remove nineteen comment lines without adding suppressions.

All 39 Varnode-related tests pass before and after. Source comparison limits
executable changes to the twenty reviewed expressions. Engine Clippy warnings
fall from 117 to 97; its 208 collection-policy errors remain. Every remaining
diagnostic matches apart from moved Varnode source positions and the once-only
lint-default note moving to the next warning. This is not a clean engine gate.

Runtime handle resolution borrows defining expressions and result templates.
The immutable table retains ownership while the parser context receives handles;
error paths and preceding updates are preserved. Constructor documentation no
longer incorrectly says its compiler pattern/error fields were removed. A
strict private-documentation check found three pre-existing broken links in
SLEIGH; qualifying two and rendering the cross-crate architecture name as code
makes public and private rustdoc pass with all warnings denied. These final
link fixes change no executable source. The initial failing check is retained.

Fresh root-library probes match 360 complete handle/context records, 144
error/retry outcomes and sixteen instruction status/count/hash records. Across
1,171 instruction calls, requests fall from 2,110 to 1,412 and requested bytes
from 824,424 to 802,088; three fresh-engine samples per fixture agree. Engine
initialization is outside the counter, and these are cumulative requests, not
peak memory. The sixteen independent lift fixtures check complete p-code.

The final native build takes 46.91 seconds without warnings. All 376 focused
compiler/SLEIGH tests, strict private rustdoc, 36 CLI comparisons, 286 compiler
comparisons, 48 XML outputs and two option-off/on stage transcripts pass.
CI's Rust 1.98.1 CLI lint check passes with its existing SLEIGH dependency warning.
Source proofs cover all six candidate files, including the documentation-only
changes. Artifacts use `/tmp/kuna-deslop-varnode-keys.hkkfXDEm`.

Balanced CPU-pinned timings use thirty measured samples per version after three
warmup pairs. The instruction probe changes -3.38% in aggregate median time
(paired -3.31%); every individual workload remains within the 5% budget.
Whole lift-oracle wall/CPU changes are -0.79%/-0.98% (paired -0.25%/-0.12%),
with all sixteen oracles passing in each of sixty-six runs. The two full stage
workloads change -0.16%/-0.18% (paired +0.03%/-0.08%) and retain their exact
stdout, stderr and exit status on every run.

All nine gates pass: 7,457 workspace tests, 38 existing ignores across 439
groups and no warnings; 675/675 upstream and 1,467/1,467 stage assertions retain
parity. All 268 CLI probes, 42 Python tests, 56 Ghidra tests, 48 XML comparisons,
17 saved CLI comparisons and eleven browser probes pass. Spec/catalog checks
and CLI linting pass. Frozen diff hash `1fb76f96` remained unchanged through the
last terminal success. Logs use `/tmp/kuna-deslop-varnode-keys-final-`. The prior
published checkpoint's CI and CodeQL also pass. No baseline moved.

### Borrowed block queries and smaller operand-walker state

Common-subexpression and earliest-use queries borrow the Varnode's descendant
sequence instead of copying it. Both are read-only. The first still chooses
the first eligible equal op in descendant order; the second chooses the lowest
within-block order. Cutoff and stale-identifier checks retain their evaluation
order, including for empty descendant lists. The neighboring mutating query
keeps its snapshot.

Two persistent tests pin these ordering and validation contracts. All 45
op-manipulation tests pass before and after. The actual two method bodies,
exported through an immutable wrapper, agree with each other and the rebuilt
root-library methods on 131,072 outcomes per overflow mode across 2,048 IR
graphs. Each mode includes 8,489 matches and 50,534 matching panics; all Varnode
flags remain unchanged. The probe checks these methods, not the entire engine.

Ten actual-library allocation cases cover both queries with empty, self-only,
eight-reader, 64-reader and other-block lists. Three fresh-process samples agree
per case. Empty queries already allocate nothing; each of the other eight
queries drops its one temporary allocation, for eight fewer requests and
2,240 fewer cumulatively requested bytes across one of each case. All ten now
allocate zero during the query. This is not a peak-memory measurement.

Operand evaluation retains only the synthetic instruction offset it uses.
An optional offset replaces the unused constructor/length state, and the
fallback reuses one walker construction. Explicit defining expressions are
borrowed; symbol-produced expressions remain owned. Current parser ownership,
cursor rebasing and instruction-mask snapshot contracts replace stale and
repeated port commentary. The preceding private-documentation link fixes remain.

The unchanged offset-helper bodies agree on 368,640 modeled outcomes per overflow
mode over 8,192 contexts: 4,753 offsets, 84,960 fallbacks, 104,366 errors and
174,561 panics. This substitutes private context/cursor probes, not the full
expression evaluator. Real runtime probes separately match 1,080 complete
p-code/context records, 144 error/retry outcomes and sixteen fixture hashes.
Standard instruction allocations remain 1,412 requests and 802,088 requested
bytes per 1,171 calls; no allocation improvement is claimed for this refactor.

The native build takes 47.38 seconds without warnings. All 376 focused
compiler/SLEIGH tests, strict public/private rustdoc, 36 CLI comparisons,
286 compiler comparisons, 48 XML outputs and two option-off/on stage transcripts
pass. CI's CLI lint gate passes with its existing dependency warning; engine
Clippy totals remain 208 errors and 97 warnings. Exact source proofs cover five
candidate files. Artifacts use `/tmp/kuna-deslop-block-queries.0XQu4vPw`.

Balanced query timings use thirty measured samples per version after three
warmup pairs. All ten cases improve: empty CSE/earliest-use queries change
-0.42%/-10.91%; self-only -48.59%/-50.15%; eight-reader -42.19%/-51.40%;
64-reader -51.01%/-39.35%; and other-block -19.61%/-20.95%. Every result is
checked, with fixture setup outside the timer and no allocation instrumentation.

The initial sixty-six-run instruction timing changes +1.08% in aggregate
(paired +0.81%), but `lzcount` is +10.38% (paired +7.72%) and the x86-16
fixture +5.37% (paired +0.70%). Those adverse results remain recorded.
A larger balanced recheck of all sixteen fixtures uses ninety measured samples
per version: aggregate -0.46% (paired -0.46%), `lzcount` -1.76% (paired -2.27%)
and x86-16 -0.36% (paired -0.40%). The largest individual increase is +1.09%
(paired +1.51%). All complete result records match in both runs. No instruction
speed improvement is claimed; the larger run does not reproduce the overruns.

Whole lift-oracle wall/CPU changes are +1.49%/+1.61% (paired +0.19%/+0.35%),
with all sixteen oracles passing in each of sixty-six runs. Two full stage
workloads change +0.18%/+0.22% (paired +0.30%/+0.40%), preserving exact stdout,
stderr and exit status on every run. These full workloads remain within budget.

All nine gates pass on the frozen source: 7,459 workspace tests, 38 existing
ignores across 439 groups and no warnings; 675/675 upstream and 1,467/1,467
stage assertions retain parity. Python, Ghidra, XML, saved CLI, browser,
spec/catalog and strict CLI-lint checks pass. The first concurrent CLI run
passed 267/268 probes: the 40-instruction disassembly probe's fastest sample
was 670 ms against its unchanged 600 ms bound. After the builds finished,
the complete CLI suite passed 268/268, followed by another successful run of
that named probe. Both logs remain; no threshold or expectation changed.
Frozen diff hash `7561e5ce` remained unchanged through the last terminal
success. Logs use `/tmp/kuna-deslop-block-queries-final-`; no baseline moved.

### Reachable snippet grammar and redundant engine expressions

Six redundant casts, borrows and a forwarding closure are removed from call
recovery, symbol synchronization and declaration rendering. Values already have
the required types; the signed-to-unsigned Varnode size conversion remains.
The alias branch's stale instructions to enable code that already runs are
replaced by its current contract. The first restructuring pass skips the
unmapped-alias check; later passes and the final sync enable it. No alias or
merge behavior changes. Engine Clippy drops from 208 errors/97 warnings to
208/91: exactly three redundant-cast, two needless-borrow and one forwarding-
closure warnings disappear. All other primary diagnostic messages and source
text match; this does not make the engine warning-clean.

The snippet parser loses its private `New` token and builder, which neither
the byte lexer nor symbol classifier produces. `new` remains an ordinary
identifier, including a local name or language-defined user operation.
The actual `borrow` token retains its syntax-error path. Documentation now
describes the parser's real reset behavior and numbers context-word bits from
most-significant 0 to least-significant 31. Context-bit noncomment code and
derived-debug fields remain unchanged.

Two persistent tests pin identifier/user-operation handling, `borrow` rejection,
and reset of non-space locals, flow symbols, results and diagnostics while
retaining spaces and the temporary base. All 43 parser tests pass on the
original implementation. Complete before/after parser modules match 10,656
outcomes per overflow mode over 444 byte programs, both byte orders, four
bindings for `new`, and fresh, reused and cleared parsers. Each mode has
3,122 accepted outcomes and no panics. Supporting library modules are real;
this models the parser, not the full instruction decoder. The rebuilt native
library matches every complete XML template, diagnostic and parser-state record.

The native build takes 45.73 seconds without warnings. All 3,073 engine units
and 378 focused SLEIGH/compiler tests pass, as do strict public/private rustdoc,
CI's CLI lint gate (with its existing dependency warning), 36 CLI comparisons,
four complete stage transcripts, 286 compiler comparisons and 48 XML outputs.
Source proofs cover all ten candidate files. Artifacts use
`/tmp/kuna-deslop-redundant-expressions.7T2LiUPy`.

Balanced CPU-pinned parser timings use thirty measured samples per version
after three warmup pairs. Aggregate median time changes -1.26% (paired -1.33%);
all eight configurations improve by 1.26% to 1.71%. Every run preserves the
result digest across 113,664 parses, with language and program setup outside
the timer. Complete call-push, argument-guard, array-cover and stack-alias stage
workloads change +0.13%/-0.21%/+0.08%/-0.46% (paired
+0.36%/-0.18%/-0.29%/-0.43%). Each has sixty-six runs, with exact stdout,
stderr and exit status checked every time. All remain within the 5% budget;
no allocation or whole-decompiler speed improvement is claimed.

All nine gates pass: 7,461 workspace tests, 38 existing ignores across 439
groups and no warnings; 675/675 upstream and 1,467/1,467 stage assertions
retain parity. All 268 CLI probes pass after the builds finish, along with
42 Python tests, 56 Ghidra tests, 48 XML comparisons, 17 saved CLI comparisons
and eleven browser probes. Spec/catalog checks and strict CLI lint pass.
Frozen diff hash `6b39ea09` remained unchanged through the last terminal
success. Logs use `/tmp/kuna-deslop-redundant-expressions-final-`. The preceding
published checkpoint's CI and CodeQL pass. No baseline or threshold changed.

## Borrowed nonzero-mask worklist and token-pattern construction

Nonzero-mask propagation appends a changed output's descendants directly to
its local worklist. The temporary vector is gone; visit order, duplicate
readers and missing-node handling are unchanged. The initial operation
snapshot and neighboring mutating walks keep their snapshots. A persistent
loop regression checks the fixed point, duplicate-reader order, and unchanged
Varnode and operation flags across three calls. All ten integration tests pass
on the original implementation and after the change.

Actual before/after method bodies and the rebuilt library match 12,288 outcomes
per overflow mode across 4,096 real IR graphs and three retries. Each mode has
11,028 successes and 1,260 panics, with all nonzero masks and node/operation
flags compared, including partial state on malformed dead-reader failures.
This is a method-level model, not a whole-engine equivalence claim.
Three fresh native allocation samples per case agree: empty and straight-line
controls retain 0/2 requests and 0/80 bytes; loop workloads with 0, 8 and 64
extra readers fall from 128/131/134 requests to 3/6/9 and from
1,112/5,408/35,872 bytes to 112/376/2,616. A sixteen-node chain falls from
1,076 requests and 9,624 bytes to 6 requests and 560 bytes. These are cumulative
allocation requests and requested bytes, not peak memory. Graph construction,
warmup and fixed-point/state verification are outside the counter.

Token patterns share the true/boolean constructor, reuse their minimum-length
query and clamp the common-token count once. Obsolete pattern-equation design
commentary is removed. Whole-module models preserve 20,736 public outcomes
per overflow mode: 7,560 successes, 13,176 errors and no panics. Private boundary
adapters compare another 32,768 outcomes per mode, including 3,091 checked
overflow panics; unchecked mode has no panics. All inputs and complete results
match. The actual rebuilt library agrees on all 20,736 public records.

The release build takes 44.73 seconds without warnings. All 3,073 engine units,
ten Varnode integration tests and 378 SLEIGH/compiler tests pass, along with
strict public/private rustdoc and CI's CLI lint check (its existing dependency
warning remains). Engine Clippy retains exactly 208 errors and 91 warnings;
all primary diagnostic messages and source text are unchanged. Four complete
stage transcripts, 36 CLI cases, 286 compiler comparisons and 48 XML outputs
match their saved references. Source proofs cover all five candidate files.
Artifacts use `/tmp/kuna-deslop-nz-worklist.zggtkxdl`.

Initial balanced mask timings use thirty measured samples per version. The
empty control slows 5.82% (paired 5.41%); a larger ninety-sample-per-version
run reproduces +5.90% (paired +5.71%), about 6.83 to 7.23 nanoseconds per call.
That adverse result is retained, not classified as noise. The same larger run
improves straight-line, zero/eight/sixty-four-reader loops and the chain by
3.47%/48.91%/22.74%/20.92%/52.65%. Keeping the simpler borrowed traversal is a
tradeoff: fewer allocations and faster nonempty cases, with a small absolute
cost in the empty microbenchmark. No universal speedup is claimed.

Token-pattern timings use thirty measured samples per version after three
warmup pairs: aggregate -0.56% (paired -0.53%), with all six configurations
between -1.06% and +0.98%. Every run verifies its 124,416 concatenation results.
Toy, x86-64 and Hexagon compiler wall time changes -1.02%/-1.79%/+0.78%, with
CPU time -1.14%/-1.79%/-0.61%; every generated SLA hash matches. Toy uses 110
measured samples per version; the larger compilers use twelve. Four complete
call-push, argument-guard, array-cover and stack-alias workloads change
-0.89%/-0.88%/-0.98%/-0.91% (paired -0.83%/-0.90%/-1.08%/-0.88%), with thirty
measured samples per version and exact streams/status on every run. These
whole-workload timings remain within the 5% budget. Native/stage runs use
CPU 41; compiler runs use CPU 40. Timing and correctness logs are retained.

All nine gates pass: 7,462 workspace tests, 38 existing ignores across 439
groups and no warnings; upstream 675/675 and stages 1,467/1,467 retain parity.
The isolated CLI run passes all 268 probes on its first try. The 42 Python
tests, 56 Ghidra tests, 48 XML comparisons, 17 saved CLI comparisons and eleven
browser probes pass, as do spec/catalog checks and strict CLI lint. Frozen
diff `161d7da6` remained unchanged through the last terminal success. Logs use
`/tmp/kuna-deslop-nz-worklist-final-`. The preceding commit's CI and CodeQL
pass. No baseline or threshold changed.

## Native integer queries and wide division

Four public bit queries now use Rust's integer primitives, retaining the
zero sentinels. Two-limb division keeps its narrow wrapper and moves wide
arithmetic to a private `u128` helper, removing digit splitting, normalization
and hand-written long division. The non-inlined wide helper preserves the
narrow path's isolation; earlier variants with worse narrow-call timings were
rejected. Numeric and address-space comments describe implemented interfaces
and ownership, and one broken method link is corrected. All six production
Rust files exactly match the reviewed implementation.

Complete actual-root division modules and verbatim bit-helper bodies agree on
4,211,522 bit-query inputs and 211,460 division outcomes per overflow mode.
Division preserves 211,078 successes, 192 errors and 190 panics, including error
and panic text and all output limbs. The rebuilt native library separately
matches the original bit routines and complete original Knuth-division module.
The harness's additional `u128` check is not independent evidence for the new
`u128` implementation. Two persistent regressions preserve narrow zero-divide
panics, wide zero-divide errors, untouched outputs on failure, and complete
output writes for zero, equal and smaller numerators. All 295 base/numeric
tests pass before and after, with three existing ignores across 23 groups.

The release build takes 48.92 seconds without warnings. The 295 numeric/base,
3,073 engine and 378 SLEIGH/compiler tests all pass: 3,746 total, three ignores
across 58 groups, no warnings. Base/numeric Clippy remains clean and passes
with warnings denied. Public and private rustdoc pass with broken links denied;
four existing private-link warnings remain. A stricter baseline had five link
errors, of which this patch fixes the unresolved `overlap_join` link. CI's CLI
lint passes with its existing SLEIGH dependency warning. Four complete stage
transcripts, 36 CLI comparisons, 286 compiler comparisons and 48 XML outputs
are unchanged. Artifacts use `/tmp/kuna-deslop-native-integers.3gJLxMUw`.

Balanced CPU-pinned native timings use thirty measured samples per version
after three warmup pairs. Aggregate median improves 63.05% (paired 63.03%):
wide division 76.59%, division by a 32-bit value 53.20%, smaller-numerator
handling 24.74%, and narrow division 18.75%. The four bit queries improve
40.51% to 78.64%. Each run checks all eight digests across 1,048,576 calls;
fixture construction and expected results are outside the timer. These are
numeric microbenchmarks, not a whole-decompiler speedup.

Toy/x86-64/Hexagon compiler wall times change +0.05%/-0.57%/-0.003%, with CPU
times -0.04%/-0.58%/+0.06%; every SLA hash matches. Toy has 110 measured samples
per version and the larger compilers twelve. Call-push, argument-guard,
array-cover and stack-alias workloads change +0.39%/+0.73%/+0.86%/+1.10%
(paired +0.59%/+0.95%/+0.84%/+0.75%), with thirty measured samples per version
and exact streams/status in every run. All whole workloads remain within the
5% budget. Numeric/stage runs use CPU 41; compiler runs use CPU 40.

All nine gates pass: 7,464 workspace tests, 38 existing ignores across 439
groups and no warnings; upstream 675/675 and stages 1,467/1,467 retain parity.
All 268 CLI probes pass in the isolated final run, along with 42 Python tests,
56 Ghidra tests, 48 XML outputs, 17 saved CLI comparisons and eleven browser
probes. Spec/catalog checks and strict CLI lint pass. Strict base/numeric
Clippy is also clean on CI's Rust 1.98.1. Frozen diff `dd5e6c5f` remained
unchanged through the last terminal success. Logs use
`/tmp/kuna-deslop-native-integers-final-`. The preceding commit's CI and CodeQL
pass. No baseline or threshold changed.

## Current ownership and error contracts

Seven Rust files lose 296 comment lines without changing any noncomment line,
API or error string. Base error, marshal and address-space documentation now
describes the implemented conversions, registry, borrowed services and shared
identity. Engine context and function-data headers describe their actual
owners and live interfaces instead of completed port waves or obsolete stub
restrictions. Genuine unreviewed limitations remain documented. Private
helpers are named without promising a public documentation target.

Base public and private documentation now build with warnings denied. Engine
public broken-link errors fall from 181 to 161, and private-documentation
errors from 267 to 241. Both modes also lose two private-link warnings. Every
other diagnostic message/file tuple is unchanged; engine documentation still
fails on its remaining debt. Engine Clippy likewise retains the same 208
errors and 91 warnings, including their primary source text. Strict base and
numeric Clippy passes on both Rust 1.90 and CI's 1.98.1; CLI lint also passes.

The release build takes 48.73 seconds without warnings. All 3,746 focused
tests pass, with three existing ignores across 58 groups and no warnings.
Four complete stage transcripts, 36 CLI cases, 286 compiler comparisons and
48 XML outputs match their saved references. Source proofs pin all eight
candidate files and verify every noncomment Rust line. Artifacts use
`/tmp/kuna-deslop-engine-contracts.R8txKdoO`. This is a documentation-only
change; no runtime speed improvement is claimed.

All nine gates pass: 7,464 workspace tests, 38 existing ignores across 439
groups and no warnings; upstream 675/675 and stages 1,467/1,467 retain parity.
The isolated CLI run passes all 268 probes on its first try. The 42 Python
tests, 56 Ghidra tests, 48 XML comparisons, 17 saved CLI comparisons and eleven
browser probes pass, as do spec/catalog checks and strict CLI lint. Frozen
diff `0d79c382` remained unchanged through the last terminal success. Logs use
`/tmp/kuna-deslop-engine-contracts-final-`. The preceding commit's CI and CodeQL
pass. No baseline or threshold changed.

## Native two-limb arithmetic

Two-limb unsigned comparison, addition and subtraction now use native `u128`
operations through two private little-endian conversion helpers. Wide division
shares those conversions. The generic limb loops and their wrapping-trait
import are removed, reducing the module by 33 lines. Public signatures,
narrow division, shifts and all existing tests are unchanged. The spec records
the representation and wrapping semantics.

Complete actual-root modules agree on 2,259,081 operand pairs in each overflow
mode, checking both comparisons and every output limb for addition and
subtraction against the original loops. Another 324,617 division cases retain
324,108 successes, 319 errors and 190 panics, with matching failure payloads
and output arrays. This division reference already uses native `u128`; it
checks conversion and control-flow preservation, not an independent division
algorithm. The rebuilt native library independently matches the complete
original module and verifies all twelve benchmark digests.

The release build takes 49.77 seconds without warnings. All 3,746 focused
tests pass, with three existing ignores across 58 groups and no warnings.
Strict base/numeric Clippy passes on Rust 1.90 and CI's 1.98.1, and CLI lint
passes. Public and private base/numeric documentation pass with broken links
denied; the two existing opcode private-link warnings remain. Four complete
stage transcripts, 36 CLI cases, 286 compiler comparisons and 48 XML outputs
match their saved references. Source proofs cover all three candidates.
Artifacts use `/tmp/kuna-deslop-limb-ops.appsMtzT`.

Balanced native timings use thirty measured samples per version after three
warmup pairs, with 1,572,864 calls and all twelve digests checked per run.
Aggregate median changes -0.87% (paired -0.78%); individual medians range
from -14.09% to +0.38%, with paired changes from -7.96% to +0.91%. All remain
within the 5% budget. Fixtures and reference evaluation stay outside timers.

Toy/x86-64/Hexagon compiler wall times change -1.45%/-0.33%/-0.19%, with CPU
times -1.67%/-0.34%/-0.20%; every generated SLA hash matches. Toy uses 110
measured samples per version and the larger compilers twelve. Four complete
call-push, argument-guard, array-cover and stack-alias workloads change
-0.61%/-1.12%/-0.93%/-0.99% (paired -0.58%/-0.98%/-1.02%/-1.01%), with thirty
samples per version and exact streams/status in every run. Compiler runs use
CPU 40, native/stage runs CPU 41. No general decompiler speedup is claimed.

All nine gates pass: 7,464 workspace tests, 38 existing ignores across 439
groups and no warnings; upstream 675/675 and stages 1,467/1,467 retain parity.
The isolated CLI run passes all 268 probes on its first try. The 42 Python
tests, 56 Ghidra tests, 48 XML comparisons, 17 saved CLI comparisons and eleven
browser probes pass, as do spec/catalog checks and strict CLI lint. Frozen
diff `9ab48a3e` remained unchanged through the last terminal success. Logs use
`/tmp/kuna-deslop-limb-ops-final-`. The preceding commit's CI and CodeQL pass.
No baseline or threshold changed.

## Opcode contracts and a rejected lookup rewrite

Opcode documentation now states the current names, SLEIGH aliases,
case-sensitive lookup, reserved names, sentinel behavior and marshaling
contracts. It removes 47 comment lines and two public-to-private links.
Every noncomment Rust line is identical to the preceding commit, including
the original signed-bound lookup and its tests. The spec describes that
existing behavior. Public and private base/numeric documentation now build
with all warnings denied; the strict baseline failed on two private links.

A one-ordering-comparison lookup rewrite passed the complete-module models
and rebuilt-library comparisons but was rejected on performance. In 66
balanced CPU-pinned runs, its aggregate median increased 9.45% (paired 9.41%).
All six distributions exceeded 5%, ranging from +5.99% to +14.68%. Near-unchanged
whole workloads did not justify retaining that regression. The original
lookup body and test comment were restored exactly, and the unpublished
implementation note was removed. All rejected sources, executable/library
hashes, correctness checks and raw timings are preserved under
`/tmp/kuna-deslop-opcode-order.GueRJzXe/rejected-ordering`.

The retained documentation-only version rebuilds in 52.80 seconds without
warnings. All 3,746 focused tests pass, with three existing ignores across
58 groups and no warnings. Actual-root modules agree on 1,215,788 names in
each overflow mode; the rebuilt library independently matches the complete
original module. The 36 CLI cases, four full stage transcripts, 286 compiler
comparisons and 48 XML outputs are unchanged. Strict base/numeric Clippy
passes on Rust 1.90 and CI's 1.98.1, and CLI lint passes. A comparison rerun
initially hit an existing output directory; its failure log is retained, and
the fresh-directory comparison passes all cases.

The restored lookup's 66-run timing check changes aggregate median +0.51%
(paired +0.25%), with all six distributions between -0.32% and +0.63%.
Thirty measured samples per version follow three warmup pairs on CPU 41;
all six digests match in every 786,432-call run. This verifies removal of the
measured regression, not a speedup. Retained evidence is under
`/tmp/kuna-deslop-opcode-order.GueRJzXe`.

All nine final checks pass with the retained source frozen through the last
CLI result: 7,464 workspace tests, 38 existing ignores across 439 groups and
no warnings; 675 upstream and 1,467 stage assertions; 268 CLI probes; Ghidra,
static/spec/catalog/lint, compatibility, XML and rebuilt browser checks.
The preceding commit's first CI attempt failed before the browser app loaded:
Chrome did not open its DevTools port within 10 seconds. All preceding checks
passed, and the unchanged failed-job rerun passed. The startup cause remains
unresolved; neither its timeout nor the application was changed here.

## Foundation contracts and persistent library lint

Address and integer-helper documentation now describes implemented register
decoding, address/sequence/range comparison constraints, modulo-width shifts
and division behavior. The complement contract explicitly preserves the
caller's reorder flag for undefined complements. These three production files
lose 64 comment lines; every noncomment, nonblank Rust line is unchanged.
One new regression checks four undefined opcodes with both initial flag values
and passes against the original implementation.

`make lint-base-num` now runs strict release Clippy on the base and numeric
libraries, and CI requires it alongside the existing CLI check. The target
passes on both Rust 1.90 and CI's 1.98.1. It does not claim that the engine,
the whole workspace or test targets are warning-clean. Public and private
base/numeric rustdoc also pass with every warning denied.

Two shorter complement implementations were rejected. Selecting the result
and flag together slowed unsupported-opcode lookups 40.09% (paired 39.54%);
a separate flag predicate still slowed them 13.34% (paired 11.18%). Both
passed all 148 opcode/flag outcomes. The original body was restored exactly,
and the unpublished implementation note was removed. Full sources, binaries,
hashes, correctness results and raw timings remain in `rejected-tuple` and
`rejected-separate-flag` under `/tmp/kuna-deslop-complement-map.3HY5DvKy`.

The retained version rebuilds in 47.38 seconds without warnings. All 3,747
focused tests pass, with three existing ignores across 59 groups and no
warnings. Complete actual-root modules agree on all 148 opcode/flag outcomes
in each overflow mode; the rebuilt library separately matches the original.
The 36 CLI cases, four complete stage transcripts, 286 compiler comparisons
and 48 XML outputs are unchanged.

The restored code's initial 66-run check had an unsupported-case median
increase of 7.20% but a paired decrease of 0.70%. A longer 190-run diagnostic,
with 90 measured samples per version after five warmup pairs on CPU 41,
changes aggregate median +0.09% (paired -0.04%). All five distributions range
from -0.19% to +3.06%; unsupported lookups change -0.19% (paired -0.63%).
Each run checks all five digests across 655,360 calls. Both sample sets are
retained. This checks removal of the rejected regressions, not a speedup.

All nine final gates pass with the source frozen through the last CLI result:
7,465 workspace tests, 38 existing ignores across 440 groups and no warnings;
675 upstream and 1,467 stage assertions; 268 CLI probes; Ghidra, static/spec/
catalog/both lint targets, compatibility, XML and rebuilt browser checks.
The previous checkpoint also passed CI and CodeQL.

## Audit still open

These are investigation targets, not a claim that the repository review is done.

| Area | Evidence / next check |
|---|---|
| CLI test structure | Private module copies and missing-command/spec skips are removed. JSON helpers use explicit field paths and preserve raw bytes. Compiler probes reject broken tools, and required native runs check both status and output. The two fixture-launch false skips now retain spelling checks on non-native hosts. Other platform gates and conditional assertions still need review. |
| Option plumbing | Loader options still use process-wide environment variables; inspect the loader API before replacing ambient configuration. The 12 matching boolean readers now share one parser; distinct vocabularies remain intentional. |
| Parsing and serialization | Standard parsers now back the registry and CLI JSON; typed baseline validation rejects false-green inputs. Review remaining command-specific JSON extraction and serialization boundaries. |
| CLI responsibilities | The worker codec is isolated and byte-pinned; graph queries, scheduling, synthesized-structure replay, object-file views, console scripts, string filtering, callee-first feedback, query function metadata and archive ingestion have separate owners. Archive member and console output files have scoped cleanup. Loading/configuration and the remaining pool module still combine several lifecycle policies. |
| Collection policy | The CLI and base/numeric libraries have strict warning-clean Clippy gates. The release engine-library check last reported 208 collection-policy errors/91 warnings, and analysis 215 errors/64 warnings. Reviewed lookup-only collections and explicitly ordered reports preserve existing implementations where iteration cannot affect output. Review iteration semantics and lookup costs before replacing other collections, then check the remaining crates and extend enforcement. |
| Engine boundaries | Unique and addressed rule outputs use shared factories, and output reassignment uses shared scope/cover bookkeeping, with high-level/lane-state regressions. Context/function-data ownership headers and base contracts now describe current implementations. Other wave-era STUB notes remain; engine documentation still has 161 public broken-link errors, with 241 when private items are included. |
| Analysis, SLEIGH, Python, integrations | Public and private analysis rustdoc links now resolve; other documentation warnings and stale migration narratives remain. Inventory and ranked-backlog corruption fail closed; the driver distinguishes pauses and errors from empty work. Python writers share atomic publication. Required fixtures fail explicitly in console, SLEIGH and Ghidra tests; conditional assertions and optional-tool coverage still need review. Compiler parity uses an independent oracle. Inherited-stderr exit diagnostics are regression-tested; the earlier real-Chrome startup timeouts remain unexplained. |

Before each commit: `make test`, `make test-stages`, `make rust-test`,
`make check-spec`. Also run the catalog check and relevant CLI probes. Preserve
the single-PR scope across subsequent milestones.
