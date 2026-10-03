typedef unsigned long long u64;
typedef long long i64;
typedef unsigned u32;

u32 ud10(u32 a) { return a / 10; }
u32 ud100(u32 a) { return a / 100; }
u32 ud1000(u32 a) { return a / 1000; }
u32 ud3600(u32 a) { return a / 3600; }
u32 ud86400(u32 a) { return a / 86400; }
u32 ud3(u32 a) { return a / 3; }
u32 ud5(u32 a) { return a / 5; }
int sd3(int a) { return a / 3; }
u32 ud7(u32 a) { return a / 7; }
u32 ud19(u32 a) { return a / 19; }
u32 dv_365(u32 a) { return a / 365; }
u32 ud7_ptr(u32 *p) { return *p / 7; }
u32 um7(u32 a) { return a % 7; }
i64 m3(i64 a) { return a * 3; }
u64 mk(u32 a) { return (u64)a * 10; }
u64 mk2(u64 a) { return a * 0xcccccccdULL; }
i64 mk3(int a, int b) { return (i64)a * b + 7; }
u64 mk4(u32 a) { return (u64)a * 0x24924925u + a; }
u64 mk5(u32 a, u32 b) { return ((u64)a * b) >> 3; }
