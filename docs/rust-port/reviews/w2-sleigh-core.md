# w2-sleigh-core
verdict: ACCEPT-WITH-LOSSES  (round 2; round 1 was REJECT)
verifier: opus-4.8 (independent)
date: 2026-06-11
round: 1
branch: worktree-wf_f18ede11-813-2 (port tip 6d1122a; adversarial-test commit 417be25)
gate: `cd rust && cargo test -p kuna-sleigh --test golden_lift` -> 16/16 pass (1171 insns);
      `cargo test -p kuna-sleigh` -> all pre-existing tests pass (159 + harness suites),
      ONLY the two new verifier tests fail (they pin F1).
cpp blob sha: all 6 in-scope files match the checklist `cpp_blob_sha` exactly (no drift).
clippy: `cargo clippy -p kuna-sleigh -- -D warnings` clean.
mechanical: no `todo!`/`unimplemented!`/`HashMap`/`HashSet`/`sort_unstable` in the
      three in-scope source files. (port-audit crate is absent from this workspace,
      same as all prior W2 reviews; blob-sha drift checked by hand instead.)

## Summary

A large, mostly faithful and well-annotated port of `sleigh.cc` / `sleighbase.cc` /
`slaformat.cc`. The p-code build engine (PcodeCacher pool, SleighBuilder dump /
generatePointerAdd aliasing, delaySlot do-while at-least-once, resolve /
resolveHandles tree walk) reproduces the C++ faithfully and the golden lift gate
(16 languages, 1171 instructions) passes byte-for-byte. **But `SleighBase::getRegisterName`
is broken for the dominant case** (exact register lookup returns `""` instead of the
register name) — a `std::map::upper_bound` + `--iter` idiom mistranslated as a
`Bound::Excluded` back-walk. This is a live `RegisterLookup` API path
(translate.rs:1332, space.rs:3049/3055), not dead code. REJECT.

## Hunt list

- **Signedness**: clean. The `int4`-vs-size and `int4`-vs-`uint4` spots are handled
  with explicit `as` casts carrying C++-lineage comments (`decode_sla_space` size/
  index/delay; `get_register_name` `size as u32`/`size as u64`). No silent
  signed/unsigned mismatch found.
- **Integer widths**: clean except a benign note (N1 below). `read_signed_integer()
  as i32` / `read_unsigned_integer() as u32` truncations all mirror the C++ implicit
  `intb`->`int4`/`uintb`->`uint4`. `spaceid_const` uses the manager index (LOSS-015,
  pre-existing).
- **Wrapping**: clean. `resolve_relatives` uses `wrapping_sub` for `labels[id] -
  calling_index` (uintb) then `& calc_mask(size)`; `dump` label-base add and
  `set_label` id add use `wrapping_add`; `get_register_name` containment uses
  `wrapping_add` for `point.offset+point.size` and `off+size` (matches C++ uintb wrap).
- **Comparator totality**: the register map key `VarnodeStorage` Ord (space.rs:204,
  out of this item's scope but relied upon here) is total: space index, offset, then
  `op2.size.cmp(&self.size)` (BIG sizes first). No float compares.
- **Iteration-order provenance**: clean. `build_xrefs`/`reregister_context` iterate
  `glb.symbol_ids()` (BTreeMap = the C++ `SymbolTree` set order). `SourceFileIndexer::
  encode` walks `0..least_unused_index` (C++ index order). `varnode_xref` is a
  BTreeMap (C++ `std::map`), matching the upper_bound/find lookups. No HashMap/HashSet.
- **Off-by-one / do-while / reverse iterators**: **FINDING F1** — the `upper_bound`
  + `--iter` idiom in `getRegisterName` was mapped to `Bound::Excluded(key)` instead of
  the `<= key` semantics of `--upper_bound` (`Bound::Included`). The two delay-slot /
  one_instruction `do {} while` loops are correctly `loop { ...; if cond { break } }`
  (at-least-once preserved).
- **Erase-while-iterating**: n/a — no in-scope erase-during-traversal.
- **Exception -> Result partial-state parity**: clean. `decode`'s `?` points mirror
  the C++ throw sites (wrong-version, bad defaultspace, duplicate register pairs);
  `one_instruction`'s UnimplError catch/rethrow with `instruction_length = fallOffset`
  is reproduced. The `symtab`/`templates` `mem::take`+restore around `SymbolTable::decode`
  restores state on both Ok and Err before propagating (`decode_res?` after restore).
- **Aliasing**: the `initialize_from_sla` `unsafe &*manager_ptr` is the LOSS-020
  self-aliasing back-pointer; SAFETY note is accurate (reads strictly after the
  inserts). `generatePointerAdd`'s `op->outvar = nextop->invar + 1` pool aliasing is
  reproduced index-for-index.

## Findings

- **F1 (BLOCKER, major)**: `SleighBase::getRegisterName` returns `""` for an *exact*
  register lookup (and for any sub-register query) where the C++ returns the
  containing register's name.
    - cpp: decompiler/cpp/sleighbase.cc:144-168
    - rust: rust/crates/kuna-sleigh/src/sleighbase.rs:285-329
  - Root cause: C++ does `iter = upper_bound(key); if (iter==begin()) return "";
    iter--;` — `--upper_bound(key)` is the *greatest element <= key*. The port
    computes `prev_iter = range(Unbounded, Excluded(&key)); prev_iter.next_back()`,
    which is the greatest element *strictly < key*, skipping an exact match. Since
    every full-register query has `key` present in the map, `range(.., Excluded(key))`
    excludes it and `next_back()` returns the previous (smaller) register or `None`.
    The dead `let _ = iter.next();` confirms the `upper_bound` was computed and then
    discarded rather than decremented. The walk-back loop (`back = range(Unbounded,
    Excluded(point))`) inherits the same wrong starting point.
  - Correct port: start from `range(Unbounded, Included(&key)).next_back()` (the
    `<= key` element), then walk back with `Excluded` as the C++ `--iter` loop does.
  - Proof: `verify_w2core_get_register_name_exact_match_every_register` decodes the
    committed `mips32be.sla`, then for all **471** registers in the populated xref
    map compares `get_register_name(space,off,size)` to the map name — **471/471
    return `""`** (want e.g. "zero","at","v0",...). `verify_w2core_get_register_name_
    subregister_containment` shows `getRegisterName(zero, 0x0, 1)` (first byte of a
    register) also returns `""`. `get_exact_register_name` (a plain `map::find`) is the
    independent witness that each entry exists at the exact triple, so the divergence
    is solely in the upper_bound/iter-- translation.
  - Reachability: this is the `RegisterLookup` register-naming path
    (rust/crates/kuna-sleigh/src/translate.rs:1332,
    rust/crates/kuna-base/src/space.rs:3049/3055 join-space naming,
    rust/crates/kuna-sleigh/src/sleigh.rs:2062). It is the standard "name the varnode
    that covers this register" operation in real decompilation. The golden lift gate
    does not catch it because the lift dump renders numeric `(space,0xoff,size)`
    triples and never calls getRegisterName.

## Notes (non-blocking, for the porter's awareness on re-submit)

- **N1**: `resolve_relatives` uses the full `u64` `pool[ptr].offset` as the label id,
  while C++ does `uint4 id = ptr->offset` (truncates to 32 bits before the
  `labels.size()`/`labels[id]` index). Label ids are bounded by the label count
  (small), so this is unreachable in practice, but a faithful port should truncate
  (`as u32`/`as u64` ladder) to match. cpp sleigh.cc:126; rust sleigh.rs:962.
- **N2**: CROSSBUILD targets are not pre-resolved into the `contexts` set
  (`one_instruction` only pre-resolves delay-slot contexts); `append_cross_build`
  errors "Could not obtain cached crossbuild instruction" when `find_context` misses.
  C++ `appendCrossBuild` calls `discache->getParserContext(newaddr)`, which can parse
  fresh. No golden fixture exercises CROSSBUILD (SuperH4 etc. are not among the 16
  languages), so this is unverified rather than a confirmed divergence — but it is a
  behavioral gap to validate before any CROSSBUILD language is claimed.
  cpp sleigh.cc:405-439; rust sleigh.rs:1381-1430, 1970-1992.
- **N3** (cosmetic): `is_sla_format` on `< 4` bytes returns the whole slice as the
  "rest" (unconsumed), whereas C++ has already consumed up to 4 bytes; irrelevant for
  real `.sla` (decode fails regardless). slaformat.rs:253-267.

## Adversarial tests added (committed 417be25 on the port branch)

rust/crates/kuna-sleigh/tests/verify_w2_sleigh_core.rs:
- `verify_w2core_get_register_name_exact_match_every_register` — **FAILS** (pins F1)
- `verify_w2core_get_register_name_subregister_containment` — **FAILS** (pins F1)
- `verify_w2core_sourcefileindexer_assignment_and_roundtrip` — passes
- `verify_w2core_is_sla_format_boundaries` — passes
- `verify_w2core_register_map_is_populated` — passes (guards F1 from being vacuous)

## Work order for the re-port

Fix F1: in `get_register_name`, replace the `Bound::Excluded(&key)` back-walk start
with `range((Bound::Unbounded, Bound::Included(&key))).next_back()` to reproduce
`--upper_bound` (greatest element `<= key`); keep the subsequent same-offset walk-back
loop on `Excluded`. Re-run `cargo test -p kuna-sleigh --test verify_w2_sleigh_core`
(the two F1 tests must pass) plus the golden gate (must stay 16/16). Optionally
address N1 (truncate the label id) while touching the file.

---

# w2-sleigh-core — round 2
verdict: ACCEPT-WITH-LOSSES
verifier: opus-4.8 (independent)
date: 2026-06-11
round: 2
branch: worktree-wf_f18ede11-813-2 (repair commit 32310e4; round-2 adversarial-test commit 2ea762e)
gate: `cd rust && cargo test -p kuna-sleigh --test golden_lift` -> 16/16 pass;
      `cargo test -p kuna-sleigh` -> all suites green (159 unit + golden_lift 16 + every
      verify_* suite, including the 5 round-1 verify_w2_sleigh_core tests now PASSING and
      the 3 NEW round-2 verify_w2core_walkback tests). No failures.
cpp blob sha: all 6 in-scope files still match the checklist `cpp_blob_sha` exactly
      (sleigh.cc f6b718d6, sleigh.hh 95249cf1, sleighbase.cc 26805d64, sleighbase.hh
      5286d9ee, slaformat.cc f8b3bcfa, slaformat.hh a8eb11b6) — no drift.
clippy: `cargo clippy -p kuna-sleigh -- -D warnings` (the protocol gate form) clean.
      (`--tests` surfaces only pre-existing crate-wide lints — mutable_key_type on every
      get_all_registers() iteration incl. the accepted round-1 test, type_complexity in
      translate.rs, drop_non_drop in slghsymbol.rs — none introduced by this round.)
mechanical: no todo!/unimplemented!/HashMap/HashSet/sort_unstable in the three in-scope
      source files. The repair touched ONLY sleighbase.rs (9 lines); no other source
      changed since round 1 (`git diff 6d1122a..HEAD -- src/` = sleighbase.rs only).

## Disposition of round-1 findings

- **F1 (was BLOCKER) — FIXED & VERIFIED.** `get_register_name` now starts the back-walk
  from `range((Unbounded, Included(&key))).next_back()` (greatest element `<= key`),
  reproducing C++ `--upper_bound(key)`; the dead `iter.next()` is gone; the subsequent
  same-offbase walk-back correctly stays on `Excluded(point)`.
    - cpp: decompiler/cpp/sleighbase.cc:144-168
    - rust: rust/crates/kuna-sleigh/src/sleighbase.rs:285-329
  - Round-1 tests `verify_w2core_get_register_name_exact_match_every_register` (471/471
    MIPS registers) and `_subregister_containment` now PASS.
  - **Independent oracle proof (round 2):** I dumped the full 1440-entry x86-64 register
    map from the engine, fed it to a standalone C++ re-implementation of the exact
    `getRegisterName` algorithm (upper_bound/iter--/offbase walk-back, `operator<` BIG-
    sizes-first), and swept **73,728 queries** (0x1200 offsets x 16 sizes). Engine output
    was **byte-for-byte identical** to the C++ oracle (`diff` empty). This exercises the
    walk-back loop that round-1's tests did NOT reach (the `while (iter != begin())`
    same-offbase back-step + the `point.offset != offbase` early-out). The fix is correct
    in full, not just on the exact-match case.

## Round-2 hunt list (re-derived; focus on the changed function + whole-diff re-scan)

- **Signedness**: clean. `size as u64` / `size as u32` in get_register_name carry the
  C++ `int4`->`uint4`/`uintb` comment; size is always non-negative at every call site
  (varnode sizes; `hisz+losz` sums in space.rs). The `id >= self.labels.len() as u64`
  compare (resolveRelatives) is uint-vs-uint.
- **Integer widths**: one confirmed (unreachable) departure -> **LOSS-028** (resolveRelatives
  keeps the label id at full u64 vs C++ `uint4 id = ptr->offset`). The get_register_name
  containment widths (`u64::from(point.size)`) mirror `uint4`->`uintb`.
- **Wrapping**: clean. get_register_name containment uses `wrapping_add` on both sides
  (`point.offset + point.size` and `off + size`), matching C++ uintb wrap; resolveRelatives
  `wrapping_sub`/`& calc_mask` unchanged and correct.
- **Comparator totality**: `VarnodeStorage::cmp` (space.rs:204, relied on here, out of this
  item's scope) is total — space index, offset, then `op2.size.cmp(&self.size)` (BIG first),
  transcribing `VarnodeData::operator<` (pcoderaw.hh:67) field-for-field. No floats.
- **Iteration-order provenance**: clean. `varnode_xref` is a `BTreeMap` (= C++ `std::map`);
  the `range(.., Included)/.next_back()` and the `range(.., Excluded(point)).next_back()`
  walk-back reproduce the C++ reverse iteration off `upper_bound`. No HashMap/HashSet in
  the in-scope files.
- **Off-by-one / do-while / reverse iterators**: the F1 idiom is now correct (Included =
  `<= key`). The same-offbase walk-back's starting position (`Excluded(point)`) matches
  C++ `--iter` from the matched element. Delay-slot / one_instruction `do {} while` loops
  remain `loop { ...; if cond break }` (at-least-once preserved) — unchanged by the repair.
- **Erase-while-iterating**: n/a in scope.
- **Exception -> Result partial-state parity**: unchanged by the repair; round-1 clearance
  stands (decode `?` sites mirror the C++ throws; symtab/templates mem::take+restore on
  both Ok and Err).
- **Aliasing**: unchanged by the repair (LOSS-020 self-back-pointer; generatePointerAdd
  pool aliasing).

## Findings (round 2)

No blockers or majors. Two non-blocking divergences accepted as losses (both unreachable
from the w2-sleigh-core gate and from any committed fixture):

- **F2 (minor, ACCEPTED as LOSS-027)**: `append_cross_build` cannot lazily disassemble an
  uncached CROSSBUILD target — it only consults the pre-resolved `contexts` slice (main +
  delay-slot), whereas C++ `appendCrossBuild` calls `discache->getParserContext(newaddr)`,
  which disassembles any address on demand. Errors loudly ("Could not obtain cached
  crossbuild instruction") rather than producing wrong p-code. Only Hexagon emits the
  `crossbuild` directive in the vendored specs (0 occurrences across MIPS/x86/6502/V850/
  RISCV/Atmel/Toy/SuperH4); Hexagon is not a golden language, so no fixture reaches it.
    - cpp: decompiler/cpp/sleigh.cc:405-439
    - rust: rust/crates/kuna-sleigh/src/sleigh.rs:1381-1421, 1953-1992
- **F3 (trivial, ACCEPTED as LOSS-028)**: resolveRelatives label id kept at full u64 vs
  C++ `uint4 id = ptr->offset`. Label ids are small sequential `set_label` indices, so the
  values coincide for every valid spec; the Rust outcome is strictly safer on corrupt input.
    - cpp: decompiler/cpp/sleigh.cc:126
    - rust: rust/crates/kuna-sleigh/src/sleigh.rs:962

Cosmetic-only (NOT a loss): `is_sla_format` returns the whole `< 4`-byte slice as the
unconsumed tail where C++ has consumed up to 4 bytes; unobservable (decode fails either way).
slaformat.rs:253-267.

## Adversarial tests added (round 2; committed 2ea762e on the port branch)

rust/crates/kuna-sleigh/tests/verify_w2core_walkback.rs (all PASS):
- `verify_w2core_walkback_rax_cluster` — RAX/EAX/AX/AL @ off 0 + interior bytes (AH@0x1,
  CL@0x8); asserts the C++-oracle outputs incl. the offbase-constrained walk-back stops.
- `verify_w2core_walkback_mm0_cluster` — the deepest chain (ST0/MM0/MM0_Da/MM0_Wa/MM0_Ba +
  MM0_Bb@0x1101, ST0h@0x1108); whole-register, interior, and past-the-end misses.
- `verify_w2core_walkback_bnd_and_space_mismatch` — BND0/BND0_LB/BND0_UB pair + the
  `point.space != base` short-circuit (constant space holds no registers -> always "").

All three assert genuine C++ algorithm outputs (the 73,728-query engine==oracle equivalence
above is the provenance of every expected value).

## Verdict

F1 is fully repaired and independently re-proven against a C++ oracle over the complete
real register map. The only remaining divergences (CROSSBUILD lazy-disassembly, label-id
width) are unreachable from the gate and recorded as LOSS-027/LOSS-028. **ACCEPT-WITH-LOSSES**
(LOSS-027, LOSS-028; pre-existing item losses LOSS-015, LOSS-020, LOSS-021 still apply).
