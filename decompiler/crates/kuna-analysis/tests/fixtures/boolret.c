/* kuna `boolbranch return` fixture: functions whose whole body is one
   short-circuit test that gcc -O0 materializes with a jump to a block that
   sets 1 and a block that sets 0, and neighbours that keep their `if`.
   Built with Ubuntu 22.04's gcc 11.4:
     gcc -O0 -o boolret_gcc_O0_x86_64 boolret.c  */
#include <stdio.h>

struct run {
    int pad[0x17];
    unsigned in_ch;
    int in_len;
};

struct node {
    int kind;
    int flags;
    struct node *next;
};

volatile int knob;

__attribute__((noipa)) int probe(const char *p)
{
    return p[0] == 'x' ? 0 : -1;
}

int is_abs(const char *path)
{
    return path != 0 && path[0] == '/';
}

int is_full(const struct run *s)
{
    if (s->in_ch < 256 && s->in_len > 0)
        return 0;
    else
        return 1;
}

int probe_ok(const char *p)
{
    return probe(p) == 0 && p[1] != 0;
}

int last_match(const struct node *n)
{
    return n && (n->kind == 3 || (n->flags & 4)) && n->next == 0;
}

int same(int a, int b)
{
    if (a == b)
        return 1;
    return 0;
}

int either(int a, int b)
{
    if (a == 3)
        return 1;
    if (b == 9)
        return 1;
    return 0;
}

int bump_and_test(const int *p, int a)
{
    knob = a + 1;
    return p != 0 && *p == a;
}

#ifndef BOOLRET_NO_MAIN
int main(int argc, char **argv)
{
    struct run r = { { 0 }, (unsigned)argc, argc - 1 };
    struct node n = { argc, argc, 0 };
    int v = argc;
    printf("%d %d %d %d %d %d %d\n", is_abs(argv[0]), is_full(&r), probe_ok(argv[0]),
           last_match(&n), same(argc, 2), either(argc, argc), bump_and_test(&v, argc));
    return 0;
}
#endif
