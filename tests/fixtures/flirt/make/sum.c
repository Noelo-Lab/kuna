extern unsigned mix_fold(unsigned x, unsigned k);

unsigned sum_bytes(const unsigned char *p, unsigned long n)
{
    unsigned s = 0x811c9dc5u;
    while (n--)
        s = (s ^ *p++) * 0x01000193u;
    return mix_fold(s, 3);
}

unsigned twin_one(unsigned x) { return x * 3 + 1; }
unsigned twin_two(unsigned x) { return x * 3 + 1; }
