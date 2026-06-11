# w2-sleigh-emulate

verdict: ACCEPT-WITH-LOSSES
verifier: independent verifier agent (round 1; not the porter)
date: 2026-06-11
gate: `cd rust && cargo test -p kuna-sleigh` -> ok (45 lib tests + 7 new
      emulate verifier tests + 6 loadimage verifier tests + 9 translate
      verifier tests, 0 failed); full `cargo test --workspace` -> 34 suites
      ok, 0 failed.
      `cargo run -p port-audit -- --item w2-sleigh-emulate`: tool does not
      exist in the workspace (skipped; blob shas re-checked by hand:
      `git hash-object` on the six in-scope files gives
      memstate.cc=874909f0..., memstate.hh=7f1ccfe4...,
      emulate.cc=b26bf24d..., emulate.hh=953b0450...,
      emulateutil.cc=73fcfec9..., emulateutil.hh=31434941..., all matching
      the checklist `cpp_blob_sha`).

## Scope reviewed

C++ in scope: decompiler/cpp/memstate.{cc,hh} (947 LOC), emulate.{cc,hh}
(1014 LOC), emulateutil.{cc,hh} (577 LOC). Rust diff: exactly commit
33c4ec4 on worktree-wf_6343a4ba-016-11 — rust/crates/kuna-sleigh/src/
memstate.rs (+1475), emulate.rs (+1791), emulateutil.rs (+755). The branch
also carries the already-reviewed w2-sleigh-translate and
w2-sleigh-loadimage work (translate.rs, loadimage.rs, loadimage_xml.rs,
space.rs, address.rs + their verifier tests); not re-reviewed. Dependency
surfaces this item leans on were spot-re-checked against C++:
`Address::operator+(int8)` (address.hh:87/454 -> address.rs:545,
`wrap_offset(offset.wadd(off as u64))` — matches the
`current_address + instruction_length` int4->int8 promotion in
fallthruOp); `calc_mask`'s `(uint4)size` clamp (address.rs:1624);
`PcodeOpRaw` get/set surface incl. `get_opcode`'s unconditional behavior
deref and `get_input(i32)` UB-as-panic (pcoderaw.rs:283-397); the
`get_space_from_const` manager-index encoding (pcoderaw.rs:158, LOSS-015).

Architecture mapping (verified coherent): abstract `MemoryBank` splits
into `MemoryBankCore` + a `MemoryBank` trait with the C++ default
getPage/setPage bodies as provided methods (the TranslateBase seam
precedent); `map<uintb,uint1*>` pages -> `BTreeMap<u64, Vec<u8>>` with the
destructor's delete-loop as Drop (ADR 0002); `Emulate` splits into
`EmulateCore` + trait with non-virtual executeCurrentOp as a provided
method; C++ `EmulateMemory`'s method bodies become free functions in
`emulate_memory` that the concrete engine's `impl Emulate` delegates to
(the delegation is the inheritance edge; EmulatePcodeCache's two C++
overrides — executeBranch, executeCallother — provide their own bodies and
do not delegate, correctly); the C++ `varcache` disappears because the
Rust `PcodeOpRaw` owns varnodes by value; throws -> `KunaError` Results
with the C++ explain strings (ADR 0004); HOST_ENDIAN pinned to 0 against
the little-endian oracle host (the w1-base-foundation review pinned
HOST_ENDIAN the same way), making every `(uint1*)&curval` reinterpretation
a `to_le_bytes`/`from_le_bytes` pair; `EmulateSnippet`'s `Architecture*`
replaced by its two used slices (loader + manager).

## C++ oracle harness

A standalone differential oracle was compiled and run for this review
(g++ -O0 -Wno-sign-compare, little-endian host): verbatim transcriptions
of MemoryBank::setValue/getValue (memstate.cc:182-294), the default
getPage/setPage word loops (93-171), set/getChunk (302-359),
MemoryHashOverlay::insert/find + constructor (551-614),
EmulatePcodeCache::executeBranch's uintm arithmetic (emulate.cc:404-421),
and EmulateSnippet::executeBranch's `(int4)offset` truncation
(emulateutil.cc:204-219). Probe pins:

- A (ws=4 hash bank): `setValue(0x100,4,0x1122334455667788)` stores the
  FULL uintb through the `size == wordsize` early insert and
  `getValue(0x100,4)` returns it UNMASKED through the early `return val1`
  -> 1122334455667788 (defined C++ behavior, not UB); sub-word read
  0x102/2 -> 5566; spill 0x106/4 -> find(0x104)=0000aabbccdd0000,
  find(0x108)=aabb, getValue=aabbccdd.
- B (big-endian ws=8): unaligned non-spill 0x103/4 ->
  find(0x100)=000000aabbccdd00; spill 0x10e/4 -> find(0x108)=1122,
  find(0x110)=3344000000000000.
- C/D (default page word loops via chunk API): LE page-crossing
  setChunk(0x108,24) -> words 0807060504030201 / 100f0e0d0c0b0a09 /
  1817161514131211, getChunk(0x110,12) = 09..14; BE full-word
  byte_swap path -> find(0x100)=1122334455667788, partial-word
  find/patch path setChunk(0x108,[de,ad,bf]) -> deadbf0000000000.
- E (executeBranch): `uintm id = (uintb)-2` truncates to 0xFFFFFFFE;
  `id + (uintm)2` wraps around 2^32 to current_op=0, in-bounds.
- F (snippet): `(int4)(uintb)-2` = -2; pos 3 -> 1.

The Rust port matched every probe (the seven `verify_w2emulate_*` tests).

## Hunt list

- signedness: clean. Every C++ int4-vs-unsigned comparison identified and
  transcribed with the sign-extending conversion: `i < userops.size()`
  (emulate.cc:31 -> emulate.rs:218), `current_op < / >= / == opcache.size()`
  (establishOp/fallthruOp/executeBranch -> emulate.rs:891/1012/1015/1089),
  `index >= memspace.size()` (memstate.cc:626/640 -> memstate.rs:1020/1037),
  snippet `pos > opList.size()` evaluated only after the explicit `pos < 0`
  short-circuit, order kept (emulateutil.cc:212 -> emulateutil.rs:385).
  Noted, not a finding (F3): `sz < sizeof(uintb)` in getLoadImageValue is
  signed-vs-size_t in C++ but a plain i32 compare in Rust — divergent only
  for sz < 0, which requires a varnode size > 2^31.
- integer widths: clean. `uintm uniq` -> u32 with the uintb uniqReserve
  constructor truncation kept (emulate.cc:104-109 -> emulate.rs:737);
  executeBranch's uintb->uintm->int4 chain transcribed exactly and
  oracle-pinned (probe E; the porter's u64::MAX error case plus this
  review's valid backward-branch wrap); snippet `(int4)vn->offset`
  truncation oracle-pinned (probe F); `int4 size` params carried as slice
  lengths with `// cast:` per the workspace convention; `uint4
  VarnodeData::size as i32` at every evaluate/setValue call site matches
  the C++ implicit uint4->int4.
- wrapping: clean. The spill-path shifts that can hit a count of 64 only
  out of contract use wshl/wshr (x86 count-masking, consistent with the
  HOST_ENDIAN little-endian-oracle pin); in-contract shifts stay bare with
  bound comments (verified: gap/skip <= 56 in contract); construct_value
  uses wshl/wadd; `uniq.wadd(1)`, `pos.wadd(rel)` (C++ signed-overflow UB
  -> deterministic wrap, documented); hash probe `offset.wadd(collideskip)
  % size` with the int4->uintb sign-extending modulus.
- comparator totality: clean. `BTreeMap<Address,_>` (addresscallback) over
  the W1-verified total Address Ord (clippy mutable_key_type allow
  correctly justified: Ord reads only the immutable space index +
  offset); `BTreeMap<u64,_>` (pcodecallback, tempValues),
  `BTreeMap<u64,Vec<u8>>` (pages). No float keys, no partial_cmp.
- iteration-order provenance, per loop: clean. opcache/opList vectors —
  translation emission order in both; userops vector — index order =
  the C++ `(uintb)i` key derivation; hashtable probe sequence —
  transcribed exactly (start slot, +1023 stride, 0xBADBEEF marker);
  page BTreeMap — iterated only by Drop (C++ only by the destructor);
  addresscallback/pcodecallback — lookup-only (`find`) in both; the ONE
  C++ map iteration in scope, BreakTableCallBack::setEmulate's walk over
  both maps (emulate.cc:54-68), is removed with the back-pointer paradigm
  (LOSS-024) and had no other observable effect. No HashMap/HashSet.
- off-by-one / do-while / reverse iterators: clean. Both default-page
  do-while loops -> `loop` + tail `break` (at-least-once kept, including
  the size==0 degenerate that runs once in both); executeInstruction's
  `do { } while(!instruction_start)` -> loop + break-after-body — pinned
  by the instruction_start-quirk adversarial test (a relative branch as
  the first op leaves instruction_start true, so the C++ do-while executes
  exactly one op; Rust identical); fallthru/halt boundaries kept `==` vs
  `>=` exactly as C++ (snippet fallthruOp/executeBranch use `==`,
  pcode-cache fallthruOp uses `>=`); `if (size == wordsize)` early paths
  before/after `find` exactly as C++ (setValue inserts WITHOUT a find;
  getValue finds then returns unmasked — oracle probe A).
- erase-while-iterating: clean — none in scope. clearCache clears whole
  vectors (C++ delete loops -> Rc drops). C++ `currentOp` dangles after
  clearCache until establishOp; the Rust Rc keeps the old op alive —
  divergent only where C++ would deref freed memory (UB).
- exception -> Result partial-state parity: clean. createInstruction:
  cache cleared (possibly partially re-filled by an emitting-then-throwing
  oneInstruction) while instruction_length/current_op/instruction_start
  stay stale in both languages; fallthruOp advances current_address BEFORE
  the throwing createInstruction and skips establishOp in both — pinned by
  the eager-fallthru adversarial assertions (op executed + address
  advanced + error); executeCurrentOp's `?` skips fallthruOp at exactly
  the C++ throw points; set_chunk leaves earlier pages written;
  MemoryHashOverlay::insert full-table error mutates nothing in both;
  executeUnary/Binary evaluate-then-store order kept (an EvaluationError
  leaves the state unwritten in both).
- aliasing: clean. setExecuteAddress's copy-addr-BEFORE-createInstruction
  comment and order preserved (the C++ hazard — addr may point into the
  cache being cleared — is moot with the clone but the sequence matches);
  emulate_memory free functions clone the op/behavior Rcs up front,
  matching C++ member reads that precede any mutation.

## Findings

- F1 (minor; ledgered as LOSS-023): `EmulatePcodeOp` (emulateutil.hh:43-99,
  emulateutil.cc:22-148) is not ported — a deferral to the kuna-decomp IR
  wave (it emulates over PcodeOp/Varnode/FlowBlock + Architecture::userops,
  none of which exist yet; only in-tree subclass is jumptable.hh:110
  EmulateFunction, itself in that wave). Sound engineering call, correctly
  documented in the module docs, but the cut is inside this item's
  verified module scope and had NO losses.md entry — the accountability
  gap is closed by this review (the LOSS-021/022 precedent).
     cpp: decompiler/cpp/emulateutil.cc:22-148
     rust: rust/crates/kuna-sleigh/src/emulateutil.rs:15-20 (module docs)
- F2 (minor; ledgered as LOSS-024): the C++ `setEmulate` back-pointer
  (BreakCallBack/BreakTable) is replaced by an invocation-time
  `&mut dyn EmulateMemory` parameter; consequently a callback that drives
  the emulator far enough to re-consult its own break table — defined
  C++ behavior — panics on the RefCell double-borrow
  (execute_callother/execute_instruction hold the table borrow across the
  callback). No in-tree C++ consumer re-enters (the emulate.hh PutsCallBack
  pattern works identically); divergence is client-API-shaped only, and
  disclosed in the module docs. Ledgered for accountability.
     cpp: decompiler/cpp/emulate.cc:54-68,337 + emulate.hh:48,87,125-129
     rust: rust/crates/kuna-sleigh/src/emulate.rs:1048-1059 (and 952-958)
- F3 (informational): `get_load_image_value`'s `sz < 8` is a plain i32
  compare where C++ compares int4 against `sizeof(uintb)` (size_t,
  unsigned): for a negative sz C++ takes the mask branch, Rust the shift
  branch (then panics on the oversized shift). Unreachable: sz comes from
  `VarnodeData::size as i32` and would need a size > 2^31. The adjacent
  cast comment describes the C++ semantics but the code implements the
  signed compare — worth tightening on a later pass; no behavioral loss in
  any reachable execution.
     cpp: decompiler/cpp/emulateutil.cc:160
     rust: rust/crates/kuna-sleigh/src/emulateutil.rs:121-124
- F4 (informational; upstream, ledgered as UB-5): the default
  getPage/setPage adjust the first partial word against `addr` (page
  start) instead of `addr + skip`, so unaligned chunk I/O on a
  default-page bank (MemoryHashOverlay) overruns the caller's buffer in
  C++ (UB); the related full-word `*((const uintb *)val)` 8-byte read
  overreads for wordsize < 8. The port transcribes the arithmetic exactly;
  the overrun becomes a slice-bounds panic (ADR 0004, pinned by the
  should_panic adversarial test) and the overread zero-fills. Both
  anomalies were correctly identified and documented by the porter
  (memstate.rs module docs 1-2); this review adds the upstream-bugs.md
  entry.
     cpp: decompiler/cpp/memstate.cc:113-116,153-156,164
     rust: rust/crates/kuna-sleigh/src/memstate.rs:240-252,296-321
- F5 (out of scope, already flagged by the w2-sleigh-loadimage review as
  its F3): `cargo clippy -p kuna-sleigh --all-targets -- -D warnings`
  fails on the two pre-existing lints in the merged w2-sleigh-translate
  test code (translate.rs:1202 type_complexity, translate.rs:1437
  mutable_key_type). This item's three files and its verifier test target
  are clippy-clean under `-D warnings`.

Mechanical pass: no todo!/unimplemented!/HashMap/HashSet/sort_unstable in
any of the three files; every value-changing `as` cast carries a
`// cast:` justification on or adjacent to the line (spot-checked all
grep hits); the remaining bare `as` are slice-index/Vec-length casts
following the workspace convention, plus test-fixture code.

## Adversarial tests

rust/crates/kuna-sleigh/tests/verify_w2_sleigh_emulate.rs (committed on
the port branch as e2dd91b, all 7 passing):

- verify_w2emulate_hash_bank_wordsize4_unmasked_value_oracle
- verify_w2emulate_setvalue_bigendian_unaligned_spill_oracle
- verify_w2emulate_default_page_word_loop_chunk_oracle
- verify_w2emulate_unaligned_chunk_default_getpage_cpp_overrun_panics
- verify_w2emulate_backward_intra_instruction_branch_uintm_wrap
- verify_w2emulate_relative_branch_first_op_instruction_start_quirk
- verify_w2emulate_snippet_backward_cbranch_int4_truncation

losses: LOSS-023 (EmulatePcodeOp deferral), LOSS-024 (setEmulate
back-pointer paradigm / break-table re-entry panic) — both appended by
this review; upstream-bugs UB-5 appended for the C++ default-page
overrun the port intentionally does not replicate as UB.
