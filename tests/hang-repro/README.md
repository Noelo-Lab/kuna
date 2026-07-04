# kuna hang repro — infinite loop on fully-stripped x86-64 ELF binaries

**Status:** underlying convergence bug OPEN; **watchdog shipped**. Without a watchdog,
`kuna decompile-all` hung **indefinitely** (100% CPU, never returns) on certain
fully-stripped x86-64 ELF binaries. Discovered 2026-07-02 while running kuna as a
backend in the `decbench` decompiler benchmark. `decompile-all` now carries a
per-function watchdog (`--max-fn-seconds N`, default 120, `0` disables): the
pathological function is cut off and recorded as its own `error` record
(`"per-function decompile budget exceeded"`) and the batch completes. The
non-convergence itself is still unfixed — the repro below is the testbed for that fix.

**Diagnosis so far:** the hang is ONE pathological function's decompile pipeline, not
discovery — on `ssh-sk-helper` all 843 functions enumerate fine, but `sub_1bd04` @
`0x1bd04` (only 99 instructions) never converges: the pipeline cycles forever between
heritage (`Heritage::heritage` → `LocationMap::add`) and dead-code elimination
(`ActionDeadCode::apply`) under `ActionRestartGroup::apply`. Single-function repro:

```bash
timeout 90 decompiler/target/release/kuna decompile-all \
    tests/hang-repro/ssh-sk-helper --addr 0x1bd04 --json --max-fn-seconds 0
# exit 124 (hang) — with the watchdog disabled.  With the default watchdog it
# returns in ~120s with `"error": "per-function decompile budget exceeded (120 s)"`.
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
# EXPECTED (bug): times out at 120s (exit 124), no JSON produced, 100% CPU throughout.
# A healthy binary of this size returns in seconds.
```

Confirmed: kuna ran **>120s with no return** on `ssh-sk-helper`. Same for
`ssh-pkcs11-helper`.

## Full list of known-hanging inputs (from the benchmark)

All are **fully stripped, x86-64 ELF PIE**. openssh-portable @ `-O0`:
`ssh`, `sshd`, `ssh-add`, `ssh-agent`, `ssh-keygen`, `ssh-keyscan`, `ssh-keysign`,
`ssh-pkcs11-helper`, `ssh-sk-helper`. Also: `bash` @ `-O2`.

The two smallest (`ssh-sk-helper`, `ssh-pkcs11-helper`) are checked in here; the rest
follow the same pattern (large, stripped openssh/bash binaries).

## Notes for whoever fixes the underlying bug

- The trigger appears to be **stripped input** (no symbols → kuna does its own
  function discovery/CFG recovery). The *unstripped* variants of the same programs
  did **not** hang in the benchmark; only the stripped ones did.
- It's an **infinite loop**, not merely slow: 4+ hours at 100% CPU with no progress.
- Localized by gdb backtrace sampling to `sub_1bd04` @ `0x1bd04` in `ssh-sk-helper`
  (99 instructions): the action pipeline cycles between
  `s3_dataflow::heritage::Heritage::heritage` → `LocationMap::add` and
  `s9_emit::coreaction_render::ActionDeadCode::apply`, under
  `ActionRestartGroup::apply` inside `decompile_func_full_with_override_dyn`
  (`infra/decompile_drive.rs`) — a heritage/dead-code fixpoint that never converges.
- The per-function **watchdog** (`--max-fn-seconds`, cooperative deadline probes at
  the action/rule-pool/heritage loop boundaries) makes the batch survive; the real
  fix is making that fixpoint converge. Regression test:
  `decompiler/crates/kuna-cli/tests/decompile_all_cli.rs`
  (`decompile_all_watchdog_bounds_pathological_function`).

Provenance: decbench `projects/sailr/openssh-portable.toml` and `bash.toml`,
decompiled from the stripped copy the benchmark hands each decompiler.
