## What was broken

RE-friction round 1, challenge `69a54bd70f5b9757a6a5f72f` (PE x86-64
`PolyMLP.exe`), observation *"PE decompilation loses allocator arguments inside
the model loader"*:

> The same decompilation identifies `sub_140008160(0x28)`, `sub_140008160(v33)`
> and other sized calls, but renders several paths as `sub_140008160()` with no
> argument, destroying the buffer-size dataflow.

Its acceptance probe — `kuna decompile <bin> 0x1400024a0 --addr` must not contain
`sub_140008160();` — **now passes on the default build**.

## The mechanism

The site that loses its argument is MSVC's aligned `operator new[]`, where the
requested size is also the operand of an overflow check
(`lea rcx,[rbx+0x27]; cmp rcx,rbx; jbe abort; call`). `Funcdata::onlyOpUse`
rejects a trial whose Varnode has a `CPUI_CBRANCH` among its descendants, and
`checkInputTrialUse` marks the trial inactive — permanently.

Relaxing that rejection was designed and **rejected on the refutation question**:
`mov rcx,[rbx]; test rcx,rcx; je skip; call foo` is structurally identical and
ubiquitous, so it would fabricate an argument at each one. No local dataflow
property separates the two. The callee does — and the caller already records the
callee's opinion at its other call sites.

`calleearity` reads a finalized sibling call's **register** argument storage and
promotes this call's matching trials. The storage cannot be read off the sibling
CALL op's inputs (those carry argument *values*; at `0x14000252c` the argument is
the constant `0x28`), so `buildInputFromTrials` records it on the call spec one
statement before the trials are deleted.

Root cause: [`analysis.md`](analysis.md) · design and rejected alternatives:
[`plan.md`](plan.md) · corpus evidence: [`sweep.md`](sweep.md).

## The option

`option calleearity on|off` (P4, `trial-finalization`, tier `core`,
`change_kind = correctness-fix`, ElementId 4133, default **on**, DIV-102).

## Ablation, sweep, gates

- **Ablation**: `kuna test --datatests --baseline docs/baseline.json` →
  **675/675, PARITY OK**, 0 assertions changed. `docs/baseline.json` untouched.
- **Sweep** (standing requirement 7): 46 binaries / 3857 functions → **25 change
  (0.65%)**, all classified, no regression found. The first cut ("same callee,
  same arity") changed 43 and two of the extra 18 were wrong — `Sleep(200,0)` and
  an internal variadic logger gaining two arguments — which is where the
  "only a call that recovered nothing" condition comes from.
- **Stage test**: `tests/stages/kuna-calleearity.xml`, two passes over an
  assembled SysV x86-64 fixture.
- **Speed**: flat (+0.50% against a +0.45% null-control floor on the witness),
  measured on interleaved child CPU time because the box was loaded.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
