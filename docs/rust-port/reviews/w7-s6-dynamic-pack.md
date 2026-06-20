# w7-s6-dynamic-pack
verdict: ACCEPT-WITH-LOSSES
verifier: Claude Opus 4.8 (1M context)
date: 2026-06-13
gate: `cd rust && cargo test -p kuna-decomp` -> lib 1235/1235 pass; the 3 new
      module test groups (dynamic 12, kuna_dynamichashmax 3, kuna_stackguard
      ~adjacent) pass; verifier suite `verify_w7_s6_dynamic_pack` 7/7 pass;
      clippy `-p kuna-decomp --lib` clean (0 warnings). The ONLY failing test in
      the whole crate is `corpus_bootstrap::ldefs_pspec_cspec_parse_matrix`,
      which is PRE-EXISTING and environmental (needs a built `.sla` artifact —
      `make specs` — absent in the worktree), unchanged by this item
      (`git diff rust-port...branch -- corpus_bootstrap.rs` is empty) and
      unrelated to dynamic-pack code. `--no-fail-fast` confirms that single
      failure is the only one across every suite.

## Scope

C++: dynamic.cc/.hh, kuna_dynamichashmax.cc/.hh, kuna_stackguard.cc/.hh.
Rust: dynamic.rs (+tests), kuna_dynamichashmax.rs (+tests),
kuna_stackguard.rs (+tests), lib.rs (3 `pub mod` lines).

## Hunt list

- **Signedness** — clean. `ToOpEdge::operator<` `ord1<ord2` is unsigned
  (`uintm`/u32) both sides; `slot < op2.slot` is signed (`int4`) both sides —
  matches C++. `slot >= numInput()` (find_op/unique_hash_op) both `int4`. The
  `slot < 0` guards always precede any `getIn(slot)`. `(uint4)slot` bit-reinterp
  (`self.slot as uint4`) reproduces C++ `(uint4)slot` for slot=-1 -> 0xFFFFFFFF.
- **Integer widths** — clean. `uintb`->u64, `uint4`->u32, `int4`->i32, `uintm`
  ->u32 throughout. `(uint4)val` truncation of the u64 address offset (dynamic.rs
  :235,:591) matches C++. `(uint4)root->getSize()` matches.
- **Wrapping** — clean. `wadd` (wrapping_add) used on every counter
  (`opedgeproc`/`opproc`/`vnproc`/`method`/`pos`) and `get_total_from_hash`'s
  `+1`; none can actually wrap in range (counters bounded by container sizes,
  total field 0..7). `champion.len() as uint4 - 1` (dynamic.rs:739,:824) uses
  plain `-1` but is guarded by the immediately-preceding `if champion.is_empty()
  { return }`, so len>=1 — no debug underflow. Faithful to C++ `(uint4)size()-1`.
- **Comparator totality** — clean. `ToOpEdge::less_than` -> `sort_by` is a total
  order over (Address, order, slot): `Address`/`SeqNum` `Ord` impls are total
  (space-rank then index then offset; no float, no partial_cmp().unwrap()).
- **Iteration-order provenance** — clean. No `HashMap`/`HashSet` in ported logic
  (stackguard's `seen` is a `BTreeSet`, used only for membership, never
  iterated). `gather_first_level_vars`/`gather_ops_at_address` iterate
  `obank().iter_at(addr)` = a `BTreeMap<SeqNum,_>::range` = the C++
  `PcodeOpTree::const_iterator` (std::map) order. `descend_snapshot` =
  Varnode descend `list` order (C++ `beginDescend`). markvn/markop/opedge/vnedge
  are `Vec` in insertion order. F1 below is the one sort substitution.
- **Off-by-one / do-while / reverse iterators** — clean. The `gatherUnmarkedOp`
  `for(;opedgeproc<size;++opedgeproc)` increment-before-continue is reproduced
  (dynamic.rs:399 increments before the `is_mark()` continue). `calcHash(vn)`'s
  split `for(i=vnproc;...)` (local, non-advancing) vs `for(;vnproc<...;++vnproc)`
  (advancing) is reproduced exactly (markvn does not grow in either loop, so both
  cover the same range). No reverse iterators.
- **Erase-while-iterating** — clean. `dedup_varnodes` is collect-into-new-Vec +
  mark/clear-mark + swap, identical to C++ `resList`/`varlist.swap`; order
  preserved.
- **Exception -> Result parity** — clean. The C++ never throws in dynamic.cc; the
  zero-hash early returns (`hash=0; addrresult=Address()`) are reproduced as
  plain returns at every site (calcHash slot-mismatch, empty opedge, champion
  empty, pos>total, moveOffSkip dead-end). `dynamic_unique_hash` returns
  `KunaResult` but is infallible (always `Ok`). No partial-state restart paths.
- **(wave) HighVariable merge / Cover-intersection order** — N/A to this item
  (no HighVariable/Cover code here; `unique_hash` is the leaf consumed by
  `buildDynamicSymbol`, which is out of scope / seamed — LOSS-099).
- **(wave) varmap stack-layout order** — N/A.
- **(wave) block-structuring schema precedence** — partially relevant via
  stackguard's CFG surgery, which is fully seamed (LOSS-098).

## TRANSTABLE / hash-encoding (output-determining, byte-checked)

- TRANSTABLE is 75 entries, positionally verified against the OpCode enum
  (CPUI_COPY=1..CPUI_SPULL=74, index 0 = unused/skip). Spot-checked the lumping
  invariants (NOTEQUAL=EQUAL, SUB=ADD, LEFT=MULT@29, MULT@32, PTRADD/PTRSUB=
  INT_ADD, CAST=0@64, unused slot 45=0, FLOAT lumping) — all match. The
  compile-time `TRANSTABLE.len()==CPUI_MAX(75)` assertion holds.
- `crc_update` is byte-identical to C++ (verified table + fold formula).
- The 64-bit packing in `piece_together_hash` (`attachedop<<4 | method<<7 |
  opcode<<5 | (slot&0x1f)`, then `<<32 | reg`) and every accessor
  (`getSlot/Method/OpCode/Position/Total/IsNotAttached`, `clearTotalPosition`,
  `getComparable`) match the C++ shifts/masks exactly. `slot&0x1f` for slot=-1
  packs 0x1f and decodes back to -1 (verified by a real calcHash, not just a
  hand-packed value — verifier test T1).

## Findings

- F1 (info, not a defect): `build_vn_down` substitutes a STABLE `sort_by` for
  the C++ `std::sort` (unstable). Within a single buildVnDown sub-range every
  edge has a distinct sort key (each descendant is a distinct op => distinct
  SeqNum; the only same-op case is a varnode read in two slots, which differ by
  the slot tiebreak), so the keys are strict-distinct and stable vs unstable are
  order-identical. The porter's comment states this; verifier test T3
  (`x = a + a`, one op reads root in two slots) confirms determinism.
       cpp: decompiler/cpp/dynamic.cc:147-148
       rust: rust/crates/kuna-decomp/src/dynamic.rs:341-356

- F2 (info, faithful bug-for-bug): the OP-FORM `unique_hash_op` ALWAYS returns
  hash 0, because the C++ `oplist.clear()` (dynamic.cc:519) runs at the top of
  the method loop BEFORE the inner `for(i<oplist.size())` collision scan, so the
  scan never executes and `champion` stays empty. The Rust transcribes this
  exactly (dynamic.rs:794 clears, then iterates the cleared list). This is a
  deliberate spec-faithful transcription (C++ is the oracle), documented in-code.
  Verifier test T2 pins it (asserts 0 for slots {-1,0,1}, narrow AND wide budget).
       cpp: decompiler/cpp/dynamic.cc:512-531
       rust: rust/crates/kuna-decomp/src/dynamic.rs:775-808

- F3 (loss, ACCEPTED -> LOSS-098): `ActionStripStackGuard` is detection-only; the
  terminal `removeBranch`/`removeUnreachableBlocks` pair is the W4/W8
  funcdata_block seam (the same primitives cut in LOSS-035/036/078), so the
  Action returns 0 even on a detected canary. Default-off => zero regression.
       cpp: decompiler/cpp/kuna_stackguard.cc:150-152
       rust: rust/crates/kuna-decomp/src/kuna_stackguard.rs:319-329

- F4 (loss, ACCEPTED -> LOSS-099): the two `ArchOption` decision bodies are
  ported but not registered into the option database / console; `unique_hash`
  takes the collision budget as a parameter rather than reading
  `Architecture::dynamic_hash_maxdup_high` inline (SEAM(W4)). The arch flag
  exists and matches kuna-C++'s DIV-3 default-on (16). Default-equivalent.
       cpp: decompiler/cpp/dynamic.cc:438,503; kuna_dynamichashmax.cc; kuna_stackguard.cc
       rust: rust/crates/kuna-decomp/src/dynamic.rs:696,759; kuna_dynamichashmax.rs; kuna_stackguard.rs

## Mechanical pass

- grep clean: no `todo!`/`unimplemented!`/`panic!`/`HashMap`/`HashSet`/
  `sort_unstable` in the three source modules.
- bare `as` casts: all are enum-discriminant (`OpCode as u32` in TRANSTABLE,
  self-documenting) or width conversions matching a C++ `(uint4)`/`(uint8)` cast,
  audited above (no silent width/signedness surprises).
- `cargo clippy -p kuna-decomp --lib` -> clean.

## Adversarial tests (rust/crates/kuna-decomp/tests/verify_w7_s6_dynamic_pack.rs)

- t1_output_attached_root_encodes_slot_minus_one  (slot=-1 / 0x1f pack via real calcHash)
- t2_op_form_unique_hash_is_always_zero           (faithful oplist.clear() quirk, both budgets)
- t3_root_read_in_two_slots_hashes_deterministically (x=a+a sort substitution safe)
- t4_input_cast_is_invisible_to_op_hash           (moveOffSkip / method-5 CAST peel)
- t5_dynamichashmax_budget_and_default            (8/16 budget, ceiling)
- t5_stackguard_action_inert_both_gates           (seam boundary, CFG untouched gate on+off)
- t5_stackguard_option_body                       (OptionStackGuard apply on/off message)

All 7 pass (committed on the branch).

## losses

LOSS-098 (stackguard detection-only CFG-surgery seam),
LOSS-099 (ArchOption registration + budget flag-threading seam).
Both are seam deferrals over primitives genuinely absent at this wave, both
default-off / default-equivalent (zero regression-suite impact).
