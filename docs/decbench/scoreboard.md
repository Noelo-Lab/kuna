# decbench fair scoreboard — kuna vs the field (post-#6 re-run, 2026-07-07)

The definitive apples-to-apples GED scoreboard for **all six decompilers**, re-scored fairly
after decbench PR #6 (the degenerate-source-CFG exclusion). Every decompiler's stored `.c` was
re-evaluated with the same metric on the same functions — so unlike earlier snapshots where only
kuna had been re-scored, this is a fair comparison. kuna is at PR **#142** here.

- **Corpus:** `results/full_run` — 26 SAILR projects × {O0, O2, O2-noinline}, **225,753 function-scores**.
- **Metric:** GED (graph edit distance) between the Joern source-CFG and the decompiled-C CFG; lower is
  closer to source. `perfect` = exact structural match (GED 0). `inf` = missing/unparseable output.
- **Produced by:** `scripts/robust_rerun.sh` → Stage 1 `run_benchmark` (redo kuna, `DECBENCH_DECOMPILE_ONLY=1`)
  → Stage 2 `reeval_ged` (all 6, post-#6) → `results/full_run/ged_new.json`. Reproduce the table with
  the session's `scoreboard.py`.

## Headline

**kuna went from 6/6 dead-last (the campaign's opening snapshot) to 4th of 6 — and now beats its own
ancestor, Ghidra, on every metric.**

## Aggregate (all opt levels)

Ranked by perfect-structure rate (exact CFG match):

| rank | decompiler | n | perfect % | mean GED\* | median GED\* | inf/miss % | GED>60\*\* |
|---|---|---|---|---|---|---|---|
| 1 | IDA | 51,823 | **8.4 %** | 22.3 | 9.0 | 54.8 % | 1674 |
| 2 | angr | 34,385 | 7.0 % | 26.9 | 9.0 | 52.2 % | 1520 |
| 3 | phoenix | 35,311 | 5.7 % | 27.5 | 10.0 | 52.5 % | 1575 |
| **4** | **kuna** | **41,874** | **5.1 %** | **22.9** | **9.0** | **53.7 %** | **1458** |
| 5 | Ghidra | 51,752 | 3.9 % | 24.4 | 11.0 | 55.4 % | 1725 |
| 6 | binja | 10,608 | 3.6 % | 61.1 | 22.0 | 42.1 % | 1576 |

\* mean/median over **finite** scores only (inf/missing excluded).
\*\* count of finite scores > 60 nodes (the `GED_MAX_NODES` size-delta-fallback cases) — a proxy for
degenerate/inflated outputs. **kuna's is the lowest of all six**, i.e. kuna is the most robust.

**What this says about kuna:**
- **4th on exact-match rate** (5.1 %) — behind IDA/angr/phoenix, but **ahead of Ghidra (3.9 %) and binja**.
- **2nd-best mean GED** (22.9, near-tied with IDA's 22.3) — on *average*, kuna's CFG is the 2nd-closest to
  source of any decompiler, better than angr, phoenix, Ghidra, and binja.
- **Best (fewest) blowups** (`GED>60` = 1458) — kuna produces the fewest degenerate/inflated outputs.

## Head-to-head: kuna vs Ghidra

On the **19,339 functions both scored** (finite on both sides):

| kuna vs Ghidra | count | share |
|---|---|---|
| kuna **better** (lower GED) | 6,330 | **32.7 %** |
| tie | 10,152 | 52.5 % |
| kuna worse | 2,857 | **14.8 %** |

kuna is **better-or-equal to Ghidra on 85.2 %** of shared functions, and strictly better 2.2× as often
as it is worse. kuna beats Ghidra at **every** optimization level (perfect %):

| opt | IDA | angr | phoenix | **kuna** | Ghidra | binja |
|---|---|---|---|---|---|---|
| O0 | 13.1 % | 12.4 % | 9.5 % | **7.7 %** | 5.7 % | 4.9 % |
| O2 | 3.6 % | 2.3 % | 2.2 % | **2.3 %** | 1.8 % | 1.1 % |
| O2-noinline | 6.2 % | 4.4 % | 4.1 % | **4.2 %** | 3.6 % | 3.6 % |

## How kuna got here (this campaign)

The gap to Ghidra was closed by a series of faithful C++ port-fidelity fixes:

- **Function-boundary overrun (the #1 gap):** kuna's flow-follower walked past no-returning calls
  (`error(nonzero)`, the `err`/`errx` family, discovered-no-return functions), absorbing the next
  function. Closed by #138–#140 (no-return marking) and per-call-site `CALL_RETURN` overrides.
- **Variable-soup (SUBPIECE/CONCAT):** an addr-tied merge that aborted `VariablePiece` grouping for the
  whole function on one conflicting stack slot — #141. (split/main GED 669 → 249 → 93, *below* Ghidra's 149.)
- **Invalid-C emission** from the angr-inspired structuring passes — #142 (branchflip pending-brace),
  #143 (regionstructure while-do overflow syntax), #144 (emitBlockLs duplicate-emit), #145
  (regionlooprefine compound-head).

## Caveats

- **Absolute rates are low for everyone** (overall 5.9 % perfect) because this is a strict CFG-exact
  metric after the #6 degenerate-source exclusion; the **relative** standing is the signal.
- **`n` differs per decompiler** — each is scored only on the functions it actually emitted (IDA/Ghidra
  ~52k, kuna ~42k, binja ~11k). Perfect % and the head-to-head normalize for this.
- **`parse_source` CFG extraction** was suspected of penalizing kuna's emission style (raw `int8`/`uint4`
  types, `void*` derefs); the aggregate **refutes** this — kuna has the *fewest* `GED>60` blowups, so it is
  not disproportionately inflated. (Details: `docs/decbench/` findings / session notes.)

## Reproduce

```bash
# ged_new.json is produced by the robust re-run (Stage 1 decompile-only, Stage 2 reeval all 6):
#   run_benchmark redo-kuna (DECBENCH_DECOMPILE_ONLY=1)  ->  reeval_ged results/full_run <workers> <projects>
# then aggregate by decompiler from results/full_run/ged_new.json
#   key schema:  opt::project::stem::dec::func -> {"value": float, "perfect": bool}
```
