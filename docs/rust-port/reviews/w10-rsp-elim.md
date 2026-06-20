# w10-rsp-elim
verdict: ACCEPT-WITH-LOSSES
verifier: verifier-agent (opus-4.8)
date: 2026-06-16
round: 2
commit: 10fc2ab (rport/w10-rsp-elim, HEAD); branch = 3 commits on top of rust-port 582e067
gate: cargo test --workspace --no-fail-fast -> GREEN (3541 passed, 0 failed)

## What round 2 actually delivers (scope correction)

The review framing (and the round-1 verdict) anticipated the full RSP-elimination
keystone — `ActionExtraPopSetup::apply` inserting the per-call INT_ADD/INDIRECT
spacebase op, plus `propagateSpacebaseRef` + the spacebase-store deadcode. **None of
the op-inserting `apply` body lands in round 2.** The branch net-diff vs rust-port is
THREE files, +140/-10:

- coreaction_protos.rs    — `ActionExtraPopSetup::apply` stays DEFERRED (`return 0`);
                            only the SEAM comment is rewritten to explain the deferral.
- coreaction_stackptr.rs  — `analyze_extra_pop` model guard rewritten to the C++
                            `evalfp_called ?: defaultfp` fallback (a real fix).
- coreaction_stackptr/tests.rs — 3 verifier adversarial tests.

History: commit 141a2f6 LANDED the faithful `apply` body (round-1 reviewed). Commit
10fc2ab (the round-2 "repair") REVERTED that body to deferred. The round-1 work order
said "keep `apply`, fix one stale snapshot string (+2 datatest parity)." The porter
took the OTHER path — withdrawing the feature entirely. **That was the correct call**,
for reasons round-1 missed (see "Round-1 verdict was wrong" below).

## Faithfulness (PASS)

`analyze_extra_pop`'s new guard is a faithful, line-accurate transcription of
coreaction.cc:279-282:
  ProtoModel *myfp = data.getArch()->evalfp_called;
  if (myfp == 0) myfp = data.getArch()->defaultfp;
  if (myfp->getExtraPop() != extrapop_unknown) return;
Rust: `eval_fp_called().or_else(|| default_fp()).map(get_extra_pop)`; `if Some(ep) &&
ep != EXTRAPOP_UNKNOWN -> return 0`. `EXTRAPOP_UNKNOWN = 0x8000` matches fspec.hh:772.
This is a genuine FIX over the rust-port baseline, whose code claimed the
`?: defaultfp` fallback in a comment but only consulted `eval_fp_called()` with
`.unwrap_or(false)` — so a None eval-model wrongly fell through instead of reading
defaultfp. (Safe extra divergence: if BOTH options are None — impossible post-bootstrap,
defaultfp is always set — Rust falls through where C++ would deref-null; defensive, not
behavioral.)

The DEFERRED `apply` body is an honest subtraction, not a fabrication or an
unconditional RSP strip: it returns 0 and inserts nothing, leaving spacebase flow
untouched. The SEAM comment names the exact downstream keystone
(`propagateSpacebaseRef` + spacebase-store ActionDeadCode) it must co-land with, and
the concrete regression it triggers without it (jump-table index recovery).

## NO special-casing (PASS)

grep of the changed production source for register/function/address/offset literals:
the only "RSP" hit is a doc-comment describing the C++ spacebase shape
(`RSP = RSP + *(RSP+k)`); the only address literal (0x1000) is in `#[cfg(test)]`
helpers. The suppression is driven entirely by the model-derived extrapop, never a
literal. CONFIRMED: no heuristic strips RSP unconditionally.

## Mandatory hunt list

- Signedness: clean. `ep != EXTRAPOP_UNKNOWN` is int4 vs int4 (get_extra_pop returns i32).
- Integer widths: the only width-cast in the realized diff is none (the guard adds
  `.or_else`/`.map` combinators, no casts). `extrapop as uintb` lives only in the
  DEFERRED `apply` body + the forward-contract adv test; pinned sign-extending.
- Wrapping: n/a in the realized diff (no arithmetic added).
- Comparator totality: n/a.
- Iteration-order: n/a (the guard has no loop; the deferred loop is `0..num_calls()` over
  the qlst Vec by index == C++ getCallSpecs(i), but it does not run).
- do-while / reverse / lower_bound: n/a.
- Erase-while-iterating: n/a.
- Exception->Result partial-state: the guard's `.map`/Option chain has no panic/`?`; on a
  double-None it falls through (cannot occur in practice). clean.

## Mechanical pass

- grep HashMap/HashSet/todo!/unimplemented!/sort_unstable in changed src: NONE.
- bare `as`: NONE added by the realized diff.
- clippy -p kuna-decomp --lib: CLEAN (0 warnings). (`--tests` surfaces ONE pre-existing
  error at heritage.rs:3232 `!is_heritaged() || true` — byte-identical on rust-port
  baseline, in a file this branch never touches; not a finding here.)
- C++ oracle (only rust/ touched; no decompiler/cpp, specs, docs, kuna, tools, Makefile):
  `kuna.run_tests --all --baseline docs/baseline.json` -> 207/207 unit + 675/675 data,
  PARITY OK. Untouched.

## No regression (the decisive evidence — re-run on each tree, specs present)

Datatest `<stringmatch>` oracle (identical .sla, identical datatests/, both confirmed
untouched by the branch):
  - base rust-port  : 298 passing
  - round-2 HEAD    : 298 passing
  - FULL passing-SET diff: byte-IDENTICAL (0 pass<->fail swap, not just equal counts).
cargo test --workspace --no-fail-fast:
  - base rust-port (specs present): GREEN
  - round-2 HEAD  (specs present): 3541 passed, 0 failed.
B0 universalAction listing (tests/universalaction_listing.rs + the allowlist tests):
PASS, registration order unchanged (ActionExtraPopSetup still registered, body inert).
Golden/B5: boolless / readstruct / condconst_conn tests all PASS; the golden test files
and harness fixtures are UNTOUCHED by the branch (git diff name-only = empty for tests/
and fixtures/).

## Round-1 verdict was wrong (why the revert is correct, not a retreat)

Round 1 REJECTed 141a2f6 but called the landed `apply` "a strict improvement, +2
datatest, 0 regressions, only one stale snapshot." I re-ran 141a2f6 WITH the compiled
`.sla` present and found that is **false**:
  - 141a2f6 + specs: `verify_w10_jts_chain` is 3/7 — **4 structural tests FAIL**
    (switchind label-set/order, data-driven recovery). On baseline rust-port + specs the
    same suite is 7/7. So landing `apply` broke 4 committed structural jump-table tests.
  - The +2 datatest gain was real (Concat split #6, For-loop thru special #2,
    independently reproduced: 300 vs 298, 0 datatest pass removed) — but it came AT THE
    COST of the 4 jts_chain structural regressions, which round 1 did not detect (it
    scored parity on the loose `<stringmatch>` count + a partial cargo run; the jts_chain
    failures need `.sla`, absent in a fresh worktree, so they were silent there).
The round-2 SEAM comment ("REGRESSES jump-table index recovery ... breaking the committed
verify_w10_jts_chain structural tests") is ACCURATE. The revert restores jts_chain 7/7
and a fully green workspace. Deferring the keystone until it can co-land with
`propagateSpacebaseRef` is the right sequencing, not a weakening.

## Findings

- F1 (info, non-blocking): the realized `analyze_extra_pop` guard fix is behaviorally
  INERT on the entire current suite (datatest passing-set byte-identical; 0 cargo delta).
  It is a correct faithfulness improvement but produces no observable parity gain today.
  Not a defect — recorded so the item's "value delivered" is not overstated.
     cpp: decompiler/cpp/coreaction.cc:279-282
     rust: rust/crates/kuna-decomp/src/coreaction_stackptr.rs:647-656
- F2 (info, non-blocking): the item's headline effect ("raw RSP eliminated") is NOT
  realized in round 2 — `ApplyExtraPopSetup::apply` is deferred. This is a documented
  partial (LOSS below), correctly sequenced behind the spacebase keystone.
     cpp: decompiler/cpp/coreaction.cc:1452-1482 (ActionExtraPopSetup::apply)
     rust: rust/crates/kuna-decomp/src/coreaction_protos.rs (apply body returns 0)

## Adversarial tests (land with the verdict, all PASS on round-2 HEAD)

rust/crates/kuna-decomp/src/coreaction_stackptr/tests.rs:
  - adv_w10_rspelim_extrapop_constant_widening_sign_extends
      (forward contract: pins `extrapop as uintb` == C++ (uintb)(int4) for +/- values,
       unmasked newConstant — guards the DEFERRED `apply`'s constant when it re-lands)
  - adv_w10_rspelim_analyze_extrapop_known_defaultfp_suppresses_solve
      (known defaultfp extrapop -> early-return, asserts ZERO ops created — directly
       exercises the new or_else(default_fp) guard; fails under the old unwrap_or(false))
  - adv_w10_rspelim_analyze_extrapop_unknown_defaultfp_does_not_suppress
      (unknown defaultfp falls through to the solver; idempotent/deterministic)

## Disposition

ACCEPT-WITH-LOSSES. Round 2 is a faithful PARTIAL with ZERO regression: the realized
`analyze_extra_pop` guard fix is an accurate C++ transcription (and a real fix over
baseline), there is no special-casing, the full datatest passing-set is byte-identical
to baseline, cargo test --workspace is green (no-fail-fast), the C++ oracle is 675/675
PARITY OK, B0 is unchanged, and the boolless/readstruct/condconst golden tests pass
untouched. The RSP-elimination keystone (`ActionExtraPopSetup::apply` op insertion) is
DEFERRED — correctly, because landing it standalone regresses 4 jump-table structural
tests until `propagateSpacebaseRef` co-lands. Per the verifier protocol, a faithful
partial with 0 regression is ACCEPT-WITH-LOSSES.

losses: LOSS-ExtraPopSetup-deferral (see losses.md, appended by this verdict)

---

# w10-rsp-elim (ROUND 1 — superseded, retained for history)
verdict: REJECT
verifier: verifier-agent (opus-4.8)
date: 2026-06-16
commit: 141a2f6 (rport/w10-rsp-elim)

NOTE (round-2 correction): round 1 called 141a2f6 "a strict improvement, 0 regressions,
only one stale snapshot." Round-2 re-verification WITH the compiled `.sla` shows 141a2f6
actually fails 4/7 `verify_w10_jts_chain` structural tests (baseline is 7/7) — a real
structural regression round 1 missed (it scored on the loose datatest count + a
spec-less cargo run). The original round-1 text is preserved below.

(Original round-1 verdict body retained verbatim from the prior file; its parity claim of
"+2 datatest, 0 regressions" is true ONLY for the loose `<stringmatch>` oracle and is
contradicted by the jts_chain structural suite when specs are present.)
