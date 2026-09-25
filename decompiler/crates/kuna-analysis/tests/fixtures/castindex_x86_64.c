/* castindex round-trip fixture: a pointer plus a variable index, and the
 * difference of two byte pointers.  The round trip compiles the prelude, kuna's
 * printing of every function between the markers, and the original main, and
 * compares its output with this binary's.  Built with -std=gnu11 at gcc -O0,
 * clang -O0 and gcc -O2. */
#include <stdio.h>
#include <string.h>

/* prelude */
#include <stdlib.h>
#include <sys/mman.h>
#if defined(__clang__)
#define KEEP __attribute__((noinline))
#else
#define KEEP __attribute__((noipa))
#endif
KEEP void sink(long x) { printf("%ld\n", x); }
KEEP void sinku(unsigned long x) { printf("%lu\n", x); }
KEEP void fill(void *p, int c, unsigned long n) { memset(p, c, n); }
#define HEAD(p) sink(*(unsigned int *)((char *)(p) + 4) + *(long *)((char *)(p) + 8))
#define CALL(f, ...) ((void (*)())(f))(__VA_ARGS__)
char *b64_table;
/* tested */
KEEP void ld_s8(void *p, int i) { HEAD(p); sink(*(signed char *)((char *)p + i)); }
KEEP void ld_u8(void *p, unsigned int i) { HEAD(p); sink(*(unsigned char *)((char *)p + i)); }
KEEP void ld_s16(void *p, int i) { HEAD(p); sink(*(short *)((char *)p + (long)i * 2)); }
KEEP void ld_u16(void *p, unsigned int i) { HEAD(p); sink(*(unsigned short *)((char *)p + (unsigned long)i * 2)); }
KEEP void ld_s32(void *p, int i) { HEAD(p); sink(*(int *)((char *)p + (long)i * 4)); }
KEEP void ld_u32(void *p, unsigned int i) { HEAD(p); sink(*(unsigned int *)((char *)p + (unsigned long)i * 4)); }
KEEP void ld_s64(void *p, long i) { HEAD(p); sink(*(long *)((char *)p + i * 8)); }
KEEP void ld_u64(void *p, unsigned long i) { HEAD(p); sinku(*(unsigned long *)((char *)p + i * 8)); }
KEEP void ld_f64(void *p, int i) { HEAD(p); sink((long)*(double *)((char *)p + (long)i * 8)); }
KEEP void ld_short(void *p, short i) { HEAD(p); sink(*(int *)((char *)p + (long)i * 4)); }
KEEP void ld_schar(void *p, signed char i) { HEAD(p); sink(*(long *)((char *)p + (long)i * 8)); }
KEEP void ld_uchar(void *p, unsigned char i) { HEAD(p); sink(*(int *)((char *)p + (unsigned long)i * 4)); }
KEEP void st_all(void *p, int i, long v)
{
    HEAD(p);
    *(long *)((char *)p + (long)i * 8 + 0x100) = v;
    *(int *)((char *)p + (long)i * 4 + 0x200) = (int)v;
    *(short *)((char *)p + (long)i * 2 + 0x300) = (short)v;
    *(char *)((char *)p + i + 0x400) = (char)v;
    sink(*(long *)((char *)p + (long)i * 8 + 0x100));
    sink(*(int *)((char *)p + (long)i * 4 + 0x200));
    sink(*(short *)((char *)p + (long)i * 2 + 0x300));
    sink(*(char *)((char *)p + i + 0x400));
}
KEEP void walk(void *p, int n)
{
    long s = *(unsigned int *)((char *)p + 4) + *(long *)((char *)p + 8);
    for (int i = 0; i < n; i++)
        s += *(long *)((char *)p + (long)i * 8) * (i + 1);
    sink(s);
}
KEEP void rec16(void *p, int i) { HEAD(p); sink(*(long *)((char *)p + (long)i * 16 + 8)); }
KEEP void odd_scale(void *p, int i) { HEAD(p); sink(*(int *)((char *)p + (long)i * 12)); }
KEEP void byte_off(void *p, long off) { HEAD(p); sink(*(long *)((char *)p + off)); }
KEEP void diff_c(char *a, int c) { char *e = strchr(a, c); sink(e - a); }
KEEP void diff_back(char *a, int c) { char *e = strrchr(a, c); sink(a - e); sink((a - e) / 3); sink((a - e) >> 1); }
KEEP void diff_len(char *a, char *b, unsigned long n)
{
    char *e = strchr(b, 'z');
    sinku((unsigned long)(e - a) < n);
    sink(e - a < (long)n);
    sink(strnlen(a, e - a));
}
KEEP void b64_decode(long data, unsigned long len)
{
    const char *in = (const char *)data;
    static const char alphabet[] = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    b64_table = malloc(256);
    fill(b64_table, 0xa5, 256);
    for (unsigned long i = 0; i < 64; i++)
        b64_table[(unsigned char)alphabet[i]] = i;
    unsigned char *out = malloc(len / 4 * 3);
    unsigned long j = 0, h = 0;
    for (unsigned long i = 0; i + 3 < len; i += 4) {
        unsigned int a = in[i] == '=' ? 0 : b64_table[(unsigned char)in[i]];
        unsigned int b = in[i + 1] == '=' ? 0 : b64_table[(unsigned char)in[i + 1]];
        unsigned int c = in[i + 2] == '=' ? 0 : b64_table[(unsigned char)in[i + 2]];
        unsigned int d = in[i + 3] == '=' ? 0 : b64_table[(unsigned char)in[i + 3]];
        unsigned int t = (a << 18) + (b << 12) + (c << 6) + d;
        h = h * 131 + t;
        out[j++] = t >> 16;
        if (in[i + 2] != '=') out[j++] = t >> 8;
        if (in[i + 3] != '=') out[j++] = t;
    }
    for (unsigned long k = 0; k < j; k++)
        h = h * 131 + out[k];
    sinku(j);
    sinku(h);
    free(out);
    free(b64_table);
}
KEEP void diff_wide(long *a, long *b) { sink(*a + *b); sink(b - a); }
/* main */
int main(void)
{
    static unsigned char buf[4096];
    for (int k = 0; k < 4096; k++)
        buf[k] = (unsigned char)(k * 37 + 11);
    for (int k = 0; k < 64; k++)
        ((double *)buf)[k] = k * 1.5 - 9;
    void *mid = buf + 2048;
    int idx[] = { 0, 1, 3, -1, -3, 7, -7, 100, -100 };
    for (unsigned t = 0; t < sizeof idx / sizeof idx[0]; t++) {
        int i = idx[t];
        ld_s8(mid, i); ld_s16(mid, i); ld_s32(mid, i); ld_s64(mid, i);
        ld_short(mid, (short)i); ld_schar(mid, (signed char)i);
        ld_f64(buf + 128, (i < 0 ? -i : i) % 32);
        rec16(mid, i); odd_scale(mid, i); byte_off(mid, i * 3);
        if (i >= 0) { ld_u8(mid, i); ld_u16(mid, i); ld_u32(mid, i); ld_u64(mid, i); ld_uchar(mid, (unsigned char)i); }
    }
    ld_uchar(buf, 200);
    st_all(mid, 5, -12345678901L); st_all(mid, -9, 0x1234567890L);
    walk(buf, 40);
    char s[] = "alpha:beta:gamma/zeta";
    diff_c(s, ':'); diff_c(s + 3, 'g');
    diff_back(s, 'a'); diff_back(s + 6, 'e');
    diff_len(s, s + 2, 9); diff_len(s + 19, s, 5);
    diff_wide((long *)buf, (long *)buf + 11);
    /* A table indexed by an input byte: '\xff' and '*' read the 0xa5 filler
       (a sign-extended index would read before the table). */
    const char *enc[] = { "aGVsbG8sIHdvcmxkIQ==", "/+/+Zm9vYg==", "\xff*Zm9v\x80" "A==" };
    for (unsigned t = 0; t < 3; t++)
        CALL(b64_decode, enc[t], strlen(enc[t]));
    /* An unsigned index with its top bit set: zero-extended it reads 2^31 and
       more elements forward, sign-extended it would read backward. */
    size_t big = (size_t)24 << 30;
    unsigned char *m = mmap(NULL, big, PROT_READ | PROT_WRITE, MAP_PRIVATE | MAP_ANONYMOUS | MAP_NORESERVE, -1, 0);
    if (m == MAP_FAILED) {
        puts("no map");
        return 0;
    }
    unsigned char *c = m + ((size_t)12 << 30);
    unsigned int u = 0x80000001u;
    c[u] = 0x5a; c[-(long)0x7fffffff] = 0xa5;
    ((unsigned short *)c)[u] = 0x1234; ((unsigned short *)c)[-(long)0x7fffffff] = 0x4321;
    ((unsigned int *)c)[u] = 0xdeadbeef; ((unsigned int *)c)[-(long)0x7fffffff] = 0xfeedface;
    ld_u8(c, u); ld_u16(c, u); ld_u32(c, u);
    ld_s8(c, (int)u); ld_s16(c, (int)u); ld_s32(c, (int)u);
    munmap(m, big);
    return 0;
}
