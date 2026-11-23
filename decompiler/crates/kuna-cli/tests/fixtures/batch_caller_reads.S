.text
.globl batch_target
.hidden batch_target
.type batch_target,@function
batch_target:
    leaq 7(%rdi), %rax
    ret
.size batch_target, .-batch_target

.globl batch_wrapper
.type batch_wrapper,@function
batch_wrapper:
    subq $8, %rsp
    call *batch_slot(%rip)
    addq $8, %rsp
    ret
.size batch_wrapper, .-batch_wrapper

.globl batch_unread
.type batch_unread,@function
batch_unread:
    subq $8, %rsp
    call *batch_slot(%rip)
    addq $8, %rsp
    ret
.size batch_unread, .-batch_unread

.globl batch_reader
.type batch_reader,@function
batch_reader:
    subq $8, %rsp
    call batch_wrapper
    cmpq $26, %rax
    sete %al
    movzbl %al, %eax
    addq $8, %rsp
    ret
.size batch_reader, .-batch_reader

.section .got,"aw",@progbits
.balign 8
.type batch_slot,@object
batch_slot: .quad batch_target
.size batch_slot, 8
.section .note.GNU-stack,"",@progbits
