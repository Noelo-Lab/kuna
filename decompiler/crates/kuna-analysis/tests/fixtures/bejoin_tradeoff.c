/* Each int function compiles to the same registers as the long long next to
 * it: the int's value is the high word of a 64-bit temporary, and its stale
 * low half is left in the second return register. ltz returns a flag. */
typedef unsigned long long u64; typedef long long i64; typedef unsigned u32;
#define V(h, l) (((u64)(u32)(h) << 32) | (u32)(l))
u64 sum64(u32 ah, u32 al, u32 bh, u32 bl) { return V(ah, al) + V(bh, bl); }
int sum_hi(u32 ah, u32 al, u32 bh, u32 bl) { return (int)((V(ah, al) + V(bh, bl)) >> 32); }
u64 uwide_mul(u32 a, u32 b) { return (u64)a * b; }
u32 mulhi(u32 a, u32 b) { return ((u64)a * b) >> 32; }
int ltz(int ah, u32 al) { return (i64)V(ah, al) < 0; }
