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
| `flagcompare` | on \| off | `on` | S3 / simplification-quiescence | GH-1276/8777 | Fold flag-modelled comparison idioms into clean compares: a boolean shifted into the sign bit ((b<<k) s< 0) and the N==V signed-overflow idiom (bra ge). | Flip on to clean flag-as-bit comparisons on architectures that model condition flags explicitly (8051, PIC24, etc.); off (default) is upstream byte-identical. |
| `switchmodbound` | on \| off | `off ⚠️ opt-in` | S2 / switch-model | GH-9191 | Bound a LOAD-table jumptable by a modulo (index % N) or and-mask on its index when no guard bounds it. | Set on PER PROGRAM when a switch reports 'Could not recover jumptable ... Too many branches' and renders as a computed call; DESTRUCTIVE as a global default (may over-bound an unrelated indirect jump). |
| `stackprobeloop` | on \| off | `on` | S2 / stack-pointer-normalization | GH-8017/6858 | Resolve a gcc -fstack-check / stack-clash probe loop's stack-pointer MULTIEQUAL to a fixed offset from the input SP, so the frame and call arguments recover cleanly. | Set on when a large-frame function shows &pxVar[-0x1000] page-probe noise or argument-less calls; shape-gated, so it is inert on functions without a probe loop. |
| `dynamichashmax` | on \| off | `on` | S6 / alias-facets | GH-8467 | Raise the DynamicHash same-address collision budget 8->16 so dense unrolled code can still resolve a unique dynamic symbol hash. | Set on PER FUNCTION when decompilation aborts with 'Unable to find unique hash for varnode' (e.g. AArch64/Go NEON byte-search loops); off (default) is upstream byte-identical. |
| `stackalias` | on \| off | `off ⚠️ opt-in` | S6 / alias-facets | GH-8500 | Hold a store-through-a-stack-pointer-alias (int *p=&x; *p=x; return *p) alive across the deadcode race so it is not dropped to an uninitialized stack read. | Set on PER FUNCTION when a take-address-of-local + store-through-pointer returns a spurious uninitialized local (xStack_*); DESTRUCTIVE as a global default (conservatively pins stack stores live, suppressing legitimate dead-store removal). |
| `arraystride` | on \| off | `on` | S3 / simplification-quiescence | GH-8724 | Re-express a strength-reduced array walk: rewrite a loop offset accumulator (acc += sizeof) as counter*stride so the array index is recovered. | Set on when a strided loop renders a raw offset accumulator (e.g. iVar += 0x414) instead of an index; off (default) is upstream byte-identical. |
| `sparcstructret` | on \| off | `off ⚠️ opt-in` | S2 / flow-classification | GH-6882 | Let the SPARC struct-return `unimp` after a call fall through instead of becoming a non-returning indirect call. | Set on PER SPARC PROGRAM when a struct-returning callee makes a function render a bogus (*pcVar1)() and drop its tail; DESTRUCTIVE as a global default (an IllegalInstructionTrap-fed BRANCHIND elsewhere is suppressed). |
| `condexeplace` | on \| off | `on` | S3 / simplification-quiescence | GH-9203 | Stop ActionConditionalConst from materializing a propagated constant as a COPY inside a loop predecessor block (a spurious `= 0` in the do/while body). | Set on to clean up a malformed do/while whose body holds an out-of-place constant assignment; off (default) is upstream byte-identical. |
| `inputvarnodeadjust` | on \| off | `on` | S6 / stack-frame-layout | GH-9218 | Absorb an input Varnode overlapping the high end of a justified parameter container instead of aborting the function. | Set on PER FUNCTION when a frame aborts with 'Cannot properly adjust input varnodes' (overlapping stack params, e.g. mc68k link/unlk); off (default) preserves the upstream abort. |

## Programmatic use

```bash
# discover (machine-readable):
python -m kuna.catalog --json

# decompile with an assertion flipped (repeatable):
python -m kuna.decompile ./a.out main --option compareform canonical
python -m kuna.decompile ./sparc.elf main --option returnpair single
```

The `⚠️ opt-in` defaults (`returnpair`, `v850indirectbranch`) are documented as destructive global defaults and ship off; apply them per function / per program only (see each row's *When to flip*).

