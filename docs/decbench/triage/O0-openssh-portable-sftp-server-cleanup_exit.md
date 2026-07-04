---
case_id: O0-openssh-portable-sftp-server-cleanup_exit
group_id: openssh-portable::cleanup_exit
status: covered-by-option
tier: L
margin: 13
bucket: kuna-specific
fresh_reproduces: true
fresh_ged: null
option_closing: listing
root_cause_family: noreturn-propagation-stripped
feature_slug: listing-default-on
scope: small
angr_ref: angr.analyses.cfg.cfg_fast.CFGFast (nonreturning fixpoint in _analyze_all_function_features + mark_nonreturning_calls_endpoints, cfg_fast.py:2302-2351)
kuna_stage: S1/external-refinement (kuna-analysis s1_noreturn_propagate, Listing-gated)
closed_by: PR #121 (benchlisting, DIV-15)
ged_after_fix: 0
---
## Side-by-side

angr (stored):

```c
// Function: cleanup_exit @ 0x102c2
void cleanup_exit(int a0)
{
    sub_40f466(a0); /* do not return */
}
```

kuna (stored == fresh, current main — byte-identical, gap reproduces):

```c
// Function: cleanup_exit @ 0x102c2
unsigned long cleanup_exit(unsigned int a0,unsigned long a1)

{
  int8 v1; // rax
  unsigned long v2; // rax

  sub_f466();
  sub_1d7a8();
  v1 = getpwuid(getuid());
                    /* WARNING: branchflip: flipped negated guard for linearity (positive condition, if/else arms swapped) */
  if (v1 != 0) {
    v2 = sub_f52e(a0,a1,v1);
  }
  else {
    fprintf(dat_35200,"No user found for uid %lu\n",(uint8)(uint4)getuid());
    v2 = 1;
  }
  return v2;
}
```

kuna fresh with `--option listing on` (activates the default-on `noreturn_propagate` pass) — structurally identical to angr, gap fully closed:

```c
// Function: cleanup_exit @ 0x102c2
void cleanup_exit(unsigned int a0)

{
                    /* WARNING: Subroutine does not return */
  sub_f466(a0);
}
```

Isolation: `--option listing on --option noreturn_propagate off` restores the overrun output, so `noreturn_propagate` (not the Listing itself) is the closing mechanism; the Listing is only its prerequisite.

## Analysis

**Symptom.** Boundary overrun past an internal noreturn wrapper. openssh declares `cleanup_exit` `__attribute__((noreturn))` (log.h), so at O0 GCC emits `call f466` and *no epilogue/ret* — the next byte (0x102db) is the next function's `endbr64`. kuna's flow falls through the missing return into the following function (`sub_1d7a8`/`getpwuid`/"No user found for uid" is that next function's body), inflating a 1-call wrapper into an 18-line function with an if/else → GED 13 vs angr/ghidra/ida/phoenix all 0.

**Root cause.** The callee `sub_f466` (`sftp_server_cleanup_exit`, stripped) is an *internal* function whose body `[0xf466, 0xf52e)` ends in `call exit@plt` with no ret (confirmed by objdump). The static name lists can't mark it (no symbol name survives stripping); only call-graph propagation from `exit@plt` can. kuna HAS exactly that pass — `noreturn_propagate`, default-ON, the explicit analog of angr's CFGFast propagation — but it is a documented no-op when the Listing is absent (`kuna-analysis/src/s1_noreturn_propagate/mod.rs:111-116`: `let Some(listing) = ctx.listing else { return }`), and `listing` is default-OFF (zero decode cost, parity-gate byte-identity). So the benchmark ran with the closing pass silently inert.

**Why angr wins.** angr's CFGFast always runs its noreturn fixpoint: `_analyze_all_function_features` iterates `_updated_nonreturning_functions`, then the post-pass (cfg_fast.py:2302-2351 in the venv 9.2.213 that ran) removes unconfirmed `fake_return` edges after calls to nonreturning functions and calls `mark_nonreturning_calls_endpoints()`. `exit` seeds via the PLT/dynsym (survives stripping), the wrapper is concluded nonreturning, and `cleanup_exit` is bounded at the call. Ghidra proper does the same via its "Non-Returning Functions - Discovered" analyzer — which is why ghidra also scores 0 and the bucket is kuna-specific.

**Why the other noreturn options don't fire.** `noreturn_known` (S1) does mark `exit@plt` — but only the direct libc symbol, not the stripped internal wrapper that calls it. `noreturn_extern`/`noreturn_externmatch` (S2) are name-list matches at the flow seam — `sub_f466` has no name to match. `noreturn_disc` (default-off) needs >=3 no-fallthrough call sites and also needs the Listing. Only `noreturn_propagate` reaches a single-caller stripped internal wrapper, and it is Listing-gated.

**Artifact check** (quick; artifact_suspect=false): not an artifact. ghidra/ida/phoenix all independently score 0, and angr's 1-call output is the *correct* rendering of the source (`cleanup_exit` is a one-line wrapper in sftp-server.c). No rescore needed.

**Siblings.**
- `O2-openssh-portable-sftp-server-cleanup_exit` (GED 186): same symptom, amplified — kuna overruns `call sub_c950` into the next function, swallowing a 61-case error-string switch (196 loc vs angr's 8).
- `O2-noinline-openssh-portable-sftp-server-cleanup_exit` (GED 186): identical to the O2 sibling (same 61-case switch overrun past `sub_d780`). Both should close the same way with `listing on`.

## Proposed fix

No new pass needed — the mechanism exists and closes the gap exactly. The fix is enablement:

1. **Benchmark-path enablement (smallest, zero-risk):** have the decbench backend / `kuna decompile-all` pass `--option listing on` (or add a CLI default for the whole-binary path). In `decompile-all` the Listing is built once per binary (load-once), so the whole-program recursive-descent decode is amortized across all functions — the <=5% speed policy is most plausibly satisfiable there, not in subprocess-per-function `kuna decompile`.
2. **Default-on candidate (`listing-default-on`):** flip `listing` default to on for the real-ELF path. Parity is trivially safe — the catalog documents the Listing tier as real-ELF-only, so all 675 datatest assertions and the stages corpus are byte-identical (0/675 policy holds by construction). The open questions are (a) decode cost on large binaries for single-function runs and (b) `noreturn_propagate` mis-fires on unusual bodies (it is HARD/REMOVES CODE); both argue for measuring the standing speed metric before flipping, per docs/pipeline.md.

Owning files: `decompiler/crates/kuna-analysis/src/s1_noreturn_propagate/mod.rs` (pass, already correct), `decompiler/crates/kuna-analysis/src/passes.rs` (`run_listing_consumers`), `decompiler/crates/kuna-decomp/stages.toml` + `p0_knowledge/options.rs` (the `listing` default), plus the option-count fixtures per the adding-option recipe if defaults change catalog docs.

Risks: whole-binary decode time on big stripped binaries (note the known infinite-loop hang on stripped openssh/bash in `tests/hang-repro/` — the Listing's program-wide decode path must be vetted against that before any default flip); false-positive noreturn conclusions dropping live code.
