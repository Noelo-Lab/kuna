/* kuna `tailform` fixture: function tails whose `if` the binary prints in the
   form the compiler's return layout picked, not the one the source wrote.
   Built with Ubuntu 22.04's gcc 11.4:
     gcc -O2 -o tailform_gcc_O2_x86_64 tailform.c
     gcc -O0 -o tailform_gcc_O0_x86_64 tailform.c  */
#include <stdio.h>

struct item {
    int value;
    struct item *next;
};

int hits;
int trail[64];
int ntrail;

__attribute__((noipa)) void note(void)
{
    hits += 1;
}

__attribute__((noipa)) void record(int v)
{
    trail[ntrail++ & 63] = v;
}

/* A short void tail ending in a sibling call: the call becomes a tail jump
   with its own exit, which reads as a guard clause. */
__attribute__((noipa)) void note_if_zero(int a)
{
    if (a == 0)
        note();
}

/* A short tail before a constant return. */
__attribute__((noipa)) int record_pair(int a, int b)
{
    if (a) {
        record(a);
        record(b);
    }
    return 0;
}

/* A compound condition. */
__attribute__((noipa)) int record_above(struct item *p, int floor)
{
    if (p && p->value > floor)
        record(p->value);
    return 0;
}

/* A compound condition before a sibling call. */
__attribute__((noipa)) void record_positive(struct item *p)
{
    if (p && p->value > 0)
        record(p->value);
}

/* An argument returned unchanged on both paths. */
__attribute__((noipa)) int keep_first(int a, int b)
{
    if (b < 0)
        return a;
    record(b);
    return a;
}

/* The same variable returned on both paths. */
__attribute__((noipa)) int pass_through(int a, int b)
{
    if (b > 1)
        record(b);
    return a;
}

/* A long void body that shares the epilogue, which reads as a wrapped block. */
__attribute__((noipa)) void replay(struct item *p)
{
    if (!p)
        return;
    record(p->value);
    record(p->value + 1);
    record(p->value + 2);
    record(p->value + 3);
    record(p->value + 4);
    hits = p->value;
}

/* Different return values: not a candidate. */
__attribute__((noipa)) int first_value(struct item *p)
{
    if (!p)
        return -1;
    record(p->value);
    return 0;
}

#ifndef TAILFORM_NO_MAIN
int main(int argc, char **argv)
{
    struct item it = {argc, 0};
    note_if_zero(argc - 1);
    record_pair(argc, argc + 1);
    record_above(&it, 0);
    pass_through(argc, argc + 1);
    record_positive(&it);
    keep_first(argc, argc - 2);
    replay(argc > 2 ? 0 : &it);
    first_value(argc > 3 ? 0 : &it);
    printf("%d %d\n", hits, ntrail);
    return 0;
}
#endif
