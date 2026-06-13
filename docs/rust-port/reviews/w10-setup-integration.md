# w10-setup-integration
verdict: REJECT
verifier: Opus 4.8 (1M context) — independent verifier
date: 2026-06-13
branch: worktree-wf_fa71d46d-9cf-2 (b36faee + verifier tests 3494c59)

## Gate

- Declared item gate (commit "Gate:" line) — `cargo test -p kuna-console --test setup_commands`
  -> **PASS** (8 store-write tests + 3 verifier adversarial = 11/11). These are
  substantive and oracle-independent: they assert the *store reflects* each
  command (parsed struct exists & is complete with size 8; mapped function/global/
  label symbols resolve in the global scope; a CODE range reads back `volatil`;
  a context default reads back; a comment lands in the comment db).
- Reported metric (commit body) — "passing 3/675 -> 19/675" datatests under
  `KUNA_ENGINE=rust`. Re-ran the harness fresh on this branch: **19/362 applied
  passing**, baseline (rust-port f3df8f6) = **3**. The +16 count is REAL as a
  count — but see F1: it is a **vacuous parity signal**.

## Why REJECT

Per the verifier protocol's explicit procedure ("spot-check a passing test's C
output vs the C++ oracle — it must MATCH the oracle, not just be non-empty") and
its REJECT condition ("REJECT iff parity claims are inflated or output diverges
from the oracle where claimed to match"):

I dumped the rust engine's `print C` for a newly-passing test (`offsetarray.xml`,
"Offset array #2") and compared to the C++ `decomp_test_dbg` oracle:

- C++ oracle: `int4 access_array1(int8 a0){ mystruct v1; populate_mystruct(&v1);
  return v1.array[a0 + -1]; }`
- rust engine: `void access_array1(void){ /* WARNING: body emission is the
  W9-emit RPN/Emit seam */ }`

The output **diverges completely** from the oracle. The test "passes" only because
its assertion is `min="0" max="0"` on `firstfield` — a negative match the empty
stub satisfies vacuously. I classified all 19 passing assertions: **every one is a
`min=0/max=0` negative match** (the baseline-3 too). None demonstrate
oracle-matching C output; the corresponding positive (`min=1`) assertions of the
same files all FAIL. The commit's "passing 3/675 -> 19/675" is therefore an
**inflated parity claim**.

REJECT is on the **metric/gate-framing**, not the port code. The setup-command
port itself is correct and well-tested (the declared integration gate passes; the
store writes match the C++ store APIs). The work order is narrow:

1. Do not present the vacuous `min=0/max=0` passes as datatest parity. Report the
   honest signal (the integration test + files-reaching-decompile / tests-applied
   counts) until the PrintC body driver lands. (LOSS-133.)
2. (Optional, latent) close the hex-size gap below. (LOSS-134.)

The port code need not be rewritten; re-land with an honest metric and LOSS-133/134
recorded, and this becomes ACCEPT-WITH-LOSSES.

## Hunt list

- Signedness: clean. The int4/size comparisons (`size == 0`, `addr2 <= addr1`,
  `value == sentinel`) transcribe the C++ directly; no signed/unsigned compare
  was silently inherited.
- Integer widths: clean-ish. `set context` value is `uintm` -> `parse_userbase_u32`
  (u32) and `set track` is `uintb` -> `_u64`, matching C++. The `0xbadbeef`
  sentinel is modeled as `None` from the parse (faithful "missing value").
- Wrapping: clean. `off.wrapping_add(size as u64)` in `mark_property_range` is the
  only arithmetic; `size` is `>0`-checked. No debug-panic path.
- Comparator totality: clean. `TreeKey::cmp` = compareDependency-then-id (total,
  no float partial_cmp); enum values use `BTreeMap<u64,String>` (key order ==
  C++ `map`); `nametree` kept sorted by `(name,id)`.
- Iteration-order provenance: clean. `TrackedSet = Vec<TrackedContext>` with
  `push` == C++ `vector::push_back`; `set track`'s `*track = def; track.push(...)`
  reproduces `track = def; track.push_back(...)`. No `HashMap`/`HashSet` added
  (grep clean over the diff).
- Off-by-one / do-while / reverse iterators: **checked, one finding-adjacent**.
  The volatile/readonly range: C++ `Range(space, off, off+(size-1))` (inclusive
  last) vs rust `set_property_range(flag, off, off+size)` over a half-open
  `[off, off+size)` — equivalent (`+size` open == `+size-1` closed); pinned by
  `w10_adv_volatile_range_boundary_is_inclusive` (last in-range byte set, off+size
  byte clear). The `parse_machaddr` shortcut branch omits the C++ `s >> ws` after
  the shortcut char (F3, latent: all datatest shortcuts are `r0x...` with no
  intervening space).
- Erase-while-iterating: n/a here. The type-construction `erase_interned` is a
  remove-then-reinsert (Rc re-keying), not an in-traversal erase; the C++ in-place
  `Datatype*` mutation is modeled as remove-old-key / insert-new-key and the
  console flow threads the returned `Rc` forward.
- Exception -> Result partial-state: clean. `parse_C`'s struct/union/enum failure
  paths call `set_error` + `destroy_type(&res)` then return a ParseError, matching
  the C++ `setError` + `destroyType` + return-null catch sites (grammar.cc
  newStruct/newUnion/newEnum). The `f_extern`/`f_typedef`/bare-struct dispatch is
  an exact transcription of `parse_C` (grammar.cc:3009-3036), including the final
  "Not sure what to do with this type".

## Mechanical pass

- `cargo run -p port-audit` — **n/a** (no `port-audit` crate in this workspace).
- diff grep — no `todo!`/`unimplemented!`/`HashMap`/`HashSet`/`sort_unstable`.
  Bare `as` casts added (6) lack `// cast:` comments (minor convention finding,
  F4): `sublist.len() as int4`, `i as int4`, `size as u64`, `num_spaces() as int4`
  (x2), `get_addr_size() as int4` — all benign-bounded, none change the C++ result.
- `cargo clippy -p kuna-console -p kuna-decomp -p kuna-sleigh -- -D warnings` —
  **clean**.

## findings

- F1 (high / metric): the "passing 3/675 -> 19/675" datatest claim is a vacuous
  parity signal — all 19 passes are `min=0/max=0` negative matches satisfied by the
  empty-body PrintC stub, not by oracle-matching C output. The rust `print C`
  diverges entirely from the C++ oracle for every passing file. -> LOSS-133.
       cpp: decompiler/datatests/offsetarray.xml (Offset array #2, min=0/max=0)
       rust: rust/crates/kuna-harness/src/testfunction.rs (bulk `print C` = W9-emit stub)
- F2 (low / latent): the `[space,offset,size]` size specifier is parsed
  decimal-only (`read_int`) instead of the C++ user-selected base — a hex/octal
  size mis-parses. No datatest hits it (all sizes decimal). -> LOSS-134.
       cpp: decompiler/cpp/grammar.cc:3122-3125
       rust: rust/crates/kuna-console/src/ifacedecomp.rs parse_machaddr bracket branch
             (-> rust/crates/kuna-console/src/interface.rs:264 read_int)
- F3 (low / latent): the non-bracket `parse_machaddr` branch omits the C++
  `s >> ws;` after the space shortcut, so `r 0x10` (space after shortcut) would
  read an empty offset token. All datatest shortcuts are `r0x...` (no space), so
  unobservable. Not separately LOSS'd (sub-case of the parse_machaddr seam).
       cpp: decompiler/cpp/grammar.cc:3152-3154
       rust: rust/crates/kuna-console/src/ifacedecomp.rs parse_machaddr shortcut branch
- F4 (info / convention): 6 bare `as` casts added without `// cast:` justification
  comments (all benign-bounded). Convention nit, not a behavior finding.

## adversarial tests (committed 3494c59, in rust/crates/kuna-console/tests/setup_commands.rs)

- w10_adv_volatile_range_boundary_is_inclusive — the volatile range paints exactly
  [off, off+size): last in-range byte set, off+size byte clear (no off-by-one).
- w10_adv_bracket_hex_size_diverges_from_cpp — PINS F2: a hex bracket size
  mis-parses to 0 (no success line) where C++ reads 16.
- w10_adv_default_codespace_address_form — the 0-prefixed default-code-space
  address form resolves and creates one global symbol.

## losses

LOSS-133 (vacuous 19/675 parity metric), LOSS-134 (decimal-only bracket size).
On a re-land that drops/reframes the parity metric and records these two, this
flips to ACCEPT-WITH-LOSSES — the port code is correct as-is.
