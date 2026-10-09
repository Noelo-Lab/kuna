@ Authored ARMv5TE controls for pushed scratch storage and byte-copy aliases (#810).
.syntax unified
.arch armv5te
.arm
.text
.align 2
.global PushedByteSum
.type PushedByteSum,%function
PushedByteSum:
    push    {r3,lr}
    mov     r1,r0
    mov     r2,sp
    mov     r3,#4
1:
    ldrb    r12,[r1],#1
    strb    r12,[r2],#1
    subs    r3,r3,#1
    bne     1b
    ldrb    r0,[sp]
    ldrb    r1,[sp,#1]
    ldrb    r2,[sp,#2]
    add     r0,r0,r1,lsl #1
    add     r0,r0,r2,lsl #2
    pop     {r3,pc}
.size PushedByteSum,.-PushedByteSum

.global StackByteCopyViews
.type StackByteCopyViews,%function
StackByteCopyViews:
    sub     sp,sp,#4
    mov     r1,r0
    mov     r2,sp
    mov     r3,#4
1:
    ldrb    r12,[r1],#1
    strb    r12,[r2],#1
    subs    r3,r3,#1
    bne     1b
    ldr     r0,[sp]
    ldrb    r1,[sp,#1]
    bic     r0,r0,#0xff000000
    orr     r0,r0,r1,lsl #24
    add     sp,sp,#4
    bx      lr
.size StackByteCopyViews,.-StackByteCopyViews

.global PushedByteBranches
.type PushedByteBranches,%function
PushedByteBranches:
    push {r3,r4,r5,lr}
    mov r5,r1
    mov r4,sp
    mov r1,r0
    mov r3,#4
1:
    ldrb r2,[r1],#1
    subs r3,r3,#1
    strb r2,[r4],#1
    bne 1b
    cmp r5,#0
    bne 2f
    ldrb r3,[sp,#1]
    ldrb r5,[sp,#2]
    ldrb r0,[sp]
    add r0,r0,r3,lsl #1
    add r0,r0,r5,lsl #2
    pop {r3,r4,r5,pc}
2:
    ldrb r0,[sp]
    ldrb r1,[sp,#1]
    ldrb r2,[sp,#2]
    add r0,r0,r1,lsl #2
    add r0,r0,r2,lsl #1
    pop {r3,r4,r5,pc}
.size PushedByteBranches,.-PushedByteBranches

.global PushedByteSeed
.type PushedByteSeed,%function
PushedByteSeed:
    mov r3,r1
    push {r3,lr}
    mov r1,r0
    mov r2,sp
    mov r3,#3
1:
    ldrb r12,[r1],#1
    strb r12,[r2],#1
    subs r3,r3,#1
    bne 1b
    ldr r0,[sp]
    pop {r3,pc}
.size PushedByteSeed,.-PushedByteSeed

.global PushedByteSnapshot
.type PushedByteSnapshot,%function
PushedByteSnapshot:
    mov r3,r1
    push {r3,r4,lr}
    ldr r4,[sp]
    mov r1,r0
    mov r2,sp
    mov r3,#4
1:
    ldrb r12,[r1],#1
    strb r12,[r2],#1
    subs r3,r3,#1
    bne 1b
    ldr r0,[sp]
    eor r0,r0,r4
    pop {r3,r4,pc}
.size PushedByteSnapshot,.-PushedByteSnapshot

.global PushedByteOverlap
.type PushedByteOverlap,%function
PushedByteOverlap:
    mov r3,r0
    push {r3,r4,lr}
    mov r1,sp
    add r2,sp,#1
    mov r3,#3
1:
    ldrb r12,[r1],#1
    strb r12,[r2],#1
    subs r3,r3,#1
    bne 1b
    ldr r0,[sp]
    pop {r3,r4,pc}
.size PushedByteOverlap,.-PushedByteOverlap

.global PushedByteBranchesSeed
.type PushedByteBranchesSeed,%function
PushedByteBranchesSeed:
    mov r3,r2
    push {r3,r4,r5,lr}
    mov r5,r1
    mov r4,sp
    mov r1,r0
    mov r3,#4
1:
    ldrb r2,[r1],#1
    subs r3,r3,#1
    strb r2,[r4],#1
    bne 1b
    cmp r5,#0
    bne 2f
    ldrb r3,[sp,#1]
    ldrb r5,[sp,#2]
    ldrb r0,[sp]
    add r0,r0,r3,lsl #1
    add r0,r0,r5,lsl #2
    pop {r3,r4,r5,pc}
2:
    ldrb r0,[sp]
    ldrb r1,[sp,#1]
    ldrb r2,[sp,#2]
    add r0,r0,r1,lsl #2
    add r0,r0,r2,lsl #1
    pop {r3,r4,r5,pc}
.size PushedByteBranchesSeed,.-PushedByteBranchesSeed

.global LargeStackByteCopy
.type LargeStackByteCopy,%function
LargeStackByteCopy:
    sub     sp,sp,#8
    mov     r1,r0
    mov     r2,sp
    mov     r3,#5
1:
    ldrb    r12,[r1],#1
    strb    r12,[r2],#1
    subs    r3,r3,#1
    bne     1b
    ldr     r0,[sp]
    ldrb    r1,[sp,#1]
    bic     r0,r0,#0xff000000
    orr     r0,r0,r1,lsl #24
    add     sp,sp,#8
    bx      lr
.size LargeStackByteCopy,.-LargeStackByteCopy

.global DynamicStackByteCopy
.type DynamicStackByteCopy,%function
DynamicStackByteCopy:
    sub     sp,sp,#4
    mov     r3,r1
    mov     r1,r0
    mov     r2,sp
1:
    ldrb    r12,[r1],#1
    strb    r12,[r2],#1
    subs    r3,r3,#1
    bne     1b
    ldr     r0,[sp]
    ldrb    r1,[sp,#1]
    bic     r0,r0,#0xff000000
    orr     r0,r0,r1,lsl #24
    add     sp,sp,#4
    bx      lr
.size DynamicStackByteCopy,.-DynamicStackByteCopy
