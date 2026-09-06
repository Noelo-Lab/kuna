    .text
    .globl fmt
    .type fmt,@function
fmt:                            # int fmt(const char *f, ...)
    movq    8(%rsp), %rax       # the 7th argument, stack-passed
    addq    16(%rsp), %rax      # the 8th
    addq    %rsi, %rax
    addq    %rdx, %rax
    addq    %rcx, %rax
    addq    %r8, %rax
    addq    %r9, %rax
    ret
    .size fmt,.-fmt

    .globl report
    .type report,@function
report:                         # void report(long x, long y)
    subq    $0x38, %rsp
    movq    cookie(%rip), %rax  # the /GS-style frame cookie
    xorq    %rsp, %rax          # ... mixed with the stack pointer
    movq    %rax, 0x28(%rsp)
    movq    %rdi, (%rsp)        # variable argument 6
    movq    %rsi, 8(%rsp)       # variable argument 7
    leaq    pct(%rip), %rdi     # the format string
    movl    $1, %esi
    movl    $2, %edx
    movl    $3, %ecx
    movl    $5, %r9d
    movl    $4, %r8d
    call    fmt
    movq    0x28(%rsp), %rcx
    xorq    %rsp, %rcx
    cmpq    cookie(%rip), %rcx
    jne     .Lfail
    addq    $0x38, %rsp
    ret
.Lfail:
    ud2
    .size report,.-report

    .section .rodata
pct:
    .asciz "%s%s"
    .data
    .globl cookie
cookie:
    .quad 0x123456789abcdef0
