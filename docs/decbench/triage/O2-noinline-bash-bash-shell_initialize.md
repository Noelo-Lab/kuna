---
case_id: O2-noinline-bash-bash-shell_initialize
pool: angr
group_id: bash::shell_initialize
status: already-fixed
tier: M
margin: 72
fresh_verdict: today's build scores GED 0 — kuna's CFG is *isomorphic* to the source (9 nodes / 12 edges, same as angr and ida). The recorded 72 was three `if (chain) v = 1; else v = 0;` boolean-materialization diamonds that `iteboolean` (DIV-51, PR #241) now folds into the short-circuit chain. Mode-independent (identical output under `--mode reliable`, `aggressive` and no-flag).
option_closing: null
feature_slug: null
scope: small
confidence: high
---

## Verify-first

```
$ ~/.virtualenvs/decbench/bin/python -m scripts.decbench.rescore \
      --case O2-noinline-bash-bash-shell_initialize
[rescore] METRIC-DRIFT: recorded 72 vs recomputed 0
 "ged_recorded": 72.0,  "ged_before": 0.0,  "ged_after": 0.0,
 "ged_perfect_after": true,  "ged_kuna_commit": "9623dc27"
```

Per-decompiler CFG probe on the stored artifacts (today's evaluator):

```
source shell.i:   9n/12e
kuna:   9n/12e  ged=0.0   iso=True    method=isomorphism
angr:   9n/12e  ged=0.0   iso=True
ida:    9n/12e  ged=0.0   iso=True
ghidra: 29n/41e ged=84.0  iso=False   method=vj_ged
binja:  11n/14e ged=12.0  iso=False
```

Mode sweep (bash O2-noinline stripped is 1.31 MB, so `auto` -> `reliable` = exactly the
benchmark's option surface):

```
$ kuna decompile-all .../O2-noinline/bash/stripped/bash --json --addr 0x34460 [<mode>]
[]                 loc 37  ifs 4  gotos 0
[--mode reliable]  loc 37  ifs 4  gotos 0
[--mode aggressive]loc 37  ifs 4  gotos 0
```

## Not a metric artifact — the code changed

`ghidra`, `ida`, `angr` and `binja` all reproduce their recorded values exactly, so the
2026-07-28 decbench GED overhaul (`e6e6b0f`, isomorphism early-out before `vj_ged`) is not
what moved kuna. Running the *old* evaluator (raw `cfgutils.similarity.vj_ged`, no
isomorphism early-out) on the stored kuna artifact also gives **0.0**:

```
$ vjprobe O2-noinline-bash-bash-shell_initialize kuna
source shell.i: 9n/12e
kuna: 9n/12e  iso=True  RAW_vj_ged=0.0
```

So kuna's *output* changed. (The `O2-noinline` kuna artifact in the results tree was
regenerated on Aug 1 — `mtime 2026-08-01 23:28` — which is why `triage`'s "stored" pane
already equals the fresh one. **The results tree is live user WIP and is being written
concurrently**; several `O0/*/decompiled/kuna_*.c` files carry `Aug 3 05:10` mtimes.
Never treat a "stored" pane as run-era without checking its mtime.)

## What the run-era output looked like

The `O0` bash artifact is still run-era (`mtime Jul 15`) and shows the defect on the same
function:

```c
  v1 = shell_is_restricted(shell_name);
  if (((privileged_mode) || (restricted)) || ((v1 || (dat_172330))))
    v4 = 1;
  else {
    v4 = 0;
  }
  initialize_shell_variables(shell_environment,v4);
  ...
  v2 = (((privileged_mode) || (restricted)) || ((v1 || (dat_172330))));   <- re-rolled
  initialize_shell_options(v2);
  if (((privileged_mode) || (restricted)) || ((v1 || (dat_172330))))
    v2 = 1;
  else {
    v2 = 0;
  }
  initialize_bashopts(v2);
```

Three identical 0/1 select diamonds; each un-rolled one adds 2 nodes / 2 edges to the CFG.
At `O2-noinline` all three were still if/else at run time (9 -> 15 nodes), which is the 72.

## Source (bash `shell.c`)

```c
  initialize_shell_variables (shell_environment,
      privileged_mode||restricted||should_be_restricted||running_setuid);
  initialize_job_control (jobs_m_flag);
  initialize_bash_input ();
  initialize_flags ();
  initialize_shell_options (privileged_mode||restricted||should_be_restricted||running_setuid);
  initialize_bashopts (privileged_mode||restricted||should_be_restricted||running_setuid);
```

Today kuna prints the chain inline at all three call sites, matching the source, angr and
IDA:

```c
  v1 = shell_is_restricted(shell_name);
  initialize_shell_variables(shell_environment,((privileged_mode || restricted) || dat_1409d4) || v1);
  initialize_job_control(jobs_m_flag);
  initialize_bash_input();
  initialize_flags();
  initialize_shell_options(((privileged_mode || restricted) || dat_1409d4) || v1);
  initialize_bashopts(((privileged_mode || restricted) || dat_1409d4) || v1);
```

## Analysis

Closed by **`iteboolean`** (P8, DIV-51 / PR #241 — "re-roll short-circuit 0/1 select
diamonds into a boolean assignment"), default-ON and mode-independent. Ghidra still scores
84 here, so this is a kuna-only win over its own ancestor.

**But the run-era O0 pane above is also a live witness of a different defect**: `iteboolean`
re-rolled only *one* of the three identical diamonds, and it still does today —

```
$ kuna decompile-all .../O0/bash/stripped/bash --json --addr 0x34684
  ... if (chain) v4 = 1; else { v4 = 0; }        <- missed
  ... v2 = (chain);                              <- re-rolled
  ... if (chain) v2 = 1; else { v2 = 0; }        <- missed
```

That is the `itecondlist` candidate filed from
`O0-betaflight-betaflight_STM32F405-mavlinkSendRCChannelsAndRSSI`; `iteboolean` shares
`leaf_bblock` with `iteregion`, so the same one-module fix covers it.

## Proposed fix

None for this case. Update `docs/decbench/backlog.md` rank #4 to closed on the next mine.
Campaign note: `bash::shell_initialize` is the second angr-pool row (after
`coreutils::factor`) whose margin was already paid by a shipped feature — always rescore
before triaging.
