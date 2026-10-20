.text
.globl _start
_start:
 andl $-16,%esp
 call validation_main
 movl %eax,%ebx
 movl $1,%eax
 int $0x80
.section .note.GNU-stack,"",@progbits
