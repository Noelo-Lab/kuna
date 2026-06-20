# w10-proto-unlock
verdict: ACCEPT
verifier: independent (Opus 4.8, round 2)
date: 2026-06-16
gate: `cd rust && cargo test --workspace` -> green (3551 passed / 0 failed / 37 ignored, incl. 4 new
verifier adversarial tests); `cargo clippy -p kuna-decomp --lib -- -D warnings` -> clean; C++ oracle
207/207 unit + 675/675 datatest assertions PARITY OK (untouched); B0 universalAction listing
byte-equal (tests/universalaction_listing.rs -> 1 passed).

## Scope and verdict

The branch is ONE porting commit (09edbd1) touching exactly two source files
(`architecture.rs` +99, `coreaction_cleanup.rs` +47) and one test edit
(`verify_w10_input_params.rs`). It delivers the two coupled pieces the proto-cluster
branch (LOSS-204 follow-up) held back "pending the promote_compare fix":

1. **`Architecture::decode_join_addr` + the `IPTR_JOIN` dispatch** in
   `decode_pentry_storage` (architecture.rs:1716-1810) — a faithful port of
   `JoinSpace::decodeAttributes` (space.cc:539) reached through the C++
   `VarnodeData::decodeFromAttributes` space dispatch (pcoderaw.cc:33). It resolves the
   x86 struct-return output pentry `<addr space="join" piece1="EDX" piece2="EAX"/>` to a
   real unified offset instead of the offset-0 fallback that made the default proto model
   build empty.

2. **The `all_writes_const_copy` over-tie guard** in `mark_output_storage_addr_tied`
   (coreaction_cleanup.rs:334-354) — a return register written ONLY by `CPUI_COPY`s of a
   constant (the `EAX = #0` / `EAX = #1` constant returns) is left un-tied so the printer
   collapses `EAX = #N; return EAX;` to `return N;`, exactly as C++ `inScope` does, and the
   overlapping AL `(uint1)` promotion cast is not dropped into a tied EAX group.

Verdict is **ACCEPT** (not ACCEPT-WITH-LOSSES): the branch fully and faithfully delivers its
declared scope with ZERO regression and net +1 real parity. The 5 still-failing retstruct
assertions (#1,#2,#4,#5,#6) are a PRE-EXISTING struct-field-write / structuring gap (fail on
base too) that this branch neither introduces nor worsens — not a loss owned by this item.

## FAITHFULNESS (1) — verified line-by-line against the C++

### decode_join_addr vs JoinSpace::decodeAttributes (space.cc:539)
- **`logicalsize`** attribute read -> `parse_int(..) as u32`; matches C++
  `decoder.readUnsignedInteger()` into the `uint4 logicalsize`.
- **`pieceN` -> ATTRIB_PIECE index**: `name.strip_prefix("piece").parse::<i32>()` then `n-1`
  reproduces the C++ `attribId - ATTRIB_PIECE.getId()` indexed-attribute mapping ("piece1"
  is the most-significant piece at index 0). The result is INDEXED BY `pos`, so the
  attribute-iteration order is irrelevant — order-independent, matching C++. No HashMap.
- **`MAX_PIECES`**: `if pos > 64 { continue }` == C++ `if (pos > MAX_PIECES) continue` with
  `MAX_PIECES = 64` (space.hh:233). The `while pieces.len() <= pos` grow == C++
  `while(pieces.size() <= pos) pieces.emplace_back()` (default `VarnodeStorage` ==
  default-constructed `VarnodeData`: space=None/null, offset=0, size=0 — `derive(Default)`
  verified).
- **register piece** (`attr_val` has no `:`): `get_register_varnode(name)` == C++
  `getTrans()->getRegister(attrVal)`; the field-by-field `VarnodeStorage { space, offset,
  size }` build == C++ `vdat = point` (kuna_num::pcoderaw::VarnodeData has the SAME three
  fields).
- **triple piece** (`space:offset:size`): the C++ `substr(offpos+1, szpos)` LENGTH-vs-position
  quirk was checked with a standalone C++ probe — C++ takes `szpos` chars from `offpos+1`
  (i.e. `"0x10:4"`), and `istringstream >> offset` stops at the inner `:`, NETTING the chars
  between the two colons. The Rust slice `&attr_val[offpos+1..szpos]` is EXACTLY those chars,
  so the net offset is identical. The size slice `[szpos+1..]` == C++ `substr(szpos+1)`.
  Verified `parse_int` (0x-hex/decimal auto-base) matches `istringstream` with
  `unsetf(dec|hex|oct)`.
- **uint4 overflow saturation**: the `size64 > u32::MAX -> u32::MAX` clamp was validated
  against a C++ probe — `istringstream >> uint4` on `0xffffffffff` SATURATES to `UINT_MAX`
  (4294967295). The Rust clamp is a faithful reproduction of a real C++ quirk, with a
  `// cast:` comment.
- **`_sizesum`**: confirmed C++ accumulates `sizesum += vdat.size` but NEVER reads it after
  the loop (only `getUnified().size`/`.offset` are used) — the `_sizesum.wrapping_add`
  parity comment is accurate.
- **return**: `rec.get_unified().offset` == C++ `return rec->getUnified().offset` via the
  already-verified `find_add_join` (== C++ `findAddJoin`, splitset comparator keys on
  pieces+size, allocate rounds up to 16 with `wadd`). `find_add_join` is NOT in this diff.

### IPTR_JOIN dispatch (architecture.rs:1715)
`if space.get_type() == IPTR_JOIN { decode_join_addr } else { read offset }` faithfully
reproduces C++ `VarnodeData::decodeFromAttributes` (pcoderaw.cc:42-46): read `space`, then
`space->decodeAttributes(...)` — for the join space that is `JoinSpace::decodeAttributes`;
for every other space the default `AddrSpace::decodeAttributes` reads the `offset` attribute.
Does NOT arbitrarily resolve non-join pentries differently. B0 unchanged.

### The over-tie un-tie (coreaction_cleanup.rs:334)
`all_writes_const_copy` = every write of the return-size/addr Varnode is a `CPUI_COPY` whose
in(0) `is_constant()`. This is a pure IR-shape predicate (no name/addr/value). C++ `inScope`
never restructures an all-constant-COPY return register into a whole-function local and
collapses `reg = #N; return reg;` to `return N;` — verified against the oracle (see (3)).
Ordering is safe: markers are MULTIEQUAL/INDIRECT, never COPY, so `all_writes_const_copy` and
the `all_marker_inputs_persist` arm are MUTUALLY EXCLUSIVE; placing it before the
`output_locked` gate changes behavior ONLY for `output_locked == false` + all-const-COPY (the
promotecompare shape), which the oracle confirms should collapse. Does NOT arbitrarily un-tie
a non-constant register (proven by adversarial test C + the necessity experiment below).

`mark_output_storage_addr_tied` as a whole is itself the pre-existing kuna ScopeLocal/inScope
STAND-IN (a documented prior loss, not this item's); this branch only ADDS a narrow, correct
arm to it.

## NO SPECIAL-CASING (2)

- grep of added EXECUTABLE lines for register names (EAX/EDX/.../AL), function names
  (promote_compare/retstruct/boolless/readpartial), `0x100xxxx` addresses, `glob1`:
  **NONE** — those tokens appear ONLY in doc comments and test assertions.
- forbidden constructs in the diff: NO `todo!`/`unimplemented!`/`HashMap`/`HashSet`/
  `sort_unstable`/`partial_cmp`.
- every bare `as` cast (`as u32` x2, `as usize` x2) carries a `// cast:` justification
  (same-line or immediately-following line). clippy `-D warnings` clean.

## REAL PARITY (3) — the new pass is genuine, oracle-faithful

- Datatest passing SET, branch's MERGE-BASE (605f962) vs branch (09edbd1), real Rust
  `decomp_test_dbg` over repo specs(.sla)+datatests:
  base = **675 applied / 319 passing**, branch = **675 applied / 320 passing**.
  `comm` of the sorted `Success --` sets: REGRESSIONS = **EMPTY**; NEW = exactly
  `Return Structure #3`. base passing SET is a STRICT SUBSET of branch.
- `Return Structure #3` (`y = 0x1e;`) is REAL: present in BOTH the Rust `print C` and the
  C++ oracle `print C` for retstruct (dumped via `KUNA_DUMP`). The recovered signature
  `foo retstruct(int4 x,int4 y,int4 z)` matches the oracle exactly (was `void
  retstruct(void)` while the join pentry failed to decode). The other 5 retstruct
  assertions need the struct-field-write render (`v1.a = tmp` / `v1.b = y` / `return v1`),
  not yet ported — they correctly STILL FAIL (the Rust renders `CONCAT(v3,v2)`); no
  weakened/fabricated assertion.
- `promotecompare`: recovered `xunknown4 promote_compare(char *)` matches the oracle's
  `xunknown4 promote_compare(char *a0)` return type + parameter (the test pin was updated
  from the stale `void promote_compare(void)`); the constant returns collapse to
  `return 0` / `return 1` exactly as the oracle (no tied-EAX round-trip).

## NO REGRESSION (4)

- `Promotion on compare #2` (the task's explicit gate): **Success on base AND on branch**.
  NECESSITY EXPERIMENT — applying ONLY the architecture.rs join-decode change to the base
  (NOT the un-tie fix) and re-running the corpus: `Promotion on compare #2` REGRESSES to
  FAIL (319 passing: +Return Structure #3, -Promotion #2 = net 0). With BOTH changes
  (full branch): Promotion #2 Success AND Return Structure #3 Success = net +1. This is
  the smoking-gun proof that the un-tie fix is REQUIRED to keep Promotion #2 passing once
  the join model builds — exactly as the branch's design and LOSS-189 predicted.
- `boolless` body byte-identical base->branch (13 lines). `condconst`/`condconst2`/
  `condconstsub`/`nan` bodies byte-identical.
- `partialmerge` body DIFFERS by 2 lines (62->60) — but its assertion results are IDENTICAL
  (Partial Merge #3 and #11 pass on both; rest fail on both). The change is the INTENDED
  const-return collapse on `readpartialstack_inbetween` (`xunknown4 v1; v1 = 0; return v1;`
  -> `return 0;`), which is the oracle DIRECTION (C++ always collapses `reg=#const;return
  reg;`); that function's return-value recovery (0 vs the oracle's `b_between`) is a
  pre-existing stack-struct gap unrelated to this branch. Zero passing-set impact.
- NO reserved file touched: `double.rs`, `funcdata_callsite.rs`, `heritage.rs`, `merge.rs`,
  `funcdata_merge.rs` all absent from `git diff 605f962..HEAD` (only architecture.rs +
  coreaction_cleanup.rs + two test files).
- `cargo test --workspace` green (3551/0); clippy clean; C++ oracle 207/207 + 675/675
  PARITY OK (only `rust/` touched — `git diff` over decompiler/cpp, specs, Makefile,
  baseline EMPTY); B0 listing test passes (byte-equal).

## Mandatory hunt list (per verification.md) — all clean

- **Signedness**: `pos:i32` vs `64:int4` (signed==signed); `pos` guaranteed `>= 0` (from the
  `n >= 1` filter, then `n-1`), so `pos as usize` is safe. clean.
- **Integer widths**: `logicalsize`/`size` are `u32` (C++ uint4); `parse_int -> uintb(u64)`
  narrowed to u32 with the documented saturation matching C++ `istringstream>>uint4`. clean.
- **Wrapping**: `_sizesum.wrapping_add` == C++ uint4 wrap (value unused); `find_add_join`'s
  `wadd` is pre-verified kuna-base. clean.
- **Comparator totality**: no comparator in the diff. n/a.
- **Iteration-order provenance**: the attribute loop indexes pieces by `pos` (name-derived),
  so order-independent; matches C++. No HashMap/HashSet. clean.
- **Off-by-one / do-while**: `while pieces.len() <= pos` == C++ `while(pieces.size() <= pos)`;
  plain forward `for i in 0..nattr`. clean.
- **Erase-while-iterating**: none. clean.
- **Exception -> Result partial-state**: `?`/`ok_or_else` land at the same malformed-piece /
  bad-logicalsize points C++ `throw LowlevelError`s; the early `return`s in
  `mark_output_storage_addr_tied` match C++ control flow. clean.

## Mechanical pass

- clippy `-p kuna-decomp --lib -D warnings`: clean.
- cargo test --workspace: 3551 passed / 0 failed / 37 ignored.
- B0 (`tests/universalaction_listing`): 1 passed (byte-equal).
- special-casing grep over added lines: NONE (executable lines clean).
- forbidden-construct grep: NONE.

## Adversarial tests (committed on branch bc0db88, marked w10_proto_unlock)

All 4 PASS on the branch; tests A and B FAIL on the merge-base (proving both changes are
load-bearing), C and D pass on both (the no-regression invariants):
- `w10_proto_unlock_join_struct_return_is_real_not_void` — retstruct recovers a non-void
  `foo` struct return (not `void retstruct(void)`) + the `y = 0x1e;` body; cross-checks the
  C++ oracle. (FAILS on base: base recovers void.)
- `w10_proto_unlock_const_return_collapses_no_tied_roundtrip` — promotecompare collapses to
  `return 0`/`return 1` with NO `reg = N;` tied round-trip + the `xunknown4 ... char *`
  signature; cross-checks the oracle. (FAILS on base: base keeps the tied round-trip.)
- `w10_proto_unlock_nonconst_return_register_unaffected` — partialmerge's non-constant
  `... + 10;` return (Partial Merge #3) is preserved (the guard is narrow). (passes on both.)
- `w10_proto_unlock_join_and_untie_are_deterministic` — retstruct + promotecompare bodies
  byte-identical run-to-run (no join-offset / un-tie order leak). (passes on both.)

losses: NONE. (The 5 still-failing retstruct assertions are a pre-existing struct-field
render gap, fail on base, not owned by this item; the partialmerge body delta is the intended
oracle-direction const-return collapse with zero passing-set impact.)
