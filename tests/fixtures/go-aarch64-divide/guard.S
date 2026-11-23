.text

#ifdef __aarch64__
.p2align 6
guarded_quotient:
    cbz x1, .Lguarded_fail
    sdiv x0, x0, x1
    ret
.Lguarded_fail:
    bl .Lpanicdivide
    brk #1

.p2align 6
raw_quotient:
    sdiv x0, x0, x1
    ret

.p2align 6
bounds_control:
    cmp x1, x2
    b.hs .Lbounds_fail
    ldr x0, [x0, x1, lsl #3]
    ret
.Lbounds_fail:
    bl .LpanicIndex
    brk #1

.p2align 6
application_guard:
    cbz x1, .Lapplication_fail
    sdiv x0, x0, x1
    ret
.Lapplication_fail:
    bl .LapplicationPanic
    brk #1

.p2align 6
mixed_panics:
    cbz x1, .Ldivide_only
    bl .LpanicIndex
    brk #1
.Ldivide_only:
    bl .Lpanicdivide
    brk #1

.p2align 6
abi_quotient:
    cbz x1, .Labi_fail
    sdiv x0, x0, x1
    ret
.Labi_fail:
    bl .LpanicdivideABI0
    brk #1

.p2align 6
.Lpanicdivide:
    brk #1
.LpanicIndex:
    brk #1
.LapplicationPanic:
    brk #1
.LpanicdivideABI0:
    brk #1

#else
.p2align 6
guarded_quotient:
    test %rsi, %rsi
    jz .Lguarded_fail
    mov %rdi, %rax
    cqto
    idiv %rsi
    ret
.Lguarded_fail:
    call .Lpanicdivide
    ud2

.p2align 6
.Lpanicdivide:
    ud2
#endif
