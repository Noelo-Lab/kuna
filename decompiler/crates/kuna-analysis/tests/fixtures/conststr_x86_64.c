/* `conststr` fixture: constant addresses of strings and objects that kuna
 * printed as `(T *)0x<addr>` or `&dat_<addr>`. A string a C library function
 * reads only as a string may print as its literal; one a user function or a
 * return hands on may not: `count` compares an end pointer at a string's NUL,
 * `hsum` reads a table with a zero byte by length.
 *
 * Build (non-PIE at 0x30000000 so a round-trip harness can map the data at the
 * same addresses; -x drops the local symbols, so the data has no names):
 *   cc -O<n> -fno-builtin -fno-inline -fno-pie -no-pie -Wl,-Ttext-segment=0x30000000 -Wl,-x \
 *      -o conststr_<cc>_O<n>_x86_64 conststr_x86_64.c
 *
 * With -DCONSTSTR_CALLEES_ONLY only the callees are compiled: a round-trip
 * harness links them against the printed callers.
 */
#include <stdio.h>
#include <string.h>

__attribute__((noinline)) long take(const char *s)
{
    long h = 7;
    while (*s)
        h = h * 31 + (unsigned char)*s++;
    return h;
}

__attribute__((noinline)) int count(const char *p, const char *end)
{
    int n = 0;
    while (p != end) {
        n += *p > '5';
        p++;
    }
    return n;
}

__attribute__((noinline)) int hsum(const char *p, int n)
{
    int s = 0;
    for (int i = 0; i < n; i++)
        s = s * 31 + p[i];
    return s;
}

__attribute__((noinline)) void set_slot(long *slot, long v)
{
    *slot = v;
}

__attribute__((noinline)) void set_name(char **slot, char *v)
{
    *slot = v;
}

#ifndef CONSTSTR_CALLEES_ONLY

static long counter;
static char *name;
static long wide;
static char buf[8] = "hi";
static const unsigned char blob[16] = {0, 0, 0, 0, 0, 0, 0, 0, 0x77, 0xdf, 0x77, 0xff, 0xfd, 0xff, 0x7f, 0};
static const char digits[16] __attribute__((aligned(16))) = "0123456789abcde";
static const unsigned tbl[4] __attribute__((aligned(16))) = {0x81223344, 0x95667788, 0x8badf00d, 0xdeadbeef};
static const char mixed[8] __attribute__((aligned(16))) = {(char)0x81, (char)0x92, 0, 0x33, (char)0xc4, 0x55, 0x66, 0x77};
static const int pad[4] __attribute__((aligned(16))) = {0x11223344, 0x55667788, 0x0badf00d, 0x7eadbeef};
static const char ov[] = "\xa1\xc1\x81\xe0\x81\x81\xed\xa0\x80z";

__attribute__((noinline)) long w_empty(const char *s)
{
    return take("") + strcmp(s, "") + take("tab\there\n");
}

__attribute__((noinline)) const char *w_quote(const char *msgid)
{
    if (!msgid[0])
        return msgid;
    if (msgid[0] == '`')
        return "\xa1\ae";
    return "\xa1\xaf";
}

__attribute__((noinline)) int w_count(const char *p)
{
    return count(p, digits + 15) * 3;
}

__attribute__((noinline)) int w_end(const char *s)
{
    return strcmp(s, digits + 15) + 1;
}

__attribute__((noinline)) int w_hsum(void)
{
    return hsum(mixed, 8);
}

__attribute__((noinline)) long w_ov(void)
{
    return take(ov) + (long)strlen(ov);
}

__attribute__((noinline)) long w_set(const char *s)
{
    return (long)strcspn(s, "\x81\x88") * 5;
}

__attribute__((noinline)) unsigned w_tbl(int i)
{
    return tbl[i & 3] + (unsigned)pad[i & 3];
}

__attribute__((noinline)) long w_word(long n)
{
    set_slot(&counter, n);
    if (counter < 0)
        return -1;
    return counter + 1;
}

__attribute__((noinline)) long w_name(char *s)
{
    set_name(&name, s);
    return (long)strlen(name) * 3;
}

__attribute__((noinline)) long w_blob(void)
{
    return take((const char *)blob);
}

__attribute__((noinline)) long w_buf(void)
{
    buf[0] = 'H';
    return take(buf);
}

__attribute__((noinline)) long w_wide(long n)
{
    set_slot(&wide, n);
    return *(int *)&wide;
}

int main(int argc, char **argv)
{
    const char *q = w_quote("`");
    const char *r = w_quote("'");
    printf("%ld %02x%02x%02x %02x%02x %ld %ld %ld %ld %ld %ld\n", w_empty("x"), (unsigned char)q[0],
           (unsigned char)q[1], (unsigned char)q[2], (unsigned char)r[0], (unsigned char)r[1], w_word(41),
           w_word(-3), w_name("abcd"), w_blob(), w_buf(), w_wide(0x100000002L));
    printf("%d %d %d %d %ld %ld %ld\n", w_count(digits), w_end(""), w_end("x") > 1, w_hsum(), w_ov(),
           w_set("ab\x88"), w_set("abc"));
    if (argc > 99)
        printf("%u\n", w_tbl(argc));
    return 0;
}

#endif
