.text
.globl conditional_frame_store
.type conditional_frame_store,@function
conditional_frame_store:
    push %rbx
    sub $16, %rsp
    mov %edi, %ebx
    movabs $0x1122334455667788, %rax
    mov %rax, (%rsp)
    mov %rsp, %rdi
    call observe_pair
    test %ebx, %ebx
    je .Lconditional_false
    movb $1, (%rsp)
    jmp .Lconditional_done
.Lconditional_false:
    movb $0, (%rsp)
.Lconditional_done:
    mov %rsp, %rdi
    call read_word
    add $16, %rsp
    pop %rbx
    ret
.size conditional_frame_store, .-conditional_frame_store

.globl boolean_frame_store
.type boolean_frame_store,@function
boolean_frame_store:
    push %rbx
    push %rbp
    sub $24, %rsp
    mov %edi, %ebx
    mov %esi, %ebp
    movabs $0x1122334455667788, %rax
    mov %rax, (%rsp)
    mov %rsp, %rdi
    call observe_pair
    test %ebx, %ebx
    je .Lboolean_false
    test %ebp, %ebp
    je .Lboolean_false
    movb $1, (%rsp)
    jmp .Lboolean_done
.Lboolean_false:
    movb $0, (%rsp)
.Lboolean_done:
    mov %rsp, %rdi
    call read_word
    add $24, %rsp
    pop %rbp
    pop %rbx
    ret
.size boolean_frame_store, .-boolean_frame_store

.globl observe_pair
.type observe_pair,@function
observe_pair:
    mov (%rdi), %eax
    xor 4(%rdi), %eax
    ret
.size observe_pair, .-observe_pair

.globl read_word
.type read_word,@function
read_word:
    mov (%rdi), %rax
    ret
.size read_word, .-read_word
.section .note.GNU-stack,"",@progbits
