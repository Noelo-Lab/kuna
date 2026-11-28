int addone(int x)
{
    return x + 1;
}

unsigned short scalar(unsigned short x, short y)
{
    unsigned short hi = (unsigned short)(x >> 4);
    unsigned short lo = (unsigned short)((unsigned short)y & 15u);
    return (unsigned short)((hi << 4) | lo);
}

int nested(int x, int y)
{
    int i, j, sum = 0;
    for (i = 0; i < x; ++i)
        for (j = 0; j < y; ++j)
            if (i > j)
                sum += i - j;
    return sum;
}

struct pair {
    unsigned short first;
    unsigned short second;
};

unsigned short bump_field(struct pair *out, const struct pair *in)
{
    out[1].second = (unsigned short)(in[0].second + 1u);
    return in[1].second;
}

unsigned short volatile_pair(volatile unsigned short *p)
{
    unsigned short a = p[0];
    unsigned short b = p[0];
    p[1] = (unsigned short)(a ^ b);
    return b;
}
