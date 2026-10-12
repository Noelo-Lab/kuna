/* Authored ARMv5T reproduction; contains no game bytes. */
.syntax unified
.arch armv5t
.arm
.text

.balign 4
.global arm_switch_conditional
.type arm_switch_conditional, %function
arm_switch_conditional:
    /* Inputs 58..61 select four entries; all other signed ints default. */
    cmp     r0, #0
    blt     .Lcond_default
    cmp     r0, #61
    bgt     .Lcond_default
    subs    r0, r0, #58
    /* In ARM state PC reads current address + 8, the first B entry below. */
    addpl   pc, pc, r0, lsl #2
    b       .Lcond_default
.Lcond_table:
    b       .Lcond_case0
    b       .Lcond_case1
    b       .Lcond_case2
    b       .Lcond_case3
.Lcond_case0:
    mov     r0, #11
    bx      lr
.Lcond_case1:
    mov     r0, #12
    bx      lr
.Lcond_case2:
    mov     r0, #13
    bx      lr
.Lcond_case3:
    mov     r0, #14
    bx      lr
.Lcond_default:
    mvn     r0, #0
    bx      lr
.size arm_switch_conditional, .-arm_switch_conditional

.balign 4
.global arm_switch_unconditional
.type arm_switch_unconditional, %function
arm_switch_unconditional:
    /* Control: the same B-entry table with an unconditional ADD PC. */
    cmp     r0, #0
    blt     .Luncond_default
    cmp     r0, #3
    bgt     .Luncond_default
    add     pc, pc, r0, lsl #2
    b       .Luncond_default
.Luncond_table:
    b       .Luncond_case0
    b       .Luncond_case1
    b       .Luncond_case2
    b       .Luncond_case3
.Luncond_case0:
    mov     r0, #21
    bx      lr
.Luncond_case1:
    mov     r0, #22
    bx      lr
.Luncond_case2:
    mov     r0, #23
    bx      lr
.Luncond_case3:
    mov     r0, #24
    bx      lr
.Luncond_default:
    mvn     r0, #0
    bx      lr
.size arm_switch_unconditional, .-arm_switch_unconditional

.global arm_switch_addls
.type arm_switch_addls, %function
arm_switch_addls:
    cmp     r0, #3
    addls   pc, pc, r0, lsl #2
    b       .Lls_default
    b       .Lls_case0
    b       .Lls_case1
    b       .Lls_case2
    b       .Lls_case3
.Lls_case0:
    mov     r0, #31
    bx      lr
.Lls_case1:
    mov     r0, #32
    bx      lr
.Lls_case2:
    mov     r0, #33
    bx      lr
.Lls_case3:
    mov     r0, #34
    bx      lr
.Lls_default:
    mvn     r0, #0
    bx      lr
.size arm_switch_addls, .-arm_switch_addls

.global arm_switch_frame
.type arm_switch_frame, %function
arm_switch_frame:
    push    {r4, lr}
    mov     r4, r1
    cmp     r0, #0
    blt     .Lframe_default
    cmp     r0, #61
    bgt     .Lframe_default
    subs    r0, r0, #58
    addpl   pc, pc, r0, lsl #2
    b       .Lframe_default
    b       .Lframe_case0
    b       .Lframe_case1
    b       .Lframe_case2
    b       .Lframe_case3
.Lframe_case0:
    mov     r0, #41
    b       .Lframe_return
.Lframe_case1:
    mov     r0, #42
    b       .Lframe_return
.Lframe_case2:
    mov     r0, #43
    b       .Lframe_return
.Lframe_case3:
    mov     r0, #44
    b       .Lframe_return
.Lframe_default:
    mvn     r0, #0
.Lframe_return:
    add     r0, r0, r4
    pop     {r4, pc}
.size arm_switch_frame, .-arm_switch_frame

.global arm_indirect_call
.type arm_indirect_call, %function
arm_indirect_call:
    push    {r4, lr}
    ldr     r4, [r0]
    mov     r0, r1
    blx     r4
    add     r0, r0, #1
    pop     {r4, pc}
.size arm_indirect_call, .-arm_indirect_call
