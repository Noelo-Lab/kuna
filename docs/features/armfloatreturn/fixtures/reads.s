.syntax unified
.arch armv7-a
.arm
.text
.global read_four, captured_count
.type read_four,%function
read_four:
    push {r4,lr}
    movw r1,#0
    movt r1,#0x5000
    ldm r1,{r0,r2,r3,r4}
    pop {r4,pc}
.size read_four,.-read_four
.global read_all, read_scalar
.type read_all,%function
read_all:
    push {r4,lr}
    movw r1,#0
    movt r1,#0x5000
    ldm r1,{r0,r2,r3,r4}
    add r0,r0,r2
    add r0,r0,r3
    add r0,r0,r4
    pop {r4,pc}
.size read_all,.-read_all
.type read_scalar,%function
read_scalar:
    push {r4,lr}
    movw r1,#0
    movt r1,#0x5000
    ldr r0,[r1]
    ldr r2,[r1,#4]
    ldr r3,[r1,#8]
    ldr r4,[r1,#12]
    pop {r4,pc}
.size read_scalar,.-read_scalar
.type captured_count,%function
captured_count:
    movw r2,#0
    movt r2,#0x5000
    ldr r1,[r2]
    mov r0,#0
1:
    cmp r0,r1
    bhs 2f
    str r0,[r2,#4]
    add r0,r0,#1
    b 1b
2:
    bx lr
.size captured_count,.-captured_count
.section .note.GNU-stack,"",%progbits
