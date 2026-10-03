typedef long long i64;
typedef unsigned long long u64;

int fits_int(i64 v) { return v == (int)v; }
int lua_fits(i64 i) { return (u64)i + 0x80000000ULL <= 0xffffffffULL; }
int ovf(int a, int b) { i64 p = (i64)a * b; return p != (int)p; }
int pick(i64 a, i64 b, int c) { return (int)(c ? a : b); }
i64 pick64(i64 a, i64 b, int c) { return c ? a : b; }
int min_len(i64 len) { return (int)(len < 0x7fffffff ? len : 0x7fffffff); }
int tabs(i64 a) { return (int)(a < 0 ? -a : a); }
i64 tabs64(i64 a) { return a < 0 ? -a : a; }
