## What was broken

RE-friction round 1, challenge `68149b8a8f555589f353117c` (Mach-O arm64
`practice_2_rengr`), observation *"Call argument recovery drops scanf destination
and printf varargs"*:

> kuna emitted `scanf("%d")` while subsequently comparing an apparently
> uninitialized `v3`, and emitted three `printf` calls containing `%s` with no
> corresponding argument.

All five variadic calls in `_main` lose their stack-passed arguments, and because
a dropped argument leaves its producing store with no consumer, dead-code
elimination reaps that too: `_main` ends up *reading a local nothing writes*.

## The mechanism

The arguments are recovered and then thrown away. `check_input_trial_use` scores
the `[sp+0]` trial ACTIVE; `ParamListStandard::fillinMap` deactivates it again,
because on AArch64 `x0`–`x7` and the outgoing stack area are ONE resource section
and its two positional rules — `forceNoUse` and `forceInactiveChain`
(`maxchain = 2`) — read the seven registers Apple's arm64 ABI structurally leaves
empty between the fixed parameter and the first vararg as evidence that the
argument list ended.

`varargstackargs` cuts such a section in two at its first stack trial, so the
register prefix and the stack tail are scored independently. Two conditions keep
the cut honest: the callee's prototype must be variadic (`FuncProto::isDotdotdot`
— for a known prototype a register hole IS evidence), and the section's stack area
must begin at callee-relative offset 0 (see the sweep below). Trial scoring itself
is untouched: a stack trial still has to reach `fillinMap` active on its own
evidence, so the option can keep an argument the recovery already believed in but
never invent one.

Full root-cause with the instrumented `fillinMap` trace:
[`docs/features/varargstackargs/analysis.md`](analysis.md) · design:
[`plan.md`](plan.md) · corpus evidence: [`sweep.md`](sweep.md).

## The option

`option varargstackargs on|off` (P4, `active-input-trial-scoring`, tier `core`,
`change_kind = correctness-fix`, ElementId 4132, default **on**, DIV-101).

## What it does not close

The observation's acceptance string is the literal `scanf("%d",&v3)`. The
behaviour is fixed — the destination is recovered and the local it writes is no
longer read-before-written — but recovering it also un-deletes two stores that had
been dead-code eliminated as collateral, and those declare ahead of the `w0`
temporary, so the destination prints as `&v2`. The probe no longer passes; the
acceptance string does not match.

## Ablation, sweep, gates

- **Ablation**: `kuna test --datatests --baseline docs/baseline.json`
  → **675/675, PARITY OK**, 0 assertions changed. `docs/baseline.json` untouched.
- **Sweep** (standing requirement 7): 46 binaries / 3857 functions across
  PE x86·x86-64·ARM, ELF x86·x86-64·ARM·ARM64·MIPS·SPARC·PPC64, Mach-O
  x86-64·PPC·ARM64 — **1 function changes**, the witness, every hunk a recovery.
  An earlier revision without the offset-0 condition changed 2, the second being a
  MIPS o32 `$gp` save slot mistaken for a `printf` argument; that counter-example
  is what the second condition buys.
- **Stage test**: `tests/stages/kuna-varargstackargs.xml`, two passes over a
  hand-assembled AArch64 fixture — pass 1 (`off`) asserts the bug, pass 2 (`on`)
  the fix.
- **Speed**: flat (+0.06% against a −0.76% null-control floor on the witness;
  −0.84% on `decompile-all iOSCrackme`), measured on interleaved child CPU time
  because the box was loaded.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
