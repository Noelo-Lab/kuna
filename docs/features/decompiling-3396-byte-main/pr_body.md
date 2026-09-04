## What was broken

> **Decompiling the 3396-byte main function takes about 68 seconds** (major, `69a3822f7b3cc38c80464da4`, 1 instance)
> The command produced no output for roughly 68 seconds, then emitted about 30 KB of highly noisy pseudocode.

This is **attempt 3** on `decompiling-3396-byte-main`. Attempts 1 and 2 took the
witness 71.46 s → 19.42 s (#380, the dead-list scan) → 14.44 s (#385, following the
function's flow once instead of twice), and both closed with the same finding: the
residue is flat. This attempt confirms that again — and then finds that "flat" did
not mean "nothing left", it meant *four* separate costs of a few percent each,
three of them re-derivations of facts upstream keeps cached.

## Mechanism

Four changes, none of which can alter emitted C:

1. **`Merge::merge_test_adjacent` stops re-deriving the isolated bit per member.**
   C++ reads `high->getSymbol()->isIsolated()` off a cached pointer. kuna's merged
   tree does not paint SymbolEntries onto Varnodes before the merge group, so
   `bank_symbol_isolated` re-derived the binding with a `findContainer` containment
   query **per member Varnode, per candidate pair**. On this witness that is
   **26,243,952 queries** in one decompile. `set_symbol_isolated` is the only route
   the `ISOLATE` dispflag has into a function-local scope, so `ScopeLocal` now
   records whether it has ever been called; a scope that has not cannot answer
   `true` from any of those queries and the scan is skipped outright.
   Measured: **1,331 ms → 3 ms.**
2. **`bank_symbol` answers from the high's cached flag word first.** The same scan
   shape: it walks every member looking for an address-tied one, and a high's
   cached flags are the OR of its members', so a clean `addrtied == false` settles
   it (`variable.rs (kuna_addr_tied_if_clean)`). `mergeTestRequired` reads
   `high_is_addr_tied` on both highs before it gets here, so the word is clean by
   construction. The surviving scan also stopped building a `LinkEntryInfo` (a
   display-name `String` and a data-type clone) to read two integers out of it.
3. **The rule-pool op cursor reads a run of successors per tree descent.** The C++
   `op_state` is a map iterator whose `++` is O(1); kuna re-derives the position
   from a `SeqNum`, which is an optree range search *per op per rule pass* — about
   13.2 M searches on this function. The op bank now counts its own optree
   insertions and removals in an epoch, and the pool buffers 64 successors from one
   descent, discarding them the moment the epoch moves. The pool's own destroy of
   the op it just left is the one mutation that does not invalidate the run (it
   removes a key that sorts before every buffered entry), and it is acknowledged
   explicitly rather than assumed away.
4. **`ActionDeadCode`'s clear loop does one arena lookup per Varnode, not three.**
   It runs over every Varnode in the function on every pass, 41 passes here.

## Measurement

Interleaved paired A/B — one loop alternating the arms so both see the same machine
load (sibling builders were running throughout). Both arms are release builds of
this worktree; the base arm is `origin/main` at the recorded base commit.

| pair | base (ms) | new (ms) | delta |
|---|---|---|---|
| 1 | 14486 | 12191 | −15.8% |
| 2 | 14712 | 12176 | −17.2% |
| 3 | 13978 | 12048 | −13.8% |
| 4 | 14644 | 12234 | −16.5% |

**median 14,565 ms → 12,184 ms, −16.3%** (min −13.8%, max −17.2%, paired mean
−15.8% ± 1.5, ≈21σ). Every pair is a win and the spread is a tenth of the effect.

Re-measured **after the rebase onto `origin/main` cb7f30d1** (a sibling's
`ptrdepthcap` option landed in between), both arms rebuilt from the rebased tree,
with `make rust-test` running alongside: 4 pairs, base 14579/14939/14981/14544,
new 12329/12032/14132/12416 → **median 14,760 → 12,373 ms, −16.2%**. Three pairs
land at −14/−15/−19%; the fourth is −5% and is the one that ran against the peak of
the workspace suite. `scripts.repipe.verify` on the merged-tree build measures
**12,437 ms**.

**Output is byte-identical.** `kuna decompile` stdout *and* exit code compared
between the two arms over 297 functions across 38 binaries pre-rebase (and 143 more over 17 binaries re-swept after the rebase) and 8
architecture/format combinations (x86-64, i386, ARM Thumb, Cortex-M, RISC-V 64,
MIPS32/MIPS16, PPC; ELF exe/PIE/.so, PE32/PE32+, COFF .obj, Mach-O .o, plus the
witness crackme itself): **0 diffs**. Plus 675 datatest assertions and the stage
corpus, unchanged.

## The acceptance probe

**Still FAILS, and the need stays open.** `a-53d616afcb6a` asks for a median under
10,000 ms; `scripts.repipe.verify` on this build measures **12,437 ms**. `exit_code` passes, `wall_ms` does not. The
probe is **not promoted** into `tests/cli/` — that would commit a red test — and it
is not relaxed. Attempt 3 is a measured, output-identical −16%; it is not a close.

What attempt 4 should inherit, from a fresh instrumented profile of *this* build:
`stage_jump_table` 31% (2 tables, and the per-table re-clone is load-bearing for
`option unrolledguard`), heritage ~23%, `oppool1` ~14%, `ActionDeadCode` ~14%,
merge now ~6%. And one lead is **refuted**: `Heritage::guard_calls` looks like a
prototype-query hot spot, but instrumenting its three model queries measures
`has_effect` ≈ 30 ms, `characterize_as_output` 19.7 ms and
`characterize_as_input_param` 9.8 ms inside a 971 ms loop — the cost is INDIRECT op
and Varnode *construction* (arena + two BTreeMap inserts each), which is the IR
growth model, not a lookup to memoize.

## Gates

| gate | result |
|---|---|
| `make test` | PARITY OK — 675/675, `docs/baseline.json` untouched |
| `make test-stages` | PARITY OK, `docs/baseline-stages.json` untouched |
| `make rust-test` | green — 348 test binaries, 0 failed (rebased tree, `env -u KUNA_DECOMP_TEST`) |
| `make check-spec` | check-spec OK |
| `make test-cli` | tests/cli: 21/21 passed |
| `kuna catalog --check` | catalog OK — no option added |
| `repipe.mergecheck` | merge guards clean — 0 rejects against `origin/main`; `counters` reports no drift on the rebuilt rebased tree |

No `phases.toml` row, no `options.rs` registration, no catalog counters, no DIV row:
the change cannot alter emitted C and is verified not to.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
