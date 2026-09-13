# Mapped ELF x86 flow boundaries

A synthetic ELF entry tests EAX, returns 7 on one path, and ends the other
path after `mov ebx,5`. Legacy flow decodes staged zero padding beyond the
mapped extent, eventually raises `Unable to load 512 bytes`, and loses the
valid return. JSON reports null code and exit 1. Both ELF32 and ELF64 reproduce.
Explicit unmapped branches already have missing-halt recovery; fall-through
needs the same treatment using the authoritative map.

This is P2 flow classification. Recovery ends only an effective fall-through
whose successor is unmapped. Actual instruction spans are validated before
context commits and p-code emission; a partial instruction is still an error.
The existing overlapbranch policy may discard a partial fall-through only
when a length-only probe proves its queued target is a complete mapped
instruction with a different end. No p-code or context commits from the
discarded stream are applied; incomplete targets and option-off remain errors.
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

The original upstream acceptance input is unavailable locally. These source-
generated Apache-2.0 fixtures are behavioral reductions, not a claim that the
registered default-entry need is closed. Upstream must replay its exact
acceptance and wrong-ISA control before closure.
