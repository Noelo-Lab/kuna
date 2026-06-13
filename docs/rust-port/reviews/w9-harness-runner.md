# w9-harness-runner
verdict: ACCEPT-WITH-LOSSES
verifier: claude-opus-4-8[1m] (independent verifier subagent)
date: 2026-06-13
branch: worktree-wf_a7f2562d-46c-15 @ a3d46fe (+ a6d34f2 verifier tests)

gate: `cd rust && cargo test -p kuna-harness` -> ok (48 lib + 3 lift_diff + 15 verify_w5; 0 failed)
clippy: `cargo clippy -p kuna-harness --tests -- -D warnings` -> clean
blob-sha drift: all four C++ modules match checklist cpp_blob_sha exactly (test.cc cd982a4, test.hh adf0e61, testfunction.cc e4f4cd9, testfunction.hh 1210326).

## Scope

C++: test.cc, test.hh, testfunction.cc, testfunction.hh.
Rust: rust/crates/kuna-harness/src/unittest.rs (test.cc/.hh), testfunction.rs +
testfunction/tests.rs (testfunction.cc/.hh), lib.rs (module wiring).

Ported: `UnitTest`/`UnitTestSuite` registry + `run`, `add_exit_code`;
`FunctionTestProperty` (restoreXml/startTest/processLine/endTest),
`FunctionTestCollection` (load/restoreXml/buildProgram/runTests/clear/strip/
evaluate), `run_test_files`. NOT ported (boundary, see findings F3): the
`test.cc` `main()` CLI arg parser and `gatherDataTests` (FileManage directory
glob) — no harness binary; the test-file list is supplied by the caller of
`run_test_files`.

## Hunt list

- Signedness: CLEAN. `endTest` compares `count`(uint4) against `min/max`(int4);
  C++ `-Wno-sign-compare` converts int4->uint4, Rust does it explicitly with
  `as uint4` (testfunction.rs:157) — faithful. Corpus min/max are 0..9 (grep:
  max literal is "9"), so the conversion is exact in practice. Every C++
  `for(int4 i=0;i<...size();++i)` (stripNewlines, restoreXml pattern walk,
  runTestFiles, gatherDataTests) is a Rust `usize`/iterator — no signed loop
  index survives.
- Integer widths: CLEAN. int4=i32, uint4=u32 (kuna_base::types); `add_exit_code`
  on `int`=i32. No promotion-into-wider-type sites.
- Wrapping: CLEAN (corpus). `add_exit_code` uses `wrapping_add` to reproduce the
  C++ signed-int overflow that the `ret < current` guard exists to catch
  (unittest.rs:194) — correct, a `checked_add` here would be WRONG (the guard
  needs the wrapped value). `istream_read_int` accumulates in i64 with
  saturating ops then `as int4` — see F2 (latent, corpus-unreachable).
- Comparator totality: N/A — no `operator<`/comparator in scope.
- Iteration-order provenance: CLEAN. `testList` (C++ `list`, insertion order) ->
  `Vec` push order; `commands` (C++ `vector`) -> `Vec`; `<script>`/`<stringmatch>`
  children and file list preserve document/argument order. No HashMap/HashSet
  anywhere (grep: none).
- Off-by-one / do-while / reverse iterators: CLEAN. The `restoreXml` C++
  `do{}while(pos!=npos)` -> Rust `loop{...; match next { None=>break, Some=>..}}`
  keeps at-least-once + the leading-ws-skip-then-break and the npos/substr(pos,n)
  vs substr(pos) tail split (testfunction.rs:185-209). The `runTests` bulk-line
  slicer reproduces the `find_first_of('\n')` walk AND the `prevpos!=size` final
  no-newline tail (testfunction.rs:550-562). Verified by adversarial tests
  w9_trailing_whitespace_segment_emits_no_empty_regex and
  w9_bulkout_final_line_is_fed_to_matcher.
- Erase-while-iterating: N/A — no in-traversal container mutation in scope.
- Exception->Result partial-state parity: CLEAN. C++ throws IfaceParseError/
  IfaceExecutionError mid-`restoreXml`, leaving `commands`/`testList` partially
  populated; the next file's `clear()` resets. Rust returns `Err` mid-walk with
  identical partial state, and `run_test_files` calls `clear()` at the top of
  each iteration before `load_test` — same sequencing as the C++
  `clear();loadTest()` in the try. The TestError From<IfaceError> split
  (is_execution -> Execution else Parse) maps the two C++ catch arms to the two
  file-error prefixes faithfully.

## Console-grammar re-derivation (this wave's focus)

- Unit summary (stderr stream): every line shape `run_tests.py` greps is
  byte-faithful — `testing : <name> ...`, `  passed.`, `  fail: <msg>`,
  bare `  fail` (catch-all arm), `==============================`,
  `<passed>/<total> tests passed.` (unittest.rs:157-180). Matches _UNIT_OPEN/
  _UNIT_PASS/_UNIT_FAIL/_UNIT_FOOT.
- Datatest grammar (stdout stream): `Success -- <name>` / `FAIL -- <name>`
  (testfunction.rs:483-491), `Total tests applied = N` / `Total passing tests
  = N` (col 0), the `Failures: ` header (trailing space, :631) capped at 10
  (.take(10)). Matches _DATA_PASS/_DATA_FAIL/_DATA_APPLIED/_DATA_PASSING.
- File-error grammar: `Error parsing <f>: <msg>` / `Error executing <f>: <msg>`
  at col 0 (run_test_files), `Error: Did not apply tests in <f>` (runTests
  error path), and the indented `Execution failed for <f>` / `No output for <f>`
  in the capped Failures summary. Every form names the *.xml file, satisfying
  _DATA_ERR's `.xml`-suffix requirement.
- stringmatch counting (min/max, multi-line sequential): the processLine state
  machine — advance patnum on match, count+rewind on full pattern, and the C++
  quirk where a mid-pattern non-match aborts AND immediately re-tests the SAME
  line against pattern[0] — is transcribed line-for-line (testfunction.rs:140-153)
  and pinned by w9_multiline_abort_restart_salvages_overlapping_first_line.
- Failures cap: `for(i=0;i<10;++i)` -> `.take(10)`; pinned by the existing
  failures_summary_caps_at_ten and re-checked.

## Mechanical pass

- grep todo!/unimplemented!/HashMap/HashSet/sort_unstable in the item's src: NONE.
- bare `as` casts: all are width/sign conversions on int4/uint4/usize/char/i64
  that faithfully reproduce a C++ implicit conversion (patnum as usize index,
  min/max as uint4 for the signed-vs-unsigned compare, len as int4 for the
  C++ `int4 numCommands()`); none is an unjustified narrowing.
- clippy -p kuna-harness --tests -D warnings: clean.

## Findings

- F1 (informational, not a defect): physical stdout/stderr routing is deferred.
  `UnitTestSuite::run` and `run_test_files` write to a `&mut String`, not to
  cerr/cout, because no harness binary (`test.cc` `main`) is ported. The two
  grammars are line-shape-distinct and byte-faithful, so a future binary can
  route unit->stderr, data->stdout and `run_tests.py` will parse unchanged. The
  checklist note ("reproduce the stdout/stderr stream split") is met at the
  grammar level; physical routing is the binary item's job.
       cpp: decompiler/cpp/test.cc:162-170 (cerr unit / cout data)
       rust: rust/crates/kuna-harness/src/unittest.rs:139, testfunction.rs:594
- F2 (minor, corpus-unreachable): `istream_read_int` saturates in i64 then
  `(sign*val) as int4`, so a min/max attribute > i32::MAX truncates rather than
  clamping to INT_MAX+failbit as C++ `istringstream >> int4` does. Corpus
  min/max are single-digit (grep: max literal "9"), so this path is never taken;
  flagged for the record only.
       cpp: decompiler/cpp/testfunction.cc:56-59 (istringstream >> int)
       rust: rust/crates/kuna-harness/src/testfunction.rs:665-688
- F3 (informational): `gatherDataTests` (FileManage `.xml` glob + the
  `for(int4 i; i<allTestFiles.size())` filter) and the `main()` arg parser
  (`-path`/`-sleighpath`/`-usesleighenv`/`unittests`/`datatests` with the
  break-into-rest-of-argv name capture) are not ported. File discovery is
  FileManage territory (LOSS-011, ported separately) and CLI parsing is the
  binary's; the runner core (`run_test_files`) takes a pre-built file list, which
  is the correct seam. No behavior the corpus exercises is lost.
       cpp: decompiler/cpp/test.cc:56-81 (gatherDataTests), :103-174 (main)
       rust: (intentionally absent)

Edge note (not a finding): an all-whitespace `<stringmatch>` body yields an
empty `pattern` vec; C++ `processLine` would then index `pattern[0]` (UB) and
Rust would panic. The corpus has no empty stringmatch bodies (grep confirmed),
so neither engine reaches it; Rust's panic is the safer failure.

## LOSS-007 corpus check

The declared loss (std::regex -> regex crate) is corpus-safe: grepping every
single-line `<stringmatch>` body for backreferences/lookaround returns only
literal `\0` / escaped-backslash patterns (`'\\0'`, `'\\0'\)`), which are
output-literal escapes the regex crate compiles identically — no `\1`-style
backreference and no `(?=...)`/`(?<...)` lookaround in the corpus. The match
decisions agree. LOSS-007 stands; its rust-anchor was "none yet" — now
realized at rust/crates/kuna-harness/src/testfunction.rs:113,168-209,245-249.

## Adversarial tests (committed a6d34f2)

unittest.rs:
- w9_add_exit_code_overflow_and_boundary — i32::MAX+i32::MAX wrap, ret==255
  kept vs 256 clamped, the `ret<current` term firing on a negative `add`
  (add_exit_code(10,-3)==255, faithful to the C++ guard not distinguishing
  wrap from decrease), i32::MIN current.
- w9_filter_uses_registration_order_not_set_order — the `set<string>` filter is
  membership-only; output follows vector registration order (zebra before
  alpha) and a non-existent filter name is silently ignored.
testfunction/tests.rs:
- w9_endtest_inclusive_boundary_count — [min,max] inclusive at count==min/max
  and just outside.
- w9_endtest_zero_max_negative_assertion — the corpus min=max=0 "must be absent"
  form at the signedness-sensitive zero boundary.
- w9_multiline_abort_restart_salvages_overlapping_first_line — the abort-then-
  re-test-same-line quirk under an overlapping first line, plus the contrast
  where no salvage line exists.
- w9_bulkout_final_line_is_fed_to_matcher — the final newline-terminated line is
  fed exactly once (no double-feed from the tail branch).
- w9_trailing_whitespace_segment_emits_no_empty_regex — a `\n`+all-ws tail does
  not add an empty trailing pattern.

losses: LOSS-007 (std::regex -> regex crate; corpus-proven match-decision parity
above).
