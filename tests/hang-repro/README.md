# kuna hang repro — infinite loop on fully-stripped x86-64 ELF binaries

**Status:** **FIXED** (convergence bug root-caused and fixed); **watchdog shipped**
and kept as a defensive bound. Without a watchdog, `kuna decompile-all` hung
**indefinitely** (100% CPU, never returns) on certain fully-stripped x86-64 ELF
binaries. Discovered 2026-07-02 while running kuna as a backend in the `decbench`
decompiler benchmark. `decompile-all` carries a per-function watchdog
(`--max-fn-seconds N`, default 120, `0` disables). The non-convergence itself is
fixed: the `loweredswitch` post-heritage repair
(`Funcdata::kuna_repair_lowered_switch_inputs`) mis-classified the constant that
`ActionConditionalConst` legitimately installed on the synthetic BRANCHIND as a
broken input (its healthy test was `is_written()||is_input()`, and a constant is
neither) and re-pointed the BRANCHIND at the register def on every heritage pass —
so condconst re-replaced it on every repeatapply `mainloop` iteration, forever. The
fix accepts heritage-known Varnodes (C++ `Varnode::isHeritageKnown()` =
insert|constant|annotation, `varnode.hh:299`) as healthy. The repro below now
converges in well under a second and is pinned by the regression test named at the
bottom.

**Diagnosis so far:** the hang is ONE pathological function's decompile pipeline, not
discovery — on `ssh-sk-helper` all 843 functions enumerate fine, but `sub_1bd04` @
`0x1bd04` (only 99 instructions) never converges: the pipeline cycles forever between
heritage (`Heritage::heritage` → `LocationMap::add`) and dead-code elimination
(`ActionDeadCode::apply`) under `ActionRestartGroup::apply`. Single-function repro:

```bash
timeout 90 decompiler/target/release/kuna decompile-all \
    tests/hang-repro/ssh-sk-helper --addr 0x1bd04 --json --max-fn-seconds 0
# BEFORE the fix: exit 124 (hang) with the watchdog disabled; with the default
# watchdog it returned in ~120s with the per-function budget error.
# AFTER the fix: converges in well under a second, `code` non-null, `error` null.
```

## Symptom

`kuna decompile-all <binary> --json` (pre-watchdog, or with `--max-fn-seconds 0`)
never terminates and never emits output. It pins one core at 100% CPU. In the
benchmark, when the parent harness killed its worker after a 300s budget, the kuna
process **orphaned and kept spinning** — we observed **9 orphaned kuna processes each
burning ~4h16m of CPU** before they were manually killed. At the time kuna had no
internal watchdog for this, so it had to be killed externally (and its process
*group* reaped, or it orphaned). The `--max-fn-seconds` watchdog (default 120s) now
bounds this per function.

## Reproduce

Two of the smallest offending binaries are checked in next to this file (fully
stripped: no `.symtab`, no DWARF; x86-64 PIE; from openssh-portable built at `-O0`):

| file | size | sha256 (first 16) |
|---|---|---|
| `ssh-sk-helper`     | 461448 | `e03a07c44cc41d6f…` |
| `ssh-pkcs11-helper` | 498312 | `b18d309908860eea…` |

```bash
# from the kuna repo root, after `make` (binary at decompiler/target/release/kuna)
timeout 120 decompiler/target/release/kuna decompile-all \
    tests/hang-repro/ssh-sk-helper --json
# BEFORE the fix (bug): timed out at 120s (exit 124), no JSON, 100% CPU throughout.
# AFTER the fix: whole-binary JSON in seconds.
```

Confirmed pre-fix: kuna ran **>120s with no return** on `ssh-sk-helper`. Same for
`ssh-pkcs11-helper` (its `sub_21e9b` was the same bug — post-fix it decompiles in
~0.2s).

## Full list of known-hanging inputs (from the benchmark)

All are **fully stripped, x86-64 ELF PIE**. openssh-portable @ `-O0`:
`ssh`, `sshd`, `ssh-add`, `ssh-agent`, `ssh-keygen`, `ssh-keyscan`, `ssh-keysign`,
`ssh-pkcs11-helper`, `ssh-sk-helper`. Also: `bash` @ `-O2`.

The two smallest (`ssh-sk-helper`, `ssh-pkcs11-helper`) are checked in here; the rest
follow the same pattern (large, stripped openssh/bash binaries).

## Root cause (as fixed)

- The trigger was **stripped input** (no symbols → kuna's own function discovery)
  *plus* the kuna-owned `loweredswitch` feature: it synthesizes a `BRANCHIND` +
  trivial `JumpTable` out of a comparison cascade, and its post-heritage repair
  hook re-pointed the BRANCHIND at the register def whenever the input was not
  `is_written()||is_input()` — which wrongly included the **constant** that
  `ActionConditionalConst` installs when the cascade's own guard proves the switch
  variable constant down the dominating edge.
- The gdb symptom (cycling `Heritage::heritage` → `LocationMap::add` /
  `ActionDeadCode::apply` under `ActionRestartGroup::apply`) was the repeatapply
  `mainloop` re-running every iteration off condconst's eternally-counted change;
  no restart was ever requested, so no restart bound could trip.
- The fix (one predicate, `substrate/funcdata_block.rs`): the repair also accepts
  heritage-known Varnodes (C++ `Varnode::isHeritageKnown()`, `varnode.hh:299`).
- The per-function **watchdog** (`--max-fn-seconds`, cooperative deadline probes at
  the action/rule-pool/heritage loop boundaries) remains as a defensive bound.
  Regression tests: `decompiler/crates/kuna-cli/tests/decompile_all_cli.rs`
  (`decompile_all_converges_on_past_pathological_function`) pins the convergence;
  `kuna-decomp`'s `repeatapply_deadline_bounds_nonconverging_action` unit test pins
  the watchdog mechanism deterministically.

Provenance: decbench `projects/sailr/openssh-portable.toml` and `bash.toml`,
decompiled from the stripped copy the benchmark hands each decompiler.
