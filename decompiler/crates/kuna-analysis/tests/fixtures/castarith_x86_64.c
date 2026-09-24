/* castarith round-trip fixture: a pointer plus a whole number of elements.
 * The round trip compiles the prelude, kuna's printing of every function
 * between the markers, and the original main, and compares its output with
 * this binary's.  Built with -std=gnu11 at gcc -O0, clang -O0 and gcc -O2. */
#include <stdio.h>
#include <string.h>

/* prelude */
#if defined(__clang__)
#define KEEP __attribute__((noinline))
#else
#define KEEP __attribute__((noipa))
#endif
#define CALL(f, ...) ((void (*)())(f))(__VA_ARGS__)
KEEP void sink(long x) { printf("%ld\n", x); }
KEEP void take(unsigned int *q) { printf("take %u\n", *q); }
KEEP void *getv(void);
unsigned char gbuf[256];
struct hold { long a; long b; };
/* tested */
KEEP void rd_u8(void *p) { sink(*(unsigned char *)((char *)p + 0x11)); }
KEEP void rd_s8(void *p) { sink(*(signed char *)((char *)p + 0x13)); }
KEEP void rd_u16(void *p) { sink(*(unsigned short *)((char *)p + 0x1c)); }
KEEP void rd_s16(void *p) { sink(*(short *)((char *)p + 0x1e)); }
KEEP void rd_u32(void *p) { sink(*(unsigned int *)((char *)p + 0xac)); }
KEEP void rd_s32(void *p) { sink(*(int *)((char *)p + 0x30)); }
KEEP void rd_u64(void *p) { sink((long)*(unsigned long *)((char *)p + 0x20)); }
KEEP void rd_s64(void *p) { sink(*(long *)((char *)p + 0x48)); }
KEEP void rd_f64(void *p) { sink((long)*(double *)((char *)p + 0x58)); }
KEEP void rd_neg(void *p) { sink(*(int *)((char *)p - 8)); sink(*(short *)((char *)p - 0x1a)); }
KEEP void rd_odd(void *p) { sink(*(unsigned int *)((char *)p + 0x6a)); sink(*(long *)((char *)p + 0x44)); }
KEEP void wr_flt(void *p, float a, float b, long v)
{
    *(float *)((char *)p + 0x18) = a * b;
    *(double *)((char *)p + 0x20) = (double)a / b;
    *(float *)((char *)p + 0x1c) = (float)v;
    sink((long)*(float *)((char *)p + 0x18));
}
KEEP void wr_all(void *p, long v)
{
    *(unsigned char *)((char *)p + 0x81) = (unsigned char)v;
    *(unsigned short *)((char *)p + 0x82) = (unsigned short)(v >> 3);
    *(int *)((char *)p + 0x84) = (int)(v * 5);
    *(long *)((char *)p + 0x88) = v * 7;
    *(unsigned int *)((char *)p - 4) = 0xfeedu;
}
KEEP void pass_ptr(void *p) { sink(*(unsigned char *)((char *)p + 1)); take((unsigned int *)((char *)p + 0x10)); }
KEEP void cmp_ptr(void *p, unsigned int *q) { sink(*(unsigned char *)((char *)p + 1)); sink(*q); sink((unsigned int *)((char *)p + 0x10) == q); }
KEEP void cast_base(void) { sink(*(unsigned int *)((char *)getv() + 0x14)); sink(*(unsigned short *)((char *)getv() + 0x2e)); }
KEEP void rd_via(int *q) { sink(*q); sink(*(unsigned short *)((char *)q + 6)); sink(*(signed char *)((char *)q + 9)); }
KEEP void span(void *p, char *q) { sink(*(unsigned char *)((char *)p + 1)); char *m = (char *)p + 0x30; sink(*m); sink(m[1]); sink(q - m); }
KEEP void walk(void *p, void *end) { long n = 0; for (void *e = p; e != end; e = (char *)e + 0x28) n += *(int *)((char *)e + 4); sink(n); }
struct rec { long a; long b; unsigned int c; unsigned int d; unsigned short e[4]; };
KEEP void rd_rec(struct rec *r)
{
    r->a = 0x1234;
    r->c = 7;
    sink(r->b);
    sink(r->d);
    sink(*(unsigned short *)((char *)r + 0x1a));
    sink(*(unsigned int *)((char *)r + 0x14));
}
KEEP void wide_cmp(void *p, int a)
{
    sink(a <= *(unsigned char *)((char *)p + 0x11));
    sink(a > *(signed char *)((char *)p + 0x13));
}
KEEP void wide_idx(void *p)
{
    sink((long)(int)*(unsigned short *)((char *)p + 0x48) << 4);
    sink((long)(int)*(short *)((char *)p + 0x4a) * 3);
}
KEEP void via_int(struct hold *h) { sink(*(unsigned char *)(h->b + 10)); sink((long)*(unsigned long *)h->b); }
/* main */
KEEP void *getv(void) { return gbuf + 8; }
int main(void)
{
    static unsigned char buf[512];
    for (int i = 0; i < 512; i++)
        buf[i] = (unsigned char)(i * 37 + 11);
    double d = -2.625e6;
    memcpy(buf + 0x58, &d, sizeof d);
    for (int i = 0; i < 256; i++)
        gbuf[i] = (unsigned char)(i * 13 + 5);
    CALL(rd_u8, buf); CALL(rd_s8, buf); CALL(rd_u16, buf); CALL(rd_s16, buf);
    CALL(rd_u32, buf); CALL(rd_s32, buf); CALL(rd_u64, buf); CALL(rd_s64, buf);
    CALL(rd_f64, buf); CALL(rd_neg, buf + 0x40); CALL(rd_odd, buf);
    CALL(wr_all, buf + 0x10, -123456789L);
    ((void (*)(void *, float, float, long))wr_flt)(buf + 0x100, 1.5f, -2.25f, 77L);
    for (int i = 0x08; i < 0xa0; i += 4)
        sink(*(int *)(buf + i));
    for (int i = 0x118; i < 0x128; i += 4)
        sink(*(int *)(buf + i));
    CALL(pass_ptr, buf);
    CALL(cmp_ptr, buf, buf + 0x10);
    CALL(cmp_ptr, buf, buf + 0x14);
    CALL(cast_base);
    CALL(rd_via, buf + 0x20);
    CALL(span, buf, buf + 0x50);
    CALL(walk, buf, buf + 0xc8);
    CALL(rd_rec, buf);
    CALL(wide_cmp, buf, 100);
    CALL(wide_cmp, buf, -40);
    CALL(wide_idx, buf);
    static struct hold h;
    h.b = (long)(buf + 0x30);
    CALL(via_int, &h);
    for (int i = 0; i < 0x20; i += 4)
        sink(*(int *)(buf + i));
    return 0;
}
