# Rebuild: clang -nostdlib -no-pie -Wl,--build-id=none stackhome_alias_x86_64.S -o stackhome_alias_x86_64
# Windows x64 reuse() stores call keys in RCX's home slot and in its fifth argument's slot.
.intel_syntax noprefix
.text
.global reuse, observe, _start
.type reuse,@function
reuse:
    push rbx
    push r12
    sub rsp,40
    mov r12,rcx
    movzx ebx,byte ptr [rsp+96]
    mov qword ptr [rsp+64],5
    mov eax,dword ptr [rsp+64]
    mov dword ptr [r12],eax
    mov dword ptr [rsp+64],12
    lea rdx,[rsp+64]
    mov rcx,r12
    call observe
    test bl,bl
    jne .Lend
    mov dword ptr [rsp+96],10
    lea rdx,[rsp+96]
    mov rcx,r12
    call observe
    mov dword ptr [rsp+96],11
    lea rdx,[rsp+96]
    mov rcx,r12
    call observe
.Lend:
    xor eax,eax
    add rsp,40
    pop r12
    pop rbx
    ret
.size reuse,.-reuse
.type observe,@function
observe:
    mov eax,dword ptr [rdx]
    movsxd r8,dword ptr [rcx+4]
    cmp r8,6
    jae 1f
    mov dword ptr [rcx+r8*4+8],eax
1:  inc dword ptr [rcx+4]
    ret
.size observe,.-observe
.type _start,@function
_start:
    sub rsp,48
    lea rcx,[rip+context]
    mov edx,2
    mov r8d,3
    mov r9d,4
    mov qword ptr [rsp+32],0
    call reuse
    lea rax,[rip+context]
    cmp dword ptr [rax],5
    jne .Lfail
    cmp dword ptr [rax+4],3
    jne .Lfail
    cmp dword ptr [rax+8],12
    jne .Lfail
    cmp dword ptr [rax+12],10
    jne .Lfail
    cmp dword ptr [rax+16],11
    jne .Lfail
    mov qword ptr [rax],0
    mov qword ptr [rax+8],0
    lea rcx,[rip+context]
    mov edx,2
    mov r8d,3
    mov r9d,4
    mov qword ptr [rsp+32],1
    call reuse
    lea rax,[rip+context]
    cmp dword ptr [rax],5
    jne .Lfail
    cmp dword ptr [rax+4],1
    jne .Lfail
    cmp dword ptr [rax+8],12
    jne .Lfail
    xor edi,edi
    jmp .Lexit
.Lfail:
    mov edi,1
.Lexit:
    mov eax,60
    syscall
.size _start,.-_start
.bss
.align 8
context:
    .zero 32
.section .note.GNU-stack,"",@progbits
