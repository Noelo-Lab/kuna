.text
.globl sample
.type sample,@function
sample:
 subl $8,%esp
 movl 12(%esp),%eax
 movl %eax,(%esp)
 movl 16(%esp),%eax
 movl %eax,4(%esp)
 pushl 4(%esp)
 pushl 4(%esp)
 call observe
 addl $8,%esp
 leal (%esp),%eax
 pushl %eax
 call fill
 flds (%esp)
 fadds 4(%esp)
 addl $8,%esp
 ret
.size sample,.-sample
.section .note.GNU-stack,"",@progbits
