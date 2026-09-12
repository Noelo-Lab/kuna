# entryretdispatch — recover stack-directed calls at a function entry

## Concrete divergence

The i386 `bm3.exe` entry at `0x401757` is a 105-byte dispatcher built from
repeated `push <continuation>; push <callee>; ret` links. A RET consumes the
callee address, the callee returns to the pushed continuation, and the next
link repeats. Default kuna stopped at the first RET and emitted only `return;`,
losing calls including `LoadLibraryA` and `GetProcAddress`.

The existing explicit `flow 0x40176f call` assertion proved the rest of the
pipeline already had the required target and stack information: after the
continuation-chain fix in #504, that one assertion recovered the complete
dispatcher. The missing decision was whether a RET at an unasserted function
entry is a genuine return or a stack-directed call.

## Owning phase

This is P2 `flow-classification`. The decision must be made before the function
is followed because a normal RETURN has no successor. The shared decompile step
therefore derives CALL flow facts from the same bounded raw-p-code recognizer
used by explicit RET-call-chain propagation, gated by `entryretdispatch`.

## Proof boundary

A link is accepted only when raw-p-code provenance proves both of these facts:

- RETURN's destination was loaded from a slot written during this straight-line
  run; and
- the adjacent slot contains this RET instruction's exact fall-through address.

Affine constants are sign-extended from their p-code operand width. LOAD and
STORE facts include the exact p-code input-0 memory-space ID, so numerically
equal affine pointers in distinct spaces cannot pair. Overlapping stores and
overlapping register writes invalidate facts, so a write through `eax-4` and a
partial write to `sp` cannot leave stale continuation evidence.
The scan stops at conditional control flow because one scanned fall-through path
cannot prove its store pair dominates a later RET. Calls, indirect control flow,
opaque stack writes, unknown-pointer stores, unsupported `CALLOTHER` userops,
decode failures and the existing instruction/site caps also decline.

Explicit flow assertions remain authoritative. A caller-supplied classification
at a derived site suppresses the automatic CALL; an explicit `flow ... return`
is treated as a clean veto and is not redundantly applied to an instruction
that already is RETURN.

## Controls and blast radius

The in-repo i386 fixture contains the three-link positive plus ordinary RET,
RET-immediate, copied incoming-return-address, unrelated computed/constant RET,
discarded-continuation and unrelated-store controls. Three adversarial controls
cover the review boundary exactly: a negative displacement overwrites the
continuation and would execute the helper twice, a conditional path bypasses
the store pair, and a partial SP write invalidates ESP-derived facts.
Recognizer unit controls additionally put the target and continuation in
different p-code memory spaces and place an output-less `CALLOTHER` between
setup and RET; both must decline.

The option-off arm restores the previous empty return. The 675-assertion
datatest corpus is byte-identical with the default enabled; the dedicated
two-pass stage test and CLI tests cover the output-changing witness.
