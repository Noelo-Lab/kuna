# w10-stackvar-promotion
verdict: ACCEPT-WITH-LOSSES
verifier: claude-opus-4-8 (independent verifier, round 2)
date: 2026-06-14
gate: `cd rust && cargo test --workspace` -> 3200 passed / 0 failed (3197 on the branch + my 3 round-2 adversarial). `cargo clippy -p kuna-decomp -p kuna-console --lib -- -D warnings` -> clean (the `--tests` clippy errors are all pre-existing `tests/verify_w{5,6}_*.rs` doc-list lints in files this branch does not touch — not in scope/not in the lib gate). C++ oracle (MAIN tree, untouched by this branch — verified zero `.cc/.hh/spec` changes): `python -m kuna.run_tests --all --baseline docs/baseline.json` -> 207/207 unit + 675/675 datatests, PARITY OK.

## Round-2 summary (one line)

The repair round (commit `cce854f`, "F0-F7") makes the chain **real, faithful, and
tested**: `ActionRestructureVarnode::apply` is now wired to the genuine
`restructure_varnode`/`sync_varnodes_with_symbols` chain, `ActionStackPtrFlow` is fully
ported (checkClog/repair/adjustLoad/isStackRelative/StackSolver/analyzeExtraPop) with
8 positive unit tests, and the F3/F4/F7 fidelity gaps are restored. Every Round-1
BLOCKER is resolved. The remaining divergences are pre-existing, documented seams
(LOSS-012/145/146/147/148, all out of this item's scope) — hence ACCEPT-WITH-LOSSES,
not ACCEPT.

## The Round-1 REJECT blockers — each verified resolved

- **F0 (promotion was fake/absent) -> RESOLVED.** Empirical proof via the harness's
  `KUNA_DUMP` capture of the live Rust release engine's `print C`. Where Round 1
  rendered raw `Stackffffffffffffffe0` + an unfolded `LOAD`, the branch now renders
  genuinely **named, mapped, typed** stack locals:
  - `noforloop_alias`: `int4 i [4]; // stack - 0x18` with `i[1]` (array-form named local;
    **byte-identical** to the C++ oracle's local declaration `int4 i [4]; // stack - 0x18`).
  - `pointercmp`/`myinit`: `uint8 pchar; // stack - 0x10`, `pchar = pchar + 1`.
  - `forloop_loaditer`: `loopvar; // stack - 0x20`.
  The promotion is REAL and engine-driven (the spacebase location became a mapped,
  named local via `ScopeLocal`), not a render shortcut.
- **F1 (`ActionStackPtrFlow::apply` was a stub) -> RESOLVED.** New `coreaction_stackptr.rs`
  (700 lines) ports `StackEqn`/`StackSolver::{propagate,duplicate,solve,build}`,
  `isStackRelative`, `adjustLoad`, `repair`, `checkClog`, `analyzeExtraPop`. `apply` now
  runs `check_clog` then (when no clog) `analyze_extra_pop`, exactly per
  coreaction.cc:496-512. Wired into the **real** pipeline at universalaction.rs:584 with
  the real `conf->getStackSpace()` (the `render_actions(... None)` at coreaction_render.rs:2049
  is only a B0 identity-roundtrip test helper, not the runtime path).
- **F2 (no tests + false coverage comment) -> RESOLVED.** 8 real positive tests
  (`check_clog_folds_load_to_copy` asserts the LOAD became `COPY(0x1234)`,
  `stacksolver_solves_add_chain`, `stacksolver_propagate_uses_guess_for_indirect`,
  `is_stack_relative_*`, `spacebase_marks_input_stack_pointer`, `spacebase_leaves_free_*`);
  the false "exercised end-to-end by the datatest corpus" comment on `loadvarnode_seam_noop`
  is removed.
- **F3 (`isReadActive` over-reports same-storage PIECE) -> RESOLVED.** The C++ slot-address
  filter (`slot = bigEndian?0:1; if(in[slot]!=vn) addr+=in[slot].size; active iff vn.addr!=addr`)
  is transcribed verbatim (funcdata_spacebase.rs:455-481).
- **F4 (`gatherVarnodes` collapsed PIECE/SUBPIECE) -> RESOLVED.** Explicit PIECE
  (two per-slot COPYs + read-active whole-output hint) and SUBPIECE (same-storage
  truncation filter via the big/little-endian `trunc` computation) cases restored
  (funcdata_spacebase.rs:354-424), matching varmap.cc:1165-1197.
- **F7 (`syncVarnodesWithSymbol` SET-addrtied violated the C++ invariant) -> RESOLVED.**
  The C++ mask is now **verbatim** (`mapped`; `addrtied|addrforce` added to the mask ONLY
  when `(fl&addrtied)==0` -> CLEAR-but-never-SET; `nolocalalias|addrforce` when nolocalalias)
  at funcdata_spacebase.rs:686-696. The Round-1 `else { mask |= addrtied }` SET-inside-the-mask
  arm is gone; the address-tied pre-tie is now a **separate, documented** step
  (funcdata_spacebase.rs:663-684) that reaches the same end-state `setSymbolEntry` reaches
  in C++. The `nolocalalias` stub remains (LOSS-147/148).

## Faithfulness audit (transcription, not shortcut) — the new ActionStackPtrFlow

Read every link against coreaction.cc:40-512:

- `StackSolver::propagate` (cc:82): `partition_point(|e| e.var1 < cur)` == C++
  `lower_bound(eqs, {var1=cur}, compare)` over the duplicate-sorted `eqs`; workstack DFS,
  the `soln[var2]==65535` guard, `soln[var2]=soln[cur]-rhs`. Faithful.
- `StackSolver::duplicate` (cc:111): mirrors every eqn (swap vars, **negate** rhs) then
  `sort_by(compare)` == `stable_sort`. Faithful — pinned by my adversarial test (sign + stability).
- `StackSolver::solve` (cc:127): the `do/while(count>0)` with `if(count==lastcount) break`
  is reproduced as `loop { …; if count==lastcount break; lastcount=count; if count<=0 break }`
  — equivalent (do-while runs the body once, then the same two exit checks in the same order).
  `propagate(0,0)`, `lastcount=size+2`. Faithful.
- `StackSolver::build` (cc:162): the spacebase-ref collection (break at first `isFree`),
  the `vnlist[0].isInput()` throw->`Err`, and every opcode arm (INT_ADD const-swap,
  COPY, INDIRECT with the **verbatim `rhs=4` guess** taken because `numCalls()==0`,
  MULTIEQUAL, INT_AND-as-copy, else `missedvariables+=1`). Faithful.
- `index_of` linear `position(|&v| v==othervn)` substituting `lower_bound(vnlist,othervn,comparePointers)`:
  VERIFIED equivalent. `comparePointers` is `*a < *b`; `Varnode::operator<` is a strict total
  order tie-breaking same-address varnodes by (size, input/written, def SeqNum), and `vnlist`
  is built in exactly that loc order from `beginLoc(size,addr)`. `othervn` is always a member
  (it carries the spacebase address), so `lower_bound` returns the index of `othervn` itself —
  the same unique element the id-equality `position` finds. The note at the file head is correct.
- `isStackRelative`/`adjustLoad`/`repair`/`checkClog` (cc:344-495): byte-faithful incl. the
  `INT_MULT`-by-`calc_mask` (multiply-by-(-1)) unwrap, the x/y swap-and-retry, and the
  block-walk back-trace. The `repair` predecessor-cross (`iter==begiter -> sizeIn()==1 ->
  cross; iter=endOp; continue; --iter`) is reproduced by crossing to `bb_op_tail` and
  inspecting it (same op visited). The wrap-prone overlap test
  `constnew <= constz.wrapping_add((loadsize-1) as uintb) && constnew.wrapping_add((datasize-1) as uintb) >= constz`
  matches the C++ unsigned arithmetic. Faithful — pinned by my adversarial overlap test.
- `analyzeExtraPop` (cc:276): `evalfp_called ?: defaultfp` extrapop-known early-out,
  `build`->`Err`-on-malformed-frame early return, `solve`, then for each solved var the
  `paramlist={invn, newConstant(sz, soln & calc_mask(sz))}` INT_ADD rewrite. The
  `(soln as uintb) & cmask` sign-extends an `int4` exactly as the C++ signed-int -> unsigned-long
  promotion (verified: `(-4i32) as u64 == 0xFFFF…FFFC`). The `setEffectiveExtraPop` call-spec
  write is the only dropped piece (LOSS-148). Faithful w/ documented loss.
- `ActionRestructureVarnode::apply` wiring (cc:2332): `aliasyes=(numpass!=0)`,
  `l1->restructureVarnode(aliasyes)`, `if(syncVarnodesWithSymbols(l1,false,aliasyes)) count+=1`,
  `numpass+=1`; `protectSwitchPaths` is the W8 jumptable seam. Faithful. (Returning `count`
  as the `ApplyResult` is the kuna Action convention — the wrapper owns the `count` member,
  action.rs:221; `ActionStackPtrFlow::apply` likewise returns its `count` while discarding
  `analyze_extra_pop`'s return, exactly mirroring C++ `count += 1; return 0` and the
  analyzeExtraPop-changes-don't-bump-count semantics.)

## Hunt list

- **Signedness:** CLEAN. `StackEqn` vars/rhs are `int4` (matching C++); `repair`'s
  `loadsize == datasize` is `int4 == int4`; `analyzeExtraPop`'s negative solution flows
  through `int4` and sign-extends under `& calc_mask`. Pinned by
  `adv_w10_stacksolver_negative_solution_sign_extends`. The carried-over `RangeHint::compare`
  signed `sstart` ordering is still pinned by `adv_w10_compare_signed_start_on_real_stack_offsets`.
- **Integer widths:** CLEAN. `int4`=i32, `uintb`=u64, `uint4`=u32 (kuna-base/types.rs);
  `point.size as int4`, `(loadsize-1) as uintb`, `soln as uintb` each verified value-faithful
  to the C++ implicit conversion. `rhs = const_off(...) as int4` matches `eqn.rhs = constvn->getOffset()`
  (uintb truncated to int4 — both truncate identically).
- **Wrapping:** CLEAN. `propagate` `soln[cur]-rhs` and `repair`'s overlap arithmetic use
  `wrapping_*` matching C++ unsigned; `gatherOffset` (carried from round 1) `wrapping_add/sub/mul`.
  Pinned by `adv_w10_gather_offset_int_sub_wraps_at_zero` + `adv_w10_repair_overlapping_store_rejects_fold`.
- **Comparator totality:** CLEAN. `StackEqn::compare` orders by `var1` only (a non-total
  pre-order, but used with `stable_sort` exactly as C++ — equal-`var1` ties keep insertion
  order, which is the behavior `propagate`/`build` rely on). Pinned by
  `adv_w10_stacksolver_duplicate_negates_and_stable_sorts` (stability assertion).
- **Iteration-order provenance:** CLEAN. No HashMap/HashSet in the new production code.
  `build`/`checkClog` iterate `iter_loc_size_addr(size, spacebase).collect()` — a verbatim
  `beginLoc(size,addr)..endLoc(size,addr)` location-sorted range (varnode.rs:1928); `gatherVarnodes`/
  `is_read_active` use `iter_loc` filtered by space index / `descend_iter` (a `SmallVec`,
  insertion order = C++ `list<PcodeOp*>`).
- **Off-by-one / do-while / reverse iterators:** CLEAN. `solve`'s `loop`+break keeps the
  do-while at-least-once. `repair`'s `for(;;)` -> `loop` with only `return` exits. `partition_point`
  == `lower_bound` (first `>= cur`).
- **Erase-while-iterating:** CLEAN. `build`/`checkClog` collect refs to a `Vec` **before**
  mutating the Funcdata (no live-iterator invalidation); `gather_varnodes`/`sync` likewise
  collect then mutate, visiting the same set in the same order as the C++ `iter++` walks.
- **Exception -> Result partial-state:** CLEAN. `build`'s `vnlist[0] not input` C++-throw ->
  `Err(())`; `analyzeExtraPop` catches it (C++ try/catch warnHeader) -> early return 0.
  The cosmetic `warningHeader` text is omitted (not load-bearing) — noted, not a finding.

## Mechanical pass

- `grep -E 'HashMap|HashSet|todo!|unimplemented!|sort_unstable|partial_cmp|.unwrap()'` over
  `coreaction_stackptr.rs` + the new `funcdata_spacebase.rs` production code -> EMPTY (CLEAN).
- cargo test --workspace: 3200 passed, 0 failed.
- lib clippy (`-p kuna-decomp -p kuna-console --lib`) clean. `--tests` clippy errors are all
  in pre-existing `tests/verify_w{5,6}_*.rs` (doc-list lints) untouched by this branch.

## No-special-casing audit (PASS)

Grepped the full branch diff for function names (`forloop_loaditer`/`pointercmp`/
`noforloop_alias`/`myinit`/`might_change`), local names (`loopvar`/`count`/`pchar`/`i`),
register names (`rsp`/`esp`/`rbp`/`rdi`/`edi`), and the literal datatest stack offsets/
addresses (`0xffffffffffffffe0`, `0xfffffffffffffff4`, `0x4005fb`, `0x4006xx`): **every hit
is in test code or a comment** — the only `loopvar`/`count`/`0xffff…fff4` occurrences are
local variable names and pinned offsets inside `varmap/tests.rs` adversarial tests, never a
production branch. The stack space is found via `manage().get_stack_space()` / spacebase
records; the magic numbers in production (`65535` solver sentinel, `4` extra-pop guess,
`calc_mask(csz)` -1 multiply test, `othervn_off==1` PTRADD) are the verbatim C++ literals.
The chain is genuinely generic.

## Findings

- F1 (LOSS): `analyzeExtraPop` drops the per-call `setEffectiveExtraPop(soln - companionSoln)`
  propagation (the IOP->FuncCallSpecs write), and `restructureVarnode`'s entire C++ tail
  (`clearUnlockedCategory`/`fakeInputSymbols`/`sortAlias`/`markUnaliased`/`annotateRawStackPtr`)
  is still absent. Consequence: on the current datatest corpus the ported checkClog/
  analyzeExtraPop bodies are faithful + unit-tested but **dormant for full normalization** —
  the stack pointer is not fully repaired (`noforloop_alias` renders raw
  `RSP = RSP + 0xffffffffffffffd8;`) and the LOAD-fold-driven `loopvar[3]` form does not
  appear; the 3 chain datatests stay 0/9 (= the `rust-port` base; no regression).
    cpp: decompiler/cpp/coreaction.cc:311-322 (setEffectiveExtraPop), varmap.cc:1256-1286 (restructure tail)
    rust: rust/crates/kuna-decomp/src/coreaction_stackptr.rs:677-697 ; funcdata_spacebase.rs:234 -> LOSS-148 (+ LOSS-145)
- F2 (LOSS): `gatherOpen` still omits the LoadGuard/StoreGuard `addGuard` loops
  (faithful-empty today, no load guards in the merged tree).
    cpp: decompiler/cpp/varmap.cc:1241-1248 (+ addGuard :1004-1039)
    rust: rust/crates/kuna-decomp/src/funcdata_spacebase.rs (gather_open) -> LOSS-146
- F3 (LOSS): `syncVarnodesWithSymbol`'s `nolocalalias` is stubbed to never-set (`fl=0` on the
  no-symbol/unmapped-alias-check arm) because the alias map is not built. (The F7 SET-addrtied
  invariant violation is REPAIRED — see above; only the nolocalalias half of LOSS-147 remains.)
    cpp: decompiler/cpp/funcdata_varnode.cc:999-1001 (`isUnmappedUnaliased ? nolocalalias : 0`)
    rust: rust/crates/kuna-decomp/src/funcdata_spacebase.rs:636-639 -> LOSS-147
- F4 (note, non-blocking): the cosmetic `warningHeader` strings ("Stack frame is not setup
  normally", "Unable to track spacebase fully") emitted by `analyzeExtraPop` on malformed/
  unsolvable frames are omitted (the analysis result is identical). Not load-bearing; not a
  divergence in produced IR.
    cpp: decompiler/cpp/coreaction.cc:288-291, :303-305
    rust: rust/crates/kuna-decomp/src/coreaction_stackptr.rs:651-654, :668-671

## Adversarial tests added (committed on the branch, 9ae33b5)

rust/crates/kuna-decomp/src/coreaction_stackptr/tests.rs:
- `adv_w10_stacksolver_duplicate_negates_and_stable_sorts` — pins `duplicate()` mirroring
  (swap vars, **negate** rhs) and `stable_sort` order-stability for equal `var1` rows.
- `adv_w10_stacksolver_negative_solution_sign_extends` — a `sp -= 0x18` add-chain solves to a
  negative `int4`, and `(soln as uintb) & calc_mask(8)` sign-extends to `0xffff…ffe8` exactly
  as the C++ `soln & calc_mask` (the signedness hunt-list trap).
- `adv_w10_repair_overlapping_store_rejects_fold` — the wrap-prone overlap guard in `repair`:
  a prior STORE at a different stack offset whose byte range overlaps the LOAD blocks the
  fold (LOAD stays a LOAD, `check_clog` returns 0).
All 3 pass. (The 3 round-1 tests in `varmap/tests.rs` remain and pass.)

## Engine reality (honest scope statement)

The named-local promotion runs end-to-end (KUNA_DUMP confirms it on the live Rust engine).
The datatest pass/fail count is **24/425, byte-identical to the `rust-port` base** (diffed
the passing-assertion sets: zero new passes, zero regressions) — so there is NO fake/vacuous
parity, and equally no new whole-assertion parity. The branch's real value is the faithful,
tested chain (the stack-pointer-flow port + the engine-driven restructure wiring + the
F3/F4/F7 fidelity fixes), with the residual gap (full stack-pointer normalization + the
folded `loopvar[3]` + the `for(...)` structuring) bounded to the documented proto-recovery /
restructure-tail / block-structuring seams.

## Losses cited

LOSS-145 (restructureVarnode tail still dropped), LOSS-146 (gatherOpen addGuard loops),
LOSS-147 (nolocalalias stub; its SET-addrtied half is now repaired), LOSS-148 (NEW:
analyzeExtraPop setEffectiveExtraPop propagation + the dormant-on-current-corpus reality),
and the pre-existing LOSS-012 (stack SpacebaseSpace / proto-recovery blocker behind it).
