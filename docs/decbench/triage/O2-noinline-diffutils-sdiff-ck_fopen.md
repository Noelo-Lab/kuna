---
case_id: O2-noinline-diffutils-sdiff-ck_fopen
group_id: diffutils::ck_fopen
status: covered-by-option
tier: M
margin: 82
bucket: kuna-specific
fresh_reproduces: true
fresh_ged: null
option_closing: listing
root_cause_family: noreturn-propagation-stripped
feature_slug: null
scope: none
angr_ref: angr.analyses.cfg.cfg_base.CFGBase._determine_function_returning
kuna_stage: S1/external-refinement (kuna-analysis s1_noreturn_propagate, gated on listing)
---
## Side-by-side

angr (stored) — full:

```c
// Function: ck_fopen @ 0x4a60
typedef struct FILE {
} FILE;

FILE * ck_fopen(char *a0, char *a1)
{
    FILE *fp;  // rax

    fp = fopen(a0, a1);
    if (!fp)
        sub_4049e0(a0); /* do not return */
    return fp;
}
```

kuna (stored == fresh default, 63 loc / 10 ifs / 2 loops; trimmed to the first ~30 lines — the rest is more of the same swallowed ck_* bodies):

```c
// Function: ck_fopen @ 0x4a60
char * ck_fopen(char *a0,char *a1)
{
  int4 v1; // eax
  uint4 v2; // eax
  char *v3; // rax
  int8 *v4; // rax

  v3 = fopen(a0,a1);
  if (v3 != (char *)0x0) {
    return v3;
  }
  sub_49e0(a0);                      /* <-- internal noreturn wrapper; flow should stop here */
  v3 = (char *)fwrite_unlocked();    /* everything below is the NEXT functions' bodies: */
  if (v3 == a1) {                    /* ck_fwrite, ck_fflush, ck_fclose, a getchar loop, */
    return v3;
  }
  sub_49e0(dcgettext(0,"write failed",5));
  v3 = (char *)fflush_unlocked();
  if ((int4)v3 == 0) {
    return v3;
  }
  sub_49e0(dcgettext(0,"write failed",5));
  do {
    v1 = getchar_unlocked();
    if (v1 == -1) break;
  } while (v1 != 10);
  /* ... ~33 more lines: ferror/fclose/__ctype_b_loc/fread_unlocked chains,
     ending in a sub_4bd0 buffer-append block — all from adjacent functions ... */
  return v3;
}
```

kuna (fresh, `--option listing on`) — full:

```c
// Function: ck_fopen @ 0x4a60
void ck_fopen(char *a0,char *a1)

{
  if (fopen(a0,a1) != (void *)0x0) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  sub_49e0(a0);
}
```

Metrics: angr 11 loc / 1 if; kuna default 63 loc / 10 ifs / 2 loops; kuna with `listing on` 9 loc / 1 if / 0 gotos — structural parity with angr (source is a 4-line wrapper: fopen, null-check, fatal, return).

## Analysis

Symptom: classic noreturn boundary overrun. `ck_fopen` calls `sub_49e0` (sdiff's stripped internal `perror_fatal`/`fatal` error wrapper, chain ending in `exit`) on fopen failure. angr's CFGFast marks that wrapper non-returning via its call-graph returning fixpoint (`CFGBase._determine_function_returning`, iterated in `_analyze_function_features`), so its decompilation stops at the call (`/* do not return */`) and scores GED 0. kuna's default run treats the call as returning, falls through past the function end, and swallows the bodies of the adjacent `ck_fwrite`/`ck_fflush`/`ck_fclose`/getc-loop functions into one 63-line blob — GED 82. Fresh kuna (current main) reproduces the stored output byte-for-byte, so no post-Jun-29 PR fixed it.

Why existing kuna noreturn options don't fire by default:
- `noreturn_known` (on) and `noreturn_extern`/`noreturn_externmatch` (on) are NAME-keyed — `sub_49e0` is an internal, stripped-name wrapper, not a known libc name.
- `noreturn_propagate` (on by default since DIV-14) is exactly the right pass — the kuna analog of angr's call-graph propagation, `kuna-analysis/src/s1_noreturn_propagate/mod.rs` — but it is a documented **no-op without the Listing substrate**, and `listing` is default-OFF (program-wide recursive-descent disassembly, kept off for zero decode cost / byte-identical parity gates).
- `noreturn_disc` (off) is the >=3-call-site evidence variant; not needed once propagation runs.

Confirmed: rerunning with `--option listing on` (activating the already-default-on `noreturn_propagate`) collapses kuna to 9 loc / 1 if, structurally equivalent to angr. So the gap is fully covered by an existing option; the only delta left vs angr is cosmetic (void vs FILE* return type on the recovered prototype), not CFG shape.

Artifact check (quick, artifact_suspect=false): angr's block is not truncated — it correctly bounds at the noreturn call and keeps the `return fp` tail; ida=0 and binja=0 independently agree, ghidra=6 (its default-on "Non-Returning Functions - Discovered" analyzer catches the wrapper; residual 6 is shape noise). Genuine kuna-default gap, not a metric artifact.

Siblings: none (group has no other cases).

## Proposed fix

Nothing to build — the pass exists and closes the case. The remaining decision is deployment of the existing knob:

1. Benchmark-side (cheapest, zero engine risk): have the decbench backend (`decbench/decompilers/raw/kuna_raw.py` → `kuna decompile-all`) pass `--option listing on` for stripped targets, activating the default-on `noreturn_propagate`. This should sweep every `noreturn-propagation-stripped`-family case in one config change.
2. Engine-side (default-on question): flip `listing` default-on for real-ELF targets. Parity gates stay byte-identical only because the Listing is real-ELF-path-only today, so 0/675 holds; the risk is the <=5% speed policy — the Listing is a program-wide recursive-descent disassembly, and its cost on large binaries needs measuring before a DIV entry. Also note the known-issues hang on some fully-stripped x86-64 ELFs (`tests/hang-repro/`) — verify `listing on` does not widen that exposure before defaulting.
3. If (2) is too costly, a lighter alternative is a lazy, on-demand callee-returning probe (decompile-time bounded walk of the direct callee chain) instead of the full Listing — but that is new infrastructure and not justified while (1) is available.

Owning files: `decompiler/crates/kuna-analysis/src/s1_noreturn_propagate/mod.rs` (the pass, correct as-is), `decompiler/crates/kuna-analysis/src/passes.rs` + `kuna-decomp/stages.toml` (defaults), decbench `kuna_raw.py` (invocation).
