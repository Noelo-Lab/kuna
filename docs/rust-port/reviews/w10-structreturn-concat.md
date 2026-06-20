# w10-structreturn-concat
verdict: ACCEPT-WITH-LOSSES
verifier: independent (Opus 4.8, round 1)
date: 2026-06-16
branch: rport/w10-structreturn-concat @ dd7f985 (base/parent ed09d28 = rust-port tip incl. mergepiece-dynsym)

## What the branch actually does (vs the prompt premise)
The prompt anticipated edits to `subflow.rs`/`merge.rs`/`printc.rs` (a SplitDatatype
graph-mutation cluster). The branch takes a DIFFERENT, smaller route: a 16-line
**type-seed** in `coreaction_infertypes.rs::input_type_local` — a faithful port of
`TypeOpReturn::getInputLocal` (typeop.cc:903-921). It flows the function's recovered
output data-type onto a RETURN's value Varnode so the PRE-EXISTING, already-reviewed
`RulePieceStructure`/`getStructuredType` splitter (ruleaction_6.rs/dtype.rs, UNTOUCHED)
sees a structured type on the CONCAT root and splits the forming PIECE tree. Diff =
EXACTLY 2 files, both under rust/crates/kuna-decomp/: `src/coreaction_infertypes.rs`
(+16) and the verifier test (+281). It does NOT touch subflow.rs/merge.rs/printc.rs or
any reserved file. Smaller-than-expected, but legitimate and verified.

## Gate (run fresh, shared built .sla)
- C++ oracle (rust-only diff -> definitionally untouched): `kuna.run_tests --all
  --baseline docs/baseline.json` -> 207/207 unit + 675/675 datatest, **PARITY OK**.
  Zero non-rust files changed (decompiler/ specs/ docs/baseline.json byte-identical).
- Rust-engine datatests (KUNA_ENGINE=rust, debug): base ed09d28 = **331/675**,
  branch = **332/675**. FULL passing-name-set diff: **+1 / -0, REGRESSED SET EMPTY**.
  The sole flip is `Concat split #7` (fail->pass): the `min=0 max=0 CONCAT`
  suppression assertion in concatsplit.xml. Per-field writes #1-#6 still FAIL on
  BOTH base and branch (the naming half is seamed) -> not a regression.
- `cargo test --workspace` -> all green, 0 failed (incl. the 3 verifier tests).
- `cargo test -p kuna-decomp` -> 2746 passed / 0 failed.
- `cargo clippy -p kuna-decomp --lib` -> clean (no warnings).
- B0 / pass order: the change is a type-inference SEED body; it touches no
  pass-ordering file. `corpus_bootstrap::ldefs_pspec_cspec_parse_matrix` passes on
  the branch worktree.

## Byte-identity guards (direct print-C diff, base binary vs branch binary, same inputs)
- boolless (BIG-ENDIAN): BYTE-IDENTICAL. The seed arm is gated on a non-VOID output
  type whose size == the returned Varnode size; never fires on a non-struct-return BE
  function. Confirmed inert.
- concat / retstruct / packstructaccess / piecestruct: BYTE-IDENTICAL.
- condconst / condconst2 / condconstsub: BYTE-IDENTICAL.
- concatsplit: the ONLY differing render -> `Concat split #7` gains (CONCAT removed);
  split-OFF (`option splitdatatype`) RESTORES the inline `CONCAT(B,A)` (proven by the
  verifier toggle test) -> the split is rule-driven, not name-keyed.
- The 31 switch datatests (If/Switch, Switch Hide/Indirect/Loop/Multi/return): fail
  set BYTE-IDENTICAL base vs branch (pre-existing W7 region gap, unperturbed).

## REAL parity, not fabricated (inspected live render)
- concatsplit `test_split` body renders genuine assignment statements with NO
  `CONCAT(` anywhere (`*(mystruct **)&v2->a = out; v2->c = A; ... return v2;`).
- concat `concatreturn` body splits to `dat_4 = retp2; v1 = retp1; return v2;` — no
  inline CONCAT, real per-piece writes. Naming is synthetic (`dat_N`/`vN`) because the
  proto-partial naming phase is seamed; the SPLIT is real.
- The C++ B5 oracle renders all 7 concatsplit assertions (Success x7); the branch now
  matches #7 (suppression) — same direction as the oracle.

## Faithfulness (line-for-line vs typeop.cc:903-921)
- `if(slot==0) return generic` == Rust `slot >= 1` entry gate; slot 0 falls to the
  generic `get_input_local`. Exact.
- `bb==0 -> generic`: implicit. A live RETURN op always has a parent; in C++
  `bb->getFuncdata()` is the SAME funcdata being inferred, which is the Rust `data`.
  Equivalent.
- `fp->getOutputType()` == `get_func_proto().get_output_type()` (= store->getOutput()
  ->getType(), NO `isOutputLocked()` gate — the C++ lock check is commented out; Rust
  matches by not gating on lock). Exact.
- `ct->getMetatype()==TYPE_VOID || ct->getSize()!=in.size -> generic` == Rust
  `get_metatype()!=TYPE_VOID && get_size()==in_size -> return ct`, else fall through.
  Exact (both sides `int4` size compare).
- `return ct` == `Rc::clone(ct)`. Exact.
- ADDED `has_store()` guard: a faithful merged-tree ADAPTATION. C++ `getOutputType()`
  unconditionally derefs `store`; Rust would panic when the W4 ProtoStoreSymbol seam
  left no store. The guard falls through to the generic UNKNOWN default — the
  CONSERVATIVE direction (no over-firing), never a divergence on the corpus (the seed
  empirically fires on concat/concatsplit, which DO have a store at this stage).

## Mandatory hunt list
- Signedness: clean. Sole compare `ct.get_size()==in_size` is `int4==int4`
  (Datatype::get_size and Varnode::get_size both -> int4), matching C++
  `getSize()!=getSize()` (both int4).
- Integer widths: clean (no arithmetic, no width conversion).
- Wrapping: clean (no arithmetic on uintb-lineage values).
- Comparator totality: N/A (no operator<).
- Iteration order: N/A (no loop, no container in the added code; no HashMap/HashSet).
- Off-by-one / do-while / reverse iters: N/A.
- Erase-while-iterating: N/A (no mutation).
- Exception->Result: the C++ never null-derefs store; the Rust `has_store()` +
  `if let Some(ct)` fall-through is the only Option seam, conservative.

## Mechanical pass
- `todo!`/`unimplemented!`/`HashMap`/`HashSet`/`sort_unstable`/bare `as`/`unwrap`/
  `panic!`/`wrapping_*` in the added production lines: NONE.
- clippy -p kuna-decomp --lib: clean.

## NO special-casing
- Production added non-comment lines: ZERO struct-name / field-name / magic-offset /
  function-name literals (grep empty; the only `foo`/`mystruct` tokens are doc-comment
  examples). The actual split runs through the PRE-EXISTING, generic
  `getStructuredType`/`get_exact_piece`/`get_component`/`RulePieceStructure` path
  (ruleaction_6.rs/dtype.rs), which this branch does NOT modify -> datatype-generic by
  construction.
- Verifier test file: the lone name literal `"concatreturn"` SELECTS which function to
  inspect in the dump (then asserts on its STRUCTURE) — it does not drive the split.

## Reserved-file / scope check
- NONE of the reserved files touched: NOT coreaction_cleanup.rs, coreaction_render.rs,
  funcdata.rs, architecture.rs, funcdata_merge.rs, variable.rs, database.rs,
  decompile_drive.rs, ifacedecomp.rs, printc.rs, subflow.rs, merge.rs. Diff = ONLY
  coreaction_infertypes.rs + the verifier test.

## findings
- F1 (loss, accepted as LOSS-213): the per-field NAMING half (concatsplit #1-#6,
  `out->a = A;` ... `out->e = E;`) is NOT delivered — the recovered field names and
  call-argument binding come from `Funcdata::linkProtoPartial` / the proto-partial
  naming phase in the RESERVED mergepiece+printc plane. The render shows the correct
  SPLIT geometry with synthetic names (`dat_N`/`vN`) instead of `out->field`.
  NON-DECISIVE, not a divergence; the C++ oracle renders all 7, the branch matches #7.
       cpp: decompiler/cpp/typeop.cc:903 (seed) + funcdata_merge linkProtoPartial naming
       rust: rust/crates/kuna-decomp/src/coreaction_infertypes.rs:105-111 (seed lands;
             naming phase deferred to reserved files)

## adversarial tests (committed to the branch, dd7f985)
- `register_returned_struct_stops_inlining_concat` — `concatreturn` (x86:LE:64) struct
  return no longer inlines a CONCAT; body emits >=2 piece assignments + plain
  `return <var>;` (structural key, no field-name key).
- `split_off_restores_unsplit_concat_form_generically` — `option splitdatatype` (off)
  RESTORES inline `CONCAT(B,A)` on concatsplit (split is rule-driven, name-agnostic);
  on/off renders genuinely DIFFER.
- `boolless_big_endian_render_is_unperturbed_by_return_seed` — BE non-struct function
  gains no spurious `.member =` write; split-off toggle is byte-inert on boolless.
All 3 ran the REAL dump path (no SKIP) and pass; each SKIPs (never false-passes) when
the binary/specs are absent.

## losses
LOSS-213 (per-field proto-partial naming/argument-binding deferred to reserved
mergepiece+printc files). Faithful, generic, datatype-driven TYPE-SEED; +1 real
datatest (CONCAT suppression); ZERO corpus regression -> ACCEPT-WITH-LOSSES.
