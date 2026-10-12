.syntax unified
.arm
.text
.global BuildFrame
.type BuildFrame,%function
BuildFrame:
    push {r4, lr}
    sub sp, sp, #24
    mov r4, r0
    mov r2, #0
    mov r1, #0
    mov r0, sp
zero_loop:
    strb r1, [r0, r2]
    add r2, r2, #1
    cmp r2, #22
    blt zero_loop
    add r0, sp, #4
    mov r1, #17
    bl FillPayload
    add r0, sp, #4
    mov r1, #17
    bl PayloadSum
    str r0, [sp]
    mov r0, r4
    mov r1, sp
    mov r2, #22
    bl CopyFrame
    add sp, sp, #24
    pop {r4, pc}
.size BuildFrame, .-BuildFrame
.global FillPayload
.type FillPayload,%function
FillPayload:
    mov r2, #1
fill_loop:
    strb r2, [r0], #1
    add r2, r2, #1
    subs r1, r1, #1
    bne fill_loop
    bx lr
.size FillPayload, .-FillPayload
.global PayloadSum
.type PayloadSum,%function
PayloadSum:
    mov r2, #0
sum_loop:
    ldrb r3, [r0], #1
    add r2, r2, r3
    subs r1, r1, #1
    bne sum_loop
    mov r0, r2
    bx lr
.size PayloadSum, .-PayloadSum
.global CopyFrame
.type CopyFrame,%function
CopyFrame:
copy_loop:
    ldrb r3, [r1], #1
    strb r3, [r0], #1
    subs r2, r2, #1
    bne copy_loop
    bx lr
.size CopyFrame, .-CopyFrame

/* Assemble and link at 0x10000 to reproduce contiguous_byte_frame.hex:
   arm-none-eabi-as -march=armv5te -o /tmp/frame.o contiguous_byte_frame.s
   arm-none-eabi-ld -Ttext=0x10000 -e BuildFrame -o /tmp/frame.elf /tmp/frame.o
   arm-none-eabi-objcopy -O binary /tmp/frame.elf /tmp/frame.bin
   xxd -p -c 164 /tmp/frame.bin

   BuildFrame writes checksum 153, payload bytes 1..17, and a trailing zero.
   Its 22-byte copy must read one C object, including the four-byte header.
*/
