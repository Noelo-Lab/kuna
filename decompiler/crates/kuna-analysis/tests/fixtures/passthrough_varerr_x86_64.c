/* passthrough_varerr_x86_64.o: gcc -O2 -fno-inline -fno-asynchronous-unwind-tables
 * -fcf-protection=none -c passthrough_varerr_x86_64.c (gcc 11.4, x86-64).
 * check reads rdi and, when x > 5, calls the variadic err with rsi..r9
 * untouched; err's register-save prologue stores all five. wrap forwards
 * rdi to check: one argument, not six. */
#include <stdarg.h>
#include <stdio.h>
#include <stdlib.h>

__attribute__((noinline)) void err(const char *fmt, ...)
{
    va_list ap;
    va_start(ap, fmt);
    vfprintf(stderr, fmt, ap);
    va_end(ap);
    exit(1);
}

__attribute__((noinline)) int check(long x)
{
    if (x > 5)
        err("bad");
    return x + 1;
}

int wrap(long x)
{
    return check(x) + 3;
}
