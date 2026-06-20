# w7-harness-kunaregion
verdict: ACCEPT
verifier: Claude Opus 4.8 (1M context) [independent verifier]
date: 2026-06-13
gate: `cd rust && cargo test -p kuna-decomp --test testkunaregion` -> ok (3 passed: regionid_diamond0, regionid_diamond1, regionid_loop)

## Scope

C++ in scope: `decompiler/unittests/testkunaregion.cc` (3 TESTs, 22 assertions) — the
kuna-owned angr `RegionIdentifier` (S7) unit tests. The diff's `kuna_regiongraph.rs`
and `kuna_regionid.rs` belong to the earlier port items `w7-s7-kuna-regiongraph` /
`w7-s7-kuna-regionid` (already verified in their own rounds); the only file this item
contributes is `rust/crates/kuna-decomp/tests/testkunaregion.rs` (commit 6d2cb17).

## Assertion-by-assertion parity (all 22 C++ assertions covered)

diamond0: top!=0 (l.76 -> .expect l.126); numNodes==2 (l.77->129); getHead()->getAddr()==1
(l.78->head_addr l.130); !isCyclic (l.79->131); regionCount==1 (l.87->direct_subregions
l.141); addrs==6 (l.92->143); got[i]==i+1 (l.95->145-147); cyclics==0 (l.96->148).
diamond1: top!=0 (l.130->189); numNodes==2 (l.131->190); addrs==8 (l.135->194);
got[i]==i+1 (l.138->196-198).
loop: top!=0 (l.156->216); !isCyclic (l.157->217); cyclics==1 (l.162->222);
loop head==2 (l.164->224); loopAddrs==2 (l.168->227); loopAddrs[0]==2 (l.169->228);
loopAddrs[1]==3 (l.170->229); found (l.182->241); addrs==4 (l.185->244);
got[i]==i+1 (l.188->246-248). No assertion dropped or weakened.

## Reconstruction faithfulness (the only deviations from literal transcription)

The external test crate has no node-pool access, so three pointer-level C++ reads are
reconstructed through the public API; each was re-derived and confirmed equivalent:
  - `head_addr()` (C++ `getHead()->getAddr()`): reads `get_head()` id, finds its key in
    `region.get_graph().node_keys()`. Sound because the head node lives inside its own
    graph (kuna_regionid.hh l.59) and `node_keys()` is the BTreeMap (KunaNodeOrder)
    iterator == C++ `beginNode()`. ADVERSARIAL 1 forces an entry whose addr is NOT the
    graph minimum to rule out a min-key shortcut.
  - `direct_subregions` (C++ `for(...top->getGraph()) if isRegion() ++`): counts regions
    entered while stack depth == 1. ADVERSARIAL 2 pins the closed identity
    `direct_subregions + top-leaf-blocks == top.num_nodes()` (count==1 here, not the
    naive 2 — two diamonds nest one level deep, matching the implementation's actual tree).
  - cyclic head/body (C++ `walkRegionBlocks(loop, loopCol)` re-walk): captured during the
    single walk while the cyclic frame is open. ADVERSARIAL 3 confirms the open-frame body
    recurses through a nested sub-region inside the loop, exactly as walkRegionBlocks does.

## hunt list
- Signedness: clean. `direct_subregions: i32` == C++ `int4 regionCount`; loop indices are
  `usize` from `enumerate()` compared to u64 via `(i+1) as u64` (mirrors C++ `(uintb)(i+1)`,
  values <= 8, no truncation).
- Integer widths: clean. addresses u64 (uintb); the `(i+1) as u64` casts are test-local on
  tiny indices.
- Wrapping: clean. No arithmetic beyond `i+1` on small loop indices.
- Comparator totality: clean. Only `sort_unstable` on distinct-address Vecs (C++ uses
  `std::sort` in `sortedAddrs`, also non-stable); no ties in any input.
- Iteration-order provenance: clean. `stack.iter_mut()` (Vec insertion order), `rbba.iter()`
  (Vec index order == C++ `for i<rbba.size()`), `node_keys()` (BTreeMap KunaNodeOrder ==
  C++ beginNode). No HashMap/HashSet.
- Off-by-one / do-while: clean. `.take(6/8/4)` with `enumerate()` == C++ `for(i=0;i<N)`;
  `got.len()` asserted == N first so `.take(N)` covers all.
- Erase-while-iterating: n/a (test-only, no mutation during traversal).
- Exception -> Result parity: clean. C++ `ASSERT(top!=0)` -> `.expect(...)`; the empty/not-
  computed paths are covered by the implementation item's own verify tests.
- This wave (HighVariable merge / varmap order / block-structuring schema precedence):
  n/a to a test harness; the structuring precedence lives in kuna_regionid (separate item).

## Mechanical pass
- `cargo clippy -p kuna-decomp --test testkunaregion -- -D warnings`: clean.
- `cargo clippy -p kuna-decomp --test verify_w7_harness_kunaregion -- -D warnings`: clean.
- greps (testkunaregion.rs): no `todo!`/`unimplemented!`/`HashMap`/`HashSet`. `sort_unstable`
  x2 (matches C++ non-stable std::sort, distinct keys). Bare `as`: `(i+1) as u64` x3 —
  test-local index->addr cast mirroring C++ `(uintb)(i+1)`, no `// cast:` comment but benign
  (small constants; not ported production logic).
- port-audit crate not present in this tree; per-item blob-sha drift check N/A.

findings: none.

adversarial tests (rust/crates/kuna-decomp/tests/verify_w7_harness_kunaregion.rs):
  - head_addr_is_entry_not_min_key
  - direct_subregions_matches_top_region_node_count
  - cyclic_body_includes_nested_region_blocks
  All 3 pass against the implementation; clippy-clean. (An initial draft of test 2
  asserted `>= 2` direct sub-regions for two series diamonds; diagnosis showed the real
  tree nests them one level deep, so the test was corrected to the exact closed identity
  — confirming the harness reconstruction is faithful, not the test's naive expectation.)

losses: none.
