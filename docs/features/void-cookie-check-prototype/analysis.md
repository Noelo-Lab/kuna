# Void `/GS` checker loses constant returns

The faithful PE witness sets EAX to `0` or `1` on three exits, then converges
through one MSVC `/GS` cookie check. With the checker asserted as locked `void`,
the existing `calleeretpreserves` handoff could preserve those definitions only
when the cookie recognizer proved the exact call site. One cookie value in this
function crosses a nested loop SCC made from multiple `MULTIEQUAL` operations,
so the recognizer missed every duplicated presentation of the shared epilogue
and the output returned an unassigned `vN // eax`.

The failure was not a traversal-depth limit. Raising the ordinary walk budget
from 512 and the value peel budget from 256 to 4096, separately and together,
left the output unchanged. The recursive fixed point returned `No` for an inner
phi whose inputs were all backedges relative to that inner invocation. That
premature negative prevented the outer invocation from reaching the real
scramble seed.

The repair propagates `Backedge` through such a pending inner phi. The top-level
proof still requires a concrete seed, so a seedless SCC fails closed. An unknown
entry or two concrete seeds with conflicting frame offsets also remains a hard
refusal. Once all exact checker sites are marked, the locked-void ABI mechanism
from `calleeretpreserves` retains the reaching EAX constants without changing
generic call modeling.

Return-tail duplication still renders the correct SSA as adjacent
`v = 0|1; checker(); return v;` statements. A final C fold spells this as
`checker(); return 0|1;` only when all of these facts coincide: the assignment
is literal zero or one, the next line is one complete standalone call statement
whose markup `opref` is the exact call occurrence marked by the `/GS` proof, and
the following return names the same local. The evidence remains occurrence-keyed
through presentation; another unmarked call with the same printed callee name
cannot authorize a fold. Different locals, non-literals, trailing expressions
or statements, intervening statements, and unmarked calls are unchanged. The
blanked assignment's markup associations move to the visible return line, which
therefore combines the literal-producing instruction origin with the RET origin
instead of stranding provenance on an empty line. The checker remains visible.

Disabling `calleeretpreserves` restores the exact undefined-EAX symptom. This is
a correctness extension to that existing default-on option and adds no option,
catalog row, or generic callee-body exception.
