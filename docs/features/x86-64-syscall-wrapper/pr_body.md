## The problem

An x86-64 syscall wrapper decompiles to a call that takes no arguments and a
return value that is really the caller's own syscall number. Build the wrapper
and look at it:

```
$ cat > w.s <<'ASM'
	.globl kernel_call
	.type kernel_call, @function
kernel_call:
	push %rbp; mov %rsp,%rbp
	mov %rdi,-0x18(%rbp); mov %rsi,-0x20(%rbp); mov %rdx,-0x28(%rbp)
	mov -0x18(%rbp),%rax; mov -0x20(%rbp),%rdx; mov -0x28(%rbp),%rsi
	mov %rdx,%rdi
	syscall
	mov %rax,-0x8(%rbp); mov -0x8(%rbp),%rax
	pop %rbp; ret
ASM
$ gcc -nostdlib -no-pie -e kernel_call -o w w.s
$ kuna decompile ./w kernel_call --option x64syscall off \
    --assert 'prototype 0x401000 long kernel_call(long number, long address, long length)'
long kernel_call(long number,long address,long length)
{
  syscall();
  return number;
}
```

Every argument is gone and the function claims to return the number it was
handed. `ia.sinc:4836` lowers the instruction as `RCX = inst_next; R11 = rflags;
syscall();` — a `CALLOTHER` with no inputs and no output. No inputs, so the four
register writes above have no reader and dead-code elimination collects them. No
output, so nothing redefines `RAX` and returning `number` is correct copy
propagation over an operation kuna has been told writes nothing.

## The fix

- `x64syscall` (P2, `off|on|abi`, default `off`, `on` in the `aggressive`
  preset) rewrites the `SYSCALL` `CALLOTHER` in place before heritage: the ABI
  registers go in as inputs, `RAX` comes out as the output.
- The read set is the one judgement, because the syscall number here is a
  *runtime parameter* — no name or arity lookup can reach it. `on` takes an
  argument register only where a bounded backward walk of the op's own basic
  block finds it written first, so no undefined register is ever introduced;
  `abi` takes all six unconditionally, which recovers a wrapper whose setup sits
  in a predecessor block at the price of reads the function never writes.
- Not reachable from `linuxsyscall`: there is no `CALLIND` to retarget, the
  number is not a constant, and it is gated to x86-32.
- The user-op ids are resolved once per program into the ArchSeam, alongside
  `simd_shuffle_userops`; an op a compiler spec specialized with its own
  `<callotherfixup>` is dropped there, so a spec-declared model wins.

## The tests

`tests/stages/kuna-x64syscall.xml` runs all three values over a witness, a
no-`SYSCALL` control and a split-block wrapper; `#1`/`#2` are the defect and fail
at `on`. `tests/cli/x86-64-syscall-wrapper.json` carries the need's acceptance
clauses verbatim against a vendored fixture.
