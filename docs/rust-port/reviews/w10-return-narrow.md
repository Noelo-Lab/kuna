# w10-return-narrow

verdict: ACCEPT
verifier: claude-opus-4-8 (independent)
date: 2026-06-15
branch: rport/w10-return-narrow (round 1)
gate: cd rust && cargo test -p kuna-decomp -> 2582 passed / 0 failed;
      cargo test --workspace -> 3428 passed / 0 failed;
      cargo run -p kuna-lift-diff -- --self-test -> PASS 8/15 (the 7 DIVERGENT are
        pre-existing ActionFuncLink/ExtraPop/condexe seams, untouched by this wave);
      C++ oracle (python -m kuna.run_tests --all --baseline docs/baseline.json)
        -> 207/207 unit + 675/675 datatest assertions, PARITY OK;
      cargo clippy -p kuna-decomp --lib -> clean (0 warnings).

## Scope note (prompt-vs-actual)

The task framing named coreaction.cc/fspec.cc/typeop.cc as the C++ in scope, but
the actual diff closes the **`SubvariableFlow::tryReturnPull`** seam in
`subflow.cc:238-284` (the SubvariableFlow return-trim). This is the genuine
"return narrowing" surface for this branch and the C++ oracle for it
(`condconst.xml`) is exactly the right B5. The named files are the *adjacent*
return-type machinery; this branch does not touch them (and must not — they are
not in the diff). Reviewed against the real C++ in the diff: subflow.cc:238-284.

## What the diff does

Closes the `try_return_pull` portion of **LOSS-063** per that loss's own
restoration criterion ("when W4 lands `FuncProto::isOutputLocked`, replace the
`Err` tails of ... `try_return_pull` with the residual locked checks and the
cross-RETURN `beginOp(CPUI_RETURN)` propagation loop"). The base branch had
`try_return_pull` as a `SEAM(W4)` stub returning `Err`, which—propagated by `?`
through `do_trace`—aborted the whole sub-variable trace whenever a logical value
reached a RETURN, leaving a stray `ZEXT` in the output. The diff:

1. wires the real `data.get_func_proto().is_output_locked()` (now `fspec::FuncProto`,
   funcdata.rs:528/77 -> fspec.rs:4964, the store-output type-lock — NOT the
   seams.rs:418 placeholder),
2. transcribes the non-aggressive consume-outside-mask bail,
3. transcribes the `!returns_traversed` cross-RETURN propagation loop (single
   return-value type for the function),
4. records the terminal parameter patch and bumps `pullcount`.

Line-for-line faithful to subflow.cc:238-284 (slot==0 bail -> output-locked bail
-> non-aggressive consume gate -> once-only RETURN propagation -> terminal patch).
Both call sites (subflow.rs:1088, :1560) keep the C++ `if (!tryReturnPull(...))
return false;` shape via `if !..? { return Ok(false); }`, matching subflow.cc:625
and :938.

## Real-parity evidence (new PASSES are genuine, not fabricated)

- `condconst.xml` (the B5 oracle) pins the sibling int4-return `condconst_conn`
  to `v1 = x;` and `v1 = 0x14;` — i.e. the **un-ZEXT'd, trimmed** form. C++ oracle
  re-run: 675/675 PARITY OK.
- The branch's Rust render of `condconst_copy` (captured via --nocapture) is now:
  `v1 = a0;` / `return v1;` — the trimmed form, matching the oracle direction.
  Before this wave the seam aborted the trace and the engine left `v1 = ZEXT(a0)`.
- The test file change flips the assertion from `v1 = ZEXT(a0)` to `v1 = a0` and
  *strengthens* the anti-over-inline guard (now rejects BOTH `return a0` and
  `return ZEXT(a0)`; previously only the latter). The 4 tests in the file are the
  same 4 on base — none deleted or disabled.

## hunt list

- Signedness: clean. `slot==0` (int4), `halt != 0` (uint4), `(consume & !mask) != 0`
  on uintb. No signed/unsigned mismatch vs C++.
- Integer widths: clean. mask=uintb (C++ uintb), slot=int4 (C++ int4),
  get_consume()=uintb, get_halt_type()=uint4, pullcount=int4 (C++ int4) — all match.
- Wrapping: clean. No arithmetic — only bitwise `&`/`!` (Rust `!` == C++ `~`) and
  `pullcount += 1` mirroring C++ `pullcount += 1`.
- Comparator totality: N/A (no comparators added).
- Iteration-order: clean. C++ `fd->beginOp(CPUI_RETURN)..endOp` is a std::list;
  Rust `obank().iter_code(CPUI_RETURN)` walks `returnlist` (intrusive list,
  insertion order). varmap is `BTreeMap` (deterministic). No HashMap/HashSet.
- Off-by-one / do-while: clean. C++ `while(iter!=enditer){retop=*iter; ++iter; ...}`
  advances the iterator BEFORE the body; the Rust `.collect::<Vec>()`-then-`for`
  reproduces that (the snapshot is the forward order, body cannot perturb it).
- Erase-while-iterating: clean. `set_replacement` mutates only the ReplaceVarnode
  subgraph (worklist/varmap), never the function's RETURN list, so the collect-then-
  iterate visits exactly the C++ elements in the C++ order. `rvn->mask` is snapshotted
  once pre-loop (== C++ re-reading the unchanging parameter each iteration).
- Exception -> Result partial-state: clean. The mid-loop `return Ok(false)`
  (subflow.cc:262) leaves patchlist/worklist partially modified exactly as C++'s
  `return false` does (no rollback added). The seam `Err` is gone; `?` at the two
  call sites now only propagates genuine errors (none arise on the faithful path).

## special-casing / reserved-file pass

- No opcode/type-name/value/function-name hardcoding. `slot == 0` is the verbatim
  subflow.cc:241 return-address-container guard, not special-casing.
- No reserved file touched: diff is `subflow.rs` + `verify_w10_implied_vars_adversarial.rs`
  only. merge.rs / funcdata_merge.rs / variable.rs / varmap.rs and the
  ActionNameVars/merge-action code are untouched.
- No `todo!`/`unimplemented!`/`HashMap`/`HashSet`/`sort_unstable`/bare-`as` in added lines.
- C++ tree byte-untouched (git diff -- decompiler/ specs/ empty).

## minor observation (not a finding)

`o.get_in(slot)` indexes `inrefs[slot]` and would panic on an out-of-range slot,
whereas C++ `retop->getIn(slot)` on a short RETURN is UB. In practice all RETURN
ops in a function share arity (param recovery), so `slot` is always valid; the
Rust path is faithful for the realistic case and strictly safer otherwise.

## adversarial tests (verifier, committed on branch f201677)

In-module `subflow::tests`, targeting the three fragile decision branches:
- `try_return_pull_slot0_is_return_address_bail` — slot==0 bail leaves
  patchlist/pullcount/returns_traversed untouched.
- `try_return_pull_nonaggressive_consume_outside_mask_bails` — consume(0xffff)
  outside mask(0xff) bails non-aggressively BEFORE propagation; the same shape with
  aggressive=true trims (the consume gate is correctly skipped).
- `try_return_pull_terminal_trim_emits_one_patch_and_sets_traversed` — happy path:
  one parameter_patch on the RETURN at the right slot/in1, pullcount +1,
  returns_traversed set once and idempotent on re-entry.
All 3 green against the closed-seam code (the terminal-trim test additionally
discriminates against the old stub, which returned `Err` and would panic on
`.unwrap()`).

## regression

None. Passing SET did not shrink (same 4 tests in the modified file; whole
workspace 3428 green, kuna-decomp 2582 green). C++ oracle 675/675 PARITY OK
untouched. Byte-parity regression guard `w10_implied_boolless_acc_unregressed_byte_parity`
still green. clippy --lib clean (the lone `--lib --tests` clippy error is a
pre-existing `heritage.rs:3122` lint identical in the merge-base, unrelated to this
wave and outside the `--lib` build the protocol mandates).

findings: none.

losses: this wave is a partial RESTORATION of LOSS-063 (closes its `try_return_pull`
arm; the `try_call_pull`/`try_call_return_push`/`try_int2float_pull`/`is_zero_extended`
arms remain seamed and LOSS-063 stays open for those). No new loss. ACCEPT (full
parity for the ported surface, zero regression).
