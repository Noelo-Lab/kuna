// The arm64 lowering of a select on the error register after a throwing
// call (swiftc's x86-64 output for `(try? f(x)) ?? 7` on a typed throw is
// `test r12, r12; cmovne`): x21 is compared with the immediate zero.
    .text
    .globl  select_error
    .type   select_error,@function
    .p2align 2
select_error:
    stp     x29, x30, [sp, #-32]!
    str     x21, [sp, #16]
    mov     x29, sp
    mov     x21, xzr
    bl      "$ss5riskyyBi64_Bi64_KF"
    mov     x8, #7
    cmp     x21, #0
    csel    x0, x8, x0, ne
    ldr     x21, [sp, #16]
    ldp     x29, x30, [sp], #32
    ret
    .size   select_error, .-select_error
