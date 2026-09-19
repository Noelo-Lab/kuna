# `argclobber` — plan

## The change

One clause of `drop_clobber_tail_arg`
(`decompiler/crates/kuna-decomp/src/p4_calls/kuna_argclobber.rs`) is replaced and
one is demoted.

* **New, admitting**: `callee_prototype_excludes` reads
  `Funcdata::kuna_protoorder_types(entry)` — the parameter list `protoorder`
  parked when it decompiled that callee — and requires it to exist and to have no
  parameter overlapping the argument's register bytes. No statement is a decline.
* **Demoted, vetoing**: the bounded callee-entry walk (`proves_input`) runs after
  it and can only refuse.
* `seed_callee_entry_dead` no longer decodes callee bodies for `argclobber` alone
  when nothing is parked: with an empty table the rule can never fire.

`values` stays `on|off`; `on` now means the new rule. There is no `proto` value,
because the old rule has no remaining use: it is unsound where they differ and
identical where they do not.

## Default

`default = "on"`, `destructive = true`.

## Tests

* `tests/cli/argclobber-drops-a-trailing-clobbered-argument.json` — the positive
  witness on `decompiler/crates/kuna-analysis/tests/fixtures/argclobber_x86_64`
  (the stage fixture's construct, as a real program so `decompile-all` decompiles
  the callee first): `sub_401031(a0,5)`, no `// rdx` local, no `% 200`.
* `tests/cli/argclobber-off-keeps-the-clobbered-argument.json` — its `off` arm,
  so the probe above measures the option and not the base.
* `tests/cli/argclobber-keeps-an-argument-the-callee-takes.json` — the
  `ce-remainder-or-pair` counterexample on
  `argclobber_pair_x86_64`: the argument stays.
* `tests/stages/kuna-argclobber.xml` becomes the negative control it now is. The
  harness decompiles one function from a byte image, so nothing is parked and the
  argument survives both passes. Its three sibling stage tests
  (`-forward`, `-guards`, `-armreturn`) already assert survival and are unchanged.
* The four counterexample programs stay in this directory as the record of what
  the rule must decline; each is reproduced by the commands in
  `default-on-evaluation.md`.

## Measurements

The flip procedure, all on this branch's own base, both arms of the same build:
`make test`, `make test-stages`, `make test-cli`, the 444-slice `typesweep`,
interleaved `timeit` min-of-15, and a whole-corpus `decompile-all` before/after
over every stripped ELF in decbench `full_run_address_2026-09-11` with every hunk
classified. `record.json` carries the numbers.
