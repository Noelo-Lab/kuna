# Takes the ADDRESS of two .got import slots (lea sym@GOTPCREL), selects one
# and jumps through it: the constant is the slot, not the import.
    .text
    .globl pickx
    .type pickx,@function
pickx:
    endbr64
    lea lgetxattr@GOTPCREL(%rip), %rax
    lea getxattr@GOTPCREL(%rip), %r8
    test %esi, %esi
    cmovne %r8, %rax
    lea .Lname(%rip), %rsi
    mov $8, %ecx
    jmp *(%rax)
    .size pickx, .-pickx
    .globl main
    .type main,@function
main:
    sub $24, %rsp
    mov (%rsi), %rdi
    xor %esi, %esi
    cmp $1, %edi
    setg %sil
    mov %rsp, %rdx
    call pickx
    add $24, %rsp
    ret
    .size main, .-main
    .section .rodata
.Lname: .string "user.x"
    .section .note.GNU-stack,"",@progbits
