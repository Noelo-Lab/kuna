# (kuna) `protoorder cycles` fixture: functions in call-graph cycles, whose own
# recovered types are the only evidence their callers have.
#
# `strwalk` calls itself and reads its argument one byte at a time, so its own
# recovery types it `char *`.  `wrap` spills its argument and hands it on, the
# -O0 shape of every coreutils caller of gnulib's self-recursive
# `quotearg_buffer_restyled`.  Under `protoorder types` a function that calls
# itself states nothing and `wrap` keeps an integer parameter; under `cycles`
# `strwalk` states its type and `wrap` reads it.  `even` and `odd` call each
# other, a two-member cycle, and `wrap2` hands its argument to `even`.
#
# `rtarget` calls itself and writes rdx (cqo) before it does.  `rcaller` passes
# it an rdx nobody set: on one path `clobber` destroyed it, on the other an
# idivl left its remainder there.  Once `rtarget` states its two-parameter
# list, `argclobber` drops that trailing argument.  `rkeep` has the same shape
# except that it forwards rdx UNTOUCHED into its own recursion, so a caller's
# rdx can reach a read no recovery accounts for, and `kcaller`'s third argument
# stays in both arms.
#
# Built with:  as -o f.o f.s && ld -o protoorder_cycles_x86_64 f.o
        .text
        .type strwalk, @function
strwalk:
        movzbl  (%rdi), %eax
        testb   %al, %al
        je      1f
        addq    $1, %rdi
        call    strwalk
        addl    $1, %eax
        ret
1:      xorl    %eax, %eax
        ret
        .size strwalk, .-strwalk

        .type wrap, @function
wrap:
        pushq   %rbp
        movq    %rsp, %rbp
        subq    $16, %rsp
        movq    %rdi, -8(%rbp)
        movq    -8(%rbp), %rax
        movq    %rax, %rdi
        call    strwalk
        leave
        ret
        .size wrap, .-wrap

        .type even, @function
even:
        movzbl  (%rdi), %eax
        testb   %al, %al
        je      1f
        addq    $1, %rdi
        call    odd
        ret
1:      movl    $1, %eax
        ret
        .size even, .-even

        .type odd, @function
odd:
        movzbl  (%rdi), %eax
        testb   %al, %al
        je      1f
        addq    $1, %rdi
        call    even
        ret
1:      xorl    %eax, %eax
        ret
        .size odd, .-odd

        .type wrap2, @function
wrap2:
        pushq   %rbp
        movq    %rsp, %rbp
        subq    $16, %rsp
        movq    %rdi, -8(%rbp)
        movq    -8(%rbp), %rax
        movq    %rax, %rdi
        call    even
        leave
        ret
        .size wrap2, .-wrap2

        .type rtarget, @function
rtarget:
        testq   %rsi, %rsi
        je      1f
        movq    %rdi, %rax
        cqto
        movl    $3, %ecx
        idivq   %rcx
        movq    %rax, %rdi
        subq    $1, %rsi
        call    rtarget
        ret
1:      movq    %rdi, %rax
        ret
        .size rtarget, .-rtarget

        .type rcaller, @function
rcaller:
        pushq   %rbx
        movq    %rdi, %rbx
        testq   %rsi, %rsi
        je      1f
        call    clobber
        jmp     2f
1:      movl    %ebx, %eax
        cltd
        movl    $200, %ecx
        idivl   %ecx
        movl    %eax, %ebx
2:      movq    %rbx, %rdi
        movl    $5, %esi
        call    rtarget
        popq    %rbx
        ret
        .size rcaller, .-rcaller

        .type rkeep, @function
rkeep:
        testq   %rsi, %rsi
        je      1f
        addq    $1, %rdi
        subq    $1, %rsi
        call    rkeep
        ret
1:      movq    %rdi, %rax
        ret
        .size rkeep, .-rkeep

        .type kcaller, @function
kcaller:
        pushq   %rbx
        movq    %rdi, %rbx
        testq   %rsi, %rsi
        je      1f
        call    clobber
        jmp     2f
1:      movl    %ebx, %eax
        cltd
        movl    $200, %ecx
        idivl   %ecx
        movl    %eax, %ebx
2:      movq    %rbx, %rdi
        movl    $5, %esi
        call    rkeep
        popq    %rbx
        ret
        .size kcaller, .-kcaller

        .type clobber, @function
clobber:
        call    helper
        ret
        .size clobber, .-clobber

        .type helper, @function
helper:
        ret
        .size helper, .-helper

        .globl _start
        .type _start, @function
_start:
        leaq    msg(%rip), %rdi
        call    wrap
        leaq    msg(%rip), %rdi
        call    wrap2
        movq    $1, %rdi
        movq    $3, %rsi
        call    rcaller
        movq    $1, %rdi
        movq    $3, %rsi
        call    kcaller
        ret
        .size _start, .-_start

        .section .rodata
msg:    .asciz  "hello"
