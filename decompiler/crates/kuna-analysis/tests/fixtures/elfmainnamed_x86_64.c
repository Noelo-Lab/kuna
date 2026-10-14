/* A classic main: it reads argc and argv, never envp. Built gcc -O2 (a PIE),
   once as is and once stripped, so the same main is reached through its own
   symbol and through the _start -> __libc_start_main decode. */
#include <stdio.h>

__attribute__((noinline)) static int score(const char *s)
{
    int h = 0;
    while (*s)
        h = h * 31 + *s++;
    return h;
}

int main(int argc, char **argv)
{
    if (argc < 2) {
        fprintf(stderr, "usage: %s word\n", argv[0]);
        return 2;
    }
    int h = score(argv[1]);
    printf("%d\n", h);
    return h & 1;
}
