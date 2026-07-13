/* AArch64 test fixture. Freestanding (no libc) so it cross-compiles to an ELF
 * object without an aarch64 sysroot/linker — the decompiler only needs the code.
 *
 * Rebuild:  clang -c --target=aarch64-linux-gnu -O0 -o sample_aarch64.o sample_aarch64.c
 * (produces an ELF ET_REL, ARM aarch64; kuna loads it via its relocobjects path.)
 */
static int add(int a, int b) { return a + b; }

static long sum_to(int n) {
    long acc = 0;
    for (int i = 1; i <= n; i++) acc += i;
    return acc;
}

void _start(void) {
    volatile long r = sum_to(add(2, 3));
    register long x0 asm("x0") = r;
    register long x8 asm("x8") = 93; /* __NR_exit */
    asm volatile("svc 0" ::"r"(x0), "r"(x8));
}
