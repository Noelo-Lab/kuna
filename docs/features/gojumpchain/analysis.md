# Forwarded Go write barriers

The pinned angr `TestBuiltinsGo122Unoptimized` test expects compiler write-barrier
helpers to disappear from unoptimized Go output. In its `builtins_N` fixture,
Kuna's `main.push` retains two `runtime.gcWriteBarrier2` calls and their buffer
writes. The actual application stores are `n.next = head` and `head = n`.

Raw p-code before SSA shows why: both successors of each flag test contain a
single unconditional jump before reaching the barrier call or the shared store.
The existing `gowritebarrier` matcher requires a direct edge. Ordinary later
branch cleanup exposes the shape after the pass's pre-SSA window has closed.

The change normalizes only jump-only forwarding blocks around a recognized
single-call barrier diamond. The existing barrier option and matcher still own
removal of the runtime bookkeeping. Blocks with side effects are never treated
as forwarding blocks. A broader prototype also changed unrelated runtime
control-flow formatting, so it was narrowed before shipping.

This covers one compiler layout. Go 1.18's register-named barriers use a different
full-diamond protocol and remain separate work. Go source syntax, types and
builtin recovery are also outside this feature.

## Breadth and speed

The final matcher was compared across all 6,533 functions in three pinned angr
fixtures. It changes 25 of 1,998 functions in Go 1.22.5 `builtins_N`, removing
77 barrier calls. All other recognized direct callee counts are unchanged.
The common-join application stores remain, including error-interface globals,
pool and file fields, and both stores in `main.push`. The 25 function diffs and
hunk classifications are checked in.

The optimized Go 1.22.5 fixture (1,996 functions) and Go 1.27.1 fixture (2,539
functions) are byte-identical with the option off and on. All six whole-binary
runs completed without decompilation errors.

A 21-pair interleaved CLI measurement after warmup measured 554.34 ms off and
550.83 ms on (-0.63%), within the 5% budget. These wall-clock numbers include
image loading; raw samples are recorded rather than claiming a speedup.
