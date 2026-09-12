Fixes RE-friction need `declaring-double-score-return` by extending the existing
default-on `calleeretpreserves` behavior to a locked-void MSVC `/GS` checker.

The exact cookie recognizer now collects every eligible call site and seeds all
non-destructive markers before one restart. Heritage uses those markers to
retain only the exact logical ABI output slice across the checks; the checker
calls and cookie algebra stay in the output. A wider XMM heritage range is split
so its upper scratch lane remains killed. The production probe retains positive
writes and STORE spaces found before an unresolved nested call, so explicit
effect overrides, every recovered return-storage write, and every STORE into an
ABI output processor space still veto preservation.

The cookie proof now tolerates bounded long INDIRECT carry chains and
loop-carried phis with agreeing concrete seeds. See
`docs/features/declaring-double-score-return/analysis.md` for the diagnosis and
exact PolyMLP witness.

Validation after replay onto `342eaa03`: datatests 675/675, stages 811/811,
CLI 150/150, authoritative acceptance `a-cc4fdb4b53b3` green, catalog/spec
checks green, and the exact target flips back to the defect with
`--option calleeretpreserves off`. Nine timing samples measured 130.35 ms off
and 128.54 ms on (-1.39%, within the 5% budget).
