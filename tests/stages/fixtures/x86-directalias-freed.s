# A stack pointer saved in a frame slot, restored, moved below the
# outgoing-argument area and used both as a store's address and as the %rdi
# of a call to nouse, which never reads %rdi.  Source of the bytes in
# kuna-x86-directalias-freed.xml (as --64; objcopy -O binary -j .text).
#
#   freed_call: w7(1,2,3,4,5,6, 7);
#               p = restored - 48; *p = 0x55; nouse(p);
#               y8(1,2,3,4,5,6, 7,8);
    .intel_syntax noprefix
    .text
    .globl freed_call
freed_call:
    push rbp
    mov rbp, rsp
    sub rsp, 48
    mov qword ptr [rsp], 7
    mov edi, 1
    mov esi, 2
    mov edx, 3
    mov ecx, 4
    mov r8d, 5
    mov r9d, 6
    call w7
    lea rax, [rsp+32]
    mov [rbp-16], rax
    mov rdi, [rbp-16]
    sub rdi, 48
    mov dword ptr [rdi], 0x55
    call nouse
    mov qword ptr [rsp+8], 8
    mov qword ptr [rsp], 7
    mov edi, 1
    mov esi, 2
    mov edx, 3
    mov ecx, 4
    mov r8d, 5
    mov r9d, 6
    call y8
    leave
    ret
    .org 0x100, 0x90
nouse:
    xor eax, eax
    ret
    .org 0x120, 0x90
w7:
    lea eax, [rdi+rsi]
    add eax, edx
    add eax, ecx
    add eax, r8d
    add eax, r9d
    add eax, [rsp+8]
    ret
    .org 0x140, 0x90
y8:
    lea eax, [rdi+rsi]
    add eax, edx
    add eax, ecx
    add eax, r8d
    add eax, r9d
    add eax, [rsp+8]
    add eax, [rsp+16]
    ret
    .org 0x160, 0x90
