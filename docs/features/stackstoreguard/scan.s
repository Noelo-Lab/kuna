.syntax unified
.arm
.text
.global inspect_bytes
.type inspect_bytes,%function
inspect_bytes:
    push {r4, lr}
    sub sp, sp, #8
    mov r1, sp
    mov r2, #0
    str r2, [sp]
    mov r3, #3
1:
    ldrb r2, [r0], #1
    strb r2, [r1], #1
    subs r3, r3, #1
    bne 1b
    ldrb r0, [sp]
    cmp r0, #0
    beq 2f
    bl helper
2:
    add sp, sp, #8
    pop {r4, lr}
    bx lr
.size inspect_bytes,.-inspect_bytes

.global helper
.type helper,%function
helper:
    mov r1, #0x50000000
    str r0, [r1]
    bx lr
.size helper,.-helper
