.syntax unified
.arch armv7-a
.arm
.text
.global wrapper, provider, consumer, void_wrapper, sink
.type wrapper,%function
wrapper:
    push {r4,lr}
    bl provider
    pop {r4,pc}
.size wrapper,.-wrapper
.type provider,%function
provider:
    ldr r0,[r0]
    bx lr
.size provider,.-provider
.type consumer,%function
consumer:
    push {r4,lr}
    bl wrapper
    cmp r0,#0
    moveq r0,#9
    pop {r4,pc}
.size consumer,.-consumer
.type void_wrapper,%function
void_wrapper:
    push {r4,lr}
    bl sink
    pop {r4,pc}
.size void_wrapper,.-void_wrapper
.type sink,%function
sink:
    str r1,[r0]
    bx lr
.size sink,.-sink
.section .note.GNU-stack,"",%progbits
