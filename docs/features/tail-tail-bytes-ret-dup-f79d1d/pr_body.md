## noreturn_extern: bound functions at undefined-extern no-return calls (opt-in)

Closes the gap angr's `test_tail_tail_bytes_ret_dup` exposes on `tail.o::tail_bytes`
(an ELF **relocatable object**). Full write-up:
[`docs/features/tail-tail-bytes-ret-dup-f79d1d/analysis.md`](../docs/features/tail-tail-bytes-ret-dup-f79d1d/analysis.md).

### The gap

kuna decompiled `tail_bytes` (a 615-byte function, `0x401e80..0x4020e7`) as **326 lines /
9 gotos**, running far past the function's real end and swallowing several adjacent
functions (`tail_lines`, a `fstatfs`/`die_pipe` function, an `lstat` loop, …), even
synthesizing a bogus `do { … } while (…)` outer loop. angr renders **96 lines / 1 goto**.

### Root cause

In a `.o`, `__stack_chk_fail` is an **undefined external** symbol (`UND`, `NOTYPE`, size 0).
kuna's analysis-tier known-no-return pass (`noreturn_known`, default on) keys its facts on
the **address** of a *defined* `FUNC` symbol, so it never marks the UND extern — even though
its base name is on the shipped ELF no-return list. At decompile flow time the call resolves
to a display name but the symbol's no-return flag is false, so
`FlowEnvironment::query_call_no_return` returns false, no `artificialHalt(noreturn)` is
planted, and flow runs off the function's end into the next function.

### Mechanism

A name-based fallback in the flow seam `ArchFlowEnv::query_call_no_return`
(`infra/decompile_drive.rs`): when the address-keyed check is false **and** the gate is on,
resolve the callee display name and return true if `matches_noreturn_extern_name` matches the
known ELF no-return list (leading-`_` strip + `std`-only namespace guard, exact match against
a closed set — mirrors the analysis-tier `name_matches`). This plants the existing artificial
halt, so flow stops at the call and the function is bounded correctly. New module
`kuna_noreturnextern.rs` (ElementId 4103), Architecture flag `noreturn_extern_calls` modelled
on the `tail_call_jumps` flow-seam gate. No new pass type, no S7 change.

### Option

`option noreturn_extern on|off` (S2 flow-classification, **default-off opt-in**).
- `kuna decompile tail.o tail_bytes --option noreturn_extern on` → **326 → 87 lines**.
- default (off) keeps the old fall-through rendering (byte-identical to before this PR).

### Why opt-in (not default-on)

The 675-datatest ablation with the gate on is **byte-identical (0/675)** and the target is
**37% faster** with it — which first argued for default-on. But `make rust-test` caught a
real interaction the bytechunk-only corpus can't exercise:
`verify_multiformat_passes::pe_exit_eliminates_dead_code_via_noreturn_list` asserts that
`option noreturn_known off` restores the dead fall-through after a PE `exit`, yet the
name-based fallback *independently* catches `exit`, so default-on would silently change
PE/ELF `exit`-family handling under `noreturn_known off`. The name match overlaps
`noreturn_known`'s for defined/imported symbols, so the honest call is **opt-in** (no DIV) —
like `tailcalljump`/`gotoreduce`/`stackguard`. The fallback only fires when the address-keyed
check already returned false, so it never double-fires on an already-marked callee.

### Parity & speed

- **Speed: 37% faster** on the target when on (299 ms → 188 ms median over 5 runs) — it
  decodes less code.
- Gates: `make test` **PARITY OK 675/675**, `make test-stages` **192/192**,
  `kuna catalog --check` **OK**, `make rust-test` green (4 new unit tests).
- Test: `tests/stages/ghangr-noreturn_extern.xml` (two-pass: pass 1 `off` reproduces the bug,
  pass 2 `on` proves the fix); `docs/baseline-stages.json` 190 → 192. No DIV (default output
  unchanged).

🤖 Generated with [Claude Code](https://claude.com/claude-code)
