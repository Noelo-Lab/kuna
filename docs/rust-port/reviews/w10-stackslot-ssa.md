# w10-stackslot-ssa
verdict: ACCEPT-WITH-LOSSES
verifier: claude-opus-4-8 (1M context)
date: 2026-06-14
branch: rport/w10-stackslot-ssa @ c9e8a5d (verifier tests committed on top: 6839878)

## Scope note (item framing vs actual diff)

The task FOCUS described the heritage stack/spacebase guard machinery
(`guardStores`/`guardLoads`/`LoadGuard`/`StoreGuard`, the spill/reload fold via
copy-prop + `RuleLoad`/`StoreVarnode`). The ACTUAL diff does NOT touch that
machinery. The wave un-seams `RuleMultiCollapse::applyOp` (ruleaction.cc:3253)
— the LOSS-056 deferral — and adds its two helper dependencies
`Funcdata::cseFindInBlock` (funcdata_op.cc:1365) and `BlockBasic::earliestUse`
(block.cc:2826). The commit reframes "stack-slot SSA / spill-reload fold" as
exactly this MULTIEQUAL collapse. I verified the diff that exists, against the
C++ it actually transcribes. The framing mismatch is recorded but is not itself
a defect: the diff stands on its own as a faithful port of a real seam closure.

gate: `cd rust && cargo test --workspace` -> 3258 passed / 0 failed (base);
3261 / 0 with the 3 verifier tests added. `cargo clippy --workspace --lib` ->
clean. C++ oracle `python -m kuna.run_tests --all --baseline docs/baseline.json`
-> 207/207 unit + 675/675 datatests, PARITY OK (untouched). Rust-engine
datatests base==branch: 47/456 assertions, 28 execution-failures (NO regression,
measured on both trees with the release binary).

## FOCUS findings

1. FAITHFULNESS — PASS. `apply_op`, `cse_find_in_block`, `block_earliest_use`
   are line-by-line transcriptions of the cited C++ (verified each branch). The
   functional-equality machinery they reuse (`functional_equality{,_level}`) was
   ported and tested in w5-s3-condexe-expression and is invoked correctly
   (the scratch-buffer init `[outvn1,outvn1]` is harmless — the buffers are
   write-before-read scratch in `functionalEqualityLevel`).

2. NO SPECIAL-CASING — PASS. `git diff ... -- rust/crates/kuna-decomp/src/`
   grepped for address/offset/function-name/register hardcoding: the only hits
   were `functional_equality_level(...) == 0` (the faithful C++ `==0` match
   sense). All `edi`/`condconst`/`0x14` hits are in test code/comments. Zero
   hardcoding in production code.

3. THE FOLD IS REAL, AND DOES NOT OVER-FOLD — PASS.
   - REAL: base (rust-port) renders condconst_copy with TWO `v1 = ZEXT(a0)`
     (one inside `if (a0==10)`); branch renders ONE (the MULTIEQUAL of two
     functionally-equal `ZEXT(a0)` collapsed, emptying the if-body). Observable,
     proven by AT1/AT2/VT2.
   - DOES NOT OVER-FOLD: a genuinely-aliased reload with distinct SSA values
     (the intervening-write case) is NOT folded. `functional_equality` returns
     false for `COPY(edi)` vs `COPY(esi)` (distinct non-constant inputs), so the
     rule declines. Proven by my VT1 (distinct-SSA) and the porter's AT3
     (constant-vs-register). The condconst_conn `0x14`-path phi correctly is
     NOT collapsed.

4. NEW POSITIVE PASSES — NONE (so none to scrutinize). The Rust datatest tally
   is byte-identical base==branch (47/456, 28 exec-fail). The fold is real but
   latent: condconst_conn's `&&`-fold needs the unported `Merge::buildDominantCopy`
   HighVariable phase. Honestly reported in the commit. -> LOSS-160.

5. BOOLLESS + READSTRUCT + EXEC-FAILURES — PASS. boolless B5 (print_b5_boolless)
   11 passed/1 ignored; readstruct (verify_w10_struct_corpus) 16 passed.
   Execution-failure count flat at 28 (base==branch). C++ oracle 675/675
   untouched.

## Hunt list

- Signedness: CLEAN. The only comparisons in the diff are `getOrder()` `<`
  comparisons; C++ `order` is `uintm` (unsigned), Rust `u32`, both unsigned `<`.
- Integer widths: CLEAN. `numinput`/`newin` are `int4` from `op_num_input`;
  used only as loop bounds and `Vec::with_capacity` hints.
- Wrapping: N/A. No arithmetic on `uintb`-lineage values in the diff (no `+`/
  `-`/`*` on offsets); only equality/order comparisons and pushes.
- Comparator totality: N/A. No new comparator/sort; `block_earliest_use` takes
  a min by `getOrder()` (total order on u32).
- Iteration order: CLEAN. `cse_find_in_block`/`block_earliest_use` collect
  `descend_iter()` (insertion-ordered `SmallVec`, mirrors C++ `list<PcodeOp*>`)
  before iterating; `cse_find_in_block` returns the first match in that order,
  matching C++. No HashMap/HashSet anywhere in the diff.
- Off-by-one / do-while: CLEAN. `while j < matchlist.len()` with `j+=1` and
  in-loop `matchlist.push` faithfully mirrors C++ `while(j<matchlist.size())`
  with `matchlist.push_back` (size re-evaluated each iteration). The
  `earliest->getOrder() < res->getOrder()` strict-`<` boundary is preserved
  (equal order is NOT skipped).
- Erase-while-iterating: CLEAN. The collapse loop iterates `skiplist.clone()`
  (the C++ skiplist is a fixed local vector at that point — the body never
  pushes to it); graph mutations (`total_replace`/`op_destroy`) affect descend
  lists, not the cloned Vec. Each skiplist entry is distinct (marks prevent
  re-add) so each `copyr` is processed once and never read after its op is
  destroyed.
- Exception->Result partial-state: CLEAN. The `?`/`.expect(...)` points
  (`total_replace`, `op_set_all_input`) are on infallible-in-practice paths;
  the C++ does not throw here. The failure path clears all marks (matched by
  the `for &v in skiplist.iter() { vn_clear_mark }` tail), verified by VT1's
  no-leaked-mark assertion.

## Mechanical pass

- port-audit: NOT RUN — the `port-audit` crate is not in the workspace
  (`w0-infra-port-audit` is still `todo`). Not a blocker for this item.
- grep todo!/unimplemented!/HashMap/HashSet/sort_unstable: NONE in the diff.
- bare `as` casts: two — `numinput as usize` and `newin as usize`, both
  `Vec::with_capacity` hints (int4 input-count, non-negative). Uncommented
  (no `// cast:`); they are capacity hints that cannot affect correctness.
  Informational nit, not a finding.
- `cargo clippy --workspace --lib -- -D warnings`: clean. (`--all-targets -D
  warnings` fails on PRE-EXISTING lints in `universalaction/tests.rs` and other
  untouched test files — also failing on the rust-port base; not this wave.)

## findings

  - F1 (info): two uncommented `as usize` casts (Vec::with_capacity hints).
       cpp: decompiler/cpp/ruleaction.cc:3256,3340 (vector default-construct)
       rust: rust/crates/kuna-decomp/src/ruleaction_3.rs:776,886
       Capacity hints only; non-blocking.
  - F2 (info): the re-pinned implied-vars test asserts on a Rust-internal
    artifact (`v1 = ZEXT(a0)`) that does NOT exist in the C++ oracle render of
    condconst_copy (which is `void`, writing globals). The test's anti-over-
    inline intent is preserved; the gross base divergence is pre-existing and
    out of this wave's scope. Recorded in LOSS-160 surface.
       cpp-oracle: condconst_copy = `void ... { glob1=0; glob2=d; ...; return; }`
       rust: rust/crates/kuna-decomp/tests/verify_w10_implied_vars_adversarial.rs:390

## adversarial tests (committed 6839878, marked w10-stackslot-ssa)

- vt1_w10_stackslot_ssa_aliased_reload_distinct_ssa_does_not_fold (the over-fold
  tripwire: MULTIEQUAL of COPY(edi)/COPY(esi) must NOT collapse; also asserts no
  leaked mark on the surviving output)
- vt2_w10_stackslot_ssa_funceq_fold_consumes_the_phi (the func-eq fold actually
  consumes the phi; exercises cseFindInBlock/earliestUse + the order boundary)
- vt3_w10_stackslot_ssa_nested_multiequal_branch_collapses (matchlist-extension
  + skiplist-mark path: phi(edi, phi(edi,edi)) collapses, both phis destroyed,
  no leaked mark)

All 6 (3 porter + 3 verifier) pass against the ported rule.

## losses

LOSS-160 (appended): the seam is genuinely closed and the fold is real, but it
produces no NEW positive datatest pass at this pipeline stage — converting the
fold into a flipped condconst_conn assertion needs the unported
`Merge::buildDominantCopy` HighVariable phase. Real-but-latent transform, the
LOSS-156/158/159 dormant-downstream pattern. C++ oracle untouched; Rust tally
base==branch (no regression, no fabricated positive).

## verdict rationale

ACCEPT-WITH-LOSSES. The port is a faithful line-by-line transcription with no
special-casing; the fold is real and provably declines on genuinely-aliased
(distinct-SSA) reloads; boolless/readstruct byte-identical; exec-failure count
flat; C++ oracle 675/675 PARITY OK; full workspace green; lib clippy clean. The
single accepted loss (LOSS-160) is that the closed seam is output-latent (no new
positive datatest) because its downstream HighVariable merge phase is unported —
honestly reported and ledgered.
