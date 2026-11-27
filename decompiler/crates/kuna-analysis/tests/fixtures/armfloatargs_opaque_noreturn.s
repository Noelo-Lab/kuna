@ armfloatargs opaque-noreturn fixture: glibc strtod's round_and_return
@ shape, two stack-guard checks that share one call to __stack_chk_fail, which
@ never returns. On one path d0 still holds the entry double, on the other the
@ result the function returns, so d0 at the call merges the two. A call that
@ does not return takes no double: neither function passes d0 there, and each
@ keeps its prototype.
@
@ arm-linux-gnueabihf-as armfloatargs_opaque_noreturn.s -o armfloatargs_opaque_noreturn.o
@
@ double guard_shared(double d, int k, int j): if k, d = d + d; then if j the
@ no-return __stack_chk_fail, otherwise return d.
@ double tail_shared(double d, int k, int j): the same, except that when k is
@ zero and j is zero it tail-calls tail with d still in d0, as round_and_return
@ tail-calls the routine that builds its result. As in glibc, __stack_chk_fail
@ is defined here and calls on, so its body does not answer for d0.
    .syntax unified
    .arch armv7-a
    .fpu vfpv3-d16
    .eabi_attribute 28, 1
    .text
    .global guard_shared
    .type guard_shared,%function
guard_shared:
    push {r4, lr}
    cmp r0, #0
    beq 1f
    vadd.f64 d0, d0, d0
    cmp r1, #0
    bne 2f
    pop {r4, pc}
1:  cmp r1, #0
    bne 2f
    pop {r4, pc}
2:  bl __stack_chk_fail
    .size guard_shared, .-guard_shared

    .global tail_shared
    .type tail_shared,%function
tail_shared:
    push {r4, lr}
    cmp r0, #0
    beq 1f
    vadd.f64 d0, d0, d0
    cmp r1, #0
    bne 2f
    pop {r4, pc}
1:  cmp r1, #0
    bne 2f
    pop {r4, lr}
    b tail
2:  bl __stack_chk_fail
    .size tail_shared, .-tail_shared

    .global __stack_chk_fail
    .type __stack_chk_fail,%function
__stack_chk_fail:
    push {r4, lr}
    bl report_fail
1:  b 1b
    .size __stack_chk_fail, .-__stack_chk_fail
