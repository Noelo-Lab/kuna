# [PROPOSAL] Structural no-return propagation (`noreturn_propagate`)

Closes the angr gap `test_decompiling_tee_O2_x2nrealloc::x2nrealloc`.

## The problem

angr renders `x2nrealloc` (coreutils `tee_O2`, x86_64 @ `0x4057f0`) as a clean, loop-free
function. kuna emits **invalid C**: a spurious `while(true)` loop, a `goto label_5813`, dead
stack-spill artifacts (`*(uint8 **)&v4[-8] = v6;`, the `0x5856`/`0x581e` return-address
stores), 38% longer (39 vs 24 loc; 1 goto + 1 label vs 0 + 0).

**Root cause (confirmed):** `xalloc_die` is no-return — its body unconditionally ends in
`error(...)` then `abort()`, and kuna already models `abort` as no-return (it decompiles
`xalloc_die` with `/* WARNING: Subroutine does not return */`). But kuna never *propagates*
that to `xalloc_die`, so the `call xalloc_die` in `x2nrealloc` is assumed to return. At -O2
the cold error path is laid out last, and the bytes after the call become a spurious
fall-through back-edge → the loop / goto / dead spills.

**Proof the mechanism is exactly no-return:** issuing `option noreturn xalloc_die` (after
`read symbols`) on the real ELF collapses the output to precisely the angr shape (see
`analysis.md` and `angr-vs-kuna.txt`):

```
void x2nrealloc(void *a0,uint8 *a1,uint8 a2) {
  uint8 n; ...
  n = *a1;
  if (a0 == (void *)0x0) { if (n == 0) { ... } }
  else { ... if (v3) { /* WARNING: Subroutine does not return */ xalloc_die(); } }
  xreallocarray(a0,n,a2);
  *a1 = n;
  return;
}
```

## The angr reference pass

angr's `CFGFast` no-return analysis (the `returning` / `_iteratively_analyze_function_features`
call-graph propagation): starting from a known no-return set (`abort`/`exit`/`__assert_fail`/…),
any function whose body unconditionally reaches a no-return call is itself no-return, propagated
to a fixpoint over the call graph — with **no evidence threshold**.

## Why this is not a one-pass `kuna_<slug>.rs` (the large-scope ruling)

A decider subagent ruled `scope: large`. Three independent reasons (any one suffices):

1. **The seam is pre-pipeline and `&self`.** The no-return flag is consumed during *initial
   flow generation* at `s2_lift/flow.rs:1838` (`query_call_no_return(entry)` →
   `symboltab.function_is_no_return_across_scopes`), *before* the S3–S9 Action/Rule pipeline.
   A `kuna_loweredswitch.rs`-style Action runs too late, and `query_call_no_return` is `&self`
   so it cannot build/analyze a callee `Funcdata`. The flag must be set on the callee symbol
   **pre-decompile** — i.e. in the analysis tier, not as an in-pipeline Rule. This is a new
   pass *type*, not one Action/Rule.

2. **The mandated firing `tests/stages/*.xml` cannot be authored.** Analysis passes and the
   Listing tier run **only on the real-ELF `load file` path**. The XML `decomp_test_dbg`
   bytechunk path that `tests/stages/*.xml` uses never runs analysis passes or builds the
   Listing — which is exactly why the existing `noreturn_known`/`noreturn_disc` options have
   *no* stages XML test (covered by kuna-analysis unit tests + real-ELF instead). A two-pass
   stages test could only exercise the pre-existing **manual** `option noreturn <name>`, which
   is not the proposed auto-discovery feature.

3. **Cross-crate footprint:** new `kuna-analysis` module + registration in
   `kuna-analysis` `lib.rs`/`passes.rs`, plus the option flag/arm in `kuna-decomp`
   `architecture.rs`, the name in `kuna-decomp` `options.rs`, and a `stages.toml` row — more
   than the single-module small shape.

## Proposed implementation plan

Option name: **`noreturn_propagate`** (default-OFF opt-in; real-ELF path only ⇒ parity oracle
byte-identical). A new Listing-consumer analysis pass, modeled on `s1_noreturn_disc`:

1. **New module** `decompiler/crates/kuna-analysis/src/s1_noreturn_propagate/mod.rs`
   (`NoReturnPropagatePass`):
   - Inputs (existing): `AnalysisCtx::listing` (the `--option listing on` recursive-descent
     Listing with funcsym names + Known/call-fixup seed metadata), and the already-no-return
     seed set (Known + call-fixup + anything `noreturn_disc` found).
   - Algorithm: worklist fixpoint. A function `F` is no-return if its **last reachable
     instruction is a `CALL` (or tail `JMP`) to a callee already in the no-return set, with no
     valid fall-through after it** (reuse `s1_noreturn_disc`'s `is_instruction_start` /
     `is_data` / `function_at` fall-through predicate). Seed from the **Known** set (this is the
     key difference from `noreturn_disc`, which only seeds from ≥3-evidence discoveries). Mark,
     add to the set, re-evaluate callers. Bounded sweeps (callgraph depth; converges quickly).
   - Output: one `NoReturnFact` per newly-marked function → the existing `commit_analysis_output`
     arm → `set_function_no_return`. **No new commit arm, no S7 work.**
2. **Register** as a `listing_consumer_passes()` entry (`kuna-analysis/src/passes.rs`), gated by
   the new flag at commit time; `pub mod` in `kuna-analysis/src/lib.rs`.
3. **Flag + option arm** in `kuna-decomp/src/infra/architecture.rs`
   (`analysis_noreturn_propagate`, default-off in reset, `set_kuna_option` arm), name in
   `KUNA_OPTION_NAMES` (`kuna-decomp/src/p0_knowledge/options.rs`), and a fully-populated
   `settableTable` row in `stages.toml` (`source_decompiler="angr"`,
   `inspiration="test_decompiling_tee_O2_x2nrealloc; angr CFGFast no-return propagation; x2nrealloc"`,
   `change_kind="structure-recovery"`, requires `option listing on`).

## Testing plan (no stages XML)

- **kuna-analysis unit tests**: a synthetic Listing with `wrapper -> abort` (one call site, no
  fall-through) asserting `wrapper` is emitted as a `NoReturnFact` only with the flag on;
  wrapper-of-wrapper depth-2 fixpoint; negative cases (callee returns; conditional call).
- **Real-ELF regression**: a checked-in helper/test that runs
  `option listing on; option noreturn_propagate on; load function x2nrealloc` on `tee_O2` and
  asserts the loop/goto are gone (the `angr-vs-kuna.txt` "fixed" rendering). This is the
  end-to-end binary→func testcase in lieu of a stages XML.

## Speed / risk

- **Speed:** the pass is a bounded call-graph fixpoint over the already-built Listing; cost is
  ~one extra Listing sweep, only when `listing on` + `noreturn_propagate on`. Default run is
  untouched (pass not registered/active). Measure on `x2nrealloc` off-vs-on at implementation.
- **Risk:** a no-return heuristic can be wrong (a callee that *looks* terminal but returns via
  a tail-`jmp` to a returning function) → it would wrongly delete the fall-through. Mitigated
  by: default-OFF opt-in, real-ELF-only (parity byte-identical), and the strict "last reachable
  insn is a call/tailjmp to an *already*-no-return callee with no valid fall-through" predicate
  (seeded only from Known/confirmed no-return, not guesses). This is strictly *more*
  conservative than angr's propagation, which also tail-recurses from its known set.
- **Generality:** benefits any binary with a no-return wrapper the Known name list misses
  (`xalloc_die`, `verror`-style diers, custom `die()`/`fatal()`), independent of the ≥3 call
  sites `noreturn_disc` requires.

## Proposed option name

`noreturn_propagate` — default-OFF opt-in, requires `option listing on`.
