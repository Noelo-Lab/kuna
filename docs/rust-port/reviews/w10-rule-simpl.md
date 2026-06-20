# w10-rule-simpl
verdict: ACCEPT-WITH-LOSSES
verifier: claude-opus-4-8 (independent verifier, round 1)
date: 2026-06-15
branch: rport/w10-rule-simpl (HEAD 19ba68b, off rust-port d052da4)
scope: the rust-only diff `git diff rust-port...rport/w10-rule-simpl -- rust/`
  — ports `Funcdata::combineInputVarnodes` (funcdata_varnode.cc:383-456) and
  un-seams its caller `RuleDoubleOut::applyOp` (double.cc:3332-3355). This is the
  W4 restoration of the `combine_input_varnodes_seam` half of LOSS-084.

gate / mechanical:
  - cargo build --workspace (debug)  -> OK
  - cargo test --workspace           -> 3405 passed / 0 failed (3401 base + 4 new verifier tests)
  - cargo clippy --lib -p kuna-decomp -> clean (0 warnings)
  - run_tests --all --engine rust --baseline docs/baseline.json
        -> datatests 187/668 (PARITY FAIL is the PRE-EXISTING W10 grind state;
           base rust-port d052da4 is ALSO 187/668 — NO regression, count HELD at the >=187 floor)
  - C++ oracle (decomp_test_dbg, run_tests --all): 207/207 unit + 675/675 datatests, PARITY OK
        -> BYTE-UNTOUCHED (diff is rust-only: `git diff --name-only` shows no decompiler/ or specs/ file)
  - byte-parity / parity-fence tests (print_b5_boolless.rs, verify_w8_s9_printc.rs,
    verify_*_byte_parity, verify_close_element_skipping_*): all green (part of the 3405)

keystone-reserved files: NONE touched. Diff edits only
  rust/crates/kuna-decomp/src/double.rs and
  rust/crates/kuna-decomp/src/funcdata_varnode.rs
  (neither is in the reserved set: merge.rs, funcdata.rs, funcdata_merge.rs,
   funcdata_union.rs, varnode.rs, variable.rs, coreaction_cleanup.rs, cast.rs,
   typeop.rs, dtype.rs, unionresolve.rs, printc.rs). No collision with the
  concurrent S6-Merge keystone.

special-casing: NONE. grep of the added lines for hex addresses / function-name /
  magic-value / string-content gating found only: (a) the contiguity equality
  `otheraddr == lo_addr` / `== hi_addr` (the algorithm itself), and (b) error-message
  string literals that match the C++ `LowlevelError` text verbatim
  ("Varnodes being combined are not inputs", "Input varnodes being combined are not contiguous").
  No behavior is keyed on a name/address/value.

hunt list:
  - Signedness: clean. C++ `for(int4 i; i<pieceList.size(); ++i)` (the -Wno-sign-compare
      site) becomes `for &op in &piece_list` (iterator) — order preserved, no signed/unsigned
      compare. `vnHi->getSize()` is `int4`>=0; widened to i64/uintb (see Integer widths).
  - Integer widths: clean. `hi_size as i64` / `lo_size as i64` (funcdata_varnode.rs:980,984)
      faithfully reproduce the C++ implicit `int4 -> int8` promotion in `addr + vnHi->getSize()`;
      `lo_size as uintb` (:1041) reproduces `newConstant(4, vnLo->getSize())` (int4 -> uintb).
      `let out_size = hi_size + lo_size` uses the sizes snapshotted BEFORE destroy, matching
      C++ `int4 outSize = vnHi->getSize()+vnLo->getSize()` computed before `vbank.destroy`.
      (Minor: F1 — the three `as` casts lack the `// cast:` convention comment.)
  - Wrapping: clean. The only arithmetic is `&addr + size` via `Address::operator+`
      (address.rs:545-551), which is `wrap_offset(offset.wadd(off))` — a faithful match for
      C++ `Address(base, base->wrapOffset(offset+off))`. Verified non-panic at high offset
      (adversarial test `..._high_offset_no_panic_noncontiguous`).
  - Comparator totality: n/a — no comparator ported (only `Address::operator==`, derived
      space-ptr + offset equality, total).
  - Iteration-order provenance: clean. Both reader loops use `descend_snapshot(vn)`
      (funcdata_varnode.rs:888-890), which collects `Varnode::descend_iter()` = the C++
      `list<PcodeOp*>` insertion order, exactly. No HashMap/HashSet anywhere in the body.
  - Off-by-one / do-while / reverse iters: clean. No do-while, no reverse iter; the two
      `for` reader loops and the `piece_list` re-loop are straight forward iteration.
  - Erase-while-iterating: clean. The detach (op_remove_input slot 1 then op_unset_input
      slot 0) and the later COPY re-point happen on a snapshotted `piece_list` collected
      before any mutation; `total_replace` itself snapshots before rewiring. Order matches C++.
  - Exception -> Result partial-state parity: clean (analyzed). The two C++ throw sites
      (not-inputs :386, not-contiguous :399) are both pre-empted by the sole caller
      `RuleDoubleOut::apply_op` (it checks `vn_is_input` for both and `is_addr_tied_contiguous`
      BEFORE the call), so the `Err(_) => 0` decline at double.rs:4954-4957 is unreachable in
      practice — mirroring the C++ where the throw never fires. The interior `?` points
      (op_set_input, new_varnode_out, set_input_varnode, vbank.destroy) map non-throwing C++
      calls: `destroy` (varnode.rs:1601) throws only on a def/descendant, and by the destroy
      point all PIECE readers are detached and all other readers `total_replace`d, so both
      pieces have no descendants — destroy returns Ok, matching C++. No partial-state divergence.

faithfulness: the body is a line-for-line transcription of funcdata_varnode.cc:383-456 —
  the input/contiguity guards, the endian-dependent base-address selection (`addr=vnHi` BE /
  `addr=vnLo` LE, endianness queried on vnLo's addr in both), the two reader-partition loops
  (`pieceList` vs `otherOpsHi/Lo` with the exact `code==PIECE && in0==vnHi && in1==vnLo`
  predicate and its De-Morgan dual for the Lo loop), the detach (opRemoveInput 1 / opUnsetInput 0),
  the SUBPIECE replacements (truncation const = vnLo->getSize() for hi, 0 for lo) with
  total_replace, the destroy + setInputVarnode, and the PIECE->COPY re-point — all match.
  The wiring in `RuleDoubleOut::apply_op` (double.rs:4927-4958) faithfully follows
  double.cc:3332-3355 (input/persist/precis gates -> attemptMarking or
  hasUnreachableBlocks/isAddrTiedContiguous -> combineInputVarnodes; return 1).

findings:
  - F1 (low/cosmetic): the three widening `as` casts (`hi_size as i64`, `lo_size as i64`,
        `lo_size as uintb`) lack the `// cast:` justification comment the protocol requests.
        They are value-preserving (getSize() is non-negative int4) and faithful to the C++
        implicit promotions — correctness-neutral. Not blocking.
       cpp: decompiler/cpp/funcdata_varnode.cc:392, 396, 429
       rust: rust/crates/kuna-decomp/src/funcdata_varnode.rs:980, 984, 1041
  - F2 (low): `bblocks_get_block(0)` + `bblocks_block_start` are evaluated UNCONDITIONALLY
        (hoisted above the `otherOpsHi`/`otherOpsLo` arms), whereas C++ fetches the entry
        block only inside each `if (otherOps*)` arm. Behaviorally inert at the real call site
        (CFG always non-empty at S6+; the call is a side-effect-free read), but would panic
        on an empty bblocks graph in the no-other-readers case where C++ would not. Accepted
        as LOSS-176.
       cpp: decompiler/cpp/funcdata_varnode.cc:425-435
       rust: rust/crates/kuna-decomp/src/funcdata_varnode.rs:1034-1035

dormancy (not a new loss): `combine_input_varnodes` is reached only after the
  `vn_is_persist` gate in RuleDoubleOut, which is itself a W4 seam
  (`set_varnode_properties` does not yet apply the symbol-table persist flag). So the
  merge is currently UNREACHED in every datatest — datatest count held at 187 (base==branch).
  This is the existing LOSS-084 situation, unchanged. The `attempt_marking`
  `getSymbolEntry`-mismatch omission (double.rs:4892-4896) is pre-existing (NOT touched by
  this diff) and remains covered by LOSS-084.

adversarial tests (committed to the porter branch, 4 new, all green):
  rust/crates/kuna-decomp/src/funcdata_varnode/tests/verify_w10_rule_simpl.rs
  - verify_w10_rule_simpl_not_inputs_errs_and_leaves_state
       (not-inputs guard -> Err; both varnodes survive, neither becomes an input)
  - verify_w10_rule_simpl_noncontiguous_errs_le
       (LE gap -> contiguity Err; inputs untouched, no combined input registered)
  - verify_w10_rule_simpl_reversed_orientation_not_contiguous_le
       (pins the directionality of the `+ size` step: low-addr-as-hi is non-contiguous)
  - verify_w10_rule_simpl_high_offset_no_panic_noncontiguous
       (top-of-space offset: `as i64` cast + Address::operator+ wrap, no panic -> Err)
  (These cover the guard paths, which the only caller pre-empts; they exercise the fragile
   endian/contiguity arithmetic + the Err partial-state parity. The full merge body needs a
   constructed S6 entry CFG and is exercised by the datatest oracle once the W4 persist seam
   lands — and is the locus of F2/LOSS-176.)

losses: LOSS-176 (this diff's `bblocks` hoist, F2); pre-existing LOSS-084 (the persist-gate
  dormancy + the `attempt_marking` getSymbolEntry omission, both unchanged by this diff —
  this diff RESTORES the `combine_input_varnodes_seam` half of LOSS-084 faithfully).
