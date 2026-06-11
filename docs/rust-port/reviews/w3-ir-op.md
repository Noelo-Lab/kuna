# w3-ir-op
verdict: REJECT
verifier: opus-4.8 (independent)
date: 2026-06-11
gate: `cd rust && cargo test -p kuna-decomp` -> porter suite green (33 lib + 6 w3-ir-varnode integration), verifier `verify_w3_ir_op` 5/7 (2 failing by design, evidencing the divergences below)

C++ in scope: decompiler/cpp/op.cc (1243 lines), op.hh (362 lines).
Rust diff (item commit d32e721): rust/crates/kuna-decomp/src/op.rs (new, 2062 lines), seams.rs (TypeOp seam).
(The varnode.rs/dtype.rs/verify_w3_ir_varnode.rs hunks in `git diff rust-port...branch`
belong to the prior item w3-ir-varnode and were not re-reviewed.)

## hunt list
- **Signedness**: clean. `getSlot`/`getRepeatSlot` loop indices are `int4` vs
  `inrefs.size()` (usize); the port walks with explicit `i: int4`/`usize` casts
  matching the C++ `int4 i; i<inrefs.size()` (sizes are small, no wrap).
  `isCollapsible` `getSize() > sizeof(uintb)` ported as `out.get_size() as i64 as u64 > 8`,
  faithfully reproducing the C++ int4->size_t unsigned promotion (op.cc:123).
- **Integer widths**: clean. `uintm=u32` matches C++ `uint32_t`; CSE hash
  `(uintm)code()` ported `op.code() as i32 as uintm` (OpCode is `#[repr(i32)]`
  with discriminants matching opcodes.hh); `(uintm)vn->getOffset()` ported
  `get_offset() as uintm` (u64->u32 low-32 truncation, op.cc:165). `(uintm)getCreateIndex()`
  ported as-is (both u32). Rotate `hash>>(sizeof(uintm)*8-8)` == `>>24` on u32, safe.
- **Wrapping**: **F1, F2 (BLOCKER class)** — two `getNZMaskLocal` plain shifts
  panic in Rust debug where the C++ relies on x86 shift-count masking. See below.
  All other arithmetic (`uniqid++`, `sqtime+1`, INT_REM `-1`, INT_MULT `<<sa`,
  INT_ADD carry) correctly uses `wadd`/`wsub`/`wshl`, which makes F1/F2 an
  inconsistency, not a blanket policy gap.
- **Comparator totality**: clean. optree is `BTreeMap<SeqNum,OpId>`; SeqNum Ord is
  kuna-base's (prior wave, separately verified). No new comparator in op.cc.
- **Iteration-order provenance**: clean. optree = std::map<SeqNum,*> -> BTreeMap
  (key order); the six `std::list<PcodeOp*>` (alive/dead/store/load/return/userop)
  -> `IntrusiveList` of prev/next OpId links preserving insertion order; per-opcode
  lists `push_back` in change_opcode order; `iter_at`/`target_lower_bound` are
  `BTreeMap::range` over `SeqNum(addr,0)..=SeqNum(addr,~0)` mirroring
  lower_bound/upper_bound exactly. No HashMap/HashSet anywhere.
- **Off-by-one / do-while / reverse iterators**: clean for what is ported.
  `insertInput` reverse fill ported with an explicit `while i>slot` loop (op.rs:538);
  `removeInput` forward shift faithful. (The C++ `isMoveable` `do{...}while` and
  `target()` reverse `--iter` scan are seam-deferred to W3-block — see F3.)
- **Erase-while-iterating**: clean. `destroyDead` (`op=*iter++; destroy`) ported as
  snapshot-then-destroy (each op destroyed once, in list order — equivalent since
  destroy only unlinks the current op). `markIncidentalCopy` advances the cursor
  via the cached next-link *before* the flag write, matching `op=*iter; ++iter`
  (the body never relinks). Verified by `mark_incidental_copy_through_tail`.
- **Exception->Result partial-state parity**: clean. The two C++ throws in scope
  (`destroy` "Deleting integrated op", `insertAfterDead` "Dead move...") are
  internal-invariant violations ported as `panic!` (ADR 0004), thrown before any
  mutation, so no partial state. `collapse`'s `throw LowlevelError("Invalid
  constant collapse")` is in a seam-deferred method (F3).
- **SeqNum/create_index allocation-order fidelity**: clean. `create_at` does
  `SeqNum(pc, uniqid); uniqid=uniqid.wadd(1)` then `optree.insert`; `create_seq`
  bumps `uniqid` to `time+1` when `time>=uniqid` — both transcribed step-for-step
  (op.cc:971-999). Verified by `uniqid_monotonic_and_seqnum_time`,
  `create_seq_advances_uniqid`.
- **Intrusive-list splice-order fidelity (ADR 0001)**: clean. `moveSequenceDead`
  -> `splice_range` (detach [first,last], reinsert before pos) with the C++
  degenerate-move guard (`previter != Some(firstop)`); head/tail invariants hold.
  Verified by the porter's move-to-middle test plus the verifier's move-to-front
  boundary (`move_sequence_dead_to_front_boundary`).

## findings
  - F1 (major/blocker): `getNZMaskLocal` CPUI_INT_RIGHT extended-precision arm uses
    a plain `>>=` for `sa-8*sizeof(uintb)`. For an input varnode wider than 8 bytes
    with a constant shift count where `sa-64 >= 64` (e.g. size 17, sa 130) the Rust
    shift count exceeds 63 and **panics in debug** ("attempt to shift right with
    overflow"), whereas the C++ relies on x86 masking the count to 6 bits, yielding a
    defined value. The faithful port is `wshr((sa - 8*usize_uintb) as u32)` — every
    sibling shift in this function already uses `wshl`/`wshr`.
       cpp: decompiler/cpp/op.cc:644
       rust: rust/crates/kuna-decomp/src/op.rs:1149
       evidence: tests/verify_w3_ir_op.rs::nzmask_int_right_wide_large_shift_diverges (panics at op.rs:1149:25)
  - F2 (major/blocker): `getNZMaskLocal` CPUI_INT_SRIGHT sign-unknown arm does an
    unguarded `(fullmask >> sa) ^ fullmask` where `sa` is the constant shift count.
    For a <=8-byte INT_SRIGHT with `sa >= 64` (a common constant-folded oversized
    shift) the Rust `fullmask >> sa` **panics in debug**, while the C++ produces the
    x86-masked value (`sa & 63`). Same missing-wrapping class as F1; fix is `wshr`.
    Reachability is broader than F1 (no >16-byte varnode needed).
       cpp: decompiler/cpp/op.cc:668
       rust: rust/crates/kuna-decomp/src/op.rs:1172
       evidence: tests/verify_w3_ir_op.rs::nzmask_int_sright_const_shift_ge_64_diverges (panics at op.rs:1172:32)
  - F3 (minor, accountability): a large set of op.cc/op.hh methods inside this
    item's verified scope are seam-deferred to W3-block / W6 but carry **no LOSS
    ledger entry**, contrary to the repo precedent (LOSS-023, LOSS-029) that every
    in-scope cut is ledgered even when its only consumers are later checklist items.
    Deferred: PcodeOp::{collapse, executeSimple, collapseConstantSymbol, encode,
    printDebug (returns seam Err), nextOp, previousOp, target, compareOrder,
    isMoveable, setCopyImmed, hasCopyImmed}; PcodeOpBank::fallthru; all of
    PieceNode (isLeaf/findRoot/gatherPieces). Each genuinely needs a not-yet-ported
    subsystem (BlockBasic/FlowBlock graph, or W6 OpBehavior/TypeFactory/Encoder), so
    this is a ledger gap, not a behavioral bug. (LOSS-012 already covers the
    restored IopSpace::printRaw seam; printRaw/print_raw is correctly seam-Err'd.)
    Resolution: on the next attempt, add a single LOSS entry enumerating these
    op.cc deferrals with their seam wave + restoration criteria.

## adversarial tests
(committed on branch worktree-wf_434ccb01-75c-3 @ 4aa5d0a,
 rust/crates/kuna-decomp/tests/verify_w3_ir_op.rs)
- nzmask_int_right_wide_large_shift_diverges (FAILING — F1)
- nzmask_int_sright_const_shift_ge_64_diverges (FAILING — F2)
- nzmask_int_right_wide_moderate_shift_ok (passing — pins the in-range sibling so a
  fix to F1 cannot regress it)
- cse_hash_order_sensitive_and_copy_zero (passing — rotate order-sensitivity + COPY->0)
- move_sequence_dead_to_front_boundary (passing — splice move-to-head invariant)
- is_collapsible_output_size_boundary (passing — size 8==sizeof(uintb) collapses, 9 does not)
- mark_incidental_copy_through_tail (passing — cursor-before-mutate, enditer==list end)

## work order for re-attempt
1. F1: op.rs:1149 `resmask >>= sa - 8 * usize_uintb` -> `resmask = resmask.wshr((sa - 8 * usize_uintb) as u32)`.
2. F2: op.rs:1172 `(fullmask >> sa)` -> `fullmask.wshr(sa as u32)` (and re-check the
   sibling `pcode_right` already guards). Sweep the whole `getNZMaskLocal` for any
   remaining plain `>>`/`<<` whose count is not provably < 64 (audited: 1184/1209/
   1216/1259 are provably bounded; 1007 is `>>24` on u32; only 1149/1172 panic).
3. F3: append one LOSS entry (next free id) ledgering the seam-deferred op.cc methods.
4. Re-run `cargo test -p kuna-decomp` — the two `*_diverges` tests must turn green
   (the assertions already encode the x86-masked C++-faithful results).

---

# w3-ir-op — round 2 re-review
verdict: ACCEPT-WITH-LOSSES
verifier: opus-4.8[1m] (independent)
date: 2026-06-11
gate: `cd rust && cargo test -p kuna-decomp` -> GREEN (33 lib + 13 verify_w3_ir_op + 6 verify_w3_ir_varnode; doctests 0). The two round-1 `*_diverges` tests now PASS (F1/F2 fixed).

Re-review of the repair commit `4c9ee9d "rport/w3-ir-op: repair fixes"` (op.rs only, +6/-2).

## round-1 findings disposition
- **F1 (blocker) — RESOLVED.** op.rs:1151 now `resmask = resmask.wshr((sa - 8 * usize_uintb) as u32)`.
  The branch is gated by `sa >= 8*usize_uintb` (so the count is non-negative) and reproduces
  the C++ x86 shift-count masking. Verified by `nzmask_int_right_wide_large_shift_diverges`
  (was failing/panicking; now passes against the x86-masked result `u64::MAX>>2`) plus the new
  lower-edge pin `nzmask_int_right_wide_shift_exactly_64` (sa==64 -> wshr by 0 -> full).
- **F2 (blocker) — RESOLVED.** op.rs:1176 now `resmask |= fullmask.wshr(sa as u32) ^ fullmask`.
  Gated only by `size <= 8`; `sa` may be any constant, so wshr is the faithful fix. Verified by
  `nzmask_int_sright_const_shift_ge_64_diverges` (now passes, result 0) and the sign-zero-arm
  sibling `nzmask_int_sright_signzero_large_shift_ok` (uses `pcode_right`, already count-safe).
- **F3 (minor, accountability) — RESOLVED by the verifier.** Per the protocol the porter must NOT
  touch `losses.md` on the branch (and correctly did not). I appended **LOSS-030** in the MAIN
  tree enumerating the seam-deferred op.cc/op.hh methods (collapse, executeSimple,
  collapseConstantSymbol, encode, printDebug, nextOp, previousOp, target, compareOrder, isMoveable,
  setCopyImmed, hasCopyImmed; PcodeOpBank::fallthru; PieceNode isLeaf/findRoot/gatherPieces) with
  their W3-block / W6 seam wave + restoration criteria. Confirmed each is SEAM-annotated in op.rs
  (not silently dropped): seam notes at op.rs:40-43, :52, :211, :571, :866-900.

## new finding (round 2)
  - F4 (minor): CPUI_SUBPIECE `getNZMaskLocal` arm uses a **signed** `sz1 < usize_uintb`
    comparison (i32 vs i32) where the C++ `sz1 < sizeof(uintb)` promotes `int4` to `size_t`
    (**unsigned**). For a negative `sz1` (truncation offset >= 2^31, i.e. `(int4)offset < 0`)
    the C++ comparison is FALSE -> `resmask = 0` (op.cc:704) / `resmask = fullmask` (op.cc:713),
    whereas the Rust enters `resmask >>= 8*sz1` with a **negative** count -> **debug panic**
    ("attempt to shift right with overflow"); release would compute a wrong value. Same
    signed/unsigned class as F1/F2 but at **structurally-nil reachability**: a SUBPIECE truncation
    offset is bounded by the input varnode size and cannot legitimately reach 2^31 bytes. A latent
    sibling exists at the INT_RIGHT `sa >= 8 * usize_uintb` test (op.rs:1146, also signed where
    C++ op.cc:640 promotes to size_t) — no panic there, only a branch-selection divergence for an
    equally-unreachable negative `sa`. Recorded as a robustness/fidelity divergence, NOT a blocker;
    the established codebase convention for this promotion (`calc_mask`: `(size as u32) < 8`,
    kuna-base/address.rs:1627) is the faithful fix if it is ever made reachable.
       cpp: decompiler/cpp/op.cc:701 (and :707), latent :640
       rust: rust/crates/kuna-decomp/src/op.rs:1212 (and :1219), latent :1146
       evidence: tests/verify_w3_ir_op.rs::nzmask_subpiece_negative_offset_diverges (#[should_panic]:
                 documents the C++=0 vs Rust=panic divergence)

## hunt list (round 2, re-derived independently)
- **Signedness**: F4 (SUBPIECE signed `sz1<sizeof` vs C++ unsigned promotion; minor/unreachable).
  All other in-scope comparisons re-checked: getSlot/getRepeatSlot loop indices, isCollapsible
  `getSize() as i64 as u64 > 8` (faithful unsigned promotion), the INT_RIGHT `sa >= 8*sz1` at
  op.rs:1144 is SIGNED in BOTH C++ (`8*sz1` is `int`, op.cc:638) and Rust — clean.
- **Integer widths**: clean (unchanged; uintm=u32, the CSE-hash and uniqid widths re-confirmed).
- **Wrapping**: F1/F2 fixed; full `getNZMaskLocal` shift sweep re-derived — 1132/1141 delegate to
  pcode_left/pcode_right (count-guarded helpers); 1144 signed-int (matches C++); 1151 (F1 wshr),
  1156 wshl, 1169 `>>1`, 1176 (F2 wshr), 1188 `>>=sa` with sa in 0..63 (mostsigbit_set), 1213/1220
  `>>=8*sz1` panic-safe ONLY for non-negative sz1 (F4), 1222 `<<` gated 0<sz1<8, 1234 `<<8*sa`
  gated sa<8 & size>=0, 1263 `>>=8*size-total` count<=63 (proof: total>=1, 8*size<=64), 1265 wshl,
  1279 `<<1`. No remaining unbounded-positive plain shift.
- **Comparator totality**: clean (optree=BTreeMap<SeqNum,OpId>; no new comparator).
- **Iteration-order provenance**: clean. optree range queries re-verified — `iter_at` uses
  `Included(SeqNum(addr,uintm::MAX))` which matches C++ `upper_bound(SeqNum(addr,~0))` (nothing at
  this addr exceeds uniq==MAX, so inclusive-MAX == first-greater-than-MAX). No HashMap/HashSet.
- **Off-by-one / do-while / reverse iterators**: clean for what is ported (the do-while isMoveable
  and reverse `target()` scan are seam-deferred, now in LOSS-030).
- **Erase-while-iterating**: clean. destroyDead snapshot-then-destroy; markIncidentalCopy advances
  cursor before mutate (op.rs:1592) — re-confirmed.
- **Exception->Result partial-state parity**: clean (destroy/insertAfterDead panic before mutation).
- **SeqNum/create_index allocation-order**: clean. create_at: SeqNum(pc,uniqid) -> uniqid.wadd(1)
  -> optree.insert -> deadlist push_back (op.rs:1469-1477); create_seq: uniqid bump to time+1 when
  time>=uniqid (op.rs:1483-1493). Step-for-step with op.cc:971-998. wadd handles the u32 wrap.
- **Intrusive-list splice-order (ADR 0001)**: clean. move_sequence_dead keeps the degenerate-move
  guard `previter != Some(firstop)` and splices [first,last] before previter (op.rs:1572-1581).

## mechanical pass (round 2)
- grep op.rs: no `todo!`/`unimplemented!`/`HashMap`/`HashSet`/`sort_unstable`. 58 `as` casts, audited
  (int4/usize/uintm/shift-count conversions; the shift-count `as u32` casts feed wshl/wshr).
- `cargo clippy -p kuna-decomp`: clean (no warnings).
- `cargo test -p kuna-decomp`: GREEN.

## adversarial tests added (round 2)
(committed on branch @ 988c849, rust/crates/kuna-decomp/tests/verify_w3_ir_op.rs)
- nzmask_int_sright_signzero_large_shift_ok (passing — F2 sibling, sign-zero arm)
- nzmask_int_right_wide_shift_exactly_64 (passing — F1 lower boundary, wshr by 0)
- nzmask_int_right_wide_zero_shift_ok (passing — common positive-shift path pin)
- nzmask_subpiece_offset_at_sizeof_boundary (passing — SUBPIECE positive-offset boundary)
- nzmask_subpiece_negative_offset_diverges (#[should_panic] — documents F4)
- nzmask_int_mult_total_shift_max_count (passing — op.rs:1263 tightest count 63)

## verdict rationale
F1 and F2 (the two round-1 blockers) are fixed and re-verified against the x86-masked C++-faithful
results. F3 is closed by the verifier-appended LOSS-030. The one new finding (F4) is a minor
signed/unsigned fidelity divergence at structurally-unreachable input (a panic on impossible IR,
never a wrong output on valid IR), so it does not block. ACCEPT-WITH-LOSSES, citing LOSS-030 for the
seam-deferred op.cc/op.hh surface.

losses: LOSS-012 (IopSpace::printRaw seam, pre-existing), LOSS-030 (op.cc/op.hh W3-block/W6 seam-deferred methods, appended this round).
