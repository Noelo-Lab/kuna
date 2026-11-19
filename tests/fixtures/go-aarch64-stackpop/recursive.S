.text
.p2align 2
.global recursive
.type recursive, %function
recursive:
.Lrecursive:
    cmp x0, #1
    b.le .Lreturn
    stp x29, x30, [sp, #-32]!
    mov x29, sp
    str x0, [sp, #16]
    sub x0, x0, #1
    bl .Lrecursive
    str x0, [sp, #24]
    ldr x0, [sp, #16]
    sub x0, x0, #2
    bl .Lrecursive
    ldr x1, [sp, #24]
    add x0, x0, x1
    ldp x29, x30, [sp], #32
.Lreturn:
    ret
.size recursive, .-recursive
