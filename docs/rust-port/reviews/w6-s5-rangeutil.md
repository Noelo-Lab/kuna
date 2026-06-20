# w6-s5-rangeutil

verdict: ACCEPT_WITH_LOSSES
verifier: claude-opus (independent verifier)
date: 2026-06-12
round: 3
gate: `cd rust && cargo test -p kuna-decomp` -> rangeutil scope GREEN.
  - lib suite: 948 pass (includes the ported `rangeutil/tests.rs` unit tests).
  - verifier suite `verify_w6_s5_rangeutil`: 14 pass (incl. the round-2 F3 test,
    which now self-converts to its `Ok` branch and asserts the x86 oracle).
  - NEW verifier suite `verify_w6_s5_rangeutil_r3`: 6 pass (fresh, hand-derived
    oracles authored this round — see below).
  - The lone whole-suite failure (`corpus_bootstrap::ldefs_pspec_cspec_parse_matrix`,
    "Could not find .sla file for x86:LE:64:default") is ENVIRONMENTAL and
    pre-existing: the gitignored `.sla` build artifacts are absent in the
    worktree (`find -name '*.sla'` -> 0). That test loads a SLEIGH spec, does
    not reference rangeutil, and is NOT touched by this branch
    (`git diff rust-port...HEAD -- corpus_bootstrap.rs` is empty). Out of scope.
blob-sha: rangeutil.cc 4613c6002e7e85644d1eb93353a3161eb2fa9e20 (match),
  rangeutil.hh 6c7cc058581b5bb4d5771cc27ae4c5ed85f89884 (match) — no C++ drift
  from round 2.
branch-diff: this branch (`worktree-wf_96bfc2d8-44d-3`, HEAD c383d67) touches
  only rangeutil.{rs,tests.rs} + its verifier suite, plus a jumptable.rs change
  that is purely the `pub use crate::rangeutil::CircleRange` re-export replacing
  the prior `Err` shell (legitimate in-scope integration, was a W3 jumptable
  loss). No scope creep.

## Round-2 finding F3 — FIXED, faithful.
- **F3 (major): `push_forward_binary(CPUI_INT_SRIGHT)` rebuilt the "Min
  negative" bound with a bare `val_left = val_right + 1` over `intb` (i64),
  which overflows / debug-panics for an 8-byte output domain
  (`out_size == 8` -> `mask == u64::MAX` -> `mask >> 1 == i64::MAX`).** The
  round-3 repair (commit c383d67) changed rangeutil.rs:1488 to
  `val_left = val_right.wadd(1)`.
  - Faithfulness re-derived: `wadd` on `i64` is `wrapping_add`
    (kuna-base/src/types.rs:115,134), so `i64::MAX.wadd(1) == i64::MIN` — exactly
    the two's-complement wrap the C++ `valLeft = valRight + 1` (rangeutil.cc:1331)
    relies on to reach "Min negative". The subsequent `sign_extend(val_left,
    bit_pos)` is unaffected.
  - Hand-derived the full out_size==8 oracle (in1=[0x10,0x05) wrapping, in2=1):
    `valRight=i64::MAX`, `valLeft=i64::MIN`, `left=(i64::MIN>>1)&mask=
    0xC000_0000_0000_0000`, `right=(((i64::MAX-1)>>1)+1)&mask=
    0x4000_0000_0000_0000`, `left!=right` so no fix-up. The porter's round-2 test
    asserts exactly this and now runs its `Ok` arm (no panic). I independently
    re-pinned the same values plus the out_size==2 (bitPos 15) variant and the
    non-wrapping common-path skip in `verify_w6_s5_rangeutil_r3.rs`.

## Round-3 fresh adversarial tests (verifier-authored)
`rust/crates/kuna-decomp/tests/verify_w6_s5_rangeutil_r3.rs` (6 tests, all green,
distinct from the porter's suite, every oracle hand-computed from the C++):
- `r3_sright_out8_canonical_rebuild_matches_x86_oracle` — the F3 panic boundary;
  asserts left=0xC000.._0000, right=0x4000.._0000, step=1. Would panic pre-fix.
- `r3_sright_out2_canonical_rebuild_matches_x86_oracle` — same rebuild at a
  SMALLER width (out_size 2, bitPos 15) where `valRight+1=0x8000` does NOT itself
  overflow, isolating the sign-extend/arith-shr bookkeeping from the F3 wrap.
  Oracle left=0xC000, right=0x4000.
- `r3_sright_out8_nonwrapping_skips_rebuild` — valLeft<valRight, proving the fix
  left the hot path untouched (left=2, right=8).
- `r3_get_size_full_8byte_step4_overflow_oracle` — `getSize` wrap-to-everything
  branch at step 4 (independent of the porter's step-2 case): mask/4+1 =
  0x4000_0000_0000_0000.
- `r3_get_size_nonwrapping_precise_count` — precise non-wrapping counts (8 and 6).
- `r3_wto_two_sibling_loops_deterministic_and_nested` — Bourdoncle WTO across two
  independent sibling loops; pins determinism across repeated runs (no
  HashSet/HashMap order leak), both loop heads as partition heads, each head
  before its body.

## Hunt list (round 3)
- Signedness/widths: clean. The only signed-arithmetic sites are the INT_SRIGHT
  `intb` operations (rangeutil.rs:1488,1492,1494) and the `(self.right/left as
  intb)` compare at 1268 (no arithmetic). All three SRIGHT arithmetic ops use
  `wadd`/`wshr`/`wsub`. `8*sizeof(uintb)-clz` and `8*in_size-1` are bounded
  i32 (max 64 / 63; clz in 0..64) — no i32 overflow, faithful to the C++.
- Wrapping: clean after F3. Mechanical grep of every bare `+`/`-`/`*`/`<<`/`>>`
  outside the `w*` helpers leaves only provably-safe sites: bool-range `val+1`
  (val in {0,1}, rs:127,924); the `(mask>>1)` rebuild now `as intb` + `wadd`;
  `step<<=1` (rs:1378) is a shift-by-1 (Rust `<<=` only panics on shift-COUNT
  overflow); `get_size` `(right-left)/step` in the `left<right` arm (no
  underflow) and `/step` (step>=1, never 0); INT_MULT `%step` (step>=1). No
  remaining bare arithmetic on a sign-extended `intb`
  (`grep 'sign_extend(...)[+\-]'` empty; `grep val_left|val_right` minus helpers
  empty).
- Comparator totality: N/A — no `operator<`/sort/`partial_cmp`/`Ord` impl in
  scope. (This wave's Datatype::compare / ParamEntry-ParamTrial sort /
  TypeFactory cache comparators live in other W6 items, not rangeutil.)
- Iteration-order provenance: clean. `add_equation` is a slot-ordered STABLE
  insert (`vector::insert`; ties keep insertion order — pinned in both suites).
  WTO successors flow through the injected `ValueSetGraph` Vec seam (ordered),
  the traversal uses an explicit `node_stack`; the new two-sibling-loop test
  pins determinism. No `HashMap`/`HashSet`/`sort_unstable` in the file (grep
  clean).
- Off-by-one / do-while / reverse iterators / erase-while-iterating: clean
  (re-confirmed from round 2; no `do/while`, only insert, `pop()`==`pop_back`).
- Exception->Result parity: the IR-coupled ValueSet methods remain `Err` SEAM
  shells with no caller-relied partial mutation (the `Err` is the first
  statement). Recorded as LOSS-083 (below).

## LOSS (ledgered this round)
- **LOSS-083** (MAIN losses.md): the IR-coupled value-set *solver* surface
  (`ValueSetSolver::establish_value_sets`/`solve`, `ValueSetRead::compute`, and
  the entirely-unported `iterate`/`set_varnode`/`generate*` constraint-generation
  methods) SEAMs to `Err`/absent because the W5/W7 IR Varnode->ValueSet
  back-pointer, descendant-op walking, and `FlowBlock` dominance do not exist at
  this item's boundary. The pure `CircleRange` domain (pull-back/push-forward,
  union/intersect, widen, getSize/getMaxInfo, minimal_container, the ARRANGE
  table) and the Bourdoncle WTO topological-order machinery ARE fully ported and
  tested. Cross-wave seam, accountability per the LOSS-050/082 pattern.

## Verdict rationale
All round-2 blockers (F3) are repaired and verified faithful; no new blocker or
major found in round-3 re-derivation. The remaining gap is the cross-wave IR
seam (LOSS-083), which is an accepted, ledgered loss — not a faithfulness defect
in the ported surface. -> ACCEPT_WITH_LOSSES.
