---
case_id: O0-gzip-gzip-abort_gzip
group_id: gzip::abort_gzip
status: covered-by-option
tier: L
margin: 10
bucket: kuna-specific
fresh_reproduces: true
fresh_ged: null
option_closing: listing
root_cause_family: noreturn-propagation-stripped
feature_slug: null
scope: small
angr_ref: angr/analyses/cfg/cfg_base.py::CFGBase._determine_function_returning
kuna_stage: S1 code-data-partition (kuna-analysis s1_noreturn_propagate, Listing-gated)
---
## Side-by-side

angr (stored):

```c
// Function: abort_gzip @ 0x8f9f
void abort_gzip(void)
{
    sub_408ebb(0);
    sub_408e40(1); /* do not return */
}
```

kuna (stored, 0.1.0 == kuna fresh, current main — byte-identical):

```c
// Function: abort_gzip @ 0x8f9f
void abort_gzip(void)

{
  int4 v1; // edi

  sub_8ebb(0);
  v1 = 1;
  sub_8e40();
  sub_8ebb(1);
  if (v1 == dat_dd940) {
                    /* WARNING: Subroutine does not return */
    _exit(2);
  }
  signal(v1,0);
  raise(v1);
  return;
}
```

kuna (fresh, `--option listing on` — noreturn_propagate, already default-on, now actually fires):

```c
// Function: abort_gzip @ 0x8f9f
void abort_gzip(void)

{
  sub_8ebb(0);
                    /* WARNING: Subroutine does not return */
  sub_8e40(1);
}
```

Ablation: `--option listing on --option noreturn_propagate off` restores the full overrun output, pinning the closing mechanism to `noreturn_propagate` (listing is only its gate).

## Analysis

Symptom: kuna decompiles past the call to `sub_8e40` (= gzip's static `do_exit`, whose every path ends in `call exit`) and swallows the entire body of the *next* function in the stripped binary — `abort_gzip_signal` (`remove_output_file(true); if (sig == exiting_signal) _exit(WARNING); signal(sig, SIG_DFL); raise(sig);`), which is exactly the `sub_8ebb(1) / if (v1 == dat_dd940) _exit(2) / signal / raise` tail in kuna's output. That extra swallowed CFG is the whole GED 10.

Root cause: `do_exit` is an internal **static** function in a **stripped** binary — no symbol, so every name-based kuna no-return mechanism (`noreturn_known`, `noreturn_extern`, `noreturn_externmatch`, all default-on) cannot match it. The only mechanism that can conclude it is no-return is body/call-graph analysis: `do_exit` ends in `call exit@plt` (dynamic symbol, survives stripping, already modeled no-return), so no-return must be *propagated* one hop up the call graph. kuna has exactly this pass — `decompiler/crates/kuna-analysis/src/s1_noreturn_propagate/mod.rs`, the explicit kuna analog of angr's CFGFast `returning` fixpoint, default-ON since DIV-14 — but it is a documented **no-op unless the Listing is built** (`option listing on`, default OFF for zero decode cost, stages.toml `option = "listing"`). The decbench backend (`kuna decompile-all --json` via `kuna_raw.py`) does not pass `listing on`, so the pass never runs.

Why angr (and ghidra/ida/binja, all GED 0) win: angr's `CFGBase._determine_function_returning` / `_analyze_all_function_features` fixpoint (in the benchmark's angr 9.2.213, `~/.virtualenvs/decbench/.../angr/analyses/cfg/cfg_base.py:1067`) marks `do_exit` `returning=False` because all its call exits reach the `exit` SimProcedure; the call site in `abort_gzip` then terminates the block, matching the 2-call source function exactly. Ghidra-proper runs its equivalent non-returning analyzers by default — kuna ported the pass but left its Listing substrate default-off, which is precisely why this bucket is kuna-specific.

Not an artifact: angr's stored block is the complete, correct source function (`remove_output_file(false); do_exit(ERROR);`), not truncated; ghidra/ida/binja independently score 0.

Siblings (same symptom, larger blast radius at O2 where more optimized code follows):
- O2-gzip-gzip-abort_gzip (GED 207): stored kuna falls through `sub_6960()` (= do_exit) into 180 lines / 14 loops of swallowed neighbor code (stored 0.1.0 output even contains malformed `while ( for (;` text).
- O2-noinline-gzip-gzip-abort_gzip (GED 98): stored kuna falls through `sub_6fd0(1)` into 85 lines / 3 loops of the following function.

## Proposed fix

No new pass needed — the mechanism exists and is default-on; only its gate is closed. Two small actions, per docs/pipeline.md rule 1:

1. Benchmark-side (immediate): make the decbench kuna backend (`decbench/decompilers/raw/kuna_raw.py` → `kuna decompile-all`) pass `--option listing on`, activating the already-default-on `noreturn_propagate` (and enabling `noreturn_disc` evaluation). This should also collapse the O2/O2-noinline siblings (group margin 207).
2. Engine-side (candidate): measure the Listing build cost on real ELF targets and evaluate flipping `listing` default-on for the real-ELF path (it is already byte-identical over the 675 datatests since no datatest goes through the loadimage/ELF path — the parity gates cannot regress). The risk is purely the <=5% speed policy: the Listing is a program-wide recursive-descent disassembly built once per load, so `kuna decompile-all` (load-once, decompile-many) amortizes it well, but single-function `kuna decompile` pays it in full — measure both before proposing DIV. Owning files: `decompiler/crates/kuna-analysis/src/s1_noreturn_propagate/mod.rs` (pass, no changes), `decompiler/crates/kuna-decomp/stages.toml` (`listing` default), plus the default-flip test/count checklist in memory `kuna-adding-option-count-tests`.
