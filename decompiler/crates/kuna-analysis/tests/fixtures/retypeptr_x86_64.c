/* A local retyped to a structure pointer keeps the name a directive pins on it.
   gcc -O0 -o retypeptr_x86_64 retypeptr_x86_64.c (walk carries optimize("O2")) */
#include <stdio.h>
#include <string.h>

__attribute__((noinline)) int squeeze(char *s)
{
    int dots = 0;
    char *out = s;
    for (char *in = s; *in; in++) {
        if (*in != '.')
            *out++ = *in;
        else
            dots++;
    }
    *out = '\0';
    return dots;
}

__attribute__((noinline, optimize("O2"))) long walk(long *p, long *end)
{
    long t = 0;
    while (p != end) {
        long *next = p + 1;
        t += *p;
        p = next;
    }
    return t;
}

int main(int c, char **v)
{
    char b[16];
    strcpy(b, c > 1 ? "x.y" : "a.b.c");
    long a[3] = {1, 2, 3};
    printf("%d %s %ld\n", squeeze(b), b, walk(a, a + 3));
    return 0;
}
