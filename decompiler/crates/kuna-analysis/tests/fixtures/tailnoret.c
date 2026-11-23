/* kuna `tailform`, the no-return form: functions that end in a call that does
   not return, behind an `if` whose other side returns.  Built by gcc 11.4 with
   `-O2 -fno-reorder-blocks-and-partition` (so the fatal paths stay in the
   function) and `-O0`. */
#include <stdio.h>
#include <stdlib.h>

struct node {
    struct node *next;
    int key;
    int val;
};

volatile int sink;

__attribute__((noinline)) int probe(int n)
{
    sink = n;
    return sink - 3;
}

__attribute__((noinline)) int checked(int n)
{
    int r = probe(n);
    if (r < 0)
        abort();
    return r;
}

__attribute__((noinline)) long scaled(long n)
{
    long r = (long)probe((int)n) * n;
    if (r == 0) {
        fprintf(stderr, "zero for %ld\n", n);
        exit(2);
    }
    return r + 7;
}

__attribute__((noinline)) int pick(int *v, unsigned i, unsigned n)
{
    if (!v)
        abort();
    if (i >= n)
        abort();
    return v[i];
}

__attribute__((noinline)) int lookup(struct node *p, int k)
{
    for (; p; p = p->next) {
        if (p->key == k) {
            if (p->val < 0)
                abort();
            return p->val;
        }
    }
    return -1;
}

__attribute__((noinline)) void store(int *slot, int v)
{
    if (!slot)
        abort();
    *slot = v;
}

#ifndef TAILNORET_NO_MAIN
int main(int argc, char **argv)
{
    struct node b = {0, 2, 20}, a = {&b, 1, 10};
    int v[4] = {1, 2, 3, 4};
    int slot = 0;
    (void)argv;
    store(&slot, argc);
    printf("%d %ld %d %d %d\n", checked(argc + 3), scaled(argc + 4), pick(v, (unsigned)argc, 4), lookup(&a, 2), slot);
    return 0;
}
#endif
