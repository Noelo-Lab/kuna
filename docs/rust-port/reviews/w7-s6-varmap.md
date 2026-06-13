# w7-s6-varmap
verdict: ACCEPT-WITH-LOSSES
verifier: claude-opus-4-8 (independent)
date: 2026-06-13
round: 3 (re-review after repair commit 25dfd04)
gate: `cd rust && cargo test -p kuna-decomp` ->
      varmap lib unit tests 33/33 PASS; r1 suite 7/7 PASS; r2 suite 4/4 PASS
      (the former F2-reproduction `verify_r2_merge_confuse_diff_plus_size_wraps_like_cpp`
      now PASSES — the repair closed it); r3 suite 7/7 PASS.  The single crate-wide
      gate failure is `corpus_bootstrap::ldefs_pspec_cspec_parse_matrix`, which needs a
      compiled `.sla` build artifact absent in the worktree (environmental, fails
      identically on the base branch — NOT this item).  The one clippy `-D warnings`
      `error` is `heritage.rs:1602` (a different module's `#[cfg(test)]` assertion);
      zero clippy diagnostics reference any varmap file (lib or tests clippy-clean).

## Round history

- Round 1 REJECT: F1 (major, sstart scale chain ended in `byteToAddressInt` divide
  instead of `addressToByte` multiply) + F2 (minor, merge resType==2 `diff + b.size`
  bare `int4 + int4`).
- Round 2 REJECT: F1 confirmed FIXED (`f4de4e1` → `address_to_byte_int` multiply at
  both sites); F2 held open as the blocker, with a failing reproduction test.
- Round 3 (this review): F2 FIXED.  Repair commit `25dfd04` switched both adds to
  `diff.wadd(b.size)` (varmap.rs:495-496).  Independently re-derived; the r2 F2
  reproduction now passes, and r3 adds size-growth tests that pin the *value* the
  `wadd` produces (8, 12), not merely the absence of a panic.

## Scope / blob parity

C++ in scope: `decompiler/cpp/varmap.cc`, `decompiler/cpp/varmap.hh`.  Blob SHAs match
the checklist `cpp_blob_sha` exactly (cc `27330a23…`, hh `5143b24b…`) — verified by
`git rev-parse HEAD:decompiler/cpp/varmap.{cc,hh}`; no drift.  (port-audit not present
in this worktree.)

Rust diff (`git diff rust-port..worktree-wf_f5333dc7-250-2 -- rust/`): `src/varmap.rs`
(1135), `src/varmap/tests.rs`, `tests/verify_w7_s6_varmap.rs` (r1), `verify_…_r2.rs`,
one line in `lib.rs`.  The ported core is the full order-determining surface
(`RangeHint::merge/reconcile/contain/preferred/absorb/attemptJoin/isConstAbsorbable/
compare`; `MapState::addRange/addFixedType/reconcileDatatypes/initialize/gatherSymbols`
+ the `compare` stable_sort; `AliasChecker::deriveBoundaries/gatherInternal/
hasLocalAlias/sortAlias`; the recommendation records).  The IR-mutating `ScopeLocal`
restructure pipeline and the Varnode-graph alias walk are `Funcdata`/`Database` seams
deferred to a later wave — documented in the module header; now ledgered as **LOSS-109**
(was previously unfiled).

## Hunt list

- **Signedness**: clean.  `RangeHint::compare` orders `sstart` (intb/i64) by explicit
  `<`; the rangeType discriminant order (`fixed=0<open=1<endpoint=2`) via derived `Ord`
  matches `varmap.cc:328-329`.  `reconcile_datatypes` uses `usize` indices reproducing
  the C++ `int4 < maplist.size()` loop bounds.  No silent signed/unsigned compare.
- **Integer widths**: ONE divergence, ledgered as LOSS-108 (see F2 below).  `int4→i32`,
  `uintb→u64`, `intb→i64`, `uint4→u32` consistent.  `getAlignSize()` confirmed `int4`
  (type.hh:245).  The `int4 trialhi` truncations in `absorb` (varmap.rs:410/424) and the
  `(int4)(b->sstart-sstart)` truncation in `merge` (varmap.rs:494) faithfully mirror
  C++ promotion-then-narrowing; pinned by r3 tests.
- **Wrapping**: F1 (varmap.rs:495-496 `diff + b.size`) FIXED → `diff.wadd(b.size)` (i32
  wrapping add, the C++ "confuse" path's deliberate silent wrap).  Independently
  re-derived at both occurrences; the i32-wrap result equals C++.  `isConstAbsorbable`,
  `contain`, `attemptJoin`/`absorb` diffsz, the sstart scale chains, and the `reconcile`
  modulus all use `wadd/wsub/wmul`/`%`.  Re-grepped: no remaining bare `+/-/*` on
  intb/uintb-lineage values.
- **Comparator totality**: clean.  `RangeHint::compare` is a strict total order (no
  floats; integer/enum tie-break chain start→size→rangeType→flags→highind, datatype
  excluded, matching `varmap.cc:321-335`).  `compare_ranges` = `a.compare(b).cmp(&0)`;
  `sort_by` is stable = C++ `stable_sort`.  Re-verified anti-symmetric over the i64
  sstart range by the r1 negative-sstart test.
- **Iteration-order provenance**: clean.  Every loop iterates a `Vec`/slice by index or
  insertion order matching the C++ vector/list (maplist from the stable_sort;
  add_base/alias in gather order; `gather_symbols` preserves caller-supplied rangemap
  order).  No `HashMap`/`HashSet` in ported logic (grep clean).  `gatherInternal`
  collects `entries` (the bases) before the loop to satisfy the borrow checker — same
  elements, same order as the C++ `addBase` iteration.
- **Off-by-one / do-while / reverse iterators**: clean.  `get_next` (`iter+=1; iter !=
  len`) = C++ `++iter; iter==end?false`; `initialize` endpoint at `wrapOffset(last+1)`
  reproduced; `reconcile_datatypes` run boundaries (`< len`) match; `contain` end
  boundary (`b_end <= a_end`) pinned by r3 (`r3_contain_end_boundary_exact`).  No
  do-while / `loop {}` constructs.
- **Erase-while-iterating**: clean.  `reconcile_datatypes` builds `new_list` via
  `mem::take` + selective push (C++ builds `newList` and `delete`s dropped heap hints);
  same elements, same order, same back-fill of the kept prefix.  The dropped `curHint`
  (`delete`) is not pushed and so is not type-back-filled — matched.
- **Exception → Result partial-state parity**: clean.  The single C++ throw
  ("Overlapping forced variable types", `varmap.cc:280`) fires before any `this`
  mutation; the Rust `Err` returns at the same pre-mutation point (varmap.rs:469).  No
  partial state diverges.

## Findings

  - F1 (RESOLVED this round): `RangeHint::merge` resType==2 grew `size` with a bare
    `int4 + int4` (`diff + b.size`).  Repair commit `25dfd04` → `diff.wadd(b.size)` at
    both occurrences (varmap.rs:495, :496).  C++ (`varmap.cc:301-304`) `int4 diff =
    (int4)(b->sstart - sstart); if (diff + b->size > size) size = diff + b->size;` —
    both adds are deliberate silent signed wraps; the i32 `wadd` reproduces them
    exactly.  Confirmed: the r2 reproduction now passes, and r3 pins the grown value
    (8, 12) on non-overflowing displaced frames.
       cpp: decompiler/cpp/varmap.cc:301-304
       rust: rust/crates/kuna-decomp/src/varmap.rs:494-497
       status: FIXED (no longer a blocker).

  - F2 (MINOR, accepted as LOSS-108): `RangeHint::isConstAbsorbable` widens
    `highind * getAlignSize()` to `intb`/i64 before multiplying; C++ multiplies
    `int4 * int4` at 32 bits (truncating) and only then widens.  They diverge when the
    product overflows i32 — which needs `highind >= 2^28` on a real frame
    (`alignSize <= 8`), an array index no real stack frame produces.  Unreachable on
    well-formed input (same pathological class as the repaired F1); the 32-bit-safe
    regime agrees to the byte.  Ledgered LOSS-108; pinned by r3
    `r3_const_absorbable_int4_mul_overflow_is_wide_in_rust` (current wide branch) and
    `r3_const_absorbable_highind_mul_boundary_32bit_safe` (exact agreement otherwise).
       cpp: decompiler/cpp/varmap.cc:46-47
       rust: rust/crates/kuna-decomp/src/varmap.rs:172

  - OBSERVATION (not a finding): the bare width `as` casts in varmap.rs (e.g. `b.size
    as intb`, `… as int4`) lack per-line `// cast:` comments the protocol's mechanical
    pass asks for, but each is a faithful C++ implicit promotion/truncation documented by
    the surrounding C++-line doc-comments and re-derived above; consistent with the rest
    of the crate.  Does not change the verdict.

## Adversarial tests

r3 file `rust/crates/kuna-decomp/tests/verify_w7_s6_varmap_r3.rs` (committed `a102321`),
7 tests, all PASS:
  - `r3_merge_confuse_grows_size_then_opens` — F1 repair: confuse path grows size to 12
    (non-power-of-two) → collapses to size 1 + Open; pins the `wadd` intermediate value.
  - `r3_merge_confuse_grows_size_to_power_of_two` — F1 repair: grows size to exactly 8
    (a power of two) → stays Fixed UNKNOWN(8).
  - `r3_const_absorbable_highind_mul_boundary_32bit_safe` — LOSS-108: 32-bit-safe
    multiply boundary agrees to the byte (end = sstart + highind*alignSize, strict `>`).
  - `r3_const_absorbable_int4_mul_overflow_is_wide_in_rust` — LOSS-108 witness: documents
    the (unreachable) wide-vs-truncated divergence; flips when the multiply is narrowed.
  - `r3_contain_end_boundary_exact` — `contain` end boundary + sstart short-circuit.
  - `r3_absorb_copy_constant_grows_highind` — `absorb` COPY_CONSTANT diffsz → highind 9.
  - `r3_absorb_clears_self_copy_constant_when_b_lacks_it` — the copy_constant xor-clear.

Prior verifier files (`verify_w7_s6_varmap.rs` 7/7, `verify_w7_s6_varmap_r2.rs` 4/4) all
PASS in this round.

## losses
- LOSS-108: `isConstAbsorbable` 64-bit-vs-int4 multiply width divergence (unreachable on
  well-formed frames).
- LOSS-109: the `Funcdata`/`Database`-bound `ScopeLocal` restructure pipeline + the
  Varnode-graph alias walk are documented cross-wave seams, unported at this wave (the
  order-determining core IS ported and tested).
