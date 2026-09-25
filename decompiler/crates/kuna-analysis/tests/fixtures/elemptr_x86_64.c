/* `elemptr` fixture: pointers the program uses only as arrays of one element
 * type -- a parameter, a buffer an allocator returned, a global that holds
 * one, and a table in `.data` read with a computed index -- plus controls that
 * must keep their integer form: a record walked by a stride, and one pointer
 * read at two widths.
 *
 * Build (non-PIE at 0x30000000 so a test harness can map the data at the same
 * addresses; -x drops the local symbols, so the data has no names):
 *   gcc   -O0 -fno-builtin -fno-pie -no-pie -Wl,-Ttext-segment=0x30000000 -Wl,-x \
 *         -o elemptr_gcc_O0_x86_64 elemptr_x86_64.c
 *   clang -O0 -fno-builtin -fno-pie -no-pie -Wl,-Ttext-segment=0x30000000 -Wl,-x \
 *         -o elemptr_clang_O0_x86_64 elemptr_x86_64.c
 *   gcc   -O2 -fno-inline -fno-builtin -fno-pie -no-pie -Wl,-Ttext-segment=0x30000000 -Wl,-x \
 *         -o elemptr_gcc_O2_x86_64 elemptr_x86_64.c
 *
 * The witnesses are the functions named `w_*`; `main` runs them over inputs
 * that include bytes at or above 0x80 (read signed and unsigned, and used as
 * indexes both ways) and negative indexes, and prints two lines.  The second
 * line reads `.data` tables whose elements have their top bit set: 2- and
 * 4-byte elements returned to a caller that widens them, one read shifted and
 * one only compared, and a byte table two functions read at two signs.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static char encoding_table[64] = {
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P',
    'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f',
    'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v',
    'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/'};
static char *decoding_table;
static int weights[8] = {3, -5, 7, 11, -13, 17, 0x7f80, -0x7f80};
static unsigned short wtab[64] = {1, 0x8000, 0xffff, 0x7fff, 0x8001, 3, 0x9abc};
static unsigned int itab[64] = {1, 0x80000000u, 0xffffffffu, 7};
static unsigned char xlat[256] = {0x41, 0x80, 0xff, 0x7f, 0xc3, 0x10};

void w_build(void)
{
    decoding_table = malloc(256);
    for (int i = 0; i < 256; i++)
        decoding_table[i] = (char)0x80;
    for (int i = 0; i < 64; i++)
        decoding_table[(unsigned char)encoding_table[i]] = i;
}

unsigned char *w_decode(const char *data, size_t input_length, size_t *output_length)
{
    if (decoding_table == NULL)
        w_build();
    if (input_length % 4 != 0)
        return NULL;
    *output_length = input_length / 4 * 3;
    if (data[input_length - 1] == '=')
        (*output_length)--;
    if (data[input_length - 2] == '=')
        (*output_length)--;
    unsigned char *decoded_data = malloc(*output_length + 1);
    if (decoded_data == NULL)
        return NULL;
    for (size_t i = 0, j = 0; i < input_length;) {
        unsigned int a = data[i] == '=' ? 0 & i++ : decoding_table[(unsigned char)data[i++]];
        unsigned int b = data[i] == '=' ? 0 & i++ : decoding_table[(unsigned char)data[i++]];
        unsigned int c = data[i] == '=' ? 0 & i++ : decoding_table[(unsigned char)data[i++]];
        unsigned int d = data[i] == '=' ? 0 & i++ : decoding_table[(unsigned char)data[i++]];
        unsigned int triple = (a << 3 * 6) + (b << 2 * 6) + (c << 1 * 6) + (d << 0 * 6);
        if (j < *output_length)
            decoded_data[j++] = (triple >> 2 * 8) & 0xFF;
        if (j < *output_length)
            decoded_data[j++] = (triple >> 1 * 8) & 0xFF;
        if (j < *output_length)
            decoded_data[j++] = (triple >> 0 * 8) & 0xFF;
    }
    decoded_data[*output_length] = 0;
    return decoded_data;
}

long w_sbytes(const char *p, int n)
{
    long s = 0;
    for (int i = 0; i < n; i++)
        s = s * 3 + p[i];
    return s;
}

long w_ubytes(const unsigned char *p, int n)
{
    long s = 0;
    for (int i = 0; i < n; i++)
        s = s * 3 + p[i];
    return s;
}

long w_words(const int *a, int n)
{
    long s = 0;
    for (int i = 0; i < n; i++)
        s = s * 5 + a[n - 1 - i];
    return s;
}

long w_back(const int *end, int n)
{
    long s = 0;
    for (int i = 0; i < n; i++)
        s = s * 7 + end[i - n];
    return s;
}

long w_sidx(const signed char *s, int n, const int *mid)
{
    long acc = 0;
    for (int i = 0; i < n; i++)
        acc = acc * 2 + mid[s[i]];
    return acc;
}

long w_table(int n)
{
    long s = 0;
    for (int i = 0; i < n; i++)
        s = s * 11 + weights[i];
    return s;
}

char *w_rev(const char *s, int n)
{
    char *out = malloc(n + 1);
    for (int i = 0; i < n; i++)
        out[i] = s[n - 1 - i];
    out[n] = 0;
    return out;
}

long w_record(const long *p, int n)
{
    long s = 0;
    for (int i = 0; i < n; i++)
        s += p[2 * i] * p[2 * i + 1];
    return s;
}

long w_mixed(const char *p, int i)
{
    return p[i] + *(const int *)(p + 4);
}

unsigned long w_wu(unsigned int i)
{
    return wtab[i & 63];
}

unsigned long w_wucall(unsigned int i)
{
    return w_wu(i) + 0x10000;
}

unsigned long w_iu(unsigned int i)
{
    return itab[i & 63];
}

unsigned long w_iucall(unsigned int i)
{
    return w_iu(i) + 1;
}

unsigned long w_iu2(unsigned int i)
{
    unsigned int v = itab[i & 63];
    return v / 2 + v;
}

long w_srch(unsigned int k)
{
    for (int i = 0; i < 4; i++)
        if (itab[i] == k)
            return i;
    return -1;
}

unsigned long w_xu(const unsigned char *p, int n)
{
    unsigned long s = 0;
    for (int i = 0; i < n; i++)
        s = s * 257 + xlat[p[i]];
    return s;
}

long w_xs(const unsigned char *p, int n)
{
    long s = 0;
    for (int i = 0; i < n; i++)
        s = s * 257 + (signed char)xlat[p[i]];
    return s;
}

int main(void)
{
    static const char hi[] = "\x81\x7f\xfe\x01\x80\x10";
    static const int wide[] = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16};
    static int span[256];
    for (int i = 0; i < 256; i++)
        span[i] = i * 3 - 384;
    static const long recs[] = {2, 3, 5, 7, 11, 13};
    size_t n = 0;
    unsigned char *dec = w_decode("aGVsbG8gd29ybGQ=", 16, &n);
    char *rev = w_rev("kuna", 4);
    long a = w_sbytes(hi, 6);
    long b = w_ubytes((const unsigned char *)hi, 6);
    long c = w_words(wide, 16);
    long d = w_back(wide + 16, 16);
    long e = w_sidx((const signed char *)hi, 6, span + 128);
    long f = w_table(8);
    long g = w_record(recs, 3);
    long h = w_mixed("abcdefgh", 2);
    printf("%s %zu %s %ld %ld %ld %ld %ld %ld %ld %ld\n", (char *)dec, n, rev, a, b, c, d, e, f, g, h);
    static const unsigned char ix[] = {0, 1, 2, 3, 4, 5};
    printf("%lu %lu %lu %lu %lu %lu %ld %ld %lu %ld\n", w_wucall(1), w_wucall(6), w_iucall(1), w_iucall(3), w_iu2(1),
           w_iu2(2), w_srch(0xffffffffu), w_srch(5), w_xu(ix, 6), w_xs(ix, 6));
    return 0;
}
