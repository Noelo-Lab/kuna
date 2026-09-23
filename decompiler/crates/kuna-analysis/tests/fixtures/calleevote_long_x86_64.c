/* calleevote fixture: the bound on the redo pass (x86-64).
 *
 * `scan_short` and `scan_long` do the same thing with the pointer `use` hands
 * them -- nothing but pass it on to `memcmp`, which declares `const void *` --
 * and `use` holds the record, so both are candidates for the caller's type.
 * The vote costs a second decompile of the callee, so only a short body is
 * decompiled again: `scan_short` (19 printed lines) takes `struct_1 *`, and
 * `scan_long` (45 lines, past CALLEE_VOTE_MAX_LINES) keeps `void *`. The
 * global array is there only to make one body long; both functions do the
 * same thing with the pointer, and `use` takes the record in a register the
 * calls have to move, so both calls carry their arguments.
 *
 * Build (symbols kept, no DWARF):
 *   gcc -O2 -fno-inline -fno-stack-protector -fcf-protection=none \
 *       -o calleevote_long_x86_64 calleevote_long_x86_64.c
 */
#include <stdio.h>
#include <string.h>
#define NI __attribute__((noinline))
struct rec { int n; long used; long buckets; long cap; };
static long g[24];
NI int scan_short(const void *p, unsigned long n)
{
    long s = (long)n;
    g[0] += s; g[1] ^= s + 1; g[2] |= s << 1; g[3] -= s + 3;
    g[4] += s; g[5] ^= s + 5; g[6] |= s << 1; g[7] -= s + 7;
    g[0] ^= g[5] + 2; g[1] += g[7] ^ s; g[2] -= g[9] + 4; g[3] |= g[11] << 2;
    g[4] ^= g[9] + 6; g[5] += g[11] ^ s; g[6] -= g[13] + 8; g[7] |= g[15] << 2;
    return (int)(g[7] + g[0]) + memcmp(p, "abcdefgh", n);
}
NI int scan_long(const void *p, unsigned long n)
{
    long s = (long)n;
    g[0] += s; g[1] ^= s + 1; g[2] |= s << 1; g[3] -= s + 3;
    g[4] += s; g[5] ^= s + 5; g[6] |= s << 1; g[7] -= s + 7;
    g[8] += s; g[9] ^= s + 9; g[10] |= s << 1; g[11] -= s + 11;
    g[12] += s; g[13] ^= s + 13; g[14] |= s << 1; g[15] -= s + 15;
    g[16] += s; g[17] ^= s + 17; g[18] |= s << 1; g[19] -= s + 19;
    g[20] += s; g[21] ^= s + 21; g[22] |= s << 1; g[23] -= s + 23;
    g[0] ^= g[5] + 2; g[1] += g[7] ^ s; g[2] -= g[9] + 4; g[3] |= g[11] << 2;
    g[4] ^= g[9] + 6; g[5] += g[11] ^ s; g[6] -= g[13] + 8; g[7] |= g[15] << 2;
    g[8] ^= g[13] + 10; g[9] += g[15] ^ s; g[10] -= g[17] + 12; g[11] |= g[19] << 2;
    g[12] ^= g[17] + 14; g[13] += g[19] ^ s; g[14] -= g[21] + 16; g[15] |= g[23] << 2;
    g[16] ^= g[21] + 18; g[17] += g[23] ^ s; g[18] -= g[1] + 20; g[19] |= g[3] << 2;
    g[20] ^= g[1] + 22; g[21] += g[3] ^ s; g[22] -= g[5] + 24; g[23] |= g[7] << 2;
    return (int)(g[23] + g[0]) + memcmp(p, "abcdefgh", n);
}
NI long fill(struct rec *r, int c)
{
    r->n = c; r->used = c * 2; r->buckets = c * 3; r->cap = 16;
    return r->used + r->buckets;
}
NI long use(int k, struct rec *r, unsigned long n)
{
    long s = k + r->n + r->used + r->buckets + r->cap;
    s += scan_short(r, n);
    s += scan_long(r, n);
    return s;
}
int main(int argc, char **argv)
{
    struct rec a, b;
    long s = fill(&a, argc) + fill(&b, argc + 1);
    s += use(argc, &a, (unsigned long)argc) + use(argc + 1, &b, 8);
    printf("%ld %s\n", s, argv[0]);
    return 0;
}
