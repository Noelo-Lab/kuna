---
case_id: O2-openssh-portable-ssh-keyscan-sshfatal
status: already-fixed
gap_survives: false
recorded_kuna_ged: 132
divergence_lever: null
proposed_new_option: null
runtime_choice: false
angr_feature: null
---
## Current kuna vs ghidra

Current kuna (`decompile-all --addr 0x9d00`, F1 listing default ON):
```c
void sub_9d00(void)

{
  unsigned long v1; // stack + 0x8
  
  sub_28100();
                    /* WARNING: Subroutine does not return */
  sub_239f0(0xff);
}
```
LOC 5, gotos 0, ifs 0, loops 0, switches 0, calls 2, one "does not return" warning.

Ghidra:
```c
void sshfatal(void)

{
  FUN_00128100();
                    /* WARNING: Subroutine does not return */
  FUN_001239f0(0xff);
}
```
LOC 4, gotos 0, ifs 0, loops 0, switches 0, calls 2, one "does not return" warning.

The two are structurally identical: `sshlogv`-equivalent call, then the noreturn
`cleanup_exit(255)` (`sub_239f0(0xff)` / `FUN_001239f0(0xff)`) tail-called after the
`/* WARNING: Subroutine does not return */` comment. Function/callee names differ only in
symbolication (`sub_*` vs `FUN_*`), which GED normalizes. The one residual delta is kuna's
extra unused `unsigned long v1; // stack + 0x8` declaration (a spurious stack slot) — worth
~1 GED node, nowhere near 132. Effective current GED ≈ 0–2.

## Divergence experiment

This case is already-fixed, so no lever needed to *close* the gap — but I ran the reverse to
confirm the mechanism. Flipping the F1 default off (`--option noreturn_propagate off`)
re-opens the entire gap: kuna stops treating `sub_239f0` as no-return, falls through past the
`cleanup_exit(255)` tail, and swallows the *following* functions (`confree`, `keyprint_one`,
…) into one giant body — ~150 LOC, multiple nested `if`/`else`, a `while (strsep(...))` loop,
two `tailcalljump` recovered tail-calls, and even a bogus recursive `sub_9d00(...)` call. That
inflated shape is precisely what the OLD kuna 0.1.0 emitted and what scored the recorded
GED 132.

- `noreturn_propagate off` -> reverts to the 150-LOC swallowed body (reproduces the old gap).
- default (`noreturn_propagate on` via F1 listing) -> the 5-LOC body matching ghidra.

No *other* lever is relevant; this is a single-mechanism case.

## Analysis / runtime-choice verdict

Root cause: the recorded 132 was produced by pre-session-1 kuna 0.1.0, where `decompile-all`
did NOT enable the "listing" and therefore `noreturn_propagate` never concluded that
`sub_239f0` (`cleanup_exit`, invoked with `0xff` = exit status 255) does not return. Without
that fact the flow-follower ran off the end of the function and merged the adjacent code
(`confree` and friends) into `sshfatal`, exploding the node count.

Session-1's **F1** change (decompile-all now enables the listing, so `noreturn_propagate` fires
on the benchmark surface) closes this: the no-return conclusion truncates the body right after
the tail call, and kuna now emits the same two-call stub as ghidra, with the same
"Subroutine does not return" annotation. This is a clean **already-fixed** case — the gap does
not survive on current main. Not a runtime choice and not a bug; the only remaining nit is a
harmless extra unused local declaration that does not move the metric meaningfully.
