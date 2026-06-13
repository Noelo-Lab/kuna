# Verification: w6-s5-unionresolve (round 1)

**Verdict: ACCEPT_WITH_LOSSES**

- Item: `w6-s5-unionresolve`
- C++ in scope: `decompiler/cpp/unionresolve.cc`, `decompiler/cpp/unionresolve.hh`
- Branch / worktree: `worktree-wf_91be7ffe-767-8` @ `596cf3c` (port) + `5611de6` (verifier tests)
- Rust file: `rust/crates/kuna-decomp/src/unionresolve.rs` (1829 lines; the commit touches only this file)
- Verifier tests: `rust/crates/kuna-decomp/tests/verify_w6_s5_unionresolve.rs` (7 tests, all pass)

## Gate evidence

`cd rust && cargo test -p kuna-decomp`:
- lib (`unittests src/lib.rs`): **1032 passed, 0 failed** (includes the 17 `unionresolve::tests::*`)
- `verify_w6_s5_unionresolve`: **7 passed, 0 failed**
- all other suites: pass
- **One red:** `corpus_bootstrap::ldefs_pspec_cspec_parse_matrix` — `Could not find .sla file for x86:LE:64:default`. This is **pre-existing on the `rust-port` base** (the test exists at `corpus_bootstrap.rs:821` on base) and is **environmental** (needs a built SLEIGH `.sla` artifact via `make specs`). unionresolve is a pure value/scoring module that references no spec files, so it cannot have caused this. **Not a blocker for this item.**

`cargo clippy -p kuna-decomp` (library): clean, no warnings/errors. The verifier test file is clippy-clean after doc-list formatting fixes.

## What was ported, and is faithful (re-derived independently)

1. **`ResolvedUnion`** — all three C++ constructors (self / field+TypeFactory / copy), `update()` with the lock semantics, accessors. Pure value type over `Rc<Datatype>`. The `update` early-returns are exact: locked+differing-fieldNum -> no change; same-fieldNum + pointer-identical resolve -> no change; else adopt+report. The "same resolve" test is `Rc::ptr_eq` (C++ `resolve == op.resolve` pointer identity), correctly so.
2. **`ResolveEdge`** — both constructors and the **load-bearing total order** (`operator<` -> `Ord`: `typeId` u64, then `encoding` i32 **signed**, then `opTime` u32). Field widths match C++ exactly (`uint8`/`int4`/`uintm`). The signed `encoding` correctly orders the output edge (`slot == -1` -> encoding −1) before input edges. The address form stores `addr.getOffset()` (u64) into the u32 `opTime` as a **truncating** `as uintm` cast, and ignores `slot` (encoding constant `0x2000`) — both faithful and documented.
3. **The facet scoring matrices** — `score_locked_type`, `score_parameter`/`score_return_type`, `deref_pointer`, `score_truncation`, `score_constant_fit` (+ `looks_like_pointer`), `compute_best_index`, and the per-opcode `score_trial_{down,up}_pure` tables. **Mechanically diffed opcode coverage in both switches: zero missing, zero extra arms in each direction.** Spot-checked score values across CBRANCH/INT_SLESS/INT_LESS/INT_LEFT/SEGMENTOP/FLOAT_*/PIECE/INT_2COMP/comparison groups against C++ — all match, including the down-vs-up asymmetry of `FLOAT_TRUNC` and `FLOAT_INT2FLOAT`, and every `else { 0 }` fall-through maps to a C++ initialized-`score=0` default.

### Hunt-list findings (all clear)

- **signedness/ordering**: `ResolveEdge` encoding compared signed (correct for slot −1); `score_truncation` int branch `ct.get_size() as int8 >= vn_size as int8 + cur_off` is signed-int8 throughout, matching the C++ `int4`/`int8` promotion. Verified with a nonzero-offset window test (offset 4 inclusive boundary -> score 1, offset 5 -> overflow the int width -> null/−10 path).
- **width/truncation**: u64->u32 `opTime` truncation verified observable via `Eq`/`Ord` (high-bit-only offsets collapse; a larger full-u64 with smaller low-32 sorts first).
- **comparator totality**: `Ord`/`PartialOrd` are total (lexicographic on three orderable fields); `Eq`/`PartialEq` derived consistently.
- **iteration-order provenance**: `compute_best_index` uses strict `>` (`.skip(1)`), keeping the earliest maximum — verified including single-element, all-negative, and tie cases.
- **off-by-one**: `field_num = best_index - 1` (index 0 = whole union -> field_num −1); `i+1` field indexing matches the C++ `scores[fieldNum+1]` convention.
- **exception->Result parity**: missing-field/missing-ptrto/etc. map to `Err(KunaError::lowlevel(...))` where C++ would deref a null or read OOB; in the realistic domain these never fire.
- **aliasing/erase-while-iterating**: none — this module is value-typed and the iteration is over `&[int4]`/`&Vec`.
- **panicking casts/indexing**: `param_types[param_slot as usize]` is guarded by `num_params > param_slot` and only reachable with `param_slot >= 0` in the real driver (CALL params use `inslot-1 >= 0`); `i as int4` and `ct_out.as_ref().unwrap()` are bounded by loop conditions. Sound.

## Losses (recorded in MAIN-tree `docs/rust-port/losses.md`)

- **LOSS-087** — the entire `ScoreUnionFields` data-flow driver (`run`/`runOneLevel`/`newTrials*`/the three constructors + the recursing/live-varnode arms of `scoreTrial{Down,Up}`) is W4/W6 seamed; only the pure value types and score tables are ported. The scoring *decisions* are complete and tested; the *orchestration* (trial-list management, visited dedup, pass loop, constructor graph-walk) awaits the Funcdata/Varnode/PcodeOp + FuncProto/unionMap surfaces.
- **LOSS-088** — `ResolveCache` is a documented stub: `addResolution`/`inheritResolution` absent, `resolve` always returns the unresolved type (the C++ "no entry" path), pending the W6/W7 `unionMap`/`setUnionField`/`updateType` wiring. The `resolve` match logic itself is ported and correct.
- **LOSS-089** — two defensive `!= 0` div/modulo guards (`deref_pointer` ptr-size, `score_truncation` array align-size) diverge from the unguarded C++ `% size`; unreachable in the realistic domain (real type sizes/aligns are >= 1), value-identical whenever size >= 1.

## Why ACCEPT_WITH_LOSSES (no blocker)

No blocker or major stands. The ported surface (value types + total order + pure scoring matrices) is a faithful, mechanically-verified transcription with exact opcode/score parity and correct signedness/width/ordering handling. The unported surface is the IR-coupled data-flow driver + the Funcdata `unionMap` plumbing — genuine, well-documented W4/W6/W7 structural seams, each routed through frozen ported helpers so the later driver wires in without re-deriving scores. The single red gate test is pre-existing and environmental (missing `.sla`), unrelated to this module.
