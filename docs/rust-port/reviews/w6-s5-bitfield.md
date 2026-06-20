# w6-s5-bitfield
verdict: ACCEPT_WITH_LOSSES
verifier: claude-opus-4-8 (independent verifier)
date: 2026-06-13
round: 1
gate: `cd rust && cargo test -p kuna-decomp` -> bitfield lib 28/28 pass; `verify_w6_s5_bitfield` 9/9 pass. One UNRELATED failure: `corpus_bootstrap::ldefs_pspec_cspec_parse_matrix` requires compiled `.sla` SLEIGH artifacts (gitignored, absent in this worktree, `corpus_bootstrap.rs` untouched by this branch); pre-exists on `rust-port`. NOT a blocker. Clippy: bitfield files clean; one pre-existing `overly_complex_bool_expr` error in the untouched W5 `heritage.rs:1602`.

## Scope

Item commit `9ee5460` changes exactly two files: `rust/crates/kuna-decomp/src/bitfield.rs`
(+776) and `src/bitfield/tests.rs` (+415). The port has three layers:

1. `BitRange` — the endian-aware bit-range value type. Its source is **not** `bitfield.cc`;
   the C++ keeps it in `address.hh:256-285` / `address.cc:630-868`. The port docstring and
   commit message both name that anchor correctly. This is the load-bearing, deterministic
   foundation of the whole subsystem.
2. `BitFieldNodeState` + `FieldRef` (`bitfield.cc:21-51`) — the per-Varnode field tracker.
   Live `Varnode*`/`const TypeBitField*` become a `VarnodeId` handle and a `FieldRef` enum
   carrying only the field's `is_int` datum (the sole thing the ported methods read off the
   type pointer).
3. The six `Rule` skeletons (`get_op_list` + `clone_rule` + `specs()`), with every
   `apply_op` returning 0 at the transform hand-off.

The two transform engines (`BitFieldInsertTransform`, `BitFieldPullTransform`) and the
`applyOp` bodies of all six rules are unported — recorded as **LOSS-092** (distinct from
the pre-existing LOSS-066, which covers the `expression.cc` bitfield-expression family).

## Re-derivation (BitRange + BitFieldNodeState)

Independently re-derived every `BitRange` method against `address.cc:630-868` and
`address.hh:256-285`, and the three `BitFieldNodeState` constructors + two predicates
against `bitfield.cc:21-51` / `bitfield.hh:39,41`. All match. Specific findings:

- `in_container` field-init ordering: C++ sets byteOffset/byteSize/numBits/isBigEndian
  THEN computes `leastSigBit = translateLSB(op2)` (which reads `this`'s now-set
  byteOffset/byteSize/isBigEndian). Rust sets `least_sig_bit: 0` placeholder then
  overwrites via `res.translate_lsb(op2)` after all dependency fields are populated.
  Correct ordering; `op2` correctly bound to the source range.
- `get_mask`: the `numBits >= 64` guard (`res = 0`) then `wrapping_sub(1)` (faithful to
  C++ `res -= 1` on a `uintb`) then `<<= leastSigBit`. Matches; pinned at lsb 60/nb 4
  (0xf0..00), nb 63 (0x7fff..), nb 64 / nb 64+shift (oracle-confirmed).
- `shift`: the `if leastSigBit<0 ... else if most>byteSize*8` clamp structure is
  preserved exactly (the `else if` matters — high-clamp only when lsb stayed >=0), plus
  the trailing `if numBits<0` zero-clamp. Verified negative-shift underflow (lsb drops to
  0, numBits loses the underflow), exact down-shift (no loss), full underflow (-> empty),
  and high-end overflow.
- `intersectMask` minimal cover: `leastsigbit_set`/`mostsigbit_set` (u64->i32, faithful)
  drive both clamp branches; verified non-contiguous mask 0x0100000000000081 -> nb 57.
- Endian-sensitive methods (`translateLSB`, `overlapTest`, `minimizeContainer`,
  `truncateMost/LeastSigBytes`, `extendBytes`, `isMostSignificant`): both branches
  transcribed; BE paths pinned against the C++ oracle (BE minimize trims byteSize, BE
  truncLSB leaves byteOffset, BE overlapTest containment codes 2/3).
- `compare`: total order over (byteOffset, byteSize, leastSigBit, numBits), strict
  tie-break chain, ignores isBigEndian — matches C++; antisymmetry pinned.
- `BitFieldNodeState::follow_field`: `isSignExtended = is_int && bitsField.isMostSignificant`
  and `origLeastSigBit = bitsField.leastSigBit` after the `in_container` translate.
  Sign-ext matrix (mid-field no-ext, top int sign-ext, top unsigned no-ext, hole) pinned.
- `does_sign_extension_match` returns `false` defensively for a hole (C++ would deref a
  null `field->type`); the only caller is in the unported transform, so no observable
  divergence. Documented.

## Hunt list

- signedness/widths: all four geometry fields are `int4`=i32 exactly as C++; helpers are
  u64->i32. No silent narrowing (`as` casts: only `size_of::<uintb>() as int4`, 8->8).
- wrapping: `get_mask` uses `wrapping_sub` faithful to C++ unsigned `-= 1`. Other
  arithmetic (`byte_size * 8`, `num * 8`) would panic on i32 overflow in Rust debug vs
  C++ UB, but only for non-physical (multi-hundred-MB) byte sizes — not reachable.
- get_mask shift edge: `1u64 << numBits` (Rust debug-panics on negative/>=64 RHS; C++ is
  UB) is gated by the `>= 64` branch and by `byteSize > sizeof(uintb)` checks at every
  C++ call site; not observable for realistic ranges. Noted, not a blocker.
- comparator totality: `compare` is a verified total order; `allowedFinalWrites` is
  confirmed STRICTLY ASCENDING in the shared OpCode numbering (1,11,12,...,46,63), so the
  C++ `binary_search` "must be sorted" invariant survives for the future transform port.
- iteration-order provenance / erase-while-iterating / exception->Result: N/A at this
  boundary (no IR traversal or container mutation is ported; the transforms that do are
  the seam).

## Rule layer

`get_op_list` bodies match C++ element-for-element: STORE; allowedFinalWrites+INDIRECT;
LOAD; the explicit 17-entry RuleBitFieldIn list in exact order; ZPULL,SPULL; INSERT.
`ALLOWED_FINAL_WRITES` matches the `bitfield.cc:118-125` initializer verbatim. `specs()`
emits the six rules in C++ declaration order with a `group: ""` placeholder; `specs()` is
not yet consumed by any registry (no Rust `universalAction` exists), so the placeholder is
dormant and harmless — the W8 builder assigns the real group.

## Tests added

`rust/crates/kuna-decomp/tests/verify_w6_s5_bitfield.rs` — 9 adversarial tests pinned to
values from a freshly compiled C++ oracle (`g++ -O0` transcription of address.cc):
negative-shift underflow + high-clamp; non-contiguous intersectMask minimal cover;
BE minimize/rounding; BE truncLSB endian + field-eat; BE overlapTest containment;
getMask high-shift/wrapping; compare total-order antisymmetry; follow_field sign-ext
matrix; BE in_container/translateLSB provenance. All pass.

## Verdict

ACCEPT_WITH_LOSSES. The ported surface (BitRange + BitFieldNodeState + rule skeleton) is
a faithful, fully-pinned transcription with no correctness divergence found on any probed
path. The unported transform engine + applyOp bodies are a legitimate cross-wave seam,
now ledgered as LOSS-092 with explicit restoration criteria. No blocker stands.
