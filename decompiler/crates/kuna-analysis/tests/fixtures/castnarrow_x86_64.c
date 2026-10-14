/* castnarrow fixture: a value narrowed into a destination C converts to the
 * narrow type by itself (an assignment, a store, a prototyped argument, a
 * return), and narrowings that must keep their cast (an arithmetic or a
 * comparison operand, a destination of another type).  Built with
 * gcc/clang -std=gnu11 -O0/-O2; CASTNARROW_NO_MAIN drops main for the round
 * trip in kuna-cli/tests/castnarrow_round_trip.rs. */
#include <ctype.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

__attribute__((noinline)) unsigned long trunc_local(unsigned long x, unsigned long *out) {
    unsigned int v = x;
    *out = x * 2;
    unsigned int w = 0;
    for (unsigned int i = 0; i < v % 13u; i++) w = (w << 1) ^ (i + v);
    return w + x;
}

__attribute__((noinline)) unsigned long cond_narrow(const char *s, unsigned long *out) {
    unsigned long n = strtoul(s, 0, 16);
    unsigned int v = 100;
    if (n > 1000) {
        *out = n;
        v = n;
    }
    unsigned int acc = 0;
    for (unsigned int i = 0; i < (v & 15); i++) acc = (acc << 1) ^ (v + i);
    return acc + n;
}

__attribute__((noinline)) unsigned long pair_low(const char *s, unsigned long *out) {
    unsigned long pair[2];
    pair[0] = 0;
    pair[1] = 0;
    sscanf(s, "%lx %lx", &pair[0], &pair[1]);
    unsigned int count = pair[0];
    unsigned int acc = 0;
    for (unsigned int i = 0; i != 4 && i < count; i++) {
        acc += count * i;
        puts(".");
    }
    *out = pair[1];
    return acc;
}

__attribute__((noinline)) unsigned long scanned_low(const char *s, unsigned long *out) {
    unsigned long n = 0;
    if (sscanf(s, "%lx", &n) != 1) return 0;
    unsigned int v = 5;
    if (strlen(s) > 4) {
        puts("long");
        v = n;
    }
    unsigned int acc = 0;
    for (unsigned int i = 0; i != 4 && i < v; i++) {
        acc += v * i;
        puts(".");
    }
    *out = n;
    return acc;
}

__attribute__((noinline)) int libc_args(unsigned long x, unsigned long *out) {
    *out = x >> 3;
    return toupper(x) + abs((int)x);
}

__attribute__((noinline)) unsigned long narrow_short(unsigned long n, unsigned short *out) {
    unsigned short h = 9;
    unsigned char c = 1;
    if (n & 0x10) {
        h = n;
        c = n >> 3;
    }
    for (int i = 0; i < 3; i++) {
        out[i] = h;
        h = h * 3 + c;
    }
    return n * 5;
}

__attribute__((noinline)) int sign_change(unsigned int u, unsigned int *out) {
    int v = -1;
    if (u & 4) v = u;
    *out = u >> 2;
    int s = 0;
    for (int i = 1; i < 4; i++) s += v / i;
    return s;
}

__attribute__((noinline)) void put_bytes(unsigned long x, char *buf) {
    buf[0] = x;
    buf[2] = x >> 16;
    buf[4] = x >> 32;
}

__attribute__((noinline)) int ret_narrow(long x, long *out) {
    *out = x / 3;
    return x;
}

__attribute__((noinline)) short ret_short(long x, long *out) {
    *out = x + 1;
    return x;
}

__attribute__((noinline)) unsigned long keep_store(unsigned long x, unsigned int *p) {
    *p = x;
    return x >> 1;
}

__attribute__((noinline)) unsigned long keep_arith(unsigned long x, unsigned int y, unsigned long *out) {
    unsigned int v = (unsigned int)x + y;
    *out = x;
    return v;
}

__attribute__((noinline)) int keep_cmp(unsigned long x, unsigned long *out) {
    *out = x + 9;
    return (int)x < 0;
}

#ifndef CASTNARROW_NO_MAIN
int main(int argc, char **argv) {
    const char *hex = argc > 1 ? argv[1] : "fedcba9876543261";
    unsigned long a = strtoul(hex, 0, 16), o = 0, r;
    unsigned short h[3];
    unsigned int u = 0;
    char b[5] = {0, 0, 0, 0, 0};
    r = trunc_local(a, &o);
    printf("%lu %lu\n", r, o);
    r = cond_narrow(hex, &o);
    printf("%lu %lu\n", r, o);
    r = pair_low("123456789abc 3", &o);
    printf("%lu %lu\n", r, o);
    r = scanned_low(hex, &o);
    printf("%lu %lu\n", r, o);
    int t = libc_args(0xffffffff00000061UL, &o);
    printf("%d %lu\n", t, o);
    r = narrow_short(a, h);
    printf("%lu %u %u %u\n", r, h[0], h[1], h[2]);
    t = sign_change(0xfffffff4u, &u);
    printf("%d %u\n", t, u);
    put_bytes(a, b);
    printf("%d %d %d\n", b[0], b[2], b[4]);
    long lo = 0;
    t = ret_narrow(-5000000123L, &lo);
    printf("%d %ld\n", t, lo);
    t = ret_short(0x12345fedcbL, &lo);
    printf("%d %ld\n", t, lo);
    r = keep_store(a, &u);
    printf("%lu %u\n", r, u);
    r = keep_arith(a, 0x80000000u, &o);
    printf("%lu %lu\n", r, o);
    t = keep_cmp(a, &o);
    printf("%d %lu\n", t, o);
    return 0;
}
#endif
