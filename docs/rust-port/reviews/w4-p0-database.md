# w4-p0-database
verdict: ACCEPT-WITH-LOSSES
verifier: claude (independent verifier, round 2)
date: 2026-06-11
gate: `cd rust && cargo test -p kuna-decomp` -> PASS (231 lib + every test binary;
      12/12 in verify_w4_p0_database, incl. the formerly-failing F1 evidence test)

## Scope

C++ in scope: `decompiler/cpp/database.{cc,hh}` (4.5k LOC), blob shas verified to
match the checklist exactly (`database.cc` b73d17b…, `database.hh` 45e1bf5…).
Rust diff: `git diff rust-port...worktree-wf_a6e29a1b-137-3 -- rust/`
  -> `rust/crates/kuna-decomp/src/database.rs` (+3458 LOC),
     `rust/crates/kuna-decomp/tests/verify_w4_p0_database.rs` (verifier tests).
Round-2 repair under review: commit `7bf727b` "repair fixes" (7 +/2 -, the F1 fix only).

## Round-1 blocker resolution

- **F1 (round-1 MAJOR) FIXED.** `addMapInternal`'s overflow check now computes
  `let last_offset = (addr + (sz - 1) as i64).get_offset();`
  (`database.rs:1601`), routing through kuna-base `Add<i64> for &Address` ==
  C++ `Address::operator+` == `base->wrapOffset(offset+off)`. A symbol
  straddling the top of a sub-64-bit space now wraps below its start and is
  rejected exactly as `database.cc:1883-1889`. The repair is surgical (only the
  overflow line changed) and introduces no regression: `sz >= 1` (guaranteed by
  `addSymbolInternal`'s `getSize() < 1` check), so `(sz - 1) as i64 >= 0` never
  underflows. The round-1 evidence test now passes.
       cpp: decompiler/cpp/database.cc:1883-1889 (address.hh:454-455)
       rust: rust/crates/kuna-decomp/src/database.rs:1601
       test: verify_w4_p0_database::w4_p0_database_addmap_overflow_uses_space_mask (PASS)

## Hunt list (re-derived independently, round 2)

- **Signedness**: clean. `find_closest_fit`'s signed `olddiff`/`newdiff` test
  (`(olddiff<0 && newdiff>olddiff) || (olddiff>=0 && newdiff>=0 && newdiff<olddiff)`)
  transcribes `database.cc:2335-2336` with int4 arithmetic; re-pinned by
  `find_closest_fit_negative_diff_loses`. `catindex:u16` (C++ `uint2`) truncation
  on `category[cat].len() as u16` matches C++ `catindex = list.size()`. int4 loop
  indices vs sizes use explicit `as int4`/`as usize`.
- **Integer widths**: clean. `(b-a)+1 as int4`, `get_last`=`off+size-1`,
  `symbolId = ID_BASE + ((uniqueId&0xffff)<<40) + next` reproduce C++ widths.
  `find_container`/`find_overlap` `end = off + (size as uintb) - 1`: `size as uintb`
  sign-extends i32->u64 identically to the C++ `uintb + int4` promotion (matches
  even for the impossible negative-size case).
- **Wrapping**: clean post-F1. `find_container`/`find_overlap`/`get_last` `end`
  use raw 64-bit `wrapping_add` because the C++ sites use raw `uintb` arithmetic
  (NOT `Address+`), whereas the addMap overflow check now correctly uses
  `&Address + i64` (wrapOffset). `equate_is_value_close` uses
  `wrapping_neg`/`wrapping_add(1)`/`wrapping_sub(1)`. `hashScopeName`'s `nm[i]`
  signed-char promotion difference is masked away by `crc_update`'s `& 0xff`
  (verified: only the low byte of `val` is consulted) — no divergence.
- **Comparator totality**: clean. `EntrySubsort` derives Ord (useindex,useoffset)
  == `EntrySubsort::operator<`; `EntrySubsort(true)`'s C++-uninitialized
  `useoffset` is made deterministic (`0`) in Rust and is never read (useindex
  0xffff dominates) — fix-of-UB with no observable effect. `NameKey` derives Ord
  (name,dedup) == `SymbolCompareName`. No floats, no partial_cmp.
- **Iteration-order provenance**: clean. `nametree`/`multi_entry_set` (C++
  `set<Symbol*,SymbolCompareName>`) -> `BTreeMap<NameKey,_>`; `children`/`idmap`
  (C++ `map<uint8,_>`) -> `BTreeMap<uint8,_>`. No HashMap/HashSet in the diff
  (grep clean). `resolve_child_scope` linear scan and `find_by_name` walk iterate
  ascending key order == C++ ScopeMap/SymbolNameTree order.
- **Off-by-one / do-while / reverse iterators**: re-derived.
  `makeNameUnique`'s C++ `do {} while(uniqid==0xffffffff)` -> `loop { ...; pop();
  ... }`: the first-decrement / break-on-`first_key` / digCount==2-vs-5 arms all
  match; the `lower_bound(bound)-1` (`<`) vs Rust `range(..=bound).next_back()`
  (`<=`) boundary differs only if `nm_x99999` with dedup 0xffffffff exists
  (2^32 collisions — unreachable). Newly pinned: the digCount==5 x-form
  continuation (`make_name_unique_continues_xform`). `getScopePath` reverse-fill,
  `buildUndefinedName` `lower_bound("$$undefz");--iter`, and the `find*` reverse
  subsort walk (`--res.second` -> `it.next_back()`) all re-checked and tested.
- **Erase-while-iterating**: clean. `assignDefaultNames` (C++ `++iter` before
  rename) -> collect undefined ids up-front then rename in nametree order;
  `build_default_name` never yields a `$$undef` name so no re-visitation, and the
  collected (name,dedup) order == C++ visit order, so the by-ref `base` counter
  is consumed identically. `removeSymbolMappings`/`removeSymbol` erase by stable
  handle (rangemap idx / dynamicentry slot set to None; slots never reused),
  matching std::list iterator stability. `removeSymbol` category-tail trim
  (`while matches!(list.last(), Some(None))`) == C++ `while(!empty && back==null)
  pop_back`; newly pinned by `category_index_reused_after_tail_removal`.
- **Exception -> Result partial-state parity**: clean. `addSymbolInternal`
  increments `next_unique_id` before the C++ try-block; the port increments then
  `?`-returns, leaving the counter bumped (matches the C++ catch path) and an
  orphaned Symbol in the arena (vs C++ `delete sym`) — not observable through the
  public API (the orphan is absent from nametree and every query). `attach_scope`
  duplicate-id RecovError matches.

## Findings

None at blocker/major severity. (Round-1 F1 resolved; see above.)

## Notes (non-blocking)

- N1 (minor, carried from round 1): `resolve_child_scope` numeric-id branch.
  C++ `istringstream >> key` partial-reads malformed octal ("08" -> 0, stopping
  at '8'); the port's `parse_auto_base_u64` returns `None` on the radix-8 parse
  error. Divergence only for a child-scope path element that is a *malformed
  numeric string* — the port doc-comment acknowledges it. Not a blocker.
       cpp: decompiler/cpp/database.cc:1331-1340
       rust: rust/crates/kuna-decomp/src/database.rs:1330-1339
- N3 (acceptable per stated invariant, carried from round 1): `ScopeResolve`
  uses `BTreeMap<Address,(Address,ScopeId)>` keyed by first-address and `find`
  returns the largest `first<=addr` containing addr, vs the C++ `rangemap`
  returning the first (smallest-first) covering range. Identical for *disjoint*
  namespace ranges, which is the documented invariant (non-global namespace
  ranges from `addRange`/`setRange` do not overlap in practice). Diverges only
  for overlapping namespace ranges, which the database does not produce.
- L1 -> LOSS-046 (deferral): `add_map` defers the join-address piece-expansion
  arm (`database.cc:1161-1183`, `glb->findJoin`) to W5 — a join-addressed Symbol
  gets one SymbolEntry instead of per-piece `precislo`/`precishi` entries. The
  `JoinRecord` table is a W5 subsystem absent at this boundary. Recorded and
  cited (round-1's N2 framing: "verify this is captured as a declared loss").
       cpp: decompiler/cpp/database.cc:1161-1183
       rust: rust/crates/kuna-decomp/src/database.rs:1676-1740
- Acknowledged seams / out-of-scope (no behavioral claim): `retypeSymbol`,
  `setAttribute`/`clearAttribute`, `encode`/`decode`, `getSizedType`/`updateType`,
  `printEntry` — encode/decode + type/funcdata-subsystem methods for later waves.

## Mechanical pass

- `cargo run -p port-audit -- --item w4-p0-database`: port-audit crate is not
  present in this worktree's workspace (future tooling). Blob-sha drift checked
  by hand instead: both in-scope cpp blobs match the checklist `cpp_blob_sha`.
- grep `todo!|unimplemented!|HashMap|HashSet|sort_unstable`: none in database.rs.
- bare `as` casts: all numeric width conversions (int4/uintb/uint4/u16/usize/i64),
  inspected individually; no unjustified pointer/lossy casts. The one
  signedness-sensitive site (`hashScopeName` `b as uint4`) is masked by
  `crc_update & 0xff` — verified harmless.
- `cargo clippy -p kuna-decomp --lib --test verify_w4_p0_database -- -D warnings`:
  clean. (Workspace-wide clippy fails only in *other* items' W3 test files —
  `verify_w3_ir_jumptable.rs`, `verify_w3_ir_op.rs`, `verify_w3_ir_funcdata_varnode.rs`
  — none in this item's scope; the diff touches only the two database files.)

## Adversarial tests

`rust/crates/kuna-decomp/tests/verify_w4_p0_database.rs` (committed on the branch,
7e4953b). Round-1 set (9, all now PASS incl. the F1 evidence test) plus round-2
additions (3):
- w4_p0_database_make_name_unique_continues_xform   (digCount==5 x-form +1 roll)
- w4_p0_database_category_index_reused_after_tail_removal  (pop_back + reuse;
  middle-hole-not-backfilled)
- w4_p0_database_query_properties_finalscope_flags  (mapped|addrtied|persist |
  property bits when an owning scope but no SymbolEntry covers the range)

## Verdict

ACCEPT-WITH-LOSSES. The sole round-1 blocker (F1, the address-space-mask overflow
check) is fixed by a minimal, correct, regression-free change, and the full item
gate is green (231 lib + 12 verifier tests). The remaining departures are
non-blocking: N1 (malformed-numeric child-scope id, minor), N3 (overlapping-
namespace resolution, acceptable per the documented disjoint invariant), and the
join-address piece-expansion seam, now recorded and cited as LOSS-046.

losses: LOSS-046
