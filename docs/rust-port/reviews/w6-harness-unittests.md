# w6-harness-unittests

## Round 1

verdict: ACCEPT-WITH-LOSSES
verifier: independent verifier agent (Claude, not the porter)
date: 2026-06-13
branch: worktree-wf_91be7ffe-767-21 (worktree `.claude/worktrees/wf_91be7ffe-767-21`)
  - port commit: 626bd61 (the 4 test files, +1580 lines, nothing else)
  - verifier adversarial tests committed: 516720e
cpp blob shas: all four in-scope files match the checklist entry exactly
(`git hash-object` in the MAIN tree):
  - testfuncproto.cc  963aae19c755ae79322294d21fdf1880e23bfd02 ✓
  - testparamstore.cc 1c1ef1f4b71beeaf89201ae57c9cb9c79fb1a929 ✓
  - testtypes.cc      eb41872baba597550d5cf07a0abdc37679d16a4b ✓
  - testcirclerange.cc e8a19937a8c91d125a62ba879fddf49e8eb9c5ec ✓
  No blob-sha drift; the port verified against the recorded source.

gate (verifier-prompt form): `cd rust && cargo test -p kuna-decomp --test
testfuncproto --test testparamstore --test testtypes --test testcirclerange`
-> **PASS**:
  - testcirclerange: 117 passed / 0 failed / 0 ignored
  - testfuncproto:   0 passed / 0 failed / 21 ignored
  - testparamstore:  0 passed / 0 failed / 4 ignored
  - testtypes:       2 passed / 0 failed / 6 ignored
Re-run with the verifier suite added: verify_w6_harness_unittests 5/5.
clippy: `cargo clippy -p kuna-decomp --test {each of the 4 + verify}` -> clean
on every in-scope target (the only `-D warnings` failure in the crate is a
pre-existing `clippy::overly_complex_bool_expr` in
`src/heritage.rs:1602`, an inline lib test arriving via a merged branch — NOT
touched by the item commit 626bd61, which changes only the 4 test files;
out of scope).
port-audit: no `port-audit` crate exists in this worktree (carried gap, same as
prior waves); blob shas verified manually and all four match.

NOTE on the declared gate: the checklist entry declares
`"crate": "kuna-harness"` / `"gate": "cargo test -p kuna-harness"`, which runs
**0** of this item's tests (vacuously green — kuna-harness/tests holds only the
lift-diff infra). The substantive gate is the kuna-decomp one above. Recorded
as LOSS-096 (same shape as w1-harness-unittests F1).

## Method

Re-read all four C++ files end to end and the four Rust ports
statement-for-statement; the port commit (626bd61) touches only those four
files. Name-match audit: each file's `TEST(...)` set is reproduced 1:1 under
identical names (verified by `diff` of sorted name lists):
testcirclerange 117/117, testfuncproto 21/21, testparamstore 4/4, testtypes
8/8 — names identical in every file.

Coverage reality (the heart of this item — it is a *test harness* port):
  - testcirclerange: all 117 are REAL, asserting, passing tests. The
    brute-force reference harness `CircleRangeTest` (getMin/getNext element
    enumeration, getStartStopStep reconstruction, std::set_intersection/union)
    is faithfully transcribed.
  - testtypes: 2 REAL (enum_matching, enum_matching2 — exercise the already-
    ported `Datatype::get_matches`); 6 `#[ignore]`d (cast_* / type_ordering,
    blocked on the C grammar + CastStrategy/TypeOpCast).
  - testfuncproto (21) + testparamstore (4): ALL `#[ignore]`d scaffolds,
    blocked on the C/prototype grammar + a spec-loaded Architecture/Translate.
  29 of the 33 non-circlerange tests assert nothing -> LOSS-095. Verified the
  blockers are genuine (grep over rust/crates/*/src): `parse_type`/
  `parse_protopieces`/`parse_C` grammar = 0 hits; `CastStrategy`/
  `markExplicitLongSize`/`getInputCast`/`getCastStrategy` = 0 hits. The
  scaffolds are honest (`#[ignore = "blocked: ..."]`, no fabricated pass) and
  do not mask a regression in already-ported code (the ported
  `assign_parameter_storage`/`derive_input_map` are tested by the w6-s4-fspec
  items, not here).

## Mandatory hunt list

(in-scope = the harness reproduction in the 4 test files; CircleRange itself
is the dependency item w6-s5-rangeutil and is out of scope — its faithfulness
is independently re-confirmed by my adversarial round-trip oracle.)

- **Signedness**: clean. `getStartStopStep`'s mixed int4/uintb ops mirror the
  C++: `step`/`bigpos`/counts are int4, gaps/elements are uintb; the int4↔uintb
  conversions (`*step as uintb`, `diff as int4`) reproduce the C++ implicit
  promotions. No int4-index-vs-size comparison hazard (loops use `1..len()`).
- **Integer widths**: clean. `bytes` int4, `mask`/`elements` uintb (u64),
  matching `int4`/`uintb`. `from_range`'s byte derivation (`-1; >>=1; +=1; /8`)
  is verbatim.
- **Wrapping**: clean. Every `uintb` arithmetic op uses the `Wrap` helpers
  (`wadd`/`wsub`): `mask.wadd(1)` (C++ `mask+1`, which overflows to 0 for the
  full-64-bit mask -> the `bytes=8` arm), `elements[0].wsub(elements[1])`,
  `last().wadd(step)`, `tmp.wsub(mask.wadd(1))` — each a C++ silent-wrap site.
  No `checked_*` that should wrap or vice versa.
- **Comparator totality**: clean. `set_intersection`/`set_union` and
  `sort_unstable` operate on `uintb` (total `Ord`); no float comparators.
  `sort_unstable` matches C++ `std::sort` and is safe here (equal u64s are
  indistinguishable, so instability is unobservable). get_matches' BTreeMap
  ordering = C++ `std::map<uintb,string>` ordering (out-of-scope ported fn).
- **Iteration-order provenance**: clean. testcirclerange harness loops are
  `vector` index walks (C++ `for(i=0;i<size();++i)`) -> `iter()`/`1..len()`;
  the enum namemap is a `BTreeMap` (= C++ ordered `std::map`), so
  enum_matching's ordered `match_name` output is reproduced. No HashMap/HashSet
  anywhere in the item (grep: 0 hits).
- **Off-by-one / do-while / reverse iterators**: clean. The C++ `do { push }
  while(getNext)` (testcirclerange.cc:164-166) is ported as `loop { push; if
  !get_next { break } }` — at-least-once semantics preserved. dedup walk
  `1..len()` matches `for(i=1;...)`. `bigpos`/`bigpos-1` indexing matches.
- **Erase-while-iterating**: n/a — the harness rebuilds vectors (`elements =
  res`, `swap(dedup)`), no in-place erase during traversal.
- **Exception -> Result parity**: clean. The C++ `catch(EvaluationError&)`
  "output not in range" skip in pullbackUnary/pullbackBinary is ported as
  `Err(KunaError::Evaluation{..}) => {}` (skip) with any OTHER `Err` re-raised
  via `panic!` — i.e. the port refuses to silently swallow a non-Evaluation
  error, which is stricter-but-faithful (the C++ only ever catches
  EvaluationError; any other throw would propagate/abort the test too).
  push_unary/push_binary's `.unwrap()` matches the C++ non-try evaluate path.

This wave's extra hunts:
- **Datatype::compare/compareDependency total order**: the type_ordering TEST
  that would exercise it is `#[ignore]`d (LOSS-095); not realized here. The
  comparator itself is an earlier wave's item.
- **ParamEntry/ProtoModel sort orders**: the funcproto/paramstore TESTs that
  would exercise `assign_parameter_storage`/`derive_input_map` orderings are
  `#[ignore]`d (LOSS-095); the core is covered by the w6-s4-fspec items.
- **TypeFactory cache determinism**: not reached (no TypeFactory in the 2 real
  type tests — `make_enum` builds the enum directly).

## Mechanical pass

- greps over the 4 ported test files: no `todo!`/`unimplemented!`/`HashMap`/
  `HashSet`. `sort_unstable` x5 in testcirclerange = C++ `std::sort`, safe on
  total-ordered u64 (justified above). Bare `as` casts are all in test code and
  each mirrors a C++ implicit conversion (`opcode as usize` array index;
  `diff/biggest{1,2} as int4`; `i/bigpos as int4/usize`; `*step as uintb`
  promotion) — faithful, and the bare-`as`-justification rule targets library
  code, not test harnesses.
- clippy clean on all in-scope targets.

## Adversarial pass

5 new tests at rust/crates/kuna-decomp/tests/verify_w6_harness_unittests.rs
(committed 516720e), all passing:
  - `w6_harness_unittests_roundtrip_wrapping_and_step` — INDEPENDENT third
    oracle: re-derives `getStartStopStep` straight from the C++ source (not the
    porter's Rust) with explicit signed/unsigned/wrap handling, brute-forces a
    CircleRange's elements through the public API, and asserts the reference
    reconstruction round-trips to (getMin, getEnd, getStep). Covers the
    wrap-past-0, big-step, 8-byte, single-element, and 2-element-power-of-two
    arms drawn from the in-scope TEST inputs. This catches a porter
    harness/impl co-bug that a green TEST alone could not.
  - `w6_harness_unittests_roundtrip_full_byte_step_then_wrap` — the
    `tmp -= (mask+1)` wrap-closure arm and the `biggest2==0` uniform-step arm,
    well-formed (left<=mask) wrapping ranges. (During authoring this test first
    failed on a MALFORMED input `left=0xffe0` with byte mask — confirming the
    port CORRECTLY rejects out-of-domain ranges via the `back() > mask` guard;
    fixed the test input, not the port.)
  - `w6_harness_unittests_multiset_ops` — pins std::set_intersection/set_union
    multiset duplicate-multiplicity (min/max run lengths), disjoint->empty, and
    empty-operand cases the harness relies on.
  - `w6_harness_unittests_enum_zero_and_complement_boundary` — make_enum +
    get_matches on OR-of-flags (3, 0xb), full-complement (~0 -> ZERO
    complement), and unrepresentable (0x10) values the two ported TESTs skip;
    asserts most-significant-first match ordering.
  - `w6_harness_unittests_enum_size_changes_complement_mask` — proves
    make_enum's `size` argument actually feeds get_matches' complement mask
    (size-1 0xfb -> FOUR complement; size-8 same value must NOT match that),
    i.e. the helper isn't silently defaulting the enum size.

## Findings

- F1 (minor, -> LOSS-095): 29 of 33 non-circlerange tests are `#[ignore]`d
  name-match scaffolds with zero behavioral coverage. Blockers verified
  genuine; scaffolds honest; no regression masked. Accepted as a cited
  deferral.
     cpp: decompiler/cpp/unittests/testfuncproto.cc (all), testparamstore.cc
          (all), testtypes.cc:121-213 (cast_*/type_ordering)
     rust: rust/crates/kuna-decomp/tests/testfuncproto.rs,
           testparamstore.rs, testtypes.rs (the `#[ignore]`d fns)
- F2 (minor, -> LOSS-096): tests live in kuna-decomp/tests, not the
  checklist-declared kuna-harness crate, so the checklist-declared gate is
  vacuous. Placement is correct (tests need kuna-decomp internals); the fix is
  a checklist gate-metadata correction.
     cpp: decompiler/cpp/unittests/testcirclerange.cc (+3 others)
     rust: rust/crates/kuna-decomp/tests/testcirclerange.rs (+3 others)

No blocker, no major. The 117 real circlerange tests + 2 real enum tests are
faithful (independently re-confirmed) and pass; the deferrals are honest and
cited.

losses: LOSS-095 (29 ignored scaffolds), LOSS-096 (gate-crate divergence).
