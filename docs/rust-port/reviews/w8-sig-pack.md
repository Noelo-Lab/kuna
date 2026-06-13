# w8-sig-pack
verdict: ACCEPT-WITH-LOSSES
verifier: claude-opus-4-8 (independent verifier subagent)
date: 2026-06-13
gate: `cd rust && cargo test -p kuna-decomp` -> item tests PASS (lib 1390/1390;
  verify_w8_sig_pack 3/3). One unrelated integration test fails:
  `corpus_bootstrap::ldefs_pspec_cspec_parse_matrix` ("Could not find .sla file
  for x86:LE:64:default") — a missing gitignored `make specs` BUILD ARTIFACT,
  pre-existing on `rust-port` (the test exists verbatim at
  rust-port:tests/corpus_bootstrap.rs:821) and untouched by this item's diff
  (the diff touches only signature.rs / paramid.rs / analyzesigs.rs). Environmental,
  not a port defect.

scope (C++): signature.{cc,hh}, analyzesigs.{cc,hh}, paramid.{cc,hh}
scope (Rust): rust/crates/kuna-decomp/src/{signature,paramid,analyzesigs}.rs

## hunt list
- **Signedness:** clean. `int4` loop indices (`for(int4 i=...)` over
  `vector`/`map`) become `usize`/range iteration; `noiseDominator`'s
  `for(int4 i=postOrder.size()-2;i>=0;--i)` -> `for i in (0..n-1).rev()` (empty when
  n<=1, matching the C++ `i=-1` no-run). `finger1/finger2` (int4 post-order
  indices) stay non-negative (every node's index set in `noise_post_order`;
  virtual root index = `post_order.len()`). `Signature::compare` (`sig != op2.sig`
  then `<`) ports the unsigned compare faithfully.
- **Integer widths:** clean. `hashword`=`u64`, `sig`=`u32` (the C++ `(uint4)h`
  truncation reproduced at every `Signature::new*` via `h as u32`). `opc as i32 as
  Hashword` matches `(hashword)op->code()` (OpCode is `#[repr(i32)]` with C++ enum
  values). `vndata.size = sz as u32` matches C++ `uint4 size = int4 sz`.
- **Wrapping:** clean. Every wrap-prone op uses the wrapping helper: `callhash`
  (`+= 100001` / `*= 0x78abbf`), `copyhash +=`, `accum +=` (commutative mix),
  `ophash.wrapping_add(0xfeedface)`. The `^ (x<<9) ^ (x<<18)` mixes (signature.cc:311,849)
  and `^ (val<<7)^(val<<14)^(val<<21)` (hashSize) use plain `<<` — VERIFIED faithful:
  Rust `<<` with an in-range shift amount (9/14/18/21 < 64) drops high bits exactly
  like C++ unsigned `<<` and does NOT panic on lost bits (overflow-checks only
  catches out-of-range shift AMOUNTS). Confirmed empirically against a maximal
  CPOOLREF tag offset (0xFFFF_FFFF_FFFF_FFFF -> ophash high32 = 0xFFFFFFFF) — the
  full pipeline runs without panic and deterministically (test
  `cpoolref_high_offset_must_not_panic`).
- **Comparator totality:** clean. `compare_ptr` is `a.sig.cmp(&b.sig)` (total on
  u32); `get_signature_vector` sorts `Vec<u32>` (`sort_unstable`, equivalent to C++
  `sort` for raw integers); `sort_by_hash` uses `sort_by(compare_ptr)`. No float
  comparators in scope.
- **Iteration-order provenance:** clean. `sigmap` = `BTreeMap<u32,_>` keyed by
  `getCreateIndex()` (matches C++ `map<int4,SignatureEntry*>` create-index order) —
  every traversal (`set_current_function`, `remove_noise` rootlist,
  `signature_iterate`, `collect_varnode_sigs`, `calculate_shadow`/`local_hash`
  passes) collects `.keys()` in that order. `blockmap` = `BTreeMap<i32,_>` keyed by
  `bl->getIndex()` (matches C++ `map<int4,BlockSignatureEntry*>`). `iter_loc` =
  loc_tree order (= C++ `beginLoc`), `iter_def_flag(input)` = def_tree order (= C++
  `beginDef(input)`), `descend_snapshot` = C++ descend-`list` order, block op walk
  via `basic_neighbours().1` = C++ `beginOp()..endOp()`. NO `HashMap`/`HashSet` in
  ported logic (grep clean).
- **Off-by-one / do-while / reverse iterators:** clean. `calculate_shadow`'s
  `for(;;)` -> `loop{}` keeps the at-least-once member-`op` rewrite (transcribed
  back as `entry.op = eff_op`). `noise_post_order` DFS faithfully reproduces the
  stack/`iter++`-before-recurse. `walkforward`'s `iter++`-every-iteration (even on
  `op==ignoreop`) is preserved (`iter.next()` advances each turn before the
  ignoreop guard).
- **Erase-while-iterating:** N/A — signature generation and paramid are read-only
  over the graph; no mid-traversal container mutation. `descend_snapshot` is a
  pre-collected `Vec`, equivalent to the live (non-mutating) C++ iterators.
- **Exception -> Result partial-state parity:** clean. The two throw sites
  (`GraphSigManager::new` bad-settings, `set_current_function` size threshold) raise
  before any output/overlay mutation; the `?`-propagation in
  `simple_signature`/`debug_signature` escapes before any encode output, matching
  the C++ throw-out-of-command.

## mechanical pass
- grep todo!/unimplemented!/HashMap/HashSet: NONE in the three item files.
- `sort_unstable`: 1 (signature.rs:1496) — on `Vec<u32>`, equivalent to C++
  `sort` (raw-integer order); accepted.
- bare `as`: all are width casts faithful to explicit C++ casts (`(uint4)h`,
  `(hashword)opc`, `getSize() as Hashword`, `.len() as i32`, post-order index
  `as usize`); no value-semantics drift.
- `cargo clippy -p kuna-decomp`: clean (no warnings on the lib). The
  `cargo clippy -p kuna-decomp --tests` error is entirely in pre-existing,
  untouched test files (verify_w4_fw_options, ruleaction_4, ...) — none of
  signature.rs/paramid.rs/analyzesigs.rs/verify_w8_sig_pack.rs is flagged.

## findings
(none — no blocker or major. F1, a suspected `ophash<<18` overflow-panic, was
RETRACTED after empirical verification: Rust `<<` with in-range amount drops high
bits like C++, no panic — see Wrapping above.)

## accepted divergences (losses)
- LOSS-115: `simpleSignature` drops the trailing `<call>` element list (W4
  `FuncCallSpecs` surface unthreaded) — the `<sig>` payload is byte-identical;
  only call-site address annotations are missing.
- LOSS-116: `print signatures` renders the sorted bare-hash list, not C++
  `SigManager::print`'s insertion-order + per-feature `printOrigin` string (W8
  print surface). Affects only the human-readable console command; the XML
  encode paths are unaffected.
- Pre-existing declared SEAMs covered by the above + the in-file `// SEAM`
  comments: `VarnodeSignature::encode`/`BlockSignature::encode`
  (Varnode/PcodeOp/Address marshal, W8), `ParamMeasure::encode`
  (`Datatype::encodeRef`, W8), `ParamIDAnalysis(justproto=true)` and the
  `encode`/`savePretty` model/extrapop header (recovered `FuncProto`, W4). These
  are body-fills against ported element ids, not re-derivations.

## adversarial tests
rust/crates/kuna-decomp/tests/verify_w8_sig_pack.rs (3 tests, all pass):
- `cpoolref_high_offset_must_not_panic` — drives a maximal CPOOLREF tag offset
  through `set_current_function`+`generate` (the high-bit `<<` mix path);
  asserts no panic and a deterministic feature vector across two runs.
- `simple_signature_feature_vector_is_sorted_and_deterministic` — asserts the
  feature vector is sorted ascending and that `simple_signature` emits exactly
  one `<sig>` per feature.
- `paramid_direct_read_hits_terminal_rank` — an input read directly by INT_ADD
  ranks DIRECTREAD under both the best (min) and worst (max) walks (terminal-rank
  short-circuit).
