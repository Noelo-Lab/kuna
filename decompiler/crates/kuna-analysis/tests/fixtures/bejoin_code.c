typedef long long i64;
typedef unsigned long long u64;
typedef unsigned u32;

int chk_err(i64 v) { if (v < -2147483648LL || v > 2147483647LL) return 34; return 0; }
int chk_m1(i64 v) { return v == (int)v ? 0 : -1; }
int st_code2(i64 a, i64 b) { i64 d = a - b; if (d != (int)d) return -1; return 1; }
int fits_neg(i64 v) { return v == (int)v ? -1 : 0; }
int ovf_m1(int a, int b) { i64 p = (i64)a * b; return p != (int)p ? -1 : 0; }
int d_code(i64 a, i64 b) { i64 d = a - b; return d == (int)d ? 0 : 22; }
i64 add_k(i64 v) { return v + 0x80000000LL; }
i64 d64(i64 a, i64 b) { return a - b; }
i64 mul_k(int a, int b) { return (i64)a * b + 0x80000000LL; }
i64 clamp_ll(i64 v) { if (v != (int)v) return 0; return v + 1; }
u64 zsum(u32 a, u32 b) { return a + b; }
