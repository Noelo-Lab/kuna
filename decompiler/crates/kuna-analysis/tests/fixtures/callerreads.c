#include <stdio.h>
#include <string.h>

int gi, gj;

__attribute__((noinline)) int triple(int a) { gi = a * 3; return gi; }
__attribute__((noinline)) int keep(int *p) { int x = *p; gi = x + 1; gj = x; return x; }
__attribute__((noinline)) int same(char **p) { return strcmp(p[0], p[1]); }
__attribute__((noinline)) void scale(int a) { gj = a * 5; }

int main(int argc, char **argv)
{
    int v = argc + 4;
    scale(argc);
    if (argc > 2 && same(argv + 1) == 0)
        puts("same");
    printf("%d %d %d %d\n", triple(argc), keep(&v), gi, gj);
    return 0;
}
