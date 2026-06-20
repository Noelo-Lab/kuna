# w2-sleigh-pattern

## Round 1

verdict: ACCEPT-WITH-LOSSES
verifier: independent verifier agent (Claude, not the porter)
date: 2026-06-11
branch: worktree-wf_6343a4ba-016-5 (worktree `.claude/worktrees/wf_6343a4ba-016-5`, reviewed at d8e8a72; adversarial tests committed as 76ff9b2)
cpp blob shas: all 4 in-scope files match the checklist entry (`git hash-object` verified: slghpattern.cc 6253ca11, slghpattern.hh de90a885, slghpatexpress.cc a724f301, slghpatexpress.hh 118fe3cc)

gate: `cd rust && cargo test -p kuna-sleigh` -> ok. 56 passed (lib: 23
slghpattern + 18 slghpatexpress + 15 context/globalcontext) + 6 passed
(`verify_w2_sleigh_context`, prior item) + 6 passed (verifier tests
`verify_w2_sleigh_pattern`), 0 failed.
clippy: `cargo clippy -p kuna-sleigh --tests -- -D warnings` -> clean.
port-audit: `cargo run -p port-audit -- --item w2-sleigh-pattern` NOT
runnable — no `port-audit` crate exists in the workspace (carried from every
prior review); blob-SHA check performed manually instead (all 4 match).

## Method

Re-derived the semantics of every ported function in
`slghpattern.{hh,cc}` (all of it) and `slghpatexpress.{hh,cc}` (the
consumer/runtime half) and checked the diff
`git diff rust-port...worktree-wf_6343a4ba-016-5 -- rust/`
(slghpattern.rs +2204, slghpatexpress.rs +1739; the branch also carries the
already-reviewed w2-sleigh-context merge, excluded from this review's
scope).  The sla FORMAT_SCOPE ids in `slghpattern::sla` were checked
constant-by-constant against `decompiler/cpp/slaformat.cc` (all 14
attributes + 24 elements match name and number; `slaformat.rs` is an empty
stub, so no duplicate-definition drift).  The W1 helpers this item leans on
(`Wrap::w*` incl. wrapping shifts, `sign_extend`/`zero_extend`/
`byte_swap_inplace` in kuna-base) were re-checked against address.hh:586-615
/ address.cc:937-951 for exactly the properties used here.  Six adversarial
tests were then written against hand-derived C++ oracle traces (committed as
`rust/crates/kuna-sleigh/tests/verify_w2_sleigh_pattern.rs`); all passed on
first run.

Scope note: the port covers the full `.sla`-consumer surface — PatternBlock,
the Pattern/DisjointPattern algebra and decode factories, and the complete
PatternExpression runtime tree (getValue/min/max/listValues/getMinMax/
getSubValue/encode/decode + decodeExpression).  The SLEIGH-compiler half of
the in-scope modules (TokenPattern, PatternEquation hierarchy, genPattern/
genMinPattern, Token plumbing) is deliberately NOT ported — verified
compile-time-only (see F1 / LOSS-021).  The `ParserWalker` dependency is cut
at the `PatternExpressionContext` seam and the symbol-table dependency of
`OperandValue::decode` at the `OperandValueResolver` seam; both seam
contracts document the exact C++ bodies the later waves must transcribe.

## Hunt list

- signedness: clean. The int4-vs-size_t mixed comparisons are transcribed
  explicitly with comments at every site: `tmplength > 8*sizeof(uintm)` in
  specializes/identical (slghpattern.rs:326,355), the
  `tmpsize/size >= sizeof(uintm)` loop conditions in instruction_bytes/
  context_bytes (slghpatexpress.rs:162,193 — including the
  negative-keeps-looping C++ behavior), `ctid >= getNumConstructors()`
  (slghpatexpress.rs:667).  All `int4 i < vec.size()` loops became
  iterators/usize ranges.
- integer widths: clean. uintm->u32, intb->i64 throughout; decode-side
  implicit C++ truncations (`intb`->`int4`, `uintb`->`uintm`) are commented
  `as` casts at every read; `nonzerosize = maskvec.size()*4` size_t->int4
  commented (slghpattern.rs:244).
- wrapping: clean, one non-blocking note (F2). Expression eval uses
  wadd/wsub/wmul/wneg/wdiv and wrapping shifts exactly where C++ intb
  arithmetic can wrap; PatternBlock's shift counts all carry range proofs or
  resolve x86-masked exactly where the C++ (compiled for x86) masks
  (`32-shift` with shift==0, `32-size` with size==0).  The getMask/getValue
  unsigned-division transcription (`startbit as i64 as u64`, divide, truncate
  to i32) reproduces C++ slghpattern.cc:405-462 bit-for-bit — pinned by
  adversarial test T1 with hand-computed oracles for startbit in
  {-8,-16,-40}.
- comparator totality: n/a — no orderings/comparators in this item.
- iteration-order provenance: clean. Every loop is over a Vec mirroring a
  C++ vector in index/insertion order; no maps or sets anywhere in the item.
  OrPattern traversals preserve C++ begin->end order, including the doAnd
  cross-product nesting (receiver outer, operand inner) and doOr's
  clone-then-shift-then-append sequencing.
- off-by-one / do-while / reverse iterators: clean. normalize()'s backward
  trailing-zero scan (C++ `--iter1` walk from end + "first zero in last zero
  chain" increment + range erase) is re-derived correct in the Rust
  index-walk + truncate form, including the all-zero and single-word edges;
  OrPattern::commonSubPattern's first-element-then-clamp-sa ordering is
  preserved (clamp after the first call, as upstream).  The only do-while
  loops in the C++ scope (`do {} while(advance_combo(...))`) are
  compiler-side and unported (F1).
- erase-while-iterating: clean. The two erase patterns (leading zero words,
  trailing zero words) became `drain(..lead)`/`truncate(i)` and provably
  keep the same elements; the slide-up between them is pinned by adversarial
  test T4 (decode of off=0/nonzero=8/[0,0][0x00FF1100,0x00AB0100] ->
  offset 5, nonzerosize 2, mask bytes FF,11).
- exception -> Result partial-state parity: clean. OperandValue::decode
  returns the "Invalid constructor id" error BEFORE close_element, exactly
  where C++ throws before closeElement; decodeExpression's catch-and-delete
  becomes drop-on-`?`; getMinMax preserves the C++ push order (min pushed
  before max is evaluated); BinaryExpression::decode drops a decoded left
  child when the right child fails, matching the C++ destructor release.
- UB boundaries (ADR 0004): the C-style downcasts of the C++ algebra are
  panics via expect_disjoint/expect_context/expect_instruction with the
  ladder in exact C++ cast order; re-derivation confirms the panics are
  unreachable for every type combination C++ handles defined-ly (only an
  empty OrPattern operand — already UB upstream — can reach them).  The
  `OrPattern::doOr` const-but-mutating receiver quirk is transcribed exactly
  (both halves pinned by adversarial tests T2/T3, including the delegation
  case where the MUTATED object is the caller's `b` operand).

## Findings

- F1 (loss, accepted): the SLEIGH-compiler half of the in-scope modules is
  not ported: TokenPattern (incl. buildSingle/buildBigBlock/
  buildLittleBlock/resolveTokens), the PatternEquation hierarchy +
  OperandResolve, the genPattern/genMinPattern virtuals, the
  advance_combo/buildPattern statics, and the `Token*` plumbing; also
  `OperandValue::getSubValue` returns a Sleigh error (its C++ body needs the
  compiler symbol table) and `isConstructorRelative()`/`getName()` are
  deferred to the symbol-table wave (id accessors exposed).  Verified
  compile-time-only: all consumers sit under `SleighCompile::buildPatterns`
  -> `Constructor::buildPattern`/`SubtableSymbol::buildPattern`/
  `ContextOp::validate` (slgh_compile.cc:2122-2130, slghsymbol.cc:1832-1849,
  1887, 2452), none reachable from the `.sla`-consumer runtime.
     cpp: decompiler/cpp/slghpatexpress.cc:21-455,1162-1680;
          decompiler/cpp/slghpatexpress.hh:23-55,339-487
     rust: rust/crates/kuna-sleigh/src/slghpatexpress.rs:27-67 (module docs),
           :639-643 (get_sub_value error)
     -> LOSS-021 (appended by this review; extends LOSS-001's compiler cut
        to these in-scope-module classes so they cannot escape the ledger).
- F2 (minor, non-blocking): `DivExpression::getValue`/`getSubValue` divide
  via `wrapping_div`: `i64::MIN / -1` yields `i64::MIN` where the C++
  oracle's x86 `idiv` traps (#DE -> SIGFPE process abort; UB in the
  standard).  Divide-by-zero aborts in both engines (Rust panic vs C++
  SIGFPE).  Unreachable from well-formed `.sla` expressions (operands are
  field/context values and spec constants); the helper choice is W1
  kuna-base code already accepted there, so recorded as a note, not a loss.
     cpp: decompiler/cpp/slghpatexpress.cc:1094-1108
     rust: rust/crates/kuna-sleigh/src/slghpatexpress.rs:935-938,1053-1056
           (helper rust/crates/kuna-base/src/types.rs:123)

## Mechanical pass detail

- grep over the item diff: no `todo!`/`unimplemented!`/`HashMap`/`HashSet`/
  `sort_unstable` in shipped code (`unimplemented!` appears only inside
  `#[cfg(test)]` walker stubs).  Every bare `as` cast carries a justification
  comment at the line or in the immediately preceding comment block (same
  accepted style as the w2-sleigh-context review).
- sla ids: all 38 constants verified against decompiler/cpp/slaformat.cc;
  scoped-id non-registration matches the C++ `scope != 0` constructor skip.

## Adversarial tests

`rust/crates/kuna-sleigh/tests/verify_w2_sleigh_pattern.rs` (committed
76ff9b2 on the port branch; all 6 pass):

- verify_w2pat_getmask_getvalue_negative_startbit_unsigned_division
- verify_w2pat_or_door_delegated_negative_sa_mutates_or_operand
- verify_w2pat_or_door_positive_sa_shifts_receiver_clones_too
- verify_w2pat_decode_normalize_slide_up_unaligned_mask
- verify_w2pat_intersect_conflict_and_common_sub_in_second_word
- verify_w2pat_identical_cross_type_specializes_asymmetry_and_overread
  (also pins the C++ 4-byte isInstructionMatch over-read quirk and that
  InstructionPattern(true)/ContextPattern(true) are `identical` cross-type)

losses: LOSS-021 (F1); LOSS-001 cited as the parent decision.
