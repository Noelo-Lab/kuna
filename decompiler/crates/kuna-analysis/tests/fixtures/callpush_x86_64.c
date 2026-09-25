/* callpush fixture: calls made after an alloca, through a stack pointer the
 * frame cannot track.  Every call pushes its return address through that
 * pointer; `callpush` deletes the push and keeps everything else, including
 * the stack-passed arguments of `spread` and the push `pc_here` reads back.
 * Built with -std=gnu11 at gcc -O0, gcc -O2, clang -O0 and clang -O2. */
#include <alloca.h>
#include <stdio.h>
#include <string.h>

/* prelude */
#if defined(__clang__)
#define KEEP __attribute__((noinline))
#else
#define KEEP __attribute__((noipa))
#endif
KEEP long weigh(const char *s, long k)
{
    long t = k;
    while (*s)
        t = t * 31 + *s++;
    return t;
}
KEEP long spread(long a, long b, long c, long d, long e, long f, long g, long h)
{
    return a + 2 * b + 3 * c + 4 * d + 5 * e + 6 * f + 7 * g + 8 * h;
}
/* tested */
KEEP long joined(const char *a, const char *b)
{
    size_t la = strlen(a), lb = strlen(b);
    char *buf = alloca(la + lb + 2);
    memcpy(buf, a, la);
    buf[la] = '/';
    memcpy(buf + la + 1, b, lb + 1);
    return weigh(buf, (long)(la + lb));
}
KEEP long stacked(int n)
{
    long *v = alloca(n * sizeof(long));
    for (int i = 0; i < n; i++)
        v[i] = weigh("ab", i);
    return spread(v[0], v[1], n, 4, 5, 6, v[n - 1], v[n - 2]);
}
KEEP long twice(int n)
{
    char *p = alloca(n + 1);
    memset(p, 'x', n);
    p[n] = 0;
    long w = weigh(p, n);
    char *q = alloca(2 * n + 1);
    memset(q, 'y', 2 * n);
    q[2 * n] = 0;
    return w ^ weigh(q, w);
}
KEEP long pc_here(int n)
{
    volatile char *p = alloca(n + 1);
    long pc;
    p[0] = (char)n;
    __asm__ volatile("call 1f\n1: pop %0" : "=r"(pc));
    return (pc & 0xf) + p[0];
}
/* main */
int main(int argc, char **argv)
{
    printf("%ld\n", joined("usr", argc > 5 ? argv[1] : "bin"));
    printf("%ld\n", stacked(argc + 5));
    printf("%ld\n", twice(argc + 6));
    printf("%d\n", pc_here(argc) - (long)(argc & 0xff) >= 0);
    return 0;
}
