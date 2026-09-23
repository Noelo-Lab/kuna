# passthrough — default-on evaluation

Verdict: **on by default** (round G). The criterion the first evaluation failed
was return fabrication; the rule now refuses a callee's recovered return unless
the callee computed it, and the whole-corpus DWARF check that replaced the
26-binary sample is in `dwarf-confirmation.md`.

## Round F (commit 7da4afecf): held off

The argument arm passed every criterion — 454 of 506 gained parameters confirmed
by DWARF over 26 binaries, 0 contradicted, 0 arguments deleted, typesweep +24
perfect with 0 moved off — but the tail-return arm gave 23 functions a return
value DWARF says they do not have, and `type_match` cannot see a return type.
What the note asked for was "a gate on the return arm that refuses a callee
whose own return is a value it did not compute".

## Round G: the gate, and the whole-corpus check

`recovered_output` (`kuna_protoorder.rs`) now states a callee's return only when
`kuna_returnuncomputed::every_return_computes` holds of that callee: every live
RETURN must hand back a value produced in every byte and on every path. gnulib's
`void version_etc_arn` ends its fallthrough on a `__fprintf_chk` and returns
`CONCAT44(<leftover>, <that call's result>)`, which the pair repair's relaxed
question calls computed and this one does not.

Measured over the **whole 444-slice decbench corpus** (every function that gains
something, checked against its unstripped twin — `dwarf-confirmation.md`):

| | rows | confirmed | contradicted | no twin |
|---|---:|---:|---:|---:|
| parameters | 2,910 | 2,783 (95.6%) | 4 | 123 |
| returns, before the gate | 4,267 | — | 212 | — |
| returns, with the gate | 4,088 | 4,012 (98.1%) | 6 | 69 |

The 4 contradicted parameters are gnulib `savewd_save` in two binaries, whose
forwarded register reaches a variadic `open_safer`; the 123 rows with no twin
are 69 forwarding thunks the toolchain emitted with no debug entry. No function
and no call site loses an argument.

## The flip procedure

| step | result |
|---|---|
| (a) `make test` | PARITY OK, 675/675 — the console decompiles no callee first, so no datatest can reach the rule |
| (b) `make test-stages` | PARITY OK, 1309/1309; `tests/stages/kuna-passthrough.xml` already pins `option passthrough off` in pass 1 |
| (c) `make test-cli` | 225/225 after re-pinning two `protoorder` probes (below) |
| (d) 444-slice typesweep, new default vs `--option passthrough off` | PERFECT 1,524 → **1,548** (+24), aggregate 3,806.41 → 3,865.67, 24 onto perfect, 0 off, 104 improved, 6 worse; re-run after rebasing onto `calleevote`: 1,569 → **1,593**, the same +24 and the same six worse rows |
| (e) interleaved min-of-15, fmt/ls/sort -O2 + bash -O2 | fmt -2.59%, ls -6.65%, sort -2.98%, bash +2.09%; worst **+2.09%**, budget +5% |
| (f) whole-corpus `decompile-all` before/after, 8 binaries | 290 functions change and every changed line is one of the documented effects; a second control over all 444 slices (100,198 functions) shows the `off` arm identical to the pristine origin/main build |
| (g) `modes.rs` | the `UNEVALUATED` entry is removed: a default-on option needs no preset override |

Two `tests/cli` probes pinned output the new default moves, both `protoorder`
probes and neither about `passthrough`:

* `protoorder-types-never-moves-an-arity` now runs with `--option passthrough
  off`. Under the default `overrec` gains the `rdx` it forwards untouched to
  `vararg`, which really adds it — a genuine arity move by another rule, which
  would otherwise destroy this probe's claim.
* `protoorder-types-keeps-a-float-pointee-bitwise` expects `double s3(...)`
  instead of `void s3(...)`: `s3` hands back its tail call's result. Its clause
  — the struct pointee and the two integer-bit arguments — is unchanged.

## What ON still costs

The callee's recovery error. A callee whose recovered list over-states a
parameter its body reads for another purpose hands that parameter to every
caller that forwards the register (`savewd_save`), and a callee whose computed
return is not the one the source returns hands that up (6 functions on the
corpus). Both are bounded by the callee's own recovered prototype: the rule
never guesses, and `--option passthrough off` restores upstream's reading
exactly.
