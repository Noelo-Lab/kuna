# Analysis — `test_decompiling_tee_O2_x2nrealloc::x2nrealloc`

angr testcase: `test_decompiling_tee_O2_x2nrealloc` (angr 9.2.213)
Binary: `binaries/tests/x86_64/decompiler/tee_O2`, function `x2nrealloc` @ `0x4057f0`.

## The gap (angr is better)

angr renders `x2nrealloc` as a clean, loop-free function with `xalloc_die(); /* do not
return */`. kuna emits an **invalid spurious `while(true)` loop** plus a `goto label_5813`,
extra stack-spill artifacts (`*(uint8 **)&v4[-8] = v6;`, the `0x5856`/`0x581e` return-address
stores), and is 38% longer (39 vs 24 loc, 1 goto / 1 label vs 0/0).

```
--- kuna (default) ---            --- angr ---
  v4 = PTRSUB(RSP,0);             v1 = *(ptr);
  while( true ) {                 if (a0) { ... if ((char)__CFADD__(...)) xalloc_die(); }
    ...                          else { ... }
    xalloc_die(v4[-0x20]);        ptr = xreallocarray();
    a2 = 1; v4 = v5; v6 = v7;     *(ptr) = v4;
  }                               return ptr;
  ... goto label_5813 ...
```

## Root cause (pinned)

`xalloc_die` is a **no-return** function: its body unconditionally ends in `error(...)` then
`abort()`, and kuna *already* knows `abort` is no-return (it decompiles `xalloc_die` with
`/* WARNING: Subroutine does not return */`). But kuna does **not propagate** that fact to
`xalloc_die` itself, so when decompiling `x2nrealloc` it assumes control returns from the
`call xalloc_die`. At -O2 the cold error path is laid out last, and the bytes after the call
are interpreted as a fall-through back-edge → the spurious loop + goto + the dead stack spills.

**Proof:** marking `xalloc_die` no-return by hand collapses everything to the angr shape:

```
$ decomp_dbg: load file … ; read symbols ; option noreturn xalloc_die ; load function x2nrealloc
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

(full side-by-side + this fixed output: `angr-vs-kuna.txt`.)

## Owning stage

S2 flow classification (`docs/stage-mapping.md`: "noreturn kills fall-through"). The no-return
flag is consulted in `s2_lift/flow.rs:1838` (`query_call_no_return(entry)` →
`function_is_no_return_across_scopes`) during initial flow generation — *before* the S3–S9
Action pipeline — and plants an `artificialHalt(noreturn)` that cuts the dead fall-through.

The flag is populated **pre-decompile** by the P0/analysis tier:
- `noreturn_known` (`kuna-analysis::s1_loader::noreturn`) — name list (`abort`, `exit`, …).
- `noreturn_disc` (`kuna-analysis::s1_noreturn_disc`) — evidence heuristic (≥3 call sites with
  no valid fall-through), requires `option listing on`. **Tested: does not fire for
  `xalloc_die`** (too few no-fallthrough call sites to meet the ≥3 threshold, and it does not
  seed propagation from the *Known* list).

angr's equivalent is its CFGFast call-graph **no-return propagation**: starting from a known
no-return set (`abort`/`exit`/…), any function whose body unconditionally reaches a no-return
call is itself no-return, propagated to a fixpoint — with no evidence threshold.

## Hypothesis for the kuna change

A new option-gated **structural no-return discovery** pass (analog of angr's CFGFast no-return
propagation): seed from the already-no-return set (Known + discovered) and mark any function
whose final reachable instruction is a `CALL` to a no-return callee (no valid fall-through),
iterated to a fixpoint — without the `noreturn_disc` ≥3-evidence gate. It emits the existing
`NoReturnFact` → the existing `set_function_no_return` commit seam → the existing
`flow.rs` artificial-halt path. No new commit arm, no S7 work.

## Scope / testability concern (feeds the decider)

This is inherently an **analysis-tier** feature: the no-return flag must be set on the callee
symbol *before* flow generation, and like `noreturn_known`/`noreturn_disc` it lives in
`kuna-analysis` and runs only on the **real-ELF `load file` path**. The XML `decomp_test_dbg`
bytechunk path used by `tests/stages/*.xml` never runs the analysis passes (nor builds the
Listing), so a *firing* two-pass `tests/stages/ghangr-…​.xml` (default-on auto-discovery fixing
the bug) **cannot be constructed** the way the worker template assumes — exactly why the
existing `noreturn_known`/`noreturn_disc` options have no stages XML test and are covered by
`kuna-analysis` unit tests + real-ELF instead. Combined with the cross-function call-graph
propagation and the cross-crate registration footprint, this pushes the feature toward the
**proposal (large)** fork. See `proposal.md` / the decider record in `record.json`.
