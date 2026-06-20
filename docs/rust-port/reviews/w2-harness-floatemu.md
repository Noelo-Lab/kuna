# w2-harness-floatemu

verdict: ACCEPT-WITH-LOSSES
verifier: independent verifier agent (round 1; not the porter)
date: 2026-06-11
gate: `cd rust && cargo test -p kuna-num --test testfloatemu` -> ok
      (30 passed, 0 failed, 1 ignored — the #[ignore]d
      double_decimal_precision, ledgered as LOSS-025). Full
      `cargo test -p kuna-num` -> 94 passed / 0 failed / 1 ignored
      (includes the 6 new verifier tests); `cargo test --workspace` ->
      36 suites ok, 0 failed.
      `cargo run -p port-audit -- --item w2-harness-floatemu`: tool still
      does not exist in the workspace (carried gap since
      w1-harness-unittests; skipped). Blob sha re-checked by hand:
      `git hash-object decompiler/unittests/testfloatemu.cc` =
      2571f55f1a41796e342b94074f560f53ac97de76, matching the checklist
      `cpp_blob_sha`.
      clippy: `cargo clippy -p kuna-num -p kuna-base --all-targets --
      -D warnings` -> clean after the F2 one-line comment fix (it failed
      on this item's file as committed; see F2).

## Scope reviewed

C++ in scope: decompiler/unittests/testfloatemu.cc (520 LOC, 31 TESTs).
Rust diff: exactly commit 9047bb0 on worktree-wf_6343a4ba-016-16 —
rust/crates/kuna-num/tests/testfloatemu.rs (+588). The branch also
carries the already-reviewed w2-sleigh-translate/loadimage/emulate work;
not re-reviewed. The kuna-num placement (checklist says crate
`kuna-harness`) follows the convention adjudicated in
w1-harness-unittests F1 (tests live in the crate under test; the
checklist `crate`/`gate` fields are stale process metadata).

All 31 C++ `TEST(name)` functions are present as `#[test] fn name()`
with exact names (file-level `allow(non_snake_case)`), same constants,
same assertion order, including the vestigial `FloatFormat format8(8)`
in float_opTrunc_to_int (`_format8`) and the C++ `// TODO` /
`// generated` comments. Counted: 31 TEST( in C++, 31 #[test] in Rust
(a 32nd `#[test]` grep hit is inside the module doc comment).

Scaffolding mapping verified against the C++ source: raw-bits helpers
(memcpy on the little-endian host -> from_bits/to_bits; the 4-byte
memcpy out of a uintb -> `e as u32` truncation, commented);
ASSERT_FLOAT_ENCODING/ASSERT_DOUBLE_ENCODING -> fns taking f64 (every
C++ call site's argument is or is promoted to double at
`getEncoding(f)`; the float macro's `floatToRawBits(f)` narrowing is
reproduced by `f as f32` — for float-typed C++ call sites this inserts
an f32->f64->f32 roundtrip the macro doesn't perform, bit-exact on this
host for every call-site value including ±qNaN, pinned by adversarial
test); numeric_limits constants pinned by bit pattern
(denorm_min=from_bits(1), qNaN=0x7fc00000/0x7ff8...000) or mapped
(min->MIN_POSITIVE, max->MAX, infinity->INFINITY); the file-static
vectors -> fixture fns returning arrays in identical order (the
`min ± denorm_min` one-ulp arithmetic is exact in both languages —
independently re-derived, see adversarial tests).

## Hunt list

- signedness: clean. The only C++ signed/unsigned comparisons are
  `(int64_t)f > INT_MAX || < INT_MIN` (same-type i64 in Rust) and the
  bool->uintb coercions (`u64::from(...)`).
- integer widths: clean. int->uintb at opInt2Float call site is modular
  (sign extension); port uses `i as i64 as u64`. `(uintb)(int32_t)f &
  0xffffffff` -> `f as i32 as i64 as u64 & 0xffffffff`, identical.
  floatFromRawBits's 4-byte memcpy -> low-32 truncation, only fed
  32-bit constants.
- wrapping: clean — no integer arithmetic beyond masks; float
  arithmetic is intentionally host IEEE in both.
- comparator totality: n/a — no sorts/ordered containers; the IEEE
  partial comparisons (NaN cells) are the test subject and mirror the
  C++ operators cell-for-cell.
- iteration order: clean. C++ loops over `vector` by insertion order;
  Rust arrays in the same order; nested f1-outer/f2-inner preserved.
- off-by-one/do-while/reverse: clean — the macro `do{}while(0)` is
  hygiene only; no loops with at-least-once semantics.
- erase-while-iterating: n/a.
- exception->Result parity: n/a — every FloatFormat op in scope is
  infallible in both languages.
- UB sites (extra): the C++ `(int64_t)f` guard in float_opTrunc_to_int
  is UB for NaN/±inf/FLT_MAX; the oracle host's cvttss2si yields
  i64::MIN for all four (probe-verified), so C++ skips exactly those 4
  of 18 fixtures. The port's `host_float_to_int64` emulation
  (testfloatemu.rs:415-432) reproduces precisely that skip set — Rust's
  saturating `as` would have silently changed it (NaN->0 row kept).
  Correct and documented. Verified independently (probe + adversarial
  test pinning skip set and all 14 kept cells).

## Mechanical pass

No todo!/unimplemented!/HashMap/HashSet/sort_unstable in the item's
file. Bare `as` casts: all 14 sites carry adjacent prose justifications
(the `// cast:` token convention from the kuna-sleigh items isn't used,
matching the w1 ported-test precedent; note-level, see F3). clippy: one
real violation in the item's file (F2, fixed in-review); kuna-num and
kuna-base otherwise clean under `--all-targets -- -D warnings`.

## Findings

- F1 (loss, ledgered): double_decimal_precision is #[ignore]d because
  the f1 = DBL_MAX row diverges — kuna-num float.rs print_decimal's
  roundtrip readback uses `str::parse` (overflow -> inf) where C++11
  num_get stores numeric_limits::max() on ERANGE, so the C++ ladder
  stops at 15 digits ("1.79769313486232e+308") and the Rust ladder
  falls through to 17 ("1.7976931348623157e+308"). Independently
  confirmed against a standalone C++ oracle probe (verbatim
  printDecimal transcription + num_get behavior on this host's
  libstdc++: reads back == DBL_MAX, failbit set) and by running the
  ignored test (only the f1 assertion fails; the other five rows pass —
  pinned by adversarial test). This is a library bug in the
  already-accepted w1-num-float-multiprec scope (its review's
  printDecimal sweeps had no MAX-boundary cell), surfaced by this
  harness item; the harness port itself is faithful and correctly
  refuses to re-pin the oracle string. Ledgered as LOSS-025 with
  restoration criteria (fix the float.rs readback, un-ignore the test,
  re-run this gate at 31/31).
     cpp: decompiler/cpp/float.cc:465-473;
     decompiler/unittests/testfloatemu.cc:186-187
     rust: rust/crates/kuna-num/src/float.rs (print_decimal readback);
     rust/crates/kuna-num/tests/testfloatemu.rs:203-230
- F2 (minor, fixed in-review): the C++ `//// FloatFormat tests` section
  divider transcribed verbatim trips clippy::four_forward_slashes under
  the project's `-D warnings` gate (the lint fires on the test target,
  which the established review standard includes via `--all-targets`).
  Comment-only one-line fix committed by the verifier (d52e8d6); no
  behavioral content.
     cpp: decompiler/unittests/testfloatemu.cc:84
     rust: rust/crates/kuna-num/tests/testfloatemu.rs:84
- F3 (note): the bare `as` casts carry prose justifications rather than
  the `// cast:` token used in kuna-sleigh source items; consistent
  with the w1 ported-test files. Fix opportunistically if the token
  ever becomes machine-checked.
     cpp: n/a (process)
     rust: rust/crates/kuna-num/tests/testfloatemu.rs:39,68,258-263,
     388,391,431,450
- F4 (note, carried): no `port-audit` crate exists in the workspace, so
  the protocol's blob-sha drift check was done by hand (matches).
  Carried gap since the w1-harness-unittests review.
     cpp: n/a (process)
     rust: rust/ (workspace; no port-audit member)

## Adversarial tests

Committed in the worktree as 7988d6b
(rust/crates/kuna-num/tests/verify_w2_harness_floatemu.rs), all
passing; fixtures and expected values re-derived from the C++ source
and a standalone C++ probe, not from the porter's helpers:

- verify_fixture_bits_rederivation — the 18 fixture bit patterns
  (one-ulp `min ± denorm` cells included) re-derived independently.
  (Caught the verifier's own initial sign error on the
  `-min + denorm`/`-min - denorm` rows — the porter's
  expression-based fixture is correct by construction.)
- verify_fixture_encodings_bit_exact — get_encoding(f64::from(f)) is
  bit-exact for all 18 fixture values (the promotion path every op
  test's inputs depend on, incl. ±qNaN and the subnormal boundary).
- verify_float_encoding_macro_nan_roundtrip — the macro rewrite's
  f32->f64->f32 roundtrip is bit-exact for ±qNaN; the widened doubles
  equal the x86 numeric_limits NaNs.
- verify_trunc_guard_skip_set_and_kept_cells — the UB-guard skip set
  is exactly {FLT_MAX, qNaN, -inf, +inf} (pinned from the C++ probe's
  cvttss2si results) and op_trunc matches
  `((uintb)(int32_t)f) & 0xffffffff` on all 14 kept cells, incl. the
  sign-extension cell (-1.234 -> 0xffffffff).
- verify_double_decimal_precision_non_overflow_rows — the five
  non-DBL_MAX rows of the ignored TEST match the C++ oracle strings
  (bounds the LOSS-025 surface to exactly one assertion).
- verify_print_decimal_dbl_max_loss_trace — the LOSS-025 divergence
  trace: "1.79769313486232e+308".parse::<f64>() == inf (root cause;
  C++11 num_get stores DBL_MAX, probe-verified), the f32-path analogue,
  and an either-or pin on print_decimal(DBL_MAX) that survives the
  future repair.

losses: LOSS-025 (double_decimal_precision DBL_MAX row; test #[ignore]d
pending the float.rs readback fix owned by the w1-num-float-multiprec
surface).
