# w4-p0-override-comment
verdict: ACCEPT-WITH-LOSSES
verifier: claude-opus (independent)
date: 2026-06-11
gate: `cd rust && cargo test -p kuna-decomp` -> ok (264 lib tests + all integration suites green; 0 failed). New verifier suite `verify_w4_p0_override_comment` (6 tests) green.

## Scope
C++: `decompiler/cpp/override.{cc,hh}` (the P0 `Override` store) + `comment.{cc,hh}`
(the `Comment`/`CommentDatabaseInternal`/`CommentSorter` trio). Blob shas verified
against the checklist — exact match, no drift:
`override.cc 0c2b5734…`, `override.hh 693c2e62…`, `comment.cc d9d2000e…`, `comment.hh fe2a251a…`.
Rust: `rust/crates/kuna-decomp/src/overrides.rs` (+ `overrides/tests.rs`, 14 porter
tests) and `src/comment.rs` (+ `comment/tests.rs`, 17 porter tests). Reviewed commit
is `17426e9` only; the branch also carries the *separately-verified* `w4-p0-database`
work (`database.rs`, `verify_w4_p0_database.rs`) which is out of this item's scope and
was not re-reviewed.

The pointer/iterator C++ data model is realized as: `std::map<Address,_>` →
`BTreeMap<Address,_>` (Address `Ord` is total: Null < Spc-by-index < Max, then offset);
`std::set<Comment*,CommentOrder>` → `BTreeMap<CommentKey,Comment>` keyed on the same
`(funcaddr,addr,uniq)` triple; `std::vector` → `Vec`; the C++ `lower_bound(beg)..lower_bound(end)`
probes → half-open `range(beg..end)`. The W4/W5 seams (`FuncProto`, `FuncCallSpecs`,
`Heritage`, the live op-tree) are captured as local traits (`FuncProtoOverride`/`ProtoDecoder`,
`SorterFuncdata`/`OpProbe`/`OpCursor`) with no behavior elided in the in-scope logic.

## Hunt list
- **Signedness.** clean. The two C++ signed/unsigned compares (`-Wno-sign-compare`)
  in `override.cc` are both index-vs-`size()`: `insertDeadcodeDelay`'s
  `while(deadcodedelay.size() <= spc->getIndex())` and `hasDeadcodeDelay`'s
  `if (index >= deadcodedelay.size())`. Space indices are always >= 0, so the C++
  unsigned promotion and the Rust forms (`(len as i64) <= index as i64`,
  `index as usize >= len`) agree on all reachable inputs; both casts carry a
  justification comment (overrides.rs:224-227, :243). The `int4 i < vector.size()`
  loops (`applyDeadCodeDelay`/`printRaw`/`encode`/`generateOverrideMessages`) are
  ported as `.iter().enumerate()` — identical 0..len iteration; `i as int4` cannot
  truncate (space-count-bounded). `comment.cc` has no signed/unsigned compares in
  scope (`uniq` is `int4`, compared only for equality/`<` against `int4`).
- **Integer widths.** clean. `int4→i32`, `uint4→u32`, `uintb→u64` throughout. The
  `delay as i64` widening in `encode` (write) and the post-check `delay as int4`
  narrowing in `decode` both mirror the C++ `int4 delay = readSignedInteger()`
  truncation; the negative-guard precedes the narrowing exactly as the C++ throws
  before `insertDeadcodeDelay` (verified: `override_decode_bad_deadcodedelay_errors`).
  `Subsort` `(int4,uint4,uint4)` and `EntrySubsort` widths match. No promotion-
  sensitive arithmetic (no `int4*int4→intb`).
- **Wrapping.** clean. The only arithmetic on lineage values is `uniq+1`
  (`addComment`/`addCommentNoDuplicate`) and `subsort.pos += 1`. `uniq` is `int4`
  starting at 0 with a 65535 probe sentinel — never near `i32::MAX`; the C++ `+1`
  is signed (UB on overflow) and the Rust plain `+` panics on overflow, the
  standard ADR-0004 posture for this port (LOW note, not a finding — unreachable on
  real input). `pos` is `uint4` bounded by the comment count. No `wrapping_*`/
  `checked_*` is warranted (no value can wrap on valid input).
- **Comparator totality.** clean. `CommentKey` derives lexicographic `Ord` over
  `(Address,Address,int4)` — exactly the three-level `CommentOrder::operator()`
  (`getFuncAddr` then `getAddr` then `getUniq`, trailing `return false` = keys-equal).
  `Subsort` derives `(int4,uint4,uint4)` lexicographic = `Subsort::operator<`
  (index/order/pos). Both rest on the total `Address::Ord`. No floats anywhere.
  Verified by `subsort_orders_index_then_order_then_pos` and
  `ordering_is_funcaddr_then_addr_then_uniq`.
- **Iteration-order provenance.** clean. **Zero `HashMap`/`HashSet`.** Every map is a
  `BTreeMap` keyed exactly as its C++ `std::map`/`std::set` comparator
  (`forcegoto`/`indirectover`/`protoover`/`flowoverride` by `Address`; `commentset`
  by `CommentKey`; `commmap` by `Subsort`); every `vector` (`deadcodedelay`,
  `multistagejump`) is a `Vec` walked by index/`enumerate` in insertion order. The
  `Override::encode` element-group order (forcegoto → deadcodedelay → indirect →
  proto → multistage → flow) is preserved verbatim.
- **Off-by-one / do-while / reverse iterators.** clean. `addCommentNoDuplicate`'s
  backward C++ `while(iter!=begin()){--iter; ...}` is ported as
  `range(..probe).rev()` — same elements (strictly `< probe`), same descending order,
  same `break`-on-first-non-match (verified `add_comment_no_duplicate_walk_breaks_at_different_address`).
  `addComment`'s `lower_bound` then `if(iter!=begin())--iter` predecessor is
  `range(..probe).next_back()`; the only behavioral difference (C++ inspects `begin()`
  when there is no strict predecessor) is **provably inert** — it can bump `uniq` only
  if the first element shares `(addr,fad)` AND has `uniq >= 65535`, impossible for
  real comments, so both yield `uniq = 0` (verified
  `add_comment_first_element_and_lower_funcaddr_predecessor_keep_uniq_zero`). The
  `lower_bound(beg)..lower_bound(end)` → `range(beg..end)` half-open equivalence holds
  because no real comment keys on the `m_maximal` sentinel space. `CommentSorter`
  `lower_bound`/`upper_bound`/`++start` are faithfully `range(k..).next()` /
  `range((Excluded(k),Unbounded)).next()`. No C++ `do/while` in scope.
- **Erase-while-iterating.** clean. `clearType`'s `while(iterbegin!=iterend){ iter=iterbegin; ++iter; if(type&tp) erase(iterbegin); iterbegin=iter; }`
  (carefully advancing before erase) is ported as collect-keys-then-remove over the
  same `range(beg..end)` with the same `type & tp != 0` filter — visits the identical
  element set, and removal order is irrelevant to the result. The cross-function /
  cross-space bracketing is verified by
  `clear_type_brackets_one_function_across_space_boundary` (a same-type comment of a
  *different* function, and the named function's non-matching-type comment, both
  survive). `~CommentDatabaseInternal`/`clear`'s delete-all is `BTreeMap::clear`
  (Box drop).
- **Exception → Result partial-state.** clean. `Override::decode` `?`-propagates at
  exactly the C++ throw sites: `delay<0` → `Err("Bad deadcodedelay tag")` *before*
  the insert (matching C++); `type==NONE || addr.isInvalid()` → `Err("Bad flowoverride
  tag")` *before* the insert. Already-decoded elements remain installed in `self` on
  the error, exactly as the C++ leaves `this` partially populated when it throws
  mid-loop. The `addr.isInvalid()` arm — *not* covered by the porter's tests — is
  verified by `override_decode_flow_rejects_invalid_address`. `Comment::encode`'s
  `decode_comment_type` returns `Err` for an unknown type (the C++ `throw`), same
  partial-stream state. `CommentSorter::findPosition`'s "Dead op reaching
  CommentSorter" throw is the `op_block_info` `Err` seam.
- **Aliasing / ownership.** clean. `insertProtoOverride`'s C++ "delete pre-existing
  then setOverride then assign" is `set_override(true)` then `BTreeMap::insert`
  (drops the old Box) — observationally identical, "newest at a call point wins"
  preserved (verified `proto_override_insert_marks_and_merges`). The `mutable bool
  emitted` (C++ set through `const Comment*`) is a plain field; the sorter clones the
  comment into `commmap` (`setEmitted(false)` on the clone), matching the C++ which
  also resets `emitted` on each `setupFunctionList`. `Comment::decode` does not touch
  `uniq` (matching C++); `CommentDatabaseInternal::decode` recomputes it via
  `add_comment` (matching C++).

## CommentSorter::findPosition / setupFunctionList re-derivation (the high-risk core)
- The branch ladder (header-at-fad → op-at-or-after contains → backupOp on exact-addr
  match → previous-op contains (placed at order `0xffffffff`) → backupOp fallback →
  no-ops-at-all `setBlock(0,0)` → unplaced-header → drop) matches `comment.cc:270-325`
  line-for-line, including that `backup_op` is captured **only** inside the
  `OpProbe::At` arm after the contains check fails, and consulted **after** the
  previous-op arm. `OpProbe`/`prev_op` model the single C++ `opiter` against
  `endOpAll()`/`beginOpAll()`: an at-end probe over a non-empty tree decrements to the
  last op (the C++ `--opiter` from `endOpAll()`), and `prev_op` returns `None` only at
  `beginOpAll()`. Verified across all five placement outcomes by the porter's
  `sorter_*` suite plus the order-landmark stop.
- `setupFunctionList`'s single `Subsort subsort{pos=0}` carried across iterations
  (index/order overwritten by `findPosition`, `pos` monotonically advanced only on a
  successful placement) is reproduced exactly; `tp==0` early-returns and `tp` is an
  all-or-nothing gate, not a per-comment filter (verified
  `sorter_tp_zero_collects_nothing_else_collects_all_nonzero_types`).
- `setupOpList`(None→`opstop=stop`; Some→`upper_bound`), `setupBlockList`
  (`lower_bound(idx,0,0)`..`upper_bound(idx,MAX,MAX)`), `setupHeader`, `hasNext`
  (`start!=opstop`), `getNext` (`(*start).second; ++start`) all match; `get_next`'s
  `.expect()` on a misuse-only `start==None` is the faithful translation of the C++
  end-iterator-deref UB.

## Mechanical pass
- Blob-sha drift: all four in-scope C++ files match the checklist `cpp_blob_sha`
  exactly (checked via `git hash-object`). `port-audit` tool is not present in the
  workspace, so this was done manually as in prior reviews.
- grep over `comment.rs`/`overrides.rs` (+ their test modules):
  `todo!`/`unimplemented!`/`HashMap`/`HashSet`/`sort_unstable` → **none**.
- Bare `as` casts: 10, all in `overrides.rs`, all space-index/width conversions on
  non-negative space indices or post-checked delays; the load-bearing ones carry
  inline justification (overrides.rs:224-227, :243, :345, :526). `comment.rs` has no
  bare `as` casts. Consistent with the accepted standard for this port (w3-ir-block
  ACCEPTed with 153 uncommented mechanical casts).
- `cargo clippy -p kuna-decomp -- -D warnings`: **clean** (covers the in-scope library
  code). `--tests` surfaces one *pre-existing* `doc-lazy-continuation` warning in
  `verify_w3_ir_op.rs` (a W3 file, not touched by this branch, not in scope); the new
  `verify_w4_p0_override_comment.rs` test binary compiles under `-D warnings`.

## Findings
- F1 (loss): the C++ class `Override` lives in `override.{cc,hh}`, but `override` is a
  Rust reserved keyword, so the Rust module is named `overrides` (the type, constants,
  and every marshaling/element name — incl. `ELEM_OVERRIDE`/`<override>` id 223 — are
  preserved verbatim). Forced source-level rename, zero behavioral/wire impact;
  recorded for ledger completeness as **LOSS-048** (consistent with the porter's
  module-doc "NAMING LOSS"). Drives the ACCEPT-**WITH-LOSSES** verdict.
       cpp: decompiler/cpp/override.hh:50 (`class Override`)
       rust: rust/crates/kuna-decomp/src/overrides.rs:1-5 (module `overrides`, type `Override`)
- (LOW, not a finding): `uniq+1` / `pos+=1` use plain signed/unsigned `+` (debug-panic
  on overflow) where C++ is silent — unreachable on valid input (`uniq` starts at 0,
  65535 is only a probe sentinel). Documented here; no code change required.

## Adversarial tests added
`rust/crates/kuna-decomp/tests/verify_w4_p0_override_comment.rs` (committed on branch
as `73292a5`, 6 tests, all green):
- `add_comment_first_element_and_lower_funcaddr_predecessor_keep_uniq_zero` — the
  `iter==begin()` predecessor-divergence is inert (uniq stays 0).
- `add_comment_no_duplicate_walk_breaks_at_different_address` — backward-walk `break`
  boundary at a lower address; duplicate rejected after walking several non-dups.
- `clear_type_brackets_one_function_across_space_boundary` — half-open
  `[m_minimal,m_maximal)` clears one function only, across a space boundary.
- `comments_for_isolates_functions_in_different_spaces` — cross-space funcaddr
  range isolation.
- `override_decode_flow_rejects_invalid_address` — the `addr.isInvalid()` reject arm
  (previously uncovered).
- `override_decode_deadcode_high_index_grows_vector_and_flow_none_default` — decode
  vector-growth at a high space index; `getFlowOverride` NONE default.

## losses
LOSS-048 (the `Override` class is in the `overrides` module — `override` is a Rust
keyword; type and all wire names preserved).
