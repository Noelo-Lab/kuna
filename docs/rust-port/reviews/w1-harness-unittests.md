# w1-harness-unittests

## Round 1

verdict: ACCEPT
verifier: independent verifier agent (Claude, not the porter)
date: 2026-06-11
branch: worktree-wf_cea42066-041-20 (worktree `.claude/worktrees/wf_cea42066-041-20`, reviewed at 4c03e6c; adversarial tests committed as e7620a5)
cpp blob shas: both in-scope files match the checklist entry (`git hash-object` verified in the main tree AND the worktree: testmarshal.cc 6b94553e, testmultiprec.cc 529b8c81)

gate: `cd rust && cargo test -p kuna-base --test testmarshal && cargo test
-p kuna-num --test testmultiprec` -> **PASS**: testmarshal 18/18,
testmultiprec 5/5; re-run with the verifier suite added: kuna-base
verify_w1_harness_unittests 3/3, kuna-num verify_w1_harness_unittests 1/1 —
green in **both debug and `--release`**.
clippy: `cargo clippy -p kuna-base -p kuna-num --all-targets -- -D warnings`
-> clean (including the new verifier tests).
mechanical greps over the item diff (rust-port d144c94..4c03e6c, +730 lines):
no `todo!`/`unimplemented!`/`HashMap`/`HashSet`/`sort_unstable`; three `as
i32` casts, each with an adjacent prose justification (F2).
port-audit: still no `port-audit` crate in the workspace (carried gap,
w1-base-foundation F4); ran `python3 tools/rust-port/check_checklist.py`
-> OK, and verified the two cpp blob shas manually.
NOTE: the checklist-declared gate `cargo test -p kuna-harness` runs **0
tests** (vacuously green) — see F1.

## Method

Re-read both C++ files end to end and re-derived each TEST's semantics, then
diffed statement-for-statement against the item's own commit (4c03e6c:
testmarshal.rs +640, testmultiprec.rs +90; nothing else in the diff).
Coverage audit: C++ testmarshal.cc has 18 `TEST(...)` (the XML variant of
closemismatch deliberately does not exist upstream — XmlDecode::closeElement
only checks under CPUI_DEBUG, marshal.cc:197-210) and testmultiprec.cc has 5;
the Rust files contain exactly the same 18 + 5 `#[test]`s under the same
names, same constants, same assertion order. Structural adaptations audited:

- The C++ shared helpers (`test_signed_attributes` etc.) take outStream +
  encoder + decoder; Rust splits each at the `istringstream
  inStream(outStream.str())` line into `*_encode`/`*_decode` halves because
  PackedEncode/XmlEncode mutably borrow the buffer. Statement order inside
  each half verified unchanged against the C++ line ranges cited in the
  file's comments (spot-checked every helper).
- The three identical C++ encode preambles of test_noremaining /
  test_openmismatch / test_closemismatch (testmarshal.cc:437-440, 458-461,
  477-480 — byte-identical `<input><off/></input>`) are consolidated into
  one `test_input_off_encode`; decode halves stay separate and faithful.
- The C++ static `theEnviron`/`spcManager` (testmarshal.cc:47-66) becomes a
  per-test `the_environ_build()` constructing the same single space:
  `AddrSpace(IPTR_PROCESSOR,"ram",bigend=false,size=8,ws=1,ind=3,
  hasphysical,dl=1,dead=1)` — parameter-for-parameter match against
  space.rs `AddrSpace::new` (space.rs:229-239) and the C++ ctor call
  (testmarshal.cc:53); `t->isBigEndian()` correctly resolved to Translate's
  default `false`. Per-test rebuild is observably equivalent (the C++
  static is only ever read through `getSpace(3)`/decoder construction).
- `DummyTranslate` dropped: it exists only to seed `isBigEndian()`; no
  Translate method is otherwise reachable from these tests. Correct.
- C++ overload pairs map to the `_id`-suffixed Rust methods
  (`openElement(ELEM)` -> `open_element_id`, `readSignedInteger(ATTRIB)` ->
  `read_signed_integer_id`, ...) — checked each call site against the
  Decoder/Encoder traits (marshal.rs:571-761).
- String constants verified **byte-for-byte by script** against the C++
  source: the 214-byte `longString` (3-literal concatenation incl. trailing
  spaces and the 18x"blah" run; Rust `\`-continuations strip correctly) and
  the escape-heavy CONSTRUCTOR string (UTF-8 Euro bytes, quotes, backslash,
  \n\t) — both exact.
- `ASSERT_EQUALS(spc, spcManager->getSpace(3))` (pointer compare,
  testmarshal.cc:284) -> `Rc::ptr_eq` — the right identity analogue.
- testmultiprec: constants byte-identical; C++ passes raw `uint8[2]`
  pointers, `udiv128` copies into locals before writing outputs
  (multiprecision.cc:288-333, no input mutation), so Rust's `&[u64;2]`
  inputs + reuse of `NUM2` across calls is faithful. The C++ uninitialized
  `q`/`r` become zero-init (see F3, fenced by a verifier test).
- `marshal_bufferpad` does not call `theEnviron.build()` in C++ (it reuses
  the static left by earlier tests, or null if run alone — never
  dereferenced); the Rust test builds the fixture explicitly and documents
  the divergence. Not observable: the manager is only stored, never read.

## Hunt list

- signedness: **clean.** `uint4 flags` -> u32 with u32 literals; `int4
  alignVal/extrapopVal` -> i32; `uint4 attribId`/element ids -> u32;
  bufferpad `int4 i` loop -> `0..511i32`. No mixed-sign comparisons
  introduced or dropped.
- integer widths: **clean.** All `int8` reads stay i64; the two C++
  int8->int4 truncating stores (testmarshal.cc:232/234 expect-string
  results, :357 readSignedInteger) are transcribed `as i32` with comments
  (values 456/800/0x1000 fit; bit-exact either way) — see F2 for the
  missing `// cast:` tag. 0x8000000000000000 stays u64.
- wrapping: **clean / n/a.** No arithmetic in the harness beyond `i & 1`
  and flag ORs. (Signed-extreme encode/decode wrap behavior of the marshal
  layer additionally fenced by verifier test, both profiles.)
- comparator totality: **n/a** (no comparators in scope).
- iteration-order provenance: **clean.** Every loop is either a C++
  `for(;;)` over the decoder's attribute stream (order = stream order, both
  languages) or an index loop; no maps/sets anywhere in the item diff.
- off-by-one / do-while / reverse iterators: **clean.** `for(;;)`+break ->
  `loop`+break (same break points); `for(int4 i=0;i<511;++i)` -> `0..511`;
  no do-while in scope. Bufferpad's exact-fill arithmetic (1+2*511+1=1024)
  re-derived and extended to the 1022/1026/2048 cells by verifier test.
- erase-while-iterating: **n/a** (no containers mutated).
- exception -> Result partial-state parity: **clean.** All four C++
  `try{...}catch(DecoderError&)` frames (unexpected_eof, noremaining,
  openmismatch, closemismatch) become `?`-closures / direct
  `matches!(.., Err(KunaError::Decoder{..}))` with the try-block boundary at
  the same statement (ingestStream inside the frame for unexpected_eof,
  single call for the other three). A non-Decoder error leaves the flag
  false and fails the assert — same suite-level outcome as an uncaught C++
  exception. No partial state is reused after the catch in any of the four.

## Findings

- F1 (minor): the item's tests do not live in the checklist-declared crate,
  so the checklist-declared gate is vacuous.
     cpp: n/a (process) — spec anchors: docs/rust-port/checklist.json
     (`w1-harness-unittests`: `"crate": "kuna-harness"`, `"gate": "cd rust
     && cargo test -p kuna-harness"`), docs/rust-port/plan.md:43
     ("`kuna-harness` | ported unit tests, ...").
     rust: rust/crates/kuna-base/tests/testmarshal.rs:1,
     rust/crates/kuna-num/tests/testmultiprec.rs:1 (actual placement);
     rust/crates/kuna-harness/src/lib.rs (crate has no tests).
     claim/observed: the commit gates on `cargo test -p kuna-base --test
     testmarshal` / `-p kuna-num --test testmultiprec` (real, green), but
     `cargo test -p kuna-harness` exits green with **0 tests run** — if
     wave-gate automation replays checklist gates, this item can never go
     red. Placement itself is defensible (tests sit beside the code under
     test and run on every `cargo test -p kuna-base`); the bookkeeping is
     what's wrong.
     repro: `cd rust && cargo test -p kuna-harness` -> "running 0 tests".
     work order: update the checklist entry's `crate`/`gate` to match the
     real placement (and the W2/W6/W7 unit-test items' plans accordingly),
     or re-home the tests under kuna-harness. Either is a one-liner; not
     blocking the port's correctness.
- F2 (note): three bare `as i32` casts carry prose justifications but not
  the `// cast:` tag verification.md asks for.
     cpp: decompiler/unittests/testmarshal.cc:232,234 (int4 = intb
     expect-string results), :357 (int4 = intb readSignedInteger).
     rust: rust/crates/kuna-base/tests/testmarshal.rs:239-246, :400-401.
     Each mirrors the C++ truncating store at the same spot and the values
     (456/800/0x1000) are range-safe; fix opportunistically.
- F3 (note): `multiprec_udiv`'s outputs are zero-initialized where C++
  leaves them uninitialized — zero-init could mask a `udiv128` that fails
  to write a limb whose expected value is 0 (q[1]/r[1] in 3 of 4 cases).
     cpp: decompiler/unittests/testmultiprec.cc:31-32.
     rust: rust/crates/kuna-num/tests/testmultiprec.rs:29-31.
     Documented in the port; now fenced by the verifier's sentinel-prefill
     test (`verify_udiv128_writes_every_output_limb`), which proves every
     limb is written (pack32_64 zero-fills the tail, multiprecision.cc).
     No action needed.

## Adversarial tests

Committed in the worktree as e7620a5:

- rust/crates/kuna-base/tests/verify_w1_harness_unittests.rs
  - verify_bufferpad_off_by_one_and_two_buffer_cells — extends the C++
    exact-fill bufferpad cell (N=511 bools = 1024 bytes) to N=510/512/1023
    (1022 / 1026 / exactly-two-buffers 2048 bytes); for N>=512 the 2-byte
    bool attribute at k=511 straddles the 1024-byte ingest-buffer boundary
    (header byte last-of-buffer-1, data byte first-of-buffer-2), a path the
    C++ test never reaches. Full round-trip + peek==0 + clean close at
    every size.
  - verify_signed_extremes_roundtrip_both_codecs — i64::MIN, MIN+1, -1, 0,
    1, MAX through write/read signed on Packed AND Xml codecs (the C++
    suite tops out at 63 bits); pins the C++ x86-64 wrap-negation
    round-trip of INT64_MIN in both build profiles (ADR 0003).
  - verify_close_element_skipping_whole_document_parity —
    closeElementSkipping invoked at the document root over the full C++
    hierarchy document (6 nesting levels, skipped subtrees containing
    every attribute kind); both codecs must land on end-of-document with
    peekElement()==0 and no-arg openElement()==0 (C++ oracle behavior
    re-derived from marshal.cc:126-150 / :719-738 / :785-800).
- rust/crates/kuna-num/tests/verify_w1_harness_unittests.rs
  - verify_udiv128_writes_every_output_limb — the four C++ udiv cells with
    sentinel-prefilled (0xdeadbeef...) output arrays: every expected limb,
    including the zeros the ported test's zero-init would mask, is proven
    written (the F3 fence).

All 4 pass in debug and release; clippy -D warnings clean.

losses: none cited — every structural adaptation in the port is
behavior-preserving against the C++ oracle; no losses.md entry required.
ACCEPT.
