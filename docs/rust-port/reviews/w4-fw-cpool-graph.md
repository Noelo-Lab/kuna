# w4-fw-cpool-graph
verdict: ACCEPT-WITH-LOSSES
verifier: claude-opus (independent verifier, round 1)
date: 2026-06-11
gate: `cd rust && cargo test -p kuna-decomp` -> PASS (13/13 test binaries green; lib 232/232, all per-module packs green, verifier pack `verify_w4_fw_cpool_graph` 8/8); `cargo clippy -p kuna-decomp -- -D warnings` clean

## Scope

C++: `decompiler/cpp/cpool.{cc,hh}` (248+212), `graph.{cc,hh}` (502+28),
`callgraph.{cc,hh}` (468+128).
Rust: `rust/crates/kuna-decomp/src/cpool.rs` (+968), `src/graph.rs` (+746),
`src/callgraph.rs` (+1090).
Branch: `worktree-wf_a6e29a1b-137-4` (c730345, after the verifier test commit).

The port is a careful, near-complete structural transcription of all three
files. cpool: the full `CPoolRecord` field set + accessors, the `encode`/`decode`
round-trip (tag-string map, `<value>`/`<data>`/`<token>` children, ctor/dtor
flag bits, `MAX_STRING_SIZE` bound check, the 16-byte-wrap byte dump and the
`istringstream >> ws >> hex` tokenizer as `HexByteTokens`), `createRecord`
duplicate error, `CheapSorter` lexicographic key, BTreeMap container ordering.
graph: all three Renoir dumpers, the verbatim preamble/attribute blocks, the
alive-op traversal, the `LOAD/STORE/BRANCH/CALL`/`INDIRECT` input-range special
cases, the set/clear-mark bookkeeping, the block/dom edge passes. callgraph: the
full edge-redundancy + complement-index model, `addEdge`/`insertBlankEdge`/
`deleteInEdge` re-indexing, `findNoEntry`/`snipCycles`/`snipEdge`/`cycleStructure`
spanning-tree + cycle snip, the `pop/pushPossible` leaf walk, encode/decode.

Two genuine seams are deferred to W4/W6 with explicit error/seam-trait surfaces
(not silent breakage): `CallGraph::build_all_nodes`/`build_edges` (symbol-table
scope iteration + `FuncCallSpecs`) return `Err(...SEAM...)`; the `Datatype`
encode/decode (`type->encodeRef`, `TypeFactory::decodeType`) routes through the
`CPoolTypeRef`/`CPoolTypeFactory`/`CPoolTypeRefProvider` seam traits; the
`Varnode::printRawNoMarkup` register-name branch routes through the
`GraphVarnodePrinter` seam. The pure data-structure + algorithm half — the
algorithmically risky bulk of all three files — is ported fully.

## Hunt list

- **Signedness:** clean. The `-Wno-sign-compare` sites are all `int4 idx` vs
  `vector::size()` where the index is provably `>= 0` (`snipCycles` `st >=
  outedge.size()`, `addEdge`/`pushPossible` loop bounds, `insertBlankEdge`
  `i >= slot`). Each is reproduced as a signed `int4` compare with the length
  `as int4`; no silent flip. `findNoEntry` lownode tie-break is strict `<` in
  both (first min-in-edge node in address order wins) — faithful.
- **Integer widths:** `byteDataLen` (int4) bounded by `MAX_STRING_SIZE`; the
  Rust loops the `int8`-typed length but the count is identical. `(uint1)val`
  narrowing reproduced as `v as uint1`. `parentedge = walked` (uint4->int4)
  reproduced as `walked as int4` (seed index, small). All index<->usize casts
  are structural pointer-arithmetic analogues. No promotion-into-wider-type
  arithmetic in scope.
- **Wrapping:** the only arithmetic on decoded values is `HexByteTokens`
  `val.wrapping_mul(16).wrapping_add(digit)`. See F2 — a bounded divergence vs
  C++ `>>` saturation, but unreachable on encode-produced data (only 2-hex-digit
  tokens, never overflow). The complement `+= 1`/`-= 1` bookkeeping is plain
  signed int4 arithmetic on small in-range indices; cannot wrap.
- **Comparator totality:** `CheapSorter` derives `Ord` (a then b, total) matching
  C++ `operator<`; `Address` Ord (verified in W3) is total and is the `addEdge`
  out-edge sort key and the BTreeMap key. No float/partial compare. Clean.
- **Iteration-order provenance, per loop:** every C++ `map<Address,_>` /
  `map<CheapSorter,_>` traversal (`findNoEntry`, `clearMarks`, `encode`,
  container `encode`) is reproduced over a `BTreeMap` in the same key order;
  loops that mutate during traversal (`find_no_entry`, `clear_marks`) collect
  the index list first then iterate — order preserved. `outedge`/`inedge`
  vector-index loops preserve insertion/sorted order. No `HashMap`/`HashSet` in
  ported logic. Clean.
- **Off-by-one / do-while / reverse iterators:** `cycleStructure`'s
  `do { } while(!allcovered)` ported as `loop { ...; if allcovered { break } }`
  (at-least-once kept; exercised by the pure-cycle test). `snipCycles`'s
  `--end()`-free stack walk and `deleteInEdge`'s `for(j=i+1;...)` /
  `for(j=fromi+1;...)` down-shifts are index-faithful. Clean.
- **Erase-while-iterating:** `deleteInEdge` is the only erase; it shifts both
  the callee `inedge` and caller `outedge` vectors down and `pop()`s the tail,
  fixing complements with the same `>= fromi` / `>= i` predicates as C++ — a 1:1
  index transcription (the C++ `>= i` quirk that can drive a complement to -1 is
  preserved, pinned by the porter's `delete_in_edge_removes_both_sides` test).
  Clean.
- **Exception -> Result partial-state parity:** **FINDING F1** —
  `ConstantPool::decode_record` reorders `createRecord` and `record.decode`
  relative to C++. Everything else faithful: cpool `decode` size-check throw,
  missing-`<data>` throw, callgraph `findNode`-miss throws all match.
- **Aliasing:** `insertBlankEdge`'s `nodeout->inedge[...]` bump when
  `nodeout == node` (self-recursion) is two sequential `self.nodes[...]` borrows
  in Rust (reads `to`/`complement` into locals first) — faithful to the separate
  C++ in/out vectors. Verified by `callgraph_self_recursion_snips_and_terminates`.
  Clean.

## Findings

  - F1 (minor): `ConstantPool::decode_record` decodes the `<cpoolrec>` body
       into a temporary record *before* `create_record` inserts/duplicate-checks
       it; C++ `createRecord`-then-`decode-in-place` does the reverse. On the
       happy path (no duplicate, well-formed) the result and stream consumption
       are identical. On the error path the partial state differs: C++ leaves a
       blank (or partially decoded) record in the map and, for a duplicate ref,
       throws *before* consuming the body (stream left at the dup's start); Rust
       leaves the map unchanged and, for a duplicate ref, consumes the body
       *then* errors. Both still abort the decode with an error, and a partial
       constant pool on a fatal XML decode failure is not a recovery point in
       the decompiler — so this does not force REJECT, but it is a real
       partial-state / stream-position divergence. Recorded as LOSS-041.
       cpp: decompiler/cpp/cpool.cc:169-175 (`decodeRecord`), :199-208
            (`createRecord` throws on duplicate before any body decode)
       rust: rust/crates/kuna-decomp/src/cpool.rs:532-545

  - F2 (info, not a divergence on valid data): `HexByteTokens::next_hex_u32`
       uses `wrapping_mul`/`wrapping_add` and `as uint1`, whereas C++
       `istringstream >> hex` saturates to `UINT_MAX` + failbit on >8-hex-digit
       overflow. Unreachable on encode-produced data (every emitted token is
       exactly two hex digits, max 0xff). Differs only on hand-authored XML with
       a >8-digit hex run, where C++ would store `(uint1)UINT_MAX = 0xff` and
       Rust stores the wrapped low byte. The encode path never produces such
       input; flagged for completeness, no loss recorded (no observable
       divergence for any value the encoder can emit). The `0x`-prefix branch in
       the tokenizer is likewise dead on encode output.

  - F3 (info, SEAM consequence): `add_node_func`'s duplicate-entry error message
       uses the node's stored display-name where C++ uses the existing
       `Funcdata::getName()` (source name). Reachable only with two distinct
       Funcdata at one address; the existing funcdata's `getName()` is not even
       addressable through the W4 `FuncId` seam (node stores no `Funcdata*`).
       Error-string-only, on an error path, under a declared seam. No loss.
       cpp: decompiler/cpp/callgraph.cc:213
       rust: rust/crates/kuna-decomp/src/callgraph.rs:410-413

## Losses

  - LOSS-041 (this review): cpool container decode partial-state / stream-position
    divergence on a malformed or duplicate `<cpoolrec>` (F1). Accepted because
    both implementations abort the decode with an error and a partially-built
    constant pool is not a decompiler recovery point; the only observable
    difference (residual map entry, decoder cursor) is behind a fatal error.

  - The two deferred drivers `CallGraph::build_all_nodes`/`build_edges` are
    non-functional SEAM stubs (explicit `Err`), pending the W4 symbol-table /
    `FuncCallSpecs` ports; consistent with the W4 framework-skeleton scoping
    (cf. w4-fw-options seam handling). Tracked here, not separately numbered, as
    they will be completed (not diverged) by the depending W4/W5 items.

## Adversarial tests

Added `rust/crates/kuna-decomp/tests/verify_w4_fw_cpool_graph.rs` (committed on
the branch, c730345), 8 tests over the public API:
  - `cpool_bytedata_roundtrip_crosses_newline_wrap` (PASS — 33-byte `<data>`
    dump across two 16-byte wrap newlines, incl. 0x00/0xff, decoded via the real
    `CPoolRecord::decode`)
  - `cpool_duplicate_error_names_existing_token` (PASS — duplicate error echoes
    the *existing* token, not the replacement)
  - `cpool_container_lexicographic_after_roundtrip` (PASS — out-of-order inserts,
    incl. `(a,0)` vs `(a,1)` vs `(a+1,_)` and a u64::MAX `b`, survive round-trip)
  - `callgraph_insert_before_existing_fixes_complements` (PASS — middle-insert
    shift keeps every callee's in-edge cross-link 1:1)
  - `callgraph_self_recursion_snips_and_terminates` (PASS — one-node in/out
    aliasing; self back-edge snipped; each node visited once)
  - `callgraph_disconnected_forest_covers_all_callees_first` (PASS — multi-seed
    forest, callees before callers per tree)
  - `callgraph_pure_cycle_pseudo_root_terminates` (PASS — all-in-a-cycle SCC
    forces the pseudo-root + do-while re-seed; terminates, covers each node once)
  - `callgraph_roundtrip_preserves_structure` (PASS — encode/decode rebuilds the
    out-degrees and the shared two-in-edge callee)

(`complement` is crate-private with no public getter, so the complement-index
re-indexing is verified through its observable consequence — the 1:1 in/out
cross-link count — rather than the raw index. Noted as a minor API-surface gap,
not a porter bug.)

## Disposition

ACCEPT-WITH-LOSSES. The algorithmically risky core (callgraph spanning-tree /
cycle snip / complement bookkeeping / leaf walk, and the cpool decode tokenizer
and BTreeMap ordering) is faithfully transcribed and survives adversarial
boundary/aliasing/cycle/order tests; the gate and clippy are clean. The single
real divergence (F1) is a narrow, error-path-only partial-state mismatch behind
a fatal decode error, recorded as LOSS-041; F2/F3 are unreachable-on-valid-data
/ declared-seam notes. The two deferred build drivers are explicit seams, not
silent breakage, consistent with the W4 framework-skeleton scope.
