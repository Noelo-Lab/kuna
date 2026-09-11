# call-pop-pointer-helper analysis

The round-12 witness at `0x4f7036` is a pointer-return helper, but kuna follows
its CALL fall-through into a NUL-separated import-name table and emits three
`in(...)` operations plus writes through undefined registers. The bytes establish
the real control flow: `call 0x4f708d` pushes `0x4f703b`; the callee executes
`pop eax; inc eax; ret`, leaving `eax = 0x4f703c` and returning through the word
above the discarded address. Control never reaches `0x4f703b`.

This is not a section-permission problem: the packer's code and inline data share
one section whose executable flag is clear. It is also not covered by
`calltrampoline`; that option owns the sibling shape which discards the address
and ends in a direct jump back into the caller. The existing `callreturn` flow
override prevents the garbage decode but loses the pointer result.

The owning decision is P2 `flow-classification`, at the direct-CALL arm of
`FlowInfo::xref_control_flow`. A bounded decode of the callee can prove both
facts needed for a safe rewrite: the stack pointer passes through
`entrySP + pointer-size`, and the terminal RETURN uses the untouched word loaded
from that exact slot. Rewriting that CALL to BRANCH admits the callee's p-code
into the helper's graph, where existing stack and value propagation recovers the
pointer without a new dataflow rule.

IDA reference output was unavailable because the reference server exited before
registering. The comparison therefore uses the byte-level semantics and the
existing manual flow override as its reference; both agree that the fall-through
is unreachable, while only the branch rewrite preserves EAX's value.
