/* kuna `rettriangle` fixture: a one-block `if` before a constant return,
   `if (c) f(); return 0;`, whose shared epilogue `returndup` split into
   `if (!c) return 0; f(); return 0;`.  guard_first is the source's own guard
   clause, which compiles to the same shape; two_way joins two conditions.
   Built with Ubuntu 22.04's gcc 11.4 and clang 14:
     gcc -O0 -fstack-protector-strong -o rettriangle_gcc_O0_x86_64 rettriangle.c
     gcc -O2 -fstack-protector-strong -o rettriangle_gcc_O2_x86_64 rettriangle.c
     clang -O0 -fstack-protector-strong -o rettriangle_clang_O0_x86_64 rettriangle.c  */
#include <stdio.h>
#include <string.h>

static int hits;

__attribute__((noinline)) void celebrate(void)
{
    hits += 1;
}

__attribute__((noinline)) int finish(int ok)
{
    if (ok)
        celebrate();
    return 0;
}

__attribute__((noinline)) int check_name(const char *name)
{
    char buf[32];
    snprintf(buf, sizeof buf, "%s!", name);
    if (strcmp(buf, "go!") == 0)
        celebrate();
    return 0;
}

__attribute__((noinline)) int guard_first(int ok, int *out)
{
    if (!ok)
        return 0;
    out[0] = 1;
    out[1] = 2;
    celebrate();
    return 0;
}

__attribute__((noinline)) int two_way(int a, int b)
{
    if (a)
        return 0;
    if (b)
        celebrate();
    return 0;
}

#ifndef RETTRIANGLE_NO_MAIN
int main(int argc, char **argv)
{
    int out[2] = {0, 0};
    finish(argc > 1);
    check_name(argc > 1 ? argv[1] : "go");
    guard_first(argc, out);
    two_way(argc > 2, argc > 1);
    printf("%d %d %d\n", hits, out[0], out[1]);
    return 0;
}
#endif
