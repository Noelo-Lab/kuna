@ armfloatargs opaque-split fixture: glibc's acospif shape, a float computed
@ into s0 for a callee nothing describes, in a function that also writes d0
@ whole, so the call's d0 range is a piece of the new s0 and the s1 the caller
@ left. That is a float argument, not a double.
@
@ arm-linux-gnueabihf-as armfloatargs_opaque_split.s -o armfloatargs_opaque_split.o
@
@ float pieced_f(float x, int k): if k, tail-call invalidf((x - x) / (x - x));
@ otherwise return (float)((double)x * 1.5).
@
@ double guard_d(double d, int k), glibc's drand48 shape: d0 = d + d, then if k
@ the no-return __stack_chk_fail, with that pending result still in d0, which
@ is not its argument. As in glibc, __stack_chk_fail is defined here and calls
@ on, so its body does not answer for d0. guard_spin has the same shape around
@ spin, an unnamed loop that never returns.
    .syntax unified
    .arch armv7-a
    .fpu vfpv3-d16
    .eabi_attribute 28, 1
    .text
    .global pieced_f
    .type pieced_f,%function
pieced_f:
    cmp r0, #0
    beq 1f
    vsub.f32 s0, s0, s0
    vdiv.f32 s0, s0, s0
    b invalidf
1:  vcvt.f64.f32 d7, s0
    vmov.f64 d0, #1.5
    vmul.f64 d7, d7, d0
    vcvt.f32.f64 s0, d7
    bx lr
    .size pieced_f, .-pieced_f

    .global guard_d
    .type guard_d,%function
guard_d:
    vadd.f64 d0, d0, d0
    cmp r0, #0
    bxeq lr
    push {r4, lr}
    bl __stack_chk_fail
    .size guard_d, .-guard_d

    .global __stack_chk_fail
    .type __stack_chk_fail,%function
__stack_chk_fail:
    push {r4, lr}
    bl report_fail
1:  b 1b
    .size __stack_chk_fail, .-__stack_chk_fail

    .type spin,%function
spin:
    push {r4, lr}
    bl report_fail
1:  b 1b
    .size spin, .-spin

    .global guard_spin
    .type guard_spin,%function
guard_spin:
    vadd.f64 d0, d0, d0
    cmp r0, #0
    bxeq lr
    push {r4, lr}
    bl spin
    pop {r4, pc}
    .size guard_spin, .-guard_spin
