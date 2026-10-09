# Rebuild: clang -nostdlib -no-pie -Wl,--build-id=none,-Ttext=0x401000,-e,_start -o lifetimes_temporaries_x86_64 lifetimes_temporaries_x86_64.S
.intel_syntax noprefix
.text
.global multiply_names, observe_number, _start
.type multiply_names,@function
multiply_names:
    push rbx
    push r12
    push r13
    mov rax,rdi
    mov edx,7
    mul rdx
    mov rbx,rax
    mov r12,rdx
    mov rdi,rax
    call observe_number
    mov rdi,r12
    call observe_number
    mov rdi,rbx
    call observe_number
    mov rdi,r12
    call observe_number
    pop r13
    pop r12
    pop rbx
    ret
.size multiply_names,.-multiply_names
.global multiply_names32
.type multiply_names32,@function
multiply_names32:
    push rbx
    push r12
    push r13
    mov eax,edi
    mov edx,7
    mul edx
    mov rbx,rax
    mov r12,rdx
    mov rdi,rax
    call observe_number
    mov rdi,r12
    call observe_number
    mov rdi,rbx
    call observe_number
    mov rdi,r12
    call observe_number
    pop r13
    pop r12
    pop rbx
    ret
.size multiply_names32,.-multiply_names32
.type observe_number,@function
observe_number:
    add qword ptr [rip+total],rdi
    ret
.size observe_number,.-observe_number
.type _start,@function
_start:
    mov edi,13
    call multiply_names
    cmp qword ptr [rip+total],182
    jne failed
    mov qword ptr [rip+total],0
    mov edi,0xffffffff
    call multiply_names32
    mov rax,0x1fffffffe
    cmp qword ptr [rip+total],rax
    jne failed
    xor edi,edi
    mov eax,60
    syscall
failed:
    mov edi,1
    mov eax,60
    syscall
.size _start,.-_start
.bss
.align 8
total: .zero 8
