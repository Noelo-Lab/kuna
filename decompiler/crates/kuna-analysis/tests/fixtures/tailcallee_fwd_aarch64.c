/* protoorder tail-callee fixture for a narrowed batch: `forward` ends in `b target`, a tail
 * call that passes its five argument registers through untouched. */
__attribute__((noinline, noreturn)) void panic(const char *m)
{
    for (;;)
        __asm__ volatile("" ::"r"(m));
}

__attribute__((noinline)) long target(int a, int b, long *c, long *d, unsigned e)
{
    long r = *c + a;
    if (b > 3)
        r += *d * b;
    if (e & 4)
        r ^= e;
    d[1] = r;
    return r;
}

long forward(int a, int b, long *c, long *d, unsigned e)
{
    if (*d)
        return target(a, b, c, d, e);
    panic("null");
}
