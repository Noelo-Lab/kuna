# w1-base-util

## Round 1

verdict: ACCEPT-WITH-LOSSES
verifier: independent verifier agent (Claude, not the porter)
date: 2026-06-10
branch: worktree-wf_cea42066-041-5 (worktree `.claude/worktrees/wf_cea42066-041-5`, reviewed at b188eea; adversarial tests committed as a82cdaf)
cpp blob shas: all 6 in-scope files match the checklist entry (`git hash-object` verified: crc32.cc 8e8f129b, crc32.hh 87c61ae8, compression.cc 81b8c59b, compression.hh 1151d6d9, filemanage.cc 9e03ab2d, filemanage.hh 9c2ab1b6)

gate: `cd rust && cargo test -p kuna-base` -> ok. 52 passed (lib) + 6 passed
(`verify_w1_base_foundation`) + 7 passed (verifier tests
`verify_w1_base_util`), 0 failed.
clippy: `cargo clippy -p kuna-base --tests -- -D warnings` -> clean
(workspace `disallowed-types` denies HashMap/HashSet per ADR 0002).
port-audit: `cargo run -p port-audit -- --item w1-base-util` NOT runnable —
no `port-audit` crate exists in the workspace yet (carried from foundation
review F4); blob-SHA check performed manually instead.

## Method

Re-derived the semantics of every function in `crc32.{cc,hh}`,
`compression.{cc,hh}`, `filemanage.{cc,hh}` and diffed against
`git diff rust-port...worktree-wf_cea42066-041-5` (crc32.rs / compression.rs
/ filemanage.rs, 1,365 insertions). Then built **independent C++ oracle
harnesses against the vendored sources** (compression.cc linked with C zlib;
filemanage.cc run plain and under `-fsanitize=address,undefined` with
byte-identical output; crc32.cc for raw register vectors) and pinned traces
the Rust port must reproduce. Oracle outputs are committed as
`rust/crates/kuna-base/tests/data/w1_util_{compression,filemanage}_oracle.out`.

Key empirical results (all reproducible from the committed oracle files):

- **Inflate is call-for-call identical to C zlib**: feeding the exact
  zlib-produced 3916-byte stream in 7-byte chunks through a 64-byte output
  buffer (the `slaformat.cc:235` ingestStream consumption pattern), every
  per-call `avail_out` return value, the finish point, and the payload match
  (`B:` trace, 600+ calls).
- **Deflate output bytes diverge at the default level** (the only level the
  tree uses): 3902 bytes vs zlib's 3916, differing from byte 2 onward, and
  the per-call deflate return traces differ (`A:` C++ `62 64 64 64 0...54`
  vs Rust `64 64 64 64 0...2`). Interop verified in BOTH directions: the
  Rust port decodes the zlib stream (above) and C zlib decodes the
  miniz-produced stream (`REV: len=5000 match=1 fin=1` harness). ->
  **LOSS-010** (appended by this review, refines LOSS-004).
- **Byte-identical edge streams pinned**: empty stream at level -1
  (`789c030000000001`, also matches `python3 zlib.compress(b'')`),
  stored-block output for a 300-byte incompressible payload at levels 1 and
  9 through 1-byte output buffers (312 deflate calls each, identical bytes).
- **After-stream-end behavior matches**: a second `inflate` after
  `Z_STREAM_END` returns 64/no-error with finished still true (`C:` line);
  a second `deflate(finish=true)` after completion does NOT throw in C++
  (zlib returns `Z_STREAM_END`; `E:ret 64`) and the port returns `Ok(64)`.
- **splitPath's wrapped-size_type arithmetic is faithfully clamped**: 20
  oracle vectors including `"/"` (C++ `end = size()-2` wraps to npos; port's
  explicit clamp produces the same `("/","")`), `"a/"` -> `("","a/")`
  (bounded `find_last_of` misses the trailing separator), `"a//"` ->
  `("a/","")`, `""` (C++ reads `full[size()-1]` out of bounds; observable
  result `("","")` matches), and UTF-8 components.
- **discoverGhidraRoot machinery matches end-to-end**: the aliased
  `splitPath(cur,cur,base)` component walk with the `.`/`..` skiplevel
  arithmetic reproduces all 15 oracle walks (including leftover `skip=1`
  cases and the faithful odd elements `"a/"`, `"./"`, `""`), and the
  development / dotted-argv0 / install / not-found layouts resolve
  identically on a real fixture tree.

## Hunt list

- signedness: **clean.** Every C++ signed/unsigned comparison site is
  resolved explicitly and commented: `level + 2 >= pathels.size()`
  (filemanage.cc:340, int promoted to size_t) -> `(level + 2) as usize`
  with caller-nonnegativity noted (filemanage.rs:315,345); `int i` loop
  indices vs `.size()` (filemanage.cc:330,403) -> i32/enumerate;
  deflate `level` int -> match on `-1 | 0..=9` (compression.rs:60-67).
- integer widths: **clean.** `int4 sz` buffer parameters become slice
  lengths; `avail_out` int4 returns are `i32` (buffers ≤ 4096, cannot
  truncate — commented at compression.rs:109-111); C++
  `int sizebefore = cur.size()` truncation (filemanage.cc:379) is
  immaterial and documented (filemanage.rs:370).
- wrapping: **clean.** The only wrap-bearing C++ arithmetic is splitPath's
  `size()-1`/`size()-2` size_type wrap for `""`/`"/"` (filemanage.cc:309-311)
  — ported as explicitly clamped `search_end`/`base_end`
  (filemanage.rs:254-269) and oracle-pinned (`S|` vectors). `crc_update` is
  xor/shift/index only. flate2 `total_in/total_out` deltas are monotonic u64
  differences bounded by buffer sizes.
- comparator totality: **n/a.** No comparators or sorts in ported logic
  (test-only `Vec<String>::sort` is total).
- iteration-order provenance, per loop: **clean.** `pathlist` iteration
  (findFile/matchList; C++ `vector` order) -> `Vec` insertion order,
  first-path-wins oracle-tested; directory scans (C++ `readdir` order,
  filesystem-defined) -> `read_dir` (same provenance class — C++ itself is
  unordered here; in-tree consumers count/filter, tests sort);
  `scanDirectoryRecursive` visits `directoryList` output in list order both
  sides. No HashMap/HashSet anywhere in the diff (grep + clippy deny).
- off-by-one / do-while / reverse iterators: **clean.** `flushInput`'s
  `do{}while(outAvail==0)` (compression.cc:136-140) -> `loop`+break-at-end
  keeps at-least-once semantics — pinned by the empty-flush oracle (`D:`
  emits the 8-byte trailer from a zero-length finish). The bounded
  `find_last_of(separatorClass,end)` -> `rposition` over
  `bytes[..=search_end]` boundary inclusion is oracle-pinned (incl. the
  `"a/"` miss). The 4096-chunk boundary of CompressBuffer (C++ reserves the
  4096th slot for the overflow char, flushing exactly-4096-byte chunks;
  Rust fills to 4096 then flushes on the next write) produces identical
  chunk sizes, and Z_NO_FLUSH boundaries are not observable in deflate
  output (porter's `test_compression_buffer_matches_direct_compress`).
- erase-while-iterating: **n/a.** None in scope.
- exception -> Result partial-state parity: constructor failures match
  (deflateInit level rejection -> `Err`, same explain string; inflateInit's
  only failure is OOM -> infallible `new()` documented). One note-level
  divergence on the inflate error path (F3).

## Findings

- F1 (note, accepted as LOSS-010): default-level deflate byte stream
  diverges from C zlib.
     cpp: decompiler/cpp/compression.cc:46-57 (deflate over zlib)
     rust: rust/crates/kuna-base/src/compression.rs:100-112
     claim/observed: LOSS-004 substitutes flate2/miniz_oxide; this review
     demonstrates the compressed bytes are NOT identical at level -1 (3902
     vs 3916 bytes for the oracle payload, divergent from byte 2) and the
     per-call deflate return values differ. Decompression of zlib streams
     is call-for-call identical, and interop holds both directions.
     repro: `g++ -I decompiler/cpp /tmp/.../comp_oracle.cc
     decompiler/cpp/compression.cc -lz` vs
     `verify_compression_default_level_stream_interop_loss010`.
     Disposition: LOSS-010 appended (refines LOSS-004's restoration gate to
     content equality, since byte-identical recompress is impossible
     without binding C zlib).
- F2 (minor): bare `as` casts without the protocol's `// cast:` tag.
     cpp: n/a (port-introduced bookkeeping)
     rust: rust/crates/kuna-base/src/compression.rs:107,108,261 (u64-delta
     -> usize, i32 -> usize); filemanage.rs has prose justifications at
     each cast site but not the `// cast:` tag format.
     claim/observed: all enumerated casts are provably lossless in context
     (deltas bounded by 4096-byte buffers, indices nonnegative); same shape
     as foundation review F1. Non-blocking; fix opportunistically.
- F3 (note): inflate-error partial-state parity.
     cpp: decompiler/cpp/compression.cc:84-96 (throw after ::inflate may
     have already consumed input/produced output; z_stream keeps the
     partial cursor)
     rust: rust/crates/kuna-base/src/compression.rs:178-189 (`Err` branch
     skips the `in_pos` update while flate2's internal counters advanced)
     claim/observed: a caller that catches "Error decompressing stream" and
     keeps using the same Decompress would re-feed bytes C++ would not. No
     in-tree caller does (slaformat.cc:235 ingestStream propagates and
     abandons the object). Not oracle-observable today; flag if a recover
     path ever appears.
- F4 (note): CompressBuffer flush doc overstates the C++ contract.
     cpp: decompiler/cpp/compression.cc:158-163 (sync); E: oracle line
     rust: rust/crates/kuna-base/src/compression.rs:204-213 module doc
     ("call it exactly once")
     claim/observed: C++ does NOT throw on a second sync/finish (zlib
     returns Z_STREAM_END; oracle `E:ret 64`), and the port also returns
     `Ok` — behavior parity holds; only the doc comment implies otherwise.
- F5 (note, accepted as LOSS-011): std::fs emulation deviations
     (read_dir omits "."/".." — observable only via `allowdot=true`, which
     no in-tree caller uses; non-UTF-8 names skipped; getcwd 256-byte cap
     dropped; dangling-symlink stat in C++ reads uninitialized stbuf (UB)
     -> deterministic false in Rust).
     cpp: decompiler/cpp/filemanage.cc:177-206,245-279,92-102,263-267
     rust: rust/crates/kuna-base/src/filemanage.rs:14-23 (module docs),
     134-158, 167-207, 57-63, 185-189
     claim/observed: the dot-entry divergence is pinned exactly by
     `verify_filemanage_fs_layouts_match_cpp_oracle` (Rust result == C++
     result minus exactly the two dot entries); the UB paths are outside
     the oracle. LOSS-011 appended.
- F6 (note): the relative-argv0 branch of `discover_ghidra_root`
     (filemanage.cc:391-401 cwd walk; filemanage.rs:390-406) is verified by
     re-derivation only — an end-to-end oracle run is cwd-nondeterministic.
     The Rust panic on `pathlist[0]` when `current_dir()` fails replaces
     C++ UB (documented inline).
- F7 (note): `cargo run -p port-audit` still not executable (no such crate);
     blob SHAs checked manually. Same infrastructure gap as foundation F4.

## Adversarial tests

Committed in the worktree as a82cdaf
(`rust/crates/kuna-base/tests/verify_w1_base_util.rs` +
`tests/data/w1_util_{compression,filemanage}_oracle.out`; oracle outputs from
the verifier's own C++ harnesses, filemanage oracle ASan/UBSan-clean):

- verify_crc32_raw_register_vectors_match_cpp_oracle — raw crc_update
  register vectors from the vendored crc32.cc (0..=255 sweep, 1000-byte
  feed from 0xffffffff, wide-val masking, all-ones register).
- verify_compression_inflate_trace_matches_cpp_zlib_oracle — per-call
  inflate return-value trace on the exact zlib bytes (ingestStream
  consumption pattern) + inflate-after-stream-end (C: line).
- verify_compression_deflate_byte_parity_edges_match_cpp_oracle — empty
  stream bytes, deflate-after-finish (E: line), stored-block levels 1/9
  through 1-byte buffers (call counts + bytes).
- verify_compression_default_level_stream_interop_loss010 — LOSS-010
  canary: header parity + self round-trip of the diverged default-level
  stream; auto-quiesces if miniz ever becomes byte-identical.
- verify_filemanage_split_path_matches_cpp_oracle — 20 splitPath vectors
  incl. all wrapped-size_type edges and UTF-8.
- verify_filemanage_component_walk_matches_cpp_oracle — 15 aliased
  splitPath walks with skiplevel parity.
- verify_filemanage_fs_layouts_match_cpp_oracle — discoverGhidraRoot
  dev/dotted/install/none layouts, scanDirectoryRecursive depth sweep with
  nested-match no-descend, and the allowdot dot-entry loss pin.

losses: LOSS-004 (cited by the port), LOSS-010 and LOSS-011 (appended by
this review).
