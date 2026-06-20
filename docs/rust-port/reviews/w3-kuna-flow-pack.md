# w3-kuna-flow-pack
verdict: ACCEPT
verifier: opus-4.8 (independent verifier, round 1)
date: 2026-06-11
gate: `cd rust && cargo test -p kuna-decomp` -> ok (309 passed; 0 failed, incl. 8 new verifier tests)

## Scope

C++ in scope (decompiler/cpp): kuna_v850indbranch.{cc,hh}, kuna_switchmodbound.{cc,hh},
kuna_thumbfuncptr.{cc,hh}, kuna_inferfuncentry.{cc,hh}, kuna_sparcstructret.{cc,hh}.
Each `.cc` is small: a predicate free-function plus an `ArchOption::apply`.
`kuna_switchmodbound.cc` deliberately contains ONLY the ElementId + ArchOption
(its recovery walk `kunaTryModuloBoundTable` is a `JumpBasic` member living in
jumptable.cc, out of this item's scope — the Rust port mirrors that boundary).

Rust: src/kuna_{v850indbranch,switchmodbound,thumbfuncptr,inferfuncentry,
sparcstructret}.rs (+ their unit `tests.rs`), plus supporting seams.rs skeleton
(Architecture/Scope/FuncProto placeholders).

## Hunt list

- Signedness: clean. thumb `multiplier > 1`, `bextra <= 0` are signed int8 in both
  C++ and Rust (`int8`); a negative multiplier/bextra is treated identically.
  v850/sparc/inferfunc predicates have no signed/unsigned comparison.
- Integer widths: clean. sparc user-op id `(uint4)idvn->getOffset()` ported as
  `vnref.get_offset() as u32` — faithful 64->32 truncation (justified inline).
  thumb shift count `funcptr_align as u32` over `1i64 << ..` matches C++
  `(int8)1 << int4` (see casts below). address_to_byte_int = `val.wmul(ws)` =
  C++ `val*ws` (kuna-base, not in this diff).
- Wrapping: clean. The only arithmetic is address_to_byte_int (wrapping, ported
  prior wave) and the thumb shift (proven in-range, see below). No uintb add/sub
  in scope.
- Comparator totality: clean. Address::eq = `(base==base)&&(offset==offset)`,
  an exact match of C++ Address::operator== (space-by-pointer + offset). No
  float/partial_cmp anywhere. inferfuncentry exact-entry test relies on this.
- Iteration-order provenance: ONE order-sensitive loop — sparc's backward walk.
  C++ walks `op->getInsertIter()` back to `data.beginOpDead()` via `--iter`.
  Rust collects `obank.iter_dead()` (intrusive insert-list, ADR 0001, = C++
  beginDead()..endDead() order), finds `op`'s index, walks `pos` down to 0.
  Same elements, same order. No HashMap/HashSet in scope.
- Off-by-one / do-while / reverse: clean. The C++ `for(;;)` body order
  (CALLOTHER-check -> instruction-start break -> `iter==beg` break -> `--iter`)
  is transcribed exactly (check trap, then `is_instruction_start` break, then
  `pos==0` break, then `pos -= 1`). A trap that is ALSO the instruction-start
  matches before the break — verified by porter test `walk_reaches_dead_list_head`
  and my dead-list-head termination tests. No reverse-iterator idiom.
- Erase-while-iterating: N/A — all five predicates are read-only (no mutation
  of any list during traversal). The fall-through rewrite is W4 (flow.rs), not
  in this item.
- Exception -> Result partial-state parity: N/A — no fallible (`?`) path; the
  predicates return `bool`. `ArchOption::apply` returns `&'static str`, no throw.
- SeqNum/create_index allocation order: N/A — no op/varnode allocation in the
  ported predicates (read-only).
- Intrusive-list splice order: N/A — no splice; only a read traversal of the
  dead list (provenance covered above).
- Aliasing: clean. `get_in(0)` direct-indexes `inrefs[0]`, matching C++
  `inrefs[slot]` (`vector::operator[]`, no bounds check); reachable only after
  an opcode check (CALLIND/BRANCHIND/CALLOTHER) that guarantees input 0 exists.

## Mechanical pass

- port-audit: tool `port-audit` not present on this branch (not in workspace) —
  N/A; no blob-sha manifest yet at this wave.
- grep todo!/unimplemented!/HashMap/HashSet/sort_unstable/panic!/unwrap/expect in
  the five src files: NONE.
- bare `as` casts (2, both justified inline; see F-note):
  - kuna_sparcstructret.rs:170 `vnref.get_offset() as u32` — C++ `(uint4)` user-op
    id truncation (comment lines 167-169).
  - kuna_thumbfuncptr.rs:194 `funcptr_align as u32` — shift count for
    `1i64 << ..`; proven non-negative (`funcptr_align` is bit-position of first
    set bit of a non-negative alignment, architecture.cc:1064-1069; `== 0`
    returns early) and < 31 (comment lines 188-193).
- cargo clippy -p kuna-decomp -- -D warnings: clean.

## Findings

(no blockers, no majors)

- F1 (informational): the two bare `as` casts carry correct inline
  justification comments but not the literal `// cast:` token from
  verification.md §Mechanical. Both casts are behaviorally faithful (the (uint4)
  truncation and the proven-in-range shift count). Convention nit only.
       cpp: decompiler/cpp/kuna_sparcstructret.cc:36, kuna_thumbfuncptr.cc:33
       rust: rust/crates/kuna-decomp/src/kuna_sparcstructret.rs:170,
             rust/crates/kuna-decomp/src/kuna_thumbfuncptr.rs:194
- F2 (informational): sparc's backward walk is O(n) per call
  (`iter_dead().collect()` + `.position()`) vs the C++ O(1) `getInsertIter()`.
  Behaviorally identical (same visited set/order); a performance, not a
  correctness, difference. No loss.
       cpp: decompiler/cpp/kuna_sparcstructret.cc:29-47
       rust: rust/crates/kuna-decomp/src/kuna_sparcstructret.rs:142-150

## Seam boundaries (documented, not losses)

The five predicates hoist their architecture/type-system reaches out as resolved
inputs with explicit `// SEAM(W4)`/`// SEAM(W6)` notes, because the reached
subsystems are unported at this wave. These are deferred-port boundaries, not
behavioral divergences, and the hoisted value is passed verbatim:
- v850: `glb->translate->getRegisterName(...)` -> `register_name: Option<&str>`.
- sparc: `glb->userops.getOp(id)->getName()` -> `userop_name: Fn(u32)->Option<String>`.
- thumb: the `Datatype`/`TypePointer` walk (getTypeReadFacing/getPtrTo/
  getWordSize/getSubType) -> `ThumbPtrTypeFacts`; `funcptr_align` -> param.
- inferfunc: `glb->resolveConstant(...)` -> `rampoint: Option<&Address>`;
  `scope->queryFunction(rampoint)->getAddress()` -> `function_entry`.
- switchmod: the entire recovery (`kunaTryModuloBoundTable`) is a JumpBasic
  member in jumptable.cc — out of this item's C++ scope; only the gate is owned.
All option `default()`s match the shipped flags (v850/sparc/switchmod off;
thumb/inferfunc on per DIV-2), and `apply` strings are byte-for-byte the C++.

## Adversarial tests

Added rust/crates/kuna-decomp/tests/verify_w3_kuna_flow_pack.rs (8 tests, all pass):
- flowpack_sparc_userop_id_truncates_high_bits_and_matches
  (0x1_0000_0007 user-op id truncates to 7 -> trap match)
- flowpack_sparc_userop_id_high_bits_only_does_not_alias_into_trap
  (0x1_0000_0000 truncates to 0, not 7)
- flowpack_sparc_walk_passes_nontrap_callother_to_reach_trap
  (walk skips a non-matching CALLOTHER to find the trap)
- flowpack_sparc_nearest_trap_matches_before_instruction_start
- flowpack_sparc_lone_branchind_at_deadlist_head_no_panic_false
- flowpack_sparc_walk_to_deadlist_head_non_start_terminates
  (terminates at pos==0 without panic)
- flowpack_v850_only_exact_pc_spellings_excluded
  ("PC"/"pc" excluded; "Pc"/"pC"/"r10" fire — exact-string, not case-insensitive)
- flowpack_inferfuncentry_cross_space_same_offset_not_equal
  (full Address equality: same offset / different space -> no inference)

losses: none
