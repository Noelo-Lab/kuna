---
case_id: O0-coreutils-factor-factor
pool: angr
status: already-fixed
tier: S
margin: 12
fresh_verdict: today's no-flag run (auto -> aggressive, 92 KiB binary) already emits the source's early-return shape; the 12-point gap is closed by the `returndup` pass that `aggressive` turns on
option_closing: returndup
feature_slug: null
scope: none
confidence: high
---

## Side-by-side

Recorded: angr GED=0, kuna GED=12 (ghidra 12, ida 17, binja 3, phoenix 0). Source CFG
13 nodes / 14 edges, so the GED is a real edit distance, not the >60-node approximation.

angr (stored, GED 0) — four guards, three of them early returns:

```c
void factor(unsigned long long *a0, unsigned long long a1, struct_0 *idx)
{
    v1 = a0;
    v0 = a1;
    idx->field_fa = 0;
    idx->field_8 = 0;
    if (!v1 && v0 <= 1)
        return;
    v0 = sub_4037c3(&v1, v1, v0, idx);
    if (!v1 && v0 <= 1)
        return;
    if ((char)sub_404d2e(v1, v0))      { sub_4033a7(idx, v1, v0); return; }
    else if (!v1)                      { sub_405a75(v0, 1, idx, ...); return; }
    else                               { sub_406110(v1, v0, 1, idx, v6); return; }
}
```

kuna (stored, 2026-07-27 benchmark run = `reliable` + `listing on`) — the two source
early returns are fused into ONE short-circuit condition with a comma expression, so the
whole body sits one level deeper and two CFG nodes vanish:

```c
  if (((a0 != 0) || (2 <= a1)) && ((v3 = a0, v2 = sub_37c3(&v3,a0,a1,a2), v3 != 0 || (2 <= v2)))) {
    v1 = sub_4d2e(v3,v2);
    if (v1 != '\0') { sub_33a7(a2,v3,v2); }
    else { if (v3 != 0) { sub_6110(v3,v2,1,a2); } else { sub_5a75(v2,1,a2); } }
  }
  return;
```

kuna (fresh, today's build, NO flags — `auto` resolves to `aggressive` at 92,544 bytes):

```c
void factor(int8 a0,uint8 a1,int8 a2) // return-dupe x4
{
  *(void *)(a2 + 0xfa) = 0;
  *(void *)(a2 + 8) = 0;
  if ((!a0) && (a1 <= 1))
    return;
  v3 = a0;
  v2 = sub_37c3(&v3,a0,a1,a2);
  if ((!v3) && (v2 <= 1))
    return;
  v1 = sub_4d2e(v3,v2);
  if (v1) { sub_33a7(a2,v3,v2); return; }
  if (!v3) { sub_5a75(v2,1,a2); return; }
  sub_6110(v3,v2,1,a2);
  return;
}
```

That is angr's CFG exactly (4 ifs, 0 gotos, per-guard early returns, `else` flattened by
the default-on `ifelseflatten`), and it is the source's CFG. kuna's callee argument
recovery is in fact cleaner than angr's here (`sub_5a75(v2,1,a2)` = 3 args, matching the
source; angr invents `v3,v4,v5`).

Ablations run today (all on the stripped binary, `--addr 0x718c`):

| run | shape |
|---|---|
| no flags (= `auto` = `aggressive` here) | source shape, 4 ifs, early returns |
| `--mode reliable` | merged short-circuit `if ((A\|\|B) && (v3=a0, v2=f(..), C\|\|D))` — the stored gap |
| `--mode reliable --option returndup on` | source shape (identical to the no-flag run) |
| `--mode aggressive --option returndup off` | back to the merged short-circuit gap |

So `returndup` is the sole lever, and `aggressive` is what turns it on.

## Source

`~/github/decbench/results/full_run/O0/coreutils/compiled/factor.i:28022` (src/factor.c:2219):

```c
static void
factor (uintmax_t t1, uintmax_t t0, struct factors *factors)
{
  factors->nfactors = 0;
  factors->plarge[1] = 0;

  if (t1 == 0 && t0 < 2)
    return;

  t0 = factor_using_division (&t1, t1, t0, factors);

  if (t1 == 0 && t0 < 2)
    return;

  if (prime2_p (t1, t0))
    factor_insert_large (factors, t1, t0);
  else
    {
      if (t1 == 0)
        factor_using_pollard_rho (t0, 1, factors);
      else
        factor_using_pollard_rho2 (t1, t0, 1, factors);
    }
}
```

Two guard clauses that `return` early, then a two-level if/else. Today's default output
is a line-for-line structural match.

## Analysis

**Structural symptom (as it stood at benchmark time):** the two source early-return
guards, which gcc -O0 compiles to two conditional branches into ONE shared bare-epilogue
RETURN block, were emitted as a single merged short-circuit condition with an embedded
comma expression — `if (((a0 != 0) || (2 <= a1)) && ((v3 = a0, v2 = sub_37c3(...), v3 != 0
|| (2 <= v2)))) { ... }` — instead of two `if (...) return;` statements followed by
de-indented body.

**Root cause:** P8 (structure). `CollapseStructure::rule_block_or` fuses guards that share
an out-target, and the shared out-target here is the single epilogue RETURN block; the
existing `ActionReturnSplit` (the goto-driven ReturnDuplicatorLow analog) only splits when
structuring already left a goto into the return, which a clean guard collapse never does.
The fix is the gotoless complement: duplicate the shared const-return epilogue into each
predecessor before final structuring (angr SAILR `ReturnDuplicatorHigh`).

**That fix already exists and already ships in the default.** It is the `returndup`
option (P8 / `goto-quality`, `decompiler/crates/kuna-decomp/phases.toml`), and this exact
case is named in its own metadata — `inspiration = "decbench:O0-coreutils-factor-factor;
..."` and `use_when` literally records "coreutils factor: kuna 3 ifs / 0 early returns ->
4 ifs / early returns, GED 12 -> 0". It landed default-ON, then **DIV-18** reverted the
default on 2026-07-05 because the decbench aggregate measured −976 GED-perfect functions
(most source functions do NOT use early returns, so unconditional duplication diverges
from source more often than it converges). It stayed a per-run opt-in.

What closed the case since the benchmark is therefore a **mode default, not a code fix**:
**DIV-40** made the file front-ends default to `--mode auto`, and `auto` selects
`aggressive` below 500 KiB. `factor` is 92,544 bytes, and `AGGRESSIVE_OVERRIDES`
(`decompiler/crates/kuna-decomp/src/p0_knowledge/modes.rs:114`) contains `("returndup",
"on")`. So today's PRODUCT DEFAULT for this binary structures it exactly like angr and
exactly like the source. The `// return-dupe x4` slug on the prototype (DIV-39 warnstyle)
is the pass announcing its four duplications.

The benchmark ran the equivalent of `--mode reliable`, which is why the stored kuna block
shows the old shape. This is a stale-benchmark case, not a missing feature.

Two caveats worth carrying forward, neither of which changes this verdict:
- Under `--mode reliable` (and for binaries ≥ 500 KiB, where `auto` picks `reliable`) the
  gap is still live and is closed only by an explicit `--option returndup on`. Flipping
  `returndup` default-ON globally is **not** the follow-up — DIV-18 already measured that
  as a large aggregate regression. If anything is left here it is a *selectivity* question
  (when should the duplicator fire without a mode opt-in), which the default-on
  `earlyreturn` / `switchreturn` options already cover for the const-phi cases returndup's
  whole-block gate skips.
- The sibling `O2-noinline-coreutils-factor-factor` was not re-measured here.

## Proposed fix

None. The mechanism exists (`returndup`), it is correct on this function, and it is ON in
the shipped default path for this binary. The action item is a **rescore**, not a PR: the
benchmark should be re-run at today's product default (`--mode auto`) rather than
`reliable + listing on`, which will move this case (and every other case whose gap is an
aggressive-preset option) off the queue.

Do not open a feature PR for this case. Do not propose flipping `returndup` default-on —
that flip is DIV-18, already tried and already reverted with measurements.
