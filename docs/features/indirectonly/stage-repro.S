.section .text
.globl take
take:
    push   %rbp
    mov    %rsp, %rbp
    sub    $0x120, %rsp
    mov    c1(%rip), %eax
    test   %eax, %eax
    je     1f
    lea    -0xc0(%rbp), %rax
    mov    %rax, %rdi
    mov    str(%rip), %rsi
    call   parse
    jmp    8f
1:
    call   now_ns
    mov    %rax, -0xc8(%rbp)
    mov    $1000000000, %rdi
    mov    -0xc8(%rbp), %rcx
    mov    %rcx, %rax
    cqto
    idiv   %rdi
    mov    %rax, -0xc0(%rbp)
    mov    -0xc8(%rbp), %rcx
    mov    %rcx, %rax
    cqto
    idiv   %rdi
    mov    %rdx, -0xb8(%rbp)
8:
    lea    -0xc0(%rbp), %rax
    mov    %rax, %rdi
    call   use
    leave
    ret
.globl keep
keep:
    push   %rbp
    mov    %rsp, %rbp
    sub    $0x120, %rsp
    mov    -0xb8(%rbp), %rsi
    mov    %rsi, c2(%rip)
    mov    c1(%rip), %eax
    test   %eax, %eax
    je     2f
    lea    -0xc0(%rbp), %rax
    mov    %rax, %rdi
    mov    str(%rip), %rsi
    call   parse
    jmp    9f
2:
    call   now_ns
    mov    %rax, -0xc8(%rbp)
    mov    $1000000007, %rdi
    mov    -0xc8(%rbp), %rcx
    mov    %rcx, %rax
    cqto
    idiv   %rdi
    mov    %rax, -0xc0(%rbp)
    mov    -0xc8(%rbp), %rcx
    mov    %rcx, %rax
    cqto
    idiv   %rdi
    mov    %rdx, -0xb8(%rbp)
9:
    lea    -0xc0(%rbp), %rax
    mov    %rax, %rdi
    call   use
    leave
    ret
.globl parse
parse:
    mov    %rsi, %rax
    ret
.globl now_ns
now_ns:
    mov    $5, %eax
    ret
.globl use
use:
    mov    (%rdi), %rax
    ret
.globl _start
_start:
    call   take
    call   keep
    hlt
.section .data
.globl c1
c1: .quad 0
.globl c2
c2: .quad 0
.globl str
str: .quad 0
