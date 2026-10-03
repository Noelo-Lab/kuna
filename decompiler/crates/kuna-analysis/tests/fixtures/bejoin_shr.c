typedef unsigned long long u64;
typedef long long i64;
typedef unsigned u32;

u32 avg_u(u32 a, u32 b) { return ((u64)a + b) >> 1; }
int avg_s(int a, int b) { return ((i64)a + b) >> 1; }
u32 avg_d(u32 a, u32 b) { return ((u64)a + b) / 2; }
u32 avg3(u32 a, u32 b, u32 c) { return ((u64)a + b + c) >> 1; }
u32 mshr1(u32 a, u32 b) { return ((u64)a * b) >> 1; }
int sshr1(int a, int b) { return ((i64)a * b) >> 1; }
u32 mshr1k(u32 a) { return ((u64)a * 0x9e3779b9u) >> 1; }
u32 ashr1(u32 a, u32 b) { return ((u64)a + b + 1) >> 1; }
u32 sub_shr1(u32 a, u32 b) { return ((u64)a - b) >> 1; }
u32 avg_ret(u32 a, u32 b) { u32 r = ((u64)a + b) >> 1; return r ? r : 1; }
u32 uq1r(u32 a, u32 b) { return (u32)(((u64)a * b + 1) >> 1); }
u32 mid64(u64 a, u64 b) { return (u32)((a + b) >> 1); }
u64 avg64(u32 a, u32 b) { return ((u64)a + b) >> 1; }
u64 mshr1_64(u32 a, u32 b) { return ((u64)a * b) >> 1; }
i64 sshr33(i64 a) { return a >> 33; }
