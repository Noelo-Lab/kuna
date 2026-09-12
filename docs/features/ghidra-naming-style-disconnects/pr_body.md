## The problem

`--option namestyle ghidra` printed an unnamed parameter as `param_1` in the
signature but used a newly declared, uninitialized `a0` in the body.

```console
$ kuna decompile decompiler/crates/kuna-analysis/tests/fixtures/assertranges_x86_64 sample --option namestyle ghidra
int sample(int param_1)
{
  int a0;

  return scale * a0 + bias + dat_50000000 * 2;
}
```

## The fix

- Use one naming rule when P6 binds a prototype input and when P9 prints or
  suppresses its declaration.
- Materialize an unnamed input as `aN` only when the angr local-name flag is
  active; otherwise use `param_N`.
- Preserve explicit and recovered names, along with GUI Ghidra mode's deliberate
  angr-style locals and parameters.

## The tests

Focused tests cover both unnamed styles, multiple slots, explicit names, and
GUI Ghidra mode. The promoted CLI probe requires the complete `sample` signature
and return expression and rejects every orphan `a0` declaration or use.

All gates pass: 3/3 focused unit cases, 1/1 promoted acceptance probe, 675/675
parity fixtures, 788/788 stage fixtures, 134/134 CLI probes, the full Rust
workspace and doctests, lenient and strict spec checks, catalog validation,
counter validation, and mergecheck with zero rejects.
