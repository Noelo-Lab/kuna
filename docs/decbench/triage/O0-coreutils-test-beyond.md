---
case_id: O0-coreutils-test-beyond
group_id: coreutils::beyond
status: covered-by-option
tier: L
margin: 58
bucket: kuna-specific
fresh_reproduces: true
fresh_ged: null
option_closing: listing
root_cause_family: noreturn-propagation-stripped
feature_slug: listing-default-on
scope: small
angr_ref: angr.analyses.cfg.cfg_fast.CFGFast._iteratively_analyze_function_features (call-graph no-return propagation)
kuna_stage: S1/external-refinement (kuna-analysis s1_noreturn_propagate, gated on the Listing)
closed_by: PR #121 (benchlisting, DIV-15)
ged_after_fix: 0
---
## Side-by-side

angr (stored) — complete, NOT truncated (source `beyond` really is a two-call noreturn body):

```c
// Function: beyond @ 0x2b53
void beyond(void)
{
    unsigned long long v2, v3, v5, v6, v7, v8;
    unsigned long long v0;  // [bp-0x10]

    v0 = v2;
    v3 = sub_407078(*((long long *)(g_40d0d4 * 8 - 8 + g_40d0d8)));
    sub_402a28(gettext("missing argument after %s"), v3, v5, v6, v7, v8); /* do not return */
}
```

kuna (stored == fresh, byte-identical; 47 loc, 5 ifs, 3 loops — trimmed to the shape, cut ~20 lines of the swallowed body):

```c
// Function: beyond @ 0x2b53
char * beyond(void)
{
  ...
  v3 = sub_7078(*(void *)((int8)dat_d0d4 * 8 + -8 + dat_d0d8));
  v4 = (char *)gettext("missing argument after %s");
  sub_2a28(v4,v3);          /* <-- noreturn wrapper, kuna keeps going */
  v6 = v4;
  while( true ) {           /* everything below is the NEXT function (find_int): */
    v2 = *(int8 *)__ctype_b_loc();   /* isspace skip loop */
    ...
  }
  if (*v6 != '+') { ... } else { ... }   /* +/- sign handling */
  if ((uint4)((int4)*v6 - 0x30U) <= 9) {
    do { v6 = &v6[1]; } while (...);     /* digit scan */
    while( true ) { ... }                /* trailing isspace */
    if (*v6 == '\0') { return v7; }
  }
  v3 = sub_7078(v4);
  v5 = gettext("invalid integer %s");
  return (char *)sub_2a28(v5,v3);
}
```

kuna (fresh, `--option listing on` — `noreturn_propagate` is already default-on and fires once the Listing exists; gap fully closed):

```c
// Function: beyond @ 0x2b53
void beyond(void)
{
  unsigned long v1; // rax

  v1 = sub_7078(*(void *)((int8)dat_d0d4 * 8 + -8 + dat_d0d8));
                    /* WARNING: Subroutine does not return */
  sub_2a28(gettext("missing argument after %s"),v1);
}
```

## Analysis

- **Symptom.** coreutils `test`/`beyond` is `test_syntax_error(_("missing argument after %s"), quote(argv[argc-1]))` — a call to a *static internal* noreturn wrapper (`sub_2a28` = `test_syntax_error`, which ends in `exit`). In the stripped binary kuna does not know `sub_2a28` is noreturn, treats the call as returning, and decompiles straight through into the next function's body (`find_int`: the ctype/isspace loops, +/- sign, digit scan). 16 source-shaped lines balloon to 47 with 5 ifs + 3 loops → GED 58. Fresh kuna (current main) is byte-identical to the stored run: still reproduces.
- **Root cause.** Internal-noreturn propagation is inert by default. kuna's name-keyed passes cannot fire here: `noreturn_known` (S1) and `noreturn_extern`/`noreturn_externmatch` (S2 flow seam) match *known ELF names* (`exit`, `abort`, `__stack_chk_fail`, …) — `sub_2a28` is a stripped local symbol with no name to match. The pass that DOES handle this, `noreturn_propagate` (`decompiler/crates/kuna-analysis/src/s1_noreturn_propagate/mod.rs`, the kuna analog of angr's CFGFast returning-analysis fixpoint, default-ON since DIV-14/#110), is a structural no-op unless the Listing is built — and `listing` is **default-off** (program-wide recursive-descent disassembly cost). The decbench backend (`decbench/decompilers/raw/kuna_raw.py` → `kuna decompile-all --json`, no options) therefore ran with the entire discovered/propagated-noreturn machinery disabled.
- **Why angr wins.** angr's CFGFast always runs whole-program and its `_iteratively_analyze_function_features` no-return propagation concludes `test_syntax_error` never returns (its only exit is a call chain reaching `exit`), so the decompiled `beyond` is bounded at the call (`/* do not return */`). ghidra/ida/binja all score 0 too (Ghidra's "Non-Returning Functions - Discovered" analyzer, etc.) — this is kuna-specific enablement, not a structuring gap.
- **Verified closure.** `--option listing on` alone closes the gap: kuna's output becomes structurally identical to angr's (straight-line, call marked noreturn, no swallowed code). `noreturn_propagate` needs no flip (already on); `noreturn_disc` was not needed (its >=3-call-site evidence threshold is irrelevant once propagate fires).
- **Artifact check (quick).** Not an artifact: angr's block is complete for the true 2-call source body, and ghidra/ida/binja score 0 independently of the angr engine. No rescore needed (tier L, artifact_suspect=false).
- **Siblings.** All 5 siblings are the same function in `test`/`[` at other opt levels and share the symptom: `O2-coreutils-test-beyond` — identical overrun, much worse at O2 (kuna 202 loc / 11 gotos / 9 labels vs angr's 7-line noreturn body; margin 221, the group driver). `O0-coreutils-_-beyond` — byte-for-byte the same story at the same address 0x2b53 in the `[` binary (margin 58). Expect `--option listing on` to close the whole group.

## Proposed fix

No new pass needed — this is an **enablement** decision for existing, already-default-on machinery:

1. **Benchmark-side (immediate, zero kuna risk):** pass `--option listing on` in `decbench/decompilers/raw/kuna_raw.py`'s `kuna decompile-all` invocation (the CLI accepts `--option N V`). Measure the whole-binary wall-time delta on a decbench project first (per the always-measure standing requirement) — the Listing is one program-wide recursive-descent disassembly per load, amortized across all functions in the load-once/decompile-many path, so the relative cost on `decompile-all` should be small.
2. **kuna-side (the real fix, `small`):** flip `listing` default-on for the real-ELF path in `decompiler/crates/kuna-decomp/stages.toml` (+ the settableTable count-test/golden sweep per the adding-option recipe). Parity gates stay byte-identical by construction (the Listing is real-ELF-only, so all 675 datatests + stages corpus are unaffected — same argument that let `noreturn_propagate` go default-on in DIV-14). The gate is the <=5% decompile-speed policy: benchmark the Listing build cost on single-function `kuna decompile` (where it is NOT amortized) before flipping; if single-function cost is too high, an alternative is defaulting `listing on` only in the `decompile-all` in-process path.
   Risk: `noreturn_propagate` becoming live everywhere REMOVES CODE after calls it concludes are noreturn — a wrong conclusion hides real code. Its precondition set (terminal-seeded fixpoint, no return path, no computed jump, no escaping branch) is conservative, and it has two shipped witnesses (tee_O2 x2nrealloc, this group); the KUNA_DEFAULT_ON ablation hook covers rollback.
