volatile unsigned call_count;
volatile unsigned ordered_digest;

__attribute__((noinline,used)) unsigned pre_three(unsigned a,unsigned b,unsigned c) {
    call_count++;
    ordered_digest = ordered_digest * 33u + a * 3u + b * 5u + c * 7u;
    return a + b + c;
}

__attribute__((noinline,used)) unsigned observe(unsigned char *p,unsigned n) {
    call_count++;
    ordered_digest = ordered_digest * 33u + (unsigned)p[0] + n;
    return (unsigned)p[0] + n;
}

/* The unused saved r3 is alignment/callee-save storage, not a fifth argument. */
__attribute__((naked,noinline,used)) unsigned sample(unsigned n) {
    __asm__ volatile(
        ".syntax unified\n"
        "push {r3,r4,r5,r6,r7,r8,r9,lr}\n"
        "mov r7,sp\n"
        "mov r4,r0\n"
        "movs r1,#12\n"
        "movs r2,#9\n"
        "bl pre_three\n"
        "adds r5,r4,#7\n"
        "bic r5,r5,#7\n"
        "sub sp,sp,r5\n"
        "mov r0,sp\n"
        "strb r4,[r0]\n"
        "mov r1,r4\n"
        "bl observe\n"
        "mov sp,r7\n"
        "pop {r3,r4,r5,r6,r7,r8,r9,pc}\n");
}
