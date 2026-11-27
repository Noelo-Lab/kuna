@ armfloatargs merged-store fixture: glibc's printf_size shape, a d-register
@ the function writes only in a loop and stores only on the paths where the
@ loop ran. Its incoming value reaches the store through the loop's merge, on
@ a path that never runs, so it is not a parameter.
@
@ arm-linux-gnueabihf-as armfloatargs_merged_store.s -o armfloatargs_merged_store.o
@
@ void scaled(const double *p, const char *s): d7 = *p, then for each byte
@ of s, d5 = d7 / 1024.0 and r4 = 1; afterwards, if r4, the slot at sp is
@ d5; the slot's address goes to sink.
@
@ void put_mmio(double v): stores d0 to 0x50000000, which a test marks
@ volatile, so the store is a write_volatile of the incoming d0.
    .syntax unified
    .arch armv7-a
    .fpu vfpv3-d16
    .eabi_attribute 28, 1
    .text
    .global scaled
    .type scaled,%function
scaled:
    push {r4, lr}
    sub sp, sp, #8
    vldr d7, [r0]
    vstr d7, [sp]
    vldr d6, .Lk
    mov r4, #0
1:  ldrb r2, [r1], #1
    cmp r2, #0
    beq 2f
    vdiv.f64 d5, d7, d6
    vmov.f64 d7, d5
    mov r4, #1
    b 1b
2:  cmp r4, #0
    beq 3f
    vstr d5, [sp]
3:  mov r0, sp
    bl sink
    add sp, sp, #8
    pop {r4, pc}
    .align 3
.Lk: .double 1024.0
    .size scaled, .-scaled

    .global put_mmio
    .type put_mmio,%function
put_mmio:
    movw r3, #0
    movt r3, #0x5000
    vstr d0, [r3]
    bx lr
    .size put_mmio, .-put_mmio
