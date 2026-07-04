---
case_id: O2-tar-tar-write_fatal_details
group_id: tar::write_fatal_details
status: metric-artifact
tier: S
margin: 14
bucket: ahead
fresh_reproduces: true
fresh_ged: 11
option_closing: noreturn_disc
root_cause_family: angr-truncation-artifact
feature_slug: null
scope: none
angr_ref: angr.analyses.cfg.cfg_fast.CFGFast (prologue-scan seeds spurious function start at entry+6 via archinfo ArchAMD64.function_prologs `sub rsp,imm` pattern)
kuna_stage: S1/external-refinement
---
## Side-by-side

angr (stored) — the ENTIRE scored output; obviously truncated (real function is 0x58 bytes with an if/else, per `nm -S` on the unstripped binary):

```c
// Function: write_fatal_details @ 0xda10
long long write_fatal_details(void)
{
    unsigned long long v2;  // rax
    unsigned long long v0;  // [bp-0x8]

    v0 = v2;
    sub_40da16(); /* do not return */
}
```

kuna (stored == fresh, byte-identical on current main d12ef72):

```c
// Function: write_fatal_details @ 0xda10
void write_fatal_details(void)

{
  int4 v1; // edi

  sub_33510();
  if (dat_82a34 <= 0x3fffffff) goto label_da4e;
  v1 = dat_82a34 + -0x40000000;
  if (sub_34350(v1) != 0) goto label_da57;
  do {
    do {
      sub_2bee0(dat_82168,0);
      sub_16fd0();
label_da4e:
    } while (close() == 0);
label_da57:
    sub_32d60(*dat_82828);
  } while( true );
}
```

kuna fresh with `--option listing on --option noreturn_disc on` — structurally identical to Ghidra-proper's stored output, vj_ged vs source = **0.0**:

```c
// Function: write_fatal_details @ 0xda10
void write_fatal_details(void)

{
  int4 v1; // eax

  sub_33510();
  if (0x40000000 <= dat_82a34) {
    v1 = sub_34350(dat_82a34 + -0x40000000);
  }
  else {
    v1 = close();
  }
  if (v1 != 0) {
    sub_32d60(*Unique10000093);
  }
  sub_2bee0(dat_82168,0);
                    /* WARNING: Subroutine does not return */
  sub_16fd0();
}
```

## Analysis

Source (tar `src/buffer.c`, preprocessed `buffer.i`, Joern source CFG = 6 nodes / 7 edges — NOT degenerate):
`write_fatal_details` = `write_error_details(...)` + inlined `rmtclose(archive)` (the `fd < 0x40000000 ? close : __rmt_close` bias test) + `if (!= 0) close_error(*archive_name_cursor)` + `fatal_exit()` (noreturn tail).

**The recorded angr 0 / margin 14 is a scoring artifact, twice over:**

1. **angr's output is truncated.** The function starts `endbr64; push rax; pop rax` (a
   -fstack-clash-protection probe); the next instruction at entry+6 (0xda16) is
   `48 83 ec 08` (`sub rsp,8`), which matches archinfo's AMD64 `function_prologs` pattern
   `\x48[\x83\x81]\xec[\x00-\xff]` (verified against the venv archinfo). On the stripped
   binary angr CFGFast's prologue scan seeds a *spurious function start at 0xda16*, splitting
   the entire real body (the rmtclose if/else, close_error, fatal_exit tail) into
   `sub_40da16`; `write_fatal_details` itself is rendered as a 2-statement noreturn
   tail-call stub. phoenix=0 is the same angr CFG, not independent confirmation.
2. **The recorded run values do not reproduce on the current scorer** (rescore printed
   METRIC-DRIFT). Recomputing every stored block against the same 6-node source CFG with
   today's decbench vj_ged: angr 0→**21** (worst of all engines), phoenix 0→21,
   kuna 14→**11**, ida 20→**7**, ghidra 21→**0** (perfect). The recorded numbers are
   near-inverted (ghidra's stored output actually matches source exactly); the stored
   full_run GED column for this function is stale scorer output, and the "angr ahead by 14"
   margin does not exist — on the current metric kuna (11) beats angr's real output (21).

**The real (secondary) kuna gap — behind ghidra 11 vs 0 — is noreturn discovery on a
stripped internal function.** `sub_16fd0` is `fatal_exit` (confirmed by `nm` on the
unstripped binary), a local function with no reachable RET (ends in `call error@plt` with
status 2). Ghidra-proper's "Non-Returning Functions - Discovered" analyzer catches it
(`/* WARNING: Subroutine does not return */` in ghidra's stored block), so the fall-through
edge after the call disappears and the backward layout jumps to the fatal_exit tail at
0xda3c stop reading as loop back-edges. Default kuna keeps that edge and structures the
shared noreturn tail as nested `do{}while(true)` + 2 gotos (Joern CFG 3n/5e vs source 6n/7e).

Why the default-on noreturn options don't fire here: `noreturn_known`/`noreturn_extern`/
`noreturn_externmatch` match callee NAMES against the known-no-return list (exit/abort/
`__stack_chk_fail`/...) — `sub_16fd0` is an internal stripped-name function, and its own
tail call `error@plt` is only conditionally noreturn (status!=0), so no name match exists.
`noreturn_propagate` propagates existing no-return facts through the call graph but has no
seed fact to propagate. The discovery analog already exists in kuna: `noreturn_disc`
(S1/external-refinement, default OFF, the evidence-tally fixpoint over the Listing/xref
tier, which itself needs `listing on`, also default OFF). Flipping both closes kuna to a
perfect vj_ged **0.0** (6n/7e, isomorphic to source) — no new feature needed.

Siblings: `O2-noinline-tar-tar-write_fatal_details` — identical symptom end to end: angr
stub tail-calls `sub_40d676` (= entry 0xd670 + 6, same push/pop probe then `sub rsp,8`
prologue-scan split), kuna fresh==stored with the same 2-goto/2-loop shape. Same artifact,
same `noreturn_disc` coverage expected.

## Proposed fix

No kuna feature work (scope: none — the case's margin is a benchmark artifact).

Decbench-side bugs to file:
1. **Stale/inverted stored GEDs**: the full_run `evaluated/*.toml` GED values were produced
   by an older extraction/scorer and can be arbitrarily wrong on the current code (here
   ghidra recorded 21 vs true 0, angr recorded 0 vs true 21). Any margin mined from the
   stored run should be re-validated with the current scorer before triage tiering; ideally
   re-score the whole full_run once with the current `decbench.metrics.ged` + Joern.
2. **Truncation rewarded**: a decompiler that emits a stub for a split function should not
   be scoreable near 0 against a multi-node source; consider a guard (e.g. penalize
   `decompiled_nodes==1` when `source_nodes>2`, or detect `/* do not return */` stub
   tail-calls into unnamed `sub_<entry+k>` functions).

kuna follow-up worth tracking separately (not this case's blocker): evaluate default-on for
`listing` + `noreturn_disc`. They are the Ghidra "Non-Returning Functions - Discovered"
analog and here close a real 11-point structural deficit vs both ghidra (0) and ida (7).
Gate per policy: 0/675 datatest parity + `make test-stages` + <=5% speed — the listing tier
is a program-wide recursive-descent disassembly, so the speed budget is the likely blocker
on large binaries; if it exceeds budget, keep default-off but add the pair to the decbench
run options (the benchmark path is exactly where discovered-noreturn pays).
