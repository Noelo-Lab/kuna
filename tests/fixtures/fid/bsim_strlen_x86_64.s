# Instruction bytes of kuna_strlen from the x86-64 GCC 15.3 -O2 FID sample.
# The remote BSim smoke test compares stock Ghidra 12.1.2 signature hashes.
.intel_syntax noprefix
.text
.globl kuna_strlen
.type kuna_strlen, @function
kuna_strlen:
    cmp byte ptr [rdi], 0
    je .Lempty
    mov rax, rdi
    .byte 0x0f, 0x1f, 0x84, 0x00, 0x00, 0x00, 0x00, 0x00
.Lloop:
    add rax, 1
    cmp byte ptr [rax], 0
    jne .Lloop
    sub rax, rdi
    ret
    .byte 0x0f, 0x1f, 0x00
.Lempty:
    xor eax, eax
    ret
.size kuna_strlen, . - kuna_strlen
.section .note.GNU-stack,"",@progbits
