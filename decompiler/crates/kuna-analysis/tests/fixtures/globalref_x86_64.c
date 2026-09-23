/* `globalref` fixture: functions that hand the address of a file-scope object
 * to a callee, compare against it, walk to it, or return it.
 *
 * Build (non-PIE at 0x30000000 so a test harness can map the data at the same
 * addresses; -x drops the local symbols, so the data has no names):
 *   gcc -O0 -fno-builtin -fno-pie -no-pie -Wl,-Ttext-segment=0x30000000 -Wl,-x \
 *       -o globalref_x86_64 globalref_x86_64.c
 *
 * With -DGLOBALREF_CALLEES_ONLY only the callees are compiled: a round-trip
 * harness links them against the printed callers.
 */
#include <stdio.h>
#include <string.h>

struct pair {
    long len;
    const char *text;
};

__attribute__((noinline)) long put(struct pair *p)
{
    return p->len * 100 + (p->text ? p->text[0] : 0);
}

__attribute__((noinline)) long setbits(unsigned int *f, long *t)
{
    *f |= 4;
    *t += 10;
    return (long)*f + *t;
}

__attribute__((noinline)) long sum(const int *b, const int *e)
{
    long s = 0;
    while (b != e)
        s += *b++;
    return s;
}

#ifndef GLOBALREF_CALLEES_ONLY

static struct pair colors[3] = {{2, "\033["}, {1, "m"}, {0, 0}};
static unsigned int flags = 1;
static long total = 5;
static const int weights[5] = {3, 5, 7, 11, 0x7f80};
static char scratch[16];
static const char glyph[] = "\xa1\x07" "e";
static char namebuf[16] = "\x81\x82\x83\x84\x85\x86\x87\x88";

long w_struct(void)
{
    return put(&colors[1]) + 1;
}

long w_scalar(void)
{
    return setbits(&flags, &total) + 1;
}

long w_range(void)
{
    return sum(weights, weights + 4) + 1;
}

int w_buffer(void)
{
    memset(scratch, 'x', 8);
    return scratch[3] + 1;
}

int w_compare(struct pair *p)
{
    if (p->len < 0)
        return -1;
    return p == &colors[0];
}

long w_glyph(void)
{
    return (long)strlen(glyph) + 1;
}

long w_direct(void)
{
    return colors[0].len + put(&colors[0]);
}

long w_numeric(unsigned long n)
{
    setbits(&flags, &total);
    return n < (unsigned long)&flags;
}

long w_width(void)
{
    return (long)strlen(namebuf) + *(long *)namebuf;
}

long w_count(unsigned long n)
{
    memset(scratch, 0, 4);
    return (long)(n / (unsigned long)scratch);
}

int main(void)
{
    long a = w_struct();
    long b = w_scalar();
    long c = w_range();
    int d = w_buffer();
    int e = w_compare(&colors[0]);
    int f = w_compare(&colors[1]);
    long g = w_glyph();
    long h = w_direct();
    long i = w_numeric(1);
    long j = w_width();
    long k = w_count(0x7fffffff);
    printf("%ld %ld %ld %d %d %d %ld %ld %ld %ld %ld\n", a, b, c, d, e, f, g, h, i, j, k);
    return 0;
}

#endif
