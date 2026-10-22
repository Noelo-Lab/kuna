# Two zeroed 64-byte records receive partial header fields and are observed.
# The family value is loaded once, before either loop, and reused afterward.
.text
.globl binary_fill_two_requests
.type binary_fill_two_requests,@function
binary_fill_two_requests:
 pushq %rbx
 pushq %r12
 subq $136,%rsp
 movl (%rdi),%ebx
 leaq (%rsp),%rdi
 movl $8,%ecx
 xorl %eax,%eax
 rep stosq
 movl $20,(%rsp)
 movw %bx,4(%rsp)
 movw $1,6(%rsp)
 movw $0x103,16(%rsp)
 leaq (%rsp),%rdi
 movl $1,%esi
 call .Lrecord_hash
 movl %eax,%r12d
 leaq 64(%rsp),%rdi
 movl $8,%ecx
 xorl %eax,%eax
 rep stosq
 movl $20,64(%rsp)
 movw %bx,68(%rsp)
 movw $0x301,70(%rsp)
 movw $0x103,80(%rsp)
 leaq 64(%rsp),%rdi
 movl $2,%esi
 call .Lrecord_hash
 addl %r12d,%eax
 addq $136,%rsp
 popq %r12
 popq %rbx
 ret
.size binary_fill_two_requests,.-binary_fill_two_requests
.p2align 5
.globl binary_record_hash
.type binary_record_hash,@function
binary_record_hash:
.Lrecord_hash:
 xorl %eax,%eax
 xorl %edx,%edx
1:
 movzbl (%rdi,%rdx),%ecx
 imull $33,%eax,%eax
 addl %ecx,%eax
 incq %rdx
 cmpq $64,%rdx
 jb 1b
 xorl %esi,%eax
 # The saved family must not be reread from the first record after this call.
 notw 4(%rdi)
 ret
.size binary_record_hash,.-binary_record_hash
.section .note.GNU-stack,"",@progbits
