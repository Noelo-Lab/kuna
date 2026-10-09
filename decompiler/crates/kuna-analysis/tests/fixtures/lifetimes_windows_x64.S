.intel_syntax noprefix
# Windows x64 register identities and stack views with 32-byte caller home space.
# Rebuild: clang --target=x86_64-pc-windows-msvc -c lifetimes_windows_x64.S -o lifetimes_windows_x64.obj
#          ld -mi386pep --image-base 0x140000000 --entry register_names --subsystem console --export-all-symbols -o lifetimes_windows_x64.exe lifetimes_windows_x64.obj
.text
.global register_names, consume_number, identity_pointer, observe_number, observe_pointer
register_names:
    push rbx
    push rbp
    sub rsp,40
    mov rbp,rdx
    call consume_number
    mov rbx,rax
    mov rcx,rbx
    call observe_number
    mov rcx,rbx
    call observe_number
    mov rcx,rbp
    call identity_pointer
    mov rbx,rax
    mov rcx,rbx
    call observe_pointer
    mov rcx,rbx
    call observe_pointer
    add rsp,40
    pop rbp
    pop rbx
    ret
consume_number:
    lea rax,[rcx+rcx]
    ret
identity_pointer:
    mov rax,rcx
    ret
observe_number:
    add qword ptr [rip+observed_total],rcx
    ret
observe_pointer:
    mov eax,dword ptr [rcx]
    add qword ptr [rip+observed_total],rax
    ret
.global win_stack_reuse, win_read_pair, win_read_handle
win_stack_reuse:
    push rbx
    sub rsp,48
    mov dword ptr [rsp+32],5
    mov dword ptr [rsp+36],7
    lea rcx,[rsp+32]
    call win_read_pair
    mov ebx,eax
    lea rax,[rip+win_datum]
    mov [rsp+32],rax
    lea rcx,[rsp+32]
    call win_read_handle
    add eax,ebx
    add rsp,48
    pop rbx
    ret
win_read_pair:
    mov eax,[rcx]
    add eax,[rcx+4]
    ret
win_read_handle:
    mov rax,[rcx]
    mov eax,[rax]
    ret
.data
.p2align 3
observed_total: .quad 0
win_datum: .long 55
#ifndef _WIN32
.text
.global _start
_start:
    call win_stack_reuse
    xor edi,edi
    cmp eax,67
    setne dil
    mov eax,60
    syscall
.section .note.GNU-stack,"",@progbits
#endif
