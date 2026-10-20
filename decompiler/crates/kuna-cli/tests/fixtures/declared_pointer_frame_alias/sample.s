.text
.globl sample
.type sample,@function
sample:
 pushl %ebp
 movl %esp,%ebp
 subl $8,%esp
 movl 8(%ebp),%eax
 movl %eax,-8(%ebp)
 movl 12(%ebp),%eax
 movl %eax,-4(%ebp)
 pushl %ebp
 movl %esp,%ebp
 call helper
 popl %ebp
 leal -8(%ebp),%eax
 pushl %eax
 call fill
 addl $4,%esp
 flds -8(%ebp)
 fadds -4(%ebp)
 leave
 ret
.size sample,.-sample
.globl helper
.type helper,@function
helper:
 ret
.size helper,.-helper
.section .note.GNU-stack,"",@progbits
