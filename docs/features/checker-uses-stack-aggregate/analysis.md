# Constant address references suppress real local declarations

The witness model already contained `v12`: an eight-byte stack scalar with a
type, offset, and all use sites. P9 nevertheless printed its two four-byte
pieces and address uses without printing a declaration.

An instrumented declaration walk found two real storage highs, one for each
four-byte half. Both reached `is_scalar_partial_piece` and were suppressed
because `high_name_has_scalar_whole_sibling` reported a same-name, offset-zero,
eight-byte sibling. That sibling was not storage. It was one of the
constant-only highs created for the `PTRSUB` address expression `&v12`, and the
declaration walk had already filtered it as an address reference. Consequently
no candidate remained to emit the declaration. The unrelated 20-element array
in the model never participated.

The repair makes the helper match its existing contract: a whole scalar
storage representative must have a non-constant first member. If no such
sibling exists, the partial highs continue into the existing symbol-keyed
collapse, which emits exactly one declaration. Actual whole storage siblings
still suppress their partial declarations, preserving the LOSS-245 duplicate
declaration guard.

This is an unflagged strict bug fix under `docs/agents.md`: a constant address
reference can never emit the declaration whose existence the predicate claims,
so accepting it is universally wrong rather than a configurable judgment.

The complete before/after scope and both changed-function classifications are
recorded in [`sweep.md`](sweep.md); compact raw hunks are in
[`sweep.diff`](sweep.diff).
