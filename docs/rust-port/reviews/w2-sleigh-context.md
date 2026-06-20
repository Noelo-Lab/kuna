# w2-sleigh-context

## Round 1

verdict: ACCEPT-WITH-LOSSES
verifier: independent verifier agent (Claude, not the porter)
date: 2026-06-11
branch: worktree-wf_6343a4ba-016-3 (worktree `.claude/worktrees/wf_6343a4ba-016-3`, reviewed at d53f716; adversarial tests committed as ef3e756)
cpp blob shas: all 4 in-scope files match the checklist entry (`git hash-object` verified: context.cc 272a3e37, context.hh abe1701d, globalcontext.cc ef2d8800, globalcontext.hh 2a9abad5)

gate: `cd rust && cargo test -p kuna-sleigh` -> ok. 15 passed (lib: 2 context
+ 13 globalcontext) + 6 passed (verifier tests `verify_w2_sleigh_context`),
0 failed.
clippy: `cargo clippy -p kuna-sleigh --tests -- -D warnings` -> clean
(workspace `disallowed-types` denies HashMap/HashSet per ADR 0002).
port-audit: `cargo run -p port-audit -- --item w2-sleigh-context` NOT
runnable — no `port-audit` crate exists in the workspace (carried from the
W1 reviews); blob-SHA check performed manually instead (all 4 match).

## Method

Re-derived the semantics of every function in `globalcontext.{hh,cc}` and
the ported subset of `context.{hh,cc}` (Token, FixedHandle) and diffed
against `git diff rust-port...worktree-wf_6343a4ba-016-3 -- rust/`
(context.rs +131, globalcontext.rs +1735). Because the entire database rides
on `partmap.hh`, the W1 `PartMap` (kuna-base/src/partmap.rs) was re-checked
against the C++ template specifically for the properties this item depends
on: `split` clones via `V::Clone` exactly where C++ does
default-construct-then-`operator=` (so `FreeArray::clone`'s mask-zeroing
lands at the same sites), `begin(pnt)` = `lower_bound` = `range(Included..)`,
`clearRange` boundary-split/erase-strictly-between semantics, and `bounds`
valid-code/out-param behavior. Six adversarial tests were then written
against derived C++-oracle traces (committed as
`rust/crates/kuna-sleigh/tests/verify_w2_sleigh_context.rs`).

Scope note: the port deliberately covers only `Token` + `FixedHandle` from
`context.{hh,cc}`; the parser-state machinery (ConstructState, ContextSet,
ParserContext, ParserWalker, ParserWalkerChange — ~70% of the item's C++
LOC) is deferred (finding F1 / LOSS-017). `SleighError` is correctly
represented by the pre-existing `KunaError::Sleigh` (ADR 0004; verified in
kuna-base/src/error.rs).

## Hunt list

- signedness: clean. All `int4 i < vec.size()` loops became iterators; the
  `sbit/(8*sizeof(uintm))` int/size_t unsigned-division nuance is documented
  at both use sites (non-negative inputs only, identical results);
  `mem.size` (uint4) promoted via `u64::from` before u64 arithmetic.
- integer widths: clean. `uintm -> u32` consistently; the C++ implicit
  truncation `uintm val = readUnsignedInteger(...)` (globalcontext.cc:351)
  transcribed as a commented `as u32`; `calc_mask(mem.size as i32)` matches
  the C++ implicit uint4->int4; encode side widens with `u64::from`.
- wrapping: clean with one note (F4). `endoff`/`tendoff` (globalcontext.cc:
  228,235) and `after.getOffset()-1` (globalcontext.cc:420) use wadd/wsub.
  The endian trim shifts use bare ops — proven non-wrapping whenever the
  C++ result is defined (a wrap there requires C++ to continue into an
  oversized-shift UB); pinned by verifier test
  `verify_w2ctx_tracked_value_wrapped_endoff_containment`.
- comparator totality: clean. `BTreeMap<Vec<u8>,_>` (total, memcmp order)
  and `PartMap<Address,_>` over the W1 Address `Ord` (total; space index
  then offset, transcribing C++ `Address::operator<`). No floats.
- iteration-order provenance: clean, per loop: `encode_context` over
  `variables` = std::map<string> memcmp order -> BTreeMap<Vec<u8>>
  byte-lex (pinned by `verify_w2ctx_encode_variable_order_unsigned_byte_lex`
  with a 0xC3-leading name); `encode` over database/trackbase =
  std::map<Address> -> PartMap BTreeMap<Address>; region walks =
  lower_bound-ordered map ranges; decode loops = stream order. No
  HashMap/HashSet anywhere in the diff.
- off-by-one/do-while/reverse iterators: clean. `for(;;)`+break decode
  loops transcribed as `loop`; the `biter = begin(addr2)` stop transcribed
  as `key >= addr2`, exactly the lower_bound frontier after `split(addr2)`;
  `aiter==biter` empty-range no-op verified
  (`verify_w2ctx_equal_range_region_set_is_noop_and_unmasked`); the
  first-entry-then-rest structure of getRegionToChangePoint preserved
  (`verify_w2ctx_change_point_first_entry_only_mask`). Reversed ranges are
  C++ UB (iterates past map end) — Rust paints nothing; unreachable
  in-tree (context.cc:248-252 guards the wrap before calling the region
  flavor) — note F5.
- erase-while-iterating: clean. Only `clearRange` (W1 PartMap:
  collect-then-remove of keys strictly between the boundary splits),
  identical visit/erase set.
- exception->Result partial-state parity: clean. `get_variable` errors
  propagate before any mutation in set_variable/set_variable_region/
  set_variable_default (same as the C++ throw-before-mutate);
  decode/decode_context/decode_from_spec error paths leave the same
  partially-painted database and the same unclosed-element stream state;
  register_variable error ordering (empty-check, then fit-check, then
  mutate) matches globalcontext.cc:368-383.
- aliasing: the one structural redesign. C++ getRegionForSet/
  getRegionToChangePoint hand back `vector<uintm *>` and the caller writes
  through the pointers afterwards; safe Rust can't hold multiple `&mut`
  into the map, so the port invokes a callback per blob in the same order.
  Verified equivalent: within getRegionForSet nothing reads masks during
  the walk; within getRegionToChangePoint the stop-test reads only masks of
  not-yet-visited entries and the callback touches only `array`, never
  `mask`, so interleaving "mark, mutate" per entry produces the identical
  final state. Exercised by verifier tests 2 and 3.

## Findings

  - F1 (major; resolved by LOSS-017): ~70% of context.{hh,cc} is not ported
    — ConstructState, ContextSet, ParserContext, ParserWalker,
    ParserWalkerChange and all their methods are deferred to the sleigh
    decode-engine item, but checklist item w2-sleigh-core's `modules` list
    does not include context.{cc,hh}, so the deferred code had no owning
    item and would have escaped every later verifier's in-scope diff. The
    deferral itself is sound (the types are built on Constructor/
    TripleSymbol/OperandSymbol/Translate, all unported) and consistent with
    the crate plan (lib.rs assigns ParserContext to sleigh.{hh,cc}), but it
    was tracked only in module docs. Now ledgered as LOSS-017.
       cpp: decompiler/cpp/context.hh:61-292, decompiler/cpp/context.cc:22-323
       rust: rust/crates/kuna-sleigh/src/context.rs (module docs)
  - F2 (major; resolved by LOSS-018): ContextCache is redesigned — no
    encapsulated `ContextDatabase *` (no `getDatabase()`; the database is a
    per-method parameter) and cache hits re-fetch the blob via the
    single-lookup `get_context` instead of serving the C++ cached raw
    pointer. Through-cache flows are equivalent (the three-test
    invalidation is transcribed exactly, including the space-less second
    and third offset tests, and in-place paints reaching into the cached
    range from below are observed identically), but a DIRECT database
    mutation that inserts a split inside a live cached range diverges: C++
    serves the stale pre-split blob, the port serves the fresh one. The
    porter's module docs claim this was "reported as a loss by the porting
    item" — no such entry existed (the branch touches only rust/). Appended
    by this review as LOSS-018; pinned by canary test
    `verify_w2ctx_cache_direct_mutation_divergence_canary`.
       cpp: decompiler/cpp/globalcontext.hh:317-331, decompiler/cpp/globalcontext.cc:556-616
       rust: rust/crates/kuna-sleigh/src/globalcontext.rs (ContextCache)
  - F3 (minor, no action): `register_variable` on a range whose negative
    shift exceeds startbit (e.g. sbit=0, ebit=40) panics in debug inside
    `ContextBitRange::new` (`(startbit+shift) as u32` wraps huge, shift
    overflow) before the does-not-fit error can be returned. C++ reaches
    its LowlevelError only by passing through the same negative-shift UB
    first (globalcontext.cc:40 evaluated at :374, before the :376 check),
    so there is no defined oracle behavior to match; ADR 0004 sanctions
    panic-on-C++-UB. Release builds mask the shift and return the error.
       cpp: decompiler/cpp/globalcontext.cc:33-41,368-377
       rust: rust/crates/kuna-sleigh/src/globalcontext.rs (ContextBitRange::new)
  - F4 (minor, no action): `get_tracked_value` endian trim shifts use bare
    `-`/`*`/`>>`; on inputs where those could wrap or overflow (tracked
    registers > 8 bytes, zero-size queries far from the tracked start) the
    C++ is already in oversized-shift UB, so the debug-Rust panic is the
    ADR 0004 outcome. All C++-defined paths — including the wrapped-endoff
    containment oddity — are reproduced exactly (verifier test 4). The
    upstream big-endian trim `res >>= 8*(tendoff - mem.offset)` (arguably
    should be `tendoff - endoff`; correct only for 1-byte or whole-register
    reads) is transcribed as-is, C++-is-the-spec; not in
    upstream-bugs.md — left to the w2-sleigh-core review, where a real
    consumer exists.
       cpp: decompiler/cpp/globalcontext.cc:239-246
       rust: rust/crates/kuna-sleigh/src/globalcontext.rs (get_tracked_value)
  - F5 (note): `get_region_for_set` with addr2 < addr1 (both valid): C++
    walks past `database.end()` (UB); Rust breaks immediately and paints
    nothing. Unreachable in-tree: the only range-flavor caller chain
    (context.cc applyCommits -> ContextCache::setContext(addr1,addr2)) is
    guarded by the explicit wrap check at context.cc:248-252.
       cpp: decompiler/cpp/globalcontext.cc:424-444
       rust: rust/crates/kuna-sleigh/src/globalcontext.rs (get_region_for_set)
  - F6 (process note): `port-audit` still does not exist in the workspace
    (carried from W1 reviews F4 onward); blob SHAs checked manually.

## Adversarial tests

`rust/crates/kuna-sleigh/tests/verify_w2_sleigh_context.rs` (commit
ef3e756 on the port branch), all passing:

- verify_w2ctx_default_pointset_rezeroed_per_set_child — the default blob
  is re-zeroed for EVERY `<set>` child (globalcontext.cc:354-358): with two
  children only the last survives; catches hoisting the zeroing.
- verify_w2ctx_change_point_first_entry_only_mask — change-point paints
  mark the explicitly-set mask only on the first entry; later paints flow
  through intermediate value-only splits and stop exactly at genuine
  change points of the same variable.
- verify_w2ctx_equal_range_region_set_is_noop_and_unmasked — equal-range
  region set paints nothing AND leaves the forced split unmasked, so later
  paints flow through it; catches mark-before-stop-test ordering bugs.
- verify_w2ctx_tracked_value_wrapped_endoff_containment — `endoff`
  wraparound (mem at the top of the u64 offset space) makes containment
  pass and the defined LE/BE trims produce 0x11/0x7788; catches
  checked/saturating substitutions for the C++ unsigned wrap.
- verify_w2ctx_encode_variable_order_unsigned_byte_lex — `<set>` elements
  emit in unsigned-byte-lexicographic name order (0xC3-leading name sorts
  after all ASCII), the std::map<string> provenance.
- verify_w2ctx_cache_direct_mutation_divergence_canary — pins LOSS-018:
  direct db mutation splitting inside a live cached range serves the fresh
  value in Rust where C++ serves the stale pre-split blob.

losses: LOSS-017 (context parser-state machinery deferred; appended by this
review), LOSS-018 (ContextCache re-fetch divergence + dropped getDatabase;
appended by this review).
