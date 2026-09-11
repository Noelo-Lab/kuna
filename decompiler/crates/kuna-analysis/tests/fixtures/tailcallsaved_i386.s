# Fixture for `tailcallsaved` (kuna): the cdecl shape that makes a frame-teardown
# tail-call test misread argument cleanup as a teardown.  Reduced from the
# round-8 RE-friction witness (crackmes.one 5ab77f5f33c5d40ad448c820, the
# character classifier at 0x4010b0).
#
#   classify's entry block pushes two callee-saved registers, so the frame is -8.
#   The run ending at `jmp .Ljoin` is the `addl $8,%esp` that discards the two
#   arguments of the `call helper` above it, so its delta is +8 and the two
#   deltas cancel -- but %ebx and %esi are still on the stack, and .Ljoin is an
#   ordinary block of classify that the other side of the branch reaches too.
#
# Built with:
#   gcc -m32 -nostdlib -no-pie -o tailcallsaved_i386 tailcallsaved_i386.s
        .text
        .globl  classify
        .type   classify, @function
classify:
        pushl   %ebx
        pushl   %esi
        movl    12(%esp), %eax
        testl   %eax, %eax
        je      .Lconst
        pushl   $1
        pushl   $2
        call    helper
        addl    $8, %esp
        jmp     .Ljoin
.Lconst:
        movl    $0x1234, %eax
.Ljoin:
        testl   %eax, %eax
        je      .Lzero
        orl     $7, %eax
.Lzero:
        popl    %esi
        popl    %ebx
        ret
        .size   classify, .-classify

        .globl  helper
        .type   helper, @function
helper:
        movl    4(%esp), %eax
        addl    $0x5a5a5a, %eax
        ret
        .size   helper, .-helper

        .globl  _start
        .type   _start, @function
_start:
        pushl   $0x35
        call    classify
        addl    $4, %esp
        movl    %eax, %ebx
        movl    $1, %eax
        int     $0x80
        .size   _start, .-_start
