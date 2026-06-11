# w1-base-space-address

## Round 1

verdict: ACCEPT-WITH-LOSSES
verifier: independent verifier agent (Claude, not the porter)
date: 2026-06-11
branch: worktree-wf_cea42066-041-13 (worktree `.claude/worktrees/wf_cea42066-041-13`, reviewed at ec80296; port commit 325cbc7; adversarial tests committed as 7a34a97)
cpp blob shas: all 4 in-scope files match the checklist entry (`git hash-object` verified: space.cc dbaa2e77, space.hh bc88fcd6, address.cc 3a3d33a2, address.hh 510432e5)

gate: `cd rust && cargo test -p kuna-base --test golden_addrsort` -> ok.
1 passed (1296-row C++ oracle pair matrix from
`tests/golden/vectors/addrsort.csv`, emitted by `kuna_goldengen.cc` at C++
rev 87016be — a genuine oracle replay, all `<`/`==`/`<=` decisions match).
Full-crate `cargo test -p kuna-base` is currently red **only** because of
the w1-base-marshal verifier's two pinned-failing F1 tests
(`verify_w1_base_marshal.rs`, `getIndexedAttributeId`) — that item's
verdict is REJECT and its repair is out of scope here; every space/address
test (89 lib + 1 golden + 5 new verifier tests) passes.
clippy: `cargo clippy -p kuna-base --all-targets -- -D warnings` -> clean
(workspace `disallowed-types` denies HashMap/HashSet per ADR 0002).
port-audit: `cargo run -p port-audit -- --item w1-base-space-address` NOT
runnable — no `port-audit` crate exists in the workspace yet (pre-existing
gap, flagged since the w1-base-foundation review); blob SHA check performed
manually instead (matches, above).
mechanical grep over the item diff: no `todo!`, `unimplemented!`,
`HashMap`, `HashSet`, or `sort_unstable`.

## Method

Re-derived every function in space.cc/.hh and address.cc/.hh from the
vendored C++ and diffed against
`git diff rust-port...worktree-wf_cea42066-041-13 -- rust/` (space.rs
+1836, address.rs +2258, golden_addrsort.rs +172; marshal.rs in the same
commit belongs to the marshal item). The port also vendors the **minimal
AddrSpaceManager** lookup core plus `FspecSpace`/`IopSpace`; I verified
those against their actual C++ homes (translate.cc:309-657, fspec.cc:2118,
op.cc:33-64, op.hh:37-41) even though they are outside the item's four
declared modules, since they ship in this diff and the gate depends on
them. The `decode_varnode_attributes` inline was checked against
`VarnodeData::decode/decodeFromAttributes` (pcoderaw.cc:23-55).
Arithmetic-promotion questions were settled with a standalone C++
micro-oracle transcribing the exact upstream expressions
(`g++ -std=c++11`, x86-64): `calcScaleMask` for wordsize {0,1,2,3,0x80}
across address sizes, and `wrapOffset` in the negative-modulus regime
(`mod = i64::MIN`, `res += mod` wrap). One micro-oracle case exposed F1;
the wrapOffset case confirmed the port (`wrap_offset(u64::MAX) ==
0x7fffffffffffffff` on the as=7/ws=0x80 space, matching gcc's wrap of the
C++ signed-overflow UB, pinned by a new test).

## Hunt list

- signedness: **clean** — every mixed C++ comparison found is transcribed
  deliberately with a comment: `dist >= size` sign-extends int4 size
  (`as i64 as u64`, space.rs:586, address.rs:311), `baselist.size() <=
  index` converts index to usize (space.rs:1479/1499/1647), `(uint4)size`
  in calc_mask (address.rs:1587), negative `numBits`/`sa` convert huge
  (address.rs:1508, 1615), `addressSize != size` (space.rs:1525). The
  size=-1 overlap quirk is pinned by a new adversarial test.
- integer widths: **F1** — `(wordsize-1)` in calcScaleMask is uint4
  arithmetic in C++ and was ported as a 64-bit wrap; otherwise clean
  (uintb->uintm truncation in SeqNum::decode, intb->int4/uint4 truncations
  in decode attributes, strtoul->int4 in get_offset_size, int4 cut in
  printRaw all transcribed with comments).
- wrapping: **clean** — `w*` helpers used throughout per ADR 0003;
  wrapOffset's `(intb)(highest+1)` / `res += mod` path verified against
  the micro-oracle including the i64::MIN modulus; `longest_fit`'s
  `last + 1 - offset` wrap at u64::MAX verified correct-by-wrap (pinned by
  test); Address +/- wrap through wrapOffset; out-of-range shift counts
  resolve via wrapping_shl/shr == x86 masking where C++ is UB (getMask,
  calc_int_min, extend_signbit, sign_extend_sized — all commented).
- comparator totality: **clean with notes** — Address `Ord` is total
  (Null < Spc(index, then offset) < Max rank model) and replays the C++
  `operator<`/`<=` pair exactly on all 1296 golden rows; the
  distinct-spaces-sharing-an-index fallthrough (F4, note) and the SeqNum
  Eq(uniq)-vs-Ord(pc,uniq) mismatch (F3, note) are exact C++
  transcriptions, documented in module docs, and only observable outside
  the one-manager invariant. Range Eq is defined as `cmp == Equal`
  (tree-equivalence, `last` excluded) so Eq/Ord are mutually consistent.
- iteration-order provenance, per loop: **clean** — RangeList tree:
  BTreeSet with the transcribed (space-index, first) comparator ==
  std::set order (insert/remove/merge/print/encode walks all ascending);
  name2space: BTreeMap<String> == std::map<string> byte-lexicographic;
  shortcut2space: BTreeMap<i32> keyed by sign-extended char ==
  std::map<int4> (cxx_char_to_int, pinned by test with a 0xC3-leading
  name); baselist: Vec index order == C++ vector; decode loops follow
  stream attribute order. No HashMap/HashSet anywhere (clippy-denied).
- off-by-one / do-while / reverse iterators: **clean** — every
  `upper_bound(probe)` + conditional `--iter` C++ idiom maps to
  `range(..=probe).next_back()` with the same qualification test
  (insertRange/removeRange/inRange/getRange/longestFit/
  getLastSignedRange — each re-derived); longest_fit keeps the do-while
  at-least-once shape by starting the range at `Included(first_range)`;
  leastsigbit_set/mostsigbit_set/count_leading_zeros keep do-while via
  loop+break-after-body; bit_transitions' empty range for sz<=0 matches
  the C++ for-loop.
- erase-while-iterating: **clean** — C++ `tree.erase(iter1++)` walks in
  insertRange/removeRange become collect_overlap (snapshot of
  `[iter1,iter2)`) + remove loop; proven equivalent: the predecessor
  qualification (`spc` pointer + `last >= first`) is identical, elements
  strictly inside `(probe1, probe2]` are collected in the same ascending
  order, and removeRange's mid-loop trimmed re-inserts land strictly
  outside the remaining C++ iteration range on both sides (keys < probe1
  or > probe2), so visit set and final tree agree. Boundary behavior
  (merge at overlap-by-one, no merge on adjacency, equal-first merge,
  top-edge removal without wrap-around insert) pinned by new tests.
- exception -> Result partial-state parity: **clean** — insert_space
  mutates the special slots before the duplicate-id/name errors exactly
  like C++ (including the quirk that a duplicate const space overwrites
  `constantspace` and *then* fails); decode_basic_attributes /
  decode_overlay `?`-propagate leaving the same partially-assigned fields;
  panics are used only where C++ is UB (null/~0 sentinel deref in
  printRaw/encode/SeqNum::encode, negative space index, missing overlay
  base, empty baselist in getNextSpaceInOrder, default-space null deref in
  getDefaultSize), per ADR 0004, each marked "(C++ UB)" at the site.

## Findings

- F1 (minor): `calc_scale_mask` computes `highest = mask*ws + (ws-1)`
  with the `(ws-1)` term wrapping at 64 bits; the C++ `(wordsize-1)` is
  **uint4** arithmetic that zero-extends into the uintb sum. For
  wordsize==0 the C++ yields `highest == 0x00000000FFFFFFFF` (micro-oracle
  verified, any addressSize); the Rust yields `u64::MAX`.
     cpp: decompiler/cpp/space.cc:38
     rust: rust/crates/kuna-base/src/space.rs:297
     claim/observed: module claims exact transcription of the wrapping
     arithmetic; observed 64-bit wrap where C++ wraps at 32.
     repro: `cargo test -p kuna-base --test verify_w1_base_space_address
     -- --ignored` -> `verify_calc_scale_mask_wordsize_zero_uint4_wrap_f1`
     fails (left: 18446744073709551615, right: 4294967295).
     severity rationale: only reachable with wordsize==0 — no in-tree
     constructor call, spec, or oracle datatest produces it (SLEIGH word
     sizes are >= 1); behavior on all real inputs is identical (pinned for
     ws in {1,2,0x80} against the micro-oracle). Suggested repair:
     `highest.wmul(ws as u64).wadd(self.wordsize.wrapping_sub(1) as u64)`.
- F2 (minor): zero `// cast:` justification comments in space.rs /
  address.rs against the protocol's mechanical clause; the sibling xml.rs
  established the convention. Mitigating: the truncating/sign-changing
  casts all carry prose justifications ("intb -> uint4 truncating
  conversion as in C++", "mixed comparison: ...", "(C++ UB)"), and I
  audited each against the C++ implicit conversion it transcribes —
  no behavioral mismatch found. Same convention gap as foundation F1 /
  marshal F2; fix opportunistically.
     cpp: decompiler/cpp/address.cc:139 (representative explicit C++ cast)
     rust: rust/crates/kuna-base/src/space.rs, src/address.rs (throughout)
- F3 (note): `SeqNum` PartialEq compares `uniq` only while Ord orders by
  (pc, uniq) — an exact transcription of the C++ operator pair, documented
  in the module docs; violates Rust's Eq/Ord consistency convention for
  out-of-contract inputs (two SeqNums sharing uniq across functions), but
  BTree containers consult only Ord, matching std::set on the C++ side.
     cpp: decompiler/cpp/address.hh:148-158
     rust: rust/crates/kuna-base/src/address.rs:650-676
- F4 (note): `Address::cmp` falls through to the offset comparison when
  two *distinct* spaces share an index; C++ `operator<` treats such
  addresses as mutually not-less (and `operator<=` as mutually not-<=,
  which is not even a partial order). Divergence is only observable
  outside the one-manager invariant (a registered index identifies a
  unique space); the Rust choice keeps Ord total. Documented in module
  docs.
     cpp: decompiler/cpp/address.hh:406-447
     rust: rust/crates/kuna-base/src/address.rs:492-518
- F5 (note): fields C++ leaves uninitialized are zero-initialized
  (Address() offset, SeqNum order, VarnodeData-scan offset when no space
  attribute, the partial decode constructor's name/size/index) — reads of
  those are UB in C++; deterministic zeroes are strictly tamer and each
  site is commented.
     cpp: decompiler/cpp/address.hh:294-296, decompiler/cpp/pcoderaw.cc:33-41
     rust: rust/crates/kuna-base/src/address.rs:125-131, 447-473
- F6 (note, loss accounting): the deferred Translate/JoinRecord-dependent
  paths (JoinSpace virtuals, FspecSpace/IopSpace printRaw + fspec encode
  payloads, register-name branches of read/decode/Range, renormalize,
  SpacebaseSpace, the manager's join machinery) return explicit "not yet
  ported" errors — except `AddrSpace::read`, where a register name
  silently parses through the C++ *catch* branch (parity with C++ for an
  unknown register; real registers need Translate). These departures were
  described in module docs but had **no losses.md entry**; appended as
  LOSS-012 (verifier append, per protocol).
     cpp: decompiler/cpp/space.cc:255-298, 454-650; fspec.cc:2124-2169;
     op.cc:41-59; translate.cc:659-937
     rust: rust/crates/kuna-base/src/space.rs:744-775 and the
     Join/Fspec/Iop match arms; src/address.rs:364-371, 447-473, 706-736
- F7 (note): the minimal `AddrSpaceManager` (translate.cc, outside the
  item's declared modules) was verified anyway: insertSpace's check order
  and partial-state-on-error, assignShortcut's collision walk (27-way
  exhaustion reuses 'z' *without* remapping shortcut2space — pinned by
  test), the signed-char shortcut keys, set_default_*_space index
  validation (negative index converts huge and errors, as in C++), and
  getNextSpaceInOrder's null/~0 sentinels all match. The full Translate
  port should absorb this code; until then its review home is this file.
     cpp: decompiler/cpp/translate.cc:309-657
     rust: rust/crates/kuna-base/src/space.rs:1234-1657

## Adversarial tests

`rust/crates/kuna-base/tests/verify_w1_base_space_address.rs`
(commit 7a34a97; 5 passing + 1 #[ignore]d failing pin for F1):

- verify_calc_scale_mask_wordsize_scaling_cpp_oracle (micro-oracle pins
  for ws in {2, 0x80} incl. the 64-bit-wrap highest and the
  negative-modulus wrapOffset values)
- verify_calc_scale_mask_wordsize_zero_uint4_wrap_f1 (**#[ignore]d,
  fails when run — F1**)
- verify_rangelist_boundary_merge_and_top_edge (overlap-by-one merge vs
  adjacency non-merge, equal-first merge, split removal, u64::MAX
  top-edge removal without wrap-insert, wrapped longest_fit chain,
  in_range size==0 wrap)
- verify_shortcut_collision_walk_and_signed_char_key (26-slot walk, 27th
  reuses 'z' without remap, uppercase lowering, "register" -> '%',
  0xC3-leading name keyed at sign-extended -61)
- verify_addrspace_read_strtoul_quirks (octal base-0, hex size/offset
  suffixes, bare "0x" endptr, leading-'+' early return, wordsize scaling)
- verify_address_overlap_signedness_and_pointer_identity (size=-1
  sign-extension quirk, negative skip, value-equal-but-distinct space
  pointers)

## Losses

LOSS-012 (appended this round): Translate-/JoinRecord-dependent paths of
space/address deferred to the pcoderaw (kuna-num) and sleigh waves; the
golden gate's SpacebaseSpace stand-in is comparator-equivalent. No other
divergences accepted.

F1 and F2 are repair-when-touched minors: F1's one-line fix
(`wordsize.wrapping_sub(1) as u64`) should land with the next change to
space.rs and flip the #[ignore]d pin to a passing test.

## Round 2

verdict: ACCEPT-WITH-LOSSES
verifier: independent verifier agent (Claude, round-2, not the porter and
not the round-1 verifier session)
date: 2026-06-11
branch: worktree-wf_cea42066-041-13 (worktree
`.claude/worktrees/wf_cea42066-041-13`, reviewed at 31b53dd; round-2
adversarial tests committed as 3a8df4e)
cpp blob shas: re-hashed in the main tree, all 4 match round 1
(space.cc dbaa2e77, space.hh bc88fcd6, address.cc 3a3d33a2,
address.hh 510432e5)

### What changed since round 1

Nothing in this item's scope.  The repair commit 60bcd4d
("marshal+space-address: repair round fixes") touched **only**
`rust/crates/kuna-base/src/marshal.rs` — the F1..F5 bullets in its message
are the *marshal* item's findings.  `git diff 7a34a97..31b53dd -- src/space.rs
src/address.rs tests/golden_addrsort.rs tests/verify_w1_base_space_address.rs`
is empty; space.rs/address.rs were last touched by the port commit 325cbc7.
Consequences:

- **This item's F1 (minor) is unrepaired.** Confirmed live: `cargo test -p
  kuna-base --test verify_w1_base_space_address -- --ignored` still fails
  the `verify_calc_scale_mask_wordsize_zero_uint4_wrap_f1` pin (Rust
  highest == u64::MAX vs C++ 0xFFFFFFFF for wordsize==0).  Still minor:
  wordsize==0 remains unreachable in-tree (re-checked: every constructor /
  decode / spec path supplies ws >= 1).
- **F2 (minor) is unrepaired** for this item: still zero `// cast:` markers
  in space.rs/address.rs (the repair added them to marshal.rs only).
- **Round 1's full-crate caveat is resolved**: with the marshal repair in,
  `cargo test -p kuna-base` is now fully green (90 lib + all integration
  targets; the only ignored tests are deliberate pins).

### Re-verification performed (independent of round 1)

Gate: `cd rust && cargo test -p kuna-base --test golden_addrsort` -> ok
(1 passed, 1296-row oracle matrix).
Clippy: `cargo clippy -p kuna-base --all-targets -- -D warnings` -> clean
(including the new round-2 test target).
Mechanical: re-grepped the full item diff
(`git diff rust-port...worktree-wf_cea42066-041-13 -- rust/`): zero
`todo!`/`unimplemented!`/`HashMap`/`HashSet`/`sort_unstable`.
LOSS-012 confirmed present in `docs/rust-port/losses.md:123`.

Fresh re-derivation spot-checks (C++ read independently, not from the
round-1 text): calcScaleMask + wrapOffset (incl. proving the
`(intb)(highest+1)` modulus can never be 0 or -1 for any reachable
highest, so wrem div-by-zero/MIN%-1 is unreachable); Address::containedBy/
justifiedContain/overlap/isContiguous (sign-extending `(sz-1)` adds,
truncating `(int4)` returns, `dist >= size` mixed comparison — all
transcribed with comments); RangeList insertRange/removeRange vs the
collect_overlap restructure (re-proved: predecessor qualification
identical, [ub(probe1), ub(probe2)) walk identical, removeRange's
mid-loop re-inserts land outside the remaining C++ iteration range — the
`b > last` insert can only happen on the final visited element);
merge/inRange/getRange/longestFit/getLastSignedRange (upper_bound
step-back idioms, do-while at-least-once shape, non-chaining inRange);
Range Ord/Eq (space-index+first only, matches std::set<Range>);
Range::decodeFromAttributes error order; and the **entire BitRange class**
(address.cc:630-868) plus the uintb sign_extend/signbit_negative/
pcode_left/pcode_right/calc_int_min helpers, which round 1 re-derived but
never pinned with tests.  Promotion questions settled with a fresh
standalone micro-oracle (/tmp/oracle_r2.cc, g++ -std=c++11 -O0, x86-64)
transcribing the verbatim upstream code: getMask's int4-vs-size_t numBits
comparison + the UB `res <<= -1` on the undefined range, the BE/LE
overlapTest matrix, every BitRange mutator, and the sign_extend size
clamps with negative/oversized sizes.  All 30 oracle rows match the port.

### Hunt list (round 2)

- signedness: re-confirmed clean on fresh anchors — getMask
  (address.rs:1508), pcode_right/left (1615/1625), sign_extend_sized size
  clamps (1693-1694), overlap `dist >= size` (311), in_range
  offset+size-1 (1094); all carry mixed-comparison comments and now have
  oracle-pinned tests.
- integer widths: F1 stands (only known width divergence, unreachable
  input); BitRange is int4 arithmetic throughout in both languages.
- wrapping: clean — wrap_offset modulus edge cases proven unreachable;
  BitRange uses plain i32 ops where C++ int4 overflow is UB (debug panic
  is tamer, per ADR 0004).
- comparator totality: clean — Range Ord re-read (space index, first);
  BTreeSet::insert keeps the existing element exactly like std::set.
- iteration-order provenance: clean — all RangeList walks ascending
  BTreeSet order == std::set; merge iterates op2 ascending.
- off-by-one / do-while: clean — longest_fit starts Included(first_range)
  (at-least-once); leastsigbit_set/mostsigbit_set loop+break re-checked.
- erase-while-iterating: clean — collect_overlap equivalence re-proved
  from the C++ (see above).
- exception->Result partial-state: clean — Range::decode_from_attributes
  error order (no-space, !seen_last default, illegal-range) matches
  address.cc:340-352.

### Round-2 adversarial tests

`rust/crates/kuna-base/tests/verify_w1_base_space_address_r2.rs`
(commit 3a8df4e; 5 tests, all passing — **no new divergences found**):

- verify_bitrange_getmask_undefined_and_oversize_cpp_oracle (numBits=-1
  huge-conversion branch + x86-masked UB shift -> 0x8000000000000000;
  nb=64/70; empty() is `numBits <= 0`)
- verify_bitrange_overlap_matrix_big_endian_cpp_oracle (all seven BE
  return codes incl. cross-container translateLSB == 35)
- verify_bitrange_mutators_cpp_oracle (intersection empty/partial,
  intersectMask minimal cover, shift clamps both edges, truncate
  least/most for both endians, minimizeContainer LE/BE, recontained ctor)
- verify_sign_extend_sized_and_signbit_negative_cpp_oracle (negative
  sizein/sizeout clamp huge->8, sizeout<sizein negative-shift x86
  masking, sizein==0, signbit_negative size 0/9)
- verify_rangelist_merge_last_signed_and_nonchaining_inrange (merge
  bridging + cross-space ordering, getLastSignedRange two-probe walk
  incl. wrong-space/empty, inRange non-chaining vs longestFit chaining +
  maxsize early break)

### Findings (round 2)

- F1 (minor): **stands, unrepaired** — see round 1.  The advertised
  one-line fix was not applied in the repair round (which only touched
  marshal.rs).  Repro unchanged.
- F2 (minor): **stands, unrepaired** for space.rs/address.rs.
- F3-F7 (notes): unchanged (files untouched).
- F8 (note, new): the repair commit message 60bcd4d says
  "marshal+space-address: repair round fixes" but contains no
  space-address changes; its F-numbered bullets refer to the marshal
  review.  Cosmetic/log hygiene only — no code impact — but recorded so a
  later auditor doesn't conclude this item's F1/F2 were addressed.
     cpp: n/a
     rust: commit 60bcd4d (branch worktree-wf_cea42066-041-13)

### Losses (round 2)

LOSS-012 unchanged and verified present in the ledger.  No new losses.

Verdict rationale: no blocker or major findings; the two open minors are
explicitly repair-when-touched and now have double pin coverage (the
#[ignore]d F1 pin plus this round's oracle tests guarding everything
around it).  ACCEPT-WITH-LOSSES stands.
