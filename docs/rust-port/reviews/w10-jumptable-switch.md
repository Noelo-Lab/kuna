# w10-jumptable-switch
verdict: ACCEPT-WITH-LOSSES
verifier: claude-opus-4-8[1m] (independent verifier)
date: 2026-06-15
round: 1

gate: full Rust workspace `cargo test --workspace` -> **3284 passed / 0 failed / 37 ignored**. C++ oracle `python -m kuna.run_tests --all --baseline docs/baseline.json` -> **207/207 unit + 675/675 datatest, PARITY OK** (the branch touches NO C++ — only `rust/`). `cargo clippy -p kuna-decomp -p kuna-sleigh -- -D warnings` (lib) clean. The porter's 5 `verify_w10_jumptable_emulate.rs` tests PASS; my 5 `verify_w10_jumptable_switch_adversarial.rs` tests PASS. (The `--tests` clippy profile reports pre-existing `const_is_empty` errors in untouched files `printc_parity.rs` / `verify_w4_fw_architecture_r2.rs` — newer-clippy drift, NOT introduced here; my new test file is clippy-clean.)

## TL;DR
The branch is a **faithful, non-special-cased, behaviorally-inert FOUNDATION**, honestly labeled as such ("jump-table recovery foundation" in the commit; "remain seamed" / "reported as the wave's remaining blocker" in the code). It does NOT deliver the switch recovery the task brief assumes (recoverModel/recoverAddresses/stageJumpTable/BlockSwitch/emitBlockSwitch with new passing assertions and an exec-failure DROP) — those are still `Err` seams, and the ported emulator is **dead/uncalled** on this branch. ACCEPT-WITH-LOSSES because the delivered leaf is faithful and breaks nothing, but the headline capability is not realized (LOSS below), matching the project's established dormant-downstream pattern (LOSS-156/158/159/160/161).

## Scope reality (the brief vs. the diff)
The brief frames the item as full jump-table switch recovery + new parity + an exec-failure drop. The actual diff (`git diff rust-port...rport/w10-jumptable-switch -- rust/`, 1 commit, +722/-2) touches ONLY:
- `kuna_emulatefunction.rs` (NEW, 475 LOC) — `EmulatePcodeOp`+`EmulateFunction` folded into one struct over the `Emulate` trait.
- `seams.rs` (+49) — `Architecture::{max_jumptable_size,funcptr_align,loader}` fields + `get_load_image_value`.
- `architecture.rs` (+26), `sleigh.rs` (+47), `lib.rs` (+1), and the porter's test file.

What is STILL a seam returning `Err` (unchanged by this branch): `JumpTable::recover_model` / `recover_addresses` / `recover_multistage` / `match_model` / `recover_labels` / `set_override` (jumptable.rs:2143+), and `stageJumpTable` (funcdata_block.rs). `BlockSwitch` structuring and `emitBlockSwitch` are not in scope of this diff at all. CONSEQUENCE: no switch recovery runs end-to-end; **zero new passing datatest assertions; zero exec-failure drop** (the brief's premise does not apply to this branch).

## Faithfulness of the delivered leaf (verified line-by-line vs C++)
- `getLoadImageValue` (emulateutil.cc:30) — `seams.rs::get_load_image_value` + `sleigh.rs::read_loadimage_value`: `loadFill` a full `sizeof(uintb)`==8 bytes; `from_{le,be}_bytes` per HOST_ENDIAN; conditional `byte_swap(res,8)` when `(HOST_ENDIAN==1)!=isBigEndian`; then **`if big && sz<8 { res >>= (8-sz)*8 } else { res &= calc_mask(sz) }`** — EXACT transcription of C++ lines 36-44. Pinned by AT1-AT4 below (sz==8 boundary, logical-not-arithmetic shift, partial-read-at-offset, always-8-wide read).
- `EmulateFunction::emulatePath` (jumptable.cc:217) — the startop search, the MULTIEQUAL-start rewind (`startvn=getOut(); i-=1; startop=getOp(i)`, guarded by `(j==numInput())||(i==0)` throw so no underflow), `setVarnodeValue(startvn,val)` when non-constant, the `while(i>0)` reverse execution, and the final `getOp(0)->getIn(0)` read — all faithful. Index `i` is `int4` (C++ `uint4`); benign because the only decrement is `i==0`-guarded, `get_op` returns `Option` (OOB → caught Err, not UB), and the magnitudes (ops in a PathMeld) are tiny — no wrap risk.
- `executeLoad` override (jumptable.cc:114): LOAD-point collection when `loadpoints` is set, then the base `EmulatePcodeOp::executeLoad` — recompute of off/sz matches the C++ (which also recomputes). `executeMultiequal` (emulateutil.cc:96): the `bl->getIn(i)==last_bl` in-edge search then `getIn(i)` — faithful; relies on the SSA invariant that MULTIEQUAL input order == parent block in-edge order (the same C++ assumption).
- `executeBranch`/`executeBranchind`/`executeCbranch`/`executeIndirect`/`executeStore`/`executeCpoolRef`/`executeNew`/`fallthruOp`/`setExecuteAddress`/`getVarnodeValue`/`setVarnodeValue`/`getSpaceFromConst` — all faithful.
- The `Emulate::execute_current_op` dispatch (emulate.rs:420, pre-existing W2) faithfully mirrors the C++ opcode switch (RETURN→executeBranchind, special/unary/binary trichotomy). The new `EmulateFunction` plugs into it correctly.

## Hunt list
- **Signedness**: `emulate_path` `i:int4` vs C++ `uint4` — benign (above). `get_load_image_value` `sz:int4` and `(sz as usize)<8` / `(8-sz as u32)*8` match C++ `int4 sz` vs `size_t` with the same in-contract `sz>=1`. `calc_mask(sz)` clamps a bad sz to index 8 (pre-existing). OK.
- **Integer width / wrapping**: load-image value is `u64`; the BE shift `res >>= (8-sz)*8` is on `u64` (logical), pinned by AT3. `address_to_byte(off,word_size)` uses the pre-existing helper. No arithmetic that can wrap in debug. OK.
- **Wrapping**: none introduced (the `wrapping_add` in the test fixture is test-only). OK.
- **Comparator totality**: none added (no new `operator<`/sort). OK.
- **Iteration-order provenance**: `varnode_map` is a `BTreeMap<VarnodeId,uintb>` (C++ `map<Varnode*,uintb>` keyed by pointer; the port keys by id — order-irrelevant here, it's a lookup table, never iterated for output). `loadpoints` is a `Vec` (insertion order == C++ `vector<LoadTable>`). NO HashMap/HashSet in the diff. OK.
- **Off-by-one / do-while / reverse iterators**: `emulate_path`'s `while(i>0){...; --i;}` keeps C++ semantics; the MULTIEQUAL `i-=1` is `i==0`-guarded. OK.
- **Erase-while-iterating**: none. OK.
- **Exception→Result partial-state parity**: the emulator only READS the syntax tree (immutable `&Funcdata` borrow); a mid-path `Err` leaves only `varnode_map`/`loadpoints` partially populated, which the (seamed) caller discards — matches C++ where a thrown emulatePath abandons the `EmulateFunction` instance. One divergence (LOSS, below): `emulate_path`'s `.map_err` over-broadly rewrites ALL errors; C++ catches only `DataUnavailError`.

## Special-casing audit — CLEAN
Grepped the production diff for function-name / address / case-value / jump-target hardcoding: **none**. The only hex literals are in the test fixtures (legitimate). No `if name==...`, no target/offset equality gates. PASS.

## Mechanical / dead-code findings
- **The 475-LOC `EmulateFunction` is DEAD on this branch**: `EmulateFunction::new` is never constructed anywhere (production or test); `emulate_path` and every `execute_*` body are unexercised. The only reachable surface is `Architecture::get_load_image_value` (what the porter's 5 tests + my 5 tests cover). The emulator's correctness rests on source inspection alone until the recovery seam wires it in.
- **Duplicate masking logic (reuse finding, not a bug)**: the `loadFill`+swap+mask sequence is now implemented THREE times — pre-existing `emulateutil.rs::EmulateSnippet::get_load_image_value` (via `construct_value`), new `sleigh.rs::read_loadimage_value`, new `seams.rs::get_load_image_value`. All three are logically equivalent to the C++; they should be consolidated to one helper to prevent silent drift. (Cross-pinned: my AT1/AT2 exercise the seams.rs copy at the sz==8 boundary.)
- **Two uncalled forwarders**: `architecture.rs::read_loadimage_value` (forwards to sleigh.rs) and `sleigh.rs::read_loadimage_value` have no callers — the emulator uses `seams.rs::get_load_image_value` instead. Dead.

## Divergences ledgered as LOSSes (not REJECT — fail-closed / cosmetic)
1. `executeSegmentOp` UNCONDITIONALLY errors ("Segment operand missing definition"); C++ executes a real `SegmentOp` when `glb->userops` defines one. Conservative (fails the emulation path → recovery falls back, never a wrong address), documented in the code. Segment-addressed switches are rare. → LOSS-162.
2. `emulate_path` `.map_err(|_| "Could not emulate address calculation at ...")` rewrites EVERY error; C++ `catch(DataUnavailError)` rewrites only that one and lets branch/segment `LowlevelError`s propagate with their own text. Ok/Err behavior is identical (caller only branches on success); only diagnostic text differs. → LOSS-162.

## Regression / parity (no executed code changed → inert)
- C++ oracle untouched: 207/207 + 675/675 PARITY OK.
- boolless byte-parity (`print_b5_boolless` 11/11), `printc_parity::corpus_functions_byte_compare_against_cpp_oracle` PASS, readstruct/condconst_conn covered by `verify_w10_dominant_copy`/`verify_w10_struct_corpus`/`verify_w10_stackslot_ssa` (3+6+16 PASS). The `Sleigh::loader` `RefCell→Rc<RefCell>` change is semantically transparent (all uses are `borrow()`/`borrow_mut()`; `set_loader` still replaces the inner Box in place so a shared handle stays current). The new `Architecture` fields are read only by the (uncalled) load-image path. No exec-failure drop, no new passing assertion, no regression.

## Adversarial tests (5, committed in the worktree)
`rust/crates/kuna-decomp/tests/verify_w10_jumptable_switch_adversarial.rs`:
- **AT1** sz==8 BE takes the mask (not shift) branch, full-width MSB-first read.
- **AT2** sz==8 LE full-width LSB-first read.
- **AT3** BE shift is LOGICAL not arithmetic (4-byte 0xFFFFFFFF → 0x0000_0000_FFFF_FFFF, no sign extension) — the wrap/shift hunt spot.
- **AT4** `getLoadImageValue` always `loadFill`s 8 bytes regardless of `sz`; partial read honors a non-zero offset, both endiannesses. (This one initially failed with a too-small fixture window, which CONFIRMED the always-8-wide read characteristic; fixed.)
- **AT_DEAD** tripwire: documents the emulator is unreached today and the load-image leaf is the only live foundation surface — the restoration anchor for when recoverModel/recoverAddresses lands.

## Verdict rationale
Faithful transcription of `EmulatePcodeOp`/`EmulateFunction` + load-image read; no special-casing; no fabrication (nothing claims recovery works — the seams are explicit); zero regression; C++ oracle and prior byte-parity intact. The single substantive shortfall is that the WAVE'S CAPABILITY (switch recovery) is not realized — the keystone (recoverModel/recoverAddresses/stageJumpTable/BlockSwitch/emitBlockSwitch) is still seamed and the ported emulator is dead/untested-by-execution. Per the project's accepted dormant-downstream precedent and the honest in-commit framing, this is **ACCEPT-WITH-LOSSES (LOSS-162)**, not REJECT.
