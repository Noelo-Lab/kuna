# Preserve the i386 stack continuation before return normalization

A function can return with a runtime-adjusted ESP. For the authored moved
frame, the lift loads the RET target through the moved stack and computes the
post-RET ESP. The high representation loses the continuation before rendering
can report it. The initial deliverable is a read-only, per-analysis
observation for one fully decoded straight-line i386 frame.
[Observed lift and runnable replay](analysis.md)

## Capture

Capture immediately before the return-input normalization in
`ActionPrototypeTypes`. Its ordinary handling erases the indirect RETURN
operand before heritage. The capture observes the raw decoded operand,
without changing the normalization or making a return value trial. P3's
dynamic-store flag still excludes these operations from fixed-frame slot
reasoning, and cannot recover an erased RET operand.

- Require the real 32-bit ESP root in the decoded register space and flat
  memory address semantics. Bind the captured extent and terminal RET to
  the effective bytes and architecture of this analysis.
- Limit the first recognizer to one decoded instruction path, one
  nonconstant whole-ESP displacement and one exact RET load and update.
  Require bounded provenance for the stack arithmetic and each recorded
  read and write. Calls, alternative exits, ambiguous widths, unknown
  memory spaces or segments, and unsupported return cleanup are outside
  this capture.
- Record the entry root, the ordered memory effects, the RET load width
  and address, and the SP expressions on either side of RET. Addresses use
  32-bit wrapping arithmetic. Later values refer to particular ordered
  reads. The late target load must survive even when a preceding store
  aliases it. An entry return word is only an initial memory value.
- Own the record inside one analysis. Key the instruction and memory
  identities to decoded locations and event order. Clear and recompute the
  observation on reset, restart or a different effective input. Persisted
  varnode and operation IDs cannot identify these effects across SSA.

The restored-frame sibling supplies a control for a return that rejoins
the normal stack. Normalized P-code and all existing source outputs
must compare byte for byte before a consumer is added. The first proof
should expose the record's decoded addresses, order, and wraparound
behavior on authored inputs.

## Consumer boundary

A later renderer would need an explicit caller-SP and continuation
boundary with the same memory state. Its observer must accept that state
as a typed input, preserve the recorded writes and ordered load, and
report both the resolved target and post-RET SP. It must not default an
unknown register, use a zero-valued source local, or borrow the compiler's
current host stack pointer. The current native observer interface supplies
caller ESP and the input continuation explicitly, and treats the
resolved continuation as data.

A structured return requires its own proof before a consumer could fold
this boundary into ordinary source. Remaining machine state, including
the example's EBP change, is outside an ESP-only observation. The linked
PE witness also requires independent segment-effect evidence. This
proposal introduces neither a source rewrite nor a new runtime option.

## Open questions from review

The first step is a read-only record with no consumer, and every existing
output must stay byte-identical. It therefore cannot carry the two-pass
stage test that `docs/agents.md` requires (option off shows the bug, the
default shows the fix), or a one-pass test pinning fixed output, because
nothing in it changes the reproduction's output. Should the first
implementation ship together with its first consumer, or expose the record
through an existing CLI or JSON surface that a test can check?
