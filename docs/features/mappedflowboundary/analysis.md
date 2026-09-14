# Mapped ELF x86 flow boundaries

A synthetic ELF entry tests EAX, returns 7 on one path, and ends the other
path after `mov ebx,5`. Legacy flow decodes staged zero padding beyond the
mapped extent, eventually raises `Unable to load 512 bytes`, and loses the
valid return. JSON reports null code and exit 1. Both ELF32 and ELF64 reproduce.
Explicit unmapped branches already have missing-halt recovery; fall-through
needs the same treatment using the authoritative map.

This is P2 flow classification. Recovery ends an effective fall-through whose
successor is unmapped, and an instruction that decoded flow reaches on a mapped
byte but whose encoding runs past the mapped run. The second shape is the real
one: images end in padding, so the last reachable instruction usually starts on
the final mapped byte. Actual instruction spans are validated before context
commits and p-code emission, so nothing from a truncated encoding is lifted; it
becomes a one-byte missing halt at its own address. A truncated entry
instruction, a failed read on a mapped byte and a genuine decode error remain
errors. The existing overlapbranch policy may discard a partial fall-through
when a length-only probe proves its queued target is a complete mapped
instruction with a different end; otherwise the partial stream is cut like any
other truncated instruction. In-lined callee flows take neither cut, because
the in-line clone turns a callee RETURN into a branch to the return address and
the missing halt would read as a normal return.
The image's live map, rather than executable flags or a cached loader probe,
distinguishes zero-filled RAM from unmapped padding. Matching linked ELF x86
metadata confines the new policy; ARM mode inference remains separate.

The missing halt is registered during discovery, so queued targets can resolve
through already decoded NOPs to the endpoint. Stub filling reuses that endpoint
if an explicit branch also queued it. A declared range is checked before mapped
recovery, preserving fatal, warning and ignore policies. The expected-error
stage runner collects output independently of the console's mutable redirect,
so closing it retains diagnostics and subsequent output without reopening it.
Rebuilding an eligible image refreshes its mapping warnings even after recovery
is disabled, while retaining unrelated warnings.
Cleanup and publication of buffered comments wait until replacement flow
succeeds; a failed reload preserves the selected body's existing diagnostics.

The registered input (`5ee1f28c`, SHA-256 `506fc3c8...890`) is a movfuscated
i386 keygenme: its whole 562,613-byte text is one function that ends
`mov cs,eax` followed by three zero alignment bytes before a gap to the data
segment. Its acceptance, `kuna decompile keygenme 0x804834c --addr --json`,
now exits 0 with a null error and about 1 MB of C that ends in the lifted
padding `add [eax],al` and `halt_missing()`; with the option off it still exits
1 at `r0x080d1955`. The size of that function is the program, not decoded
garbage. The generated fixtures reduce both tail shapes.
