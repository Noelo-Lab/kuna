# w1-base-foundation

## Round 1

verdict: ACCEPT
verifier: independent verifier agent (Claude, not the porter)
date: 2026-06-10
branch: worktree-wf_cea42066-041-1 (worktree `.claude/worktrees/wf_cea42066-041-1`, reviewed at 76f7be3; adversarial tests committed as a65f2ac)
cpp blob shas: all 4 in-scope files match the checklist entry (`git hash-object` verified: error.hh d294a993, types.h ef752b59, partmap.hh 6bed941e, rangemap.hh ba29f988)

gate: `cd rust && cargo test -p kuna-base` -> ok. 28 passed (lib) + 6 passed
(verifier tests `verify_w1_base_foundation`), 0 failed.
clippy: `cargo clippy -p kuna-base --tests -- -D warnings` -> clean
(workspace `disallowed-types` denies HashMap/HashSet per ADR 0002).
port-audit: `cargo run -p port-audit -- --item w1-base-foundation` NOT
runnable — no `port-audit` crate exists in the workspace yet (see F4); blob
SHA check performed manually instead.

## Method

Re-derived the semantics of each C++ function in `error.hh`, `types.h`,
`partmap.hh`, `rangemap.hh` from source and diffed line-by-logic against
`git diff rust-port...worktree-wf_cea42066-041-1 -- rust/` (2,252 insertions
across error.rs / types.rs / partmap.rs / rangemap.rs). Then built an
**independent C++ oracle harness** against the vendored headers
(g++ -std=c++11/libstdc++, re-run under `-fsanitize=address,undefined` with
byte-identical output) and pinned full traces that the Rust port must
reproduce byte-for-byte — including a 45-op randomized differential run with
a verifier-chosen LCG (different constants *and* seed from the porter's own
digest test). All traces matched on the first run; the only failures during
the adversarial pass were two wrong expectations of mine, both of which the
C++ oracle resolved **in the port's favor** (`find_subsorts(7,20,10)` over
mixed subsorts is defined C++ behavior yielding tag 1, and erase-at-`a==0`
leaves the surviving record's duplicated cell visited twice, `2 2` — the
Rust matched the C++ both times).

Spot-verified port claims that reach outside the in-scope headers: the
`KunaError` variant table against `xml.hh:297` (DecoderError **not** derived
from LowlevelError — `is_lowlevel()` correctly excludes it),
`translate.hh:53` (UnimplError carries `int4 instruction_length`),
`ghidra_arch.hh:55` (JavaError carries `type`), `database.hh:434`
(DuplicateFunctionError is RecovError-derived; its deferral to the database
wave is documented and additive).

## Hunt list

- signedness: **clean.** No int4-vs-uintb comparisons exist in these
  header-only templates; types.rs maps each typedef to the same-width,
  same-signedness Rust primitive (table checked against types.h:27-40,
  including the uint8/int8 = 64-bit trap and uintm/intm pinned to 32 bits).
- integer widths: **clean.** `HOST_ENDIAN` (types.h:42-51, `part[3]` of a
  one-initialized union = 1 iff big-endian) -> `cfg!(target_endian)` const,
  same values.
- wrapping: **clean.** `Line::plus1/minus1` are wrapping; every wrap-capable
  C++ expression is covered: `aminus1 = a-1` with `a == 0`
  (rangemap.hh:293 -> rangemap.rs erase), `(*low).a-1 == b`
  (rangemap.hh:318), `i+1` in zip/unzip (rangemap.hh:183,202),
  `(*low).first-1` / `(*low).last+1` (rangemap.hh:252,261 — proven
  non-wrapping in valid states, wrapping helpers used anyway). Debug builds
  exercise the `a==0` path without panicking
  (`verify_defensive_ub_divergence_paths`). `Wrap` trait (types.rs) follows
  ADR 0003; `wshl/wshr` pin x86 modulo-width shift semantics, documented.
- comparator totality: **clean.** `AddrRange::operator<` (rangemap.hh:88-91,
  compare `last` then `subsort`) becomes derived lexicographic `Ord` on
  `SubRangeKey{last, subsort, seq}` — total; the `seq` tiebreak makes
  multiset duplicates distinct keys. `Subsort: Ord` strengthens C++'s
  strict-weak `<` requirement; fine for all integer subsorts.
- iteration-order provenance: **clean, differentially verified.**
  (a) sub-range multiset order = `(last, subsort)` + libstdc++
  insert/hinted-insert placement among equal keys; the Rust gap-allocated
  `seq` machinery (`hinted_seq`, transcribing `_M_get_insert_hint_equal_pos`)
  reproduces it — pinned byte-identical by the `equal_keys`,
  `same_subsort_overlap`, and 45-op random oracle traces (hinted in-loop
  inserts stack at the *front* of the equal range, unhinted trailing inserts
  append at the *back*: C++ `6 5 4 3 2 1 2 3 4 5 6` reproduced exactly).
  (b) record list order = std::list insertion+splice order
  (rangemap.hh:235-245) -> explicit doubly-linked slab; pinned by every `L:`
  trace line. (c) partmap = std::map key order -> BTreeMap. No
  HashMap/HashSet anywhere in the diff (grep + clippy deny). The one
  internal `BTreeMap<u64,u64>` in `renumber_group` is ordered, not hashed.
- off-by-one / do-while / reverse iterators: **clean.** partmap `getValue`'s
  upper_bound-then-`--iter` -> `range(..=pnt).next_back()`; `bounds`'s
  begin/end cases reproduce all four validity codes (oracle-pinned);
  rangemap `erase`'s `do{}while` keeps at-least-once semantics (first
  element processed unconditionally; `low.expect(...)` at rangemap.rs:874
  replaces the C++ deref-of-end UB with a panic per ADR 0004); the backward
  `--uplow` scan and `find_end`'s double upper_bound are transcribed and
  oracle-pinned (`BE` trace lines). `iter_between` supports `rev()` for the
  C++ `--res.second` walks.
- erase-while-iterating: **clean.** `tree.erase(iter++)` (zip,
  rangemap.hh:182) and `tree.erase(low++)` (erase, rangemap.hh:304) both
  become next-key-before-remove; `insert`'s `++low` correctly *does* visit a
  same-`last`, higher-subsort cell the loop itself just inserted (skipped
  via `f <= (*low).last` failing), matching C++ — covered by the
  random-45 trace. The right-boundary duplicate-cell quirk
  (rangemap.hh:260 `break` falling into the line-273 trailing insert,
  duplicating the `[f,b]` cell) is faithfully reproduced, not "fixed" —
  independently confirmed against C++ (`find(0)` -> `2 2`).
- exception -> Result partial-state parity: **clean.** Nothing in scope
  throws; KunaError `Display` is the bare `explain` string (prefixes left to
  catch frames, per ADR 0004); panics appear only where the C++ is UB
  (stale iterator deref, inverted `erase(beg,end)` range).

## Findings

- F1 (minor): bare `as` cast without the protocol's `// cast:` justification
  comment in ported logic.
     cpp: decompiler/cpp/rangemap.hh:207 (insert-position machinery; no
     direct C++ counterpart — kuna-added seq renumbering)
     rust: rust/crates/kuna-base/src/rangemap.rs:399
     (`SEQ_START + (i as u64) * SEQ_GAP`)
     claim/observed: protocol requires every bare `as` to carry a `// cast:`
     comment; this one is a lossless usize->u64 widening of an enumerate
     index (group size < 2^32 by construction) but lacks the comment.
     repro: `grep -n "i as u64" rust/crates/kuna-base/src/rangemap.rs`.
     Non-blocking; fix opportunistically.
- F2 (note): `find_overlap` API shape narrowed.
     cpp: decompiler/cpp/rangemap.hh:409-423 (returns `const_iterator`, a
     positional sub-range iterator)
     rust: rust/crates/kuna-base/src/rangemap.rs:1015 (returns
     `Option<RecordIdx>`)
     claim/observed: the "first overlapping record" semantics are preserved
     and oracle-pinned (`O:` trace lines); the *position* is dropped. If a
     later wave's caller (database.cc) advances from the returned iterator,
     the API must grow then. Watch item, not a defect.
- F3 (note): defensive behavior on C++-UB-only paths, documented in code:
     inverted `find_subsorts`/`iter_between` iterator pairs -> empty
     (rust/crates/kuna-base/src/rangemap.rs:971,1044 `bound_inverted`;
     cpp rangemap.hh:353-369 would be UB), deref-of-end -> panic
     (rangemap.rs:874; cpp rangemap.hh:302), `clear_range(p1 >= p2)` -> no-op
     (rust/crates/kuna-base/src/partmap.rs `clear_range`; cpp
     partmap.hh:148-156 `erase(beg,end)` UB). Not oracle-observable (the C++
     has no defined behavior there), so no losses.md entry required.
- F4 (note): the verification protocol's mechanical step
     `cargo run -p port-audit -- --item <id>` is not executable — no
     `port-audit` crate exists in `rust/` yet. Blob-SHA drift was checked
     manually (all 4 match). Infrastructure gap for the wave, not a defect
     of this port.
- F5 (note): seq-space limits of the multiset emulation
     (rust/crates/kuna-base/src/rangemap.rs:387-430): a single
     `(last, subsort)` equal range approaching ~2^30 members could overflow
     renumbered seqs, and renumbering invalidates outstanding `Position`s
     (documented at the definition). Unreachable for the symbol-database
     workloads this container serves; `debug_assert` guards seq collisions.

## Adversarial tests

Committed in the worktree as a65f2ac
(`rust/crates/kuna-base/tests/verify_w1_base_foundation.rs` +
`tests/data/w1_{rangemap,partmap}_oracle.out`, oracle outputs from the
verifier's own C++ harness, ASan/UBSan-clean):

- verify_rangemap_equal_keys_matches_cpp_oracle — six identical
  `(range, subsort)` records: equal-key hinted/unhinted placement order and
  exact-node erase from the middle of an equal range, full-trace diff.
- verify_rangemap_same_subsort_overlap_matches_cpp_oracle — right boundary
  on an existing cell with the SAME subsort (duplicate keys through the
  insert loop), both-sides unzip, zip re-sew on erase.
- verify_rangemap_subsort_windows_matches_cpp_oracle — all non-inverted
  `find(point, sub1, sub2)` windows + `find_begin`/`find_end` walks at every
  point (boundary inclusion parity).
- verify_rangemap_random45_matches_cpp_oracle — 45-op randomized
  insert/erase differential run, verifier-chosen LCG (seed/constants differ
  from the porter's digest test), full per-op trace byte-identical to C++.
- verify_partmap_matches_cpp_oracle — split copy semantics, exact re-split,
  clearRange(10, u64::MAX), all four `bounds` validity codes, u64::MAX
  boundaries, byte-identical to the C++ partmap run.
- verify_defensive_ub_divergence_paths — pure-Rust pins of the F3 paths plus
  the C++-confirmed `2 2` duplicate after erase-at-zero.

losses: none cited (no test-visible or user-visible divergence from defined
C++ behavior found; F3 paths are C++ UB and therefore outside the oracle).
