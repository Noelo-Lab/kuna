# kuna assertion catalog

**Generated** from the decompiler's `stage catalog` command (`python -m kuna.catalog --markdown`) -- do not edit by hand; edit `settableTable` in `decompiler/cpp/kuna_stages.cc` and regenerate.

These are the kuna stage-model sub-stage decisions an operator (human or LLM) can flip per decompilation. Defaults are the shipped values (post-DIV-2, see `docs/divergences.md`). Set any of them with `python -m kuna.decompile <bin> <fn> --option <name> <value>` (or, per function, `--kassert "<stage> <substage> ..."`); revert any one with its `off`/`canonical` value.

| Option | Values | Default | Stage / sub-stage | Issue | Decision | When to flip |
|---|---|---|---|---|---|---|
| `compareform` | canonical \| original | `original` | S3 / comparison-canonicalization | GH-558 | Whether comparisons keep their source form (V <= c) or the analysis-canonical rewrite (V < c+1). | Flip to canonical only to reproduce upstream Ghidra output; original (default) is more faithful to source. |
| `arraynotation` | on \| off | `on` | S9 / pointer-notation | GH-558 | Render standalone pointer arithmetic as &base[index] (on) vs base + index (off). | Flip off if the consumer expects raw pointer-arithmetic rendering; on (default) is more readable. |
| `thumbfuncptr` | on \| off | `on` | S5 / const-pointer | GH-8471 | Preserve a Thumb function pointer (fn|1) as a symbolic &fn[1] rather than a raw hex literal. | Flip off only to see the raw constant; on (default) recovers the function symbol on ARM/Thumb. |
| `inferfuncentry` | on \| off | `on` | S5 / const-pointer | GH-6930 | Infer a function pointer when a constant equals an exact function entry at a single-bit image base (e.g. 0x100000). | Flip off only to reproduce the bare-literal form; on (default) names the function. |
| `booleanmask` | on \| off | `on` | S3 / simplification-quiescence | GH-1282 | Fold the (b<<k) s>>k boolean sign-extension-mask idiom (flag-as-high-bit lowering) into a clean comparison. | Flip off only to see the raw shift idiom; on (default) cleans flag-modelled comparisons (8051 etc.). |
| `ovlesssimplify` | on \| off | `on` | S3 / simplification-quiescence | GH-7190 | Simplify the explicit S/OV-flag compare idiom into a direct signed comparison. | Flip off only to see the raw overflow-flag arithmetic; on (default) cleans V850-style compares. |
| `addcarrychain` | on \| off | `on` | S5 / simplification-quiescence | GH-8913 | Recover an 8-bit carry-chain (ADC) pair into a single wide add instead of CONCAT11(CARRY1(...)). | Flip off only to see the raw carry intrinsics; on (default) recovers wide arithmetic (6502 etc.). |
| `memsetrecover` | on \| off | `on` | S5 / constsequence | GH-9230/1537 | Collapse a run of constant-fill stores (incl. inlined/SIMD bzero) into a single builtin_memset. | Flip off only to see the individual element stores; on (default) reconstructs the memset. |
| `returnpair` | pair \| single | `pair ⚠️ opt-in` | S4 / trial-budget | GH-6990 | Whether a passively-active second return register may be joined into a wide return (pair) or dropped (single). | Set single PER FUNCTION when a void/single-register function shows a spurious CONCAT44 return (e.g. SPARC); DESTRUCTIVE as a global default (truncates real multi-register returns). |
| `v850indirectbranch` | on \| off | `off ⚠️ opt-in` | S2 / flow-classification | GH-8817 | Reclassify a V850 jmp [reg] CALLIND to BRANCHIND so switch-table recovery runs. | Set on PER V850 PROGRAM to recover jump-table switches; DESTRUCTIVE as a global default (matches register-indirect calls on other architectures). |

## Programmatic use

```bash
# discover (machine-readable):
python -m kuna.catalog --json

# decompile with an assertion flipped (repeatable):
python -m kuna.decompile ./a.out main --option compareform canonical
python -m kuna.decompile ./sparc.elf main --option returnpair single
```

The `⚠️ opt-in` defaults (`returnpair`, `v850indirectbranch`) are documented as destructive global defaults and ship off; apply them per function / per program only (see each row's *When to flip*).

