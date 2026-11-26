/* kuna `condfold noreturn`: chains of tests that all branch to one call that
   does not return.  Built by gcc 11.4 with `-O2 -fno-reorder-blocks-and-partition`
   (so the fatal paths stay in the function) and `-O0`. */
#include <stdio.h>
#include <stdlib.h>

volatile int sink;
int fail_at, steps;
unsigned order;

__attribute__((noinline)) int put(int *b, int v)
{
    steps++;
    order = order * 31 + (unsigned)v;
    if (steps == fail_at)
        return -steps;
    *b += v;
    return 0;
}

__attribute__((noinline, noreturn)) void fatal(int r, const char *what)
{
    fprintf(stderr, "%s: %d\n", what, r);
    exit(3);
}

__attribute__((noinline)) void reply(int *b, int type, int id, int n)
{
    int r;

    if ((r = put(b, type)) != 0 || (r = put(b, id)) != 0 || (r = put(b, n)) != 0)
        fatal(r, "reply");
}

__attribute__((noinline)) int compose(int *b, int id, int n)
{
    int r;

    if ((r = put(b, 7)) != 0 || (r = put(b, id)) != 0 || (r = put(b, n)) != 0)
        fatal(r, "compose");
    sink = *b;
    if (*b > 100)
        return -1;
    return *b + id;
}

__attribute__((noinline)) int twice(int *b, int id)
{
    int r;

    if (b == NULL)
        fatal(0, "null");
    if ((r = put(b, 1)) != 0 || (r = put(b, id)) != 0)
        fatal(r, "first");
    sink = id;
    if ((r = put(b, 2)) != 0 || (r = put(b, *b)) != 0)
        fatal(r, "second");
    return *b;
}

__attribute__((noinline)) void wide(int *b, int id)
{
    int r;

    if ((r = put(b, id)) != 0 || (put(b, 1), put(b, 2), put(b, 3), r = put(b, id + 1)) != 0)
        fatal(r, "wide");
}

#ifndef FATALCHAIN_NO_MAIN
int main(int argc, char **argv)
{
    int b = 0;
    (void)argv;
    fail_at = argc > 4 ? argc : 0;
    reply(&b, argc, 2, 3);
    wide(&b, argc);
    printf("%d %d %d\n", compose(&b, argc, 4), twice(&b, argc), b);
    return 0;
}
#endif
