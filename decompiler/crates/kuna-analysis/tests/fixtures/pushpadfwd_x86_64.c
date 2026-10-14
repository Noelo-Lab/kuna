/* A callee that forwards its own third argument as a STACK argument of a
   printf-family call, and another that forwards it to a non-variadic nine-
   parameter callee. gcc pushes the forwarded register the same way it pushes
   an alignment pad -- `push %reg` then `add $N,%rsp` after the call -- so the
   push alone cannot tell a real forwarded argument from a pad. The format
   string of the printf-family call says how many stack arguments it takes, and
   a same-binary callee's arity says the same, so a push BELOW that count is a
   real argument the default must keep. Built gcc -O2. */
#include <stdio.h>

__attribute__((noinline)) void logit(const char *tag, long a, long c)
{
    printf("%s %ld %ld %ld %ld %ld %ld\n", tag, a, a * 7, a * 9, a ^ 5, c, a * 3);
}

__attribute__((noinline)) int report(long *p)
{
    long z = p[1];
    if (z == 0)
        return -1;
    logit("x", p[0], z);
    return 0;
}

__attribute__((noinline)) long nine(long a, long b, long c, long d, long e,
                                    long f, long g, long h, long i)
{
    return a + b + c + d + e + f + g + h + i;
}

__attribute__((noinline)) long forward(long *p)
{
    long c = p[2];
    if (c == 0)
        return -1;
    return nine(p[0], p[1], c, p[3], p[4], p[5], p[6], p[7], p[8]);
}

int main(int argc, char **argv)
{
    (void)argv;
    static long v[9] = { 1, 2, 3, 4, 5, 6, 7, 8, 9 };
    return report(v) + (int)forward(v) + argc;
}
