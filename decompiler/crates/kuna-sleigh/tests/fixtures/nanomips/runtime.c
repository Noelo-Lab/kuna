/* The compiler support routines the workload needs, so the image is self-contained. */
#include <stdint.h>
typedef uint32_t u32;
typedef uint64_t u64;
typedef int64_t s64;
#define PLAIN __attribute__((optimize("no-tree-loop-distribute-patterns")))

PLAIN void *memcpy(void *d, const void *s, unsigned long n)
{
    unsigned char *o = d; const unsigned char *i = s;
    while (n--) *o++ = *i++;
    return d;
}

static u64 divmod(u64 n, u64 d, u64 *rem)
{
    u64 q = 0, r = 0;
    for (int i = 63; i >= 0; i--) {
        r = (r << 1) | ((n >> i) & 1);
        if (r >= d) { r -= d; q |= (u64)1 << i; }
    }
    if (rem) *rem = r;
    return q;
}
u64 __udivdi3(u64 n, u64 d) { return divmod(n, d, 0); }
u64 __umoddi3(u64 n, u64 d) { u64 r; divmod(n, d, &r); return r; }
s64 __divdi3(s64 n, s64 d)
{
    u64 q = divmod(n < 0 ? -(u64)n : (u64)n, d < 0 ? -(u64)d : (u64)d, 0);
    return (n < 0) != (d < 0) ? -(s64)q : (s64)q;
}
u64 __lshrdi3(u64 v, int n)
{
    u32 hi = (u32)(v >> 32), lo = (u32)v;
    if (n == 0) return v;
    if (n >= 32) return hi >> (n - 32);
    return ((u64)(hi >> n) << 32) | ((lo >> n) | (hi << (32 - n)));
}
u64 __ashldi3(u64 v, int n)
{
    u32 hi = (u32)(v >> 32), lo = (u32)v;
    if (n == 0) return v;
    if (n >= 32) return (u64)(lo << (n - 32)) << 32;
    return ((u64)((hi << n) | (lo >> (32 - n))) << 32) | (lo << n);
}
s64 __ashrdi3(s64 v, int n)
{
    int32_t hi = (int32_t)(v >> 32); u32 lo = (u32)v;
    if (n == 0) return v;
    if (n >= 32) return (s64)(hi >> (n - 32));
    return ((s64)(hi >> n) << 32) | ((lo >> n) | ((u32)hi << (32 - n)));
}
