        .text
        .globl  fwd
        .type   fwd, @function
fwd:                                    # jmp *(%rdi) -- never names rsi/rdx
        movq    (%rdi), %rax
        jmp     *%rax
        .size   fwd, .-fwd
