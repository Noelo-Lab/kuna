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

## Audit still open

These are investigation targets, not a claim that the repository review is done.

| Area | Evidence / next check |
|---|---|
| CLI test structure | Private module copies and missing-command/spec skips are removed. JSON helpers use explicit field paths and preserve raw bytes. Compiler probes reject broken tools, and required native runs check both status and output. The two fixture-launch false skips now retain spelling checks on non-native hosts. Other platform gates and conditional assertions still need review. |
| Option plumbing | Loader options still use process-wide environment variables; inspect the loader API before replacing ambient configuration. The 12 matching boolean readers now share one parser; distinct vocabularies remain intentional. |
| Parsing and serialization | Standard parsers now back the registry and CLI JSON; typed baseline validation rejects false-green inputs. Review remaining command-specific JSON extraction and serialization boundaries. |
| CLI responsibilities | The worker codec is isolated and byte-pinned; graph queries, scheduling and object-file views now have separate owners. `decompile_all.rs` and the remaining pool module still combine several lifecycle policies; review the next meaningful ownership boundary. |
| Collection policy | The release engine-library Clippy check still reports 211 collection-policy errors; the CLI-only check finds 17 errors and 22 warnings. Declaration naming, rendered-signature dedup, profiling and worker headers use reviewed lookup-only collections or explicitly sorted reports. Replay rename provenance no longer needs a separate set. Review iteration semantics and lookup costs before replacing other collections, then check the remaining crates and enforce the gate. |
| Engine boundaries | `kuna_addcarrychain`, `kuna_arraystride`, `ruleaction_3`, and `ruleaction_4` duplicate `new_unique_out`. The real method additionally assigns high variables and checks register lanes, so replacing these requires behavioral tests. Wave-era STUB notes remain; schedule and SLEIGH overview claims now distinguish implemented code from real limitations. |
| Analysis, SLEIGH, Python, integrations | Broader review remains open. Inventory and ranked-backlog corruption fail closed; the driver distinguishes pauses and errors from empty work. Python writers share atomic publication. Required fixtures fail explicitly in console, SLEIGH and Ghidra tests; conditional assertions and optional-tool coverage still need review. Compiler parity uses an independent oracle. |

Before each commit: `make test`, `make test-stages`, `make rust-test`,
`make check-spec`. Also run the catalog check and relevant CLI probes. Preserve
the single-PR scope across subsequent milestones.
