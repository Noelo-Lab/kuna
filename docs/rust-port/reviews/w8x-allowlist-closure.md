# w8x-allowlist-closure
verdict: ACCEPT-WITH-LOSSES
verifier: claude-opus-4-8 (independent, no porter notes)
date: 2026-06-13
gate: `cd rust && cargo test -p kuna-decomp --test universalaction_listing` -> ok (1 passed); full `cargo test -p kuna-decomp --lib` -> ok (1575 passed); new `--test verify_w8x_allowlist` -> ok (9 passed). (`corpus_bootstrap::ldefs_pspec_cspec_parse_matrix` fails on a missing x86 `.sla` build artifact in the worktree — pre-existing, environmental, the diff touches no spec-loading code.)

## Focus criteria (REJECT iff any fails)

1. **Allowlist empty** — PASS. `UNPORTED_ALLOWLIST: &[UnportedEntry] = &[]` (universalaction.rs). The in-tree unit test `unported_allowlist_is_empty` and the B0 integration test both assert emptiness.
2. **B0 listing test asserts FULL byte-equality** — PASS. `tests/universalaction_listing.rs::decompile_tree_dump_is_byte_equal_to_oracle` now loads the raw oracle (`include_str!`) and does `rust_dump != oracle` verbatim — the old `strip_index`/`renumber`/allowlist-line-removal scaffolding is deleted (confirmed in the diff: the helpers and `use BTreeSet` are removed; comparison is against `oracle`, not `expected`). It is NOT weakened. The 10 formerly-allowlisted names all appear in the oracle fixture (lines 11,146,156,169,206,213-217), so the byte-equality is non-vacuous.
3. **Registration POSITION of each newly-wired pass matches C++ exactly** — PASS, verified position-by-position against `coreaction.cc ActionDatabase::universalAction`:
   - `lowerswitchinstall`: C++ between `ActionVarnodeProps`(5751) and `ActionHeritage`(5756) — Rust `varnodeprops`→`lowerswitchinstall`→`heritage`. ✓
   - `splitflow`: C++ between `RuleSubvarSubpiece`(5892) and `RulePtrFlow`(5894) — Rust `subvar_subpiece`→`splitflow`→`ptrflow`. ✓
   - `subfloat_convert`: C++ between `RuleFuncPtrEncoding`(5902) and `RuleFloatCast`(5904) — Rust `funcptrencoding`→`subfloat_convert`→`floatcast`. ✓
   - `stackprobeloop`: C++ right after `RuleDoubleOut`(5916), last oppool1 rule before `extra_pool_rules` — Rust `doubleout`→`stackprobeloop`→extend(extra). ✓
   - cleanup run (C++ 5976-5989): `2comp2sub`→`dumptyhumplate`→`subright`…`piecestructure`→`splitcopy`→`splitload`→`splitstore`→`stringcopy`→`stringstore`→`memsetcopy` — Rust reproduces this exact contiguous order. ✓
   All group strings match (`subvar`, `floatprecision`, `analysis`, `switchnorm`, `cleanup`, `splitcopy`, `splitpointer`, `constsequence`). The oracle byte-equality independently confirms every position.

## Mandatory hunt list
- Signedness: clean. subflow shift guards (`val < getSize(n)*8`) cast the `int4` product `as uintb` to reproduce the C++ `-Wno-sign-compare` `uintb < int4`; stackprobeloop `int4`/`uintb` mappings exact.
- Integer widths: clean. `uintb→u64`, `int4→i32`; `calc_mask(sz)` masking matches; the lone bare `as int4` (`point.size as int4`, kuna_stackprobeloop.rs:355) is a benign size-field width conversion matching the C++ `int4` param.
- Wrapping: clean. stackprobeloop `(limitoff - page) & calc_mask(sz)` → `limitoff.wrapping_sub(page)` (C++ unsigned wrap reproduced); subflow `remain_shift = val - getSize(0)*8` uses plain `-` and is proven non-underflowing by the preceding `val == getSize(0)*8` branch + `val < getSize(0)*8` early-return (so `val > getSize(0)*8`).
- Comparator totality: clean. No new comparators; `varmap`/`max_precision_map` BTreeMaps are lookup-only caches (C++ `map<PcodeOp*,int4>` is keyed by raw pointer and also never iterated, so the order choice is behaviorally inert).
- Iteration-order provenance: clean. `vn_descend`→`descend_snapshot` reproduces the C++ descend-list order (the established kuna_arraystride/kuna_loweredswitch idiom); no `HashMap`/`HashSet` anywhere in the ported code.
- Off-by-one / do-while / reverse iterators: clean — and one subtle spot verified faithful: `SubfloatFlow::traceForward`'s `op->getRepeatSlot(vn, slot, ourIter)` where C++ does `ourIter = iter; --ourIter` (iter was post-incremented) → the half-open range `[beginDescend, current)`; the Rust passes `&descend[..idx]`, exactly that prefix. `skip_copies` 0..8 bound matches the C++ `for(i=0;i<8;++i)`.
- Erase-while-iterating: clean. `trace_forward` iterates a `descend_snapshot` copy; the transform builds a separate placeholder graph (TransformManager) and never mutates the live descend list mid-loop — matching the C++ (which builds TransformVar/TransformOp and only commits in `apply()`).
- Exception→Result partial-state: clean. Every seam `Err(_) => 0` / `None => 0` bail leaves the live IR unchanged because discovery only populated the TransformManager placeholders; the C++ likewise mutates nothing until `apply()`, which never runs on the bail. RuleDumptyHumpLate (the one fully-firing non-trivial body besides stackprobeloop) maps its funcdata-mutation `KunaResult` to 0 only via `unwrap_or(0)`, and its mutations are `?`-free up to the point they run.

## Rule-body faithfulness (spot-checked against C++)
- `RuleSplitFlow::applyOp` (subflow.cc:2045): faithful; SUBPIECE-of-PIECE-via-INDIRECT/MULTIEQUAL detection transcribed exactly; `doTrace` (full SplitFlow engine: setReplacement/addOp/traceForward/traceBackward/processNextWork) is fully ported; `apply()` declines at the W6 `TransformManager::apply` seam → 0 (no change), byte-identical to the rule not firing.
- `RuleDumptyHumpLate::applyOp` (subflow.cc:3027): fully realized; the backtrack-through-PIECE loop, the three terminal cases (preserve-SUBPIECE / SUBPIECE→COPY / totalReplace), and `removeOp = op->getIn(0)->getDef()` ordering all match; the COPY flag word `unary|nocollapse` is transcribed verbatim (no seam on this body's emitted opcode).
- `RuleSplitCopy/Load/Store::applyOp` (subflow.cc:2962/2985/3006): faithful surface; decline at the `getTypeReadFacing`/`getTypeDefFacing`/`getValueDatatype` arch/W6 seam → 0.
- `RuleSubfloatConvert::applyOp` (subflow.cc:3504) + SubfloatFlow engine (maxPrecision stack-walk, exceedsPrecision, set/trace/process): transcribed verbatim; declines because `subfloat_float_format` (arch seam) reports no format → `do_trace` false → 0.
- `RuleStringCopy/StringStore::applyOp` (constseq.cc:981/1013): the only reachable guard (the `getIn(0)/getIn(2)->isConstant()` test) is ported faithfully; declines at the W4 symbol-table / W6 type-facing seam → 0.
- `RuleStackProbeLoop::applyOp` (kuna_stackprobeloop.cc:63): faithful line-for-line incl. `skipCopies`/`stackRelative` statics, the back-edge/entry identification, the descend-search for the loop-exit compare, and `(limitoff-page)&calc_mask`. Lone residual: the rewrite uses `op_set_opcode(TypeOp::new(CPUI_INT_ADD, 0, ...))` (flags=0) vs C++ `glb->inst[CPUI_INT_ADD]` — the W6 op-flags seam (opcode VALUE exact). Lossed (LOSS-120).
- `ActionLowerSwitchInstall` (coreaction.cc:5755): structure (gate / pre-SSA window / store lookup / per-record loop) ported; the `kunaInstallLoweredSwitch` CFG surgery declines at the W7/W4 seam → count untouched (LOSS-110).

## Mechanical pass
- grep for `todo!`/`unimplemented!`/`HashMap`/`HashSet`/`sort_unstable` in the five ported modules: none.
- `cargo clippy -p kuna-decomp` (lib): clean (exit 0). `--all-targets` fails on a PRE-EXISTING deny-level error at `heritage.rs:1602` (identical on the base `rust-port` branch — NOT in this diff) plus warnings in `unionresolve.rs`/`double.rs`/`kuna_regionid.rs` (none in this diff). This diff introduces one new `warn`-level lint only: `clippy::const_is_empty` on the now-empty `UNPORTED_ALLOWLIST.is_empty()` assertions (universalaction/tests.rs:50 and the integration test) — a benign, intended consequence of the empty allowlist, not a deny-level regression. Noted as a minor finding; not a blocker.

findings:
  - F1 (low, accepted as LOSS-120): `RuleStackProbeLoop` completes its `MULTIEQUAL→INT_ADD` rewrite with a flags=0 TypeOp instead of `glb->inst[CPUI_INT_ADD]` — the W6 op-flags seam, on the one now-wired body (gate default-on) that fully fires. No effect on the B0 dump; only reachable in a live decompile of a gcc stack-probe binary.
       cpp: decompiler/cpp/kuna_stackprobeloop.cc:140
       rust: rust/crates/kuna-decomp/src/kuna_stackprobeloop.rs (RuleStackProbeLoop::apply_op, op_set_opcode call)
  - F2 (low, accepted as LOSS-120): 9 of the 10 newly-wired passes decline at W4/W6/W7 seams (no-op where C++ rewrites) — owned by the pre-existing LOSS-088 (string rules), LOSS-110 (lowered-switch install), and the W6 transform/type-facing seam losses; consolidated forward into LOSS-120 now that the passes are live in the schedule.
       cpp: decompiler/cpp/subflow.cc:2045/2962/2985/3006/3504, decompiler/cpp/constseq.cc:981/1013, decompiler/cpp/coreaction.cc:5755
       rust: rust/crates/kuna-decomp/src/{subflow,constseq,kuna_loweredswitch}.rs
  - F3 (informational): this diff adds a `clippy::const_is_empty` warn-level lint via the now-empty allowlist assertions; intended and benign. `--all-targets` clippy was already failing on a pre-existing unrelated `heritage.rs:1602` deny-error.
       rust: rust/crates/kuna-decomp/src/universalaction/tests.rs:50

adversarial tests: rust/crates/kuna-decomp/tests/verify_w8x_allowlist.rs (committed, 9 tests) — `w8x_allowlist_is_empty`, `w8x_dump_is_byte_equal_to_raw_oracle`, `w8x_all_ten_formerly_allowlisted_passes_are_present`, `w8x_lowerswitchinstall_is_between_varnodeprops_and_heritage`, `w8x_splitflow_is_between_subvar_subpiece_and_ptrflow`, `w8x_subfloat_convert_is_between_funcptrencoding_and_floatcast`, `w8x_stackprobeloop_is_immediately_after_doubleout`, `w8x_cleanup_split_string_block_is_in_exact_cpp_order`, `w8x_dump_index_is_zero_padded_width4`. These independently pin the empty allowlist, raw byte-equality, every newly-wired pass's exact neighbour-adjacency (an order-sensitive `ib == ia+1` check a name-set test would miss), and the zero-padded index format — so a future weakening of the in-tree gate or a one-slot rule transposition fails review.

losses: LOSS-120 (appended; the runtime-degradation/W6-op-flags manifestation of the now-live passes), citing existing LOSS-088 / LOSS-110 / W6 transform+type-facing seam losses as the owning entries that closed LOSS-119.
