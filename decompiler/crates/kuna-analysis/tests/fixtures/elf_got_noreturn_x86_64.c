/* ELF GOT import-call fixture: gcc -O2 -fPIC -pie -fno-plt makes every libc
 * call `call *slot(%rip)` through a GLOB_DAT slot in .got, with no PLT stub.
 * `check` and `die` end in no-return imports; `pick` reads a function pointer
 * stored in a .data.rel.ro table, the same relocation outside .got. */
#include <ctype.h>
#include <stdio.h>
#include <stdlib.h>

int g;

int check(int x)
{
    if (x < 0)
        abort();
    g = x * 3;
    return g + 1;
}

void die(const char *m)
{
    fprintf(stderr, "%s\n", m);
    exit(2);
}

static int (*const classes[])(int) = { isalpha, isdigit };

int pick(int i, int c)
{
    return classes[i & 1](c);
}

int main(int argc, char **argv)
{
    if (argc < 2)
        die("usage");
    return check(atoi(argv[1])) + pick(argc, argv[1][0]);
}
