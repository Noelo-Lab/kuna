# ADR 0005: Rule/Action dispatch — trait objects + declarative SchedNode schedule

## Context

The C++ scheduler is `ActionDatabase::universalAction` (`coreaction.cc`): a
~600-line constructor that builds one nested Action/Rule tree imperatively,
then `clone(grouplist)`s it per configuration, dropping nodes whose group is
not enabled. `ActionPool::processOp` dispatches rules per opcode with subtle,
output-affecting details. Transcribing the constructor literally would be
unmaintainable; changing the schedule shape would change output.

## Decision

Rules and actions are `Box<dyn Rule>` / `Box<dyn Action>` trait objects, as in
C++. The universal action becomes a **declarative `SchedNode` spec table** —
variants `Action` / `Pool` / `Group` / `RestartGroup`, each row carrying name
and group string — which is *filtered by `ActionGroupList` at root
construction*. The filtered tree must be observably identical to the C++
`clone(grouplist)` result, including child order. `ActionPool::processOp` is
transcribed exactly: per-opcode rule lists in insertion order, dispatch walks
that order, `rule_index` resets to 0 whenever an apply changes the op's
opcode, and the mid-application resumability state (op iterator + rule index
breakpoint cursor) is kept so single-stepping behaves like the C++ console.

## Consequences

- The schedule is data: diffable against the C++ constructor during review,
  printable for `debugaction`-style tooling, and the natural anchor point for
  ADR 0007's `PassRegistration`.
- Reproducing `clone(grouplist)` semantics in the filter (a `Group` whose
  children all drop disappears, etc.) is a one-time transcription cost paid in
  tests against the C++ tree dump.
- `processOp`'s reset-on-opcode-change rule means rules can legally observe
  each other's effects mid-op; the exact transcription preserves fixpoints
  that a "cleaner" worklist would alter.
- Trait objects forgo static dispatch; irrelevant — apply cost dominates.
