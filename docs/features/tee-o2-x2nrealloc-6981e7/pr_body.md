## `noreturn_propagate`: structural no-return propagation (angr CFGFast analog)

Closes the angr gap `test_decompiling_tee_O2_x2nrealloc::x2nrealloc` (coreutils
`tee_O2`, x86_64). Implements the approved
[proposal](../docs/features/tee-o2-x2nrealloc-6981e7/proposal.md); analysis in
[analysis.md](../docs/features/tee-o2-x2nrealloc-6981e7/analysis.md).

### The gap

angr renders `x2nrealloc` cleanly; kuna emits **invalid C** — a spurious
`while(true)` loop + `goto label_5813` + dead stack-spill artifacts, 38% longer
(39 vs 24 loc). **Root cause:** `xalloc_die` is structurally no-return (its body
unconditionally ends in `error(...)` then `abort()`, and kuna already models
`abort` no-return), but kuna never *propagates* that to `xalloc_die` — so the
`call xalloc_die` in `x2nrealloc` is assumed to return, and at `-O2` (cold error
path laid out last) the bytes after the call become a spurious fall-through
back-edge. Marking `xalloc_die` no-return by hand (`option noreturn xalloc_die`)
collapses the output to the angr shape, confirming the mechanism is exactly
no-return propagation.

### The mechanism

A new `kuna-analysis` Listing-consumer pass
(`s1_noreturn_propagate::NoReturnPropagatePass`, the kuna analog of angr's CFGFast
call-graph no-return propagation). It seeds the terminal set from the **Known**
no-return list and concludes a function no-return when its **last real
instruction** (last by address, skipping trailing NOP alignment padding) is a
`CALL`/tail `JMP` to an already-no-return callee, with **no `RETURN` path, no
computed jump, and no branch escaping the reachable body** — iterated to a
fixpoint, with **no evidence threshold**. This is the key difference from the
existing `noreturn_disc`, whose ≥3-call-site rule and "valid fall-through after the
call" predicate both miss `xalloc_die` (one cold call site; `call abort` followed
by valid NOP padding).

It emits the existing `NoReturnFact` → the existing `set_function_no_return` commit
seam → the inherited `flow.rs` artificial-halt path. **No new commit arm, no S7
work.** Soundness: with no `RETURN` and the only reachable exit a call/jump to an
already-no-return function, the function cannot return — strictly more conservative
than angr's propagation.

### The option (LLM-discoverable)

`option noreturn_propagate on` — **default-OFF opt-in**, requires `option listing
on` (it reads the Listing/xref tier). Provenance in `stages.toml`:
`source_decompiler="angr"`, `change_kind="structure-recovery"`,
`inspiration="test_decompiling_tee_O2_x2nrealloc; angr CFGFast no-return
propagation …; x2nrealloc"`.

```
kuna decompile tee_O2 x2nrealloc --option listing on --option noreturn_propagate on
```

### Why a proposal (large), and testing

The no-return flag is consumed pre-pipeline during initial flow generation
(`s2_lift/flow.rs:1838`, `&self`), so a `kuna_loweredswitch.rs`-style in-pipeline
Action runs too late — this must be an analysis-tier pass. And the mandated firing
`tests/stages/*.xml` cannot be authored: the XML `decomp_test_dbg` bytechunk path
never runs analysis passes or builds the Listing (exactly why
`noreturn_known`/`noreturn_disc` have no stages XML test). Coverage instead:

- **Cross-crate e2e** `verify_noreturn_propagate.rs` over a vendored **differential**
  fixture (`noreturn_propagate_x86_64`: a custom `my_die()` no-return wrapper called
  ONCE — below `noreturn_disc`'s ≥3 threshold — ending in `call abort` + NOP
  padding). Three assertions: propagation eliminates the dead code; the existing
  `noreturn_disc` consumer does **not** fix it (the differential proving a new
  capability); the wrapper itself is concluded no-return.
- **`kuna-analysis`** pass-identity unit tests.

### Ablation / parity / speed

- **Ablation 0/675** — the XML datatest path never runs analysis passes; `make
  test` stays **PARITY OK**. Kept **default-OFF** (real-ELF-only flow heuristic) —
  **no DIV entry**, default output byte-identical.
- **`make test-stages`** PARITY OK (the two `KUNA-CATALOG` provenance-count
  assertions re-pinned for the new angr/structure-recovery row; baseline
  regenerated).
- **`make rust-test`** green except the pre-existing `verify_w10_proto_unlock`
  failure (reproduces on the unmodified branch; unrelated to this change).
- **Speed** (`x2nrealloc`, repeat 5): off=154.6 ms / on=151.1 ms (−2.21%, within
  the 5% budget). The pass is one bounded call-graph fixpoint over an
  already-built Listing.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
