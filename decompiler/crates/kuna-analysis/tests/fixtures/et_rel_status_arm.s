        .syntax unified
        .text

        .global guarded_status
        .type guarded_status, %function
guarded_status:
        push    {r4, lr}
        ldr     r3, .Lguard_before_ptr
        ldr     r2, [r3]
        cmp     r0, #0
        moveq   r0, #0
        mvnne   r0, #0
        ldr     r3, .Lguard_after_ptr
        ldr     r1, [r3]
        cmp     r2, r1
        bne     .Lfail
        pop     {r4, pc}
.Lfail:
        bl      __stack_chk_fail
.Lguard_before_ptr:
        .word   guard_before
.Lguard_after_ptr:
        .word   guard_after
        .size guarded_status, .-guarded_status

        .global status_caller
        .type status_caller, %function
status_caller:
        push    {r4, lr}
        bl      guarded_status
        cmp     r0, #0
        movne   r0, #1
        pop     {r4, pc}
        .size status_caller, .-status_caller

        .data
        .balign 4
guard_before:
        .word   0x12345678
guard_after:
        .word   0x12345678
