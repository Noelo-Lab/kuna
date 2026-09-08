# (kuna) `codescalar` fixture -- RE-friction round 9, need
# `decompiler-emits-void-scalar` (challenge 5ab77f5f33c5d40ad448c834, PE32
# `crkme.exe`, the INT3-detection stub at 0x44ac6b), reduced to one witness and
# one control.
#
# `code` is built with size 1 so that `code *` arithmetic steps one byte, and
# `TypeOp::propagateFromPointer` decides what a dereference yields by testing
# only `ptrto->getSize() == sz`.  So a one-byte access through a recovered
# function pointer adopts `code` as the VALUE type, and the C back-end prints
# that widthless scalar `void`.
#
#   `codebyte`  the witness.  rbx is CALLED (`call *%rbx`), so it is typed
#               `code *`, and al -- the call's own result -- is stored back
#               through it.  With `option codescalar off` the local is declared
#               `void v1; // al` and the store reads `v1 = (void)(*a0)();`; with
#               it on, `char v1;` and `v1 = (*a0)();`.
#   `databyte`  the control.  Byte-for-byte the same function except that it
#               calls `sink` DIRECTLY, so nothing ever types rbx `code *`.  Its
#               byte was always `char`; it must be identical in both passes.
#
# Build (host binutils, no libc):
#   gcc -no-pie -nostdlib -Wl,-Ttext=0x100000 -e codebyte \
#       -o codescalar_x86_64 codescalar_x86_64.s

        .text

        .globl sink
        .type sink, @function
sink:
        ret
        .size sink, .-sink

        .globl codebyte
        .type codebyte, @function
codebyte:
        push %rbx
        mov  %rdi,%rbx
        xor  %eax,%eax
        test %esi,%esi
        jz   1f
        call *%rbx
        mov  %al,(%rbx)
1:
        add  $0x24,%al
        movzbl %al,%eax
        pop  %rbx
        ret
        .size codebyte, .-codebyte

        .globl databyte
        .type databyte, @function
databyte:
        push %rbx
        mov  %rdi,%rbx
        xor  %eax,%eax
        test %esi,%esi
        jz   1f
        call sink
        mov  %al,(%rbx)
1:
        add  $0x24,%al
        movzbl %al,%eax
        pop  %rbx
        ret
        .size databyte, .-databyte
