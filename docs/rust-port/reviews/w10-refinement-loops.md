# w10-refinement-loops
verdict: ACCEPT-WITH-LOSSES
verifier: claude-opus-4-8 (independent)
date: 2026-06-14
verifier round: 2
gate: `cargo test --workspace` -> 3175 passed / 0 failed; rust `decomp_test_dbg` datatests -> applied 391 / passing 20; C++ oracle `decomp_test_dbg` datatests -> 675/675 PARITY OK (untouched)
worktree: /tmp/kuna-loops (branch rport/w10-refinement-loops, HEAD 923da87 + verifier test commit 2e28a85)

## Verdict

**ACCEPT-WITH-LOSSES.** Round 1's single blocker (BLOCKER-1: divopt mis-lifted as
16-bit real-mode garbage, so the "applied-count rise" was vacuous and the
celebrated `do-while` was a wrong-direction artifact) is **genuinely resolved**.
Round 2 adds commit `923da87`, which ports the C++ `parseProcessorConfig`
`ELEM_CONTEXT_DATA` branch so the engine's context database receives the pspec
`<context_data><context_set>` paints (addrsize/opsize/longMode) before any
instruction decodes. I proved this fix is **real and load-bearing** (not a
shortcut): with the context decode disabled the lift reverts to the exact Round-1
16-bit garbage; with it on, divopt lifts correct 64-bit straight-line division
and forloop1 collapses to a real loop — both matching the C++ oracle DIRECTION.
The refinement + loop-emitter transcription (unchanged since the Round-1 faithful
review) is retained. Three accepted divergences are recorded as cited losses.

## Round-1 blocker resolution (BLOCKER-1 — CLEARED)

The fix (`923da87`) is faithful to C++ `Architecture::parseProcessorConfig`
(architecture.cc:1176, the `else if (subId == ELEM_CONTEXT_DATA)
context->decodeFromSpec(decoder)` dispatch):

- `Architecture::parse_processor_config` (architecture.rs:995) opens the
  `<processor_spec>`, finds `<context_data>`, and drives
  `context->decode_from_spec` — the `ContextInternal::decodeFromSpec` port
  (globalcontext.rs:1017) is a faithful transcription of globalcontext.cc:531
  (the Range/CONTEXT_SET/TRACKED_SET loop). Wired into `init_post_engine`
  (architecture.rs:1293) **before** `build_typegrp` — matching the C++ ordering
  (parseProcessorConfig before the type/action build, so the context is in place
  before any decode). Wired into the production `bootstrap_program`
  (engine.rs:+245) too.
- `VarnodeData::decode_from_attributes` `name=` (register) path
  (pcoderaw.rs:190) ported — a previously-deferred seam — faithful to
  pcoderaw.cc:33 (`trans->getRegister(name); *this = point`), reusing the
  pre-existing `RegisterLookup` manager stand-in that `Range::decode_from_attributes`
  already used.
- `SnapshotRegisterLookup` (sleighbase.rs:+604) is a standalone `RegisterLookup`
  over the engine's `varnode_xref` (no `Rc` cycle), installed via
  `Sleigh::install_register_lookup` (sleigh.rs:+1507) during bootstrap. The
  location->name algorithms (`getRegisterName`/`getExactRegisterName`) are a
  **verbatim move** of the pre-existing `SleighBase` code into free functions
  (byte-identical, confirmed by diff); the name->storage `get_register` inverts
  the xref (see LOSS-142).

**Proof the rise is REAL, not faked (measured directly via the rust decomp_test_dbg
datatest harness):**

| state | applied | passing |
|---|---|---|
| Round-1 (refinement+loops only, pspec fix reverted) | 357 | 21 |
| Round-2 (full) | **391** | **20** |

The +34 applied = divopt's assertions, which **formerly aborted the file** and now
reach the harness. The -1 passing is the honest framing the commit claims: the
two Round-1 *vacuous* negative-on-garbage passes (concatsplit #7, deindirect2 #3
are `min=0 max=0` absence matches the garbage satisfied for free) are correctly
lost, and **1 real positive is gained**. Quality up, count down by 1.

**Proof the fix is load-bearing (env-gated A/B against the C++ oracle direction):**
I spot-checked the rendered `print C` (throwaway probe, reverted) for the exact
files the wave celebrates:

- **divopt (oracle: straight-line `*divu = *divu / 81;` … Division/Modulo
  assertions, NO loop):** the Rust now renders 64-bit straight-line:
  `STORE(3,RDI + 0x10, SUB(ZEXT(LOAD(3,RDI + 0x10)) * ZEXT(0x6816816816816817),8) …)`
  — RDI-indexed array stores, the 64-bit reciprocal magic constants present,
  ZERO 16-bit AX/SI/DI, NO loop. This matches the oracle DIRECTION. (The
  reciprocal-multiply -> `/` recognition + pointer typing is a documented later
  seam, so the divopt assertions are *applied* but *fail* — honest, not vacuous.)
- **forloop1 (oracle: `for(v1=0; v1<max; v1=v1+1)`):** the Rust collapses the
  loop CFG and renders `while( true ) { … RBX = ZEXT(SUB(RBX,0) + 1) & …; break; }`
  — a real structurer collapse + loop emitter firing on an ORACLE-loop function,
  not a mis-lift substring. (Exact `for(...)` normalization is the next seam.)
- **With `<context_data>` decode disabled** (env-gated, simulating Round-1), the
  lift is the exact Round-1 garbage (`AX`, `CX`, `0xffff`, 16-bit-truncated
  magics `0x6817`, goto/LAB) and **all three of my adversarial tests FAIL** —
  confirming the pspec fix is precisely what flips garbage -> correct.

## What is FAITHFUL

- **The refinement + loop-emitter port is unchanged since Round 1** (commit
  `b150a15`; not touched by `923da87`). The Round-1 review verified it
  line-by-line as faithful (concat/split pieces, buildRefinement,
  refineRead/Write/Input, remove13_refinement, refinement, guard_input,
  place_multiequals erase-while-iterating, the emitBlockWhileDo/DoWhile/InfLoop/
  Goto emitters). I re-spot-checked `split_pieces` against heritage.cc:564
  (baseoff BE `+size`, diff BE/LE, SUBPIECE in0=startvn/in1=const(4,diff)/out=vn,
  wrapping arithmetic): faithful.
- **The pspec/register wiring** (the only new code): faithful to
  parseProcessorConfig / decodeFromSpec / decodeFromAttributes / getRegister, as
  analyzed above.
- **block.rs** adds three accessors (`get_iterate_op`/`get_goto_target`/
  `get_goto_type`) matching C++ BlockWhileDo/BlockGoto getters. **space.rs** only
  widens `no_register_lookup_err` visibility `pub(crate)`->`pub`. Both benign.

## NO SPECIAL-CASING

Grepped every changed **source** file (architecture.rs, sleighbase.rs, sleigh.rs,
pcoderaw.rs, engine.rs, space.rs, block.rs, heritage.rs, printc.rs) for
function-name / address dispatch (`== "divopt"`, `name ==`, `== 0xNNNN`,
per-function branches): **NONE**. All `divopt`/`forloop1`/`elseif`/`boolless`
references are confined to the test file (legitimate corpus names in assertions).
PASS.

## Hunt list

- **Special-casing / name / address branches:** clean (source). PASS.
- **Signedness:** the moved `register_name_from_xref` uses `size as u32` (C++
  int4->uint4, size non-negative, justified) and `off.wrapping_add(size as u64)`
  (C++ `off + size` unsigned). Verbatim move of pre-existing code. OK.
- **Integer widths / Wrapping:** `wrapping_add`/`wrapping_sub` on the xref offset
  arithmetic (matches C++ silent unsigned wrap). `split_pieces` (unchanged) uses
  wrapping throughout. OK.
- **Comparator totality:** `VarnodeStorage::Ord` (the `varnode_xref`/`by_name`
  BTreeMap key) is a total order — space index, then offset, then size big-first
  — a verbatim transcription of C++ `VarnodeData::operator<`, no floats. OK.
- **Iteration-order provenance:** all new maps are `BTreeMap` (no HashMap/HashSet
  in source — grep clean). `by_name` is point-lookup-only (`.get(nm)`), order-
  insensitive. The location->name walk reuses the C++ `upper_bound`/`Included`
  range idiom (the Round-1-noted F1 fencepost is correct: `Included` not
  `Excluded`). OK.
- **Off-by-one / do-while / reverse iterators:** the moved `getRegisterName`
  `--upper_bound` -> `range(.., Included(&key)).next_back()` is the correct
  greatest-element-<=-key. `decodeFromSpec` is a faithful `for(;;) openElement`
  loop. OK.
- **Erase-while-iterating:** none new (the refinement erase-walk is unchanged,
  verified Round 1). OK.
- **Exception -> Result partial-state:** `parse_processor_config` returns
  `Ok(())` on missing pspec/`<processor_spec>`/`<context_data>` (faithful "no
  paints to apply"); a pspec read failure in `bootstrap_program` is non-fatal
  (keeps zero context) — matches the "no context configuration" graceful path.
  OK.
- **Mechanical:** `todo!`/`unimplemented!`/`unreachable!`: none in source.
  Bare `as` casts: only the verbatim-moved xref casts, each justified. OK.
- **clippy:** `cargo clippy -p <crate> --lib -- -D warnings` CLEAN for all five
  changed crates (kuna-base/num/sleigh/decomp/console). `cargo clippy -p
  kuna-decomp --test verify_w10_struct_corpus -- -D warnings` CLEAN. The one
  workspace `--all-targets` clippy error (`overly_complex_bool_expr` at
  heritage.rs:3031, `!i.is_heritaged() || true` in test
  `heritage_new_and_clear_state`) is **pre-existing** (commit `24d03fd`, w5, test
  code) — not this branch.

## Gates

- `cargo test --workspace`: **3175 passed, 0 failed** (incl. the porter's 3 new
  e2e tests + my 3 adversarial tests — all pass on the real fix, all FAIL on the
  context-disabled garbage).
- rust `decomp_test_dbg` datatests: applied **391** (357->391, the real +34
  divopt rise), passing **20** (21->20, -2 vacuous +1 real).
- C++ oracle `decomp_test_dbg` datatests: **675/675 PARITY OK**, untouched
  (re-run from main tree).

## Why ACCEPT-WITH-LOSSES (not plain ACCEPT)

Three accepted divergences (LOSS-139/140/141, carried from Round 1) plus one new
this round (LOSS-142), each a cited `losses.md` entry:

- **LOSS-139** emitBlockGoto over-emit (gotoPrints/nextFlowAfter unported) — a
  redundant trailing goto, never an under-emit; unobservable on the corpus.
- **LOSS-140** emitBlockWhileDo never emits the `for` form + skips
  emitAnyLabelStatement/emitCommentBlockTree — dormant (the structurer never sets
  iterate_op); forloop1 renders `while(true){…break;}` not `for(...)`, the
  documented next seam.
- **LOSS-141** splitPieces per-piece insertion order reversed vs the C++ fixed
  iterator — benign (independent defs, dataflow unaffected).
- **LOSS-142** (new) SnapshotRegisterLookup resolves `getRegister(nm)` by
  inverting `varnode_xref` rather than the SLEIGH symbol table; agrees on
  well-formed specs (xref is a name bijection), diverges only on the
  upstream-fatal duplicate-location case.

## Adversarial tests added (committed 2e28a85, in kuna-decomp)

- `verify_w10_r2_divopt_reciprocal_multiply_is_64bit` — the 64-bit reciprocal
  magic `0x948b0fcd6e9e0653` + `0xffffffffffffffff` mask present, `0xffff`
  absent (a 16-bit lift cannot produce 64-bit magics).
- `verify_w10_r2_divopt_stores_through_rdi_straightline` — >=8 `STORE(_,RDI,…)`
  array-element stores, zero loop keywords (oracle is loop-free).
- `verify_w10_r2_forloop1_is_bounded_loop_not_bare_keyword` — loop keyword +
  `break;` (exit edge recovered) + `+ 1` (induction step) — a real CFG collapse.

All three pass on the fix and were **verified to FAIL on the Round-1 garbage**
(context decode disabled), proving they are genuinely adversarial and the pspec
fix is load-bearing.

## losses
LOSS-139, LOSS-140, LOSS-141, LOSS-142.
