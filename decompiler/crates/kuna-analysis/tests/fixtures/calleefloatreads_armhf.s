@ calleefloatreads_armhf.o: clang -target thumbv7em-none-eabihf -mcpu=cortex-m4
@   -mfpu=fpv4-sp-d16 -mfloat-abi=hard -c calleefloatreads_armhf.s
@ (clang 14). The shape of newlib's fmodf calling __ieee754_fmodf: outer2
@ hands its own s0 and s1 to inner2, which reads s0 and only spills s1 with
@ vstr before reloading it as an integer. Both are arguments.
    .syntax unified
    .cpu cortex-m4
    .fpu fpv4-sp-d16
    .eabi_attribute 28, 1
    .thumb
    .text

    .globl inner2
    .type inner2,%function
    .thumb_func
inner2:
    push {r4, lr}
    sub sp, #8
    vstr s1, [sp, #4]
    ldr r2, [sp, #4]
    vmov r4, s0
    movs r0, #0
    movs r1, #0
    movs r3, #0
    adds r0, r2, r4
    vmov s0, r0
    add sp, #8
    pop {r4, pc}
    .size inner2, .-inner2

    .globl outer2
    .type outer2,%function
    .thumb_func
outer2:
    push {r3, lr}
    vpush {d8}
    vmov.f32 s16, s1
    vmov.f32 s17, s0
    bl inner2
    vcmp.f32 s17, s16
    vmrs APSR_nzcv, fpscr
    bvs 1f
    vpop {d8}
    pop {r3, pc}
1:  vdiv.f32 s0, s17, s17
    vpop {d8}
    pop {r3, pc}
    .size outer2, .-outer2
