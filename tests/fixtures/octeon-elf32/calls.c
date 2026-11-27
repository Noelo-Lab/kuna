/* CRC-16 with polynomial 0x1021 and initial value 0. GCC compiles the bit-15
   test to an Octeon bbit0. */
unsigned int crc16(const unsigned char *p, int n)
{
    unsigned int crc = 0;
    while (n-- > 0) {
        crc ^= (unsigned int)*p++ << 8;
        for (int i = 0; i < 8; i++) {
            if (crc & 0x8000)
                crc = ((crc << 1) ^ 0x1021) & 0xffff;
            else
                crc = (crc << 1) & 0xffff;
        }
    }
    return crc;
}

/* A caller that reads crc16's result. */
int verify(const unsigned char *p, int n, unsigned int want)
{
    return crc16(p, n) == want;
}

__attribute__((noinline)) int emit(char *out, int size, const char *tag, unsigned field, unsigned low)
{
    out[0] = tag[0];
    return size + (int)(field * 8 + low);
}

/* The fourth argument is a field of the upper word (dext); the fifth travels
   in the O64 stack slot at SP + 32. */
int describe(char *out, unsigned long long word)
{
    return emit(out, 64, "w", (unsigned)(word >> 35) & 0x3ff, (unsigned)(word >> 3) & 7) + 1;
}
