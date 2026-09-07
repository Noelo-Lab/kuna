## The problem

A callee that is called exactly once loses the argument the caller passes it.
Every rule kuna has for reconciling an argument list needs a *sibling* call to
the same address to compare against, so where there is no sibling there is no
recovery — and a thread entry point, a one-shot payload or a handler reached
from one place never has one.

Reduced to two functions (`as` + `ld` on the `.s` this PR checks in), `caller`
passes `rdi` straight into a callee that dereferences it, and also branches on
it — the `test rdi,rdi; je; call` shape that makes `Funcdata::onlyOpUse` reject
the trial on its `CBRANCH` descendant:

```
$ kuna decompile calleearitybody_x86_64 caller --option calleearitybody off
unsigned long caller(long a0)
{
  if (a0)
    return callee();          <- the argument is gone
  return 0;
}
```

On the recorded witness (PE x86-64, `sub_14000b62c` calling `payload` at
`0x14000ca0c`) it is the same shape at scale: `rcx` is the caller's own first
parameter and is untouched from the entry to the `CALL`, but the call renders
with nothing:

```
$ kuna decompile runtime.exe 0x14000b62c \
    --define-function 0x14000b62c --define-function 0x14000ca0c=payload
  else {
    payload();
  }
```

...while decompiling that same callee on its own recovers the parameter kuna
just dropped at the call:

```
$ kuna decompile runtime.exe 0x14000ca0c --define-function 0x14000ca0c=payload
unsigned long long payload(unsigned int *a0)
```

`calleearity`, `calleearityfwd` and `calleearitylive` all decline, and by
construction rather than by accident: the only other mention of `payload` in the
caller is an address-taken `lea` for a `CreateThread` argument, and an
address-taken `lea` is not a call site.

## The fix

New option `calleearitybody` (P4, default on, `0/675`), in
`p4_calls/kuna_calleearitybody.rs`.

- The witness becomes the **callee's own body** — the same bounded entry decode
  `calleedeadarg` already takes — instead of a sibling call. The recovered list
  is the run of argument registers the callee reads before writing.
- That run has to **end at a register the callee provably overwrites before ever
  reading it**. "Some path reads this register" is an existential and would run
  to the end of the argument registers on its own; what bounds an argument list
  is a register the callee is proven *not* to consume. This is also what refuses
  a variadic register-save prologue, which reads every argument register there
  is.
- No argument location outside the run may be read either, so the run is the
  callee's whole register list and not a prefix of it, and no register the
  caller wrote may sit in front of the run, so a promoted value never prints in
  its neighbour's position.
- `Funcdata::onlyOpUse` is untouched. Relaxing it is the obvious route and the
  wrong one: it would fabricate an argument at every `test rcx,rcx; jz; call`.
- One shared change: `xor ecx,ecx`, `and edx,0` and `or rdx,-1` write a constant
  into a register their p-code reads, so the body probe now records those reads
  separately. Without it an API-resolution stub whose prologue is
  `xor ecx,ecx; lea rdx,[..]; call` reads as taking an argument it discards.
  `proves_dead` — the subtractive direction — is deliberately left counting them.

## The tests

`tests/cli/defined-payload-function-loses.json` on a new 4.7 KB fixture
(`calleearitybody_x86_64`, built from a checked-in `.s`): `callee()` with
`--option calleearitybody off`, `callee(a0)` by default. 13 unit tests cover the
two guards and every refusal. Whole-corpus sweep over 134 dataset binaries: 12
changed, 303 promoted calls agree with the callee's own recovered declaration
and 0 contradict it; `make test` 675/675 PARITY OK, `make test-stages` 677/677
PARITY OK, `make test-cli` 73/73, speed +0.35% on the witness.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
