/* A 64-bit comparison on big-endian ARM computes the low words' difference
 * into r1 for its flags and returns the int result in r0, so r1 holds what
 * looks like the low word of a returned long long. */
typedef unsigned long long u64; typedef long long i64; typedef unsigned u32;
#define V(h, l) (((u64)(u32)(h) << 32) | (u32)(l))
int big(u32 h, u32 l) { return V(h, l) >= 0x80000000ULL; }
int ge64(u32 ah, u32 al, u32 bh, u32 bl) { return V(ah, al) >= V(bh, bl); }
int lt64(int ah, u32 al, int bh, u32 bl) { return (i64)V(ah, al) < (i64)V(bh, bl); }
int sat(u32 ah, u32 al) {
  i64 v = (i64)V(ah, al);
  if (v > 0x7fffffff) return 0x7fffffff;
  if (v < -0x7fffffff - 1) return -0x7fffffff - 1;
  return (int)v;
}
