#include <stdarg.h>
extern int vprintf(const char *, va_list);
__attribute__((noinline)) void report(const char *fmt, ...)
{
    va_list ap;
    va_start(ap, fmt);
    vprintf(fmt, ap);
    va_end(ap);
}
__attribute__((noinline)) long sum9(long a, long b, long c, long d, long e, long f, long g, long h, long i)
{
    return a + b + c + d + e + f + g + h + i;
}
__attribute__((noinline)) long check(long *p, const char *file, long line)
{
    if (!p) {
        report("NULL passed @%s:%ld", file, line);
        return 0;
    }
    return sum9(*p, 1, 2, 3, 4, 5, 6, 7, line);
}
int main(int argc, char **argv)
{
    long v = argc;
    return (int)check(&v, argv[0], 42);
}
