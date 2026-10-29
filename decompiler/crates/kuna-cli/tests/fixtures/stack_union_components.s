.text
.globl sample
.type sample,@function
sample:
 push %rbp
 mov %rsp,%rbp
 sub $0x40,%rsp
 mov %rsi,-0x10(%rbp)
 movl $0x3f800000,-0x30(%rbp)
 movl $0x40000000,-0x2c(%rbp)
 flds (%rdi)
 fstps -0x20(%rbp)
 flds (%rdi)
 fadds (%rdi)
 fstps -0x1c(%rbp)
 lea -0x20(%rbp),%rdi
 lea -0x1c(%rbp),%rsi
 call combine
 mov -0x10(%rbp),%rax
 movss %xmm0,(%rax)
 leave
 ret
.size sample,.-sample
.globl combine
.type combine,@function
combine:
 movss (%rdi),%xmm0
 addss (%rsi),%xmm0
 ret
.size combine,.-combine
.section .note.GNU-stack,"",@progbits
