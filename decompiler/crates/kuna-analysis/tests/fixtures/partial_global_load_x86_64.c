#include <stdio.h>
#include <stdint.h>

unsigned long gp, gb, seen;
volatile unsigned long gv;

__attribute__((noinline)) void touch(void) {
    seen = gp;
    gp ^= 0x1234000000000000UL;
}

#ifndef PARTIAL_GLOBAL_HARNESS
__attribute__((noinline)) unsigned long halves(unsigned long *p, unsigned a, unsigned b) {
    ((unsigned *)&gp)[0] = a;
    ((unsigned *)&gp)[1] = b;
    unsigned long x = *p;
    gp = 0;
    return x;
}

__attribute__((noinline)) unsigned high(unsigned *p, unsigned b) {
    ((unsigned *)&gp)[1] = b;
    unsigned x = *p;
    gp = 0;
    return x;
}

__attribute__((noinline)) unsigned low(unsigned *p, unsigned a) {
    ((unsigned *)&gp)[0] = a;
    unsigned x = *p;
    gp = 0;
    return x;
}

__attribute__((noinline)) unsigned long mixed(unsigned long *p, unsigned short a, unsigned char b) {
    *(unsigned short *)((char *)&gb + 2) = a;
    *((unsigned char *)&gb + 5) = b;
    unsigned long x = *p;
    gb = 0;
    return x;
}

__attribute__((noinline)) unsigned long across_call(unsigned long *p, unsigned a, unsigned b) {
    ((unsigned *)&gp)[0] = a;
    touch();
    ((unsigned *)&gp)[1] = b;
    unsigned long x = *p;
    gp = 0;
    return x;
}

__attribute__((noinline)) unsigned long conditional(unsigned long *p, unsigned a, unsigned b, int k) {
    ((unsigned *)&gp)[0] = a;
    if (k) ((unsigned *)&gp)[1] = b;
    unsigned long x = *p;
    gp = 0;
    return x;
}

__attribute__((noinline)) unsigned long two_reads(unsigned long *p, unsigned a, unsigned b) {
    ((unsigned *)&gp)[0] = a;
    unsigned x = *(unsigned *)p;
    ((unsigned *)&gp)[1] = b;
    unsigned long y = *p;
    gp = 0;
    return y + x;
}

__attribute__((noinline)) unsigned long volatile_halves(unsigned long *p, unsigned a, unsigned b) {
    ((volatile unsigned *)&gv)[0] = a;
    ((volatile unsigned *)&gv)[1] = b;
    unsigned long x = *p;
    gv = 0;
    return x;
}

__attribute__((noinline)) unsigned long affine(unsigned long *p, unsigned a, unsigned b, unsigned i) {
    ((unsigned *)&gp)[0] = a;
    ((unsigned *)&gp)[1] = b;
    unsigned long x = *(unsigned long *)((uintptr_t)p + ((uintptr_t)i << 3));
    gp = 0;
    return x;
}

__attribute__((noinline)) unsigned long agreeing_phi(unsigned long *p, unsigned a, unsigned b, int k) {
    unsigned long *q;
    if (k) q = p;
    else q = p;
    ((unsigned *)&gp)[0] = a;
    ((unsigned *)&gp)[1] = b;
    unsigned long x = *q;
    gp = 0;
    return x;
}

__attribute__((noinline)) unsigned long differing_phi(unsigned long *p, unsigned long *q, unsigned a, unsigned b, int k) {
    unsigned long *r;
    if (k) r = p;
    else r = q;
    ((unsigned *)&gp)[0] = a;
    ((unsigned *)&gp)[1] = b;
    unsigned long x = *r;
    gp = 0;
    return x;
}

__attribute__((noinline)) unsigned long real_input_frame(unsigned long *p, unsigned a, unsigned b, unsigned c) {
    unsigned long s = 11, *q = &s;
    *q = c;
    ((unsigned *)&gp)[0] = a;
    ((unsigned *)&gp)[1] = b;
    unsigned long x = *p, z = *q;
    ((unsigned *)&gp)[0] = b;
    unsigned long y = *p;
    gp = 0;
    return x ^ y ^ z;
}

__attribute__((noinline)) unsigned long stack_input_frame(unsigned long d0, unsigned long d1, unsigned long d2, unsigned long d3, unsigned long d4, unsigned long d5, unsigned long *p, unsigned a, unsigned b, unsigned c) {
    unsigned long s = 11, *q = &s;
    *q = c;
    ((unsigned *)&gp)[0] = a;
    ((unsigned *)&gp)[1] = b;
    unsigned long x = *p, z = *q;
    ((unsigned *)&gp)[0] = b;
    unsigned long y = *p;
    gp = 0;
    return x ^ y ^ z;
}
#else
unsigned long halves(unsigned long *, unsigned, unsigned);
unsigned high(unsigned *, unsigned);
unsigned low(unsigned *, unsigned);
unsigned long mixed(unsigned long *, unsigned short, unsigned char);
unsigned long across_call(unsigned long *, unsigned, unsigned);
unsigned long conditional(unsigned long *, unsigned, unsigned, int);
unsigned long two_reads(unsigned long *, unsigned, unsigned);
unsigned long volatile_halves(unsigned long *, unsigned, unsigned);
unsigned long affine(unsigned long *, unsigned, unsigned, unsigned);
unsigned long agreeing_phi(unsigned long *, unsigned, unsigned, int);
unsigned long differing_phi(unsigned long *, unsigned long *, unsigned, unsigned, int);
unsigned long real_input_frame(unsigned long *, unsigned, unsigned, unsigned);
unsigned long stack_input_frame(unsigned long, unsigned long, unsigned long, unsigned long, unsigned long, unsigned long, unsigned long *, unsigned, unsigned, unsigned);
#endif

int main(void) {
    const unsigned values[][2] = {{6, 9}, {0xffffffffU, 0x11223344U}, {0, 0}, {0x80000000U, 0xffffffffU}};
    for (unsigned i = 0; i < 4; ++i) {
        unsigned a = values[i][0], b = values[i][1];
        for (int alias = 0; alias < 2; ++alias) {
            unsigned long other = 0x1020304050607080UL;
            unsigned long x;
#define RESET() gp = 0xaabbccddeeff0011UL; gb = 0x8877665544332211UL; gv = 0xaabbccddeeff0011UL; seen = 0
#define SHOW(name) printf(name " %d %u %lx %lx %lx %lx %lx\n", alias, i, x, gp, gb, (unsigned long)gv, seen)
            RESET(); x = halves(alias ? &other : &gp, a, b); SHOW("halves");
            for (unsigned offset = 0; offset < 2; ++offset) {
                RESET(); x = high(alias ? (unsigned *)&other : (unsigned *)&gp + offset, b); SHOW("high");
                RESET(); x = low(alias ? (unsigned *)&other : (unsigned *)&gp + offset, a); SHOW("low");
            }
            RESET(); x = mixed(alias ? &other : &gb, (unsigned short)a, (unsigned char)b); SHOW("mixed");
            RESET(); x = across_call(alias ? &other : &gp, a, b); SHOW("across_call");
            for (int k = 0; k < 2; ++k) {
                RESET(); x = conditional(alias ? &other : &gp, a, b, k); SHOW("conditional");
            }
            RESET(); x = two_reads(alias ? &other : &gp, a, b); SHOW("two_reads");
            RESET(); x = volatile_halves(alias ? &other : (unsigned long *)&gv, a, b); SHOW("volatile_halves");
            RESET(); x = affine(alias ? &other : &gp, a, b, 0); SHOW("affine");
            RESET(); x = real_input_frame(alias ? &other : &gp, a, b, i); SHOW("real_input_frame");
            RESET(); x = stack_input_frame(0, 1, 2, 3, 4, 5, alias ? &other : &gp, a, b, i); SHOW("stack_input_frame");
            for (int k = 0; k < 2; ++k) {
                RESET(); x = agreeing_phi(alias ? &other : &gp, a, b, k); SHOW("agreeing_phi");
                RESET(); x = differing_phi(alias ? &other : &gp, &other, a, b, k); SHOW("differing_phi");
            }
        }
    }
    return 0;
}
