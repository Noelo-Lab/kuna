/* callbacktype: a callback whose computed return is narrower than the slot's
 * (x86-64).
 *
 * The slot's declared return is also the width the declaration prints. When a
 * body computes fewer bytes, the declaration prints the rest of the register
 * too, and whether those bytes are clean depends on the machine code, not on
 * the source type.
 *
 * Refused (the bytes above the value are whatever the caller left):
 *   `ccmp` (clang -O2): `mov (%rdi),%al; sub (%rsi),%al; ret`, a `signed char`
 *   comparator cast into `qsort`'s `int` slot. Parked, it printed
 *   `CONCAT31((undefined3)((unsigned int)v1 >> 8), ...)` with `v1` never set.
 *   `cw` (clang -O2): `mov (%rdi),%al; inc %al; ret` in `pthread_create`'s
 *   `void *` slot.
 *   `bw` (gcc -O2): `cmpq $0,(%rdi); setg %al; ret`, a `bool` routine in
 *   `pthread_create`'s slot. Parked, it printed `(void *)CONCAT71(v1, ...)`.
 *
 * Parked (the bytes above the value are zero on every path):
 *   `zcmp` (gcc -O2): `movzbl (%rdi),%eax; sub (%rsi),%al; ret`.
 *   `zw` (gcc -O2): `xor %eax,%eax; cmpq $0,(%rdi); setg %al; ret`.
 *   `iw` (gcc -O2): an `int` routine; a 32-bit write clears the upper half.
 *   `lw` (gcc -O2): `mov (%rdi),%eax; test %eax,%eax; setg %al; ret`. The
 *   bytes above `al` hold the loaded value, but the body's own recovery
 *   already returns all four bytes of `eax` (it prints the same `CONCAT31`
 *   with the option off), and the 32-bit load clears the half the `void *`
 *   adds.
 *
 * Build (symbols kept, no DWARF):
 *   clang -O2 -DCLANG_PART -fno-stack-protector -fcf-protection=none \
 *       -c -o narrow_clang.o callbacktype_narrow_x86_64.c
 *   gcc -O2 -fno-inline -fno-stack-protector -fcf-protection=none \
 *       -c -o narrow_gcc.o callbacktype_narrow_x86_64.c
 *   gcc -o callbacktype_narrow_x86_64 narrow_gcc.o narrow_clang.o -lpthread
 */
#include <pthread.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

#ifdef CLANG_PART

__attribute__((noinline)) signed char ccmp(const void *a, const void *b) {
    return *(const signed char *)a - *(const signed char *)b;
}

__attribute__((noinline)) char cw(void *p) {
    return *(char *)p + 1;
}

#else

signed char ccmp(const void *a, const void *b);
char cw(void *p);

__attribute__((noinline, noipa)) static bool bw(void *p) {
    return *(long *)p > 0;
}

__attribute__((noinline, noipa)) static bool lw(void *p) {
    return *(int *)p > 0;
}

__attribute__((noinline, noipa)) static signed char zcmp(const void *a, const void *b) {
    return *(const signed char *)a - *(const signed char *)b;
}

__attribute__((noinline, noipa)) static int zw(void *p) {
    return *(long *)p > 0;
}

__attribute__((noinline, noipa)) static int iw(void *p) {
    return *(int *)p ^ 5;
}

int main(int argc, char **argv) {
    pthread_t t;
    void *r[5];
    long v = argc > 1 ? strtol(argv[1], 0, 10) : argc;
    signed char c[4] = { (signed char)argc, 3, 2, 1 };
    qsort(c, 4, 1, (int (*)(const void *, const void *))ccmp);
    qsort(c, 4, 1, (int (*)(const void *, const void *))zcmp);
    pthread_create(&t, 0, (void *(*)(void *))bw, &v); pthread_join(t, &r[0]);
    pthread_create(&t, 0, (void *(*)(void *))cw, &v); pthread_join(t, &r[1]);
    pthread_create(&t, 0, (void *(*)(void *))lw, &v); pthread_join(t, &r[2]);
    pthread_create(&t, 0, (void *(*)(void *))zw, &v); pthread_join(t, &r[3]);
    pthread_create(&t, 0, (void *(*)(void *))iw, &v); pthread_join(t, &r[4]);
    printf("%d %p %p %p %p %p\n", c[0], r[0], r[1], r[2], r[3], r[4]);
    return 0;
}

#endif
