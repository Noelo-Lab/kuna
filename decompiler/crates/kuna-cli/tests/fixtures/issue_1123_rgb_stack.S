.text
.globl build
.type build,@function
build:
    pushq %rbp
    movq %rsp,%rbp
    subq $32,%rsp
    movzbl colors(%rip),%eax
    movb %al,-3(%rbp)
    movzbl colors+1(%rip),%eax
    movb %al,-2(%rbp)
    movzbl colors+2(%rip),%eax
    movb %al,-1(%rbp)
    leaq -3(%rbp),%rdi
    call consume
    leave
    ret
.size build,.-build

.globl consume
.type consume,@function
consume:
    movzbl (%rdi),%eax
    movzbl 1(%rdi),%edx
    shll $8,%edx
    orl %edx,%eax
    movzbl 2(%rdi),%edx
    shll $16,%edx
    orl %edx,%eax
    movl %eax,sink(%rip)
    ret
.size consume,.-consume

.data
colors:
    .byte 0x11,0x22,0x33
.comm sink,4,4
.section .note.GNU-stack,"",@progbits
