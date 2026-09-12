# pushimmediateret — preserve a stack-built tail transfer

## Concrete divergence

The CryptoME i386 entry at `0x4f7001` executes `call 0x4f7990`, then
`push 0x40365a; ret`. The call unpacks the original entry point and the RET
transfers there. Default kuna emits only `sub_4f7990();`, so the terminal
transfer disappears. An explicit `flow 0x4f700b branch` proves the existing
flow, dataflow, and emitter paths already preserve the destination as
`(*(void *)0x40365a)();`.

The destination must remain an address rather than become a synthesized
function. Bytes at `0x40365a` are encrypted in the file and are rewritten only
by the preceding unpacker call; defining them statically would invent a body.

## Owning phase

This is P2 `flow-classification`. A raw RETURN has no successor, so the
decision must be made before following the function. The shared decompile step
can seed the existing BRANCH override only when a bounded raw-p-code walk proves
that RETURN pops an immediate value written to the current stack slot.

## Proof boundary

The classifier reuses the affine register and exact stack-store provenance used
by `entryretdispatch`. It accepts only a RETURN target loaded by that RETURN
from a slot written during the current straight-line run, whose last exact
store carries a constant and whose stack pointer advances by the loaded width.
An adjacent in-run stack store declines so the two-push call-emulation shape
continues to belong exclusively to `entryretdispatch`.

A normal return has no in-run target store. A pushed call argument is forgotten
at the call boundary. Stack adjustment or overwrite prevents the RETURN load
from resolving to the immediate store. Register-computed targets are not
constants. Conditional or indirect control flow ends the proof walk. Explicit
flow assertions at the derived RET site remain authoritative.

## Expected rendering

The automatic fact is `branch`, not `call`: `push immediate; ret` has no
fall-through and the original function never regains control. The ordinary
tail-call rendering may spell the branch as an indirect call statement in C,
but the P2 graph contains no call-return continuation. No target function is
created or decoded.
