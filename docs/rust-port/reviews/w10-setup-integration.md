# w10-setup-integration
verdict: ACCEPT-WITH-LOSSES
verifier: Opus 4.8 (1M context) — independent verifier
date: 2026-06-13
branch: worktree-wf_fa71d46d-9cf-2 (re-land ac9bdaf + round-2 verifier tests c41bbcf)
round: 2 (round-1 verdict was REJECT — see "Round 1 → Round 2" below)

## Gate

- Declared item gate — `cargo test -p kuna-console --test setup_commands`
  -> **PASS, 15/15** (8 store-write tests + 3 round-1 adversarial + 1 round-2
  honest-metric guard + 3 round-2 adversarial). Re-ran fresh on this branch.
  Specs are present (`specs/Ghidra/Processors/8051/8051.sla`), so the tests
  genuinely bootstrap and exercise the engine — confirmed NOT a vacuous skip
  (round-2 adversarial `w10_adv_r2_guard_is_not_a_skipped_false_green`).
- `cargo clippy -p kuna-console --tests -- -D warnings` -> **clean**.

## Round 1 → Round 2 (why this flips to ACCEPT-WITH-LOSSES)

Round 1 REJECTed on the **metric framing**, not the port code, and stated the
restoration criterion verbatim: "On a re-land that drops/reframes the parity
metric and records these two [LOSS-133/134], this flips to ACCEPT-WITH-LOSSES —
the port code is correct as-is." The re-land (ac9bdaf) does exactly that:

1. **The inflated "3/675 → 19/675 passing" parity claim is GONE.** The re-land
   commit body presents the honest signal instead — files-reaching-decompile
   7→47, did-not-apply 73→33, assertions-applied 24→362 — and states explicitly
   "NOT a parity claim: the 19/362 applied 'passes' are vacuous min=0/max=0
   negative matches; positive matches FAIL on the empty stub." (LOSS-133.)
2. **LOSS-133 and LOSS-134 are recorded** in `docs/rust-port/losses.md` (both
   the main tree and the branch, merged from rust-port — verified present, well
   anchored to `offsetarray.xml` and `grammar.cc:3122-3125`).
3. **An in-tree honest-metric guard test** (`printc_body_is_the_w9emit_stub_so_
   datatest_passes_are_vacuous`) pins the vacuousness so no future reader
   mistakes the count for parity: it asserts the function REACHES `print C` (the
   real win) AND that the emitted body is still the W9-emit stub marker.

The port **code is byte-identical** to the round-1-reviewed commit (verified:
`git diff b36faee ac9bdaf -- rust/crates/.../src/` is empty). The full hunt list
was already clean against that code in round 1; it remains clean here.

## Independent re-derivation of the framing claim (the pivot)

The whole REJECT→ACCEPT pivot rests on the honest framing being FACTUALLY TRUE.
I re-derived it independently rather than trusting the commit body:

- `printc.rs:1101` unconditionally emits
  `/* WARNING: body emission is the W9-emit RPN/Emit seam */` as the ENTIRE
  function body (the pre-existing LOSS-130 seam, not this item's code). So every
  rust-decompiled function gets a body with no real C — a `min=1` positive
  datatest has nothing to match (FAILS); a `min=0/max=0` negative match passes
  for free. The "+16" datatest count is therefore real-as-a-count but vacuous as
  parity, exactly as LOSS-133 records.
- My 3 round-2 adversarial tests confirm this is not self-serving narration:
  - the guard genuinely bootstraps + reaches print (not a skipped false green),
  - the stub body leaks NO real C statement (`return`/`if`/`;`/`=`/`while`/`goto`),
  - concrete oracle identifiers from `offsetarray.xml`'s min=0 assertions
    (`firstfield`/`array`/`mystruct`/`populate_mystruct`) are genuinely absent.
  All pass. The metric is now framed honestly and the divergence is pinned.

C++ oracle UNTOUCHED (no `decompiler/cpp/` change on the branch; 675/675 PARITY
remains the kuna oracle).

## Hunt list (port code unchanged since round 1 — re-confirmed clean)

- Signedness: clean. int4/size comparisons (`size == 0`, `addr2 <= addr1`,
  sentinel equality) transcribe C++ directly; no silently-inherited signed/unsigned
  compare.
- Integer widths: clean. `set context` value `uintm`→`parse_userbase_u32` (u32),
  `set track` `uintb`→`_u64`; `0xbadbeef` sentinel modeled as `None`.
- Wrapping: clean. `off.wrapping_add(size as u64)` in `mark_property_range` is the
  only arithmetic, `size > 0`-checked. No debug-panic path.
- Comparator totality: clean. `TreeKey::cmp` = compareDependency-then-id (total,
  no float partial_cmp); enum values `BTreeMap<u64,String>` (== C++ map order);
  `nametree` sorted by `(name,id)`.
- Iteration-order provenance: clean. `TrackedSet = Vec<TrackedContext>`, `push` ==
  `vector::push_back`; `set track` reproduces `track = def; track.push_back(...)`.
  No `HashMap`/`HashSet` in the diff (grep clean over both the port diff and the
  round-2 test).
- Off-by-one / do-while / reverse iterators: checked. Volatile range C++
  `Range(space, off, off+(size-1))` (inclusive last) vs rust half-open
  `[off, off+size)` — equivalent, pinned by
  `w10_adv_volatile_range_boundary_is_inclusive`. `parse_machaddr` shortcut omits
  the C++ `s >> ws` (F3, latent — all datatest shortcuts are `r0x...`).
- Erase-while-iterating: n/a. `erase_interned` is remove-then-reinsert (Rc
  re-keying), not in-traversal erase.
- Exception → Result partial-state: clean. `parse_C` failure paths call `set_error`
  + `destroy_type` then return `ParseError`, matching grammar.cc newStruct/newUnion/
  newEnum catch sites.

## Mechanical pass

- `cargo run -p port-audit` — **n/a** (no `port-audit` crate in this workspace).
- diff grep — no `todo!`/`unimplemented!`/`HashMap`/`HashSet`/`sort_unstable` in
  the port diff or round-2 tests. Round-2 test adds no bare `as` casts; the 6 bare
  `as` casts in the port code (round-1 F4, all benign-bounded) are unchanged.
- `cargo clippy -p kuna-console --tests -- -D warnings` — **clean**.
- `git diff b36faee ac9bdaf -- rust/crates/.../src/` — **empty** (port code
  untouched since the round-1 review; only the test file grew +53 in the re-land).

## findings (carried from round 1; all latent / informational — no blocker)

- F1 (RESOLVED): the round-1 "3/675 → 19/675 passing" inflated parity claim is
  removed and reframed honestly; LOSS-133 recorded; pinned in-tree by the
  honest-metric guard test. No longer a blocker.
- F2 (low / latent → LOSS-134): `[space,offset,size]` size parsed decimal-only,
  not the C++ user-selected base. No datatest hits it. Recorded + pinned by
  `w10_adv_bracket_hex_size_diverges_from_cpp`.
       cpp: decompiler/cpp/grammar.cc:3122-3125
       rust: rust/crates/kuna-console/src/ifacedecomp.rs parse_machaddr bracket branch
- F3 (low / latent): non-bracket `parse_machaddr` branch omits the C++ `s >> ws;`
  after the space shortcut; unobservable (all datatest shortcuts are `r0x...`).
       cpp: decompiler/cpp/grammar.cc:3152-3154
       rust: rust/crates/kuna-console/src/ifacedecomp.rs parse_machaddr shortcut branch
- F4 (info / convention): 6 bare `as` casts in the port code lack `// cast:`
  comments (all benign-bounded). Convention nit, unchanged from round 1.

## adversarial tests

Round 1 (committed 3494c59, in rust/crates/kuna-console/tests/setup_commands.rs):
- w10_adv_volatile_range_boundary_is_inclusive
- w10_adv_bracket_hex_size_diverges_from_cpp (PINS LOSS-134)
- w10_adv_default_codespace_address_form

Round 2 (committed c41bbcf, same file — attack the re-land's honest-metric guard):
- w10_adv_r2_guard_is_not_a_skipped_false_green — engine genuinely bootstraps and
  decompile reaches `print C` (specs present); the guard's `else{return}` skip is
  not a silent green.
- w10_adv_r2_stub_body_leaks_no_real_c_statements — the W9-emit stub is the WHOLE
  body; no `return`/`if`/`;`/`=`/`while`/`goto` leaks that a `min=1` positive
  datatest could match.
- w10_adv_r2_negative_match_passes_vacuously_not_by_parity — concrete oracle
  identifiers from offsetarray.xml's min=0 assertions are absent from the stub, so
  the +16 negative passes are vacuous, not parity.

All 15 pass; clippy clean.

## losses

LOSS-133 (vacuous 19/675 parity metric — now reframed honestly and pinned),
LOSS-134 (decimal-only bracket address size). Both recorded in
`docs/rust-port/losses.md`. No new LOSS required for round 2.
