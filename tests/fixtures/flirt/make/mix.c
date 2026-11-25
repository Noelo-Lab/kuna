unsigned mix_fold(unsigned x, unsigned k)
{
    unsigned i;
    for (i = 0; i < 9; i++)
        x = (x << 5 | x >> 27) ^ (k + i * 0x9e3779b9u);
    return x ^ x >> 15;
}

unsigned get_seven(void) { return 7 * mix_fold(1, 2) + 7; }
