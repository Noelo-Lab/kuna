# Plan — GH-657

**Shape:** strict correctness fix, no option. `foldcallret` is default-on and
folds a call past a write the callee may read; the fold is wrong, not a matter
of taste, so there is nothing to gate. `--option foldcallret off` remains the
escape hatch for the whole pass.

1. Reproduce the issue (`repro.c`) and find whether the same hole is reachable
   past the single use (`travel.s`). Both yes.
2. Share `op_writes_tied_storage` with `foldcallretphi` rather than copying it:
   move it, and the print-point machinery, into `kuna_callretfold.rs`, which is
   the module that owns order safety. `foldcallretphi` keeps only the version
   discount.
3. Take the barrier test, and only the barrier test, to the landing statement.
   Leave the INDIRECT test on the call-to-use span (widening it adds folds).
4. Gate: `make test` must stay 675/675 with no re-pin. Any stage assertion that
   moves gets read, not absorbed.
5. Measure: 16-binary `decompile-all` sweep with every changed function
   classified; a third build isolating clause 1; `varcensus`; the campaign
   typesweep on both builds; interleaved min-of-15 timing.
6. Ship: stage test (single pass, both shapes), `tests/cli` probe on a
   hand-assembled fixture, spec §6.4 prose.

**Held back deliberately:** exempting the self-copy from the INDIRECT test as
well. It is sound (the op is inert on both sides) and it would let
`kuna-elfmain`'s `sub_1357(a1[1])` fold, but a fix for a wrong-output defect
should not add folds. It belongs in its own PR with its own sweep.
