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

## Audit still open

These are investigation targets, not a claim that the repository review is done.

| Area | Evidence / next check |
|---|---|
| CLI test structure | Private module copies and missing-command/spec skips are removed. JSON helpers now use explicit field paths and preserve raw bytes where required. Duplicate process helpers and conditional assertions still need review. |
| Option plumbing | Loader options still use process-wide environment variables; inspect the loader API before replacing ambient configuration. The 12 matching boolean readers now share one parser; distinct vocabularies remain intentional. |
| Parsing and serialization | Standard parsers now back the registry and CLI JSON; typed baseline validation rejects false-green inputs. Review remaining command-specific JSON extraction and serialization boundaries. |
| CLI responsibilities | The worker codec is isolated and byte-pinned; graph queries, scheduling and object-file views now have separate owners. `decompile_all.rs` and the remaining pool module still combine several lifecycle policies; review the next meaningful ownership boundary. |
| Collection policy | The release engine-library Clippy check still reports 216 collection-policy errors. Declaration naming now uses one reviewed lookup-only table. Review iteration semantics and lookup costs before replacing other collections, then check the remaining crates and enforce the gate. |
| Engine boundaries | `kuna_addcarrychain`, `kuna_arraystride`, `ruleaction_3`, and `ruleaction_4` duplicate `new_unique_out`. The real method additionally assigns high variables and checks register lanes, so replacing these requires behavioral tests. Ninety engine files still contain wave-era STUB notes. |
| Analysis, SLEIGH, Python, integrations | Broader review remains open. Inventory corruption fails closed and Python text writers share atomic publication. Required fixtures fail explicitly in console, SLEIGH and Ghidra tests; conditional assertions and optional-tool coverage still need review. Compiler parity uses an independent oracle. |

Before each commit: `make test`, `make test-stages`, `make rust-test`,
`make check-spec`. Also run the catalog check and relevant CLI probes. Preserve
the single-PR scope across subsequent milestones.
