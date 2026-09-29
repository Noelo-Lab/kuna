/* narrowload round-trip fixture: a read or a write narrower than the field or
 * element its pointer is typed to.  Every object main passes ends at the last
 * byte the function touches, at the end of a readable page, so a printed read
 * or write of the whole field or element faults where the binary does not.
 * The round trip compiles kuna's printing of every function between the
 * markers against the export's header, links it with the prelude and main, and
 * compares the output with this binary's.  Built with -std=gnu11 at gcc -O0,
 * clang -O0, gcc -O2 and clang -O2. */
/* prelude */
#include <stdio.h>
#include <string.h>
#include <sys/mman.h>
#if defined(__clang__)
#define KEEP __attribute__((noinline, used))
#else
#define KEEP __attribute__((noipa, used))
#endif
struct rec { const char *name; unsigned flags; unsigned short tag; unsigned short pad; long id; };
long r_low(const struct rec *r, int all);
long r_slow(const struct rec *r, int all);
long r_byte(const struct rec *r, int all);
long r_bit(const struct rec *r, int all);
long r_eq(const struct rec *r, int all);
long r_wid(const struct rec *r, int all);
long r_hib(const struct rec *r, int all);
void r_set(struct rec *r, int all, unsigned char b);
void r_set2(struct rec *r, int all, unsigned short b);
void r_setid(struct rec *r, int all, unsigned v);
long l_mix(const long *p, int n);
long l_bit(const long *p, int n);
void l_put(long *p, int n, int v);
unsigned n_sum4(const unsigned *p, int n);
unsigned long n_hdr(const unsigned *p);
long n_flag(const unsigned *p);
void n_put(unsigned *p, unsigned short v);
/* tested */
KEEP long r_low(const struct rec *r, int all)
{
    long s = (long)strlen(r->name);
    if (all) s += r->flags + r->id;
    return s + *(const unsigned short *)&r->flags;
}
KEEP long r_slow(const struct rec *r, int all)
{
    long s = (long)strlen(r->name);
    if (all) s += r->flags + r->id;
    return s + *(const short *)&r->flags;
}
KEEP long r_byte(const struct rec *r, int all)
{
    long s = (long)strlen(r->name);
    if (all) s += r->flags + r->id;
    return s + ((const unsigned char *)&r->flags)[0];
}
KEEP long r_bit(const struct rec *r, int all)
{
    long s = (long)strlen(r->name);
    if (all) s += r->flags + r->id;
    return s + ((((const unsigned char *)&r->flags)[1] & 0x20) != 0);
}
KEEP long r_eq(const struct rec *r, int all)
{
    long s = (long)strlen(r->name);
    if (all) s += r->flags + r->id;
    return s + ((((const unsigned char *)&r->flags)[1] & 0x30) == 0x10);
}
KEEP long r_wid(const struct rec *r, int all)
{
    long s = (long)strlen(r->name);
    if (all) s += r->flags + r->id;
    return s + *(const int *)&r->id;
}
KEEP long r_hib(const struct rec *r, int all)
{
    long s = (long)strlen(r->name);
    if (all) s += r->flags + r->id;
    return s + ((((const unsigned char *)&r->id)[4] & 0x80) != 0);
}
KEEP void r_set(struct rec *r, int all, unsigned char b)
{
    if (all) r->flags = (unsigned)strlen(r->name), r->id = 3;
    ((unsigned char *)&r->flags)[1] = b;
}
KEEP void r_set2(struct rec *r, int all, unsigned short b)
{
    if (all) r->flags = (unsigned)strlen(r->name), r->id = 3;
    *(unsigned short *)&r->flags = b;
}
KEEP void r_setid(struct rec *r, int all, unsigned v)
{
    if (all) r->flags = (unsigned)strlen(r->name), r->id = 3;
    *(unsigned *)&r->id = v;
}
KEEP long l_mix(const long *p, int n)
{
    long s = 0;
    for (int i = 0; i < n; i++) s += p[i];
    return s + *(const int *)&p[n];
}
KEEP long l_bit(const long *p, int n)
{
    long s = 0;
    for (int i = 0; i < n; i++) s += p[i];
    return s + ((((const unsigned char *)&p[n])[1] & 0x40) != 0);
}
KEEP void l_put(long *p, int n, int v)
{
    p[0] = p[n - 1] + 1;
    *(int *)&p[n] = v;
}
KEEP unsigned n_sum4(const unsigned *p, int n)
{
    unsigned s = 0;
    for (int i = 0; i < n; i++) s += p[i];
    return s;
}
KEEP unsigned long n_hdr(const unsigned *p) { return n_sum4(p, 1) + *(const unsigned short *)((const char *)p + 4); }
KEEP long n_flag(const unsigned *p) { return n_sum4(p, 1) + ((*((const unsigned char *)p + 5) & 0x20) != 0); }
KEEP void n_put(unsigned *p, unsigned short v)
{
    p[0] = n_sum4(p, 1) + 1;
    *(unsigned short *)((char *)p + 4) = v;
}
/* main */
static unsigned char *page_end(unsigned n, const unsigned char *bytes)
{
    unsigned char *pg = mmap(0, 8192, PROT_READ | PROT_WRITE, MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
    mprotect(pg + 4096, 4096, PROT_NONE);
    memcpy(pg + 4096 - n, bytes, n);
    return pg + 4096 - n;
}
int main(void)
{
    static const char name[] = "kuna";
    unsigned char rb[24];
    const char *np = name;
    memcpy(rb, &np, 8);
    const unsigned char tail[16] = {0x34, 0xa2, 0x71, 0x08, 7, 0, 0, 0, 0x81, 0x92, 0xa3, 0xb4, 0xc5, 0, 0, 0};
    memcpy(rb + 8, tail, 16);
    printf("%ld %ld %ld %ld %ld\n", r_low((const struct rec *)page_end(10, rb), 0),
           r_slow((const struct rec *)page_end(10, rb), 0), r_byte((const struct rec *)page_end(9, rb), 0),
           r_bit((const struct rec *)page_end(10, rb), 0), r_eq((const struct rec *)page_end(10, rb), 0));
    printf("%ld %ld\n", r_wid((const struct rec *)page_end(20, rb), 0), r_hib((const struct rec *)page_end(21, rb), 0));
    unsigned char *w1 = page_end(10, rb), *w2 = page_end(10, rb), *w3 = page_end(20, rb);
    r_set((struct rec *)w1, 0, 0x5a);
    r_set2((struct rec *)w2, 0, 0x6b7c);
    r_setid((struct rec *)w3, 0, 0x8d9eafb0u);
    printf("%02x%02x %02x%02x %02x%02x%02x%02x\n", w1[8], w1[9], w2[8], w2[9], w3[16], w3[17], w3[18], w3[19]);
    const long lv[3] = {5, -9, 0x4000c3d2e1f0L};
    unsigned char *lm = page_end(20, (const unsigned char *)lv), *lb = page_end(18, (const unsigned char *)lv);
    unsigned char *lp = page_end(20, (const unsigned char *)lv);
    l_put((long *)lp, 2, -77);
    printf("%ld %ld %ld %d\n", l_mix((const long *)lm, 2), l_bit((const long *)lb, 2), *(const long *)lp, *(const int *)(lp + 16));
    const unsigned uv[2] = {3, 0x92342034u};
    unsigned char *uh = page_end(6, (const unsigned char *)uv), *uf = page_end(6, (const unsigned char *)uv);
    unsigned char *up = page_end(6, (const unsigned char *)uv);
    n_put((unsigned *)up, 0x2155);
    printf("%lu %ld %u %02x%02x\n", n_hdr((const unsigned *)uh), n_flag((const unsigned *)uf), *(const unsigned *)up, up[4], up[5]);
    return 0;
}
