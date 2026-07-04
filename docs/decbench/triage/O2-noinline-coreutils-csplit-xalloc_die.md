---
case_id: O2-noinline-coreutils-csplit-xalloc_die
group_id: coreutils::xalloc_die
status: covered-by-option
tier: L
margin: 183
bucket: kuna-specific
fresh_reproduces: true
fresh_ged: null
option_closing: listing
root_cause_family: noreturn-propagation-stripped
feature_slug: noreturn-propagate-without-listing
scope: small
angr_ref: angr.analyses.cfg.cfg_fast.CFGFast (Function.returning no-return fixpoint, add_nonreturning_function / _iteratively_analyze_function_features)
kuna_stage: kuna-analysis S1 (s1_noreturn_propagate, gated on option listing)
closed_by: PR #121 (benchlisting, DIV-15)
ged_after_fix: 0
---

## Side-by-side

angr (stored) — full, 10 lines:

```c
// Function: xalloc_die @ 0x4260
void xalloc_die(void)
{
    unsigned long long v2;  // rax
    unsigned long long v0;  // [bp-0x8]

    v0 = v2;
    dcgettext(NULL, "memory exhausted", 5);
    error(0, 0, "%s");
    sub_403100(); /* do not return */
}
```

kuna (stored, run-time) — TRIMMED to the head + representative overrun (full body is 118 LOC, 2 gotos, 17 ifs, 7 loops; it swallows the next several functions in the binary including `find_line` from src/csplit.c):

```c
// Function: xalloc_die @ 0x4260
int8 xalloc_die(void)
{
  /* ... 18 declarations trimmed ... */
  v11 = (char *)0x0;
  error(0,0,0xac20,dcgettext(0,"memory exhausted",5));
  sub_3100();                      /* <-- should end here; kuna falls through */
  v7 = *v11;
  v15 = *(void *)(v13 + 0x28);
  if (v7 != '\0') {
    v1 = 0;
    do {
      while ((v7 == '%' && (v8 = &v9[1], v9[1] != '%'))) {
        if (v1) goto label_43ca;
        for (v8 = &v8[sub_2ed0(v8,&v10)]; (uint4)((int4)*v9 - 0x30U) <= 9; v9 = &v9[1]) {
        }
        /* ... suffix-format scanning loop (the NEXT function's body) ... */
      }
      v7 = v9[1];
      v9 = &v9[1];
    } while (v7 != '\0');
label_4350:
    if (v1) {
      v3 = __snprintf_chk(0,0,1,0xffffffffffffffff,v11,0x7fffffff);
      if (0 <= v3) {
        return (int8)v3;
      }
      xalloc_die();
      /* ... ~60 more lines: buffer-management loop, then find_line with
         __assert_fail(0xa138,"src/csplit.c",0x247,"find_line") ... */
    }
  }
  error(1,0,dcgettext(0,"missing %% conversion specification in suffix",5));
label_43ca:
  error(1,0,dcgettext(0,"too many %% conversion specifications in suffix",5));
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}
```

kuna (fresh, current main build, no options) — byte-identical to the stored block above (same 118 LOC / 2 gotos / 17 ifs / 7 loops). The gap fully reproduces on today's main.

kuna (fresh, `--option listing on`) — the closing run:

```c
// Function: xalloc_die @ 0x4260
void xalloc_die(void)

{
  error(0,0,0xac20,dcgettext(0,"memory exhausted",5));
                    /* WARNING: Subroutine does not return */
  sub_3100();
}
```

## Analysis

**Symptom.** kuna decompiles past the terminal `call 0x3100` in `xalloc_die` (4 instructions of real body) and swallows the next several functions in the stripped binary — the suffix-format scanner, a buffer-management loop, and `find_line` (visible via the surviving `__assert_fail(..., "src/csplit.c", 0x247, "find_line")`) — producing 118 LOC / 2 gotos / 7 loops where the source function is a single basic block. angr, ghidra, ida, binja all score GED 0.

**Root cause.** `0x3100` is not a PLT stub — it is an unnamed *internal* cold-path wrapper (`call 0x3090; mov edi,1; call exit@plt`). `exit` itself is resolvable by name even in the stripped binary (the dynsym/JUMP_SLOT reloc survives stripping) and kuna's default-on `noreturn_known` models it, but the no-return fact is never *propagated one call-graph hop* to the unnamed wrapper `sub_3100`, so `xalloc_die`'s `call 0x3100` is assumed to fall through into the alignment padding and the following functions. This is exactly the class documented in `decompiler/crates/kuna-analysis/src/s1_noreturn_propagate/mod.rs` (whose doc comment literally uses coreutils `xalloc_die` as the motivating example): the Known analyzer is name-based (wrapper is unnamed), and `noreturn_disc`'s ≥3-callsite evidence rule doesn't fire (too few callsites at -O2, and the NOP padding after the call parses as a valid fall-through).

**Why angr wins.** angr's CFGFast computes `Function.returning` as a call-graph fixpoint with no evidence threshold (`add_nonreturning_function` / the `_iteratively_analyze_function_features` loop in `angr/analyses/cfg/`): `exit` (SimProcedure, NO_RET) seeds the set, `sub_403100` ends unconditionally in a call to it, so it becomes non-returning, so `xalloc_die` terminates at the call — hence `sub_403100(); /* do not return */` and a perfect single-block match with the source CFG. Not a metric artifact: angr's stored block covers the full function (all three calls present), the source CFG is non-degenerate, and three independent decompilers also score 0.

**Why the default kuna knobs don't cover it.** `noreturn_known` / `noreturn_extern` / `noreturn_externmatch` (all default-on) are name-list matchers — they fire on `exit`/`abort`/`__stack_chk_fail` PLT calls, never on an unnamed internal wrapper in a stripped binary. `noreturn_propagate` — the kuna port of exactly angr's fixpoint — has been default-on since DIV-14, but it is a **no-op unless the program-wide Listing is built**, and `listing` is default-off (zero decode cost policy). Flipping `--option listing on` (which activates the default-on `noreturn_propagate`) closes the gap completely: kuna emits the same 3-call straight-line body with the call marked no-return, structurally identical to angr (0 gotos / 0 ifs / 0 loops).

**Sibling.** `O2-coreutils-csplit-xalloc_die` (margin 181): identical symptom — fresh kuna reproduces the same overrun (129 LOC / 2 gotos / 19 ifs / 5 loops vs angr's same 10-line block); the same `listing on` knob should close it.

## Proposed fix

No new pass needed — the mechanism (`s1_noreturn_propagate`, kuna-analysis S1) already exists, is angr-faithful, and is default-on; it is only starved of its input because building the Listing is default-off. Two small follow-up paths, per the one-PR/feature rule:

1. **Measure and, if it fits, flip `listing` default-on for the decompile-all path** — the 0/675 gate is unaffected (datatests don't load ELF programs through the analysis tier), so the question is purely the ≤5% speed budget: benchmark `kuna decompile-all` with/without `listing on` on a mid-size coreutils binary. If the full-program decode blows the budget:
2. **`noreturn-propagate-without-listing` (scope: small)** — decouple the propagation pass from the full Listing: it only needs a terminal-instruction decode of each function body (last basic block reachability to a `call <known-noreturn>`), which can run off the existing function-extent markup with a bounded on-demand decode instead of the whole-program Listing. Owning files: `decompiler/crates/kuna-analysis/src/s1_noreturn_propagate/mod.rs` (input source), `decompiler/crates/kuna-analysis/src/passes.rs` (gating), `decompiler/crates/kuna-decomp/stages.toml` + catalog count tests if the option surface changes. Risk: low — the pass is conservative by design (unconditional-terminal rule); the only new risk is decode cost, which the bounded variant caps.

Benchmark-side, nothing to file: the score is legitimate.
