## The problem

A four-argument XOR decryptor that returns the buffer it allocates comes back
with a fifth argument and a 128-bit return, and the fifth argument's only
appearance is as the high half of that return.

```
$ kuna decompile ./WeeperVM--Level_1 0x10e27 --addr
undefined16 sub_10e27(long a0,int a1,long a2,int a3,unsigned long a4)
{
  char v1 [16];
  ...
  *(char *)((long)v1._0_8_ + (long)a1) = 0;
  v1._8_8_ = a4;
  return v1._0_16_;
}
```

The idiom that causes it is two instructions of stack maintenance, reproducible
on eleven bytes:

```
$ rasm2 -a x86 -b 64 -d 41504889f84883c0075ac3
push r8
mov rax, rdi
add rax, 7
pop rdx
ret
```

`RDX` at the `ret` holds `R8` as it arrived from the caller, so the return-half
rule reads it as an argument the function *placed* in the return register --
which is what gives `R8` a reader, which is what makes `R8` a parameter. The
loop closes on itself. Assemble those bytes as `f` and `kuna decompile ./f f`
recovers five arguments for a function that takes one.

## The fix

- A register is **push-only** when some stack-adjusting instruction stores it to
  memory and no instruction in the function ever writes it. A push-only register
  is stack maintenance, not a value the function placed anywhere, so it can no
  longer satisfy `retinputhalf`'s placement test. New option `retpushedhalf`
  (default on, DIV-156).
- The evidence is gathered during the flow build rather than at the repair,
  because it does not survive to the repair: copy propagation collapses the
  store and the load, and the surviving `RDX = COPY(R8)` at the RETURN is
  byte-for-byte what a deliberate `mov %r8,%rdx` leaves behind. Both were dumped
  and compared.
- The write test is what keeps the ordinary callee-saved save/restore out: the
  pop writes the register it pushed. The stack-adjust test keeps out everything
  with no push, which is every genuine shape -- a returned fifth argument moved
  with `mov`, an `__int128` built from two untouched arguments, a
  `struct{long,long}` whose halves are computed.
- Not fixed, and deliberately: `mov %rdi,%rax; add $7,%rax; mov %rsi,%rdx; ret`
  shows the same output with no stack involved at all, and is indistinguishable
  from a genuine `struct{long,long}` return.

## The tests

`tests/stages/kuna-retpushedhalf.xml` runs three functions in two passes: the
witness idiom, the same-register save/restore, and the stack-free register move.
Assertion #2 is the one that fails without the fix. Five unit tests on the
push-only record, and `tests/cli/alignment-push-pop-invents.json` on a new
vendored fixture.

`make test` 675/675 PARITY OK (byte-identical), `make test-stages` 754/754,
`make rust-test` green, `make check-spec` OK, `make test-cli` 120/120. Whole-binary
`decompile-all` before/after: 0 changed lines across 7,426 functions in five
linked binaries, 1 changed function across 33 crackme ELFs -- the witness.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
