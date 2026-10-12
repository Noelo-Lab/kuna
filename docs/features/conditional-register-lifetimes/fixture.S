/* Authored ARMv5 guard and register-lifetime regression; no external assets. */
.syntax unified
.arch armv5te
.arm
.text
.global candidate
.type candidate,%function
candidate:
 push {r4,lr}
 mov r4,r0
 bl valid
 cmp r0,#0
 ldrne r1,[r4,#4]
 ldrbne r0,[r1]
 cmpne r0,#0
 beq skip
 ldr r0,[r1,#4]
 pop {r4,pc}
skip:
 mov r0,#0
 pop {r4,pc}
.size candidate,.-candidate
.global valid
.type valid,%function
valid:
 cmp r0,#0
 ldrne r0,[r0]
 bx lr
.size valid,.-valid
.global scan
.type scan,%function
scan:
 push {r4,r5,r6,r7,r8,lr}
 mov r4,r0
 mov r5,r1
 mov r6,#0
 cmp r5,#0
 beq scan_done
scan_loop:
 ldr r7,[r4],#4
 mov r0,r7
 bl valid
 cmp r0,#0
 ldrne r1,[r7,#4]
 ldrbne r0,[r1]
 cmpne r0,#0
 beq scan_next
 ldr r0,[r1,#4]
 add r6,r6,r0
scan_next:
 subs r5,r5,#1
 bne scan_loop
scan_done:
 mov r0,r6
 pop {r4,r5,r6,r7,r8,pc}
.size scan,.-scan

.syntax unified
.arch armv5te
.arm
.text
.global unconditional
.type unconditional,%function
unconditional:
 ldr r1,[r0]
 ldr r0,[r1,#4]
 bx lr
.size unconditional,.-unconditional
.global retained
.type retained,%function
retained:
 cmp r0,#0
 movne r1,#19
 mov r0,r1
 bx lr
.size retained,.-retained
.global distinct
.type distinct,%function
distinct:
 cmp r0,#0
 ldrne r3,[r1]
 ldreq r3,[r2]
 ldr r0,[r3,#4]
 bx lr
.size distinct,.-distinct
.global nested
.type nested,%function
nested:
 cmp r0,#0
 bxeq lr
 ldr r2,[r0]
 cmp r2,#0
 moveq r0,#0
 bxeq lr
 ldr r1,[r0,#4]
 ldrb r2,[r1]
 cmp r2,#0
 moveq r0,#0
 bxeq lr
 ldr r2,[r1,#4]
 cmp r2,#10
 ldrhi r0,[r1,#8]
 movls r0,#0
 bx lr
.size nested,.-nested
.global unknown_used
.type unknown_used,%function
unknown_used:
 push {r4,lr}
 mov r4,r0
 bl valid
 cmp r0,#0
 movne r1,#19
 mov r0,r1
 pop {r4,pc}
.size unknown_used,.-unknown_used
.global remember
.type remember,%function
remember:
 push {r4,r5,r6,r7,r8,lr}
 mov r4,r0
 mov r5,r1
 mov r8,r2
 cmp r5,#0
 beq remember_done
remember_loop:
 ldr r7,[r4],#4
 mov r0,r7
 bl valid
 cmp r0,#0
 ldrne r1,[r7,#4]
 ldrbne r0,[r1]
 cmpne r0,#0
 movne r8,r1
 subs r5,r5,#1
 bne remember_loop
remember_done:
 ldr r0,[r8,#4]
 pop {r4,r5,r6,r7,r8,pc}
.size remember,.-remember
.global unknown_loop
.type unknown_loop,%function
unknown_loop:
 push {r4,r5,r6,r7,r8,lr}
 mov r4,r0
 mov r5,r1
 mov r8,#0
 cmp r5,#0
 beq unknown_loop_done
unknown_loop_next:
 ldr r0,[r4],#4
 bl valid
 cmp r0,#0
 movne r8,r1
 subs r5,r5,#1
 bne unknown_loop_next
unknown_loop_done:
 mov r0,r8
 pop {r4,r5,r6,r7,r8,pc}
.size unknown_loop,.-unknown_loop
