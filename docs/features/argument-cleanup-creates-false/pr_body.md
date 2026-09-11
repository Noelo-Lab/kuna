## The problem

A cdecl function that cleans up call arguments before an internal jump is cut in
half: kuna reads the `add esp,8` in front of the jump as a frame teardown, calls
the jump a tail call, and emits everything after the jump as a separate callee.
Paste this and build it with `gcc -m32 -nostdlib -no-pie -o t t.s`:

```asm
classify:
        pushl   %ebx
        pushl   %esi                  # frame = -8
        movl    12(%esp), %eax
        testl   %eax, %eax
        je      .Lconst
        pushl   $1
        pushl   $2
        call    helper
        addl    $8, %esp              # argument cleanup, +8
        jmp     .Ljoin
.Lconst:
        movl    $0x1234, %eax
.Ljoin:                               # reached by both paths
        testl   %eax, %eax
        je      .Lzero
        orl     $7, %eax
.Lzero:
        popl    %esi
        popl    %ebx
        ret
helper: movl 4(%esp), %eax; addl $0x5a5a5a, %eax; ret
```

`kuna decompile ./t classify`:

```c
unsigned int classify(int a0)
{
  if (!a0)
    return 0x1237;
  helper(2,1);
  return sub_804901d(); // warn: tailcallframe: recovered tail call
}
```

`sub_804901d` is `.Ljoin` — a block of `classify` itself. `%ebx` and `%esi` are
still on the stack there, and everything from `.Ljoin` on is gone from the
output.

## The fix

- `tailcallframe` requires the stack-pointer run ending at the jump to cancel the
  run leaving the entry address. Both scans stop at the first control-flow op, so
  the backward one stops at the `call` and never sees the two pushes the cleanup
  is undoing — two unrelated stack motions of 8 bytes each satisfy the test.
- New option `tailcallsaved` (default on, DIV-157) asks each scan for a second
  number: the bytes it moved *through* the stack pointer. Lowering the stack
  pointer and storing through it is saving; raising it and loading through it is
  restoring. A tail jump runs with the stack as `ret` would find it, so the rule
  is `restored >= saved`.
- A frame built purely by `sub rsp,N` saves nothing and so demands nothing, which
  is why gcc's `-O2` `sub rsp,8; call f; add rsp,8; jmp g` is decided exactly as
  before. Scanning backward *through* the call would also work on this witness
  and was rejected: a stdcall callee cleans its own arguments with `ret N`, so a
  call is not stack-neutral on the PE this came from.
- One-directional — it can only decline a recovery `tailcallframe` would have
  made.

## The tests

`tests/stages/kuna-tailcallsaved.xml` runs the shape twice (option off = the
false call and the truncation, default = the whole function).
`tests/cli/argument-cleanup-creates-false.json` pins it end to end against a new
in-repo fixture. 10 unit cases for the accounting, 3 more on the predicate
itself; the two existing `tailcallframe` witnesses still recover their tail calls.

Gates: datatests 675/675, stages 758/758, `tests/cli` 121/121, `check-spec` OK,
`catalog --check` OK.

Collateral, `decompile-all` off-vs-on: 0 changed lines across all 803 decbench
binaries (all gcc ELFs, so the shape does not occur there), and 19 of 143 32-bit
PE crackmes changed — 36 functions, 33 of them declining a tail call to a block
of their own body. The largest is `crkme.exe sub_4047e4`, which was the single
line `sub_4047f6();` and is now the whole 313-line decryptor. One function gets
no better: `illusion.exe sub_40be3d` is garbage on both arms and swaps one
failure warning for another.

Speed on `openssh-portable/sftp`, medians of 5 interleaved rounds: 9.319 s off vs
9.240 s on (−0.84%, byte-identical output).

🤖 Generated with [Claude Code](https://claude.com/claude-code)
