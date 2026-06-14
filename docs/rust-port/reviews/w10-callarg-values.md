# w10-callarg-values
verdict: ACCEPT
verifier: claude-opus-4-8 (independent verifier, round 1)
date: 2026-06-14
item: rport/w10-callarg-values (call-argument VALUE resolution: port `ActionDeadCode::markConsumedParameters` so the recovered call arguments' def-chains survive deadcode and render as resolved data-flow values; RESTORE the C++ `Heritage::guard` "Free varnode with multiple reads" throw the prior wave downgraded — close LOSS-150)

## Scope of the diff (3 files, rust-only)
`git diff rust-port...rport/w10-callarg-values` touches ONLY:
- `rust/crates/kuna-decomp/src/coreaction_render.rs` — new `dc_mark_consumed_parameters` (ports `markConsumedParameters`, coreaction.cc:4002) + wires the `numCalls()` loop into `ActionDeadCode::apply` step 4 (coreaction.cc:4233-4235), previously a no-op seam.
- `rust/crates/kuna-decomp/src/heritage.rs` — `Heritage::guard` `descend != 1` arm: replaces the prior guard-and-continue downgrade with `panic!("kuna heritage: Free varnode with multiple reads")`, the faithful C++ throw (heritage.cc:1172).
- `rust/crates/kuna-decomp/tests/verify_w10_callsite_args_adversarial.rs` — porter's prior 3 adversarial tests UPDATED from "arg == raw register" to "arg resolves to a data-flow value" (correctly STRENGTHENED, not weakened).

ZERO non-`rust/` files (no C++ oracle, no specs, no docs). Confirmed by `--name-only`.

## Gate
- `cd /tmp/kuna-cav/rust && cargo test --workspace` -> **2946 passed, 0 failed** (2943 base + my 3 new verifier adversarial tests). All green.
- Porter's `verify_w10_callsite_args_adversarial` (3 tests) -> all pass; my `verify_w10_callarg_values_adversarial` (3 tests) -> all pass.
- `cargo clippy -p kuna-decomp --lib -- -D warnings` -> clean. `--test verify_w10_callsite_args_adversarial` -> clean. My `--test verify_w10_callarg_values_adversarial` -> clean. (`-p kuna-decomp -p kuna-harness --tests` surfaces only PRE-EXISTING test-file lints in w3/w4/w5/w6/printc_parity files — NONE in this wave's touched files nor my new file.)
- New code has NO `todo!`/`unimplemented!`/`HashMap`/`HashSet`/`sort_unstable`/`partial_cmp` (grepped the diff). One bare cast `(i - 1) as usize`, justified with a `// cast:` comment.
- C++ oracle UNTOUCHED: `python -m kuna.run_tests --all --baseline docs/baseline.json --engine cpp` -> **207/207 unit, 675/675 datatest, PARITY OK**.
- Rust datatest corpus, base (rust-port) vs branch (built `decomp_test_dbg`, `-path datatests -sleighpath specs`):
  - applied **456 -> 456** (unchanged), passing **24 -> 25 (+1)**, file-level errors ("Did not apply") **28 -> 28 (no regression)**.
  - The +1: `forloop_loaditer.xml` "For-loop iterator load #2" (`count = count + 1;`) FAIL -> Success. (One within-file swap in `ptrtoarray.xml`: #6 `return v1;` -> #4 `display(v1);`, same pass-count, both real `min=1 max=1` matches.)

## The value resolution is REAL (FOCUS 3) — before/after on the live engine
`forloop_loaditer`, rendered through the built `decomp_test_dbg` (`KUNA_DUMP`):
- **BASE (rust-port)**: `count` variable entirely dead-coded away; `printf(RDI,RSI)` (raw incoming registers).
- **BRANCH**: `count = 0;`, `count = count + 1;`, and `printf(0x400844, ZEXT(count & SUB(0xffffffff,0)) & 0xffffffff)` — the loop counter survives AND both printf args are resolved data-flow values (format-string address constant + the `count` value).

This is exactly `markConsumedParameters`' purpose: marking the recovered printf args consumed keeps the def-chains that PRODUCE them (the format-string load, the `count` increment) alive through deadcode, so they survive instead of collapsing to raw function-input registers on the next heritage pass. Genuine SSA reaching-def + consume propagation, matching the C++. The new datatest pass is REAL C++-oracle parity (`count = count + 1;` is the pinned C++ output; the 675/675 C++ run includes this file).

Spot-checked the same on three more functions (console path): `noforloop_alias` -> `might_change(RSP + 0xffffffffffffffe8)` (resolved `&i` stack address), `printf(0x40085d, ZEXT(i[1] & ...))`; `deindirect` -> `puts(0x10088f)`/`puts(0x10088a)` (resolved constant string addresses); `loadstore_fieldarray` -> `sub_100684(RSP + 0xffffffffffffff58)` (resolved stack address). DIFFERENT value KINDS (stack expr vs constant) through the IDENTICAL code path.

## The restored throw is FAITHFUL and does NOT regress (FOCUS 5) — LOSS-150 closed
The prior wave (w10-callsite-args) downgraded the C++ `throw LowlevelError("Free varnode with multiple reads")` (heritage.cc:1172) to a guard-and-continue (LOSS-150, measured FIRING 24x). This wave RESTORES the throw as `panic!`. Verified faithful:
- **Fires 0x on the corpus**: the branch `decomp_test_dbg` run over all 83 datatests shows ZERO "Free varnode with multiple reads" in stdout/stderr (vs the prior wave's measured 24 firing sites). With `markConsumedParameters` keeping each recovered argument's def-chain alive, the premature multi-read collapse no longer happens — the LOSS-150 restoration criterion ("confirm the 24 firing sites no longer occur") is MET.
- **Degrades gracefully, never aborts the process**: the panic is caught at the `decompile_func`/`decompile_func_with_symbols` boundary (`decompile_drive.rs:195`, `std::panic::catch_unwind` -> per-function `KunaError`), discarding `fd`/`arch` so no half-mutated state escapes. This is the same per-function-abort route the C++ `throw LowlevelError` takes (it propagates to the top-level decompile boundary and fails that one function; it is NOT a restart-triggering exception). My av3 wraps each corpus run in `catch_unwind` and asserts no panic escapes the boundary — confirmed.
- **No execution-failure regression**: file-level errors 28 -> 28; +1 passing; the restoration did not abort any function the downgrade previously handled.

## NO SPECIAL-CASING (FOCUS 2) — clean
Grepped the production diff (`coreaction_render.rs` + `heritage.rs`) for function-name / callee-name / address / value / string-literal hardcoding: ZERO. The only string/register literals in the whole diff are in the TEST file (assertion expectations `"RDI"`/`"RSP"`/`"RSI"` — legitimate). `dc_mark_consumed_parameters` dispatches purely on the general C++ predicates `is_input_locked()` / `is_input_active()` / `is_auto_live()` / `get_nz_mask()` / `get_input_bytes_consumed()` — never on a fixture/callee identity. My av2 proves the resolution is generic: two structurally distinct value kinds (stack-address expr in `noforloop_alias`, constant address in `deindirect`) flow through the identical path; if it were per-callee/per-address hardcoded they could not both resolve.

## Faithfulness of `dc_mark_consumed_parameters` (FOCUS 1) — line-by-line vs coreaction.cc:4002
- `callOp = fc->getOp()`; `pushConsumed(~0, getIn(0))` UNCONDITIONALLY before the lock check — preserved (the Rust `if let Some(in0) = ... get_in(0) { dc_push_consumed(!0, in0) }` runs before the `fully_consumed` branch). EXACT.
- `if (isInputLocked() || isInputActive())` -> for i in 1..numInput: `pushConsumed(~0, getIn(i))`; return. EXACT (`fully_consumed = is_input_locked() || is_input_active()`).
- else per-param: `consumeVal = isAutoLive() ? ~0 : minimalmask(getNZMask())`; `bytesConsumed = getInputBytesConsumed(i)`; `if (bytesConsumed != 0) consumeVal &= calc_mask(bytesConsumed)`; `pushConsumed(consumeVal, getIn(i))`. EXACT. `getInputBytesConsumed` (fspec.cc:5877) is a faithful `slot >= len ? 0 : inputConsume[slot]` (fspec.rs:6137).
- The borrow-split (gather `fc` reads up front, release, then `&mut` `dc_push_consumed` calls) is a sound translation of the C++ aliasing — `dc_push_consumed` mutates only the varnode consume flags, never `numInput()` or the nz-mask, so caching `n` and reading the nz-mask after the slot-0 push is equivalent to the C++ fresh reads.
- The loop wired into `apply` step 4 sits exactly between the per-op consume seeding and the propagation fixpoint (coreaction.cc:4233-4239), feeding the SAME `worklist`. EXACT.
- `dc_push_consumed`/`dc_propagate_consumed` (pre-existing, re-verified): faithful transcriptions of `pushConsumed`/`propagateConsumed` (coreaction.cc:3714/3734).

## hunt list
- **Signedness**: clean. `num_input()` is `int4`, `get_in(slot: int4)`, loops `for i in 1..n` over `int4` — matches C++ `int4 i; i<callOp->numInput()`. `slot_consumed[(i-1) as usize]` with `i >= 1` so `i-1 >= 0` and `i < n` so `i-1 < len`; in-bounds. The `0..data.num_calls()` loop is `int4`, matching `for(i=0;i<data.numCalls();++i)`.
- **Integer widths**: clean. consume masks are `u64`/`uintb`; `calc_mask`/`minimalmask`/`get_nz_mask` are the pre-existing uintb helpers. No new promotions.
- **Wrapping**: clean. `i - 1` with `i >= 1` cannot underflow; no new uintb arithmetic introduced (only mask `&`).
- **Comparator totality**: N/A (no new comparator).
- **Iteration-order provenance**: the `numCalls()` loop is a `Vec`-index walk over `qlst` (matches C++ `vector` index order); the param loops are op-input-index order (matches C++ `getIn(i)`). NO HashMap/HashSet.
- **Off-by-one / do-while / reverse**: the `1..numInput` skip-slot-0 loop and the unconditional slot-0 `pushConsumed` are preserved exactly. No do-while/reverse idioms.
- **Erase-while-iterating**: `dc_push_consumed` never resizes the op input list; `num_input()` is stable across the two reads of `n`. No erase-during-iteration. The `None => continue` on an in-bounds-but-null input slot is a safe defensive equivalent (a real CALL op never has a null input slot; alignment of `slot_consumed[i-1]` is preserved because the gather loop pushes for every `i`).
- **Exception -> Result partial-state**: the RESTORED `panic!` leaves `fd` partially mutated (earlier `read` slots already `set_active_heritage`), but `fd`/`arch` are DISCARDED at the `catch_unwind` boundary, so no half-state escapes — matching the C++ throw's function-abort (the partial in-function state is thrown away with the function). Faithful.

## findings
- F1 (nit, non-blocking): the restored throw's positive-coverage gap. LOSS-150's restoration criteria ask for "a fixture that a GENUINE multi-read free read is rejected, not silently linked." The wave verifies the 0x-firing on the real corpus (the load-bearing half) but adds no synthetic IR that FORCES the panic to fire and confirms the per-function `Err` degradation on a true multi-read. My av3 confirms the corpus path completes with the throw never escaping the catch boundary, but a constructed genuine-multi-read positive remains absent. Not a blocker: synthesizing a true multi-read free-read IR is hard, the C++ itself relies on the same invariant being unreachable, and the restoration is verified correct on every corpus function.
       cpp: decompiler/cpp/heritage.cc:1172
       rust: rust/crates/kuna-decomp/src/heritage.rs:1316
- F2 (nit, non-blocking): the porter's adversarial tests (and the harness-driven ones generally) carry a SKIP-on-error fallback (return without asserting if `.sla` bootstrap is unavailable). In this environment the specs are built and the tests demonstrably run the real assertions (they print the rendered bodies). A spec-less CI would silently skip rather than fail. Pre-existing pattern, carried from w10-callsite-args; not introduced here.
       rust: rust/crates/kuna-decomp/tests/verify_w10_callsite_args_adversarial.rs (the `Err(e) => { eprintln!("SKIP ..."); return; }` arms)

## adversarial tests (added by the verifier, land on the branch, all green)
rust/crates/kuna-harness/tests/verify_w10_callarg_values_adversarial.rs (3 tests; drive the SAME public `FunctionTestCollection` datatest harness the gate uses, with `KUNA_DUMP` to inspect the rendered C):
- `av1_forloop_callarg_keeps_counter_and_resolves_printf_args` — the load-bearing claim: `forloop_loaditer` must render the surviving `count = count + 1;` AND a value-resolved `printf` (`0x400844`/`count`, NOT the pre-wave `(RDI,RSI)`); the datatest verdict "For-loop iterator load #2" must be Success. Directly tests the `markConsumedParameters` def-chain-survival mechanism.
- `av2_callarg_resolution_is_generic_not_special_cased` — `noforloop_alias` resolves the slot to a STACK-ADDRESS value (`RSP + <neg const>`), `deindirect` resolves the SAME slot to a CONSTANT string-address; neither is the bare register `RDI`. Two value KINDS through one path = no special-casing.
- `av3_restored_multiread_throw_fires_zero_and_corpus_completes` — runs a spread of call-heavy datatests under `catch_unwind`; asserts the restored "Free varnode with multiple reads" panic NEVER surfaces (fires 0x) and NEVER escapes the `decompile_func` boundary, and the value-resolution match still passes after the restoration. Guards both LOSS-150 restoration criteria.
All three exercise real assertions (verified non-vacuous via `--nocapture`).

## losses
NONE introduced. This wave CLOSES **LOSS-150** (the `Heritage::guard` multi-read throw downgrade): the throw is restored (heritage.rs:1316), the 24 prior firing sites now fire 0x (verified on the corpus), and the degradation is gone. LOSS-150's losses.md entry is left intact per the append-only protocol; its restoration criteria are met (modulo the F1 synthetic-positive nit). The pre-existing LOSS-149 (call-side INDIRECT/output recovery), LOSS-151 (ActionRestartGroup drop), LOSS-152 (callee_pop i64 vs int4), LOSS-153 (stack/JOIN/name seams) are UNCHANGED by this wave.

The wave's load-bearing claim — call arguments render as REAL resolved data-flow values (genuine SSA reaching-def + consume propagation, C++-faithful) and the prior heritage-guard degradation is restored without regression — is verified. No special-casing, no faked resolution, no shared-code regression (corpus 24->25 passing, 28->28 errors, C++ 675/675 PARITY OK untouched). **ACCEPT.**
