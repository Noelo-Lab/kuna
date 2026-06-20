# w5-kuna-rule-pack
verdict: ACCEPT-WITH-LOSSES
verifier: independent verifier (Claude Opus 4.8)
date: 2026-06-11
gate: `cd rust && cargo test -p kuna-decomp` -> PASS (exit 0; 641 tests, 0 failed; the
      7 in-file rule tests + the 7 new `verify_w5_kuna_rule_pack` adversarial tests green)

scope: the 7 kuna simplification rules —
  - `kuna_addcarrychain.{cc,hh}`  RuleAddCarryChain   (GH-8913)
  - `kuna_booleanmask.{cc,hh}`    RuleBoolSignShift   (GH-1282)
  - `kuna_ovlesssimplify.{cc,hh}` RuleOvLessSimplify  (GH-7190)
  - `kuna_flagcompare.{cc,hh}`    RuleBoolSignLess + RuleSborrowGe (GH-1276 / GH-8777)
  - `kuna_stackalias.{cc,hh}`     OptionStackAlias    (GH-8500, toggle only)
  - `kuna_arraystride.{cc,hh}`    RuleArrayStride     (GH-8724)
  - `kuna_condexeplace.{cc,hh}`   OptionCondExePlace  (GH-9203, toggle only)

## Hunt list

- Signedness: CLEAN. The `int4` loop indices (`zero_idx`/`add_idx` in
  `kuna_is_induction_phi`, the `for i in 0..2` carry/limb scans) stay `int4` and
  compare `< 0` exactly as C++; `width: int4` is cast to `uintb` only where C++
  writes `(uintb)width`. No `int4`-vs-size comparison was silently widened.
- Integer widths: CLEAN. `int4`/`uintb`/`intb` mappings preserved. `sign_extend((intb)k1,
  sz*8-1)` is `k1 as intb` (bit reinterpret, matching C++ `(intb)k1`); `(uintb)(-ksigned)`
  is `ksigned.wrapping_neg() as uintb`. The `width*8` products stay in `uintb`.
- Wrapping: CLEAN. `wrapping_mul`/`wrapping_sub`/`wrapping_neg`/`wrapping_add` used at
  every `uintb`-lineage op (booleanmask `tophalf`, flagcompare `above`/`negk`, ovless
  `negk`, addcarrychain `negc`). The `1u64 << (width*8 - sa)` shift cannot overflow:
  the preceding `width*8 <= sa` early-out forces the count into `[1, 63]` (booleanmask,
  flagcompare-BoolSignLess). `signbitpos = width*8-1-(sa as int4)` is in `[0,63]` for the
  same reason, so `1u64 << signbitpos` is safe.
- Comparator totality: N/A — no `operator<`/sort/BTree in scope.
- Iteration-order provenance: CLEAN. The only container walk is RuleArrayStride's
  `bb_ops(bl)` sibling-phi scan, which reproduces `bl->beginOp()..endOp()` head..tail
  list order via the intrusive op-link chain (funcdata.rs:768), preserving the
  `code()!=MULTIEQUAL -> break` early-out. No `HashMap`/`HashSet` anywhere in scope.
- Off-by-one / do-while / reverse iterators: CLEAN. No do-while/`rbegin`; the
  `for i in 0..2` scans match the C++ `for(i=0;i<2;++i)`; the `lone_descend`/`descend`
  reads are exact.
- Erase-while-iterating: CLEAN. RuleArrayStride's `totalReplace`+`opDestroy(op)`+
  `opDestroy(accadd)` is ordered exactly as C++ (snapshot-then-rewire via the W3
  `total_replace`, which faithfully mirrors `op=*iter++`). `accout` is captured before
  the replace; `multout` reads `cntout` (untouched by the replace). Verified by
  re-derivation + `rewrites_accumulator_as_counter_times_stride`.
- Exception -> Result partial-state parity: FINDING F1 (see below). Every emit
  `op_set_input(...).expect(...)` / `new_unique_out(...).expect(...)` converts a W3 `Err`
  to a PANIC where C++ `opSetInput`->`addDescend` throws a *recoverable* `LowlevelError`.
- AND (rule-specific) transposed conditions / swapped slots / inverted returns /
  early-outs: CLEAN. Re-derived rule-by-rule. addcarrychain checks `hi1` before `hi0`
  (matches C++ :100-101); the commuted INT_CARRY `c0==b && c1==a` branch is present;
  arraystride's `ci != initIdx || cb != backIdx` edge-alignment and `cstep != 1` /
  `stride == 1` guards are faithful (swapped-edge + commuted-add cases pinned by new
  tests); flagcompare's AND-of-ORs vs OR-of-ANDs XNOR shape checks (`pA!=gB`,
  `(p0==n2&&p1==v2)||(p0==v2&&p1==n2)`) match C++; ovless's `mone == calc_mask`
  (-1) guard and `k1!=k2`/size checks are faithful.

## Mechanical pass

- `cargo run -p port-audit -- --item w5-kuna-rule-pack`: TOOL ABSENT — `port-audit` is
  not a crate in the worktree (a future protocol tool); could not run.  Not a blocker.
- grep `todo!`/`unimplemented!`/`HashMap`/`HashSet`/`sort_unstable`: NONE in the 7 files.
- bare `as` casts: 6 total (booleanmask:154/158, ovless:192/194, flagcompare:121/125),
  each reproducing an explicit C++ cast shown in the adjacent transcription comment
  (`(uintb)width`, `(intb)k1`, `(uintb)(-ksigned)`, `(int4)sa`).  They lack the literal
  `// cast:` prefix the protocol's mechanical bullet suggests (informational nit, not a
  finding — each is provably the C++ cast and bounded).
- `cargo clippy -p kuna-decomp -- -D warnings`: CLEAN (library).  The crate's `--tests`
  clippy fails only in the UNRELATED `verify_w4_fw_options` test (doc_lazy_continuation);
  the 7 rule files and the new `verify_w5_kuna_rule_pack.rs` are clippy-clean.

## Findings

  - F1 (minor / loss): emit-path exception->panic divergence + W3/W6 op-creation seam.
       The const-folded carry form of RuleAddCarryChain (`kunaIsCarryOf` INT_LESSEQUAL
       branch) re-wires an original low-add operand into a new op while it still feeds
       the low INT_ADD; in that form one operand is a constant (`isFree()`), so the
       re-wire makes a free varnode gain a 2nd descendant.  C++ THROWS a recoverable
       `LowlevelError` (function decompile aborts/restarts, leaving partial baseop/zextop
       state — the LOSS-055 class); the port PANICS via `.expect()`.  Output-faithful
       (both fail to fuse — this is a latent GH-8913 upstream bug), the only delta is
       panic vs. catchable exception.  Also: every `op_set_opcode(op, TypeOp::new(opc,0,
       ".."))` carries a ZERO W6 property-flag word (the `glb->inst[opc]` table is
       unported).  Both are the documented LOSS-035/042/055/067 seam family.
       cpp: decompiler/cpp/kuna_addcarrychain.cc:116-134 (+ varnode.cc Varnode::addDescend)
       rust: rust/crates/kuna-decomp/src/kuna_addcarrychain.rs:185-218 (new_unique_out :335-346)
       -> ledgered as LOSS-073 (appended by the verifier to docs/rust-port/losses.md).

  - F2 (informational, not a blocker): the 7 module docs repeatedly claim "Noted in the
       structured losses," but BEFORE this review there was NO losses.md entry for
       w5-kuna-rule-pack.  Resolved by the verifier appending LOSS-073 (the protocol
       permits the verifier to append).  No code change required.

No blockers and no majors stand.  The rule decision logic is a faithful, complete
transcription of all 7 C++ rules (every guard, both XNOR shapes, both carry forms, the
commuted-operand and swapped-edge paths); the only departures are the cross-wave
op-creation seams ledgered as LOSS-073, which produce no wrong output relative to C++ at
this boundary (the seamed const-folded carry path is itself a no-fuse latent bug in the
C++).

## Adversarial tests

Added at rust/crates/kuna-decomp/tests/verify_w5_kuna_rule_pack.rs (committed to the
branch, 7 tests, all green):
- w5_arraystride_swapped_sibling_edges_decline  (swapped init/back slot on the counter)
- w5_arraystride_commuted_back_edge_add_folds    (INT_ADD(step, self) order)
- w5_addcarrychain_commuted_int_carry_operands_fold  (INT_CARRY(b,a))
- w5_addcarrychain_carryelim_lessequal ... wrong_negc_declines  (cst != (-c)&mask guard)
- w5_addcarrychain_carryelim_const_baselo_panics_where_cpp_throws_lowlevelerror  (F1)
- w5_ovlesssimplify_mone_not_minus_one_declines  (mone != -1 SLESS-hi guard)
- w5_booleanmask_byte_aligned_shift_declines      ((sa & 7) == 0 defer-to-RuleLeftRight)

## losses

LOSS-073 (appended by the verifier): the 7 kuna rules' op-graph emit is W3/W6-seamed
(`op_set_output`/`glb->inst[opc]`) and the emit errors panic via `.expect()` where C++
throws a recoverable `LowlevelError`; the const-folded RuleAddCarryChain carry path
deterministically reaches it.
