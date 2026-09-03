# calleearity — one callee called twice, two different argument lists

Route: RE-friction loop (`docs/re-pipeline.md`), round 1, challenge
`69a54bd70f5b9757a6a5f72f` — PE x86-64 `PolyMLP.exe`.
Track: `quality`.

## The observation, as filed

> **PE decompilation loses allocator arguments inside the model loader**
> (`wrong-output` / `major`)
> The same decompilation identifies `sub_140008160(0x28)`, `sub_140008160(v33)`
> and other sized calls, but renders several paths as `sub_140008160()` with no
> argument, destroying the buffer-size dataflow.
> probe: `kuna decompile {{BIN}} 0x1400024a0 --addr` contains `sub_140008160();`
> acceptance: same command does **not** contain `sub_140008160();`

Reproduced on a fresh build of `fca7a971`:

```
 79:  v13 = sub_140008160(0x28);            0x14000252c
127:      v13 = sub_140008160();            0x1400026b0   <- the size is gone
145:      v39 = (char *)sub_140008160(v33); 0x1400026cf
435:          v16 = (long long *)sub_140008160(0x28);
```

The filed hypothesis — *"prototype recovery is inconsistent across call sites of the
same internal allocator wrapper"* — is correct, and unusually so: it names both the
symptom and the fix.

## What the code actually does

`0x1400026b0` is MSVC's aligned `operator new[]`, where the requested size is also the
operand of an overflow check:

```
  0x14000269a: cmp  rbx, 0x1000
  0x1400026a1: jb   0x1400026cc          ; the small path -> mov rcx,rbx; call
  0x1400026a3: lea  rcx, [rbx + 0x27]    ; the argument
  0x1400026a7: cmp  rcx, rbx             ; the overflow guard
  0x1400026aa: jbe  0x140002fad          ; ... which BRANCHES on it
  0x1400026b0: call 0x140008160
```

`FuncCallSpecs::check_input_trial_use` reaches `Funcdata::ancestor_op_use` ->
`Funcdata::only_op_use` for the `rcx` trial, and `only_op_use` walks every descendant
of the argument Varnode and rejects the trial on the first `CPUI_BRANCH` /
`CPUI_CBRANCH` it reaches. Instrumented:

```
ONLYUSE reject match=0x1400026b0 slot=2 at op 0x1400026aa CPUI_CBRANCH
```

The trial is then `mark_inactive`d, which also sets CHECKED, so nothing re-scores it,
and the argument is dropped. The neighbouring site at `0x1400026cf` (`mov rcx,rbx;
call`) has no branch on `rcx`, so it keeps its argument. Hence two arities for one
callee, thirty bytes apart.

This is upstream Ghidra's rule verbatim, and it is not obviously wrong on its own
terms: a register that is compared and branched on may well be live caller state that
happens to survive into the call. Relaxing it — tolerating a CBRANCH reached only
through comparison/boolean ops — was considered and rejected: `test rcx,rcx; jz;
call` is one of the most common shapes in any x86-64 binary and is structurally
identical to the witness, so the relaxation would fabricate an argument at each one.
Local dataflow cannot separate the two cases. **Only the callee can**, and the caller
already contains the evidence: three other call sites to `0x140008160` that do pass a
register argument.

## The owning decision

P4 (calls & prototypes), sub-stage **trial-finalization**, in
`FuncCallSpecs::build_input_from_trials`
(`decompiler/crates/kuna-decomp/src/p4_calls/funcdata_callsite.rs`) — the point where
`deriveInputMap` has set the `used` flags and the CALL op's final input list has not
been written yet, so both the trial map and the argument Varnodes are still in hand.
Spec chapter: `docs/spec/04-calls-and-prototypes.md` 4.2.

## The limit this design accepts

`ActionActiveParam` finalizes each call spec as soon as *that* spec is fully checked,
walking `qlst` in order, so at `build_input_from_trials` time the only siblings whose
argument list is final are the ones **earlier** in the function. A call site is
therefore reconciled against the sites before it, never after. In this witness that
is enough (`0x14000252c` precedes `0x1400026b0`); in general, a callee whose *first*
call site is the broken one is not helped.

Making the direction symmetric means deferring every finalization until every spec is
checked, which changes what `Funcdata::check_call_double_use` sees while it is still
scoring — a behaviour change in trial scoring itself, not just in finalization. That
is a bigger option than this one and is recorded here rather than smuggled in.
