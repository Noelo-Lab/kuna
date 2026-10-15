/* Two classic mains that elfmain must leave as the analysis finds them. Built
   gcc -O2 twice (-DARGV_ONLY selects the second):
   - elfmainfwd_x86_64: main hands argc and argv unchanged to its first call,
     so the body reads them only through that call; the analysis recovers
     both parameters as whole 64-bit registers.
   - elfmainargv_x86_64: main reads only argv, after a helper call; the
     analysis recovers an unread first slot ahead of argv.
   Declaring the C runtime's main on either would add or drop a parameter. */
#include <stdio.h>

__attribute__((noinline)) static int count(int n, char **v)
{
    return n > 1 ? v[1][0] : 0;
}

__attribute__((noinline)) static void ping(void)
{
    static volatile int seen;
    seen++;
}

#ifndef ARGV_ONLY
int main(int argc, char **argv)
{
    int r = count(argc, argv);
    printf("%d\n", r);
    return 0;
}
#else
int main(int argc, char **argv)
{
    (void)argc;
    ping();
    return puts(argv[1]);
}
#endif
