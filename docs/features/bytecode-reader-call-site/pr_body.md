## The problem

A callee that is called exactly once, whose caller passes its own incoming
parameters straight through, renders with no arguments at all — even though a
separate decompile of that same callee recovers four.

```
$ kuna decompile decompiler/crates/kuna-analysis/tests/fixtures/calleearitycut_x86_64 caller
int caller(int a0,long a1,long a2)
{
  if (a2 != a1)
    return callee() + a0;
  return 0;
}
```

`callee` is `mov %rdi,%rax; add %esi,%eax; add %edx,%eax; add %ecx,%eax; call
sink; ret` — it reads all four argument registers — and `caller` writes
`mov $1,%ecx` immediately before the `call`. Filed from the `Layers` crackme,
where the bytecode reader `sub_875e0(ctx, buf, len, flags)` was reduced to
`sub_875e0();` and the four values an agent needed were gone.

## The fix

- Two things go wrong at once. `AncestorRealistic::execute`
  (`funcdata_varnode.cc:2159`) refuses an input Varnode outright — *"we expect
  to see active movement into the parameter"* — so the three pass-through
  registers score inactive, and `ParamListStandard::fillinMap` reads the hole in
  front of the one register the caller did write as the end of the list. Nothing
  survives, not even that one. Upstream scoring is left exactly as it is.
- `calleearitybody` already reaches this site and picks the right four registers
  from the callee's own body, then declines: it requires the read run to end at a
  register the callee is proven to overwrite, and the callee-body walk ends every
  path at the callee's first nested call, so a register clobbered past that call
  is invisible to it. That is most callees.
- New option `calleearitycut` (default on) supplies the bound from the run's own
  shape instead. A run that **stops short** of the last argument register is
  bounded by the register it stops at, provided the run is **contiguous** and
  that register is **quiet** — the caller placed neither a computed value nor a
  constant in it. Contiguity is why the only reachable error is a missing
  trailing argument and never a misplaced one; stopping short is what still
  refuses a variadic register-save prologue, which reads every argument register
  there is.
- Testing the boundary for a *constant* as well as a write is load-bearing and
  was found by the negative control: constant propagation had already replaced
  `mov $2,%r8d` with the literal at the CALL, so an `is_written()` test alone let
  that case through.

## The tests

`tests/stages/kuna-calleearitycut.xml` is two-pass over a vendored x86-64
fixture, with both negative controls: a callee that reads *every* argument
register (no boundary) and a caller that writes the boundary register (not
quiet); both are declined in both passes. `tests/cli/bytecode-reader-call-site.json`
is the promoted acceptance probe.

Sweep — `decompile-all` over 82 decbench binaries in both arms: 8 binaries
changed, **53 empty argument lists filled, 0 calls lost an argument, 0 arguments
emitted in the wrong position**. 50 of the 53 are exact-arity and checkable
against upstream source (`xrealloc(padding_buffer,padding_buffer_size)`,
`sshbuf_ptr(buf)`, `debugging.isra.0(pc.parse_datetime_debug)`,
`memcpy_lowcase.isra.0(v39,v35,v38)` matching its `memcpy_uppcase` sibling on the
next line). The other 3 are one `-O0` ARM helper whose second instruction is
`bl`, so the run is one register long against a real list of four — short, not
wrong, and still more than the `(void)` it replaces.

Gates: `make test` PARITY OK 675/675 (0 assertions moved), `make test-stages`
PARITY OK 687/687 (purely additive), `make test-cli` 78/78, `make rust-test`
green, `make check-spec` OK, `kuna catalog --check` OK. Speed: −29.8% on the
witness (median of 7), which is this machine's single-target noise floor rather
than an effect — the rule adds a few comparisons to a call that was already
captured and takes no new decode.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
