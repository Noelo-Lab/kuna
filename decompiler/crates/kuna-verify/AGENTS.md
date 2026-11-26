# kuna-verify — agent notes

`kuna-verify` is a dev-only tool: it runs kuna's final IR and the machine code from one
state and compares them. Read `docs/features/diffverify/README.md` before changing it, and
`docs/features/diffverify/triage.md` before naming the cause of a `DIFFERS`. The root
`AGENTS.md` still applies.

## Rules

- **A `DIFFERS` is not a kuna bug until the harness is ruled out.** Most false ones so
  far were the checker's, not the decompiler's. Follow `triage.md`: the cause can be
  kuna, the SLEIGH spec, the reference emulator, the checker's evaluator or the harness.
- **Cross-check against an external tool wherever one can run the same thing.** The
  machine side and kuna's side share the p-code semantics and the SLEIGH specs, so an
  error there is the same on both and this check cannot see it. Run the instruction,
  function or input through an independent tool as well (Unicorn or QEMU, as Tier L does;
  real hardware; a compiler's own output) and compare. When they disagree, do not assume
  either side is right: decide whether the external tool, our evaluator or the spec is
  wrong, from the architecture manual or a third run, and write down which (`triage.md`).
- **A harness change must not move verdicts it did not mean to move.** Before committing,
  compare every verdict on the corpus, `ls` (`O2`, `O0`) and `find` against the previous
  build, and explain each one that moved.
- **`docs/baseline-verify.json` is re-recorded only for a reason the commit names** (new
  stage tests from main, a deliberate harness change), never to absorb a regression.
- **Fault injections into the engine stay local.** Plant them to test the check, then
  revert; mutants (`kuna-verify mutate`) change only kuna's side, never the machine side.
- **New work goes in its folder** (`README.md` §5): a comparison in `compare/`, a fact
  about calls in `abi/`, a tool in `tools/`, a later tier in a folder of its own. Keep
  the old paths re-exported from `lib.rs`.
- **Recheck decbench functions by address, or leave out names a binary defines twice.**
  Two functions of one binary can share a name, and `load function NAME` picks one.

## Regressions

One entry per trap an agent fell into here. Date, model and PR say when it was true; an
entry that no longer applies can be removed.

- 2026-11-24 · Claude Opus 5.5 · PR #1830
  Treated a `CALL_RETURN` flow override as "does not return". It also turns a branch
  into a tail call, so gh8471's test became a false `DIFFERS`. Only a `CALL_RETURN` on a
  call instruction is a no-return cut.
- 2026-11-24 · Claude Opus 5.5 · PR #1830
  Read `RDI` at every nested call to decide whether `error(status, ...)` returns. LiveIn
  tracking counted that read as the callee reading `RDI`, a false `DIFFERS` in
  `calleefloatreads`. Read the status only for the `error` names, with tracking off.
- 2026-11-24 · Claude Opus 5.5 · PR #1830
  Took every PLT thunk as an external call. That exposed an out-of-bounds frame pointer
  (`ls` `O0` `group_number`) the check cannot yet judge. Keep it to the no-return check
  until that has a design.
- 2026-11-24 · Claude Opus 5.5 · PR #1830
  Followed pointers stored in a passed frame object (transitive escape), which reached a
  `va_list`'s register save area. kuna's `varargsave` folds the stores that fill it, so
  about 160 variadic functions stopped agreeing. kuna's side now fills the area as
  `va_start` does, from `Funcdata::vararg_folded`.
- 2026-11-24 · Claude Opus 5.5 · PR #1830
  Merged a by-name decbench recheck into an `--all` run keyed by name. Functions sharing
  a name were mixed up, and four `libedit` functions looked fixed by a main merge when
  they were not.
- 2026-11-24 · Claude Opus 5.5 · PR #1830
  The first `drop-arg` mutant shortened the call and left the prototype, so the check
  caught it by construction and recall read too high. A mutant must leave kuna's facts
  as a real fault would.
- 2026-11-25 · Claude Opus 5.5 · PR #1830
  Stopped the machine at a no-return call kuna thinks returns, to catch the missed
  no-return. A callee that aborts only on some inputs then made `hash_insert` and others
  `DIFFERS`, though kuna's C calls the same callee and stops too. Code after a call that
  never returns is unreachable on both sides; missing that is not a semantic difference.
- 2026-11-25 · Claude Opus 5.5 · PR #1830
  Counted every argument suspect as a doubt that keeps a run from agreeing. A callee
  reading a register its declared prototype does not take (Go's context register) is not
  an argument kuna dropped; only a recovered prototype's suspect is a doubt.
- 2026-11-25 · Claude Opus 5.5 · PR #1830
  Compared a system call's arguments to the syscall registers by position. kuna's
  `x64syscall` passes only the registers the block provably sets, in order, so a pass
  that leaves one out became a false `DIFFERS` (`kuna-x64syscall` `pointer_before`).
  They are matched in order from the first argument register; a match only as a
  subsequence, with a register skipped in the middle, is `UNKNOWN`, since kuna's C then
  passes the later values in the wrong registers.
- 2026-11-26 · Claude Opus 5.5 · PR #1830
  Skipped every comparison of a run in which kuna's `memcpy` overlapped. A fault before
  the copy then read as the declared `copy-overlap`. A declared divergence cuts only
  what follows it; what came before is compared.
- 2026-11-26 · Claude Opus 5.5 · PR #1830
  Took a system-call register the machine never touched as matching any value. It
  holds its starting value, which the machine side now records at the call.
- 2026-11-26 · Claude Opus 5.5 · PR #1830
  Put back, after a callee run for real, every register the callee's prototype does
  not list as killed. The x86-64 cspec lists only RAX, RDX and XMM0, so RDI, RSI, RCX
  and R8-R11 came back too, and kuna's C keeping a value in R11 across a call that
  changes it agreed (`ghangr-rust-probestack` `probe_observes_r11`). Only the model's
  `unaffected` registers are put back now. A first try that instead stopped putting
  anything back moved verdicts for that same reason; read the cspec lists before
  blaming the rerun.

- 2026-11-26 · Claude Opus 5.5 · PR #1830
  Took a top-level `RETURN` to an address other than the run's return address as a
  return when the target was unmapped or kuna made no branch there. A function that
  returns past its return address then agreed. Such a return is `UNKNOWN` now, but only
  in a run that made no call: with calls, a stub's stack effect can move the stack
  pointer (Go, `kuna-gobounds`), and refusing those broke pinned verdicts. Do not "fix"
  the runs this leaves by stubbing every indirect call to a non-entry: that also stubs
  real pointers the image holds and import slots (README §9).
