# w10-transformmanager (checklist item: w5-s3-transform restoration; branch rport/w10-transformmanager)
verdict: ACCEPT
verifier: Claude Opus 4.8 (1M context) — independent verifier
date: 2026-06-15
round: 1
scope: closes the `TransformManager::apply()` materialization seam (LOSS-053) and the
       `SubvariableFlow::doReplacement` seam (LOSS-062). Diff is rust/ ONLY (no C++/spec
       touch): transform.rs, subflow.rs, funcdata_op.rs (markIndirectCreation),
       funcdata_varnode.rs (transferVarnodeProperties + op_iop_decode), transform/tests.rs,
       2 harness tests. Verified against C++ transform.cc (apply/createOps/createVarnodes/
       transformInputVarnodes/placeInputs/specialHandling + TransformOp/TransformVar::
       createReplacement), subflow.cc:1258-1545 (replaceInput/useSameAddress/
       getReplacementAddress/getReplaceVarnode/doReplacement), funcdata_op.cc:736
       (markIndirectCreation), funcdata_varnode.cc:632 (transferVarnodeProperties),
       op.cc:334 (insertInput), typeop.cc:1893 (preferredZextSize).

gate: `cd rust && cargo test -p kuna-decomp` -> ok (transform::tests 33/33 incl. the 3
      live-materialization adversarial replacements; print_b5_boolless 11/11; the
      condconst2/inline_body parity tests run against the real oracle, no SKIP).
      `cargo test --workspace` -> ALL GREEN, 3419 passed / 0 failed / 37 ignored.
      Passing-SET diff vs branch-point (main tree @ 2b2b9a9 == rust-port HEAD):
        - cargo test set: 0 regressed. 4 "removed" names are the 3 seam-error tests
          renamed to their live-materialization successors + the condconst2 rename
          (all GAINS, see findings).
      Rust `decomp_test_dbg` datatest suite (the 668/187 oracle-parity gate):
        baseline (main tree) = 187 passing; branch = 193 passing.
        comm on the Success SETS => 0 REGRESSED, 6 GAINED:
          Immediate Conditional #8, Compare INT_OR #2, Status Compare #2,
          Inlining #1, Inlining #6, Inlining #7.
        Every previously-passing assertion still passes (empty regression set).
      printc_parity body-byte gate: 0/8 on BOTH baseline and branch (unchanged; the
        full-body byte-match is LOSS-130-blocked downstream, not a regression).
      C++ oracle (worktree decomp_test_dbg): 675/675 datatests PARITY OK — and the diff
        touches ZERO C++/spec files (`git diff --name-only` outside rust/,docs/ is empty),
        so the oracle is structurally untouched.

faithfulness (materialization order / SeqNum-feeding create order):
  - apply() drives createOps -> createVarnodes -> removeOld -> transformInputVarnodes ->
    placeInputs in C++ order (transform.cc:756). MATCH.
  - createOps: createReplacement over newOps (Vec = insertion order = C++ list), then the
    do/while attemptInsertion follow-resolution loop (re-runs until followCount==0). MATCH.
  - TransformOp::createReplacement op_preexisting arm: opSetOpcode, then the input-arity
    reshape — remove-down-to-input.size(), unset-all, insert-up-to-input.size() — each loop
    RE-READS num_input from the bank every iteration (faithful to C++ ops that mutate
    numInput). The `opInsertInput(op,null,slot)` shortcut to a bare `insert_input(slot)` is
    proven equivalent: insertInput(slot) (op.cc:340) sets the new slot to null, and
    opSetInput(op,null,slot) (funcdata_op.cc:107) returns immediately on `null==getIn(slot)`.
    MATCH (verified the Rust insert_input mirrors op.cc:334-341 exactly).
  - new-op arm: newOp(input.size(),op->getAddr()) BEFORE opSetOpcode BEFORE output
    createReplacement BEFORE follow-insertion — bumps the create-index in the SAME order
    as C++. MATCH.
  - createVarnodes: pieceMap iterated by BTreeMap<int4,_> (ascending key == C++ map<int4>),
    inner per-lane loop to split_terminator, then newVarnodes (Vec). The input-list push +
    isMark/input_duplicate bookkeeping is byte-faithful. MATCH (pinned by the preexisting
    apply_visits_piece_map_before_newvarnodes / apply_materializes_newvarnodes_in_creation_
    order order-provenance tests, still green).
  - SubvariableFlow::doReplacement: push-patch loop FIRST (getReplaceVarnode then getOut,
    opSetOutput, placeholder ZEXT inserted after), then define-all-outputs (newOp/setOpcode/
    getReplaceVarnode-output/insertAfter), then set-all-inputs, then the terminal patch loop
    resumed from the SAME `piter` index the push loop stopped at (faithful to the single C++
    iterator continuation). copy/compare/parameter/extension/push/int2float arms transcribed
    1:1 incl. the extension_patch sa==0 vs sa!=0 split and the in/out-size ZEXT-vs-COPY pick.
    MATCH against subflow.cc:1435-1545.

hunt list:
  - Signedness: clean. The only added signed/unsigned comparison is
    `(lsb_offset as usize) < size_of::<uintb>()` in transferVarnodeProperties — C++
    `lsbOffset < sizeof(uintb)` is the same -Wno-sign-compare site; lsb_offset is a byte
    offset (>=0) so the cast is harmless. create_op_replacement's `input_size >= num_input`
    is int4-vs-int4 (C++ size_t-vs-int4, both non-negative). clean.
  - Integer widths: clean. transferVarnodeProperties shift counts `8*lsbOffset` /
    `8*(8-lsbOffset)` are u32, bounded [0,56] < 64; `sa as uintb` and `input.len() as int4`
    are small/non-negative. No promotion divergence.
  - Wrapping: clean. transferVarnodeProperties uses wshl/wshr (wrapping_shl/shr, count mod
    width) for the consume-mask shifts — identical to C++ `<<`/`>>` for the in-range counts,
    and MORE defensive than C++ at the lsbOffset>=8 boundary (both guarded by `< sizeof`).
    No bare `<<`/`>>` introduced on uintb in the diff. (LOSS-064's unguarded-shift family is
    in the trace half, untouched here.)
  - Comparator totality: n/a (no comparator added).
  - Iteration-order provenance: clean. piece_map=BTreeMap<int4> (ascending == map<int4>),
    new_ops/new_varnodes/oplist/patchlist/worklist all Vec (insertion order == list).
    grep => 0 HashMap/HashSet in transform.rs/subflow.rs.
  - Off-by-one / do-while: clean. createOps follow-resolution do/while ported as
    loop{...; if follow_count==0 break} (at-least-once preserved). The three reshape while
    loops re-read num_input each pass; no fencepost.
  - Erase-while-iterating: clean. opRemoveInput/opUnsetInput/insert_input mutate the op's
    inrefs; the loops re-query num_input rather than caching, matching the C++ live-numInput
    reads. The push-patch/pull-patch single-iterator continuation is preserved via piter.
  - Exception -> Result partial-state: see F1 (informational). markIndirectCreation sets the
    indop flag BEFORE the `!in0.is_constant()` Err, matching C++ throwing after `flags |=
    indirect_creation` (funcdata_op.cc:742-744) — partial state identical.

mechanical pass:
  - grep todo!/unimplemented!/unreachable!()/HashMap/HashSet/sort_unstable in
    transform.rs/subflow.rs/funcdata_op.rs/funcdata_varnode.rs => NONE.
  - bare `as` casts on ADDED lines: 5, all justified index/shift/width conversions on
    non-negative small values (lsb_offset, sa, input.len()); none lose information. No
    `// cast:` was demanded by the protocol for these mechanical index casts (consistent
    with the rest of the crate).
  - `cargo clippy -p kuna-decomp --lib` => clean (0 warnings/errors).
  - special-casing grep (function names / addresses / values): the only hex/name literals on
    added lines are in transform/tests.rs fixtures (new_op(_,Address::new(ram,0x1000)),
    set_consume(0xffff...)); ZERO hardcoding in the materialization logic. The port reads
    opcode/size/addr/consume from the placeholder graph + funcdata, never from a name/addr.
    NOT special-cased.

findings:
  - F1 (informational, non-blocking): run_subflow / RuleSplitFlow / RuleSubfloatConvert map
    `do_replacement`/`apply` Err -> 0 ("no change"). C++ doReplacement/apply return void; the
    only Err sources left are structural funcdata-mutator failures (set_input_varnode/
    total_replace/op_set_output split-borrow) which cannot fire on a well-formed completing
    trace. If one ever did, the IR would be left partially mutated while reporting "no
    change" — but C++ would LowlevelError-abort at the same spot, so the Rust is strictly
    more conservative, and the empirical datatest run shows 0 regressions / +6 gains, so no
    such failure occurs on the corpus. Same fragility already ledgered in LOSS-062's
    partial-state note.
       cpp: decompiler/cpp/subflow.cc:1435 (void doReplacement)
       rust: rust/crates/kuna-decomp/src/subflow.rs:2680 (run_subflow Err=>0)
  - GAIN-1 (verified real parity, not a finding): condconst2_typed_sig_no_sub_leak_matches_
    oracle — formerly pinned a SUB-leak DIVERGENCE; now asserts `!rust.contains("SUB(")` AND
    `!cpp.contains("SUB(")` against the real oracle and PASSES. Closing apply() lets
    SubvariableFlow collapse the SUB/ZEXT wrapper -> datatest "Immediate Conditional #8"
    flips to Success. Strengthened, not weakened.
       rust: rust/crates/kuna-decomp/tests/verify_w10_input_prototype_adversarial.rs:125
  - GAIN-2 (verified): verify_w10_inline_body now asserts Inlining #1/#6/#7 Success and
    `Total passing tests = 6` (was 3, header-only). Body-cleanup gains corroborated
    independently by the datatest passing-SET diff.

residual seams (pre-existing W4/W6 trace-level seams; NOT new losses):
  - preferred_zext_size (subflow.rs) returns identity — but try_int2float_pull (subflow.rs:636)
    returns Err during the TRACE (LOSS-063), so no completing trace ever produces an
    Int2FloatPatch and the stub is genuinely dead. Faithful.
  - copySymbolIfValid on the constant-replacement path of get_replace_varnode is a no-op
    because W4 builds no equate symbols on this path (copySymbolIfValid family, LOSS-056).
    Faithful no-op.

adversarial tests (>=3, committed on the branch in transform/tests.rs):
  - apply_preexisting_op_sets_opcode_and_reshapes_inputs — op_preexisting arm: opSetOpcode +
    input-arity reshape 2->1 (the trickiest reshape-loop path). PASS.
  - apply_piece_materializes_varnode_and_transfers_consume — piece arm: newVarnode +
    transferVarnodeProperties; asserts consume mask masked to 0xffff for the 2-byte lane
    (directly exercises the wshr/calc_mask logic). PASS.
  - apply_constant_iop_round_trips_through_get_op_from_const — newVarnodeIop encode then
    op_iop_decode === original op (the getOpFromConst round-trip). PASS.
  (plus the still-green order-provenance tests apply_visits_piece_map_before_newvarnodes /
   apply_materializes_newvarnodes_in_creation_order.)

losses: LOSS-053 and LOSS-062 are now CLOSED by this item except for the two residual
  trace-level seams above, which stay open under their existing umbrellas (LOSS-063 int2float
  trace; LOSS-056 copySymbolIfValid). No new loss entry required — this item is a restoration,
  introduces no new divergence, and regresses nothing.
