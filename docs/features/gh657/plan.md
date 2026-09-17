# Plan — GH-657

**Shape:** strict correctness fix, no option. `foldcallret` is default-on and
folds a call past a write the callee may read; the fold is wrong, not a matter
of taste, so there is nothing to gate. `--option foldcallret off` remains the
escape hatch for the whole pass.

1. Reproduce the issue (`repro.c`) and find whether the same hole is reachable
   past the single use (`travel.s`). Both yes.
2. Share the *helpers* with `foldcallretphi` — the storage test, the print-point
   walk, the opcode test — by moving them into `kuna_callretfold.rs`, which is
   the module that owns order safety. Do **not** share the policy:
   `foldcallretphi` composes its own guard, exactly as it shipped.
3. For the default fold, take a write test to the landing statement. Leave the
   opcode `LOAD` and the INDIRECT test on the call-to-use span: asking either
   further de-folds renderings the binary's own order backs (117 functions and
   27 functions respectively), and past the use those ops share the folded
   expression's one statement.
4. Gate: `make test` must stay 675/675 with no re-pin. Any stage assertion that
   moves gets read, not absorbed.
5. Measure: 37-binary `decompile-all` sweep with every changed function
   classified AND attributed to its clause (five instrumented builds of the
   second span); a disjoint 5-binary sweep; `varcensus`; the campaign typesweep
   on both builds; interleaved min-of-15 timing.
6. Ship: stage test (single pass, both shapes), `tests/cli` probe on a
   hand-assembled fixture, spec §6.4 prose.

**Held back deliberately:** exempting the self-copy from the INDIRECT test as
well. It is sound (the op is inert on both sides) and it would let
`kuna-elfmain`'s `sub_1357(a1[1])` fold, but a fix for a wrong-output defect
should not add folds. It belongs in its own PR with its own sweep.

**The trap this hit, twice:** sharing a helper between two options is not free.
The first revision moved #654's span guard wholesale and kept only its
landing-statement clause, which silently widened `foldcallretphi`. The second
put the INDIRECT half back but left the barrier half sharing
`foldcallret`'s — by then softened by two exemptions #654 never had — so
`--option foldcallretphi on` still gained folds on `ssh` O2 and `dpkg` O2. The
delta *set* (`on` vs `off`, per function) cannot see that: it was identical
both times. The instrument that can is per-function **content** equality of the
`on` dumps, main build against the branch (`phicmp.py`), and the standard is
that a function may render differently under the option only where the default
rendering changed too. Any PR that re-homes another option's guard owes that
measurement.
