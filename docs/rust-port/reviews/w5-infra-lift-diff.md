# w5-infra-lift-diff
verdict: ACCEPT-WITH-LOSSES
verifier: opus-4.8 (independent verifier, round 2)
date: 2026-06-12
gate: `cd rust && cargo run -p kuna-lift-diff -- --self-test` -> 8/15 PASS, exit 0 (gate held; >= 8 structural PASS, every documented divergence/exclusion attribution holds)

## Scope

The W5 per-instruction Rust-vs-C++ lift/output differential, built as a **B2-boundary
STRUCTURAL differential**: it parses the C++ `break start heritage` `print raw` snapshot
into a `StructModel` and compares it against the model the Rust `FlowInfo` follow-flow +
basic-block builder + `structure_reset` produces — op count, per-op (address, coarse
op-class, output-presence), the block partition (cover `(start,stop)` as data), and
out-edges (by target-block index). Files: `rust/crates/kuna-harness/src/liftdiff.rs`,
`src/corpus.rs`, `tests/lift_diff.rs`, `tests/fixtures/*.b2.txt`,
`rust/crates/kuna-lift-diff/src/main.rs`, `tools/rust-port/liftdiff.py`.

Round 1 (same file, below in git history) REJECTed on F1-F5. This round re-derives the
whole thing against the C++ source and confirms each repair.

## Round-1 findings — disposition

- **F1 (was BLOCKER): blocks keyed by cover-start `BTreeMap`, collapsing shared-cover-start
  blocks → spurious PASS.** FIXED. The model is now an ordered `Vec<BlockRec>` matched
  across engines by POSITION (== block `index` in print order); cover `(start,stop)` is
  compared as data, edges are keyed by target-block INDEX. `parse_cpp_b2` validates
  position==index and rejects any non-`structureReset` capture (liftdiff.rs:589-597).
  `gh6990-returnpair` is now a genuine **5-block / 209-op PASS** (was 2 keyed blocks / 31
  ops), confirmed in the live gate run and pinned by
  `tests/lift_diff.rs::gh6990_is_a_validated_five_block_pass`.
- **F2 (was, coupled to F1): entry-head strip fired on any block sharing entry_start.**
  FIXED. The strip is gated on `i == 0` (the architectural entry block, matching
  `ActionConstbase`'s `getBasicBlocks().getBlock(0)`, coreaction.cc:699) — liftdiff.rs:516.
  Pinned by `verify_w5_infra_lift_diff.rs::entry_head_strip_is_entry_block_only_f2`.
- **F3 (was minor): DivergentBy attributions unchecked.** FIXED. `tests/lift_diff.rs::
  divergent_cases_have_pinned_count_deltas` pins the exact first-divergence signature
  (op-count delta or block-count) per DivergentBy case; a `generate_ops` count regression
  now flips an assertion.
- **F4 (was minor): op print order sorted by Rust `SeqNum.order` vs C++ list order.** The
  Rust extractor sorts each block's ops by the seqnum `order` field (corpus.rs:583); C++
  `BlockBasic::printRaw` walks the `op` list in list order (block.cc:2728). These coincide
  because at B2 the op-list IS in increasing-time/order (decode emits increasing
  `uniqid`/order, and the only post-decode head insertions are the excluded entry-head
  `opInsertBegin` ops). Now stated in the corpus.rs/liftdiff.rs docs; the gh6990 209-op PASS
  is end-to-end evidence. Not a blocker.
- **F5 (was BLOCKER): declared gate crate absent.** FIXED. `rust/crates/kuna-lift-diff` is a
  thin bin delegating to `kuna_harness::corpus::run_self_test`; `cargo run -p kuna-lift-diff
  -- --self-test` now resolves and exits 0.

## Boundary-equivalence argument (the focus) — re-derived against the C++ source

The snapshot is taken at `break start heritage`, INSIDE `universalAction` right before
`ActionHeritage` (coreaction.cc:5756). I re-checked the pre-heritage action table and group
membership against `decompiler/cpp/coreaction.cc`:

- Action order (5737-5756) matches the doc table EXACTLY: ActionStart, ActionConstbase,
  ActionNormalizeSetup, ActionDefaultParams, ActionExtraPopSetup, ActionPrototypeTypes,
  ActionFuncLink, ActionFuncLinkOutOnly, then the mainloop head ActionUnreachable,
  ActionVarnodeProps, ActionLowerSwitchInstall, ActionHeritage.
- `decompile` group set (`buildDefaultGroups`, 5683-5692) does NOT contain `normalanalysis`
  or `noproto`, so `ActionNormalizeSetup`/`ActionFuncLinkOutOnly` correctly DON'T run. The
  op-adding actions (`ActionConstbase`, `ActionExtraPopSetup`, `ActionFuncLink`,
  `ActionPrototypeTypes`) are all in `base`/`protorecovery` ⊂ `decompile` and DO run.
- **ActionConstbase entry-head exclusion is sound.** `ActionConstbase::apply`
  (coreaction.cc:693-721) inserts a single-input COPY per tracked-context register at
  `getBlock(0)` head via `newOp(1,bb->getStart())` + `opInsertBegin`. `PcodeOpBank::create`
  (op.cc:971-974) stamps the SeqNum time from a monotonic `uniqid++`, and `generateOps`
  runs to completion BEFORE any action — so the inserted COPY's time is strictly greater
  than every decode op at the entry address, while a decode op at the head is the
  SMALLEST-time op there. The parser's strip (liftdiff.rs:517-542) — leading entry-addr
  output op whose time strictly exceeds every OTHER entry-addr op, index-0 only — exactly
  matches this signature and is conservative (declines when it cannot prove). Verified
  against the real `floatprint`/`condconst`/`promotecompare`/`readvolatile`/`gh6990`
  fixtures (the excluded op is in every case the genuine `DF=#0x0`/`A0=#..`/`DECOMPILE_MODE`
  COPY, never a decode op).
- **ActionPrototypeTypes is invisible or excluded.** Its RETURN-input rewrite
  (coreaction.cc:4862-4869) and output-lock `opInsertInput` (4879) touch RETURN inputs only
  (no output, class unchanged) — invisible to the structural compare. Its other inserts
  (truncated-space ZEXT, 4901-4905; locked `extendInput`, 4836-4840) are all `opInsertBegin`
  at block-0 head with high time → caught by the SAME entry-head strip. None fire on the
  PASS cases (none input/output-locked, non-truncated).
- **The comparison is NOT optimistic.** Every DivergentBy case is REPORTED as a divergence
  with a pinned count/block delta; no unported insertion is silently absorbed into a PASS.
  The 8 PASS cases are call-free (or, for gh6990, the calls' placeholders don't fire) so
  ActionExtraPopSetup/ActionFuncLink provably added nothing — the per-block op-count
  identity is a checked invariant of the diff.

## Per-case exclusion/divergence justifications — checked

- `sbyte` EXCLUDED: the corpus maps `printf` `nocode` (`<com>map fun r0x101008 printf
  nocode</com>` in `sbyte.xml`) so its bytes are not in the image; the Rust `generate_ops`
  follows the CALL edge into unmapped bytes and seam-`Err`s. Legitimate W4-call-subsystem
  gap, correctly EXCLUDED (the C++ stops at the call via the symbol table). Not a hidden
  failure.
- `condexesub` / `gh1243-8051-addc` DivergentBy: the FIRST divergence is honestly the
  reported one (block count 1-vs-9 for the condexe lattice; a missing `write_volatile_1`
  inject op respectively). Note (F6-adjacent, minor): the `DivergentBy` doc-comment frames
  these as "unported pre-heritage action" divergences, but both are actually FLOW-TIME
  divergences (a CFG-shape split during decode; a volatile-access inject inside
  `generateOps`), not pre-heritage actions. The prose for each case (corpus.rs:247-261,
  235-246) is transparent about the real cause, and the outcome is DIVERGENT either way, so
  this is a taxonomy imprecision, not optimism. Left as a note, not a blocker.
- `ccmp`/`convert`/`nan`/`gp` DivergentBy: ActionFuncLink/ActionExtraPopSetup call
  placeholders; each pins an exact op-count delta (e.g. convert block#0 79->130).

## Hunt list

- Signedness: clean. `index/tidx as i64` (i32->i64 widening), loop `i: usize` vs `n: usize`
  with `i + 1 < n` bound-checked; no signed/unsigned compare hazard.
- Integer widths: clean. addresses/times `u64`, block index `i32`->`i64`/`usize`; the only
  arithmetic is `i + 1` on a `usize` loop index, bound-checked.
- Wrapping: clean. No arithmetic on `uintb`-lineage values; offsets parsed straight from hex.
- Comparator totality: clean. `sort_unstable` on `Vec<i64>` (`out_indices`, deduped) — total
  order on integers.
- Iteration-order provenance: clean (post-F1). Per-block ops collected into a `BTreeMap<i32,
  ..>` keyed by block index then sorted by seqnum `order` (corpus.rs:550,583); blocks walked
  in `bblocks_get_block(i)` list order; out-edges sorted+deduped. No `HashMap`/`HashSet`.
- Off-by-one / do-while / reverse iterators: clean. Fall-through `i+1` bound-checked; the
  implied-goto / terminator suppression matches `printRawImpliedGoto` (block.cc:2736-2752:
  `sizeOut()!=1`, target!=next, last-op-not-branch). The entry-head strip's `loop+break`
  keeps at-least-zero semantics correctly.
- Erase-while-iterating: n/a (parser builds, never erases mid-traversal).
- Exception->Result partial-state parity: n/a (flow `?` lands in the EXCLUDED bucket via
  `seam_reason`, the intended boundary).

## Findings

- **F6 (minor, ledgered LOSS-080): the parser cannot recognize the CALLIND op-line.**
    `TypeOpCallind::printRaw` (typeop.cc:800) emits `s << name` with NO trailing space, so a
    real CALLIND renders `callindpc(free)(..)` (keyword glued to the target — see
    `gp.b2.txt`). `TypeOpCall::printRaw` (typeop.cc:676) DOES emit `name << ' '`, so `call
    <target>` is parsed; CALLIND is not. The parser's `call_class` keys on the first
    whitespace token, so CALLIND folds into `OpClass::Value`. The shipped unit test
    `classify("callind r0(free)")` (liftdiff.rs:728) tests a FICTIONAL space-separated form
    the C++ never emits. SAFE direction: the Rust side maps `CPUI_CALLIND -> Callind`, so a
    comparable CALLIND DIVERGES (class mismatch), never spuriously PASSes; no gate PASS
    contains a CALLIND, and the only CALLIND case (`gp`) diverges earlier on op-count. ACCEPT
    with LOSS-080.
       cpp: decompiler/cpp/typeop.cc:793-811 (`TypeOpCallind::printRaw`, no space) vs
            :669-687 (`TypeOpCall::printRaw`, with space)
       rust: rust/crates/kuna-harness/src/liftdiff.rs:312-322 (`call_class` whitespace-token
             match), :728 (the fictional unit test)
       trace: rust/crates/kuna-harness/tests/verify_w5_infra_lift_diff.rs
              ::callind_real_grammar_misclassified_as_value_r2f6 (parser yields Value),
              ::callind_misclass_diverges_never_spurious_pass_r2f6 (safe direction)

- **F7 (minor, note only): bare `as` casts without `// cast:` justification.**
    Five bare integer-widening casts (corpus.rs:572,591,596; liftdiff.rs:568,590), all
    lossless i32->i64/usize widenings. The verifier protocol asks each bare `as` to carry a
    `// cast:` comment or be a finding; these are correctness-clean (no narrowing/no sign
    flip) so it is a style/process note, not a correctness defect.
       rust: rust/crates/kuna-harness/src/corpus.rs:572,591,596;
             rust/crates/kuna-harness/src/liftdiff.rs:568,590

- **F8 (minor, note only): DivergentBy taxonomy conflates flow-time divergence with
  "unported pre-heritage action".** `condexesub` (block-count CFG split) and
  `gh1243-8051-addc` (volatile-access inject inside `generateOps`) are FLOW-time, not
  pre-heritage-action, divergences; the per-case prose is honest about the real cause and
  the outcome is DIVERGENT regardless, so this is documentation imprecision, not optimism.

## Mechanical pass

- `cargo run -p kuna-lift-diff -- --self-test` (declared gate) -> 8/15 PASS, exit 0.
- `cargo test -p kuna-harness` -> 8 unit + 3 lift_diff + 15 verify (incl. 6 new R2) = all pass.
- `cargo clippy -p kuna-harness -p kuna-lift-diff --all-targets -- -D warnings` -> clean.
- grep `todo!`/`unimplemented!`/`HashMap`/`HashSet` -> none in item code (one `panic!` in a
  `#[cfg(test)]` block).
- grep `sort_unstable` -> two, both on `Vec<i64>` `out_indices` (total order, deduped): OK.
- bare `as` casts -> five lossless i32->i64/usize widenings (F7).
- port-audit: n/a (this is an `infra` item with empty `cpp_blob_sha`; no blob-drift check).

## Adversarial tests (committed to worktree-wf_ac003af6-b71-2,
`rust/crates/kuna-harness/tests/verify_w5_infra_lift_diff.rs`, round-2 set)

- `callind_real_grammar_misclassified_as_value_r2f6` — pins F6 against the REAL glued
  `callindpc(..)` rendering (parser yields Value, not Callind).
- `callind_misclass_diverges_never_spurious_pass_r2f6` — proves F6 is the SAFE direction (a
  faithful Rust `Callind` op DIVERGES against the parser's `Value`, never a spurious PASS).
- `call_with_trailing_space_is_recognized_r2` — pins that CALL (space-separated) IS parsed,
  so F6 is CALLIND-specific.
- `entry_head_strip_declines_without_a_sibling_op_r2` — the strip declines when there is no
  sibling op at the entry address (conservative, no over-strip).
- `entry_head_strip_stops_at_first_decode_op_r2` — a buried higher-time entry-addr op defeats
  the leading strip (head op removed only when STRICTLY the latest).
- `out_edges_are_order_insensitive_r2` — out-edge target indices compare as a set.

(Round-1 repair tests retained: gh6990 5-block survival, dup-start survival, diff catches
shared-start differences, F2 entry-block-only strip, implied-goto/fall-through grammar,
non-structureReset rejection.)

losses: LOSS-080 (CALLIND op-line parser-grammar gap, SAFE direction — never a spurious PASS).
