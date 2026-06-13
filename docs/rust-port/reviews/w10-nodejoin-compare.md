# w10-nodejoin-compare
verdict: ACCEPT
verifier: claude-opus-4-8 (independent verifier, round 1)
date: 2026-06-13
gate: `cd rust && cargo test -p kuna-decomp` -> ok (1601 lib + integration tests, 0 failed)
scope: the Rust diff `rust-port...rport/w10-nodejoin-compare` touches ONLY
  `rust/crates/kuna-decomp/src/blockaction.rs` (+187/-37) and its `tests.rs`
  (+210). It ports `ConditionalJoin::execute` and its three helpers; it does
  NOT touch the compare-form / printc layer. The "present-compare" half named in
  the commit message was already on `rust-port` (prior `w10-structure-printbody`
  / W6 `kuna_compareform`), not produced by this branch (proven below).

## What this branch actually is
This branch CLOSES **LOSS-103** (`blockaction.cc:2094` `ConditionalJoin::execute`
was a seam returning `KunaError::lowlevel`). The four functions are now ported
verbatim from the vendored upstream `blockaction.cc` (unchanged vs rust-port,
commit 17b3c5e):
  - `setup_multiequals`  (cc:2023)  newOp/opSetOpcode(MULTIEQUAL)/newUniqueOut/opSetInput/opInsertEnd
  - `move_cbranch`       (cc:2043)  opUninsert+opInsertEnd surviving CBRANCH, repoint cond, opDestroy dup
  - `cut_down_multiequals` (cc:1981) opRemoveInput(hi)/opSetInput(lo)/1-input->COPY rewrite
  - `execute`            (cc:2094)  nodeJoinCreateBlock + the three above, in C++ order
`ApplyNodeJoin::apply` now mirrors C++ exactly: `count += 1; execute(); clear(); break`.
LOSS-103's restoration criteria ("land opSetOutput/opSetOpcode and nodeJoinCreateBlock
op-side so execute can build and cut down the join MULTIEQUALs") are now fully met —
LOSS-103 should be marked resolved. No NEW loss is introduced, so this is a clean ACCEPT.

## FAITHFULNESS (primary focus)
- The mutation sequence is a line-by-line transcription with C++-anchor comments
  on each statement. `execute` order = nodeJoinCreateBlock, setupMultiequals,
  moveCbranch, cutDownMultiequals(exita), cutDownMultiequals(exitb) — exact.
- `cbranch1->getAddr()` is read once and reused; cbranch1 is not mutated before
  setupMultiequals (moveCbranch runs after), so caching is equivalent to the C++
  per-iteration re-read. Faithful.
- `move_cbranch` reads vn1/vn2 BEFORE opUninsert, sets input AFTER insert, then
  opDestroy(cbranch2) — matches cc:2046-2056.
- `cut_down_multiequals` removes `hi` then sets `lo` (hi>lo, so no index shift);
  re-checks numInput()==1 fresh after the removes; matches cc:2004-2015.
- The `vn1==vn2` branches in BOTH move_cbranch (vn=vn1) and cut_down_multiequals
  (opRemoveInput only) correctly DO NOT consult mergeneed — those keys were never
  inserted (findDups/checkExitBlock skip vn1==vn2). Verified by mutation test.
- Not a shortcut to render the compare string: there is NO compare/printc code in
  the diff at all. The node-join is genuinely the IR-mutation port it claims.

## NO SPECIAL-CASING
grep of the item diff and the two changed files for
`boolless|dat_52|<= 10|function-name|0x1040|"main"|special.?case|hardcod`:
the ONLY hit is a test comment asserting the ABSENCE of special-casing. No
hardcoded names/addresses/strings anywhere. -> clean.

## THE NEW DATATEST PASS IS REAL (boolless `if (dat_52 <= 10)`)
Re-ran on the branch (`cargo test -p kuna-decomp --test print_b5_boolless`):
4/4 pass. Captured the engine output with `--nocapture`:
```
void boolless(void)
{
  if (dat_52 <= 10) {
    ACC = 1;
  }
  return ACC;
}
```
and the analyzed IR the printer consumes:
```
0xa002:2:  u0x00000600:1 = CPUI_INT_LESSEQUAL(i0x52:1,#0xa:1)
```
So `dat_52 <= 10` is produced by a REAL `CPUI_INT_LESSEQUAL(i0x52,#0xa)` in the
IR, not a printer string. The anti-special-casing guard
`w10_boolless_lessequal_comes_from_real_ir_not_printer` asserts a real
INT_LESSEQUAL exists AND no raw INT_LESS survives — passes. NOT vacuous, NOT
hardcoded.
(Provenance caveat, not a defect: this same test ALSO passes on `rust-port`
without this branch's change — the boolless `<= 10` win is inherited from the
prior structure-printbody/compareform waves and is orthogonal to the node-join
port. The commit-message phrasing "node-join + present-compare (boolless ...)"
overstates the causal link, but the pass is genuine and the compare mechanism
was verified under W6.)
Spot-check of two other compare-affected renders vs the C++ oracle: the C++
oracle suite is untouched and PARITY OK (see below); the only compare-rendering
code paths (`kuna_compareform`, `printc::opIntLess`) are unchanged by this branch,
so their previously-verified W6 behavior stands.

## Hunt list
- Signedness: in1/in2/a_in*/b_in* are all `int4` (= C++ int4); `in1 > in2`,
  `get_in(int4)`, `op_remove_input(int4)` all signed-consistent with C++. clean.
- Integer widths: no int4*int4/intb promotion in this path; only literal `newOp(2,..)`
  and slot indices. clean.
- Wrapping: no uintb arithmetic on this path. clean.
- Comparator totality: `merge_pair_less` (create-index u32 then u32, `.cmp`) is a
  total order, no floats. clean.
- Iteration-order provenance: setup_multiequals iterates the sorted `mergeneed`
  Vec (mirrors std::map key order); cut_down/check iterate the intrusive op list
  via op_head/basic_neighbours (= beginOp..endOp insertion order). No HashMap/HashSet
  in the diff (grep NONE). clean.
- Off-by-one / do-while / reverse iter: no do-while; the numInput()==1 COPY-rewrite
  boundary verified by adversarial test 3 (3-input merge -> stays MULTIEQUAL). clean.
- Erase-while-iterating: cut_down_multiequals captures `next` BEFORE
  opUninsert/opInsertBegin, so the re-inserted-at-front op is not re-visited —
  exactly C++ `++iter` before inserts. move_cbranch's opDestroy(cbranch2) is not
  inside any iteration. clean.
- Exception->Result partial-state: C++ execute is `void`/cannot fail; the Rust
  returns Ok always with inner `.expect` on impossible bank-lookup failures, and
  `apply` `.expect`s execute — faithfully matching C++'s no-error-path / UB-on-
  corruption semantics (old graceful-Err swallow is correctly removed). clean.

## Mechanical pass
- `cargo run -p port-audit`: N/A — `port-audit` crate not present in this workspace,
  and the checklist has no `w10-nodejoin-compare` entry with a recorded cpp_blob_sha.
  Instead verified blockaction.cc is the vendored upstream blob, unchanged vs rust-port.
- grep todo!/unimplemented!/HashMap/HashSet/sort_unstable in diff: NONE.
- grep bare `as` casts in diff: NONE.
- clippy `cargo clippy -p kuna-decomp --tests`: no warnings anchor to the new
  node-join code or the verifier tests (only pre-existing crate-wide doc/style
  nits unrelated to this diff).

## Gates run
- `cargo test -p kuna-decomp` (item gate): ok, 0 failed.
- `cargo test --workspace`: 3166 passed, 0 failed (3163 baseline + 3 verifier tests).
- C++ oracle `python -m kuna.run_tests --all --baseline docs/baseline.json`:
  207/207 unit, 675/675 datatest assertions, PARITY OK (untouched).

## Adversarial tests (added by the verifier, in the item crate, marked w10-nodejoin-compare)
Added to `rust/crates/kuna-decomp/src/blockaction/tests.rs` (with a shared
`cj_build_split` topology builder; no name/address special-casing):
  - `cj_cutdown_same_varnode_branch_no_mergeneed_lookup` — exit MULTIEQUAL merges
    the SAME varnode both sides: exercises cut_down's `vn1==vn2` opRemoveInput-only
    branch (must NOT touch mergeneed) and the 1-input->COPY collapse.
  - `cj_movecbranch_shared_condition_uses_vn1_not_mergeneed` — boolless-shape:
    both CBRANCHes read the same condition varnode; move_cbranch must set `vn=vn1`
    without a mergeneed lookup, keep the surviving CBRANCH on the shared cond, and
    opDestroy the dup.
  - `cj_cutdown_keeps_multiequal_when_extra_pred_remains` — 3-input exit MULTIEQUAL
    with a third (unrelated) predecessor: after cut-down numInput==2, so it must
    STAY a MULTIEQUAL (off-by-one guard on the numInput()==1 COPY rewrite).
All 3 pass. Non-vacuity proven by mutation: forcing the `vn1==vn2` cut-down branch
to `false` (always-lookup) makes test 1 panic "cutDownMultiequals: mergeneed entry
missing"; reverting restores green.

losses: none (this item CLOSES LOSS-103; no new loss introduced).
