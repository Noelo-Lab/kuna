typedef unsigned long long u64;
typedef long long i64;
typedef unsigned u32;

u32 xy_shr48(u64 x, u64 y) { return (x + y) >> 48; }
u32 xb_shr36(u64 x, u32 b) { return (x + b) >> 36; }
u32 xy_hi_shr4(u64 x, u64 y) { return (u32)((x + y) >> 32) >> 4; }
u32 xy_hi_and(u64 x, u64 y) { return (u32)((x + y) >> 32) & 0xff; }
u32 xy_hi_add(u64 x, u64 y) { return (u32)((x + y) >> 32) + 5; }
int sxy_hi_neg(i64 x, i64 y) { return -(int)((x + y) >> 32); }
int ss_shr34(int a, int b) { return ((i64)a + b) >> 34; }
u32 s3_hi_mul(u32 a, u32 b, u32 c) { return (u32)(((u64)a + b + c) >> 32) * 7; }
u32 d_shr40(u32 a, u32 b) { return ((u64)a - b) >> 40; }
u32 sum3_shr33(u32 a, u32 b, u32 c) { return ((u64)a + b + c) >> 33; }
u32 xk_shr40(u64 x) { return (x * 3) >> 40; }
u32 p_hi_and(u32 a, u32 b) { return (u32)(((u64)a * b) >> 32) & 0xff; }
u32 p_hi_add(u32 a, u32 b) { return (u32)(((u64)a * b) >> 32) + 5; }
int p_hi_neg(int a, int b) { return -(int)(((i64)a * b) >> 32); }
u32 xm_hi_and(u64 x, u32 k) { return (u32)((x * k) >> 32) & 0xff; }
u32 xx_hi_xor(u64 x, u64 y) { return (u32)((x ^ y) >> 32) & 0xffff; }
u32 hash64(u64 x) { x ^= x >> 33; x *= 0xff51afd7ed558ccdULL; x ^= x >> 33; return (u32)x; }
u64 xy_add(u64 x, u64 y) { return x + y; }
u64 xy_sub(u64 x, u64 y) { return x - y; }
u64 neg64(u64 x) { return -x; }
u64 pm(u32 a, u32 b) { return (u64)a * b; }
u64 pack8(u32 v) { return ((u64)(v & 0xff) << 32) | (v >> 8); }
