# w3-ir-flow
verdict: ACCEPT-WITH-LOSSES
verifier: claude-opus (independent)
date: 2026-06-11
gate: `cd rust && cargo test -p kuna-decomp` -> ok (149 in-tree + 17 verify_w3_ir_flow + all sibling verify suites green; 0 failed)

## Scope
C++: `decompiler/cpp/flow.cc` (1462 LOC) + `flow.hh` (171 LOC); blob shas matched
against the checklist (`f155217a…` flow.cc / `360a8645…` flow.hh — both verified
identical to `HEAD:decompiler/cpp/flow.{cc,hh}`). Rust: the `w3-ir-flow` commit
(`5d0fced`) is a single new file `rust/crates/kuna-decomp/src/flow.rs` (1688 LOC);
the other files in the branch diff (`block.rs`, `funcdata*.rs`, their tests) are
the already-reviewed dependency items (`w3-ir-block`, `w3-ir-funcdata{,-op,
-varnode}`) merged in, NOT in this item's scope.

The C++ pointer web (`Funcdata &data` + `obank`/`bblocks`/`qlst` references +
owned `PcodeEmitFd emitter`) is realized as one owned `Funcdata` arena (ADR 0001)
plus a borrowed local seam trait `FlowEnvironment` (`E`) carrying the non-W3-IR
slice `FlowInfo` reaches through `glb`/`Override` — the W2 `Translate`, the W6
op-code→`TypeOp` table, the W4 flow-override/userop predicates, and the
GH-8817/GH-6882 kuna anchors. `qlst` is a placeholder count (`FuncCallSpecs` is
W4). The `std::map<Address,VisitStat> visited` is a `BTreeMap` (ADR 0002), keyed
by `Address`'s transcribed total order.

## Hunt list
- **Signedness.** clean. The C++ flow ops compute on `uintm`(=u32) op-times,
  `uintb`(=u64) address offsets, and `int4`(=i32) instruction sizes. Every
  compare re-derived: `newtime > maxtime`, `op->getTime() >= maxtime` are u32-vs-u32
  (faithful); the `visited`/`addrlist`/`unprocessed` `Address` compares route
  through the verified total `Ord` (address.rs:499, space-rank/index then offset);
  `bound <= addrlist.back()`, `bound == eaddr`, `to < baddr || eaddr < to` all use
  it. No `int4`-loop-index-against-`size()` compare in scope (the C++ index loops
  are over W4 vectors — `qlst`/`tablelist`/`newTables` — in the deferred bodies).
- **Integer widths.** clean. `uintm→uintm`(u32), `uintb→uintb`(u64), `int4→int4`
  (i32). The one width-sensitive expression is `findRelTarget`'s
  `uintm id = op->getTime() + addr.getOffset()` (C++ u32 + u64, narrowed to u32 on
  assignment). Rust (flow.rs:473) `o.get_time().wrapping_add(addr.get_offset() as
  uintm)` truncates u64→u32 first then wrap-adds in u32 — congruent mod 2^32, so
  the low-32-bit result the SeqNum lookup uses is bit-identical (re-derived and
  pinned by the strict-advance adversarial test). `getSize`'s `(int4)(maxaddr-
  minaddr)` is `wrapping_sub … as int4` (flow.rs:383) — exact C++ truncation.
- **Wrapping.** clean (1 theoretical note). The `Address + i64`/`- i64` ops route
  through `wrap_offset(offset.wadd/wsub)` matching C++ `wrapOffset(offset±off)`;
  `id` uses `wrapping_add`/`wrapping_sub` (the `id-1` back-up); `getSize` uses
  `wrapping_sub`. NOTE (very-low, not a finding): `insn_count += 1` (flow.cc:424)
  is a bare `+= 1` on `uint4`; C++ wraps silently at 2^32, Rust panics in debug.
  Unreachable — it would require 4 billion decoded instructions held in the arena;
  the C++ `insn_max`-truncation halts flow long before. Matches the porter's choice
  of a bare operator where overflow is not a real input.
- **Comparator totality.** clean. The only ordered container is `visited`
  (`BTreeMap<Address,_>`); `Address::Ord` is a strict total order (space rank
  {Null<Spc<Max}, then space index, then offset). No float compares. `dedup`
  sorts `unprocessed` by the same `Address::Ord` (`sort()`+`dedup()`), total and
  duplicate-stable — pinned by the cross-space adversarial test.
- **Iteration-order provenance, per loop.** clean. Zero `HashMap`/`HashSet`.
  - `target` no-op chain (flow.cc:123): `visited.get`/`+size` re-find loop —
    BTreeMap point lookups, order-free; the chain key sequence is deterministic
    (verified `verify_target_chains_over_multiple_noop_instructions`).
  - `setFallthruBound`/`reinterpreted`/`fallthruOp`/`checkContainedCall`'s
    `upper_bound`/`--iter` idioms → `visited.range(..=k).next_back()` (last ≤ k)
    and `range((Excluded(k),Unbounded)).next()` (first > k) — exact `lower/
    upper_bound` semantics over the same total key order (re-derived line-by-line;
    the `++iter`-back-to-upper-bound in `setFallthruBound` is reproduced by
    computing the strict-greater key unconditionally, which both C++ branches
    converge to).
  - `xrefControlFlow`/`deleteRemainingOps`/`collectEdges`/`splitBasic`/
    `checkEZModel` walk the **dead list** in insertion order via
    `obank().iter_dead()` (intrusive `ListKind::Insert`, the dependency item's
    verified order) — matches C++ `beginDead()..endDead()`.
  - `connectBasic` walks `block_edge1`/`block_edge2` index-parallel `Vec`s
    (C++ `list` insertion order) — `for i in 0..n` over the two vecs (faithful).
  - `findUnprocessed`/`fillinBranchStubs` walk `addrlist`/`unprocessed` `Vec`s in
    order (C++ `vector` iteration).
- **Off-by-one / do-while / reverse iterators.** clean.
  - `fallthru`'s `for(;;)` is a Rust `loop` with the same break ladder; the
    at-least-once decode is preserved (the first `process_instruction` always runs).
  - `findRelTarget`'s `id-1` back-up + the **strict** `op->getAddr() < res` guard
    (flow.cc:171) is the off-by-one hot-spot — reproduced exactly (flow.rs:496);
    pinned by `verify_find_rel_target_next_instruction_guard_requires_strict_advance`
    (size-0 visited → res==addr → guard fails → error; size-4 → res>addr → fallthru).
  - `fallthruOp`'s off-cut `first+size <= op->getAddr()` → None (flow.cc:106) is
    reproduced (flow.rs:417), pinned by `verify_fallthru_op_offcut_boundary_is_none`.
  - `collectEdges`'s BRANCHIND `iter1=end()` reverse `--iter` mark-clear is in the
    W4-deferred BRANCHIND branch (no JumpTable in W3 → that block is the C++
    `findJumpTable==NULL` early-`break`, faithful).
- **Erase-while-iterating.** clean (faithful re-derivation).
  - `deleteRemainingOps` (`while(oiter!=end){op=*oiter; ++oiter; opDestroyRaw(op);}`)
    is ported as collect-`[start..end)`-then-destroy (flow.rs:741) — the advance
    is captured before the (deferred) destroy, same victim set + order.
  - `xrefControlFlow`'s `op=*oiter++` with mid-loop `deleteRemainingOps(oiter)` /
    `oiter=endDead()` / the CALL `--oiter` back-up: the Rust tracks `cursor`,
    advances `cursor=dead_next(curop)` BEFORE the body may delete the tail, sets
    `cursor=None` for the trim, and re-derives `dead_next(curop)` after a CALL
    insertion (so the next read is the just-inserted halt, exactly C++ `--oiter`
    onto the `opDeadInsertAfter` halt). Re-derivation confirms identical visit
    order. (The CALL/CALLIND back-up only fires when `setupCallSpecs` returns true,
    which is W4-deferred — see losses; the cursor logic is correct for when wired.)
- **Exception → Result partial-state parity.** clean (1 minor finding, F1).
  `newAddress`'s OOB path: `handle_out_of_bounds(...)?` returns Err before the
  `unprocessed.push(to)` — matching C++ where a thrown `handleOutOfBounds` skips
  the push (partial state identical). `target`/`findRelTarget`/`reinterpreted`/
  `handleOutOfBounds`'s `LowlevelError` throws map to `Err` at the same points
  with the same explain strings (re-derived char-for-char). The decode-error
  ladder (`UnimplError`/`BadDataError` catch → ignore/error/truncate) is
  transcribed with the same flag-set + halt-plant order (flow.rs:1062). See F1
  for the one `fallthruOp` swallow divergence.

## Mechanical pass
- `port-audit`: tool not present in this branch's workspace (a later-wave
  artifact); blob-sha drift checked manually instead — both flow blobs match the
  checklist exactly.
- grep (flow.rs): `todo!`/`unimplemented!`/`panic!`/`HashMap`/`HashSet`/
  `sort_unstable` → **zero**. `as` casts → 11, all narrow address/size/width
  conversions whose surrounding line quotes the C++ (`as i64` for `Address±size`,
  `as int4` for the `(int4)` getSize truncation + emitter `isize`/slot, `as uintm`
  for the `id` narrowing). All semantically correct and re-derived; they carry the
  quoted-C++ context rather than the literal `// cast:` prefix the codebase uses
  elsewhere (268 sites) — a style nit (F2, low), not a correctness issue.
- `cargo clippy -p kuna-decomp -- -D warnings` → clean (forced rebuild).

## Findings
  - F1 (minor): `fallthruOp` ends with `self.target(&endaddr).ok()` — a `target()`
    failure (C++ throws `LowlevelError "Could not find op at target address"`) is
    swallowed to `None`. In `collectEdges` the CBRANCH case then raises a
    *different* error ("collectEdges: CBRANCH has no fall-thru op") and the default
    case silently skips the edge, whereas C++ propagates the original throw (or,
    on the default path, pushes a NULL edge that `connectBasic` later derefs — UB).
    Observable only on a **malformed** op graph (a fall-thru target absent from
    `visited`); unreachable on well-formed flow, where every fall-thru address is
    visited. Error-text/placement divergence per ADR 0004, but bounded to
    malformed input where C++ is itself partly UB. Not a blocker.
       cpp: decompiler/cpp/flow.cc:108 (`return target(...)`), :963/:972
       rust: rust/crates/kuna-decomp/src/flow.rs:421 (`self.target(&endaddr).ok()`)
  - F2 (low / style): the 11 `as` casts lack the codebase's `// cast:` justifying
    comment (they quote the C++ line instead). Cosmetic; semantics verified correct.
       cpp: n/a
       rust: rust/crates/kuna-decomp/src/flow.rs:383,415,442,473,494,1025,1052,1135,1645,1680,1683

  No blockers, no majors. The deferred W4/W3-funcdata subsystems are clean seams
  (precise `KunaError` at the boundary or documented `// SEAM` no-ops that still
  set the discovery flags), not silent corruption — ledgered as LOSS-037.

## Adversarial tests (added, committed on branch as 73734e7)
`rust/crates/kuna-decomp/tests/verify_w3_ir_flow.rs` (+5, all green):
  - `verify_target_chains_over_multiple_noop_instructions` — `target` chain-skips
    *several* consecutive no-p-code instructions, and errors when the chain runs
    off the map (flow.cc:123-133).
  - `verify_find_rel_target_next_instruction_guard_requires_strict_advance` — the
    `id-1` back-up + the strict `op->getAddr() < res` guard: size-0 visited →
    res==addr → error; size-4 → res>addr → fallthru resolved (flow.cc:162-172).
    (First draft mis-constructed `id` so it pointed at the branch op itself — the
    port was correct; the test was fixed, a positive faithfulness signal.)
  - `verify_fallthru_op_offcut_boundary_is_none` — the off-cut
    `first+size<=op->getAddr()` → None boundary, with a positive control
    (flow.cc:106-107).
  - `verify_dedup_unprocessed_orders_across_spaces` — sort/dedup across two address
    spaces (lower space index sorts first regardless of offset), de-duplicating
    cross-space and in-space dups (flow.cc:872 + the Address total order).
  - `verify_collect_edges_default_fallthru_gated_on_nextstart` — the default-case
    fall-thru edge is emitted only when the next op `isBlockStart` (`nextstart`),
    suppressed within one block (flow.cc:971).

## Losses
LOSS-037 (appended, main tree): flow.cc call-spec/jumptable/injection/inline-clone
machinery (W4 `FuncCallSpecs`/`JumpTable`/`PcodeInjectLibrary`/`Override` +
`Funcdata::inlineFlow`/`cloneOp`), the `PcodeEmitFd::dump` output-varnode
(`newVarnodeOut`→`opSetOutput`→`banks_mut`) + code-ref (`newCodeRef`) emitter
surface and `opDestroyRaw` (the SAME LOSS-035/036 seam owner's API), and the W4
`warning`/`warningHeader`/`overrideFlow`/`removeUnreachableBlocks` reporting — all
seam-deferred with precise errors / documented no-ops, none reachable through the
W3 surface (pinned by the strand-2 real-Sleigh emitter-seam tests). The pure W3-IR
control-flow analysis is fully ported and verified.
