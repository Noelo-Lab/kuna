        .text

        .global guarded_status
        .type guarded_status, %function
guarded_status:
        stp     x29, x30, [sp, #-16]!
        mov     x29, sp
        adrp    x3, guard_before
        ldr     w2, [x3, :lo12:guard_before]
        cmp     w0, #0
        csetm   w0, ne
        adrp    x3, guard_after
        ldr     w1, [x3, :lo12:guard_after]
        cmp     w2, w1
        b.ne    .Lfail
        ldp     x29, x30, [sp], #16
        ret
.Lfail:
        bl      __stack_chk_fail
        .size guarded_status, .-guarded_status

        .global status_caller
        .type status_caller, %function
status_caller:
        stp     x29, x30, [sp, #-16]!
        mov     x29, sp
        bl      guarded_status
        cmp     w0, #0
        cset    w0, ne
        ldp     x29, x30, [sp], #16
        ret
        .size status_caller, .-status_caller

        .data
        .balign 4
guard_before:
        .word   0x12345678
guard_after:
        .word   0x12345678
