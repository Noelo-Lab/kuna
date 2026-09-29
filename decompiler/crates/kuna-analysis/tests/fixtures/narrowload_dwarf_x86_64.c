/* narrowload DWARF round-trip fixture: a masked byte compare of a declared
 * record's 4-byte field, through a pointer the declaration does not type
 * itself: a call's result, a loop's phi, and a pointer read out of another
 * record.  Every object the byte is read from ends at that byte, at the end of
 * a readable page, so a printed read of the whole field faults where the
 * binary does not.  The round trip compiles kuna's printing of every function
 * between the markers against the export's header, links it with the prelude
 * and main, and compares the output with this binary's.  Built with
 * -std=gnu11 -g at gcc -O0, clang -O0, gcc -O2 and clang -O2. */
/* prelude */
#include <stdio.h>
#include <string.h>
#include <sys/mman.h>
#if defined(__clang__)
#define KEEP __attribute__((noinline, used))
#else
#define KEEP __attribute__((noipa, used))
#endif
struct rec { const char *name; unsigned flags; unsigned short tag; unsigned short pad; long id; struct rec *next; };
struct rec *src(int k);
long d_call(int k);
long d_loop(const struct rec *r);
long d_next(const struct rec *r);
/* tested */
KEEP long d_call(int k)
{
    struct rec *n = src(k);
    return (((const unsigned char *)&n->flags)[1] & 0x81) == 0x80;
}
KEEP long d_loop(const struct rec *r)
{
    long s = 0;
    for (; (((const unsigned char *)&r->flags)[0] & 0x81) != 0x80; r = r->next) s++;
    return s;
}
KEEP long d_next(const struct rec *r)
{
    return (((const unsigned char *)&r->next->flags)[1] & 0x81) == 0x80;
}
/* main */
static unsigned char *page_end(unsigned n, const unsigned char *bytes)
{
    unsigned char *pg = mmap(0, 8192, PROT_READ | PROT_WRITE, MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
    mprotect(pg + 4096, 4096, PROT_NONE);
    memcpy(pg + 4096 - n, bytes, n);
    return pg + 4096 - n;
}
#if defined(__clang__)
__attribute__((noinline))
#else
__attribute__((noipa))
#endif
struct rec *src(int k)
{
    unsigned char *p = page_end(10, (const unsigned char *)"\0\0\0\0\0\0\0\0\x80");
    p[9] = 0x80 + k;
    return (struct rec *)p;
}
int main(void)
{
    static unsigned char b[32];
    b[8] = 0x80;
    unsigned char *last = page_end(9, b);
    b[8] = 0x01;
    memcpy(b + 24, &last, 8);
    unsigned char *mid = page_end(32, b);
    b[8] = 0x81;
    memcpy(b + 24, &mid, 8);
    unsigned char *first = page_end(32, b);
    static unsigned char h[32];
    h[8] = 0x80, h[9] = 0x80;
    unsigned char *tail = page_end(10, h);
    memcpy(h + 24, &tail, 8);
    unsigned char *head = page_end(32, h);
    printf("%ld %ld %ld %ld\n", d_call(0), d_call(1), d_loop((const struct rec *)first),
           d_next((const struct rec *)head));
    return 0;
}
