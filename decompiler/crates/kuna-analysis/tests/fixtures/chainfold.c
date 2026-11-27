/* kuna `condfold auto`: chains of tests that all branch to one block only they
   enter, which does not end in a call that does not return.  Built by gcc 11.4
   with `-O2 -fno-reorder-blocks-and-partition` and `-O0`. */
#include <stdio.h>

volatile int sink;
int fail_at, steps, errors;
unsigned order;

__attribute__((noinline, noipa)) int put(int *b, int v)
{
    steps++;
    order = order * 31 + (unsigned)v;
    if (steps == fail_at)
        return -steps;
    *b += v;
    return 0;
}

__attribute__((noinline, noipa)) void note(int r)
{
    errors += r;
}

__attribute__((noinline)) int send(int *b, int x, int y)
{
    int r;

    if ((r = put(b, x)) != 0 || (r = put(b, y)) != 0 || (r = put(b, x + y)) != 0) {
        note(r);
        return -1;
    }
    sink = *b;
    return *b + x;
}

__attribute__((noinline)) void store(int *b, int x)
{
    int r;

    if ((r = put(b, x)) != 0 || (r = put(b, 2 * x)) != 0) {
        note(r);
        return;
    }
    sink = *b;
    note(0);
}

#ifndef CHAINFOLD_NO_MAIN
int main(int argc, char **argv)
{
    int b = 0;
    (void)argv;
    fail_at = argc > 4 ? argc : 0;
    store(&b, argc);
    printf("%d %d %d\n", send(&b, argc, 5), b, errors);
    return 0;
}
#endif

__attribute__((noinline)) void commit(int *b, int x)
{
    int r;

    if ((r = put(b, x)) != 0 || (order = 1, (r = put(b, x + 3)) != 0)) {
        note(r);
        return;
    }
    sink = *b;
    note(0);
}
