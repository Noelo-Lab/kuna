.text
.globl _sample
.type _sample,@function
_sample:
    subl $8, %esp
    movl %edi, (%esp)
    movl (%esp), %eax
    pushl %eax
    calll _logflags
    addl $4, %esp
    movl 12(%esp), %eax
    movl %eax, (%esp)
    leal (%esp), %eax
    pushl %eax
    calll _read_value
    addl $4, %esp
    addl $8, %esp
    retl

.size _sample, .-_sample
.section .note.GNU-stack,"",@progbits
