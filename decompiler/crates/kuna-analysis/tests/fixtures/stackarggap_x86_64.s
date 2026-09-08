# Fixture for the `stackarggap` option (RE-friction need
# sha-finalization-receives-two).  `hash_final` calls a three-argument helper
# through a data slot -- the shape of a body-less import, whose arity the engine
# has to infer from the call site.  The SysV argument registers RDI/RSI/RDX/RCX/R8
# are all loaded, R9 is never written, and the frame leaves a 1 in the first
# outgoing stack slot.  With `stackarggap` off, forceInactiveChain fills the R9
# hole to reach that stack slot and the call gains two arguments the caller never
# passes; with it on the list ends at R9.
    .text
    .globl  _start
_start:
    lea     ctx(%rip), %rdi
    call    hash_final
    mov     $60, %eax
    xor     %edi, %edi
    syscall

    .globl  hash_final
    .type   hash_final, @function
hash_final:
    sub     $0x28, %rsp
    movq    $1, (%rsp)
    mov     (%rdi), %r8
    mov     8(%rdi), %rcx
    mov     16(%rdi), %rdx
    mov     24(%rdi), %rsi
    mov     32(%rdi), %rdi
    call    *slot(%rip)
    add     $0x28, %rsp
    ret
    .size   hash_final, .-hash_final

    .data
    .align 8
slot:
    .quad   0
ctx:
    .quad   0, 0, 0, 0, 0
