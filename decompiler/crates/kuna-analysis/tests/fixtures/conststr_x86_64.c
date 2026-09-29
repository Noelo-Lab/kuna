/* `conststr` fixture: constant addresses of strings and objects that kuna
 * printed as `(T *)0x<addr>`.
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
    return 0;
}

#endif
