int stack_args(int a, int b, int c, int d, int e, int f, int g, int h, int i)
{
    return a + b + c + d + e + f + g + h + i;
}

static long from_q7(long n)
{
    if (n < 0)
        return -(long)((unsigned long)-n >> 7);
    return (long)((unsigned long)n >> 7);
}

long dot_q7(const short *x, const short *y)
{
    int i;
    long sum = 0;
    for (i = 0; i < 4; ++i)
        sum += (long)x[i] * (long)y[i];
    return from_q7(sum);
}

void fir_q7(short *out, const short *in, const short *coeff)
{
    int i, tap;
    for (i = 0; i < 3; ++i) {
        long acc = 0;
        for (tap = 0; tap < 3; ++tap)
            acc += (long)in[i + tap] * (long)coeff[tap];
        out[i] = (short)from_q7(acc);
    }
}
