# Call-result folding duplicates one machine call

The witness calls `helper` once, narrows its return to sixteen bits, compares
that value throughout an inner loop, and stores the same value after the loop.
With `foldcallret` enabled, kuna made the direct call output implied because it
had one descendant (`SUBPIECE`). The derived low-word value had two descendants,
but ordinary multiplier accounting considered the expression cheap enough to
print twice. The resulting C called `helper()` in both sinks.

The filed diagnosis is upheld, with one refinement: the unsafe fan-out need not
cross a basic-block edge. A same-block value such as `u = f() & 0xffff`, used by
two stores, had the same duplication. A CFG-only guard therefore did not enforce
the option's documented single-evaluation contract.

The repair keeps a multi-use root explicit whenever the expression the printer
would recursively emit reaches a call output that passed `foldcallret`'s fold
predicate. The walk follows the same printable operands as multiplier analysis:
LOAD slot 1, PTRADD base only, SEGMENTOP slot 2, and all operands for other ops.
It stops at an already-explicit input because that input already supplies the
single textual evaluation point.

This is a correctness repair to the existing default-on `foldcallret` option,
not a new judgment call. Turning the option off already avoided the bug but also
discarded its safe single-use folding behavior.

The whole-corpus evidence and every changed-function classification are in
[`sweep.md`](sweep.md); the corresponding raw hunks are in
[`sweep.diff`](sweep.diff).
