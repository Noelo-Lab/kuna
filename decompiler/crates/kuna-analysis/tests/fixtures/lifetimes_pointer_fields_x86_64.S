# clang -nostdlib -no-pie -Wl,--build-id=none,-Ttext=0x401000,-Tdata=0x402000,-e,_start -o lifetimes_pointer_fields_x86_64 lifetimes_pointer_fields_x86_64.S
.text
.globl group_fields, construct_set, _start
.type group_fields,@function
group_fields:
    sub $40, %rsp
    mov %rsp, %rdi
    call construct_set
    mov 8(%rsp), %rax
    mov (%rax), %rax
    mov 16(%rax), %eax
    add $40, %rsp
    ret
.size group_fields,.-group_fields

.type construct_set,@function
construct_set:
    lea first_node(%rip), %rax
    mov %rax, 8(%rdi)
    mov $1, %eax
    ret
.size construct_set,.-construct_set

.type _start,@function
_start:
    call group_fields
    xor %edi, %edi
    cmp $31, %eax
    setne %dil
    mov $60, %eax
    syscall
.size _start,.-_start

.data
.align 8
first_node:
    .quad second_node, 0
    .long 7, 0
second_node:
    .quad 0, 0
    .long 31, 0
.section .note.GNU-stack,"",@progbits
