# `cancelbytearithmetic` implementation plan

1. Add a P3-owned option module containing the `on|off` parser and an exact
   matcher for the full modulo-256 cancellation tree.
2. Place one `(kuna)` hook in `RuleSubCommute`'s INT_LEFT input gate. Preserve
   the INT_ZEXT/PIECE path and do not touch DIV, REM, or any later guard.
3. Rewrite only the outer one-byte add to the independent addend; never create
   a narrowed shift. Require exact source/width/offset/coefficient equality and
   a sole-consumer chain.
4. Pin shifts 1–7, all commuted operand orders, exact coefficients, option-off
   behavior, every structural guard, CALL/LOAD/constant-mask preservation,
   legacy paths, DIV/REM identity, and fixed-point convergence.
5. Embed the exact full witness in a two-pass stage. Promote the same bytes into
   a minimal in-repo PE so CI can exercise the original CLI command and exact
   closure acceptance without the external dataset.
6. Register and regenerate the phase catalog, option documentation, stage
   baseline and counters; document the default divergence as DIV-174.
7. Rebuild an immutable release binary, prove exactly five target matches, run
   all 1,590 corpus arms with zero on/off body differences, and rerun speed and
   repository gates.
8. Preserve the authoritative need history. While the candidate is unlanded,
   update only its acceptance identity and covering option; leave status open,
   attempts zero, and all PR/closure fields null.
