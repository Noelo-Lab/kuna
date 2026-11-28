// AArch64 callers that hand a stated parameter on untouched to a call.
// Assemble: aarch64-linux-gnu-as -o wholeinputarg_a64.o wholeinputarg_a64.s
// Declarations for --types: wholeinputarg_a64.h (g is left undeclared).
    .text
    .global g, f2, fwd2, sfld
    .type g, %function
g:                      // recovered as int g(int a0): reads only w0
    add  w0, w0, #5
    ret
    .type f2, %function
f2:                     // unsigned int f2(long n): return g((int)n); x0 untouched
    stp  x29, x30, [sp, #-16]!
    mov  x29, sp
    bl   g
    ldp  x29, x30, [sp], #16
    ret
    .type fwd2, %function
fwd2:                   // unsigned int fwd2(char *p, void *fp): tail-calls fp(p)
    br   x1
    .type sfld, %function
sfld:                   // int sfld(struct S s, void *fp): tail-calls fp(s.a)
    br   x1
