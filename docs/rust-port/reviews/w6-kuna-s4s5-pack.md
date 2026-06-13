# w6-kuna-s4s5-pack
verdict: ACCEPT
verifier: claude-opus-4-8 (independent verifier)
date: 2026-06-13
round: 2

gate: `cd rust && cargo test -p kuna-decomp`
  -> lib: 1106 passed / 0 failed; catalog_bytecompat: 5/5; corpus_bootstrap:
     4/5 (1 FAILED). The single failure `corpus_bootstrap::ldefs_pspec_cspec_parse_matrix`
     is ENVIRONMENTAL and unchanged from round 1: it panics at
     `build_spec_file(x86:LE:64:default) failed: Could not find .sla file` because
     the worktree has no compiled `x86-64.sla` (a gitignored build artifact present
     only in the main tree, `specs/.../x86-64.sla`). The test file
     (`corpus_bootstrap.rs`) is NOT touched by this item (`git diff
     rust-port...HEAD -- corpus_bootstrap.rs` is empty), exercises sleigh/spec-file
     resolution, and touches NONE of the three in-scope kuna modules. All in-scope
     lib + integration tests pass, including the round-2 adversarial suite (6/6).
clippy: `cargo clippy -p kuna-decomp` clean (no warnings) — same as round 1. NB:
     `cargo clippy -p kuna-decomp --tests` reports a `nonminimal_bool` error in
     `src/ruleaction_4/tests.rs` / `ruleaction_5/tests.rs` (`assert!(!(hi < lo))`),
     pre-existing test files from OTHER items merged onto this branch — NOT in any
     of the three in-scope files nor in the verify file. Out of scope.
port-audit: `port-audit` is not a package in this worktree (as in round 1). Blob
     shas verified manually against the checklist `cpp_blob_sha` record: all six
     in-scope blobs match with NO drift —
       kuna_returnpair.cc 8fc8169d…, .hh 26468a96…,
       kuna_memsetsequence.cc 3bc12382…, .hh e04810cb…,
       kuna_compareform.cc 3c062864…, .hh cea69dfa….

## Scope
In-scope C++ (kuna-owned): kuna_returnpair.{cc,hh}, kuna_memsetsequence.{cc,hh},
kuna_compareform.{cc,hh}.  Rust: kuna_returnpair.rs, kuna_memsetsequence.rs,
kuna_compareform.rs (+ their `tests.rs`).  The fspec/dtype/constseq/prefersplit/
space changes also in the branch diff belong to OTHER items merged onto this
branch and are out of scope (the wave-extras hunt — Datatype::compare/
compareDependency total order, ParamEntry/ProtoModel sort orders, TypeFactory
cache determinism — touches none of the three in-scope files).

## Round-1 findings — disposition
- F1 (MAJOR, the round-1 blocker): `OptionCompareForm::apply` unported.
  **RESOLVED** in repair commit f2117e6. `parse_compare_form` (kuna_compareform.rs:279)
  is now the faithful analogue of the sibling `parse_return_pair_form` /
  `parse_memset_recover_form`: `original` => present_lessequal=true, `canonical`
  => false, else `KunaError::parse("Must specify compareform as 'canonical' or
  'original'")` (byte-identical to the C++ `throw ParseError(...)`), confirmation
  `"Comparison presentation set to <p1> form"`. Re-derived: the parse is an exact
  string match (no case fold / trim), and `CompareForm::present_lessequal()` is
  exactly the gate `ActionPresentCompareForm` consumes. Covered by in-crate tests
  and the round-2 verify test r2_t1.
- F2 (MINOR, never a blocker): the `form_fill_run` plain `+`. Still present; see
  the wrapping hunt entry below. Remains MINOR — not blocker/major.

## Hunt list (round-2 re-derivation)
- Signedness: clean. `restore_lessequal` keeps `intb diff` (i64) as the C++ `intb`;
  width-correct `calc_int_min`/`calc_int_max`/`calc_uint_max` boundary compares
  (verified against `funcdata_op.cc:1049-1058` forward and `kuna_compareform.cc:42-52`
  inverse). `form_fill_run` low-byte compares are `u8`. `keep_single_return` is a
  pure `usize > 2`. (T1/T2/r2_t2 exercise the int-max/int-min/uint-max edges and
  the i=0 unsigned path.)
- Integer widths: clean. int4/uintb mappings consistent. `(expect - run_start) as
  int4` reproduces the C++ `(int4)(expect-runStart)` truncation; `diff as u64` for
  -1 reproduces the C++ `intb`->`uintb` promotion to all-ones; `(first & 0xff) as
  u8` mirrors `(uint1)(first & 0xff)`.
- Wrapping: F2 (MINOR). `form_fill_run` (kuna_memsetsequence.rs:235) computes
  `expect = move_ops[i].offset + move_ops[i].slot as u64` with a plain `+`; the
  C++ `uint8 expect = moveOps[i].offset + moveOps[i].slot` wraps on overflow, while
  debug Rust panics. Only reachable through the `pub(crate)` seam bridge
  `from_collected` with adversarial offsets near u64::MAX; the real (deferred)
  `collectFillRun` confines offsets to within `arrayType->getSize()`, so neither
  the C++ nor the real Rust path can overflow. The merged constseq base uses
  `wrapping_add` for exactly these uintb sums, so `wrapping_add` would be the
  consistent choice — recommended, but not blocking. `restore_lessequal`'s
  `val.wrapping_add(diff as u64)` is correct (and its four overflow guards prevent
  the only wrap-significant inputs). `expect - run_start` cannot underflow (`expect`
  is monotone from `run_start`), so the plain `-` there is fine.
- Comparator totality: clean. `move_ops.sort_by(|a,b| a.offset.cmp(&b.offset))` is
  a total order on u64. C++ uses unstable `std::sort`; Rust `sort_by` is stable —
  they can differ only on DUPLICATE offsets, which the contiguity check
  (`offset != expect` -> break) rejects after the first, so the difference is
  unobservable (Rust is more deterministic, not wrong). No `partial_cmp` anywhere.
- Iteration-order provenance: clean. `ActionPresentCompareForm::apply` snapshots
  `data.obank().iter_alive()` (the intrusive `alivelist` link order == C++
  `beginOpAlive()..endOpAlive()`), and `restore_lessequal` only mutates opcode +
  one input (never inserts/removes ops), so the snapshot is order-equivalent and
  the post-snapshot `get(op)` liveness re-check is dead-but-harmless. No HashMap/
  HashSet in the three files (mechanical grep: NONE).
- Off-by-one / do-while / reverse iter: clean. `keep_single_return` size>2 boundary
  is exact (T3 pins 2 vs 3). `form_fill_run` `last_idx < 1` (require >=2 COPYs) and
  `total_bytes < 16` transcribed verbatim; the `do { } while` Symbol-walk in
  `collectFillRun` is the seam-stub (out of the transcribed surface). No reverse
  iterators in scope.
- Erase-while-iterating: clean. compareform mutates ops in place; never erases.
  `form_fill_run` `truncate(keep)` faithfully reproduces `moveOps.resize(lastIdx+1,
  WriteNode(0,0,-1))`: `keep = last_idx+1 <= len` always, so the resize can only
  shrink and the fill WriteNode is never materialized — equivalent to `truncate`.
- Exception -> Result parity: clean. All three option parses faithfully re-wrap
  the C++ control flow: `parse_return_pair_form` maps the C++ NON-throwing
  error-string return to `KunaError::parse`; `parse_memset_recover_form` re-wraps
  `onOrOff`'s throw; `parse_compare_form` re-wraps the `ParseError` throw with its
  literal message (the F1 fix). No partial-state mutation precedes any of these
  early returns.

## Findings
  - F2 (MINOR): possible debug-panic where C++ silently wraps.
       `form_fill_run` uses `move_ops[i].offset + move_ops[i].slot as u64` (plain
       `+`); the C++ `uint8 + uint8` wraps. Unreachable in the real path (offsets
       are array-bounded by the deferred `collectFillRun`); only the `pub(crate)`
       `from_collected` seam bridge with u64::MAX-class offsets could trigger it.
       Recommend `wrapping_add` for parity with the C++ wrap and the constseq base.
       Carried over from round 1; MINOR, not blocking.
       cpp: decompiler/cpp/kuna_memsetsequence.cc:116
       rust: rust/crates/kuna-decomp/src/kuna_memsetsequence.rs:235

## Nits (non-findings, for the record)
- Bare `as` casts in the three files (kuna_compareform.rs:207 `diff as u64`;
  kuna_memsetsequence.rs:216/231/235/236/243/250) lack the protocol's `// cast:`
  justification comment, but each sits beside the C++ line it transcribes and is a
  faithful reproduction of the corresponding C++ implicit conversion / `(intN)`
  cast. No semantic divergence. Cosmetic only.

## Notes (verified clean, for the record)
- `restore_lessequal` is the EXACT inverse of `Funcdata::replaceLessequal`
  (funcdata_op.cc:1029-1071): diff signs flipped (in0-const => diff=+1 vs forward
  -1; in1-const => diff=-1 vs forward +1), operand slots, the four overflow guards
  (signed int_min/int_max, unsigned 0/uint_max) mirrored in the inverse direction,
  opSetOpcode-before-opSetInput ordering, and `clearCanonicalLessequal` only on
  success — all transcribed; round-trip-verified (T1 signed right-const, r2_t2
  unsigned left-const), edge-decline-verified (T2), and idempotency/mark-gating
  verified (r2_t3).
- `lessequal_type_op` installs `binary | booloutput` opflags for both restored ops
  (TypeOpIntLessEqual / TypeOpIntSlessEqual), W6 seam, documented.
- The seam-blocked memset halves (`collect_fill_run`, `build_memset`, `transform`,
  the `RuleMemsetCopy::apply_op` body) decline to 0 == option-off behavior, are
  documented stubs, and are correctly ledgered as losses in the structured output.
  The ported, output-determining `form_fill_run` is faithful and well-covered by
  the in-crate suite.

## Adversarial tests added (round 2)
(rust/crates/kuna-decomp/tests/verify_w6_kuna_s4s5_pack.rs — committed 42b57af; all pass)
- r2_t1_compareform_parse_public_api — the F1 fix through the public API: exact
  `original`/`canonical` tokens, present_lessequal mapping, byte-identical
  confirmation message, and every near-miss surfacing the C++ ParseError text.
- r2_t2_unsigned_left_const_roundtrip — the i=0/diff UNSIGNED LEFT-constant round
  trip (`0x40 <= V` => `0x3f < V` => `0x40 <= V`), the path T1 (signed,
  right-const) did not exercise; constant lands back exactly, mark cleared.
- r2_t3_idempotent_and_mark_gated — a second apply after a successful restore is a
  no-op (no double-decrement), and an unmarked INT_LESS is never rewritten.
(round-1 tests T1/T2/T3 retained: signed int-max round trip both widths; restore
guards decline + keep mark at unsigned-0 / signed-INT_MIN / unsigned-UINT_MAX-left;
returnpair gate size-2-vs-3 boundary + exact-token parse totality.)

## losses
The seam-deferred memset halves and the option-wiring seams are ledgered by the
porter's structured losses (collect_fill_run/build_memset/transform/RuleMemsetCopy
body; the buildReturnOutput resize site; ArchOption console wiring; copySymbol /
opSetOpcode W4/W6 seams). No NEW loss is required for ACCEPT — F1 was repaired
(ported, not ledgered) and F2 is a MINOR within-port arithmetic nit, not an
accepted divergence.
